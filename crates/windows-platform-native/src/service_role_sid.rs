// SPDX-License-Identifier: GPL-2.0-only
//! Retained primary-token evidence for one independently selected service SID.

use core::{ffi::c_void, fmt, mem};

use crate::service_security::sid_prefix;
#[cfg(windows)]
use crate::windows_peer_process::sid_address;

/// S-1-5-80 with the five service-specific subauthorities from MS-LSAT §3.1.1.1.1.
const SERVICE_SID_PREFIX: [u8; 12] = [1, 6, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0];
/// The fixed SID header followed by six DWORD subauthorities.
const SERVICE_SID_BYTES: usize = 32;

/// Binary SID supplied by the caller's independent installed-role policy.
///
/// Construction checks the individual SCM service SID representation. The caller
/// owns the expected SID's role and deployment provenance.
#[derive(Clone, Copy, Eq, PartialEq)]
#[must_use]
pub struct ServiceRoleSid<'sid>(&'sid [u8]);

impl<'sid> ServiceRoleSid<'sid> {
    /// Check one exact individual service SID, with no trailing data.
    ///
    /// # Errors
    /// Reject a malformed, truncated, shared-account or aggregate service SID.
    pub fn new(bytes: &'sid [u8]) -> Result<Self, InvalidServiceRoleSid> {
        if bytes.len() == SERVICE_SID_BYTES && bytes.starts_with(&SERVICE_SID_PREFIX) {
            Ok(Self(bytes))
        } else {
            Err(InvalidServiceRoleSid)
        }
    }
}

impl fmt::Debug for ServiceRoleSid<'_> {
    /// Describe the checked type without exposing the SID bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ServiceRoleSid(..)")
    }
}

/// An invalid binary SID supplied for role comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidServiceRoleSid;

impl fmt::Display for InvalidServiceRoleSid {
    /// Describe the rejection without logging SID bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid expected service role SID")
    }
}

impl core::error::Error for InvalidServiceRoleSid {}

/// The expected SID's disposition in the token's normal group entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceRoleSidGroup {
    /// The exact SID was absent.
    Missing,
    /// The SID was present with neither the enabled nor deny-only flag.
    Disabled(u32),
    /// The SID was present with the deny-only flag.
    DenyOnly(u32),
    /// The SID was present with the enabled flag.
    Enabled(u32),
}

/// The expected SID's disposition in the token's restricting SID list.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceRoleSidRestriction {
    /// The returned list had no entries.
    Empty,
    /// The returned list had entries and lacked the expected SID.
    Missing,
    /// The list contained that SID with the given raw attributes.
    Present(u32),
}

/// Observations of the exact SID from two reads within one token generation.
///
/// Present variants preserve the raw attributes. These samples give no
/// object-access decision, installed-peer admission, or continuing authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain the actual group and restricting SID observations"]
pub struct ServiceRoleSidObservation<'sid> {
    /// The exact independently supplied SID compared by both reads.
    expected: ServiceRoleSid<'sid>,
    /// Evidence from the normal group list.
    group: ServiceRoleSidGroup,
    /// Evidence from the restricting SID list.
    restriction: ServiceRoleSidRestriction,
}

impl<'sid> ServiceRoleSidObservation<'sid> {
    /// Return the exact expected SID used for both comparisons.
    pub const fn expected(self) -> ServiceRoleSid<'sid> {
        self.expected
    }

    /// Return the normal group disposition and original attributes if present.
    #[must_use]
    pub const fn group(self) -> ServiceRoleSidGroup {
        self.group
    }

    /// Return the separate restricting-list disposition.
    #[must_use]
    pub const fn restriction(self) -> ServiceRoleSidRestriction {
        self.restriction
    }
}

