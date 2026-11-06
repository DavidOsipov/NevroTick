// SPDX-License-Identifier: GPL-3.0-only
// Original NevroTick bounded-ingress hardening additions.
// Existing AccessKit/Chromium grants and notices remain unchanged in other files.

//! Bounded UIA value admission, independent of Windows pointers and action types.
//!
//! The adapter supplies a code-owned indexed reader. At most1025 unit reads admit
//! a NUL-terminated payload of at most1024 UTF-16 units. No unbounded pointer length
//! operation, lossy replacement, truncation or raw-data error is used. A successful
//! conversion establishes encoding/budget only, never GUI field/context authority.
#![forbid(unsafe_code)]

/// Maximum non-terminator UTF-16 units admitted by the local UI adapter.
pub(crate) const MAX_VALUE_UNITS: usize = 1024;

/// Closed, redacted value-admission rejection; no raw string or pointer is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ValueInputError {
    /// No native pointer/reader was supplied.
    Null,
    /// The readable input ended without an in-budget terminator.
    Unterminated,
    /// The terminator is beyond the1024-unit payload budget.
    TooLong,
    /// The admitted finite prefix contains an unpaired UTF-16 surrogate.
    InvalidUtf16,
    /// A numeric value is NaN or positive/negative infinity.
    NonFinite,
    /// A movement magnitude cannot be represented by the adapter's positive i32.
    InvalidCount,
}

/// Admit a finite prefix and invoke request construction only after decoding.
///
/// The reader is trusted code, not contributor data: one call returns one indexed
/// unit or None, without a separate effect. Reads proceed exactly once per index
/// from0 through the first NUL, or at most1024 inclusive. None means null input and
/// invokes no reader. A missing unit/terminator, excess payload or invalid encoding
/// invokes no action closure. Output UTF-8 content is at most3072 bytes; allocation
/// remains bounded, including malformed conversion attempts.
///
/// # Errors
/// Returns a fixed rejection with no partially constructed request/action.
pub(crate) fn with_value<F, A, T>(reader: Option<F>, action: A) -> Result<T, ValueInputError>
where
    F: FnMut(usize) -> Option<u16>,
    A: FnOnce(String) -> T,
{
    let mut read = reader.ok_or(ValueInputError::Null)?;
    let mut units = [0_u16; MAX_VALUE_UNITS];
    for index in 0..=MAX_VALUE_UNITS {
        let unit = read(index).ok_or(ValueInputError::Unterminated)?;
        if unit == 0 {
            let value =
                String::from_utf16(&units[..index]).map_err(|_| ValueInputError::InvalidUtf16)?;
            return Ok(action(value));
        }
        if index == MAX_VALUE_UNITS {
            return Err(ValueInputError::TooLong);
        }
        units[index] = unit;
    }
    // The fixed loop returns at index1024 for both possible terminator states.
    Err(ValueInputError::TooLong)
}

/// Reject nonfinite numeric input before constructing an otherwise owned action.
///
/// This is not an implementation of target-specific minimum/maximum policy.
/// Current node/disabled/read-only semantics remain the Windows adapter's owner.
///
/// # Errors
/// NaN/infinities invoke no action closure and return a fixed redacted rejection.
pub(crate) fn with_finite_value<A, T>(value: f64, action: A) -> Result<T, ValueInputError>
where
    A: FnOnce(f64) -> T,
{
    if !value.is_finite() {
        return Err(ValueInputError::NonFinite);
    }
    Ok(action(value))
}

