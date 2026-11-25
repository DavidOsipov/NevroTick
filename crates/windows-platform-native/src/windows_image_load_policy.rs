// SPDX-License-Identifier: GPL-2.0-only
//! Early-runtime policies for image loading in the calling Windows process.
//!
//! The startup owner calls this before admitting service work. The observation
//! covers the policy at the time of each check. Earlier imports, other loading
//! mechanisms and other mitigation policies need their own evidence.

use std::fmt;

/// The original reason the process image-load policy could not be admitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum ImageLoadPolicyError {
    /// Setting the default DLL search directories failed with this Win32 error code.
    SetDefaultDllDirectories(u32),
    /// The first query failed with this Win32 error code.
    ReadInitial(u32),
    /// The first query returned this flag word containing unrecognized bits.
    UnknownInitialFlags(u32),
    /// The setter failed with this Win32 error code.
    Apply(u32),
    /// The query after setting the policy failed with this Win32 error code.
    ReadEffective(u32),
    /// Readback returned this flag word containing unrecognized bits.
    UnknownEffectiveFlags(u32),
    /// Readback omitted an enforcement flag or an existing audit flag.
    MissingEffectiveFlags {
        /// Complete flag word requested of the setter.
        requested: u32,
        /// Complete flag word returned by the readback.
        observed: u32,
    },
}

impl fmt::Display for ImageLoadPolicyError {
    /// Render the fixed stage and bounded numbers without querying native text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SetDefaultDllDirectories(code) => {
                write!(formatter, "SetDefaultDllDirectories failed (Win32 {code}).")
            }
            Self::ReadInitial(code) => {
                write!(formatter, "Image-load policy query failed (Win32 {code}).")
            }
            Self::UnknownInitialFlags(flags) => {
                write!(
                    formatter,
                    "Unrecognized initial image-load policy flags ({flags:#010x})."
                )
            }
            Self::Apply(code) => {
                write!(formatter, "Image-load policy setter failed (Win32 {code}).")
            }
            Self::ReadEffective(code) => {
                write!(
                    formatter,
                    "Effective image-load policy query failed (Win32 {code})."
                )
            }
            Self::UnknownEffectiveFlags(flags) => {
                write!(
                    formatter,
                    "Unrecognized effective image-load policy flags ({flags:#010x})."
                )
            }
            Self::MissingEffectiveFlags {
                requested,
                observed,
            } => write!(
                formatter,
                "Incomplete effective image-load policy ({observed:#010x}, requested {requested:#010x})."
            ),
        }
    }
}

impl std::error::Error for ImageLoadPolicyError {}

/// Restrict the calling process's default DLL directory search to System32,
/// then enforce remote-image and low-label-image restrictions and prefer System32.
///
/// The directory policy applies to subsequent loads using the default search.
/// The startup owner serializes these process-level settings before other work
/// begins. Existing documented image-mitigation flags are preserved.
///
/// # Errors
/// Return a typed refusal on DLL search setup, image query or setter failure,
/// unrecognized flags, or incomplete effective readback. Tightening that
/// already took effect remains.
#[cfg(windows)]
pub fn enforce_current_image_load_policy() -> Result<(), ImageLoadPolicyError> {
    enforcement::check(
        native::set_default_dll_directories,
        native::read_image_load_policy,
        native::set_image_load_policy,
    )
}

/// The original stage and cause of an extension-point policy refusal.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum ExtensionPointPolicyError {
    /// The first query failed with this Win32 error code.
    ReadInitial(u32),
    /// The first query returned a flag word that sets reserved bits.
    UnknownInitialFlags(u32),
    /// The setter failed with this Win32 error code.
    Apply(u32),
    /// The query after setting the policy failed with this Win32 error code.
    ReadEffective(u32),
    /// Readback returned a flag word that sets reserved bits.
    UnknownEffectiveFlags(u32),
    /// The effective word was zero after the setter returned success.
    NotEffective,
}