/// Refusal to report consistent observations from the two token SID lists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ServiceRoleSidObservationError {
    /// The initial token statistics query failed with the immediate Win32 error.
    InitialStatisticsQuery(u32),
    /// The normal group query failed with the immediate Win32 error.
    GroupQuery(u32),
    /// The restricting SID query failed with the immediate Win32 error.
    RestrictingQuery(u32),
    /// The final token statistics query failed with the immediate Win32 error.
    FinalStatisticsQuery(u32),
    /// The initial token statistics response had an invalid extent.
    MalformedInitialStatistics,
    /// The reported normal group data could not be parsed consistently.
    MalformedGroups,
    /// The reported restricting SID data could not be parsed consistently.
    MalformedRestrictingSids,
    /// The final token statistics response had an invalid extent.
    MalformedFinalStatistics,
    /// The token's modification identifier changed during observation.
    TokenChanged,
}

impl fmt::Display for ServiceRoleSidObservationError {
    /// Describe the observation stage without including SID or token data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitialStatisticsQuery(code) => {
                write!(
                    formatter,
                    "query initial peer token statistics failed ({code})"
                )
            }
            Self::GroupQuery(code) => write!(formatter, "query peer token groups failed ({code})"),
            Self::RestrictingQuery(code) => {
                write!(
                    formatter,
                    "query peer token restricting SIDs failed ({code})"
                )
            }
            Self::FinalStatisticsQuery(code) => {
                write!(
                    formatter,
                    "query final peer token statistics failed ({code})"
                )
            }
            Self::MalformedInitialStatistics => {
                formatter.write_str("malformed initial peer token statistics")
            }
            Self::MalformedGroups => formatter.write_str("malformed peer token groups"),
            Self::MalformedRestrictingSids => {
                formatter.write_str("malformed peer token restricting SIDs")
            }
            Self::MalformedFinalStatistics => {
                formatter.write_str("malformed final peer token statistics")
            }
            Self::TokenChanged => formatter.write_str("peer token changed during SID observation"),
        }
    }
}

impl core::error::Error for ServiceRoleSidObservationError {}

/// Each query has one fixed buffer. Oversize results preserve the native refusal.
const SID_LIST_CAPACITY: usize = 16_384;
/// Win32's enabled group attribute.
const GROUP_ENABLED: u32 = 0x0000_0004;
/// Win32's deny-only group attribute.
const GROUP_DENY_ONLY: u32 = 0x0000_0010;

/// Native `SID_AND_ATTRIBUTES` layout, used only for size and offset.
#[repr(C)]
struct SidAndAttributesLayout {
    /// Pointer-sized address of the SID in the returned buffer.
    sid: *const c_void,
    /// Original Win32 group attributes.
    attributes: u32,
}

/// Native `TOKEN_GROUPS` layout up to and including its first entry.
#[repr(C)]
struct TokenGroupsLayout {
    /// Count of inline entries.
    count: u32,
    /// First entry, aligned as in the native structure.
    group: SidAndAttributesLayout,
}

/// Writable, initialized native output with at least the SDK's alignment.
#[repr(C, align(8))]
struct SidListBuffer([u8; SID_LIST_CAPACITY]);

/// Size in bytes of the x64 `TOKEN_STATISTICS` record from the pinned Windows SDK.
const TOKEN_STATISTICS_BYTES: usize = 56;

/// Aligned native output ending in the two 32-bit halves of `ModifiedId`.
#[repr(C, align(8))]
struct TokenStatisticsBuffer([u8; TOKEN_STATISTICS_BYTES]);

/// The full opaque byte representation of the SDK's `ModifiedId` LUID.
type TokenModifiedId = [u8; 8];

/// Token information queried from the same retained handle.
#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(super) enum TokenSidInformation {
    /// Token statistics containing the current modification identifier.
    Statistics,
    /// One of the two native token SID lists.
    List(SidList),
}

/// The two native SID lists accepted by the list parser.
#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(super) enum SidList {
    /// Normal access-token group entries.
    Groups,
    /// Restricting SID entries.
    Restricting,
}

