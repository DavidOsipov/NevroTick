// SPDX-License-Identifier: GPL-2.0-only
//! Critical-error handling for the calling Windows service fixture process.

use std::fmt;

/// The process error mode could not be confirmed before SCM connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum ServiceErrorModeError {
    /// The setter's original mode differed from the mode actually sampled.
    ChangedBeforeSet {
        /// Mode from the initial process query.
        sampled: u32,
        /// Previous mode returned by the setter.
        previous: u32,
    },
    /// Effective readback omitted a requested bit.
    IncompleteReadback {
        /// Complete inherited mode plus the critical-error bit.
        required: u32,
        /// Mode reported by the process readback.
        observed: u32,
    },
}

impl fmt::Display for ServiceErrorModeError {
    /// Render the failing stage and bounded mode words.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ChangedBeforeSet { sampled, previous } => write!(
                formatter,
                "Service error mode changed before setting ({sampled:#010x} to {previous:#010x})."
            ),
            Self::IncompleteReadback { required, observed } => write!(
                formatter,
                "Incomplete service error mode ({observed:#010x}, required {required:#010x})."
            ),
        }
    }
}

impl std::error::Error for ServiceErrorModeError {}

/// Route critical-error dialog outcomes to the current service process.
///
/// The fixed startup owner uses this before connecting to SCM, with no
/// competing process error-mode writer. The read and set are separate calls.
/// The setter reports the previous mode and readback only observes the mode
/// at that instant. The fixed-role services do not launch children. Windows
/// can pass process error mode to children in other compositions. This
/// operation adds only `SEM_FAILCRITICALERRORS`.
///
/// # Errors
/// Refuse an intervening change reported by the setter or incomplete effective
/// readback. Either refusal can follow a mode change. The caller must refuse
/// startup without retrying or resetting the process mode.
#[cfg(windows)]
pub fn enforce_service_critical_error_mode() -> Result<(), ServiceErrorModeError> {
    check(native::read, native::set)
}

/// Add only the critical-error bit and confirm inherited flags on readback.
///
/// # Errors
/// Keep the first observed conflict or incomplete readback, without retry.
#[cfg(any(windows, test))]
fn check(
    mut read: impl FnMut() -> u32,
    set: impl FnOnce(u32) -> u32,
) -> Result<(), ServiceErrorModeError> {
    // SEM_FAILCRITICALERRORS in errhandlingapi.h.
    const FAIL_CRITICAL_ERRORS: u32 = 0x0001;
    let sampled: u32 = read();
    let required: u32 = sampled | FAIL_CRITICAL_ERRORS;
    if required == sampled {
        return Ok(());
    }
    let previous: u32 = set(required);
    if previous != sampled {
        return Err(ServiceErrorModeError::ChangedBeforeSet { sampled, previous });
    }
    let observed: u32 = read();
    if observed & required != required {
        return Err(ServiceErrorModeError::IncompleteReadback { required, observed });
    }
    Ok(())
}

/// Exact Kernel32 API for the sole startup owner.
#[cfg(windows)]
mod native {
    #[link(name = "kernel32")]
    #[expect(unsafe_code, reason = "exact Win32 process error-mode ABI")]
    unsafe extern "system" {
        /// Read the current process's mode. Zero is a valid mode.
        fn GetErrorMode() -> u32;
        /// Set the mode of this process and return its previous flag word.
        fn SetErrorMode(mode: u32) -> u32;
    }

    /// Return only the current process's mode at this observation.
    #[expect(unsafe_code, reason = "read the original process error mode")]
    pub(super) fn read() -> u32 {
        // SAFETY: The fixed current-process query takes no parameters.
        unsafe { GetErrorMode() }
    }

