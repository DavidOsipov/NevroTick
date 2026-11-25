# SPDX-License-Identifier: GPL-2.0-only
"""Correlate the fixed S0.6 pre-start observations for an admitted supervisor.

This module performs no I/O and grants no authority. Its caller must already own
one supervised, untouched installation window, a verified signed artifact and
independent effective-token and child-process observations. Only captured child
output crosses the byte parser. There is intentionally no file/CLI loader for
trusted inputs.
"""

from dataclasses import dataclass, field
import json
import re
from collections.abc import Callable
from enum import Enum
from uuid import UUID

from check_package import MAX_FILE, SERVICES, unique_object


OBSERVATION = "scm_observation_tests::native_installed_roles_prestart_observation"
ACCESS = "scm_access_probe::windows::native_installed_roles_prestart_access"
STANDARD = "scm_access_probe::windows::native_standard_user_prestart_change_config_refused"
RIGHTS = ("0x4", "0x1", "0x20000", "0x2", "0x10", "0x20")
PACKAGE_NAME = re.compile(r"NevroTick\.S06\.ScmFixture_1\.0\.0\.0_x64__[0-9a-z]{13}")
SHA256 = re.compile(r"[0-9a-f]{64}")
_SID: re.Pattern[str] = re.compile(
    r"S-1-(?:0|[1-9][0-9]{0,9}|0x[0-9A-Fa-f]{12})(?:-(?:0|[1-9][0-9]{0,9})){1,15}"
)
MAX_OUTPUT = 8192
DWORD = r"(?:0|[1-9][0-9]{0,9})"
NONZERO_DWORD = r"[1-9][0-9]{0,9}"
# Both native x64 configuration queries use 8192-byte buffers. Reported string
# extents exclude the terminator. The privilege-list extent includes it.
MAX_CONFIGURATION_STRING_UNITS = (8192 - 64) // 2 - 1
MAX_PRIVILEGE_LIST_UNITS = (8192 - 8) // 2
ROLE_ROW = re.compile(
    rf"scm-prestart (?P<role>\S+) configured-type=(?P<kind>{DWORD}) start=4 error=(?P<error>{DWORD}) "
    rf"before-type=(?P<before>{DWORD}) before-state=1 after-type=(?P<after>{DWORD}) after-state=1 "
    rf"sid-type=(?P<sid_type>{DWORD}) path-units=(?:None|Some\((?P<path>{DWORD})\)) "
    rf"account-units=(?:None|Some\((?P<account>{DWORD})\)) "
    rf"privilege-units=(?:None|Some\((?P<privileges>{DWORD})\)) security-control=(?P<security>{DWORD}) "
    rf"owner-bytes=(?P<owner>{NONZERO_DWORD}) dacl=(?P<dacl>absent|null|present) dacl-bytes=(?P<acl_bytes>{DWORD})"
)


class PrestartProbe(Enum):
    """Closed selection for one identity and three fixed SCM observations."""

    IDENTITY = "installed-identity"
    OBSERVATION = OBSERVATION
    ACCESS = ACCESS
    STANDARD = STANDARD


class ReceiptRefused(ValueError):
    """A missing, malformed, or inconsistent observation in the supervised run."""


@dataclass(frozen=True)
class Member:
    """Expected length and digest read from the independently verified package."""

    size: int
    sha256: str


@dataclass(frozen=True)
class DeploymentRun:
    """Independent package and deployment facts supplied by the retained owner.

    The owner verifies the signed package and the exact observer executable and
    module inputs. The actor reference identifies its independently admitted
    effective-token context. The retained owner supplies these facts independently
    of child output.
    """

    id: UUID
    package_full_name: str
    signed_package_sha256: str
    engine: Member
    broker: Member
    companion_sha256: str
    observer_sha256: str
    deployment_actor: object


@dataclass(frozen=True)
class Run(DeploymentRun):
    """Full pre-start inputs including the independently admitted access actors."""

    provisioning_actor: object
    standard_actor: object


