// SPDX-License-Identifier: GPL-2.0-only
//! Read-only checks against the SCM in the owned offline Windows guest.

use std::{
    error::Error,
    fmt,
    num::NonZeroU32,
    os::windows::{ffi::OsStrExt, io::AsHandle},
};

use windows_platform_native::{
    ObservedServicePeer, ReportedDacl, ReportedServiceConfiguration, ReportedServiceSecurity,
    ReportedServiceStatus, RunningServiceRefusal, ServiceConfigurationQuery, ServiceKey,
    ServicePeerObservationError, ServiceProcessObservationError, ServiceQueryError,
    ServiceSecurityQuery, ServiceStatusQuery, read_connected_pipe_server_pid,
};

use crate::{
    LabRole,
    native_pipe_pair::{
        LabIdentificationPipe, LabOverlappedWriter, LabPipeReader, open_lab_duplex_pipe,
    },
};

/// Two fixed, existing services in the otherwise unmodified offline guest.
const CONFIGURATION_REFERENCES: [(&str, &[u8], u32); 2] = [
    ("RemoteRegistry", b"RemoteRegistry\0", 4),
    ("SharedAccess", b"SharedAccess\0", 3),
];

/// Last status reported by a service that is stopped.
const SERVICE_STOPPED: u32 = 1;

/// A fixed installed-fixture observation failed at one operation.
#[derive(Debug)]
struct PrestartQueryFailure {
    /// Laboratory key selected by the source, with no user-chosen target.
    role: LabRole,
    /// Fixed observation stage, including which status endpoint failed.
    stage: &'static str,
    /// First typed native or adapter failure, including its raw code.
    primary: ServiceQueryError,
    /// The original query failed and its explicit close also failed.
    close_failure: Option<ServiceQueryError>,
}

impl fmt::Display for PrestartQueryFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "SCM pre-start {} {}: {}",
            self.role.name(),
            self.stage,
            self.primary,
        )?;
        if let Some(close_failure) = self.close_failure {
            write!(formatter, " (additional close failure: {close_failure})")?;
        }
        Ok(())
    }
}

impl Error for PrestartQueryFailure {}

/// An observation refused before the retained status handle also failed to close.
#[derive(Debug)]
struct PrestartStatusCloseFailure {
    /// The original refusal, including any separate configuration or security close failure.
    primary: Box<dyn Error>,
    /// Raw failure from explicitly closing the retained status handle.
    status_close: ServiceQueryError,
}

impl fmt::Display for PrestartStatusCloseFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} (additional status close failure: {})",
            self.primary, self.status_close,
        )
    }
}

impl Error for PrestartStatusCloseFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.primary.as_ref())
    }
}

/// Attribute an observer open or manager-close failure to its fixed lane.
fn prestart_open_failure(
    role: LabRole,
    stage: &'static str,
    primary: ServiceQueryError,
) -> PrestartQueryFailure {
    PrestartQueryFailure {
        role,
        stage,
        primary,
        close_failure: None,
    }
}

/// Pair a bounded read with its explicit handle-close acknowledgement.
///
/// # Errors
/// Return the first error and keep any concurrent close failure separately.
fn finish_prestart_query<T>(
    role: LabRole,
    stage: &'static str,
    result: Result<T, ServiceQueryError>,
    closed: Result<(), ServiceQueryError>,
) -> Result<T, PrestartQueryFailure> {
    match (result, closed) {
        (Ok(report), Ok(())) => Ok(report),
        (Ok(_), Err(error)) => Err(prestart_open_failure(role, stage, error)),
        (Err(primary), closed) => Err(PrestartQueryFailure {
            role,
            stage,
            primary,
            close_failure: closed.err(),
        }),
    }
}

/// Publish a staged observation only after its retained status handle closes.
///
/// # Errors
/// Return the first refusal and keep any independent status close error.
fn finish_prestart_status<T>(
    role: LabRole,
    result: Result<T, Box<dyn Error>>,
    closed: Result<(), ServiceQueryError>,
) -> Result<T, Box<dyn Error>> {
    match (result, closed) {
        (result, Ok(())) => result,
        (Ok(_), Err(error)) => Err(prestart_open_failure(role, "status close", error).into()),
        (Err(primary), Err(status_close)) => Err(PrestartStatusCloseFailure {
            primary,
            status_close,
        }
        .into()),
    }
}

