// SPDX-License-Identifier: GPL-2.0-only
//! Acquire and close individual service rights on the two fixed S0.6 fixture keys.
//! The installed package, actor and process lifetime are supplied and checked by the lab.

use crate::LabRole;

/// Win32 access-denied result.
const ACCESS_DENIED: u32 = 5;
/// Win32 result for a missing registration.
const SERVICE_ABSENT: u32 = 1060;

/// Each observation requests exactly one right, never a combined or caller-supplied mask.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Right {
    /// Read service status.
    QueryStatus,
    /// Read service configuration.
    QueryConfig,
    /// Read the service object's security descriptor.
    ReadControl,
    /// Observe whether changing configuration would require a different actor.
    ChangeConfig,
    /// Observe start access without starting the service.
    Start,
    /// Observe stop access without sending a control.
    Stop,
}

impl Right {
    /// Return the exact SDK mask for this observation.
    const fn mask(self) -> u32 {
        match self {
            Self::QueryStatus => 0x0004,
            Self::QueryConfig => 0x0001,
            Self::ReadControl => 0x0002_0000,
            Self::ChangeConfig => 0x0002,
            Self::Start => 0x0010,
            Self::Stop => 0x0020,
        }
    }
}

/// First unsuccessful operation for one fixed right and role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    /// The local active manager database could not be opened.
    OpenManager,
    /// The fixed service key could not be opened with the selected access.
    OpenService,
    /// The acquired service handle did not acknowledge closure.
    CloseService,
    /// The manager handle did not acknowledge closure.
    CloseManager,
}

/// An unsuccessful access observation retains the first raw code and later close error.
#[derive(Debug, Eq, PartialEq)]
struct Failure {
    /// Operation that failed first.
    stage: Stage,
    /// Its original Win32 last-error code.
    code: u32,
    /// Manager close error following an earlier service failure.
    additional_manager_close: Option<u32>,
}

impl Failure {
    /// Distinguish a missing service and denied access from other native failures.
    const fn kind(&self) -> &'static str {
        match (self.stage, self.code) {
            (Stage::OpenService, SERVICE_ABSENT) => "service-missing",
            (Stage::OpenManager | Stage::OpenService, ACCESS_DENIED) => "access-denied",
            _ => "failed",
        }
    }
}

/// The closed acquisition and explicit retirement operations used by the probe.
trait AccessHandles {
    /// Retained owned SCM handle, with the native close fallback.
    type Handle;

    /// Connect to the local active database with connect access.
    /// # Errors
    /// Preserve the original manager-open error.
    fn connect(&self) -> Result<Self::Handle, u32>;

    /// Open the fixed role with the one selected right.
    /// # Errors
    /// Preserve the original error from this service open.
    fn open(
        &self,
        manager: &Self::Handle,
        role: LabRole,
        right: Right,
    ) -> Result<Self::Handle, u32>;

    /// Close exactly the consumed handle.
    /// # Errors
    /// Return the failure reported by its native close operation.
    fn close(&self, handle: Self::Handle) -> Result<(), u32>;
}

/// Request one right and retire both handles before returning a result.
///
/// # Errors
/// Keep the first failure and any manager-close failure that followed it.
fn observe<A: AccessHandles>(api: &A, role: LabRole, right: Right) -> Result<(), Failure> {
    let manager: A::Handle = api.connect().map_err(|code: u32| Failure {
        stage: Stage::OpenManager,
        code,
        additional_manager_close: None,
    })?;
    let service_result: Result<(), (Stage, u32)> = match api.open(&manager, role, right) {
        Ok(service) => api
            .close(service)
            .map_err(|code: u32| (Stage::CloseService, code)),
        Err(code) => Err((Stage::OpenService, code)),
    };
    let manager_result: Result<(), u32> = api.close(manager);
    match (service_result, manager_result) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(code)) => Err(Failure {
            stage: Stage::CloseManager,
            code,
            additional_manager_close: None,
        }),
        (Err((stage, code)), other) => Err(Failure {
            stage,
            code,
            additional_manager_close: other.err(),
        }),
    }
}

