// SPDX-License-Identifier: GPL-2.0-only
//! Checked presentation flags from a current-process query.

/// The SDK's `PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND` bit.
const SIXTY_SECOND_FLAG: u32 = 1;

/// Presentation reported for one process at the time of observation.
/// It conveys no system leap-event knowledge, UTC mapping or authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain or handle the observed process leap-second mode"]
pub enum LeapPresentation {
    /// The process did not opt in to representing the sixtieth second.
    Compatibility,
    /// The process opted in to representing the sixtieth second.
    SixtySecond,
}

/// A current-process presentation query yielded no supported observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeapPresentationError {
    /// The query failed with this original Windows error, including zero.
    QueryFailed(u32),
    /// A successful query returned an unsupported field combination.
    InvalidReply {
        /// The unchanged SDK `Flags` word.
        flags: u32,
        /// The unchanged SDK `Reserved` word.
        reserved: u32,
    },
}

/// Admit either known mode with a clear reserved word, without effects.
///
/// # Errors
/// Retains both raw words when a flag is unknown or the reserved word is nonzero.
pub(super) const fn classify(
    flags: u32,
    reserved: u32,
) -> Result<LeapPresentation, LeapPresentationError> {
    match (flags, reserved) {
        (0, 0) => Ok(LeapPresentation::Compatibility),
        (SIXTY_SECOND_FLAG, 0) => Ok(LeapPresentation::SixtySecond),
        _ => Err(LeapPresentationError::InvalidReply { flags, reserved }),
    }
}

#[cfg(test)]
mod tests {
    use super::{LeapPresentation, LeapPresentationError, classify};

    /// Distinguish the two defined flag words when reserved is clear.
    ///
    /// # Panics
    /// Panics if either defined flag does not report its exact presentation.
    #[test]
    fn admits_both_defined_modes() {
        assert_eq!(classify(0, 0), Ok(LeapPresentation::Compatibility));
        assert_eq!(classify(1, 0), Ok(LeapPresentation::SixtySecond));
    }

    /// Unknown bits and reserved output cannot yield a supported mode.
    ///
    /// # Panics
    /// Panics if an unsupported reply omits either original word or is accepted.
    #[test]
    fn refuses_unsupported_fields_without_discarding_them() {
        let replies: [(u32, u32); 5] = [(2, 0), (3, 0), (0, 1), (1, 1), (u32::MAX, u32::MAX)];
        for (flags, reserved) in replies {
            assert_eq!(
                classify(flags, reserved),
                Err(LeapPresentationError::InvalidReply { flags, reserved }),
            );
        }
    }
}