#[cfg(any(windows, test))]
impl fmt::Display for ExtensionPointPolicyError {
    /// Render only the known stage and bounded words or codes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ReadInitial(code) => {
                write!(
                    formatter,
                    "Extension-point policy query failed (Win32 {code})."
                )
            }
            Self::UnknownInitialFlags(flags) => {
                write!(
                    formatter,
                    "Unrecognized initial extension-point flags ({flags:#010x})."
                )
            }
            Self::Apply(code) => {
                write!(
                    formatter,
                    "Extension-point policy setter failed (Win32 {code})."
                )
            }
            Self::ReadEffective(code) => {
                write!(
                    formatter,
                    "Effective extension-point policy query failed (Win32 {code})."
                )
            }
            Self::UnknownEffectiveFlags(flags) => {
                write!(
                    formatter,
                    "Unrecognized effective extension-point flags ({flags:#010x})."
                )
            }
            Self::NotEffective => formatter.write_str("Legacy extension points remain enabled."),
        }
    }
}

#[cfg(any(windows, test))]
impl std::error::Error for ExtensionPointPolicyError {}

/// Disable legacy extension points for the current fixed-role service process.
///
/// The same startup owner runs this after the image-load and critical-error
/// checks, before SCM connection or service work. An already enforced policy
/// needs no setter. This observation covers the original process at readback.
///
/// # Errors
/// Return the first unavailable, unsupported or malformed observation, setter
/// failure, or missing effective bit. A refusal makes no retry, no weaker
/// request and no attempt to undo previously established protections.
#[cfg(windows)]
pub fn enforce_current_extension_point_policy() -> Result<(), ExtensionPointPolicyError> {
    extension_point::check(
        native::read_extension_point_policy,
        native::set_extension_point_policy,
    )
}

/// Enforce one reviewed flag and admit only the documented effective word.
#[cfg(any(windows, test))]
mod extension_point {
    use super::ExtensionPointPolicyError;

    /// `DisableExtensionPoints` in `PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY`.
    const DISABLE_EXTENSION_POINTS: u32 = 0x0000_0001;

    /// Query, set only the documented bit if needed, and reobserve enforcement.
    ///
    /// # Errors
    /// Preserve the first stage-specific failure and stop without retrying.
    pub(super) fn check(
        mut read: impl FnMut() -> Result<u32, u32>,
        set: impl FnOnce(u32) -> Result<(), u32>,
    ) -> Result<(), ExtensionPointPolicyError> {
        match read().map_err(ExtensionPointPolicyError::ReadInitial)? {
            DISABLE_EXTENSION_POINTS => return Ok(()),
            0 => {}
            flags => return Err(ExtensionPointPolicyError::UnknownInitialFlags(flags)),
        }
        set(DISABLE_EXTENSION_POINTS).map_err(ExtensionPointPolicyError::Apply)?;
        match read().map_err(ExtensionPointPolicyError::ReadEffective)? {
            DISABLE_EXTENSION_POINTS => Ok(()),
            0 => Err(ExtensionPointPolicyError::NotEffective),
            flags => Err(ExtensionPointPolicyError::UnknownEffectiveFlags(flags)),
        }
    }

    /// Ordered portable tests of the extension-point policy decision.
    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;

        use super::{ExtensionPointPolicyError, check};

        /// One exact expected call and its independent native result.
        #[derive(Debug)]
        enum Step {
            /// Query the original or effective word.
            Read(Result<u32, u32>),
            /// Submit only the externally expected flag word.
            Set(u32, Result<(), u32>),
        }