/// Verify that a closed observation can publish its value and failed closure cannot.
///
/// # Errors
/// Report an unexpected observation or incomplete failure custody.
#[test]
fn prestart_retains_primary_query_and_close_results() -> Result<(), Box<dyn Error>> {
    let role: LabRole = LabRole::ClockBroker;
    let stage: &str = "configuration read/close";
    let query: ServiceQueryError = ServiceQueryError::QueryConfiguration(5);
    let close: ServiceQueryError = ServiceQueryError::CloseService(6);
    let value: u32 = finish_prestart_query(role, stage, Ok(3_u32), Ok(()))?;
    if value != 3 {
        return Err("closed observation lost its value".into());
    }
    for (reading, retirement, primary, additional) in [
        (Err(query), Ok(()), query, None),
        (Ok(3_u32), Err(close), close, None),
        (Err(query), Err(close), query, Some(close)),
    ] {
        let result: Result<u32, PrestartQueryFailure> =
            finish_prestart_query(role, stage, reading, retirement);
        let Err(failure) = result else {
            return Err("failed observation returned a value".into());
        };
        if (
            failure.role,
            failure.stage,
            failure.primary,
            failure.close_failure,
        ) != (role, stage, primary, additional)
        {
            return Err("pre-start observation lost its attributed failure".into());
        }
    }

    let status_close: ServiceQueryError = ServiceQueryError::CloseService(7);
    let value: u32 = finish_prestart_status(role, Ok(4_u32), Ok(()))?;
    if value != 4 {
        return Err("closed status observation lost its value".into());
    }
    let Err(failure) = finish_prestart_status(role, Ok(4_u32), Err(status_close)) else {
        return Err("failed status close published an observation".into());
    };
    let attributed: &PrestartQueryFailure = failure
        .downcast_ref()
        .ok_or("status close did not retain a typed failure")?;
    if (attributed.role, attributed.stage, attributed.primary)
        != (role, "status close", status_close)
    {
        return Err("status close did not retain its stage and code".into());
    }
    let read: Result<u32, Box<dyn Error>> =
        finish_prestart_query(role, stage, Err(query), Err(close)).map_err(Into::into);
    let Err(failure) = finish_prestart_status(role, read, Err(status_close)) else {
        return Err("two failed closes published an observation".into());
    };
    let combined: &PrestartStatusCloseFailure = failure
        .downcast_ref()
        .ok_or("missing independent status close failure")?;
    let first: &PrestartQueryFailure = combined
        .source()
        .and_then(<dyn Error>::downcast_ref)
        .ok_or("lost the first failing query")?;
    if (
        first.role,
        first.stage,
        first.primary,
        first.close_failure,
        combined.status_close,
    ) != (role, stage, query, Some(close), status_close)
    {
        return Err("pre-start observation lost a query or close failure".into());
    }
    Ok(())
}

/// Configured settings gathered through the same configuration-query handle.
struct PrestartConfiguration {
    /// Startup and identity settings from the original configuration result.
    startup: ReportedServiceConfiguration,
    /// Stored service SID type as a raw Windows DWORD.
    sid_type: u32,
    /// Literal configured privilege units with absence kept distinct.
    privileges: Option<Box<[u16]>>,
}