    /// Request the inherited mode with the required bit from the startup owner.
    #[expect(unsafe_code, reason = "set only the owner-admitted process error mode")]
    pub(super) fn set(mode: u32) -> u32 {
        // SAFETY: The ABI accepts a UINT with no pointers or external lifetime.
        // The sole caller adds only the critical-error bit to the prior query.
        unsafe { SetErrorMode(mode) }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::{ServiceErrorModeError, check};

    /// One exact operation and its return value.
    #[derive(Debug)]
    enum Step {
        /// Read the configured original or effective process mode.
        Read(u32),
        /// Set this exact flag word and return the prior process mode.
        Set(u32, u32),
    }

    /// Exercise the real decision with an independent ordered API transcript.
    ///
    /// # Panics
    /// Refuse extra, missing, reordered or altered operations.
    #[expect(
        clippy::panic,
        reason = "report unexpected calls in a finite test transcript"
    )]
    fn require_steps<const LENGTH: usize>(
        steps: [Step; LENGTH],
        expected: Result<(), ServiceErrorModeError>,
    ) {
        let remaining: RefCell<std::array::IntoIter<Step, LENGTH>> =
            RefCell::new(steps.into_iter());
        let result: Result<(), ServiceErrorModeError> = check(
            || match remaining.borrow_mut().next() {
                Some(Step::Read(mode)) => mode,
                step => panic!("unexpected query before {step:?}"),
            },
            |mode: u32| match remaining.borrow_mut().next() {
                Some(Step::Set(requested, previous)) => {
                    assert_eq!(mode, requested);
                    previous
                }
                step => panic!("unexpected setter before {step:?}"),
            },
        );
        assert_eq!(result, expected);
        assert!(remaining.into_inner().next().is_none());
    }

    /// The first read alone admits an inherited complete policy.
    #[test]
    fn admits_an_inherited_critical_error_policy_without_a_setter() {
        require_steps([Step::Read(0x8001)], Ok(()));
    }

    /// Successful readback contains the sole added bit and all prior bits.
    #[test]
    fn adds_only_the_critical_error_bit_and_preserves_inherited_flags() {
        require_steps(
            [
                Step::Read(0x8000),
                Step::Set(0x8001, 0x8000),
                Step::Read(0x8001),
            ],
            Ok(()),
        );
        require_steps([Step::Read(0), Step::Set(1, 0), Step::Read(1)], Ok(()));
    }

    /// A changed previous word refuses admission after exactly one setter call.
    #[test]
    fn refuses_an_intervening_mode_change_without_retry_or_restore() {
        require_steps(
            [Step::Read(0x8000), Step::Set(0x8001, 0x8004)],
            Err(ServiceErrorModeError::ChangedBeforeSet {
                sampled: 0x8000,
                previous: 0x8004,
            }),
        );
    }

    /// Verify both the critical flag and inherited flags at the effective edge.
    #[test]
    fn refuses_missing_required_or_inherited_readback() {
        for observed in [0x8000_u32, 1] {
            require_steps(
                [
                    Step::Read(0x8000),
                    Step::Set(0x8001, 0x8000),
                    Step::Read(observed),
                ],
                Err(ServiceErrorModeError::IncompleteReadback {
                    required: 0x8001,
                    observed,
                }),
            );
        }
    }

    /// Diagnostic consumers can distinguish the stages and the original flag words.
    ///
    /// # Panics
    /// A diagnostic changes stage or loses either flag word.
    #[test]
    fn reports_stage_and_flag_words_for_each_refusal() {
        let conflict: ServiceErrorModeError = ServiceErrorModeError::ChangedBeforeSet {
            sampled: 0x8000,
            previous: 0x8004,
        };
        let readback: ServiceErrorModeError = ServiceErrorModeError::IncompleteReadback {
            required: 0x8001,
            observed: 0x0001,
        };
        assert_eq!(
            conflict.to_string(),
            "Service error mode changed before setting (0x00008000 to 0x00008004)."
        );
        assert_eq!(
            readback.to_string(),
            "Incomplete service error mode (0x00000001, required 0x00008001)."
        );
    }
}