        /// Exercise the actual enforcement rule with an ordered native transcript.
        ///
        /// # Panics
        /// Reject omitted, additional, misordered or altered operations.
        #[expect(
            clippy::panic,
            reason = "report an unexpected native call in the fixed transcript"
        )]
        fn require_steps<const LENGTH: usize>(
            steps: [Step; LENGTH],
            expected: Result<(), ExtensionPointPolicyError>,
        ) {
            let remaining: RefCell<std::array::IntoIter<Step, LENGTH>> =
                RefCell::new(steps.into_iter());
            let result: Result<(), ExtensionPointPolicyError> = check(
                || match remaining.borrow_mut().next() {
                    Some(Step::Read(result)) => result,
                    step => panic!("unexpected query before {step:?}"),
                },
                |flags: u32| match remaining.borrow_mut().next() {
                    Some(Step::Set(requested, result)) => {
                        assert_eq!(flags, requested);
                        result
                    }
                    step => panic!("unexpected setter before {step:?}"),
                },
            );
            assert_eq!(result, expected);
            assert!(remaining.into_inner().next().is_none());
        }

        /// Compare inherited enforcement to adding exactly the one necessary bit.
        #[test]
        fn preserves_inherited_policy_or_sets_only_the_documented_bit() {
            require_steps([Step::Read(Ok(1))], Ok(()));
            require_steps(
                [Step::Read(Ok(0)), Step::Set(1, Ok(())), Step::Read(Ok(1))],
                Ok(()),
            );
        }

        /// Missing or malformed initial evidence must not reach the setter.
        #[test]
        fn refuses_unavailable_or_malformed_initial_policy_before_setting() {
            require_steps(
                [Step::Read(Err(50))],
                Err(ExtensionPointPolicyError::ReadInitial(50)),
            );
            require_steps(
                [Step::Read(Ok(3))],
                Err(ExtensionPointPolicyError::UnknownInitialFlags(3)),
            );
        }

        /// Preserve the first native refusal and the distinct invalid readback cases.
        #[test]
        fn preserves_setter_and_effective_readback_refusals() {
            require_steps(
                [Step::Read(Ok(0)), Step::Set(1, Err(87))],
                Err(ExtensionPointPolicyError::Apply(87)),
            );
            let cases: [(Result<u32, u32>, ExtensionPointPolicyError); 3] = [
                (Err(5), ExtensionPointPolicyError::ReadEffective(5)),
                (Ok(0), ExtensionPointPolicyError::NotEffective),
                (Ok(3), ExtensionPointPolicyError::UnknownEffectiveFlags(3)),
            ];
            for (readback, error) in cases {
                require_steps(
                    [
                        Step::Read(Ok(0)),
                        Step::Set(1, Ok(())),
                        Step::Read(readback),
                    ],
                    Err(error),
                );
            }
        }

        /// Distinguish unavailable, malformed and unset policy in reported causes.
        ///
        /// # Panics
        /// A diagnostic changes stage or loses the native observation.
        #[test]
        fn reports_extension_point_refusal_stage_and_observation() {
            let cases: [(ExtensionPointPolicyError, &str); 6] = [
                (
                    ExtensionPointPolicyError::ReadInitial(5),
                    "Extension-point policy query failed (Win32 5).",
                ),
                (
                    ExtensionPointPolicyError::UnknownInitialFlags(3),
                    "Unrecognized initial extension-point flags (0x00000003).",
                ),
                (
                    ExtensionPointPolicyError::Apply(87),
                    "Extension-point policy setter failed (Win32 87).",
                ),
                (
                    ExtensionPointPolicyError::ReadEffective(50),
                    "Effective extension-point policy query failed (Win32 50).",
                ),
                (
                    ExtensionPointPolicyError::UnknownEffectiveFlags(9),
                    "Unrecognized effective extension-point flags (0x00000009).",
                ),
                (
                    ExtensionPointPolicyError::NotEffective,
                    "Legacy extension points remain enabled.",
                ),
            ];
            for (error, expected) in cases {
                assert_eq!(error.to_string(), expected);
            }
        }
    }
}

/// The common enforcement path for native calls and ordered portable controls.
#[cfg(any(windows, test))]
mod enforcement {
    use super::ImageLoadPolicyError;