/// Small result of traversing one complete list.
struct SidListMatch {
    /// The original declared and extent-checked entry count.
    count: usize,
    /// The original attributes of the unique exact match, if found.
    attributes: Option<u32>,
}

/// Query both lists in order between modification identifiers from the same token.
///
/// The query writes into the initialized, aligned slice and returns its
/// successful byte extent or the immediate Win32 error. Both complete lists
/// must be valid and both modification identifiers must agree before publication.
///
/// # Errors
/// Retain the precise query stage or reject malformed output or a changed token.
pub(super) fn observe_role_sid(
    expected: ServiceRoleSid<'_>,
    mut query: impl FnMut(TokenSidInformation, &mut [u8], u32) -> Result<u32, u32>,
) -> Result<ServiceRoleSidObservation<'_>, ServiceRoleSidObservationError> {
    let initial_id: TokenModifiedId = read_modified_id(
        &mut query,
        ServiceRoleSidObservationError::InitialStatisticsQuery,
        ServiceRoleSidObservationError::MalformedInitialStatistics,
    )?;
    let groups: SidListMatch = read_list(
        expected,
        SidList::Groups,
        &mut query,
        ServiceRoleSidObservationError::GroupQuery,
        ServiceRoleSidObservationError::MalformedGroups,
    )?;
    let group: ServiceRoleSidGroup = match groups.attributes {
        None => ServiceRoleSidGroup::Missing,
        Some(attributes) if attributes & GROUP_DENY_ONLY != 0 => {
            ServiceRoleSidGroup::DenyOnly(attributes)
        }
        Some(attributes) if attributes & GROUP_ENABLED != 0 => {
            ServiceRoleSidGroup::Enabled(attributes)
        }
        Some(attributes) => ServiceRoleSidGroup::Disabled(attributes),
    };
    let restricting: SidListMatch = read_list(
        expected,
        SidList::Restricting,
        &mut query,
        ServiceRoleSidObservationError::RestrictingQuery,
        ServiceRoleSidObservationError::MalformedRestrictingSids,
    )?;
    let restriction: ServiceRoleSidRestriction = match restricting.attributes {
        Some(attributes) => ServiceRoleSidRestriction::Present(attributes),
        None if restricting.count == 0 => ServiceRoleSidRestriction::Empty,
        None => ServiceRoleSidRestriction::Missing,
    };
    let final_id: TokenModifiedId = read_modified_id(
        &mut query,
        ServiceRoleSidObservationError::FinalStatisticsQuery,
        ServiceRoleSidObservationError::MalformedFinalStatistics,
    )?;
    if initial_id != final_id {
        return Err(ServiceRoleSidObservationError::TokenChanged);
    }
    Ok(ServiceRoleSidObservation {
        expected,
        group,
        restriction,
    })
}

/// Read the full modification identifier from one complete fixed-size statistics record.
/// # Errors
/// Preserve the native refusal or reject an incorrect successful extent.
fn read_modified_id(
    query: &mut impl FnMut(TokenSidInformation, &mut [u8], u32) -> Result<u32, u32>,
    rejected_query: fn(u32) -> ServiceRoleSidObservationError,
    malformed: ServiceRoleSidObservationError,
) -> Result<TokenModifiedId, ServiceRoleSidObservationError> {
    let mut buffer: TokenStatisticsBuffer = TokenStatisticsBuffer([0; TOKEN_STATISTICS_BYTES]);
    let capacity: u32 = u32::try_from(buffer.0.len()).map_err(|_| malformed)?;
    let written: u32 =
        query(TokenSidInformation::Statistics, &mut buffer.0, capacity).map_err(rejected_query)?;
    if written != capacity {
        return Err(malformed);
    }
    buffer.0.last_chunk().copied().ok_or(malformed)
}