/// Read the three configuration results in order and close the handle.
///
/// # Errors
/// Refuse a non-Disabled startup mode before the ancillary queries. Preserve
/// the first refusal or native failure and any additional close failure.
fn read_prestart_configuration(
    role: LabRole,
    key: ServiceKey<'_>,
) -> Result<PrestartConfiguration, Box<dyn Error>> {
    let stage: &str = "configuration read/close";
    let handle: ServiceConfigurationQuery = ServiceConfigurationQuery::open_local(key)
        .map_err(|error| prestart_open_failure(role, "configuration open", error))?;
    let startup: ReportedServiceConfiguration = match handle.query() {
        Ok(startup) => startup,
        Err(error) => {
            return finish_prestart_query(role, stage, Err(error), handle.close())
                .map_err(Into::into);
        }
    };
    if !startup.starts_disabled() {
        let closed: Result<(), ServiceQueryError> = handle.close();
        let refusal: String = format!(
            "pre-start role {} reported configured type={} start={} before ancillary readback",
            role.name(),
            startup.service_type(),
            startup.start_type(),
        );
        return match closed {
            Ok(()) => Err(refusal.into()),
            Err(error) => Err(format!("{refusal} (additional close failure: {error})").into()),
        };
    }
    let result: Result<PrestartConfiguration, ServiceQueryError> = (|| {
        Ok(PrestartConfiguration {
            startup,
            sid_type: handle.query_service_sid_type()?,
            privileges: handle.query_required_privileges()?,
        })
    })();
    let closed: Result<(), ServiceQueryError> = handle.close();
    finish_prestart_query(role, stage, result, closed).map_err(Into::into)
}

/// Capture a separate owner/DACL observation under the existing read-control right.
///
/// # Errors
/// Preserve the first native failure and any additional close failure.
fn read_prestart_security(
    role: LabRole,
    key: ServiceKey<'_>,
) -> Result<ReportedServiceSecurity, PrestartQueryFailure> {
    let handle: ServiceSecurityQuery = ServiceSecurityQuery::open_local(key)
        .map_err(|error| prestart_open_failure(role, "security open", error))?;
    let result: Result<ReportedServiceSecurity, ServiceQueryError> = handle.query();
    let closed: Result<(), ServiceQueryError> = handle.close();
    finish_prestart_query(role, "security read/close", result, closed)
}

/// Complete readback for one role, ready to publish after explicit closure.
struct PrestartObservation {
    /// First status sampled on the retained status handle.
    before: ReportedServiceStatus,
    /// Configured fields read through a separate configuration-only handle.
    configuration: PrestartConfiguration,
    /// Security fields read through a separate read-control handle.
    security: ReportedServiceSecurity,
    /// Last status sampled on the original status handle.
    after: ReportedServiceStatus,
}

/// Bracket narrow configuration and security reads with the same status handle.
///
/// The retained status handle prevents deletion and replacement of the SCM
/// record during the separate opens, per the Windows `DeleteService` contract.
/// Both stopped observations are samples and do not establish inactivity between them.
///
/// # Errors
/// Refuse a non-Stopped sample or observation failure and preserve a separate
/// status close failure before a caller can publish a row.
fn read_prestart_observation(
    role: LabRole,
    key: ServiceKey<'_>,
) -> Result<PrestartObservation, Box<dyn Error>> {
    let status: ServiceStatusQuery = ServiceStatusQuery::open_local(key)
        .map_err(|error| prestart_open_failure(role, "status before", error))?;
    let result: Result<PrestartObservation, Box<dyn Error>> = (|| {
        let before: ReportedServiceStatus = status
            .query()
            .map_err(|error| prestart_open_failure(role, "status before", error))?;
        if before.state() != SERVICE_STOPPED {
            return Err(format!(
                "pre-start role {} reported type={} state={} before configuration",
                role.name(),
                before.service_type(),
                before.state(),
            )
            .into());
        }
        let configuration: PrestartConfiguration = read_prestart_configuration(role, key)?;
        let security: ReportedServiceSecurity = read_prestart_security(role, key)?;
        let after: ReportedServiceStatus = status
            .query()
            .map_err(|error| prestart_open_failure(role, "status after", error))?;
        if after.state() != SERVICE_STOPPED {
            return Err(format!(
                "pre-start role {} reported type={} state={} after security readback",
                role.name(),
                after.service_type(),
                after.state(),
            )
            .into());
        }
        Ok(PrestartObservation {
            before,
            configuration,
            security,
            after,
        })
    })();
    finish_prestart_status(role, result, status.close())
}