    /// `LOAD_LIBRARY_SEARCH_SYSTEM32` in `libloaderapi.h`.
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;

    /// `NoRemoteImages`, `NoLowMandatoryLabelImages`, and `PreferSystem32Images` in `winnt.h`.
    const REQUIRED_FLAGS: u32 = (1 << 0) | (1 << 1) | (1 << 2);
    /// `AuditNoRemoteImages` and `AuditNoLowMandatoryLabelImages` in `winnt.h`.
    const AUDIT_FLAGS: u32 = (1 << 3) | (1 << 4);
    /// The remaining bits are reserved in the selected SDK layout.
    const RECOGNIZED_FLAGS: u32 = REQUIRED_FLAGS | AUDIT_FLAGS;

    /// Set default DLL search directories before any image-mitigation call.
    /// Apply the image mitigation only if its initial flags need tightening.
    ///
    /// # Errors
    /// Return the first native or policy refusal with its original details.
    pub(super) fn check(
        set_directories: impl FnOnce(u32) -> Result<(), u32>,
        mut read: impl FnMut() -> Result<u32, u32>,
        set: impl FnOnce(u32) -> Result<(), u32>,
    ) -> Result<(), ImageLoadPolicyError> {
        set_directories(LOAD_LIBRARY_SEARCH_SYSTEM32)
            .map_err(ImageLoadPolicyError::SetDefaultDllDirectories)?;
        let original: u32 = read().map_err(ImageLoadPolicyError::ReadInitial)?;
        if original & !RECOGNIZED_FLAGS != 0 {
            // Reserved bits have no admitted write semantics.
            return Err(ImageLoadPolicyError::UnknownInitialFlags(original));
        }
        let requested: u32 = original | REQUIRED_FLAGS;
        if original == requested {
            // All required protections were already observed. Avoid a setter.
            return Ok(());
        }
        set(requested).map_err(ImageLoadPolicyError::Apply)?;
        let observed: u32 = read().map_err(ImageLoadPolicyError::ReadEffective)?;
        if observed & !RECOGNIZED_FLAGS != 0 {
            return Err(ImageLoadPolicyError::UnknownEffectiveFlags(observed));
        }
        if observed & requested != requested {
            // A successful setter does not establish effective enforcement or
            // preservation of the previous policy.
            return Err(ImageLoadPolicyError::MissingEffectiveFlags {
                requested,
                observed,
            });
        }
        Ok(())
    }

    /// Ordered portable controls for the common policy decision and its effects.
    #[cfg(test)]
    mod tests {
        use super::{ImageLoadPolicyError, check};
        use std::cell::RefCell;

        /// One planned native operation with its independent result.
        enum Step {
            /// The next operation must set exactly the declared DLL search flags.
            SetDirectories(u32, Result<(), u32>),
            /// The next operation must be a read.
            Read(Result<u32, u32>),
            /// The next operation must write exactly this flag word.
            Set(u32, Result<(), u32>),
        }

        /// Prepend the independently expected successful DLL search setup to a mitigation script.
        ///
        /// # Panics
        /// The decision changes the setup flag or deviates from the planned operations.
        fn assert_script<const N: usize>(
            steps: [Step; N],
            expected: Result<(), ImageLoadPolicyError>,
        ) {
            assert_startup_script(
                std::iter::once(Step::SetDirectories(0x800, Ok(()))).chain(steps),
                expected,
            );
        }