/// Admit the existing signed movement representation without overflowing `abs()`.
///
/// This preserves all magnitudes the existing positive-i32 loop can represent.
///
/// # Errors
/// Rejects `i32::MIN` before a movement loop or range mutation is constructed.
pub(crate) fn move_magnitude(count: i32) -> Result<i32, ValueInputError> {
    count.checked_abs().ok_or(ValueInputError::InvalidCount)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::{MAX_VALUE_UNITS, ValueInputError, move_magnitude, with_finite_value, with_value};

    /// Read a bounded safe fixture through the exact production admission helper.
    ///
    /// # Errors
    /// Returns the helper's fixed encoding, terminator or size rejection.
    fn decode(units: &[u16]) -> Result<String, ValueInputError> {
        with_value(Some(|index| units.get(index).copied()), |value| value)
    }

    /// Valid ASCII, BMP and surrogate pairs retain exact text without replacement.
    ///
    /// # Panics
    /// Fails if valid encoding or the first terminator changes decoded content.
    #[test]
    fn healthy_utf16_and_early_nul_are_exact() {
        assert_eq!(decode(&[0]), Ok(String::new()));
        assert_eq!(decode(&[65, 32, 126, 0]), Ok(String::from("A ~")));
        assert_eq!(decode(&[0x00E9, 0x6F22, 0]), Ok(String::from("é漢")));
        assert_eq!(decode(&[0xD83D, 0xDE00, 0]), Ok(String::from("😀")));
        assert_eq!(decode(&[65, 0, 0xD800]), Ok(String::from("A")));
    }

    /// Worst-case valid BMP content remains bounded after UTF-8 expansion.
    ///
    /// # Panics
    /// Fails if the admitted UTF-16 prefix exceeds its UTF-8 content budget.
    #[test]
    fn utf8_expansion_has_a_fixed_content_budget() {
        let mut units = vec![0x0800; 1024];
        units.push(0);
        let decoded = decode(&units);
        assert_eq!(decoded.as_ref().map(String::len), Ok(3072));
    }

    /// Null/missing input cannot construct an action or trigger an absent reader.
    ///
    /// # Panics
    /// Fails if a null or unterminated fixture constructs a request.
    #[test]
    fn null_and_short_unterminated_inputs_have_no_action() {
        let actions = Cell::new(0);
        let absent: Option<fn(usize) -> Option<u16>> = None;
        let result = with_value(absent, |_| actions.set(actions.get() + 1));
        assert_eq!(result, Err(ValueInputError::Null));
        assert_eq!(actions.get(), 0);
        for units in [&[][..], &[65][..], &[65, 66][..]] {
            let reads = Cell::new(0);
            let result = with_value(
                Some(|index| {
                    reads.set(reads.get() + 1);
                    units.get(index).copied()
                }),
                |_| actions.set(actions.get() + 1),
            );
            assert_eq!(result, Err(ValueInputError::Unterminated));
            assert_eq!(reads.get(), units.len() + 1);
            assert_eq!(actions.get(), 0);
        }
    }

    /// Both adjacent legal bounds and the forbidden next payload are distinguished.
    ///
    /// # Panics
    /// Fails if payload admission or terminator reads exceed the fixed budget.
    #[test]
    fn exact_payload_budget_and_terminator_read_are_bounded() {
        assert_eq!(MAX_VALUE_UNITS, 1024);
        for len in [1023, 1024] {
            let mut units = vec![65; len];
            units.push(0);
            let reads = Cell::new(0);
            let value = with_value(
                Some(|index| {
                    reads.set(reads.get() + 1);
                    units.get(index).copied()
                }),
                |value| value,
            );
            assert_eq!(value, Ok("A".repeat(len)));
            assert_eq!(reads.get(), len + 1);
        }
        let actions = Cell::new(0);
        let reads = Cell::new(0);
        let mut over = vec![65; 1025];
        over.push(0);
        let result = with_value(
            Some(|index| {
                reads.set(reads.get() + 1);
                over.get(index).copied()
            }),
            |_| actions.set(actions.get() + 1),
        );
        assert_eq!(result, Err(ValueInputError::TooLong));
        assert_eq!(reads.get(), 1025);
        assert_eq!(actions.get(), 0);
    }

    /// Every unpaired-surrogate neighbor fails without lossy text or action.
    ///
    /// # Panics
    /// Fails if malformed UTF-16 constructs any request or loses its fixed error.
    #[test]
    fn malformed_surrogates_do_not_construct_requests() {
        let actions = Cell::new(0);
        for units in [
            &[0xD800, 0][..],
            &[0xDC00, 0][..],
            &[0xD800, 65, 0][..],
            &[0xD800, 0xD800, 0][..],
            &[0xDC00, 0xD800, 0][..],
        ] {
            let result = with_value(Some(|index| units.get(index).copied()), |_| {
                actions.set(actions.get() + 1);
            });
            assert_eq!(result, Err(ValueInputError::InvalidUtf16));
            assert_eq!(actions.get(), 0);
        }
        let mut boundary_pair = vec![65; 1022];
        boundary_pair.extend([0xD83D, 0xDE00, 0]);
        assert_eq!(
            decode(&boundary_pair),
            Ok(format!("{}😀", "A".repeat(1022)))
        );
        let mut truncated_pair = vec![65; 1023];
        truncated_pair.extend([0xD83D, 0]);
        assert_eq!(decode(&truncated_pair), Err(ValueInputError::InvalidUtf16));
    }

    /// A reader is called monotonically once per required unit and never after NUL.
    ///
    /// # Panics
    /// Fails if indexed reads repeat or an action runs before complete admission.
    #[test]
    fn reader_invocation_budget_and_successful_action_are_independent() {
        let reads = Cell::new(0);
        let actions = Cell::new(0);
        let value = with_value(
            Some(|index| {
                assert_eq!(index, reads.get());
                reads.set(reads.get() + 1);
                Some(if index == 2 { 0 } else { 65 })
            }),
            |value| {
                actions.set(actions.get() + 1);
                value
            },
        );
        assert_eq!(value, Ok(String::from("AA")));
        assert_eq!(reads.get(), 3);
        assert_eq!(actions.get(), 1);
        let reads = Cell::new(0);
        let result = with_value(
            Some(|index| {
                assert_eq!(index, reads.get());
                reads.set(reads.get() + 1);
                Some(65)
            }),
            |_| actions.set(actions.get() + 1),
        );
        assert_eq!(result, Err(ValueInputError::TooLong));
        assert_eq!(reads.get(), 1025);
        assert_eq!(actions.get(), 1);
    }

    /// Finite values remain exact while nonfinite payloads construct no request.
    ///
    /// # Panics
    /// Fails if nonfinite values reach an action or finite neighbors change bits.
    #[test]
    fn numeric_nonfinite_values_have_no_action() {
        let actions = Cell::new(0);
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let result = with_finite_value(value, |_| actions.set(actions.get() + 1));
            assert_eq!(result, Err(ValueInputError::NonFinite));
            assert_eq!(actions.get(), 0);
        }
        for value in [0.0, -0.0, 1.0, -1.0, f64::MAX, f64::MIN] {
            let result = with_finite_value(value, f64::to_bits);
            assert_eq!(result, Ok(value.to_bits()));
        }
    }

    /// Extreme movement input rejects before an action while legal neighbors work.
    ///
    /// # Panics
    /// Fails if the overflowing count is accepted or signed neighbors change.
    #[test]
    fn movement_magnitude_never_panics_or_wraps() {
        let actions = Cell::new(0);
        let result = move_magnitude(i32::MIN).map(|_| actions.set(actions.get() + 1));
        assert_eq!(result, Err(ValueInputError::InvalidCount));
        assert_eq!(actions.get(), 0);
        for (raw, expected) in [
            (0, 0),
            (1, 1),
            (-1, 1),
            (i32::MAX, i32::MAX),
            (i32::MIN + 1, i32::MAX),
        ] {
            assert_eq!(move_magnitude(raw), Ok(expected));
        }
    }
}