@dataclass(frozen=True)
class Capture:
    """One supervisor-owned, bounded capture from its retained process object.

    The actor, PID, before/after input hashes, exit and reap observations come
    from that owner, outside the captured child streams. The input is the exact
    loaded companion module for the identity call and the exact executable for
    each selector. The supervisor separately pins the companion interpreter.
    """

    run_id: UUID
    actor: object = field(repr=False)
    pid: int
    input_sha256_before: str
    input_sha256_after: str
    exit_code: int
    reaped: bool
    stdout: bytes = field(repr=False)
    stderr: bytes = field(repr=False)


@dataclass(frozen=True, slots=True)
class ScmTypeSample:
    """Original raw (configuration, status) DWORDs from the retained SCM handles.

    The live window supplies these ordered fields independently of child output.
    They bind a point observation to the window's initial sample.
    """

    engine: tuple[int, int]
    broker: tuple[int, int]


@dataclass(frozen=True)
class DeploymentObservation:
    """Consistent identity and raw SCM samples for the fixed Engine/Broker order.

    Each triple is configured type, before type, and after type. This result
    records checked observations. The native owner separately establishes
    currentness and retirement. Exact descriptors, continuous inactivity,
    and supported provisioning require separate evidence.
    """

    run_id: UUID
    signed_package_sha256: str
    service_types: tuple[tuple[int, int, int], ...]


@dataclass(frozen=True)
class PrestartObservation(DeploymentObservation):
    """Deployment samples correlated with both fixed access observations."""


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ReceiptRefused(reason)


def is_dword(value: object) -> bool:
    return type(value) is int and 0 <= value <= 0xFFFFFFFF


def is_prestart_sid_text(value: object) -> bool:
    """Recognize bounded SID spelling for fixed Windows arguments."""
    return type(value) is str and len(value) <= 183 and _SID.fullmatch(value) is not None


def require_deployment_run(run: DeploymentRun) -> None:
    """Reject incomplete or out-of-profile deployment inputs before capture."""
    require(type(run.id) is UUID and run.id.int != 0, "invalid supervisor run")
    require(type(run.package_full_name) is str and PACKAGE_NAME.fullmatch(run.package_full_name) is not None,
            "wrong S0.6 package identity")
    for digest in (run.signed_package_sha256, run.companion_sha256, run.observer_sha256,
                   run.engine.sha256, run.broker.sha256):
        require(type(digest) is str and SHA256.fullmatch(digest) is not None, "invalid supervisor digest")
    for member in (run.engine, run.broker):
        require(type(member.size) is int and 0 < member.size <= MAX_FILE, "invalid member extent")
    require(run.deployment_actor is not None, "actor contexts are incomplete or conflated")


def require_run(run: Run) -> None:
    """Require independent deployment and access contexts for the full run."""
    require_deployment_run(run)
    require(isinstance(run, Run) and run.provisioning_actor is not None and run.standard_actor is not None
            and run.provisioning_actor is not run.standard_actor
            and run.deployment_actor is not run.provisioning_actor
            and run.deployment_actor is not run.standard_actor, "actor contexts are incomplete or conflated")


def read_capture(capture: Capture, run: DeploymentRun, actor: object, expected_input: str) -> str:
    """Require original run, actor and input custody before decoding a stream."""
    require(capture.run_id == run.id and capture.actor is actor, "foreign run or actor capture")
    require(is_dword(capture.pid) and capture.pid > 0, "missing observed process ID")
    require(capture.input_sha256_before == expected_input == capture.input_sha256_after,
            "captured child input differs from supervisor input")
    require(type(capture.exit_code) is int and capture.exit_code == 0 and capture.reaped is True,
            "child did not exit successfully and reap")
    require(type(capture.stdout) is bytes and 0 < len(capture.stdout) <= MAX_OUTPUT
            and type(capture.stderr) is bytes and capture.stderr == b"", "unexpected child streams")
    try:
        return capture.stdout.decode("utf-8").replace("\r\n", "\n")
    except UnicodeError:
        pass
    raise ReceiptRefused("malformed child output")