        /// Run through the common enforcement decision and consume every planned step.
        ///
        /// # Panics
        /// The decision makes an unexpected call, modifies its intended flags,
        /// or returns before the expected operations.
        #[expect(
            clippy::panic,
            reason = "unexpected policy calls fail the ordered test"
        )]
        fn assert_startup_script<S: IntoIterator<Item = Step>>(
            steps: S,
            expected: Result<(), ImageLoadPolicyError>,
        ) {
            let remaining: RefCell<S::IntoIter> = RefCell::new(steps.into_iter());
            let observed: Result<(), ImageLoadPolicyError> = check(
                |observed_flags: u32| {
                    let Some(Step::SetDirectories(expected_flags, result)) =
                        remaining.borrow_mut().next()
                    else {
                        panic!("unexpected DLL search setter");
                    };
                    assert_eq!(observed_flags, expected_flags);
                    result
                },
                || {
                    let Some(Step::Read(result)) = remaining.borrow_mut().next() else {
                        panic!("unexpected policy read");
                    };
                    result
                },
                |observed_flags: u32| {
                    let Some(Step::Set(expected_flags, result)) = remaining.borrow_mut().next()
                    else {
                        panic!("unexpected policy setter");
                    };
                    assert_eq!(observed_flags, expected_flags);
                    result
                },
            );
            assert_eq!(observed, expected);
            assert!(remaining.into_inner().next().is_none());
        }

        /// Default DLL search is set first even if no mitigation setter is needed.
        ///
        /// # Panics
        /// The adapter omits the DLL search setup or attempts a mitigation write.
        #[test]
        fn admits_existing_enforcement_without_setting() {
            assert_script([Step::Read(Ok(0x07))], Ok(()));
            assert_script([Step::Read(Ok(0x1f))], Ok(()));
        }

        /// A DLL search refusal prevents every subsequent mitigation operation.
        ///
        /// # Panics
        /// The adapter loses the original failure or attempts a mitigation call.
        #[test]
        fn rejects_dll_search_failure_before_mitigation_calls() {
            assert_startup_script(
                [Step::SetDirectories(0x800, Err(87))],
                Err(ImageLoadPolicyError::SetDefaultDllDirectories(87)),
            );
        }

        /// Request every enforcement bit and preserve both previously set audit bits.
        ///
        /// # Panics
        /// The adapter omits a protection or an inherited flag.
        #[test]
        fn sets_and_checks_the_complete_policy() {
            assert_script(
                [
                    Step::Read(Ok(0x18)),
                    Step::Set(0x1f, Ok(())),
                    Step::Read(Ok(0x1f)),
                ],
                Ok(()),
            );
            assert_script(
                [
                    Step::Read(Ok(0)),
                    Step::Set(0x07, Ok(())),
                    Step::Read(Ok(0x1f)),
                ],
                Ok(()),
            );
        }

        /// A failed or unrecognized initial read prevents a mitigation write.
        ///
        /// # Panics
        /// Either initial refusal attempts a mitigation write or loses its cause.
        #[test]
        fn rejects_initial_failure_before_setting() {
            assert_script(
                [Step::Read(Err(5))],
                Err(ImageLoadPolicyError::ReadInitial(5)),
            );
            assert_script(
                [Step::Read(Ok(0x8000_001f))],
                Err(ImageLoadPolicyError::UnknownInitialFlags(0x8000_001f)),
            );
        }

        /// Setter and readback failures keep their stage and stop the transaction.
        ///
        /// # Panics
        /// A failure is replaced, retried, or accepted.
        #[test]
        fn rejects_setter_and_readback_failures() {
            assert_script(
                [Step::Read(Ok(0)), Step::Set(0x07, Err(50))],
                Err(ImageLoadPolicyError::Apply(50)),
            );
            assert_script(
                [
                    Step::Read(Ok(0)),
                    Step::Set(0x07, Ok(())),
                    Step::Read(Err(87)),
                ],
                Err(ImageLoadPolicyError::ReadEffective(87)),
            );
            assert_script(
                [
                    Step::Read(Ok(0)),
                    Step::Set(0x07, Ok(())),
                    Step::Read(Ok(0x8000_0007)),
                ],
                Err(ImageLoadPolicyError::UnknownEffectiveFlags(0x8000_0007)),
            );
        }

        /// An individual missing bit, including audit-only readback, refuses admission.
        ///
        /// # Panics
        /// Missing enforcement or inherited audit state is accepted.
        #[test]
        fn rejects_each_missing_requested_bit() {
            let incomplete_policies: [u32; 6] = [0x1e, 0x1d, 0x1b, 0x17, 0x0f, 0x18];
            for observed in incomplete_policies {
                assert_script(
                    [
                        Step::Read(Ok(0x18)),
                        Step::Set(0x1f, Ok(())),
                        Step::Read(Ok(observed)),
                    ],
                    Err(ImageLoadPolicyError::MissingEffectiveFlags {
                        requested: 0x1f,
                        observed,
                    }),
                );
            }
        }

        /// Retain the native error code or exact flag words at the failed stage.
        ///
        /// # Panics
        /// A diagnostic changes stage or loses the native observation.
        #[test]
        fn reports_image_load_refusal_stage_and_observation() {
            let cases: [(ImageLoadPolicyError, &str); 7] = [
                (
                    ImageLoadPolicyError::SetDefaultDllDirectories(87),
                    "SetDefaultDllDirectories failed (Win32 87).",
                ),
                (
                    ImageLoadPolicyError::ReadInitial(5),
                    "Image-load policy query failed (Win32 5).",
                ),
                (
                    ImageLoadPolicyError::UnknownInitialFlags(0x8000_0007),
                    "Unrecognized initial image-load policy flags (0x80000007).",
                ),
                (
                    ImageLoadPolicyError::Apply(50),
                    "Image-load policy setter failed (Win32 50).",
                ),
                (
                    ImageLoadPolicyError::ReadEffective(120),
                    "Effective image-load policy query failed (Win32 120).",
                ),
                (
                    ImageLoadPolicyError::UnknownEffectiveFlags(0x4000_001f),
                    "Unrecognized effective image-load policy flags (0x4000001f).",
                ),
                (
                    ImageLoadPolicyError::MissingEffectiveFlags {
                        requested: 0x1f,
                        observed: 0x0f,
                    },
                    "Incomplete effective image-load policy (0x0000000f, requested 0x0000001f).",
                ),
            ];
            for (error, expected) in cases {
                assert_eq!(error.to_string(), expected);
            }
        }
    }
}

