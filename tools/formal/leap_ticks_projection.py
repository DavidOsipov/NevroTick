"""Prepare a source-bound leap scalar bridge without Garde stubs or execution.

The selected helper is byte-exact, and the complete actual coordinate module
and DomainError enum compile alongside it. No private product field is assembled.
This is preparation only; it starts no compiler, solver, admission or budget.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE_SHA256 = {
    "leap.rs": "977aa5762990fd7e3e0ed2bb536e3bff3ab89259b1af9f2370b2a068866e3e5b",
    "coordinate.rs": "63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80",
    "lib.rs": "a6b697df2267e6e72f99df1bcb9b0798dada3af11fbbaa24160415ae73b0861b",
}
HELPER_START, HELPER_END = 19_736, 19_818
HELPER_SHA256 = "894596c74443f715cd4baacd8e50b77ad07be626cb395693ad54b71e78771ff2"
ERROR_ENUM_SHA256 = "a63f855634262baf2586e21312378f935a52e9e0ea02e77805ffd1083c01fd35"
EXPRESSION = "i128::from(seconds) * (1_i128 << 32)"
WRAPPER = b"""
/// Proof-only visibility; the selected original helper remains unchanged.
pub(crate) fn proof_second_ticks(seconds: u64) -> i128 {
    second_ticks(seconds)
}
"""
CONTROLS = {
    "return_zero": {
        "before": EXPRESSION,
        "after": "i128::from(seconds.wrapping_sub(seconds))",
        "harness": "leap_ticks_control_nonzero",
        "intended_failure": "nonzero second retained",
    },
    "half_scale": {
        "before": EXPRESSION,
        "after": "i128::from(seconds) * (1_i128 << 31)",
        "harness": "leap_ticks_control_scale",
        "intended_failure": "whole-second fractional alignment",
    },
    "truncate_seconds": {
        "before": EXPRESSION,
        "after": "i128::from(seconds as u32) * (1_i128 << 32)",
        "harness": "leap_ticks_control_high_bits",
        "intended_failure": "high second bits retained",
    },
}
REQUIRED_ASSERTIONS = {
    "leap_ticks_full_domain": (
        "whole seconds remain nonnegative", "full u64 Q32 fits signed i128",
        "exact unsigned Q32 encoding", "zero fractional bits", "full second bits retained",
        "coordinate admission iff scalar envelope", "exact coordinate ticks retained",
        "outside coordinate error classification",
    ),
}
REQUIRED_COVERS = {
    "leap_ticks_full_domain": (
        "zero whole second is covered", "one whole second retains Q32 units",
        "leap baseline scalar is covered", "post-u32 second bits are covered",
        "signed-i64 whole-second endpoint is covered",
        "first outside whole-second neighbor is covered", "raw u64 maximum remains represented",
    ),
}


def digest(data: bytes) -> str:
    """Identify exact source bytes without whitespace normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Five commands, requiring a new bounded checkpoint before execution."""
    jobs = [{"subject": "leap_ticks", "case": "healthy", "harness": "leap_ticks_full_domain",
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "leap_ticks", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "leap_ticks", "case": "healthy", "harness": "leap_ticks_full_domain",
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def prepare_leap_ticks(source_root: Path, inputs: Path, case: str,
                       harness_path: Path) -> dict:
    """Reject source drift, then copy the selected body and complete dependencies.

    Healthy code copies the exact private conversion. Each control changes one
    documented expression in that copy and appends the identical visibility
    wrapper. Constructor and caller validation outside this bridge remain omitted.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown leap-ticks control")
    domain = source_root / "crates/time-domain/src"
    originals = {name: (domain / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    helper = originals["leap.rs"][HELPER_START:HELPER_END]
    if digest(helper) != HELPER_SHA256:
        raise ValueError("Reviewed second_ticks source span changed")
    library = originals["lib.rs"]
    first, last = b"/// Fixed redacted domain failures", b"impl std::fmt::Display for DomainError"
    if library.count(first) != 1 or library.count(last) != 1:
        raise ValueError("Actual DomainError extraction is ambiguous")
    error_enum = library[library.index(first):library.index(last)].rstrip(b"\n") + b"\n"
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError("Actual DomainError enum differs from reviewed bytes")
    selected = helper
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if selected.count(before) != 1:
            raise ValueError("Reviewed second_ticks mutation point is not unique")
        selected = selected.replace(before, after, 1)
    proof = (b"#![allow(dead_code)]\n" + error_enum
             + b'\nmod coordinate;\n#[path="leap_ticks_selected.rs"]\nmod leap_ticks;\n'
             + harness_path.read_bytes().replace(b"//!", b"//"))
    destinations = [inputs / name for name in
                    ("proof.rs", "coordinate.rs", "leap_ticks_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing leap-ticks inputs")
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"source": "crates/time-domain/src/leap.rs",
                            "byte_offsets_half_open": [HELPER_START, HELPER_END],
                            "sha256": digest(helper)}],
        "complete_coordinate_module_preserved": True, "domain_error_enum_sha256": digest(error_enum),
        "original_helper_preserved": case == "healthy", "compiled_helper_sha256": digest(selected),
        "appended_wrapper_sha256": digest(WRAPPER), "mutation": mutation, "assumptions": [],
        "omitted": ["Garde and LeapSecond/metadata constructors", "valid_ledger and record admission",
                    "LeapDateScope and public EraWindow construction", "whole-interval interpretation",
                    "baseline date provenance and clock authority"],
        "scope": "full raw-u64 second_ticks and actual coordinate admission/retention/error bridge only",
    }
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(originals["coordinate.rs"])
    destinations[2].write_bytes(selected + b"\n" + WRAPPER)
    destinations[3].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