def _identity_report(output: str) -> object:
    """Decode the private identity output without retaining it in a parser exception."""
    try:
        return json.loads(output, object_pairs_hook=unique_object)
    except (ValueError, RecursionError):
        pass
    raise ReceiptRefused("malformed installed identity report")


def read_installed_identity(capture: Capture, run: DeploymentRun) -> None:
    """Compare the companion's closed record with independent signed bytes."""
    output: str = read_capture(capture, run, run.deployment_actor, run.companion_sha256)
    report: object = _identity_report(output)
    expected = {
        "schema_version": 2,
        "outcome": "observed-installed-images",
        "run_id": str(run.id),
        "signed_package_sha256": run.signed_package_sha256,
        "package_full_name": run.package_full_name,
        "roles": [
            {"key": role, "member": name, "bytes": member.size, "sha256": member.sha256}
            for (name, role), member in zip(SERVICES.items(), (run.engine, run.broker), strict=True)
        ],
    }
    require(type(report) is dict and type(report.get("schema_version")) is int and report == expected,
            "installed identity differs from the original run or package")
    require(all(type(role["bytes"]) is int for role in report["roles"]), "invalid installed member extent")


def read_test(output: str, selector: str, row_count: int) -> list[str]:
    """Read one exact successful libtest invocation, retaining its own rows."""
    lines = output.split("\n")
    prefix = f"test {selector} ... "
    require(len(lines) == row_count + 7 and lines[:2] == ["", "running 1 test"]
            and lines[2].startswith(prefix) and lines[-5:-3] == ["ok", ""] and lines[-2:] == ["", ""]
            and re.fullmatch(rf"test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured; "
                             rf"{DWORD} filtered out; finished in [0-9]{{1,5}}\.[0-9]{{2}}s", lines[-3]),
            "unexpected selector, transcript extent, or libtest completion")
    return [lines[2][len(prefix):], *lines[3:-5]]


def read_role_rows(lines: list[str]) -> tuple[tuple[int, int, int], ...]:
    """Check both Disabled/Stopped observations and retain their raw type bits."""
    require(len(lines) == 3 and lines[-1] == "scm-prestart observed-rows=2", "incomplete two-role observation")
    types = []
    for role, line in zip(SERVICES.values(), lines[:2], strict=True):
        row = ROLE_ROW.fullmatch(line)
        if row is None or row["role"] != role:
            raise ReceiptRefused("wrong role or non-Disabled/non-Stopped observation")
        path, account, privileges = (
            int(row[name]) if row[name] is not None else None
            for name in ("path", "account", "privileges")
        )
        # An empty MULTI_SZ contains one zero. A nonempty one has a name and two zeros.
        require(all(extent is None or extent <= MAX_CONFIGURATION_STRING_UNITS for extent in (path, account))
                and (privileges is None or privileges == 1 or 3 <= privileges <= MAX_PRIVILEGE_LIST_UNITS),
                "invalid SCM configuration extent")
        control, owner, acl = (int(row[name]) for name in ("security", "owner", "acl_bytes"))
        # Check the facts this projection retains from service_security.rs.
        # The actual owner SID and DACL entries still require protected readback.
        require(all(is_dword(int(row[name])) for name in ("kind", "before", "after", "error", "sid_type"))
                and control <= 0xFFFF and control & 0x8000
                and bool(control & 4) == (row["dacl"] != "absent")
                and 8 <= owner <= 68 and owner % 4 == 0
                and (acl >= 8 and acl % 4 == 0 if row["dacl"] == "present" else acl == 0)
                and 20 + owner + acl <= 8192,
                "invalid SCM observation representation")
        types.append((int(row["kind"]), int(row["before"]), int(row["after"])))
    return tuple(types)


def _opening_service_types(scm_types: ScmTypeSample) -> tuple[tuple[int, int, int], ...]:
    """Require the independently retained opening sample before either capture."""
    require(type(scm_types) is ScmTypeSample, "invalid original SCM type sample")
    for pair in (scm_types.engine, scm_types.broker):
        require(type(pair) is tuple and len(pair) == 2 and all(is_dword(value) for value in pair),
                "invalid original SCM type sample")
    return tuple(
        (configuration, status, status) for configuration, status in (scm_types.engine, scm_types.broker)
    )