/// Fixed Win32 calls and layout for the current process's DLL search and image policies.
#[cfg(windows)]
mod native {
    use std::{ffi::c_void, mem::size_of};

    /// The only selected mitigation policy layouts with one DWORD of storage.
    enum DwordPolicy {
        /// `PROCESS_MITIGATION_IMAGE_LOAD_POLICY`.
        ImageLoad,
        /// `PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY`.
        ExtensionPointDisable,
    }

    impl DwordPolicy {
        /// Project the exact fixed `PROCESS_MITIGATION_POLICY` SDK selector.
        const fn selector(self) -> i32 {
            match self {
                Self::ImageLoad => 10,
                Self::ExtensionPointDisable => 6,
            }
        }
    }

    /// One DWORD in either selected policy union.
    #[repr(transparent)]
    struct PolicyFlags(u32);

    const _: [(); 4] = [(); size_of::<PolicyFlags>()];
    const _: [(); 4] = [(); align_of::<PolicyFlags>()];

    #[link(name = "kernel32")]
    #[expect(unsafe_code, reason = "exact Win32 DLL search and mitigation ABI")]
    unsafe extern "system" {
        /// Apply the default DLL directory flags for the calling process.
        fn SetDefaultDllDirectories(directory_flags: u32) -> i32;
        /// Borrow the calling process's query-capable pseudohandle.
        fn GetCurrentProcess() -> *mut c_void;
        /// Query the selected policy with one fixed writable buffer.
        fn GetProcessMitigationPolicy(
            process: *mut c_void,
            policy: i32,
            buffer: *mut c_void,
            length: usize,
        ) -> i32;
        /// Set the selected policy on the calling process.
        fn SetProcessMitigationPolicy(policy: i32, buffer: *mut c_void, length: usize) -> i32;
        /// Read the failed operation's last error on this thread.
        fn GetLastError() -> u32;
    }

