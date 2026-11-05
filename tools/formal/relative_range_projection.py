"""Prepare complete actual modules plus a private-predicate visibility wrapper.

For healthy inputs the original exchange module is a byte-exact prefix. The
only appended function calls its actual private relative_in_range predicate;
no existing executable statement or private aggregate construction is replaced.
This module performs no compiler, solver, tool admission or budget operation.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE_SHA256 = {
    "coordinate.rs": "63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80",
    "exchange.rs": "d7208286fb97303a64b719aa880ee07ec2411fef8c17abbda9acfb5e2b33e7b6",
    "rounding.rs": "387fa926a079aa7b87477c97dea8b79049595ac8a793ead449b19abb8b0b5519",
}
ERROR_ENUM_SHA256 = "a63f855634262baf2586e21312378f935a52e9e0ea02e77805ffd1083c01fd35"
PREDICATE = "(-MAX_RELATIVE_NUMERATOR..=MAX_RELATIVE_NUMERATOR).contains(&value)"
WRAPPER = b"""
/// Proof-only visibility wrapper; no existing body or private field is changed.
pub(crate) fn proof_relative_in_range(value: i128) -> bool {
    relative_in_range(value)
}
"""
CONTROLS = {
    "reject_all": {
        "before": PREDICATE,
        "after": "false",
        "harness": "relative_control_success",
        "intended_failure": "required relative success",
    },
    "accept_all": {
        "before": PREDICATE,
        "after": "true",
        "harness": "relative_control_upper",
        "intended_failure": "upper relative rejection",
    },
    "widen_lower_edge": {
        "before": PREDICATE,
        "after": "(-(MAX_RELATIVE_NUMERATOR + 1)..=MAX_RELATIVE_NUMERATOR).contains(&value)",
        "harness": "relative_control_lower",
        "intended_failure": "lower relative rejection",
    },
}
REQUIRED_ASSERTIONS = {
    "relative_full_domain": (
        "lower relative rejection", "upper relative rejection", "required relative success",
    ),
}
REQUIRED_COVERS = {
    "relative_full_domain": (
        "relative zero is admitted", "lower relative endpoint is admitted",
        "upper relative endpoint is admitted", "lower relative neighbor is rejected",
        "upper relative neighbor is rejected", "relative raw minimum is rejected",
        "relative raw maximum is rejected",
    ),
}


def digest(data: bytes) -> str:
    """Identify exact bytes without source or newline normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Five commands: healthy, three compiled defects, restored healthy.

    This metadata starts no command and requires a separately declared bounded
    checkpoint; it extends no currently frozen or completed schedule.
    """
    result = [{"subject": "relative_range", "case": "healthy",
               "harness": "relative_full_domain", "intended_failure": None,
               "restored_healthy": False}]
    for case, control in CONTROLS.items():
        result.append({"subject": "relative_range", "case": case,
                       "harness": control["harness"],
                       "intended_failure": control["intended_failure"],
                       "restored_healthy": False})
    result.append({"subject": "relative_range", "case": "healthy",
                   "harness": "relative_full_domain", "intended_failure": None,
                   "restored_healthy": True})
    return result


def prepare_relative_range(source_root: Path, inputs: Path, case: str,
                           harness_path: Path) -> dict:
    """Copy all actual modules, append the wrapper, reject drift before writes.

    The complete actual DomainError enum supplies the module graph's error type.
    Healthy exchange bytes remain an unchanged prefix; controls change exactly
    the named original predicate expression before the same wrapper is appended.
    Only this raw predicate is selected for proof, not other compiled functions.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown relative-range control")
    domain = source_root / "crates/time-domain/src"
    originals = {name: (domain / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError(f"Reviewed complete module changed: {name}")
    library = (domain / "lib.rs").read_bytes()
    first = b"/// Fixed redacted domain failures"
    last = b"impl std::fmt::Display for DomainError"
    if library.count(first) != 1 or library.count(last) != 1:
        raise ValueError("Actual DomainError extraction is ambiguous")
    error_enum = library[library.index(first):library.index(last)].rstrip(b"\n") + b"\n"
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError("Actual DomainError enum differs from reviewed source")
    harness = harness_path.read_bytes()
    if not harness.strip():
        raise ValueError("Relative-range proof harness is empty")

    prefix = originals["exchange.rs"]
    mutation = None
    if case != "healthy":
        control = CONTROLS[case]
        before, after = control["before"].encode(), control["after"].encode()
        if prefix.count(before) != 1:
            raise ValueError("Reviewed relative mutation point is not unique")
        prefix = prefix.replace(before, after, 1)
        mutation = dict(control)
    compiled = {**originals, "exchange.rs": prefix + WRAPPER}
    proof = (
        b"#![allow(dead_code)]\n" + error_enum
        + b"\nmod coordinate;\nuse coordinate::UtcCoordinateQ32;\n"
        + b"mod rounding;\nuse rounding::{RoundedPhaseOffsetNs, RoundedRoundTripNs};\n"
        + b"mod exchange;\n" + harness.replace(b"//!", b"//")
    )
    correspondence = [
        {
            "source": f"crates/time-domain/src/{name}",
            "compiled_module": name,
            "selection": (
                "complete original module prefix before named control, then proof-only private-predicate wrapper"
                if name == "exchange.rs" else "complete byte-exact module"
            ),
            "original_sha256": digest(originals[name]),
            "compiled_sha256": digest(compiled[name]),
        }
        for name in originals
    ]
    details = {
        "entrypoint": inputs / "proof.rs",
        "case": case,
        "correspondence": correspondence,
        "original_library_sha256": digest(library),
        "domain_error_enum_sha256": digest(error_enum),
        "proof_spec_sha256": digest(harness),
        "exchange_original_prefix_bytes": len(originals["exchange.rs"]),
        "exchange_compiled_prefix_bytes": len(prefix),
        "exchange_compiled_prefix_sha256": digest(prefix),
        "exchange_original_prefix_preserved": prefix == originals["exchange.rs"],
        "appended_wrapper_sha256": digest(WRAPPER),
        "mutation": mutation,
        "assumptions": [],
        "omitted": [
            "other compiled coordinate/exchange/rounding functions are not selected for this theorem",
            "DomainError Display/Error implementations and unrelated crate modules",
            "coordinate origin, association, authentication, sample admission and native effects",
        ],
        "scope": "actual private relative_in_range(i128) over full raw i128; no private construction",
    }
    targets = [inputs / name for name in (*compiled, "proof.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in targets):
        raise ValueError("Refusing to overwrite existing relative-range proof inputs")
    inputs.mkdir(parents=True, exist_ok=True)
    for name, data in compiled.items():
        (inputs / name).write_bytes(data)
    (inputs / "proof.rs").write_bytes(proof)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n"
    )
    return details
