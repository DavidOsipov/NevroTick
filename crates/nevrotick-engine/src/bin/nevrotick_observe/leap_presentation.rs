// SPDX-License-Identifier: GPL-2.0-only
//! Bounded historical projection of the current Windows process's leap presentation.

use std::io::{self, Write};

/// The native observation retained until the invocation's diagnostic output.
#[must_use = "retain the original presentation observation through cleanup"]
pub(super) enum Observation {
    /// The process uses compatibility presentation.
    Compatibility,
    /// The process explicitly opted into the sixtieth second, unexpected for V1.
    SixtySecond,
    /// The query failed with the original native code, including zero.
    QueryFailed(u32),
    /// The query returned unsupported fields, retained verbatim.
    InvalidReply {
        /// Original process presentation flags.
        flags: u32,
        /// Original reserved word.
        reserved: u32,
    },
}

impl Observation {
    /// Emit one bounded record of the original historical observation.
    ///
    /// # Errors
    /// Preserve the output sink's first refusal.
    pub(super) fn write(self, output: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Compatibility => writeln!(
                output,
                "event=leap_presentation status=observed mode=compatibility flags=0 reserved=0 mapping=unavailable"
            ),
            Self::SixtySecond => writeln!(
                output,
                "event=leap_presentation status=unexpected mode=sixty_second flags=1 reserved=0 mapping=unavailable"
            ),
            Self::QueryFailed(code) => writeln!(
                output,
                "event=leap_presentation status=unavailable reason=query_failed error={code} mapping=unavailable"
            ),
            Self::InvalidReply { flags, reserved } => writeln!(
                output,
                "event=leap_presentation status=unavailable reason=invalid_reply flags={flags} reserved={reserved} mapping=unavailable"
            ),
        }
    }
}

/// Query once through the shared own-process adapter and preserve its exact observation.
#[cfg(windows)]
pub(super) fn read() -> Observation {
    use windows_platform_native::{
        LeapPresentation, LeapPresentationError, read_leap_presentation,
    };

    match read_leap_presentation() {
        Ok(LeapPresentation::Compatibility) => Observation::Compatibility,
        Ok(LeapPresentation::SixtySecond) => Observation::SixtySecond,
        Err(LeapPresentationError::QueryFailed(code)) => Observation::QueryFailed(code),
        Err(LeapPresentationError::InvalidReply { flags, reserved }) => {
            Observation::InvalidReply { flags, reserved }
        }
    }
}