/// Observe both untouched installed fixture roles for §7.5 step 2.
///
/// The supervisor owns installation, independent installed image and package
/// evidence, actor identity, and continuous activation evidence. This selector
/// reports two sampled endpoints and redacted configuration/security shape.
///
/// # Errors
/// Stop after the first failed or non-disabled/non-stopped role. Only two
/// completed rows and the terminal marker constitute complete readback.
#[test]
#[ignore = "requires the separately admitted installed S0.6 MSIX and its owned guest"]
pub(super) fn native_installed_roles_prestart_observation() -> Result<(), Box<dyn Error>> {
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        let key: ServiceKey<'_> =
            ServiceKey::new(role.wide_name()).map_err(|_| "invalid fixed lab role key")?;
        let PrestartObservation {
            before,
            configuration,
            security,
            after,
        } = read_prestart_observation(role, key)?;
        let (dacl_kind, dacl_bytes): (&str, usize) = match security.dacl() {
            ReportedDacl::Absent => ("absent", 0),
            ReportedDacl::Null => ("null", 0),
            ReportedDacl::Present(dacl) => ("present", dacl.len()),
        };
        println!(
            concat!(
                "scm-prestart {} configured-type={} start={} error={} ",
                "before-type={} before-state={} after-type={} after-state={} sid-type={} ",
                "path-units={:?} account-units={:?} privilege-units={:?} ",
                "security-control={} owner-bytes={} dacl={} dacl-bytes={}"
            ),
            role.name(),
            configuration.startup.service_type(),
            configuration.startup.start_type(),
            configuration.startup.error_control(),
            before.service_type(),
            before.state(),
            after.service_type(),
            after.state(),
            configuration.sid_type,
            configuration.startup.binary_path().map(<[u16]>::len),
            configuration.startup.start_name().map(<[u16]>::len),
            configuration.privileges.as_deref().map(<[u16]>::len),
            security.control(),
            security.owner_sid().len(),
            dacl_kind,
            dacl_bytes,
        );
    }
    println!("scm-prestart observed-rows=2");
    Ok(())
}

/// Compare missing, running own-process, and mixed service types in the same guest.
///
/// # Errors
/// Return the original native failure or reject a fixture without the expected keys.
///
/// # Panics
/// Panic when a service reports an unexpected type, state or candidate PID.
#[test]
#[ignore = "requires the offline guest's known CryptSvc and Spooler status and an absent lab broker"]
fn native_status_only_query_matches_scm() -> Result<(), Box<dyn Error>> {
    let missing: ServiceKey<'_> = ServiceKey::new(LabRole::ClockBroker.wide_name())
        .map_err(|_| "invalid fixed lab broker key")?;
    match ServiceStatusQuery::open_local(missing) {
        Err(ServiceQueryError::OpenService(1060)) => (),
        Err(error) => return Err(error.into()),
        Ok(service) => {
            service.close()?;
            return Err("the lab broker must be absent for this fixture".into());
        }
    }

    let wide: [u16; 9] = (*b"CryptSvc\0").map(u16::from);
    let key: ServiceKey<'_> = ServiceKey::new(&wide).map_err(|_| "invalid fixed reference key")?;
    let service: ServiceStatusQuery = ServiceStatusQuery::open_local(key)?;
    let first: ReportedServiceStatus = service.query()?;
    let second: ReportedServiceStatus = service.query()?;
    let observed: Result<NonZeroU32, Box<dyn Error>> =
        check_live_service_pipe_mismatch(&service, first, second);
    let pid: NonZeroU32 = finish_prestart_status(LabRole::ClockBroker, observed, service.close())?;

    let wide: [u16; 8] = (*b"Spooler\0").map(u16::from);
    let key: ServiceKey<'_> = ServiceKey::new(&wide).map_err(|_| "invalid fixed mixed-type key")?;
    let mixed: ServiceStatusQuery = ServiceStatusQuery::open_local(key)?;
    let status: ReportedServiceStatus = mixed.query()?;
    mixed.close()?;
    assert_eq!((status.service_type(), status.state()), (0x110, 4));
    assert_eq!(
        status.reported_running_process(),
        Err(RunningServiceRefusal::NotOwnProcess)
    );
    println!("scm-observation missing=1060 running-pid={pid} mixed-type=110-refused");
    println!("scm-pipe-observation live-foreign-server=refused");
    Ok(())
}

