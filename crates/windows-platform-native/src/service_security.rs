// SPDX-License-Identifier: GPL-2.0-only
//! Bounded owner and DACL envelope from a successful local SCM security query.

use core::{fmt, ops::Range};

/// Maximum buffer, in bytes, accepted by `QueryServiceObjectSecurity`.
pub(super) const SERVICE_SECURITY_BYTES: usize = 8192;
/// Fixed part of a `SECURITY_DESCRIPTOR_RELATIVE` before the referenced objects.
const DESCRIPTOR_HEADER_BYTES: usize = 20;
/// Length of the fixed SID header before its 32-bit subauthorities.
const SID_HEADER_BYTES: usize = 8;
/// Maximum number of SID subauthorities in the Windows representation.
const SID_MAX_SUBAUTHORITIES: u8 = 15;
/// Length of the fixed ACL header before its access-control entries.
const ACL_HEADER_BYTES: usize = 8;
/// Descriptor control flag for offsets relative to the beginning of the buffer.
const SE_SELF_RELATIVE: u16 = 0x8000;
/// Descriptor control flag distinguishing a present DACL from an absent one.
const SE_DACL_PRESENT: u16 = 0x0004;

/// Owner and DACL reported together by a local service-object security query.
///
/// The report preserves the descriptor control bits and the checked byte ranges
/// for the owner SID and DACL. Access-control entries remain opaque. Current
/// permissions require a separate check on the actual service object.
#[must_use = "retain the actual service security report"]
pub struct ReportedServiceSecurity {
    /// Fixed native buffer, with only the checked regions exposed by getters.
    bytes: [u8; SERVICE_SECURITY_BYTES],
    /// Control bits and admitted regions from this same buffer.
    regions: DescriptorRegions,
}

impl ReportedServiceSecurity {
    /// Admit a snapshot after the native operation successfully supplied the
    /// requested security descriptor. Only descriptor-declared regions are read.
    ///
    /// # Errors
    /// Reject an unsupported descriptor or a malformed owner or DACL envelope.
    pub(super) fn from_snapshot(
        bytes: &[u8; SERVICE_SECURITY_BYTES],
    ) -> Result<Self, ServiceSecurityError> {
        let regions: DescriptorRegions = DescriptorRegions::parse(bytes)?;
        Ok(Self {
            bytes: *bytes,
            regions,
        })
    }

    /// Return all reported descriptor control bits.
    #[must_use]
    pub const fn control(&self) -> u16 {
        self.regions.control
    }

    /// Borrow the complete owner SID from the admitted snapshot.
    #[must_use]
    pub fn owner_sid(&self) -> &[u8] {
        &self.bytes[self.regions.owner.clone()]
    }

    /// Borrow the DACL observation, preserving absence, null, and ACL bytes.
    #[must_use]
    pub fn dacl(&self) -> ReportedDacl<'_> {
        match &self.regions.dacl {
            DaclRegion::Absent => ReportedDacl::Absent,
            DaclRegion::Null => ReportedDacl::Null,
            DaclRegion::Present(region) => ReportedDacl::Present(&self.bytes[region.clone()]),
        }
    }
}

/// DACL representation in the reported service descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportedDacl<'a> {
    /// The DACL control flag is clear and the DACL offset is zero.
    Absent,
    /// The DACL control flag is set and the DACL offset is zero.
    Null,
    /// The reported ACL bytes, including its header and any declared ACL slack.
    Present(&'a [u8]),
}

/// Reason the native descriptor envelope could not be admitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceSecurityError {
    /// The fixed descriptor header does not fit within the input.
    TruncatedHeader,
    /// The descriptor revision is unsupported.
    UnsupportedRevision,
    /// The descriptor does not use the required relative representation.
    AbsoluteDescriptor,
    /// The mandatory owner offset is outside the payload or is misaligned.
    InvalidOwnerOffset,
    /// The owner SID revision, subauthority count, or extent is invalid.
    InvalidOwnerSid,
    /// The DACL offset contradicts its presence flag or is invalid.
    InvalidDaclOffset,
    /// The ACL revision or declared extent is invalid.
    InvalidAcl,
    /// The owner SID and DACL byte ranges overlap.
    OverlappingRegions,
}

