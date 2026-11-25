# SPDX-License-Identifier: GPL-2.0-only
"""Render the fixed S0.6 native owner entries from independently admitted pins."""

from collections.abc import Mapping
from dataclasses import dataclass, field
from hashlib import sha256
from pathlib import Path
from typing import Protocol
from uuid import UUID

from check_package import Refused, read_regular
from prestart_deployment_session import (
    OwnedDeploymentProcess, PrestartAccessLaunchAttempt, PrestartCaptures, PrestartOwnerAttempt, _remaining,
    launch_prestart_session,
)
from prestart_receipts import (
    DeploymentRun, Member, PrestartObservation, PrestartProbe, ReceiptRefused, SHA256, Run,
    is_prestart_sid_text, require, require_run,
)


_PIN_LIMITS: dict[str, int] = {
    "interpreter": 2 * 1024 * 1024,
    "child_module": 2 * 1024 * 1024,
    "session_module": 2 * 1024 * 1024,
    "deployment_helper": 2 * 1024 * 1024,
    "archive": 8 * 1024 * 1024,
    "identity_entry": 2 * 1024 * 1024,
    "identity_module": 2 * 1024 * 1024,
    "identity_helper": 64 * 1024,
    "observer": 16 * 1024 * 1024,
}
_ACCESS_PIN_LIMITS: dict[str, int] = {
    label: _PIN_LIMITS[label]
    for label in ("child_module", "session_module", "deployment_helper", "observer")
}
_TEMPLATE_LIMIT: int = 2 * 1024 * 1024
_BINDINGS: bytes = b"# PRESTART_DEPLOYMENT_BINDINGS\n"


@dataclass(frozen=True, slots=True)
class RenderedAccessOwner:
    """The fixed entry and the independently supplied inputs it renders."""

    run: DeploymentRun
    probe: PrestartProbe
    expected_actor_sid: str = field(repr=False)
    path: Path
    contents: bytes = field(repr=False)
    pin: Member


@dataclass(frozen=True, slots=True)
class DeploymentOwnerInputs:
    """Reviewed authoring inputs from the admitted deployment context."""

    template: Path
    template_pin: Member
    deployment_sid: str = field(repr=False)
    signer_sha256: str
    expected_payload_sha256: str
    pins: Mapping[str, Member]
    work_milliseconds: int
    retirement_milliseconds: int


@dataclass(frozen=True, slots=True)
class RenderedDeploymentOwner:
    """Full-mode entry and pin for one run with both original access owners."""

    run: Run
    contents: bytes = field(repr=False)
    pin: Member


class DeploymentGuestAttempt(Protocol):
    """One-shot activation for the admitted deployment actor and guest.

    Stage the rendered bytes at the fixed deployment entry, in protected
    staging distinct from both access roles. Pass the rendered pin to the
    original watchdog and return its private pipes. Independently enforce
    the original cutoffs across staging, launch and unattended waits. Keep
    the native job, deadlines, diagnostics and settlement custody across an
    interrupted activation.
    """

    def start(self, entry: RenderedDeploymentOwner, deadline: float,
              retirement_deadline: float) -> OwnedDeploymentProcess:
        """Activate the fixed entry and watchdog under the original host cutoffs."""
        ...

    def request_retirement(self, retirement_deadline: float) -> None:
        """Signal the original deployment attempt, even after a failed start."""
        ...

    def wait_retired(self, retirement_deadline: float) -> None:
        """Observe original native job and stream settlement or retain recovery custody."""
        ...


class _RenderedDeploymentAttempt(PrestartOwnerAttempt[Run]):
    """Render only after the coordinator has acquired both original access owners."""

    def __init__(self, inputs: DeploymentOwnerInputs, guest: DeploymentGuestAttempt):
        self._inputs: DeploymentOwnerInputs = inputs
        self._guest: DeploymentGuestAttempt = guest
        self._started: bool = False

    def start(self, run: Run, deadline: float, retirement_deadline: float) -> OwnedDeploymentProcess:
        inputs: DeploymentOwnerInputs = self._inputs
        contents: bytes
        pin: Member
        contents, pin = render_deployment_owner(
            inputs.template, inputs.template_pin, run, inputs.deployment_sid, inputs.signer_sha256,
            inputs.pins, inputs.work_milliseconds, inputs.retirement_milliseconds,
            expected_payload_sha256=inputs.expected_payload_sha256, full_prestart=True,
        )
        _remaining(deadline)
        self._started = True
        return self._guest.start(RenderedDeploymentOwner(run, contents, pin), deadline, retirement_deadline)

    def request_retirement(self, retirement_deadline: float) -> None:
        """Forward rollback only if staging or guest activation could have begun."""
        if self._started:
            return self._guest.request_retirement(retirement_deadline)
        return None

    def wait_retired(self, retirement_deadline: float) -> None:
        """Keep the original guest's independent settlement result."""
        if self._started:
            return self._guest.wait_retired(retirement_deadline)
        return None