/// Refuse a live fixture-owned pipe whose server differs from the SCM report.
///
/// Both pipe directions stay owned through observation. No bytes are exchanged
/// and the reference service is used only for status queries.
/// # Errors
/// Retain query failures, or reject an unexpected fixture identity or result.
fn check_live_service_pipe_mismatch(
    service: &ServiceStatusQuery,
    first: ReportedServiceStatus,
    second: ReportedServiceStatus,
) -> Result<NonZeroU32, Box<dyn Error>> {
    if first != second || (first.service_type(), first.state()) != (0x10, 4) {
        return Err("reference service did not report stable own-process Running status".into());
    }
    let reported: NonZeroU32 = first
        .reported_running_process()
        .map_err(ServiceProcessObservationError::IneligibleStatus)?;
    let (_request, reply_reader, _reply_writer): (
        LabIdentificationPipe,
        LabPipeReader,
        LabOverlappedWriter,
    ) = open_lab_duplex_pipe(53)?.into_transport();
    let pipe_pid: NonZeroU32 = read_connected_pipe_server_pid(reply_reader.as_handle())?;
    if pipe_pid.get() != std::process::id() || pipe_pid == reported {
        return Err("fixture pipe did not report the expected different live process".into());
    }
    match ObservedServicePeer::observe(service, reply_reader.as_handle()) {
        Err(ServicePeerObservationError::PipeProcessMismatch) => Ok(reported),
        Err(error) => Err(error.into()),
        Ok(_) => Err("live fixture pipe was incorrectly bound to the reference service".into()),
    }
}

/// Exercise the configuration part of the §7.5 probe against fixed guest keys.
/// The Windows supervisor compares the raw rows with independent `sc.exe qc`
/// observations before and after execution, including each stopped reference.
/// # Errors
/// Preserve query or explicit retirement failures and reject the wrong fixture state.
#[test]
#[ignore = "requires the offline guest's two stopped reference services and absent lab roles"]
fn native_configuration_query_matches_scm() -> Result<(), Box<dyn Error>> {
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        let key: ServiceKey<'_> =
            ServiceKey::new(role.wide_name()).map_err(|_| "invalid fixed lab role key")?;
        match ServiceConfigurationQuery::open_local(key) {
            Err(ServiceQueryError::OpenService(1060)) => (),
            Err(error) => return Err(error.into()),
            Ok(service) => {
                service.close()?;
                return Err("both lab services must be absent for this fixture".into());
            }
        }
    }

    for (label, name, expected_start) in CONFIGURATION_REFERENCES {
        let wide: Vec<u16> = name.iter().copied().map(u16::from).collect();
        let key: ServiceKey<'_> =
            ServiceKey::new(&wide).map_err(|_| "invalid fixed reference key")?;
        let configuration: ReportedServiceConfiguration = read_configuration(key)?;

        let status: ServiceStatusQuery = ServiceStatusQuery::open_local(key)?;
        let observed: Result<ReportedServiceStatus, ServiceQueryError> = status.query();
        let closed: Result<(), ServiceQueryError> = status.close();
        let observed: ReportedServiceStatus = observed?;
        closed?;
        if observed.state() != 1 {
            return Err("reference service was not stopped".into());
        }

        if configuration.start_type() != expected_start
            || configuration.starts_disabled() != (expected_start == 4)
        {
            return Err("configured startup mode did not match the reference".into());
        }
        println!(
            "scm-configuration {label} type={} start={} error={} stopped=1",
            configuration.service_type(),
            configuration.start_type(),
            configuration.error_control(),
        );
    }
    println!("scm-configuration missing-roles=2");
    Ok(())
}