impl fmt::Display for ServiceSecurityError {
    /// Describe the failed envelope check without including descriptor bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message: &str = match self {
            Self::TruncatedHeader => "service security descriptor header is truncated",
            Self::UnsupportedRevision => "unsupported service security descriptor revision",
            Self::AbsoluteDescriptor => "service security descriptor is absolute",
            Self::InvalidOwnerOffset => "invalid service security descriptor owner offset",
            Self::InvalidOwnerSid => "invalid service security descriptor owner SID",
            Self::InvalidDaclOffset => "invalid service security descriptor DACL offset",
            Self::InvalidAcl => "invalid service security descriptor ACL envelope",
            Self::OverlappingRegions => "service security descriptor owner and DACL overlap",
        };
        formatter.write_str(message)
    }
}

impl core::error::Error for ServiceSecurityError {}

/// Positions admitted against one immutable descriptor input.
struct DescriptorRegions {
    /// All reported descriptor control bits, including unrecognized bits.
    control: u16,
    /// Complete owner SID, within the input and disjoint from the DACL.
    owner: Range<usize>,
    /// DACL representation and, when present, its complete admitted range.
    dacl: DaclRegion,
}

/// DACL state and optional byte extent in the descriptor backing storage.
enum DaclRegion {
    /// DACL control flag clear, with zero offset.
    Absent,
    /// DACL control flag set, with zero offset.
    Null,
    /// A DWORD-aligned, bounded ACL region disjoint from the owner.
    Present(Range<usize>),
}

impl DescriptorRegions {
    /// Read the relative header and admit only the owner and DACL regions.
    /// Uses bounded checked reads without allocating or following native pointers.
    /// # Errors
    /// Reject unsupported headers, malformed owner or DACL regions and overlap.
    fn parse(bytes: &[u8]) -> Result<Self, ServiceSecurityError> {
        let header: &[u8; DESCRIPTOR_HEADER_BYTES] = bytes
            .first_chunk::<DESCRIPTOR_HEADER_BYTES>()
            .ok_or(ServiceSecurityError::TruncatedHeader)?;
        if header[0] != 1 {
            return Err(ServiceSecurityError::UnsupportedRevision);
        }
        let control: u16 = u16::from_le_bytes([header[2], header[3]]);
        if control & SE_SELF_RELATIVE == 0 {
            return Err(ServiceSecurityError::AbsoluteDescriptor);
        }

        let owner_offset: u32 = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        let owner: Range<usize> = owner_region(bytes, owner_offset)?;
        let dacl_offset: u32 = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);
        let dacl: DaclRegion = match (control & SE_DACL_PRESENT != 0, dacl_offset) {
            (false, 0) => DaclRegion::Absent,
            (true, 0) => DaclRegion::Null,
            (false, _) => return Err(ServiceSecurityError::InvalidDaclOffset),
            (true, offset) => {
                let region: Range<usize> = acl_region(bytes, offset)?;
                if owner.start < region.end && region.start < owner.end {
                    return Err(ServiceSecurityError::OverlappingRegions);
                }
                DaclRegion::Present(region)
            }
        };
        Ok(Self {
            control,
            owner,
            dacl,
        })
    }
}

/// Convert a relative DWORD offset to an in-bounds position after the header.
fn payload_offset(offset: u32, length: usize) -> Option<usize> {
    let offset: usize = usize::try_from(offset).ok()?;
    ((DESCRIPTOR_HEADER_BYTES..length).contains(&offset) && offset.is_multiple_of(4))
        .then_some(offset)
}

