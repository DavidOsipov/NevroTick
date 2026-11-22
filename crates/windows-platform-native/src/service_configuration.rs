// SPDX-License-Identifier: GPL-2.0-only
//! Bounded wide strings from a local SCM configuration result.

use core::fmt;

/// Reason a reported configuration string could not be read inside its buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationStringError {
    /// The address falls before or outside the trailing output buffer.
    OutsideBuffer,
    /// The address does not identify the start of a wide code unit.
    UnalignedAddress,
    /// The required zero terminator is absent before the end of the buffer.
    MissingTerminator,
}

impl fmt::Display for ConfigurationStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message: &str = match self {
            Self::OutsideBuffer => "configuration string address is outside the output buffer",
            Self::UnalignedAddress => "configuration string address is not wide-unit aligned",
            Self::MissingTerminator => {
                "configuration string has no terminator in the output buffer"
            }
        };
        formatter.write_str(message)
    }
}

impl core::error::Error for ConfigurationStringError {}

/// Read the literal wide units up to the first zero, excluding that terminator.
///
/// `trailing_address` is the address of the first unit of `trailing` in the
/// original output buffer. Addresses are used only to calculate a slice index.
/// A null `address` represents an absent field, distinct from an empty string.
///
/// # Errors
/// Reject addresses outside the trailing region or between its code units, and
/// strings without a terminator inside that region.
pub(super) fn configured_string(
    trailing: &[u16],
    trailing_address: usize,
    address: usize,
) -> Result<Option<&[u16]>, ConfigurationStringError> {
    if address == 0 {
        return Ok(None);
    }
    let units: &[u16] = configured_wide_tail(trailing, trailing_address, address)?;
    let end: usize = units
        .iter()
        .position(|&unit| unit == 0)
        .ok_or(ConfigurationStringError::MissingTerminator)?;
    Ok(Some(&units[..end]))
}

/// Preserve one literal `MULTI_SZ`, including its terminating empty element.
///
/// A null address represents absence. A present list can be explicitly empty.
/// Nonempty lists end with a name terminator followed by the empty element.
/// The initialized tail is scratch, so bytes after the list carry no meaning.
///
/// # Errors
/// Reject addresses outside the trailing region or between its code units, and
/// lists without a terminating empty element inside that region.
pub(super) fn configured_multistring(
    trailing: &[u16],
    trailing_address: usize,
    address: usize,
) -> Result<Option<&[u16]>, ConfigurationStringError> {
    if address == 0 {
        return Ok(None);
    }
    let units: &[u16] = configured_wide_tail(trailing, trailing_address, address)?;
    if let [0, ..] = units {
        return Ok(Some(&units[..1]));
    }
    let end: usize = units
        .windows(2)
        .position(|pair| pair == [0, 0])
        .ok_or(ConfigurationStringError::MissingTerminator)?;
    Ok(Some(&units[..end + 2]))
}

/// Bind a nonnull native address to an aligned, nonempty original buffer tail.
/// The caller owns the original output buffer and handles null separately.
///
/// # Errors
/// Reject addresses outside the supplied tail or between its code units.
fn configured_wide_tail(
    trailing: &[u16],
    trailing_address: usize,
    address: usize,
) -> Result<&[u16], ConfigurationStringError> {
    let byte_offset: usize = address
        .checked_sub(trailing_address)
        .ok_or(ConfigurationStringError::OutsideBuffer)?;
    let unit_bytes: usize = size_of::<u16>();
    let unit_index: usize = byte_offset / unit_bytes;
    let units: &[u16] = trailing
        .get(unit_index..)
        .filter(|units| !units.is_empty())
        .ok_or(ConfigurationStringError::OutsideBuffer)?;
    if !byte_offset.is_multiple_of(unit_bytes) {
        return Err(ConfigurationStringError::UnalignedAddress);
    }
    Ok(units)
}

#[cfg(test)]
mod tests {
    //! Independent byte-address and raw code-unit controls for the slice reader.

    use super::{ConfigurationStringError, configured_multistring, configured_string};

    /// All malformed-output causes render fixed diagnostics with no returned data.
    ///
    /// # Panics
    /// Panics if a diagnostic loses its cause or changes its fixed text.
    #[test]
    fn renders_fixed_configuration_string_errors() {
        let expected: [(ConfigurationStringError, &str); 3] = [
            (
                ConfigurationStringError::OutsideBuffer,
                "configuration string address is outside the output buffer",
            ),
            (
                ConfigurationStringError::UnalignedAddress,
                "configuration string address is not wide-unit aligned",
            ),
            (
                ConfigurationStringError::MissingTerminator,
                "configuration string has no terminator in the output buffer",
            ),
        ];
        for (cause, message) in expected {
            assert_eq!(cause.to_string(), message);
        }
    }