/// Compare returned strings to independently supplied `Win32_Service` values.
/// # Errors
/// Retain query and closure failures, or identify the mismatching field without
/// printing any path, argument or account units.
#[test]
#[ignore = "requires independently observed WMI expectations in the offline guest"]
fn native_configured_service_identity_matches_wmi() -> Result<(), Box<dyn Error>> {
    for (label, name, _) in CONFIGURATION_REFERENCES {
        let wide: Vec<u16> = name.iter().copied().map(u16::from).collect();
        let key: ServiceKey<'_> =
            ServiceKey::new(&wide).map_err(|_| "invalid fixed reference key")?;
        let configuration: ReportedServiceConfiguration = read_configuration(key)?;
        for (field, observed) in [
            ("PATH", configuration.binary_path()),
            ("START_NAME", configuration.start_name()),
        ] {
            let expected: std::ffi::OsString =
                std::env::var_os(format!("NEVROTICK_SCM_{label}_{field}"))
                    .ok_or("missing independent service identity expectation")?;
            let observed: &[u16] = observed.ok_or("missing configured service identity")?;
            if observed.is_empty()
                || observed.len() > 1024
                || !expected.encode_wide().eq(observed.iter().copied())
            {
                return Err(format!("configured identity differs: {label} {field}").into());
            }
        }
        println!("scm-configured-identity {label} fields=2");
    }
    Ok(())
}

/// Compare configured privileges against independent before/after SCM observations.
///
/// The supervisor supplies `absent` or `present:` followed by literal names
/// separated by `|`. It determines presence independently because a rendered
/// list with no names need not distinguish absent from explicitly empty.
///
/// # Errors
/// Preserve the open, query and explicit close failures, or reject a mismatch
/// without reporting the observed names.
#[test]
#[ignore = "requires independent configured privilege observations in the offline guest"]
fn native_configured_required_privileges_match_scm() -> Result<(), Box<dyn Error>> {
    for (label, name, _) in CONFIGURATION_REFERENCES {
        let expected: Option<Vec<u16>> = expected_privilege_multistring(label)?;
        let wide: Vec<u16> = name.iter().copied().map(u16::from).collect();
        let key: ServiceKey<'_> =
            ServiceKey::new(&wide).map_err(|_| "invalid fixed reference key")?;
        let service: ServiceConfigurationQuery = ServiceConfigurationQuery::open_local(key)?;
        let reported: Result<Option<Box<[u16]>>, ServiceQueryError> =
            service.query_required_privileges();
        let closed: Result<(), ServiceQueryError> = service.close();
        let reported: Option<Box<[u16]>> = reported?;
        closed?;
        if reported.as_deref() != expected.as_deref() {
            return Err(format!("configured privileges differ: {label}").into());
        }
        println!("scm-configured-privileges {label} matched=1");
    }
    Ok(())
}

/// Compare the configured service-SID type with independent SCM observations.
/// The supervisor supplies the exact decimal DWORD for each fixed reference.
///
/// # Errors
/// Reject invalid expectations, a mismatch, or the first native failure.
#[test]
#[ignore = "requires independently observed service-SID types in the offline guest"]
fn native_configured_service_sid_type_matches_scm() -> Result<(), Box<dyn Error>> {
    for (label, name, _) in CONFIGURATION_REFERENCES {
        let expected: u32 = std::env::var(format!("NEVROTICK_SCM_{label}_SID_TYPE"))
            .map_err(|_| "missing or invalid configured SID type expectation")?
            .parse()
            .map_err(|_| "invalid configured SID type DWORD")?;
        let wide: Vec<u16> = name.iter().copied().map(u16::from).collect();
        let key: ServiceKey<'_> =
            ServiceKey::new(&wide).map_err(|_| "invalid fixed reference key")?;
        let service: ServiceConfigurationQuery = ServiceConfigurationQuery::open_local(key)?;
        let reported: Result<u32, ServiceQueryError> = service.query_service_sid_type();
        let closed: Result<(), ServiceQueryError> = service.close();
        let reported: u32 = reported?;
        closed?;
        if reported != expected {
            return Err(format!("configured service SID type differs: {label}").into());
        }
        println!("scm-configured-sid-type {label} matched=1");
    }
    Ok(())
}

