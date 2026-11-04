"""Prepare exact private month-start source; perform no compiler/solver work.

This is preparation, not an admitted runner or completed mathematical proof.
The coordinator must include these files in its frozen-input inventory before
executing the declared schedule with the existing full admission and limits.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE = "crates/time-domain/src/leap/validation.rs"
SOURCE_SHA256 = "969c082cc83fe013bf6cb64319dc99eb1c6d288430b3dddb6b9580fe5a69939a"
SPAN_FIRST, SPAN_LAST = 83, 98
SPAN_SHA256 = "284fc2fba9cdecc029fc512ece3de31c5949a73d5597bbb5518f809dd631fba2"

MUTATIONS = {
    "reject_month_start": (
        "if !seconds.is_multiple_of(86_400) {",
        "if seconds == 2_274_739_200 || !seconds.is_multiple_of(86_400) {",
    ),
    "omit_alignment": (
        "if !seconds.is_multiple_of(86_400) {",
        "if seconds == u64::MAX {",
    ),
    "omit_century_adjustment": (" + day_in_cycle / 36_524", ""),
}

REQUIRED_ASSERTIONS = {
    "leap_calendar_full_domain": ("Gregorian month-start equivalence",),
    "leap_calendar_oracle_laws": (
        "Gregorian year recurrence", "Gregorian month recurrence",
        "January begins the year", "400-year period length",
        "arbitrary-cycle forward correspondence", "1900 January NTP epoch anchor",
        "NTP epoch cycle position",
    ),
}
REQUIRED_COVERS = {
    "leap_calendar_full_domain": (
        "NTP epoch is a month start", "sub-day value is rejected",
        "1972 leap day is rejected", "2100 March starts after an ordinary February",
        "month start above signed-i64 is covered", "raw u64 maximum is covered",
    ),
    "leap_calendar_oracle_laws": (
        "ordinary leap year is covered", "non-400 century is covered",
        "400-year leap century is covered", "arbitrary cycle-count maximum is covered",
    ),
}


def schedule() -> list[dict]:
    """Seven commands: two healthy, three distinct defects, both restored healthy.

    This schedule needs its own declared checkpoint budget; it is not added to
    the earlier 24-attempt rounding/era checkpoint and starts no process itself.
    """
    result = []

    def add(case, harness, intended=None, restored=False):
        result.append({"subject": "leap_calendar", "case": case, "harness": harness,
                       "intended_failure": intended, "restored_healthy": restored})

    add("healthy", "leap_calendar_full_domain")
    add("healthy", "leap_calendar_oracle_laws")
    add("reject_month_start", "leap_calendar_control_success", "required month-start success")
    add("omit_alignment", "leap_calendar_control_alignment", "sub-day rejection")
    add("omit_century_adjustment", "leap_calendar_control_century", "century month-start boundary")
    add("healthy", "leap_calendar_full_domain", restored=True)
    add("healthy", "leap_calendar_oracle_laws", restored=True)
    return result


def digest(data: bytes) -> str:
    """Return exact byte identity without source normalization."""
    return hashlib.sha256(data).hexdigest()


def prepare_leap_calendar(source_root: Path, inputs: Path, case: str,
                          harness_path: Path) -> dict:
    """Require the reviewed full file/span and emit inputs/proof.rs once.

    Only the actual loop-free helper is projected. The wrapper exposes it solely
    inside the proof input; no product API or executable statement is replaced
    for healthy jobs. A control changes one documented expression in that copy.
    """
    if case != "healthy" and case not in MUTATIONS:
        raise ValueError("Unknown leap-calendar control")
    original = (source_root / SOURCE).read_bytes()
    if digest(original) != SOURCE_SHA256:
        raise ValueError("Leap validation source differs from reviewed correspondence")
    lines = original.splitlines(keepends=True)
    selected = b"".join(lines[SPAN_FIRST - 1:SPAN_LAST])
    if digest(selected) != SPAN_SHA256:
        raise ValueError("Reviewed leap-calendar span no longer matches")
    subject = selected.decode()
    mutation = None
    if case != "healthy":
        before, after = MUTATIONS[case]
        if subject.count(before) != 1:
            raise ValueError("Reviewed leap-calendar mutation point is not unique")
        subject = subject.replace(before, after, 1)
        mutation = {"before": before, "after": after}
    subject += """
/// Proof-only visibility wrapper; the original private body stays unchanged.
pub(crate) fn proof_is_month_start(seconds: u64) -> bool {
    is_month_start(seconds)
}
"""
    harness = harness_path.read_bytes()
    proof = ('#![allow(dead_code)]\n#[path="calendar_selected.rs"]\nmod calendar;\n'
             + harness.decode().replace("//!", "//"))
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations = [inputs / name for name in
                    ("calendar_selected.rs", "proof.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing leap-calendar proof inputs")
    details = {
        "entrypoint": inputs / "proof.rs",
        "original_validation_sha256": digest(original),
        "harness_sha256": digest(harness),
        "correspondence": [{"source": SOURCE,
                            "lines_inclusive": [SPAN_FIRST, SPAN_LAST],
                            "sha256": digest(selected)}],
        "mutation": mutation,
        "assumptions": [],
        "omitted": ["scope_rule", "ledger_rule", "valid_ledger",
                    "private checked metadata and Garde constructors"],
        "scope": "actual private month-start helper on full u64; independent Gregorian oracle; no ledger/Garde proof",
    }
    destinations[0].write_text(subject)
    destinations[1].write_text(proof)
    destinations[2].write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
