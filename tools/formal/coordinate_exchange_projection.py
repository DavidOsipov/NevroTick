"""Prepare complete actual-source coordinate and exchange proof modules.

No executable function, private type or runtime validation is projected away.
The crate wrapper selects the actual error enum and three complete modules;
the public garde window constructor and unrelated crate owners are not selected.
This module launches no compiler/solver and admits no tool or attempt budget.
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

# Each control changes one compiled source expression. All witnesses are valid
# constructor inputs except the intentionally invalid coordinate-rejection case.
CONTROLS = {
    "coordinate": {
        "always_error": {
            "module": "coordinate.rs",
            "before": "if !coordinate_in_range(ticks) {",
            "after": "if true {",
            "harness": "coordinate_control_zero",
            "intended_failure": "required coordinate success",
        },
        "wrong_coordinate": {
            "module": "coordinate.rs",
            "before": "Ok(Self { ticks })",
            "after": "Ok(Self { ticks: ticks + 1 })",
            "harness": "coordinate_control_retained",
            "intended_failure": "exact retained coordinate",
        },
        "wrong_error": {
            "module": "coordinate.rs",
            "before": "return Err(DomainError::CoordinateOutOfRange);",
            "after": "return Err(DomainError::ArithmeticOverflow);",
            "harness": "coordinate_control_rejection",
            "intended_failure": "coordinate rejection classification",
        },
    },
    "exchange": {
        "always_error": {
            "module": "exchange.rs",
            "before": "if !relative_in_range(offset_numerator) || !relative_in_range(round_trip_numerator) {",
            "after": "if true {",
            "harness": "exchange_control_zero",
            "intended_failure": "required exchange success",
        },
        "wrong_offset": {
            "module": "exchange.rs",
            "before": ".checked_add(transmit_delta)",
            "after": ".checked_sub(transmit_delta)",
            "harness": "exchange_control_phase",
            "intended_failure": "exact phase numerator",
        },
        "wrong_rtt": {
            "module": "exchange.rs",
            "before": ".checked_sub(peer_elapsed)",
            "after": ".checked_add(peer_elapsed)",
            "harness": "exchange_control_round_trip",
            "intended_failure": "exact signed round trip",
        },
    },
}


def digest(data: bytes) -> str:
    """Return identity of exact source bytes, without normalization."""
    return hashlib.sha256(data).hexdigest()


def prepare_coordinate_exchange(
    source_root: Path, inputs: Path, subject: str, case: str, harness_path: Path
) -> dict:
    """Prepare proof.rs and complete modules, rejecting drift before any write.

    ``subject`` is coordinate or exchange. A healthy case copies every module
    byte for byte; other cases apply exactly one named compiled control. Actual
    coordinate constructors produce all four exchange inputs. The surrounding
    runner must freeze inputs, admit tools and qualify actual solver results.
    """
    if subject not in CONTROLS:
        raise ValueError("Unknown coordinate/exchange proof subject")
    if case != "healthy" and case not in CONTROLS[subject]:
        raise ValueError("Unknown coordinate/exchange proof control")
    domain = source_root / "crates/time-domain/src"
    originals = {name: (domain / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError(f"Reviewed complete module changed: {name}")
    library = (domain / "lib.rs").read_bytes()
    start = b"/// Fixed redacted domain failures"
    end = b"impl std::fmt::Display for DomainError"
    if library.count(start) != 1 or library.count(end) != 1:
        raise ValueError("Actual DomainError extraction is ambiguous")
    error_enum = library[library.index(start):library.index(end)].rstrip(b"\n") + b"\n"
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError("Actual DomainError enum differs from reviewed source")
    harness = harness_path.read_bytes()
    if not harness.strip():
        raise ValueError("Coordinate/exchange proof harness is empty")

    compiled = dict(originals)
    mutation = None
    if case != "healthy":
        control = CONTROLS[subject][case]
        name = control["module"]
        before, after = control["before"].encode(), control["after"].encode()
        if compiled[name].count(before) != 1:
            raise ValueError("Reviewed control mutation point is not unique")
        compiled[name] = compiled[name].replace(before, after, 1)
        mutation = dict(control)

    proof = (
        b"#![allow(dead_code)]\n" + error_enum
        + b"\nmod coordinate;\nuse coordinate::UtcCoordinateQ32;\n"
        + b"mod rounding;\nuse rounding::{RoundedPhaseOffsetNs, RoundedRoundTripNs};\n"
        + b"mod exchange;\n"
        + harness.replace(b"//!", b"//")
    )
    correspondence = [
        {
            "source": f"crates/time-domain/src/{name}",
            "compiled_module": name,
            "selection": "complete byte-exact module before the named control",
            "original_sha256": digest(originals[name]),
            "compiled_sha256": digest(compiled[name]),
        }
        for name in originals
    ]
    details = {
        "entrypoint": inputs / "proof.rs",
        "subject": subject,
        "case": case,
        "correspondence": correspondence,
        "original_library_sha256": digest(library),
        "domain_error_enum_sha256": digest(error_enum),
        "proof_spec_sha256": digest(harness),
        "mutation": mutation,
        "assumptions": [],
        "omitted": [
            "DomainError Display/Error implementations",
            "unrelated crate modules and public garde window construction",
            "coordinate origin/time-scale truth and saved-real-T1 provenance",
            "association, authentication, sample admission and native effects",
        ],
        "scope": (
            "actual coordinate admission over raw i128"
            if subject == "coordinate"
            else "actual exchange arithmetic over four independent admitted signed-96-bit coordinates"
        ),
        "rounding_composition": (
            "exchange proves producer bounds and representable nanosecond scaling; "
            "the selected exchange harness does not call or reprove rounding"
        ),
    }
    inputs.mkdir(parents=True, exist_ok=True)
    for name, data in compiled.items():
        (inputs / name).write_bytes(data)
    (inputs / "proof.rs").write_bytes(proof)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n"
    )
    return details
