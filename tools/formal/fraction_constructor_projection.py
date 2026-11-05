"""Prepare the actual private Fraction::new without compiling or proving it.

Byte-exact definitions include Fraction, its ZERO constant, the constructor and
the actual error enums. Harness assertions remain lexically in proof.rs inside
the same private module. Product visibility and source remain unchanged.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE_SHA256 = {
    "projection_enclosure.rs": "6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023",
    "selection.rs": "da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e",
}
SPANS = {
    "fraction": {
        "source": "projection_enclosure.rs", "offsets": (8107, 8415),
        "sha256": "4742bce4685790e469ecdf5720cf569d71d9ed47c6a23ec808c4b39b14ff99a4",
        "start": b"/// Canonical proper nonnegative dyadic fraction",
        "following": b"impl Fraction {",
    },
    "zero": {
        "source": "projection_enclosure.rs", "offsets": (8432, 8595),
        "sha256": "9fcc4427cb030845d0a527fbd2cc00b3037eba2e44d01009c96aed86f74fe6d4",
        "start": b"    /// Canonical exact zero;",
        "following": b"    /// Check a transformed numerator/denominator",
    },
    "new": {
        "source": "projection_enclosure.rs", "offsets": (8596, 9327),
        "sha256": "4bdb19b66cf6e80a173cd05ba5553242c45bf1280600c24e8da49e25d7aaa6f2",
        "start": b"    /// Check a transformed numerator/denominator",
        "following": b"    /// Compare two checked fractions exactly",
    },
    "enclosure_error": {
        "source": "projection_enclosure.rs", "offsets": (1608, 2231),
        "sha256": "628d29b282da93e9bd20b15fb6b7af99a82e0985ac8330705bb6518dcffea901",
        "start": b"/// Bounded enclosure failure",
        "following": b"impl fmt::Debug for FixtureEnclosureError",
    },
    "interval_error": {
        "source": "selection.rs", "offsets": (1796, 2163),
        "sha256": "b4afc7fa071599702078052cab5205d145e01ee1c0ea91a1cfccaef540130399",
        "start": b"/// Bounded, payload-free interval-construction",
        "following": b"/// Bounded, payload-free fixture-minimum",
    },
}
CONTROLS = {
    "always_error": {
        "before": "if shift > 1041 || numerator >= (1_u64 << 53) {",
        "after": "if true || shift > 1041 || numerator >= (1_u64 << 53) {",
        "harness": "fraction_constructor::fraction_constructor_control_success",
        "intended_failure": "fraction required success",
    },
    "wrong_numerator": {
        "before": "Ok(Self { numerator, shift })",
        "after": "Ok(Self { numerator: numerator.saturating_sub(1), shift })",
        "harness": "fraction_constructor::fraction_constructor_control_numerator",
        "intended_failure": "fraction exact numerator retained",
    },
    "wrong_error": {
        "before": "if shift > 1041 || numerator >= (1_u64 << 53) {\n"
                  "            return Err(FixtureEnclosureError::Invariant);\n        }",
        "after": "if shift > 1041 || numerator >= (1_u64 << 53) {\n"
                 "            return Err(FixtureEnclosureError::Unrepresentable);\n        }",
        "harness": "fraction_constructor::fraction_constructor_control_error",
        "intended_failure": "fraction rejection classification",
    },
}
FULL_HARNESS = "fraction_constructor::fraction_constructor_full_domain"
REQUIRED_ASSERTIONS = {
    FULL_HARNESS: (
        "fraction required success", "fraction rejection classification",
        "fraction exact numerator retained", "fraction zero canonicalization",
        "fraction nonzero shift retained", "fraction output numerator bound",
        "fraction output shift bound", "fraction output proper dyadic",
    ),
}
REQUIRED_COVERS = {
    FULL_HARNESS: (
        "fraction zero at shift zero is canonical", "fraction zero at maximum shift is canonical",
        "fraction one half succeeds", "fraction maximum numerator at shift 53 succeeds",
        "fraction unit numerator at maximum shift succeeds",
        "fraction zero beyond maximum shift is rejected", "fraction zero at raw maximum shift is rejected",
        "fraction oversized proper numerator is rejected", "fraction raw maximum numerator is rejected",
        "fraction nonzero numerator at zero shift is rejected", "fraction unit boundary is rejected",
        "fraction proper neighbor succeeds", "fraction maximum numerator at shift 63 succeeds",
        "fraction maximum numerator at shift 64 succeeds",
    ),
}


def digest(data: bytes) -> str:
    """Identify exact source bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Five fixed jobs: healthy, three distinct faults, then identical restoration."""
    jobs = [{"subject": "fraction_constructor", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "fraction_constructor", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "fraction_constructor", "case": "healthy", "harness": FULL_HARNESS,
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def _extract(originals: dict[str, bytes], name: str) -> bytes:
    """Require reviewed unique context, exact offsets and unchanged span bytes."""
    span = SPANS[name]
    source = originals[span["source"]]
    first, last = span["offsets"]
    if source.count(span["start"]) != 1 or source.count(span["following"]) != 1:
        raise ValueError("Actual fraction source extraction is ambiguous: " + name)
    selected = source[first:last]
    if (source.index(span["start"]) != first
            or source.index(span["following"]) < last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed fraction source span changed: " + name)
    return selected


def prepare_fraction_constructor(source_root: Path, inputs: Path, case: str,
                                 harness_path: Path) -> dict:
    """Copy exact definitions and at most one reviewed constructor-only mutation.

    All source/collision checks precede output creation. This prepares a private
    constructor proof; it asserts no completion of callers or other operations.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown fraction-constructor control")
    core = source_root / "crates/time-core/src"
    originals = {name: (core / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    constructor = selected["new"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if constructor.count(before) != 1:
            raise ValueError("Reviewed fraction mutation point is not unique")
        constructor = constructor.replace(before, after, 1)
    harness = harness_path.read_bytes()
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Byte-exact selected owners; correspondence.json records original locations.\n"
                   + selected["interval_error"] + b"\n" + selected["enclosure_error"] + b"\n"
                   + selected["fraction"] + b"\nimpl Fraction {\n" + selected["zero"]
                   + b"\n" + constructor + b"}\n")
    proof = (b"#![allow(dead_code)]\nmod fraction_constructor {\n"
             b'include!("fraction_constructor_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "fraction_constructor_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing fraction-constructor inputs")
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in SPANS.items()],
        "original_helper_preserved": case == "healthy", "compiled_helper_sha256": digest(constructor),
        "original_struct_zero_and_enums_preserved": True, "harness_inside_private_module": True,
        "harness_sha256": digest(harness), "harness_location": "proof.rs",
        "compiled_definitions_sha256": digest(definitions), "entrypoint_sha256": digest(proof),
        "mutation": mutation, "assumptions": [],
        "omitted": ["Fraction::compare and Fraction::complement",
                    "fraction addition and binary64 decomposition",
                    "EndpointParts::new and complete enclosure composition",
                    "Garde/raw-operand admission and checked interval construction",
                    "error formatting and error-trait implementations",
                    "chart roundoff, selection and complete Core/P1",
                    "production acquisition, native Windows, clock and release authority"],
        "scope": "full raw u64/u32 Fraction::new admission, exact retention, canonical zero and errors only",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(definitions)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
