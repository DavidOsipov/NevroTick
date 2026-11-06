"""Prepare the exact leap coverage prefix and explicitly limited read-only views.

This is a fixed-source structural projection, not a proof of full interpret or
public construction. It installs nothing and executes no compiler or solver.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SUBJECT = "leap_coverage"
SOURCE = "crates/time-domain/src/leap/interpretation.rs"
SOURCE_SHA256 = {
    SOURCE: "e11905a89ce4732005f0dfa9ef6ab80a03e0563452dda60b67ed6955ca634d12",
    "crates/time-domain/src/leap.rs": "977aa5762990fd7e3e0ed2bb536e3bff3ab89259b1af9f2370b2a068866e3e5b",
    "crates/time-domain/src/leap/validation.rs": "969c082cc83fe013bf6cb64319dc99eb1c6d288430b3dddb6b9580fe5a69939a",
    "crates/time-domain/src/era.rs": "366397df0c6bcd260d566b5bac310c882796734887af886a5fdf99a023bb1dc8",
    "crates/time-domain/src/coordinate.rs": "63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80",
    "crates/time-domain/src/lib.rs": "a6b697df2267e6e72f99df1bcb9b0798dada3af11fbbaa24160415ae73b0861b",
}
PREFIX_START, PREFIX_END = 813, 1668
PREFIX_SHA256 = "56be794b9a85660fc8b050afcd108def72d88dd40e3e41ff09f7dd41af627f80"
SUFFIX_SHA256 = "39acfb00bee74e71c2c2e1b8f06b8edbdcfed82c6100b3b7073f68bfb05ed257"
FUNCTION_SHA256 = "b27541439db922fc7935b71636043cc7ca90046bdd090e5a974360c93fc2f661"
FUNCTION_HEADER = b"""pub(super) fn interpret(
    knowledge: &LeapKnowledge<'_>,
    dates: EraWindow,
    scope: LeapDateScope,
) -> Result<LeapInterpretation, LeapError> {
"""
OBSERVATION_HEAD = b"""
/// Observe the original prefix only; views carry assumed existing invariants.
fn coverage_prefix(knowledge: &MetadataView, dates: WindowView, scope: ScopeView)
    -> Result<(bool, bool), LeapError>
{
"""
OBSERVATION_TAIL = b"    Ok((offsets_complete, events_complete))\n}\n"
CONTROLS = {
    "include_exclusive_end": {
        "before": "let offsets_complete = latest < known_before || bridged;",
        "after": "let offsets_complete = latest <= known_before || bridged;",
        "harness": "leap_coverage_control_exclusive_end",
        "intended_failure": "exclusive completeness endpoint remains unknown",
    },
    "exclude_continuous_start": {
        "before": "earliest >= continuous_start",
        "after": "earliest > continuous_start",
        "harness": "leap_coverage_control_continuous_start",
        "intended_failure": "continuous start is included for events",
    },
    "bridge_one_second_gap": {
        "before": "start <= known_before",
        "after": "start <= known_before + (1_i128 << 32)",
        "harness": "leap_coverage_control_gap",
        "intended_failure": "unbridged gap remains unknown",
    },
}
REQUIRED_ASSERTIONS = {
    "leap_coverage_full_domain": (
        "scope success iff inclusive containment", "outside scope error classification",
        "offset completeness excludes every unknown point",
        "event completeness excludes every unknown point",
        "incomplete offset has a concrete gap", "incomplete events have a concrete gap",
        "complete offset has no gap witness", "complete events have no gap witness",
        "complete offset implies complete events",
    ),
}
REQUIRED_COVERS = {
    "leap_coverage_full_domain": (
        "signed minimum outside scope is rejected", "signed maximum outside narrow scope is rejected",
        "singleton at baseline remains covered", "exclusive history cutoff is unknown",
        "last fractional tick before cutoff is covered", "finite expiry extends history",
        "continuous start after gap has event-only coverage", "continuous rule exactly bridges history",
        "continuous zero rule is retained", "finite zero expiry preserves history",
        "fractional candidate starts inside an unknown gap",
        "upper coordinate with continuous coverage is admitted", "full scalar metadata endpoints are included",
    ),
}
GETTERS = {
    "earliest": b"    pub const fn earliest(self) -> UtcCoordinateQ32 {\n        self.earliest\n    }\n",
    "latest": b"    pub const fn latest(self) -> UtcCoordinateQ32 {\n        self.latest\n    }\n",
    "ntp_seconds": b"    pub const fn ntp_seconds(self) -> u64 {\n        self.second.seconds\n    }\n",
}
SECOND_TICKS = b"fn second_ticks(seconds: u64) -> i128 {\n    i128::from(seconds) * (1_i128 << 32)\n}\n"
COORDINATE_TICKS = b"fn coordinate_ticks(coordinate: UtcCoordinateQ32) -> i128 {\n    coordinate.ticks()\n}\n"
VIEWS_HEAD = b"""use crate::coordinate::UtcCoordinateQ32;

// Proof views are values, not replacement validation or a product ABI model.
// Their factory assumes only the typed invariants listed in correspondence.json.
#[derive(Clone, Copy)]
struct WindowView { earliest: UtcCoordinateQ32, latest: UtcCoordinateQ32 }
impl WindowView {
"""
VIEWS_MIDDLE = b"""}
#[derive(Clone, Copy)]
struct SecondView { seconds: u64 }
#[derive(Clone, Copy)]
struct MetadataSecondView { second: SecondView }
impl MetadataSecondView {
"""
VIEWS_END = b"""}
struct MetadataView {
    history_before: MetadataSecondView,
    expiry: Option<MetadataSecondView>,
    continuous: Option<MetadataSecondView>,
}
struct ScopeView { dates: WindowView }

/// Proof-only view construction, not EraWindow/LeapKnowledge/Garde admission.
/// Preconditions: ordered signed-96-bit windows, scope >= baseline, history
/// above baseline, and each metadata second <= i64::MAX. No record claim follows.
pub(crate) fn observe(earliest: i128, latest: i128, scope_first: i128, scope_last: i128,
    history: u64, expiry: Option<u64>, continuous: Option<u64>)
    -> Result<(bool, bool), LeapError>
{
    let coordinate = |ticks| UtcCoordinateQ32::new(ticks).expect("admitted numeric coordinate");
    let scalar = |seconds| MetadataSecondView { second: SecondView { seconds } };
    let dates = WindowView { earliest: coordinate(earliest), latest: coordinate(latest) };
    let scope = ScopeView { dates: WindowView {
        earliest: coordinate(scope_first), latest: coordinate(scope_last) } };
    let knowledge = MetadataView { history_before: scalar(history),
        expiry: expiry.map(scalar), continuous: continuous.map(scalar) };
    coverage_prefix(&knowledge, dates, scope)
}
"""


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def input_names() -> list[str]:
    return sorted({*SOURCE_SHA256, "tools/formal/leap_coverage_projection.py",
                   "tools/formal/leap_coverage_harness.rs", "tools/formal/test_leap_coverage.py"})


def schedule() -> list[dict]:
    result = [{"subject": SUBJECT, "case": "healthy", "harness": "leap_coverage_full_domain",
               "intended_failure": None, "restored_healthy": False}]
    result.extend({"subject": SUBJECT, "case": case, "harness": control["harness"],
                   "intended_failure": control["intended_failure"], "restored_healthy": False}
                  for case, control in CONTROLS.items())
    result.append({"subject": SUBJECT, "case": "healthy", "harness": "leap_coverage_full_domain",
                   "intended_failure": None, "restored_healthy": True})
    return result


def reviewed_parts(source: bytes) -> tuple[bytes, bytes]:
    """Accept one complete source and one closed structural cut, never a parser."""
    if digest(source) != SOURCE_SHA256[SOURCE]:
        raise ValueError("Reviewed interpretation source changed")
    first = source.index(FUNCTION_HEADER)
    function = source[first:]
    prefix, suffix = source[PREFIX_START:PREFIX_END], source[PREFIX_END:]
    if (digest(function) != FUNCTION_SHA256 or digest(prefix) != PREFIX_SHA256
            or digest(suffix) != SUFFIX_SHA256 or FUNCTION_HEADER + prefix + suffix != function):
        raise ValueError("Reviewed coverage factor boundaries changed")
    return prefix, suffix


def prepare_leap_coverage(source_root: Path, inputs: Path, case: str, harness_path: Path) -> dict:
    """Seal a prefix observation with exact source calls and explicit view TCB.

    The source guard/max/Option/threshold statements are copied verbatim. Only
    their parameters' read-only carriers and the final tuple observation are
    proof scaffolding. Neither public validation nor the later scan is compiled.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown leap-coverage control")
    originals = {name: (source_root / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    prefix, suffix = reviewed_parts(originals[SOURCE])
    leap, era = originals["crates/time-domain/src/leap.rs"], originals["crates/time-domain/src/era.rs"]
    if (era.count(GETTERS["earliest"]) != 1 or era.count(GETTERS["latest"]) != 1
            or leap.count(GETTERS["ntp_seconds"]) != 3
            or leap.count(SECOND_TICKS) != 1 or leap.count(COORDINATE_TICKS) != 1
            or b"pub const LEAP_BASELINE_NTP_SECONDS: u64 = 2_272_060_800;" not in leap):
        raise ValueError("Reviewed read-only getter/conversion context changed")
    library = originals["crates/time-domain/src/lib.rs"]
    error_enum = library[library.index(b"/// Fixed redacted domain failures"):
                         library.index(b"impl std::fmt::Display for DomainError")].rstrip(b"\n") + b"\n"
    leap_error = leap[leap.index(b"/// Fixed redacted failures;"):
                      leap.index(b"impl fmt::Display for LeapError")].rstrip(b"\n") + b"\n"
    selected = prefix
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if selected.count(before) != 1:
            raise ValueError("Coverage mutation point is not unique")
        selected = selected.replace(before, after, 1)
    harness = harness_path.read_bytes()
    if not harness.strip():
        raise ValueError("Coverage harness is empty")
    views = (VIEWS_HEAD + GETTERS["earliest"] + GETTERS["latest"] + VIEWS_MIDDLE
             + GETTERS["ntp_seconds"] + VIEWS_END)
    compiled = (views + b"\n" + leap_error + b"\n" + SECOND_TICKS + b"\n" + COORDINATE_TICKS
                + OBSERVATION_HEAD + selected + OBSERVATION_TAIL)
    proof = (b"#![allow(dead_code)]\n" + error_enum
             + b'\nmod coordinate;\n#[path="coverage_selected.rs"]\nmod leap_coverage;\n'
             + harness.replace(b"//!", b"//"))
    destinations = [inputs / name for name in
                    ("proof.rs", "coordinate.rs", "coverage_selected.rs", "correspondence.json")]
    if inputs.is_symlink() or any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite leap-coverage inputs")
    details = {
        "entrypoint": inputs / "proof.rs", "subject": SUBJECT, "case": case,
        "original_sources_sha256": dict(SOURCE_SHA256), "harness_sha256": digest(harness),
        "original_function_sha256": FUNCTION_SHA256, "original_prefix_preserved": case == "healthy",
        "original_prefix_sha256": digest(prefix), "compiled_prefix_sha256": digest(selected),
        "unselected_suffix_sha256": digest(suffix), "compiled_module_sha256": digest(compiled),
        "mutation": mutation,
        "correspondence": [{"source": SOURCE, "lines_inclusive": [21, 37],
                            "byte_offsets_half_open": [PREFIX_START, PREFIX_END], "sha256": digest(prefix)},
                           {"source": "crates/time-domain/src/coordinate.rs", "selection": "complete actual module"},
                           {"source": "crates/time-domain/src/leap/validation.rs",
                            "selection": "unproved assumption owners: scope_rule baseline guard and valid_ledger historical endpoint guard",
                            "sha256": SOURCE_SHA256["crates/time-domain/src/leap/validation.rs"]},
                           {"source": "crates/time-domain/src/leap.rs", "selection": "actual LeapError, scalar getters, second_ticks and coordinate_ticks"}],
        "admitted_domain": ["all ordered signed-96-bit candidate and scope Q32 endpoints, including every fractional bit",
                            "scope earliest >= 2272060800 seconds; candidate may lie anywhere in its signed domain",
                            "history in [2272060801,i64::MAX]; optional expiry/continuous in [0,i64::MAX]",
                            "all Some(0), None and present/absent combinations; no continuous>=baseline assumption"],
        "assumptions": ["EraWindow numeric/order invariants for both windows",
                        "LeapDateScope baseline invariant and valid_ledger history>baseline invariant",
                        "metadata scalar <=i64::MAX invariant; public Garde constructors remain unproved"],
        "structural_binding": {
            "kind": "trusted closed-source prefix/view correspondence, not general Rust equivalence",
            "machine_whole_interpret_equivalence": False,
            "original_function_reconstructed": FUNCTION_HEADER + prefix + suffix == originals[SOURCE][originals[SOURCE].index(FUNCTION_HEADER):],
            "view_templates_sha256": digest(views), "observation_templates_sha256": digest(OBSERVATION_HEAD + OBSERVATION_TAIL),
            "getter_sha256": {name: digest(body) for name, body in GETTERS.items()},
            "conversion_sha256": {"second_ticks": digest(SECOND_TICKS), "coordinate_ticks": digest(COORDINATE_TICKS)},
            "reviewed_rule": "read-only scalar/coordinate getters and field views preserve prefix values; exact guard and immutable flags precede the omitted scan, whose pinned suffix only reads these flags",
            "trusted_components": ["review of the exact source cut and value-only carrier correspondence",
                                   "frozen projection/templates and ordinary source custody",
                                   "existing admitted compiler/verifier/stdlib and public host runtime"],
        },
        "omitted": ["Garde/public EraWindow, leap scalar, LeapDateScope and LeapKnowledge construction",
                    "ledger/date/calendar/identity admission and data provenance",
                    "record scan, offset values/variation, transition count and final interpretation result",
                    "machine-checked whole-function equivalence, product ABI/layout and native clock authority"],
        "scope": "coverage prefix flags and scope rejection only; no complete interpret or constructor proof",
        "fixed_jobs": schedule(),
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(originals["crates/time-domain/src/coordinate.rs"])
    destinations[2].write_bytes(compiled)
    destinations[3].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