def launch_rendered_prestart_session(
    run: DeploymentRun, provisioning: PrestartAccessLaunchAttempt, standard: PrestartAccessLaunchAttempt,
    deployment: DeploymentGuestAttempt, inputs: DeploymentOwnerInputs, deadline: float,
    retirement_deadline: float, *, captures: PrestartCaptures,
) -> PrestartObservation:
    """Join the fixed full-mode renderer to one admitted three-role launch.

    The lab caller supplies independently admitted actors, signed artifact and
    continuous quiescence. Neither staging the entry nor receiving a response
    establishes any of those facts. The coordinator owns ordering; on failure
    it signals attempted owners and requires their independent retirement under
    the original deadlines. The caller retains resources and raw captures.
    """
    require(deployment is not provisioning and deployment is not standard,
            "pre-start roles require distinct launch attempts")
    return launch_prestart_session(
        run, provisioning, standard, _RenderedDeploymentAttempt(inputs, deployment),
        deadline, retirement_deadline, captures=captures,
    )


def render_deployment_owner(
    template: Path,
    template_pin: Member,
    run: DeploymentRun,
    deployment_sid: str,
    signer_sha256: str,
    pins: Mapping[str, Member],
    work_milliseconds: int,
    retirement_milliseconds: int,
    *,
    expected_payload_sha256: str,
    full_prestart: bool = False,
) -> tuple[bytes, Member]:
    """Bind a protected staging entry to one run, fixed member roster and finite budgets.

    The retained owner supplies the reviewed template, the independently checked
    digest of the approved final authored payload and the other input pins.
    It protects staged paths through use and independently establishes actor
    eligibility, signed-package admission and continuous quiescence.
    This function produces bytes and their pin for the original watchdog.
    The Windows owner still checks and consumes the actual staged objects.
    """
    run_id: UUID = run.id
    require(type(run_id) is UUID and run_id.int != 0, "invalid deployment owner run")
    require(type(full_prestart) is bool, "invalid deployment owner mode")
    if full_prestart:
        require(isinstance(run, Run), "full pre-start requires both admitted access actors")
        require_run(run)
    require(is_prestart_sid_text(deployment_sid), "invalid deployment owner SID")
    require(
        type(signer_sha256) is str and SHA256.fullmatch(signer_sha256) is not None,
        "invalid deployment owner signer digest",
    )
    require(
        type(expected_payload_sha256) is str and SHA256.fullmatch(expected_payload_sha256) is not None,
        "invalid deployment owner payload digest",
    )
    require(
        type(work_milliseconds) is int and type(retirement_milliseconds) is int
        and 0 < work_milliseconds < retirement_milliseconds <= 120_000
        and retirement_milliseconds - work_milliseconds <= 30_000,
        "invalid deployment owner deadlines",
    )
    bound_pins: dict[str, Member] = _snapshot_pins(pins, _PIN_LIMITS)
    require(
        bound_pins["archive"].sha256 == run.signed_package_sha256
        and bound_pins["identity_module"].sha256 == run.companion_sha256
        and bound_pins["observer"].sha256 == run.observer_sha256,
        "deployment owner pins differ from the admitted run",
    )
    bindings: list[str] = [
        f"[guid]$expectedRunId = '{run_id}'",
        "[string]$ownerRole = 'deployment'",
        f"[string]$expectedActorSid = '{deployment_sid}'",
        f"[string]$signerSha256 = '{signer_sha256}'",
        f"[string]$expectedPayloadSha256 = '{expected_payload_sha256}'",
        f"[int]$workMilliseconds = {work_milliseconds}",
        f"[int]$retirementMilliseconds = {retirement_milliseconds}",
        "[bool]$fullPrestart = " + ("$true" if full_prestart else "$false"),
    ]
    return _render_owner(template, template_pin, bindings, bound_pins)