/// Visit each required role/right observation in order.
/// # Errors
/// Stop at the first incomplete observation, without requesting further rights.
fn visit<E>(mut inspect: impl FnMut(LabRole, Right) -> Result<(), E>) -> Result<(), E> {
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        for right in [
            Right::QueryStatus,
            Right::QueryConfig,
            Right::ReadControl,
            Right::ChangeConfig,
            Right::Start,
            Right::Stop,
        ] {
            inspect(role, right)?;
        }
    }
    Ok(())
}

/// Own both fixed status handles for one observation and retire them in reverse order.
///
/// The body can borrow either original only after both opens succeed. Each finish
/// consumes one acquired handle and preserves any earlier failure beside its own
/// close failure. Return a successful body value only after both closes succeed.
///
/// # Errors
/// Preserve the first open or body failure and any independent close failures.
fn with_prestart_status_pair<H, T, E>(
    mut open_status: impl FnMut(LabRole) -> Result<H, E>,
    observe: impl FnOnce(&H, &H) -> Result<T, E>,
    mut finish: impl FnMut(H, Result<T, E>) -> Result<T, E>,
) -> Result<T, E> {
    let engine: H = open_status(LabRole::Engine)?;
    let result: Result<T, E> = match open_status(LabRole::ClockBroker) {
        Ok(broker) => {
            let observation: Result<T, E> = observe(&engine, &broker);
            finish(broker, observation)
        }
        Err(open_failure) => Err(open_failure),
    };
    finish(engine, result)
}

/// Read the two fixed observations and resample both originals before retirement.
///
/// Each final sample uses the corresponding observation from the same handle.
/// The caller may publish the pair after this operation returns it.
///
/// # Errors
/// Preserve the first open, observation or recheck failure and later close errors.
pub(super) fn observe_prestart_pair<H, T, E>(
    open_status: impl FnMut(LabRole) -> Result<H, E>,
    mut observe: impl FnMut(&H) -> Result<T, E>,
    mut recheck: impl FnMut(&H, &T) -> Result<(), E>,
    finish: impl FnMut(H, Result<[T; 2], E>) -> Result<[T; 2], E>,
) -> Result<[T; 2], E> {
    with_prestart_status_pair(
        open_status,
        |engine: &H, broker: &H| {
            let engine_observation: T = observe(engine)?;
            let broker_observation: T = observe(broker)?;
            recheck(engine, &engine_observation)?;
            recheck(broker, &broker_observation)?;
            Ok([engine_observation, broker_observation])
        },
        finish,
    )
}

/// Keep both original status objects through the access sequence and final samples.
///
/// The caller publishes completion only after both handles retire successfully.
///
/// # Errors
/// Preserve the first open, preamble, access or final-read failure and any
/// later status close failures. Failed access prevents later right acquisitions.
fn probe_access<H, E>(
    open_status: impl FnMut(LabRole) -> Result<H, E>,
    preamble: impl FnOnce(&H, &H) -> Result<[u32; 2], E>,
    inspect: impl FnMut(LabRole, Right) -> Result<(), E>,
    mut recheck: impl FnMut(&H, u32) -> Result<(), E>,
    finish: impl FnMut(H, Result<(), E>) -> Result<(), E>,
) -> Result<(), E> {
    with_prestart_status_pair(
        open_status,
        |engine: &H, broker: &H| {
            let [engine_type, broker_type]: [u32; 2] = preamble(engine, broker)?;
            visit(inspect)?;
            recheck(engine, engine_type)?;
            recheck(broker, broker_type)
        },
        finish,
    )
}

#[cfg(windows)]
mod windows {
    //! Bind the closed probe to actual Windows handles under the current process context.

    use core::{ffi::c_void, mem::ManuallyDrop, ptr::NonNull};
    use std::error::Error;