/// Read one list into an aligned buffer and admit only its reported extent.
///
/// # Errors
/// Preserve the native refusal or reject malformed data within the reported extent.
fn read_list(
    expected: ServiceRoleSid<'_>,
    list: SidList,
    query: &mut impl FnMut(TokenSidInformation, &mut [u8], u32) -> Result<u32, u32>,
    rejected_query: fn(u32) -> ServiceRoleSidObservationError,
    malformed: ServiceRoleSidObservationError,
) -> Result<SidListMatch, ServiceRoleSidObservationError> {
    let mut buffer: SidListBuffer = SidListBuffer([0; SID_LIST_CAPACITY]);
    let capacity: u32 = u32::try_from(buffer.0.len()).map_err(|_| malformed)?;
    let written: u32 =
        query(TokenSidInformation::List(list), &mut buffer.0, capacity).map_err(rejected_query)?;
    let written: usize = usize::try_from(written).map_err(|_| malformed)?;
    let bytes: &[u8] = buffer.0.get(..written).ok_or(malformed)?;
    match_sid_list(bytes, expected, list).ok_or(malformed)
}

/// Read SDK pointer addresses and index only this slice.
///
/// Header, table extent, pointer offset and SID extent are checked in that
/// order. SID storage is accessed only by checked indexing. Repeated matches
/// with identical attributes denote the same observation. Conflicting matches
/// or contradictory normal-group flags make the result ambiguous.
fn match_sid_list(
    bytes: &[u8],
    expected: ServiceRoleSid<'_>,
    list: SidList,
) -> Option<SidListMatch> {
    let count: usize = usize::try_from(u32::from_le_bytes(*bytes.first_chunk()?)).ok()?;
    if count == 0 {
        return Some(SidListMatch {
            count,
            attributes: None,
        });
    }
    let table_start: usize = mem::offset_of!(TokenGroupsLayout, group);
    let stride: usize = size_of::<SidAndAttributesLayout>();
    let table_end: usize = table_start.checked_add(count.checked_mul(stride)?)?;
    let table: &[u8] = bytes.get(table_start..table_end)?;
    let mut matched: Option<u32> = None;
    for entry in table.chunks_exact(stride) {
        let address: usize = sid_address(entry)?;
        let offset: usize = address.checked_sub(bytes.as_ptr().addr())?;
        if offset < table_end || !offset.is_multiple_of(align_of::<u32>()) {
            return None;
        }
        let sid: &[u8] = sid_prefix(bytes.get(offset..)?)?;
        let attributes: u32 = u32::from_le_bytes(
            *entry[mem::offset_of!(SidAndAttributesLayout, attributes)..].first_chunk()?,
        );
        let contradictory: u32 = GROUP_ENABLED | GROUP_DENY_ONLY;
        if matches!(list, SidList::Groups) && attributes & contradictory == contradictory {
            return None;
        }
        if sid == expected.0 {
            if matched.is_some_and(|previous: u32| previous != attributes) {
                return None;
            }
            matched = Some(attributes);
        }
    }
    Some(SidListMatch {
        count,
        attributes: matched,
    })
}

/// Read the address published by an independent x64 layout fixture.
#[cfg(all(not(windows), test))]
fn sid_address(entry: &[u8]) -> Option<usize> {
    Some(usize::from_le_bytes(*entry.first_chunk()?))
}

#[cfg(test)]
mod tests {
    use core::ops::Range;
    use std::error::Error;

    use super::{
        ServiceRoleSid, ServiceRoleSidGroup, ServiceRoleSidObservation,
        ServiceRoleSidObservationError, ServiceRoleSidRestriction, SidList, TokenSidInformation,
        observe_role_sid,
    };