    /// Apply the default DLL search flags from the fixed enforcement decision.
    ///
    /// # Errors
    /// Return the immediate native error on failure.
    #[expect(unsafe_code, reason = "one fixed process-wide DLL search setting")]
    pub(super) fn set_default_dll_directories(directory_flags: u32) -> Result<(), u32> {
        // SAFETY: The caller supplies the fixed documented System32 flag. The
        // synchronous call takes no pointer arguments or borrowed resources.
        let result: i32 = unsafe { SetDefaultDllDirectories(directory_flags) };
        if result == 0 {
            // SAFETY: Capture the failed setter's last error immediately on this thread.
            return Err(unsafe { GetLastError() });
        }
        Ok(())
    }

    /// Read the selected one-DWORD policy of the current process.
    ///
    /// # Errors
    /// Return the native error on a failed query.
    #[expect(
        unsafe_code,
        reason = "local policy buffer and current-process pseudohandle"
    )]
    fn read_policy(policy: DwordPolicy) -> Result<u32, u32> {
        let mut flags: PolicyFlags = PolicyFlags(0);
        // SAFETY: This call has no preconditions. The pseudo-handle is borrowed.
        let process: *mut c_void = unsafe { GetCurrentProcess() };
        // SAFETY: The local policy has the exact initialized, aligned layout and
        // extent of the requested writable Win32 policy. The call is synchronous.
        let result: i32 = unsafe {
            GetProcessMitigationPolicy(
                process,
                policy.selector(),
                (&raw mut flags).cast::<c_void>(),
                size_of::<PolicyFlags>(),
            )
        };
        if result == 0 {
            // SAFETY: Capture the failed query's last error immediately on this thread.
            return Err(unsafe { GetLastError() });
        }
        Ok(flags.0)
    }

    /// Apply a flag word assembled by the fixed enforcement decision.
    ///
    /// # Errors
    /// Return the native error on setter failure. The caller checks readback.
    #[expect(unsafe_code, reason = "synchronous setter with a single checked DWORD")]
    fn set_policy(policy: DwordPolicy, flags: u32) -> Result<(), u32> {
        let mut policy_flags: PolicyFlags = PolicyFlags(flags);
        // SAFETY: The local policy has the exact initialized, aligned layout and
        // extent of the requested Win32 policy and outlives the synchronous call.
        let result: i32 = unsafe {
            SetProcessMitigationPolicy(
                policy.selector(),
                (&raw mut policy_flags).cast::<c_void>(),
                size_of::<PolicyFlags>(),
            )
        };
        if result == 0 {
            // SAFETY: Capture the failed setter's last error immediately on this thread.
            return Err(unsafe { GetLastError() });
        }
        Ok(())
    }

    /// Query only the image-load flags of the current process.
    ///
    /// # Errors
    /// Return the original Win32 query failure.
    pub(super) fn read_image_load_policy() -> Result<u32, u32> {
        read_policy(DwordPolicy::ImageLoad)
    }

    /// Apply the image-load flags admitted by the image-load decision.
    ///
    /// # Errors
    /// Return the original Win32 setter failure.
    pub(super) fn set_image_load_policy(flags: u32) -> Result<(), u32> {
        set_policy(DwordPolicy::ImageLoad, flags)
    }

    /// Query only the extension-point flags of the current process.
    ///
    /// # Errors
    /// Return the original Win32 query failure.
    pub(super) fn read_extension_point_policy() -> Result<u32, u32> {
        read_policy(DwordPolicy::ExtensionPointDisable)
    }

    /// Apply the flag admitted by the extension-point decision.
    ///
    /// # Errors
    /// Return the original Win32 setter failure.
    pub(super) fn set_extension_point_policy(flags: u32) -> Result<(), u32> {
        set_policy(DwordPolicy::ExtensionPointDisable, flags)
    }
}