    use windows_sys::Win32::{
        Foundation::{ERROR_ACCESS_DENIED, ERROR_SERVICE_DOES_NOT_EXIST, GetLastError},
        Storage::FileSystem::READ_CONTROL,
        System::Services::{
            CloseServiceHandle, OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT,
            SERVICE_CHANGE_CONFIG, SERVICE_QUERY_CONFIG, SERVICE_QUERY_STATUS, SERVICE_START,
            SERVICE_STOP,
        },
    };

    use crate::{
        native_pipe_identity::ensure_unimpersonated,
        scm_observation_tests::{PrestartObservation, PrestartReadback, PrestartRoleStatus},
    };

    use super::{
        ACCESS_DENIED, AccessHandles, Failure, LabRole, Right, SERVICE_ABSENT, Stage, observe,
        probe_access,
    };

    const _: () = {
        assert!(ACCESS_DENIED == ERROR_ACCESS_DENIED);
        assert!(SERVICE_ABSENT == ERROR_SERVICE_DOES_NOT_EXIST);
        assert!(Right::QueryStatus.mask() == SERVICE_QUERY_STATUS);
        assert!(Right::QueryConfig.mask() == SERVICE_QUERY_CONFIG);
        assert!(Right::ReadControl.mask() == READ_CONTROL);
        assert!(Right::ChangeConfig.mask() == SERVICE_CHANGE_CONFIG);
        assert!(Right::Start.mask() == SERVICE_START);
        assert!(Right::Stop.mask() == SERVICE_STOP);
    };

    /// Own one private manager or service handle. No operation exposes its authority.
    struct Handle(NonNull<c_void>);

    impl Handle {
        /// Consume the handle exactly once, including if close fails.
        /// # Errors
        /// Preserve the native close failure.
        fn close(self) -> Result<(), u32> {
            close_handle(ManuallyDrop::new(self).0)
        }
    }

    impl Drop for Handle {
        fn drop(&mut self) {
            let _closed: Result<(), u32> = close_handle(self.0);
        }
    }

    /// Retire one uniquely owned SCM object.
    /// # Errors
    /// Preserve the original system code.
    #[expect(unsafe_code, reason = "Retire only a live uniquely owned SCM handle")]
    fn close_handle(handle: NonNull<c_void>) -> Result<(), u32> {
        // SAFETY: The owner only passes its unclosed manager or service handle.
        if unsafe { CloseServiceHandle(handle.as_ptr()) } == 0 {
            return Err(last_error());
        }
        Ok(())
    }

    /// Capture the calling thread's code immediately after an unsuccessful Win32 call.
    #[expect(unsafe_code, reason = "Read the native operation's thread-local error")]
    fn last_error() -> u32 {
        // SAFETY: GetLastError reads only the calling thread's slot.
        unsafe { GetLastError() }
    }

    /// Lab adapter with only acquisition and retirement operations.
    struct Native;

    impl AccessHandles for Native {
        type Handle = Handle;

        #[expect(unsafe_code, reason = "Connect to the local active SCM database only")]
        fn connect(&self) -> Result<Self::Handle, u32> {
            // SAFETY: Null names select the documented local machine and active database.
            let raw: *mut c_void =
                unsafe { OpenSCManagerW(core::ptr::null(), core::ptr::null(), SC_MANAGER_CONNECT) };
            NonNull::new(raw).map(Handle).ok_or_else(last_error)
        }