    /// One structurally valid role SID used as the expected fixture SID.
    const EXPECTED: [u8; 32] = [
        1, 6, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0,
        0, 0,
    ];
    /// A different SID, with a byte-for-byte matching header and extent.
    const OTHER: [u8; 32] = [
        1, 6, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 5, 0,
        0, 0,
    ];
    /// A group shared by both role fixtures.
    const COMMON: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 19, 0, 0, 0];
    /// Initial token modification identifier, with distinct nonzero halves.
    const MODIFIED_ID: [u8; 8] = [0x42, 1, 2, 3, 0x81, 4, 5, 6];
    /// Native queries expected on the retained handle, in order.
    const QUERY_ORDER: [TokenSidInformation; 4] = [
        TokenSidInformation::Statistics,
        TokenSidInformation::List(SidList::Groups),
        TokenSidInformation::List(SidList::Restricting),
        TokenSidInformation::Statistics,
    ];

    /// Write an independent SDK-shaped statistics record with both LUID halves.
    /// # Panics
    /// Fail if the observer did not supply the exact aligned native output.
    fn write_statistics(output: &mut [u8], modified_id: [u8; 8]) -> u32 {
        assert_eq!(output.len(), 56);
        assert_eq!(output.as_ptr().addr() % 8, 0);
        output[48..56].copy_from_slice(&modified_id);
        56
    }

    /// Write an independent x64 SDK-shaped output, including original buffer addresses.
    /// # Errors
    /// Return a fixture error for an unrepresentable group count or extent.
    /// # Panics
    /// Fail if the observer did not align the native output.
    fn write_list(output: &mut [u8], groups: &[(&[u8], u32)]) -> Result<u32, u32> {
        let count: [u8; 4] = u32::try_from(groups.len())
            .map_err(|_| 1_u32)?
            .to_le_bytes();
        output[..4].copy_from_slice(&count);
        assert_eq!(output.as_ptr().addr() % 8, 0);
        if groups.is_empty() {
            return Ok(4);
        }
        let sid_start: usize = 8 + 16 * groups.len();
        let mut sid_offset: usize = sid_start;
        for (index, (sid, attributes)) in groups.iter().enumerate() {
            let record: usize = 8 + 16 * index;
            let address: [u8; 8] = (output.as_ptr().addr() + sid_offset).to_le_bytes();
            output[record..record + 8].copy_from_slice(&address);
            output[record + 8..record + 12].copy_from_slice(&attributes.to_le_bytes());
            output[sid_offset..sid_offset + sid.len()].copy_from_slice(sid);
            sid_offset += sid.len();
        }
        u32::try_from(sid_offset).map_err(|_| 1_u32)
    }

    /// Send both independent lists through the same capture and interpretation path.
    /// # Errors
    /// Return a failed or malformed sample.
    /// # Panics
    /// Fail if the observer reports a different capacity from its writable slice.
    fn read<'sid>(
        expected: ServiceRoleSid<'sid>,
        groups: &[(&[u8], u32)],
        restricting: &[(&[u8], u32)],
    ) -> Result<ServiceRoleSidObservation<'sid>, ServiceRoleSidObservationError> {
        observe_role_sid(expected, |information, output, capacity| {
            assert_eq!(Some(output.len()), usize::try_from(capacity).ok());
            match information {
                TokenSidInformation::Statistics => Ok(write_statistics(output, MODIFIED_ID)),
                TokenSidInformation::List(SidList::Groups) => write_list(output, groups),
                TokenSidInformation::List(SidList::Restricting) => write_list(output, restricting),
            }
        })
    }

    /// Only a whole, supported SID can become the expected role identity.
    /// # Panics
    /// Fail if a complete healthy SID is refused or a malformed SID is accepted.
    #[test]
    fn validates_expected_sid_extent() {
        assert!(ServiceRoleSid::new(&EXPECTED).is_ok());
        assert!(ServiceRoleSid::new(&EXPECTED[..31]).is_err());
        assert!(
            ServiceRoleSid::new(&COMMON).is_err(),
            "a shared account SID cannot name an individual service"
        );
        let world: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let all_services: [u8; 16] = [1, 2, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0, 0, 0, 0, 0];
        assert!(ServiceRoleSid::new(&world).is_err());
        assert!(ServiceRoleSid::new(&all_services).is_err());
        assert!(ServiceRoleSid::new(&[1, 0, 0, 0, 0, 0, 0, 5, 0]).is_err());
        let mut bad: [u8; 32] = EXPECTED;
        bad[0] = 2;
        assert!(ServiceRoleSid::new(&bad).is_err());
        bad[0] = 1;
        bad[1] = 16;
        assert!(ServiceRoleSid::new(&bad).is_err());
    }

    /// Distinguish exact role membership from common or deny-only groups.
    /// # Errors
    /// Return an unexpected refusal of a healthy control.
    /// # Panics
    /// Fail if the observed outcome differs from the independently expected one.
    #[test]
    fn observes_exact_role_and_original_group_flags() -> Result<(), Box<dyn Error>> {
        let role: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        for (sid, flags, expected) in [
            (&EXPECTED, 0x0000_0005, ServiceRoleSidGroup::Enabled(5)),
            (&EXPECTED, 0x0000_0010, ServiceRoleSidGroup::DenyOnly(16)),
            (&EXPECTED, 0x0000_0002, ServiceRoleSidGroup::Disabled(2)),
            (&OTHER, 0x0000_0005, ServiceRoleSidGroup::Missing),
        ] {
            let outcome: ServiceRoleSidObservation<'_> =
                read(role, &[(&COMMON, 5), (sid, flags)], &[])?;
            assert_eq!(outcome.expected(), role);
            assert_eq!(outcome.group(), expected);
            assert_eq!(outcome.restriction(), ServiceRoleSidRestriction::Empty);
        }
        assert_eq!(
            read(role, &[(&EXPECTED, 0x14)], &[]),
            Err(ServiceRoleSidObservationError::MalformedGroups)
        );
        Ok(())
    }

    /// Restricting-list evidence stays separate from normal access-group attributes.
    /// # Errors
    /// Return an unexpected refusal of a healthy control.
    /// # Panics
    /// Fail if the observed outcome differs from the independently expected one.
    #[test]
    fn distinguishes_empty_missing_and_present_restricting_sid() -> Result<(), Box<dyn Error>> {
        let role: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        let present: ServiceRoleSidObservation<'_> =
            read(role, &[(&EXPECTED, 4)], &[(&EXPECTED, 0)])?;
        assert_eq!(present.group(), ServiceRoleSidGroup::Enabled(4));
        assert_eq!(present.restriction(), ServiceRoleSidRestriction::Present(0));
        assert_eq!(
            read(role, &[(&EXPECTED, 4)], &[(&OTHER, 0)])?.restriction(),
            ServiceRoleSidRestriction::Missing
        );
        assert_eq!(
            read(role, &[(&OTHER, 4)], &[(&EXPECTED, 0)])?.group(),
            ServiceRoleSidGroup::Missing
        );
        Ok(())
    }

    /// A redundant restricting SID with identical attributes still identifies the same group.
    /// # Errors
    /// Return an unexpected fixture construction failure.
    /// # Panics
    /// Fail on refusal of an allowed duplicate or a changed restricting observation.
    #[test]
    fn observes_agreeing_duplicates_in_restricting_list() -> Result<(), Box<dyn Error>> {
        let role: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        let observed: Result<ServiceRoleSidObservation<'_>, ServiceRoleSidObservationError> =
            read(role, &[(&EXPECTED, 4)], &[(&EXPECTED, 0), (&EXPECTED, 0)]);
        assert!(
            matches!(observed, Ok(value) if value.restriction() == ServiceRoleSidRestriction::Present(0)),
            "agreeing duplicates are a supported token list: {observed:?}"
        );
        Ok(())
    }

    /// Refuse either change within the bracket and accept the unchanged neighbor.
    /// # Errors
    /// Return an unexpected fixture or healthy observation failure.
    /// # Panics
    /// Fail on a changed query order, extent or observation.
    #[test]
    fn brackets_both_sid_lists_with_the_complete_modified_id() -> Result<(), Box<dyn Error>> {
        let expected: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        for change_after in [None, Some((1_usize, 0_usize)), Some((2, 4))] {
            let mut call: usize = 0;
            let mut modified_id: [u8; 8] = MODIFIED_ID;
            let observed: Result<ServiceRoleSidObservation<'_>, ServiceRoleSidObservationError> =
                observe_role_sid(expected, |information, output, capacity| {
                    assert_eq!(QUERY_ORDER.get(call), Some(&information));
                    let result: Result<u32, u32> = match information {
                        TokenSidInformation::Statistics => {
                            assert_eq!(capacity, 56);
                            Ok(write_statistics(output, modified_id))
                        }
                        TokenSidInformation::List(SidList::Groups) => {
                            write_list(output, &[(&EXPECTED, 4)])
                        }
                        TokenSidInformation::List(SidList::Restricting) => {
                            write_list(output, &[(&EXPECTED, 0)])
                        }
                    };
                    if let Some((read, changed_byte)) = change_after
                        && call == read
                    {
                        modified_id[changed_byte] ^= 0xff;
                    }
                    call += 1;
                    result
                });
            assert_eq!(call, QUERY_ORDER.len());
            if change_after.is_none() {
                let observed: ServiceRoleSidObservation<'_> = observed?;
                assert_eq!(observed.expected(), expected);
                assert_eq!(observed.group(), ServiceRoleSidGroup::Enabled(4));
                assert_eq!(
                    observed.restriction(),
                    ServiceRoleSidRestriction::Present(0)
                );
            } else {
                assert_eq!(observed, Err(ServiceRoleSidObservationError::TokenChanged));
            }
        }
        Ok(())
    }

    /// Preserve the first refusal and stop issuing queries after it.
    /// # Errors
    /// Return an unexpected fixture construction failure.
    /// # Panics
    /// Fail on a changed query order or failure, or a query after the first refusal.
    #[test]
    fn retains_query_failure_order() -> Result<(), Box<dyn Error>> {
        let expected: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        for (fault_at, refusal, outcome) in [
            (
                0,
                Err(5),
                ServiceRoleSidObservationError::InitialStatisticsQuery(5),
            ),
            (
                0,
                Ok(55),
                ServiceRoleSidObservationError::MalformedInitialStatistics,
            ),
            (
                0,
                Ok(57),
                ServiceRoleSidObservationError::MalformedInitialStatistics,
            ),
            (1, Err(5), ServiceRoleSidObservationError::GroupQuery(5)),
            (
                2,
                Err(122),
                ServiceRoleSidObservationError::RestrictingQuery(122),
            ),
            (
                3,
                Err(5),
                ServiceRoleSidObservationError::FinalStatisticsQuery(5),
            ),
            (
                3,
                Ok(55),
                ServiceRoleSidObservationError::MalformedFinalStatistics,
            ),
            (
                3,
                Ok(57),
                ServiceRoleSidObservationError::MalformedFinalStatistics,
            ),
        ] {
            let mut call: usize = 0;
            let observed: Result<ServiceRoleSidObservation<'_>, ServiceRoleSidObservationError> =
                observe_role_sid(expected, |information, output, _| {
                    assert_eq!(QUERY_ORDER.get(call), Some(&information));
                    let current_call: usize = call;
                    call += 1;
                    if current_call == fault_at {
                        return refusal;
                    }
                    match information {
                        TokenSidInformation::Statistics => {
                            Ok(write_statistics(output, MODIFIED_ID))
                        }
                        TokenSidInformation::List(SidList::Groups) => {
                            write_list(output, &[(&EXPECTED, 4)])
                        }
                        TokenSidInformation::List(SidList::Restricting) => write_list(output, &[]),
                    }
                });
            assert_eq!(observed, Err(outcome));
            assert_eq!(call, fault_at + 1);
        }
        Ok(())
    }

    /// Refuse an impossible extent or pointer even after the expected SID matched.
    /// # Errors
    /// Return an unexpected refusal of a healthy control.
    /// # Panics
    /// Fail if the observed outcome differs from the independently expected one.
    #[test]
    fn rejects_malformed_group_buffers_before_publication() -> Result<(), Box<dyn Error>> {
        let expected: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        for malformed in 0..9 {
            let result: Result<ServiceRoleSidObservation<'_>, ServiceRoleSidObservationError> =
                observe_role_sid(expected, |information, output, capacity| {
                    if matches!(information, TokenSidInformation::Statistics) {
                        return Ok(write_statistics(output, MODIFIED_ID));
                    }
                    assert_eq!(information, TokenSidInformation::List(SidList::Groups));
                    let written: u32 = write_list(output, &[(&EXPECTED, 4), (&OTHER, 4)])?;
                    let base: usize = output.as_ptr().addr();
                    let second_entry: Range<usize> = 24..32;
                    match malformed {
                        0 => return Ok(3),
                        1 => return Ok(capacity + 1),
                        2 => return Ok(39),
                        3 => return Ok(written - 1),
                        4 => output[second_entry].copy_from_slice(&(base - 4).to_le_bytes()),
                        5 => output[second_entry].copy_from_slice(&(base + 8).to_le_bytes()),
                        6 => output[second_entry].copy_from_slice(&(base + 53).to_le_bytes()),
                        7 => output[second_entry].copy_from_slice(
                            &(base + usize::try_from(written).map_err(|_| 1_u32)?).to_le_bytes(),
                        ),
                        8 => {
                            output[second_entry].copy_from_slice(&(base + 40).to_le_bytes());
                            output[32..36].copy_from_slice(&2_u32.to_le_bytes());
                        }
                        _ => return Err(1),
                    }
                    Ok(written)
                });
            assert_eq!(result, Err(ServiceRoleSidObservationError::MalformedGroups));
        }
        let restored: ServiceRoleSidObservation<'_> =
            read(expected, &[(&EXPECTED, 4), (&OTHER, 4)], &[])?;
        assert_eq!(restored.group(), ServiceRoleSidGroup::Enabled(4));
        Ok(())
    }

    /// A good first list supplies no partial result after malformed restricting data.
    /// # Errors
    /// Return an unexpected refusal of a healthy control.
    /// # Panics
    /// Fail if the observed outcome differs from the independently expected one.
    #[test]
    fn rejects_malformed_restricting_list() -> Result<(), Box<dyn Error>> {
        let result: Result<ServiceRoleSidObservation<'_>, ServiceRoleSidObservationError> = read(
            ServiceRoleSid::new(&EXPECTED)?,
            &[(&EXPECTED, 4)],
            &[(&EXPECTED, 0), (&EXPECTED, 4)],
        );
        assert_eq!(
            result,
            Err(ServiceRoleSidObservationError::MalformedRestrictingSids)
        );
        Ok(())
    }

    /// Bracket both actual SID lists of the ordinary test process's primary token.
    /// # Errors
    /// Preserve any failure to bind or query the original token.
    /// # Panics
    /// Fail if a foreign fixture SID appears in either list.
    #[cfg(windows)]
    #[test]
    fn native_token_observation_refuses_foreign_service_identity() -> Result<(), Box<dyn Error>> {
        use crate::{PeerPrimaryToken, PeerProcess};
        use core::num::NonZeroU32;

        let pid: NonZeroU32 = NonZeroU32::new(std::process::id()).ok_or("zero test process ID")?;
        let process: PeerProcess = PeerProcess::open(pid)?;
        let token: PeerPrimaryToken = process.retain_primary_token()?;
        let expected: ServiceRoleSid<'_> = ServiceRoleSid::new(&EXPECTED)?;
        let observed: ServiceRoleSidObservation<'_> = token.observe_role_sid(expected)?;
        assert_eq!(observed.group(), ServiceRoleSidGroup::Missing);
        assert!(matches!(
            observed.restriction(),
            ServiceRoleSidRestriction::Empty | ServiceRoleSidRestriction::Missing
        ));
        Ok(())
    }
}