    /// Read from a nonzero offset and retain an unpaired surrogate literally.
    /// # Panics
    /// Panics if the reader changes the original units or crosses the first zero.
    #[test]
    fn retains_exact_units_until_first_nul() {
        let trailing: [u16; 5] = [0x0061, 0xd800, 0xffff, 0, 0x0062];
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x1002),
            Ok(Some(&[0xd800, 0xffff][..]))
        );
    }

    /// Observe a null pointer and the final, empty string separately.
    /// # Panics
    /// Panics if absent and empty fields are indistinguishable.
    #[test]
    fn distinguishes_absent_and_empty() {
        let trailing: [u16; 2] = [0x0061, 0];
        assert_eq!(configured_string(&trailing, 0x1000, 0), Ok(None));
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x1002),
            Ok(Some(&[][..]))
        );
    }

    /// Admit the first unit and reject the aligned address just below it.
    /// # Panics
    /// Panics if the reader misclassifies either address.
    #[test]
    fn rejects_address_before_buffer() {
        let trailing: [u16; 2] = [0x0061, 0];
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x1000),
            Ok(Some(&[0x0061][..]))
        );
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x0ffe),
            Err(ConfigurationStringError::OutsideBuffer)
        );
    }

    /// Refuse the address immediately beyond the last unit.
    /// # Panics
    /// Panics if the reader fails to report an out-of-bounds address.
    #[test]
    fn rejects_one_past_end() {
        let trailing: [u16; 2] = [0x0061, 0];
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x1004),
            Err(ConfigurationStringError::OutsideBuffer)
        );
    }

    /// Refuse the second byte of an otherwise in-bounds unit.
    /// # Panics
    /// Panics if the reader fails to report a misaligned address.
    #[test]
    fn rejects_middle_of_unit() {
        let trailing: [u16; 2] = [0x0061, 0];
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x1001),
            Err(ConfigurationStringError::UnalignedAddress)
        );
    }

    /// A zero before the selected address cannot terminate the selected string.
    /// # Panics
    /// Panics if the reader fails to report the missing terminator.
    #[test]
    fn rejects_missing_terminator() {
        let trailing: [u16; 2] = [0, 0x0061];
        assert_eq!(
            configured_string(&trailing, 0x1000, 0x1002),
            Err(ConfigurationStringError::MissingTerminator)
        );
    }

    /// The second zero ends the list after the second unmodified name.
    ///
    /// # Panics
    /// Panics if a literal code unit, separator or the final empty name is lost.
    #[test]
    fn retains_exact_multistring_units_inside_scratch() {
        let trailing: [u16; 9] = [0, 0x0041, 0xd800, 0, 0x0042, 0x0042, 0, 0, 0xffff];
        assert_eq!(
            configured_multistring(&trailing, 0x1000, 0x1002),
            Ok(Some(&[0x0041, 0xd800, 0, 0x0042, 0x0042, 0, 0][..]))
        );
    }

    /// A null field differs from an explicit empty list at the buffer edge.
    ///
    /// # Panics
    /// Panics when the two representations collapse or a one-past address is read.
    #[test]
    fn distinguishes_absent_empty_and_outside_multistrings() {
        let trailing: [u16; 2] = [0xffff, 0];
        assert_eq!(configured_multistring(&trailing, 0x1000, 0), Ok(None));
        assert_eq!(
            configured_multistring(&trailing, 0x1000, 0x1002),
            Ok(Some(&[0][..]))
        );
        assert_eq!(
            configured_multistring(&trailing, 0x1000, 0x1004),
            Err(ConfigurationStringError::OutsideBuffer)
        );
    }

    /// One final zero terminates a name without terminating the nonempty list.
    ///
    /// # Panics
    /// Panics if an adjacent tail supplies an invented terminator or alignment.
    #[test]
    fn rejects_unterminated_or_unaligned_multistrings() {
        let trailing: [u16; 4] = [0x0041, 0, 0x0042, 0];
        assert_eq!(
            configured_multistring(&trailing, 0x1000, 0x1000),
            Err(ConfigurationStringError::MissingTerminator)
        );
        assert_eq!(
            configured_multistring(&trailing, 0x1000, 0x1001),
            Err(ConfigurationStringError::UnalignedAddress)
        );
    }

    /// The 8 KiB ABI output has room for 4,092 wide units after its pointer.
    ///
    /// # Panics
    /// Panics if an exact last-unit terminator is lost or an incomplete neighbor passes.
    #[test]
    fn accepts_final_unit_of_bounded_multistring() {
        let mut trailing: [u16; 4_092] = [0x0041; 4_092];
        trailing[4_090..].fill(0);
        assert_eq!(
            configured_multistring(&trailing, 0x1000, 0x1000),
            Ok(Some(trailing.as_slice()))
        );
        assert_eq!(
            configured_multistring(&trailing[..4_091], 0x1000, 0x1000),
            Err(ConfigurationStringError::MissingTerminator)
        );
    }
}