        #[expect(
            unsafe_code,
            reason = "Probe one fixed local service key and exact right"
        )]
        fn open(&self, manager: &Handle, role: LabRole, right: Right) -> Result<Handle, u32> {
            // SAFETY: The retained manager is live and the fixed role supplies
            // permanent NUL-terminated UTF-16. Only one SDK right is requested.
            let raw: *mut c_void = unsafe {
                OpenServiceW(manager.0.as_ptr(), role.wide_name().as_ptr(), right.mask())
            };
            NonNull::new(raw).map(Handle).ok_or_else(last_error)
        }

        fn close(&self, handle: Handle) -> Result<(), u32> {
            handle.close()
        }
    }

    /// Observe the exact requested rights in the signed, disabled S0.6 fixture.
    ///
    /// The supervisor independently binds the proposed actor/token, process and
    /// deployment. This selector uses the invoking process with no impersonation.
    ///
    /// # Errors
    /// Refuse an invalid pre-start observation or the first failed access/close.
    #[test]
    #[ignore = "requires the admitted installed S0.6 MSIX and proposed provisioning actor"]
    fn native_installed_roles_prestart_access() -> Result<(), Box<dyn Error>> {
        ensure_unimpersonated()?;
        probe_access(
            PrestartRoleStatus::open,
            |engine: &PrestartRoleStatus, broker: &PrestartRoleStatus| {
                let engine: PrestartObservation = engine.read_observation()?;
                engine.write();
                let broker: PrestartObservation = broker.read_observation()?;
                broker.write();
                println!("scm-prestart observed-rows=2");
                println!("scm-prestart-access invoking-pid={}", std::process::id());
                Ok([engine.status_service_type(), broker.status_service_type()])
            },
            |role: LabRole, right: Right| -> Result<(), Box<dyn Error>> {
                let mask: u32 = right.mask();
                let result: Result<(), Failure> = observe(&Native, role, right);
                match result {
                    Ok(()) => println!(
                        "scm-prestart-access {} mask={mask:#x} opened-and-closed",
                        role.name(),
                    ),
                    Err(failure) => return Err(format!(
                        "scm-prestart-access {} mask={mask:#x} {} stage={:?} code={} additional-manager-close={:?}",
                        role.name(), failure.kind(), failure.stage, failure.code, failure.additional_manager_close,
                    ).into()),
                }
                Ok(())
            },
            |status: &PrestartRoleStatus, raw_type: u32| {
                status.recheck(raw_type, PrestartReadback::Access)
            },
            PrestartRoleStatus::finish,
        )?;
        ensure_unimpersonated()?;
        println!("scm-prestart-access observed-rows=12");
        Ok(())
    }

    /// Compare status access with configuration denial on each same installed role.
    ///
    /// The supervisor binds the standard-user token and untouched package objects.
    /// The thread must use the invoking process's identity.
    ///
    /// # Errors
    /// Refuse failed status access, unexpected configuration access, a different
    /// denial stage, missing registration or unconfirmed handle closure.
    #[test]
    #[ignore = "requires the admitted installed S0.6 MSIX and standard-user actor"]
    fn native_standard_user_prestart_change_config_refused() -> Result<(), Box<dyn Error>> {
        ensure_unimpersonated()?;
        println!(
            "scm-prestart-standard-user invoking-pid={}",
            std::process::id()
        );
        for role in [LabRole::Engine, LabRole::ClockBroker] {
            observe(&Native, role, Right::QueryStatus).map_err(|failure: Failure| {
                format!(
                    "scm-prestart-standard-user {} status access: {failure:?}",
                    role.name()
                )
            })?;
            let access: Result<(), Failure> = observe(&Native, role, Right::ChangeConfig);
            if access
                != Err(Failure {
                    stage: Stage::OpenService,
                    code: ACCESS_DENIED,
                    additional_manager_close: None,
                })
            {
                return Err(format!(
                    "scm-prestart-standard-user {} configuration access: {access:?}",
                    role.name(),
                )
                .into());
            }
            println!(
                "scm-prestart-standard-user {} status-opened-and-closed configuration-denied code={ACCESS_DENIED}",
                role.name()
            );
        }
        ensure_unimpersonated()?;
        println!("scm-prestart-standard-user observed-rows=2");
        Ok(())
    }
}

