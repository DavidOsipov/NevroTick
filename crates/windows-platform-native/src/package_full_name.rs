// SPDX-License-Identifier: GPL-2.0-only
//! Bounded readback of the package full name returned for a retained process.

use core::fmt;

/// SDK full-name maximum plus one terminating wide unit.
pub(super) const PACKAGE_NAME_BUFFER: usize = 128;
/// Detect writable slots the native call did not fill on success.
pub(super) const UNWRITTEN_NAME_UNIT: u16 = u16::MAX;
/// SDK full-name minimum plus one terminating wide unit.
const MIN_TERMINATED_NAME: usize = 31;
/// The `AppModel` result for a process with no package identity.
const APPMODEL_ERROR_NO_PACKAGE: i32 = 15_700;

/// Failure to observe the process package full name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageFullNameError {
    /// The query returned this status directly.
    Native(i32),
    /// Success returned an invalid length, terminator or string.
    Malformed,
}

impl fmt::Display for PackageFullNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native(code) => write!(formatter, "GetPackageFullName failed ({code})"),
            Self::Malformed => formatter.write_str("invalid reported package full name"),
        }
    }
}

impl core::error::Error for PackageFullNameError {}

/// Literal, bounded package identity reported by the OS for a process.
///
/// A receiving owner must establish current peer and approved release identity.
pub struct ReportedPackageFullName {
    /// Initialized storage containing the literal native output.
    units: [u16; PACKAGE_NAME_BUFFER],
    /// Checked number of name units before the required terminator.
    length: usize,
}

impl ReportedPackageFullName {
    /// Borrow the reported name in UTF-16, excluding its terminator.
    #[must_use]
    pub fn as_wide(&self) -> &[u16] {
        &self.units[..self.length]
    }
}

/// Interpret one returned `AppModel` status and its bounded native output.
///
/// # Errors
/// Retain any other native status or reject a malformed success result.
pub(super) fn decode_package_full_name(
    status: i32,
    count: u32,
    units: [u16; PACKAGE_NAME_BUFFER],
) -> Result<Option<ReportedPackageFullName>, PackageFullNameError> {
    match status {
        0 => (),
        APPMODEL_ERROR_NO_PACKAGE => return Ok(None),
        other => return Err(PackageFullNameError::Native(other)),
    }

    let count: usize = usize::try_from(count).map_err(|_| PackageFullNameError::Malformed)?;
    if !(MIN_TERMINATED_NAME..=PACKAGE_NAME_BUFFER).contains(&count) {
        return Err(PackageFullNameError::Malformed);
    }
    let length: usize = count - 1;
    let name: &[u16] = &units[..length];
    if units[length] != 0
        || name.contains(&0)
        || name.contains(&UNWRITTEN_NAME_UNIT)
        || char::decode_utf16(name.iter().copied())
            .any(|unit: Result<char, core::char::DecodeUtf16Error>| unit.is_err())
    {
        return Err(PackageFullNameError::Malformed);
    }
    Ok(Some(ReportedPackageFullName { units, length }))
}

#[cfg(test)]
mod tests {
    use super::{
        PACKAGE_NAME_BUFFER, PackageFullNameError, ReportedPackageFullName, UNWRITTEN_NAME_UNIT,
        decode_package_full_name,
    };

    /// Keep the documented inclusive lengths and exact result bytes.
    ///
    /// # Panics
    /// Panics if the native-sized fixture or observed result differs.
    #[test]
    fn preserves_literal_name_and_both_length_boundaries() {
        let expected: Vec<u16> = "abc_0.0.0.0_x64__0123456789abc".encode_utf16().collect();
        assert_eq!(expected.len(), 30);
        let mut minimum: [u16; PACKAGE_NAME_BUFFER] = [0xffff; PACKAGE_NAME_BUFFER];
        minimum[..30].copy_from_slice(&expected);
        minimum[30] = 0;
        let mut maximum: [u16; PACKAGE_NAME_BUFFER] = [b'a'.into(); PACKAGE_NAME_BUFFER];
        maximum[127] = 0;
        let cases: [(u32, [u16; PACKAGE_NAME_BUFFER], &[u16]); 2] =
            [(31, minimum, &expected), (128, maximum, &maximum[..127])];
        for (count, units, expected) in cases {
            let observed: Result<Option<ReportedPackageFullName>, PackageFullNameError> =
                decode_package_full_name(0, count, units);
            assert_eq!(
                observed
                    .ok()
                    .flatten()
                    .as_ref()
                    .map(ReportedPackageFullName::as_wide),
                Some(expected)
            );
        }
    }

    /// Success must describe one well-formed, terminated string inside the buffer.
    ///
    /// # Panics
    /// Panics if a malformed value is admitted or the restored neighbor fails.
    #[test]
    fn refuses_invalid_length_terminator_or_contents() {
        let mut too_short: [u16; PACKAGE_NAME_BUFFER] = [b'a'.into(); PACKAGE_NAME_BUFFER];
        too_short[29] = 0;
        for count in [0_u32, 30, 129, u32::MAX] {
            assert!(matches!(
                decode_package_full_name(0, count, too_short),
                Err(PackageFullNameError::Malformed)
            ));
        }
        let mut no_terminator: [u16; PACKAGE_NAME_BUFFER] = [b'a'.into(); PACKAGE_NAME_BUFFER];
        let mut embedded_nul: [u16; PACKAGE_NAME_BUFFER] = no_terminator;
        embedded_nul[8] = 0;
        embedded_nul[127] = 0;
        let mut bad_utf16: [u16; PACKAGE_NAME_BUFFER] = embedded_nul;
        bad_utf16[8] = 0xd800;
        let mut underfilled: [u16; PACKAGE_NAME_BUFFER] = bad_utf16;
        underfilled[8] = UNWRITTEN_NAME_UNIT;
        for units in [no_terminator, embedded_nul, bad_utf16, underfilled] {
            assert!(matches!(
                decode_package_full_name(0, 128, units),
                Err(PackageFullNameError::Malformed)
            ));
        }
        no_terminator[127] = 0;
        assert!(matches!(
            decode_package_full_name(0, 128, no_terminator),
            Ok(Some(_))
        ));
    }

    /// Only the exact no-package status denotes absence.
    ///
    /// # Panics
    /// Panics if a native query status changes meaning.
    #[test]
    fn preserves_absence_and_other_native_codes() {
        let buffer: [u16; PACKAGE_NAME_BUFFER] = [0xffff; PACKAGE_NAME_BUFFER];
        assert!(matches!(
            decode_package_full_name(15_700, u32::MAX, buffer),
            Ok(None)
        ));
        for status in [5_i32, 122, 15_701, -1] {
            assert!(matches!(
                decode_package_full_name(status, 0, buffer),
                Err(PackageFullNameError::Native(code)) if code == status
            ));
        }
    }
}