def render_prestart_access_owner(
    template: Path,
    template_pin: Member,
    run: DeploymentRun,
    probe: PrestartProbe,
    expected_actor_sid: str,
    pins: Mapping[str, Member],
    work_milliseconds: int,
    *,
    stage: Path,
    private_capture_diagnostics: bool = False,
) -> RenderedAccessOwner:
    """Bind one fixed access owner entry to an admitted actor, run and stage.

    The launcher supplies the original primary token and the protected absolute
    stage. This function renders the entry without creating or modifying the
    stage. An access entry accepts only its original run ID at startup.
    The native session and enclosing watchdog keep the original cutoff and
    independently observe child and process retirement.
    Capture diagnostics require the launcher to retain the original private
    regular stderr file for this role and run.
    """
    require(isinstance(run, DeploymentRun), "invalid pre-start access owner run")
    run_id: UUID = run.id
    require(type(run_id) is UUID and run_id.int != 0, "invalid pre-start access owner run")
    role: str
    if probe is PrestartProbe.ACCESS:
        role = "access"
    elif probe is PrestartProbe.STANDARD:
        role = "standard"
    else:
        raise ReceiptRefused("invalid pre-start access probe")
    require(is_prestart_sid_text(expected_actor_sid), "invalid pre-start access owner SID")
    require(type(work_milliseconds) is int and 0 < work_milliseconds <= 120_000,
            "invalid pre-start access owner deadline")
    require(type(private_capture_diagnostics) is bool, "invalid pre-start access capture diagnostics selection")
    bound_pins: dict[str, Member] = _snapshot_pins(pins, _ACCESS_PIN_LIMITS)
    require(bound_pins["observer"].sha256 == run.observer_sha256,
            "pre-start access observer pin differs from the admitted run")
    bindings: list[str] = [
        f"[guid]$expectedRunId = '{run_id}'",
        f"[string]$ownerRole = '{role}'",
        f"[string]$expectedActorSid = '{expected_actor_sid}'",
        f"[int]$workMilliseconds = {work_milliseconds}",
        "[bool]$privateCaptureDiagnostics = " + ("$true" if private_capture_diagnostics else "$false"),
    ]
    require(isinstance(stage, Path) and stage.is_absolute(), "invalid pre-start access staging directory")
    contents: bytes
    pin: Member
    contents, pin = _render_owner(template, template_pin, bindings, bound_pins)
    return RenderedAccessOwner(run, probe, expected_actor_sid, stage / "prestart_deployment_owner.ps1", contents, pin)


def _snapshot_pins(pins: Mapping[str, Member], limits: Mapping[str, int]) -> dict[str, Member]:
    """Read each independent pin once and return the checked roster in order."""
    bound_pins: dict[str, Member] = dict(pins)
    require(bound_pins.keys() == limits.keys(), "invalid deployment owner pin roster")
    for label, maximum in limits.items():
        _check_pin(bound_pins[label], maximum, label)
    return {label: bound_pins[label] for label in limits}


def _render_owner(
    template: Path, template_pin: Member, bindings: list[str], pins: Mapping[str, Member],
) -> tuple[bytes, Member]:
    """Render one closed binding block into the original verified template."""
    _check_pin(template_pin, _TEMPLATE_LIMIT, "template")
    require(
        template.name == "prestart_deployment_owner.ps1.in" and template.is_absolute(),
        "invalid deployment owner template path",
    )
    try:
        source: bytes = read_regular(template, template_pin.size, "deployment owner template")
    except (OSError, Refused) as error:
        raise ReceiptRefused("deployment owner template read refused") from error
    require(
        len(source) == template_pin.size and sha256(source).hexdigest() == template_pin.sha256,
        "deployment owner template differs",
    )
    lines: list[bytes] = source.splitlines(keepends=True)
    require(source.isascii() and lines.count(_BINDINGS) == 1, "invalid deployment owner binding marker")
    binding_lines: list[str] = [
        *bindings,
        "[hashtable]$pins = @{",
        *(f"    {label} = @{{ Bytes = [long]{pin.size}; Sha256 = '{pin.sha256}' }}"
          for label, pin in pins.items()),
        "}",
        "",
    ]
    block: bytes = "\n".join(binding_lines).encode("ascii")
    entry: bytes = b"".join(block if line == _BINDINGS else line for line in lines)
    require(len(entry) <= _TEMPLATE_LIMIT, "deployment owner entry size exceeds limit")
    return entry, Member(len(entry), sha256(entry).hexdigest())


def _check_pin(pin: Member, maximum: int, label: str) -> None:
    require(
        type(pin) is Member and type(pin.size) is int and 0 < pin.size <= maximum
        and type(pin.sha256) is str and SHA256.fullmatch(pin.sha256) is not None,
        "invalid deployment owner pin: " + label,
    )