/// Portable controls for the native probe's exact right selection and handle custody.
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::{
        AccessHandles, Failure, LabRole, Right, Stage, observe, observe_prestart_pair,
        probe_access, visit,
    };

    /// One call in the scripted ownership trace.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Call {
        /// Open the manager.
        Connect,
        /// Open the service with the actual requested role and mask.
        Open(LabRole, u32),
        /// Consume the manager (false) or service (true) handle.
        Close(bool),
    }

    /// Distinguish original and cleanup errors without performing native effects.
    struct Script {
        /// Ordered calls made by the actual observation owner.
        calls: RefCell<Vec<Call>>,
        /// Configured refusal of opening the manager or service.
        open_failure: Option<(Stage, u32)>,
        /// Optional failure from closing the service.
        service_close: Option<u32>,
        /// Optional failure from closing the manager.
        manager_close: Option<u32>,
    }

    impl AccessHandles for Script {
        type Handle = bool;

        fn connect(&self) -> Result<Self::Handle, u32> {
            self.calls.borrow_mut().push(Call::Connect);
            if let Some((Stage::OpenManager, code)) = self.open_failure {
                return Err(code);
            }
            Ok(false)
        }

        fn open(&self, manager: &bool, role: LabRole, right: Right) -> Result<bool, u32> {
            assert!(!manager);
            self.calls.borrow_mut().push(Call::Open(role, right.mask()));
            if let Some((Stage::OpenService, code)) = self.open_failure {
                return Err(code);
            }
            Ok(true)
        }

        fn close(&self, handle: bool) -> Result<(), u32> {
            self.calls.borrow_mut().push(Call::Close(handle));
            match if handle {
                self.service_close
            } else {
                self.manager_close
            } {
                Some(code) => Err(code),
                None => Ok(()),
            }
        }
    }

    /// Healthy acquisition closes both handles, and failures retain native order.
    /// # Panics
    /// Fails when an unopened handle is closed or the earliest failure is lost.
    #[test]
    fn access_observation_preserves_close_order_and_original_code() {
        let role: LabRole = LabRole::ClockBroker;
        let complete: [Call; 4] = [
            Call::Connect,
            Call::Open(role, 2),
            Call::Close(true),
            Call::Close(false),
        ];
        for (open_failure, service_close, manager_close, calls, expected) in [
            (None, None, None, &complete[..], None),
            (
                Some((Stage::OpenManager, 5)),
                None,
                None,
                &complete[..1],
                Some((Stage::OpenManager, 5, None, "access-denied")),
            ),
            (
                Some((Stage::OpenService, 1060)),
                None,
                Some(6),
                &[Call::Connect, Call::Open(role, 2), Call::Close(false)],
                Some((Stage::OpenService, 1060, Some(6), "service-missing")),
            ),
            (
                Some((Stage::OpenService, 5)),
                None,
                None,
                &[Call::Connect, Call::Open(role, 2), Call::Close(false)],
                Some((Stage::OpenService, 5, None, "access-denied")),
            ),
            (
                None,
                Some(7),
                Some(6),
                &complete[..],
                Some((Stage::CloseService, 7, Some(6), "failed")),
            ),
            (
                None,
                None,
                Some(6),
                &complete[..],
                Some((Stage::CloseManager, 6, None, "failed")),
            ),
        ] {
            let api: Script = Script {
                calls: RefCell::default(),
                open_failure,
                service_close,
                manager_close,
            };
            let result: Result<(), Failure> = observe(&api, role, Right::ChangeConfig);
            assert_eq!(*api.calls.borrow(), calls);
            assert_eq!(
                result.err().map(|failure: Failure| (
                    failure.stage,
                    failure.code,
                    failure.additional_manager_close,
                    failure.kind()
                )),
                expected
            );
        }
    }

    /// Check every ordered right and ensure a required refusal ends the visit.
    /// # Panics
    /// Fails when the requested mask, role, ordering or stop behavior changes.
    #[test]
    fn access_probe_uses_separate_masks_and_stops_at_first_refusal() {
        let mut observed: Vec<(LabRole, u32)> = Vec::new();
        let result: Result<(), u32> = visit(|role: LabRole, right: Right| {
            observed.push((role, right.mask()));
            Ok(())
        });
        let masks: [u32; 6] = [4, 1, 131_072, 2, 16, 32];
        let expected: Vec<(LabRole, u32)> = [LabRole::Engine, LabRole::ClockBroker]
            .into_iter()
            .flat_map(|role: LabRole| masks.map(|mask: u32| (role, mask)))
            .collect();
        assert_eq!(result, Ok(()));
        assert_eq!(observed, expected);
        observed.clear();
        let blocked: Result<(), u32> = visit(|role: LabRole, right: Right| {
            observed.push((role, right.mask()));
            if role == LabRole::Engine && right == Right::ChangeConfig {
                return Err(5);
            }
            Ok(())
        });
        assert_eq!(blocked, Err(5));
        assert_eq!(observed, expected[..4]);
    }

    /// Operations in the retained-status scope, without any native service effects.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ProbeCall {
        /// Acquire the role's status-only object.
        Open(LabRole),
        /// Observe both acquired objects before access acquisition.
        Preamble([LabRole; 2]),
        /// Read one complete role observation while the pair is held.
        Observation(LabRole),
        /// Acquire and close one selected access right.
        Right(LabRole, u32),
        /// Resample the retained role and compare with its original raw type.
        Recheck(LabRole, u32),
        /// Explicitly retire the acquired status object.
        Close(LabRole),
    }

    /// Original failure followed by each independently reported close failure.
    #[derive(Debug, Eq, PartialEq)]
    struct ProbeErrors {
        /// Original failure with its recorded operation and code.
        first: (ProbeCall, u32),
        /// Later failures in the original retirement order.
        additional_closes: Vec<(ProbeCall, u32)>,
    }

    /// Bounded operation history with scripted failures on exact calls.
    struct ProbeScript<'a> {
        /// Observed calls, in execution order.
        trace: RefCell<Vec<ProbeCall>>,
        /// Injected failures, each with its native-like raw code.
        failures: &'a [(ProbeCall, u32)],
    }

    impl ProbeScript<'_> {
        /// Record the operation and return its scripted failure, if any.
        /// # Errors
        /// Return the specified call and code without performing native work.
        fn record(&self, call: ProbeCall) -> Result<(), ProbeErrors> {
            self.trace.borrow_mut().push(call);
            if let Some(&(_, code)) = self.failures.iter().find(|&&(failed, _)| failed == call) {
                return Err(ProbeErrors {
                    first: (call, code),
                    additional_closes: Vec::new(),
                });
            }
            Ok(())
        }

        /// Consume the simulated handle and record its close outcome.
        /// # Errors
        /// Preserve the original failure and append this close error if both fail.
        fn finish<T>(
            &self,
            role: LabRole,
            prior: Result<T, ProbeErrors>,
        ) -> Result<T, ProbeErrors> {
            let closed: Result<(), ProbeErrors> = self.record(ProbeCall::Close(role));
            match (prior, closed) {
                (Err(mut first), Err(cleanup)) => {
                    first.additional_closes.push(cleanup.first);
                    Err(first)
                }
                (result, Ok(())) => result,
                (Ok(_), Err(cleanup)) => Err(cleanup),
            }
        }
    }

    /// Exercise the access selector's own scope and ordered visitor using recorded calls.
    fn run_probe(failures: &[(ProbeCall, u32)]) -> (Vec<ProbeCall>, Result<(), ProbeErrors>) {
        let script: ProbeScript<'_> = ProbeScript {
            trace: RefCell::default(),
            failures,
        };
        let result: Result<(), ProbeErrors> = probe_access(
            |role: LabRole| script.record(ProbeCall::Open(role)).map(|()| role),
            |engine: &LabRole, broker: &LabRole| {
                script
                    .record(ProbeCall::Preamble([*engine, *broker]))
                    .map(|()| [16, 272])
            },
            |role: LabRole, right: Right| script.record(ProbeCall::Right(role, right.mask())),
            |role: &LabRole, raw_type: u32| script.record(ProbeCall::Recheck(*role, raw_type)),
            |role: LabRole, prior: Result<(), ProbeErrors>| script.finish(role, prior),
        );
        (script.trace.into_inner(), result)
    }

    /// Exercise the read-only selector's paired preparation and retirement.
    fn run_observation_probe(
        failures: &[(ProbeCall, u32)],
    ) -> (Vec<ProbeCall>, Result<[u32; 2], ProbeErrors>) {
        let script: ProbeScript<'_> = ProbeScript {
            trace: RefCell::default(),
            failures,
        };
        let result: Result<[u32; 2], ProbeErrors> = observe_prestart_pair(
            |role: LabRole| script.record(ProbeCall::Open(role)).map(|()| role),
            |role: &LabRole| {
                script.record(ProbeCall::Observation(*role))?;
                match role {
                    LabRole::Engine => Ok(16),
                    LabRole::ClockBroker => Ok(272),
                }
            },
            |role: &LabRole, raw_type: &u32| script.record(ProbeCall::Recheck(*role, *raw_type)),
            |role: LabRole, prior: Result<[u32; 2], ProbeErrors>| script.finish(role, prior),
        );
        (script.trace.into_inner(), result)
    }

    /// Both original handles stay open through both whole observations and final samples.
    /// # Panics
    /// Fail if an operation is skipped, reordered or returns a different pair.
    #[test]
    fn prestart_observation_returns_pair_after_both_status_handles_close() {
        use LabRole::{ClockBroker, Engine};

        let (calls, result): (Vec<ProbeCall>, Result<[u32; 2], ProbeErrors>) =
            run_observation_probe(&[]);
        assert_eq!(
            calls,
            [
                ProbeCall::Open(Engine),
                ProbeCall::Open(ClockBroker),
                ProbeCall::Observation(Engine),
                ProbeCall::Observation(ClockBroker),
                ProbeCall::Recheck(Engine, 16),
                ProbeCall::Recheck(ClockBroker, 272),
                ProbeCall::Close(ClockBroker),
                ProbeCall::Close(Engine),
            ],
        );
        assert_eq!(result, Ok([16, 272]));
    }

    /// Failures withhold the pair and keep the first cause and later close errors.
    /// # Panics
    /// Fail if an incomplete observation returns a pair or loses a failure.
    #[test]
    fn prestart_observation_refuses_partial_open_late_change_and_close_failure() {
        use LabRole::{ClockBroker, Engine};

        let engine_close: (ProbeCall, u32) = (ProbeCall::Close(Engine), 61);
        let broker_close: (ProbeCall, u32) = (ProbeCall::Close(ClockBroker), 62);
        for (primary, additional_closes, expected_calls) in [
            (
                (ProbeCall::Open(ClockBroker), 1060),
                &[engine_close][..],
                &[
                    ProbeCall::Open(Engine),
                    ProbeCall::Open(ClockBroker),
                    ProbeCall::Close(Engine),
                ][..],
            ),
            (
                // The Engine status snapshot is withdrawn while observing the Broker.
                (ProbeCall::Recheck(Engine, 16), 33),
                &[broker_close, engine_close][..],
                &[
                    ProbeCall::Open(Engine),
                    ProbeCall::Open(ClockBroker),
                    ProbeCall::Observation(Engine),
                    ProbeCall::Observation(ClockBroker),
                    ProbeCall::Recheck(Engine, 16),
                    ProbeCall::Close(ClockBroker),
                    ProbeCall::Close(Engine),
                ][..],
            ),
            (
                broker_close,
                &[engine_close][..],
                &[
                    ProbeCall::Open(Engine),
                    ProbeCall::Open(ClockBroker),
                    ProbeCall::Observation(Engine),
                    ProbeCall::Observation(ClockBroker),
                    ProbeCall::Recheck(Engine, 16),
                    ProbeCall::Recheck(ClockBroker, 272),
                    ProbeCall::Close(ClockBroker),
                    ProbeCall::Close(Engine),
                ][..],
            ),
        ] {
            let mut failures: Vec<(ProbeCall, u32)> = vec![primary];
            failures.extend_from_slice(additional_closes);
            let (calls, result): (Vec<ProbeCall>, Result<[u32; 2], ProbeErrors>) =
                run_observation_probe(&failures);
            assert_eq!(calls, expected_calls);
            assert_eq!(
                result,
                Err(ProbeErrors {
                    first: primary,
                    additional_closes: additional_closes.to_vec()
                }),
            );
        }
    }

    /// Both roles stay open through every ordered right and each final read.
    /// # Panics
    /// Fail if a status handle is closed early or a required operation is omitted.
    #[test]
    fn prestart_access_holds_both_roles_through_rights_and_final_reads() {
        use LabRole::{ClockBroker, Engine};

        let (calls, result): (Vec<ProbeCall>, Result<(), ProbeErrors>) = run_probe(&[]);
        assert_eq!(result, Ok(()));
        assert_eq!(
            calls,
            [
                ProbeCall::Open(Engine),
                ProbeCall::Open(ClockBroker),
                ProbeCall::Preamble([Engine, ClockBroker]),
                ProbeCall::Right(Engine, 4),
                ProbeCall::Right(Engine, 1),
                ProbeCall::Right(Engine, 131_072),
                ProbeCall::Right(Engine, 2),
                ProbeCall::Right(Engine, 16),
                ProbeCall::Right(Engine, 32),
                ProbeCall::Right(ClockBroker, 4),
                ProbeCall::Right(ClockBroker, 1),
                ProbeCall::Right(ClockBroker, 131_072),
                ProbeCall::Right(ClockBroker, 2),
                ProbeCall::Right(ClockBroker, 16),
                ProbeCall::Right(ClockBroker, 32),
                ProbeCall::Recheck(Engine, 16),
                ProbeCall::Recheck(ClockBroker, 272),
                ProbeCall::Close(ClockBroker),
                ProbeCall::Close(Engine),
            ]
        );
    }

    /// First refusal stops the operation and retires only actually acquired handles.
    /// # Panics
    /// Fail if a later observation runs or a cleanup failure changes the first error.
    #[test]
    fn prestart_access_preserves_first_refusal_and_all_acquired_closes() {
        use LabRole::{ClockBroker, Engine};

        let (healthy, _): (Vec<ProbeCall>, Result<(), ProbeErrors>) = run_probe(&[]);
        let engine_close: (ProbeCall, u32) = (ProbeCall::Close(Engine), 61);
        let broker_close: (ProbeCall, u32) = (ProbeCall::Close(ClockBroker), 62);
        for (index, primary, extra_closes) in [
            (0, (ProbeCall::Open(Engine), 5), &[][..]),
            (1, (ProbeCall::Open(ClockBroker), 1060), &[engine_close][..]),
            (
                2,
                (ProbeCall::Preamble([Engine, ClockBroker]), 21),
                &[broker_close, engine_close][..],
            ),
            (
                6,
                (ProbeCall::Right(Engine, 2), 5),
                &[broker_close, engine_close][..],
            ),
            (
                15,
                (ProbeCall::Recheck(Engine, 16), 33),
                &[broker_close, engine_close][..],
            ),
            (
                16,
                (ProbeCall::Recheck(ClockBroker, 272), 34),
                &[broker_close, engine_close][..],
            ),
            (17, broker_close, &[engine_close][..]),
            (18, engine_close, &[][..]),
        ] {
            let mut failed: Vec<(ProbeCall, u32)> = vec![primary];
            failed.extend_from_slice(extra_closes);
            let (observed, result): (Vec<ProbeCall>, Result<(), ProbeErrors>) = run_probe(&failed);
            let mut expected: Vec<ProbeCall> = healthy[..=index].to_vec();
            expected.extend(extra_closes.iter().map(|&(call, _)| call));
            assert_eq!(observed, expected);
            assert_eq!(
                result,
                Err(ProbeErrors {
                    first: primary,
                    additional_closes: extra_closes.to_vec()
                })
            );
        }
    }
}