def read_deployment(run: DeploymentRun, collect: Callable[[PrestartProbe], Capture],
                    scm_types: ScmTypeSample) -> DeploymentObservation:
    """Check identity before requesting observation from the same retained window.

    The caller supplies the opening SCM sample and process custody from its
    native owner. This result establishes the deployment receipt checks.
    """
    expected_types: tuple[tuple[int, int, int], ...] = _opening_service_types(scm_types)
    require_deployment_run(run)
    _, service_types = _read_deployment(run, collect, expected_types)
    return DeploymentObservation(run.id, run.signed_package_sha256, service_types)


def read_prestart(run: Run, collect: Callable[[PrestartProbe], Capture],
                  scm_types: ScmTypeSample) -> PrestartObservation:
    """Validate the live window's original SCM types before collecting access evidence."""
    return _read_prestart(run, collect, _opening_service_types(scm_types))


def _read_deployment(
    run: DeploymentRun, collect: Callable[[PrestartProbe], Capture],
    expected_types: tuple[tuple[int, int, int], ...] | None,
) -> tuple[list[str], tuple[tuple[int, int, int], ...]]:
    """Check the two common phases and retain the original observer rows."""
    read_installed_identity(collect(PrestartProbe.IDENTITY), run)
    observation = collect(PrestartProbe.OBSERVATION)
    observation_output = read_capture(observation, run, run.deployment_actor, run.observer_sha256)
    observation_rows = read_test(observation_output, OBSERVATION, 3)
    service_types: tuple[tuple[int, int, int], ...] = read_role_rows(observation_rows)
    if expected_types is not None:
        require(service_types == expected_types, "SCM types differ from the original window")
    return observation_rows, service_types


def _read_prestart(run: Run, collect: Callable[[PrestartProbe], Capture],
                   expected_types: tuple[tuple[int, int, int], ...] | None) -> PrestartObservation:
    """Validate each phase before asking the supervisor for another capture."""
    require_run(run)
    observation_rows, service_types = _read_deployment(run, collect, expected_types)
    access = collect(PrestartProbe.ACCESS)
    access_output = read_capture(access, run, run.provisioning_actor, run.observer_sha256)
    access_rows = read_test(access_output, ACCESS, 17)
    expected_access = [
        *observation_rows,
        f"scm-prestart-access invoking-pid={access.pid}",
        *(f"scm-prestart-access {role} mask={mask} opened-and-closed"
          for role in SERVICES.values() for mask in RIGHTS),
        "scm-prestart-access observed-rows=12",
    ]
    require(access_rows == expected_access, "provisioning access rows differ from the original observation")
    standard = collect(PrestartProbe.STANDARD)
    standard_output = read_capture(standard, run, run.standard_actor, run.observer_sha256)
    standard_rows = read_test(standard_output, STANDARD, 4)
    expected_standard = [
        f"scm-prestart-standard-user invoking-pid={standard.pid}",
        *(f"scm-prestart-standard-user {role} status-opened-and-closed configuration-denied code=5"
          for role in SERVICES.values()),
        "scm-prestart-standard-user observed-rows=2",
    ]
    require(standard_rows == expected_standard, "standard-user access or denial rows differ")
    return PrestartObservation(run.id, run.signed_package_sha256, service_types)

def join_prestart(run: Run, identity: Capture, observation: Capture, access: Capture,
                  standard: Capture) -> PrestartObservation:
    """Compare retained transcripts with each other in the absence of a live SCM sample."""
    captures = {
        PrestartProbe.IDENTITY: identity,
        PrestartProbe.OBSERVATION: observation,
        PrestartProbe.ACCESS: access,
        PrestartProbe.STANDARD: standard,
    }
    return _read_prestart(run, captures.__getitem__, None)