/// Admit the complete revision-one owner SID as a range in the input.
/// # Errors
/// Reject an invalid relative offset or a malformed or truncated owner SID.
fn owner_region(bytes: &[u8], offset: u32) -> Result<Range<usize>, ServiceSecurityError> {
    let start: usize =
        payload_offset(offset, bytes.len()).ok_or(ServiceSecurityError::InvalidOwnerOffset)?;
    let sid: &[u8] = sid_prefix(&bytes[start..]).ok_or(ServiceSecurityError::InvalidOwnerSid)?;
    Ok(start..start + sid.len())
}

/// Borrow one structurally valid revision-one SID from the start of a slice.
pub(super) fn sid_prefix(bytes: &[u8]) -> Option<&[u8]> {
    let header: &[u8; SID_HEADER_BYTES] = bytes.first_chunk()?;
    let count: u8 = header[1];
    if header[0] != 1 || count > SID_MAX_SUBAUTHORITIES {
        return None;
    }
    bytes.get(..SID_HEADER_BYTES + usize::from(count) * 4)
}

/// Admit an aligned ACL envelope using the size declared in its fixed header.
/// # Errors
/// Reject an invalid relative offset or an unsupported or out-of-bounds ACL envelope.
fn acl_region(bytes: &[u8], offset: u32) -> Result<Range<usize>, ServiceSecurityError> {
    let start: usize =
        payload_offset(offset, bytes.len()).ok_or(ServiceSecurityError::InvalidDaclOffset)?;
    let acl: &[u8] = &bytes[start..];
    let header: &[u8; ACL_HEADER_BYTES] = acl
        .first_chunk::<ACL_HEADER_BYTES>()
        .ok_or(ServiceSecurityError::InvalidAcl)?;
    let length: usize = usize::from(u16::from_le_bytes([header[2], header[3]]));
    if !matches!(header[0], 2 | 4) || length < ACL_HEADER_BYTES || !length.is_multiple_of(4) {
        return Err(ServiceSecurityError::InvalidAcl);
    }
    match start.checked_add(length) {
        Some(end) if end <= bytes.len() => Ok(start..end),
        _ => Err(ServiceSecurityError::InvalidAcl),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DescriptorRegions, ReportedDacl, ReportedServiceSecurity, SERVICE_SECURITY_BYTES,
        ServiceSecurityError,
    };

    /// Relative descriptor with S-1-5-18 followed immediately by an empty ACL.
    const OWNER_THEN_DACL: [u8; 40] = [
        1, 0, 4, 0x90, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 5,
        18, 0, 0, 0, 2, 0, 8, 0, 0, 0, 0, 0,
    ];

    /// Relative descriptor with an unknown ACE type before its owner SID.
    const DACL_THEN_OWNER: [u8; 44] = [
        1, 0, 4, 0x80, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 4, 0, 12, 0, 1, 0, 0, 0,
        0xff, 0, 4, 0, 1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0,
    ];

    /// Preserve the selected observations without returning initialized tail bytes.
    /// # Errors
    /// Preserve any unexpected descriptor refusal from a valid fixture.
    /// # Panics
    /// Require the original owner and the distinct absent, null and empty ACL states.
    #[test]
    fn reports_absent_null_and_empty_dacl_separately() -> Result<(), ServiceSecurityError> {
        let owner: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
        let empty_acl: [u8; 8] = [2, 0, 8, 0, 0, 0, 0, 0];
        let cases: [(u8, u8, ReportedDacl<'_>); 3] = [
            (0, 0, ReportedDacl::Absent),
            (4, 0, ReportedDacl::Null),
            (4, 32, ReportedDacl::Present(&empty_acl)),
        ];
        for (flag, offset, expected) in cases {
            let mut snapshot: [u8; SERVICE_SECURITY_BYTES] = [0xa5; SERVICE_SECURITY_BYTES];
            snapshot[..40].copy_from_slice(&OWNER_THEN_DACL);
            snapshot[2] = flag;
            snapshot[16] = offset;
            let report: ReportedServiceSecurity =
                ReportedServiceSecurity::from_snapshot(&snapshot)?;
            assert_eq!(report.control(), 0x9000 | u16::from(flag));
            assert_eq!(report.owner_sid(), owner);
            assert_eq!(report.dacl(), expected);
        }
        Ok(())
    }

    /// Accept adjacent components in either order and keep opaque ACE bytes intact.
    /// # Errors
    /// Preserve an unexpected refusal of the valid revision-four ACL fixture.
    /// # Panics
    /// Require both complete regions and the unchanged opaque ACE bytes.
    #[test]
    fn reports_revision_four_acl_before_owner() -> Result<(), ServiceSecurityError> {
        let mut snapshot: [u8; SERVICE_SECURITY_BYTES] = [0; SERVICE_SECURITY_BYTES];
        snapshot[..44].copy_from_slice(&DACL_THEN_OWNER);
        let report: ReportedServiceSecurity = ReportedServiceSecurity::from_snapshot(&snapshot)?;
        assert_eq!(report.owner_sid(), [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0]);
        assert_eq!(
            report.dacl(),
            ReportedDacl::Present(&[4, 0, 12, 0, 1, 0, 0, 0, 0xff, 0, 4, 0])
        );
        Ok(())
    }

    /// A valid fixture cut inside any selected component fails at that component.
    /// # Panics
    /// Require each truncation to fail with its expected error and the intact fixture to pass.
    #[test]
    fn refuses_truncation_of_each_selected_region() {
        let cuts: [(usize, ServiceSecurityError); 4] = [
            (19, ServiceSecurityError::TruncatedHeader),
            (26, ServiceSecurityError::InvalidOwnerSid),
            (31, ServiceSecurityError::InvalidOwnerSid),
            (39, ServiceSecurityError::InvalidAcl),
        ];
        for (length, expected) in cuts {
            assert_eq!(
                DescriptorRegions::parse(&OWNER_THEN_DACL[..length]).err(),
                Some(expected)
            );
        }
        assert!(DescriptorRegions::parse(&OWNER_THEN_DACL).is_ok());
    }

    /// Reject malformed offsets, revisions, lengths, flags, and a partial overlap.
    /// # Panics
    /// Require each malformed snapshot to return its expected refusal.
    #[test]
    fn refuses_malformed_envelopes_through_snapshot_constructor() {
        let mutations: [(usize, u8, ServiceSecurityError); 14] = [
            (0, 2, ServiceSecurityError::UnsupportedRevision),
            (3, 0x10, ServiceSecurityError::AbsoluteDescriptor),
            (4, 0, ServiceSecurityError::InvalidOwnerOffset),
            (4, 16, ServiceSecurityError::InvalidOwnerOffset),
            (4, 21, ServiceSecurityError::InvalidOwnerOffset),
            (7, 0xff, ServiceSecurityError::InvalidOwnerOffset),
            (2, 0, ServiceSecurityError::InvalidDaclOffset),
            (16, 33, ServiceSecurityError::InvalidDaclOffset),
            (20, 2, ServiceSecurityError::InvalidOwnerSid),
            (21, 16, ServiceSecurityError::InvalidOwnerSid),
            (21, 2, ServiceSecurityError::OverlappingRegions),
            (32, 3, ServiceSecurityError::InvalidAcl),
            (34, 4, ServiceSecurityError::InvalidAcl),
            (34, 9, ServiceSecurityError::InvalidAcl),
        ];
        for (index, value, expected) in mutations {
            let mut snapshot: [u8; SERVICE_SECURITY_BYTES] = [0; SERVICE_SECURITY_BYTES];
            snapshot[..40].copy_from_slice(&OWNER_THEN_DACL);
            snapshot[index] = value;
            assert_eq!(
                ReportedServiceSecurity::from_snapshot(&snapshot).err(),
                Some(expected)
            );
        }
    }
}