/// Convert the independently supplied fixture values into expected literal units.
///
/// # Errors
/// Reject absent control inputs, malformed tags or an out-of-bound expectation.
fn expected_privilege_multistring(label: &str) -> Result<Option<Vec<u16>>, Box<dyn Error>> {
    let expected: String = std::env::var(format!("NEVROTICK_SCM_{label}_REQUIRED_PRIVILEGES"))
        .map_err(|_| "missing or invalid configured privileges expectation")?;
    if expected == "absent" {
        return Ok(None);
    }
    let names: &str = expected
        .strip_prefix("present:")
        .ok_or("invalid configured privileges expectation tag")?;
    // An x64 8 KiB SDK result has at most 4,092 wide units after its pointer.
    if names.encode_utf16().count() > 4_090 {
        return Err("configured privileges expectation exceeds the fixture bound".into());
    }
    let mut units: Vec<u16> = Vec::new();
    if !names.is_empty() {
        for name in names.split('|') {
            if name.is_empty() || name.contains('\0') {
                return Err("invalid required privilege name in fixture expectation".into());
            }
            units.extend(name.encode_utf16());
            units.push(0);
        }
    }
    units.push(0);
    Ok(Some(units))
}

/// Query the named object and require explicit retirement before returning its facts.
/// # Errors
/// Preserve the first open, query or closure error.
fn read_configuration(
    key: ServiceKey<'_>,
) -> Result<ReportedServiceConfiguration, ServiceQueryError> {
    let service: ServiceConfigurationQuery = ServiceConfigurationQuery::open_local(key)?;
    let reported: Result<ReportedServiceConfiguration, ServiceQueryError> = service.query();
    let closed: Result<(), ServiceQueryError> = service.close();
    let configuration: ReportedServiceConfiguration = reported?;
    closed?;
    Ok(configuration)
}

/// Observe native security through the exact owner/DACL-only adapter.
/// The supervisor checks the captured owner and discretionary ACL independently.
/// # Errors
/// Preserve native open, query, close and unexpected fixture outcomes.
#[test]
#[ignore = "requires the prepared offline guest and its independently read service descriptors"]
fn native_security_query_matches_scm() -> Result<(), Box<dyn Error>> {
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        let key: ServiceKey<'_> =
            ServiceKey::new(role.wide_name()).map_err(|_| "invalid fixed lab role key")?;
        match ServiceSecurityQuery::open_local(key) {
            Err(ServiceQueryError::OpenService(1060)) => (),
            Err(error) => return Err(error.into()),
            Ok(service) => {
                service.close()?;
                return Err("both lab services must be absent for this fixture".into());
            }
        }
    }

    let first: [u16; 15] = (*b"RemoteRegistry\0").map(u16::from);
    let second: [u16; 13] = (*b"SharedAccess\0").map(u16::from);
    let references: [(&str, &[u16]); 2] = [("RemoteRegistry", &first), ("SharedAccess", &second)];
    for (label, wide) in references {
        let key: ServiceKey<'_> =
            ServiceKey::new(wide).map_err(|_| "invalid fixed reference key")?;
        let service: ServiceSecurityQuery = ServiceSecurityQuery::open_local(key)?;
        let report: Result<ReportedServiceSecurity, ServiceQueryError> = service.query();
        let closed: Result<(), ServiceQueryError> = service.close();
        let descriptor: ReportedServiceSecurity = report?;
        closed?;
        let owner: &[u8] = descriptor.owner_sid();
        let ReportedDacl::Present(dacl) = descriptor.dacl() else {
            return Err("reference service must have a readable discretionary ACL".into());
        };
        if dacl.len() > 1024 {
            return Err("reference discretionary ACL exceeds fixture receipt limit".into());
        }
        println!("scm-security {label} owner={owner:02x?} dacl={dacl:02x?}");
    }
    println!("scm-security missing-roles=2");
    Ok(())
}
