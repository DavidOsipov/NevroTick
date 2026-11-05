"""Prepare exact private fraction arithmetic source and full-domain harnesses.

Nine byte-exact spans include real constructors and all selected arithmetic
callees. This prepares proof inputs; it starts no compiler, solver or admission.
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
    "compare": {
        "source": "projection_enclosure.rs", "offsets": (9328, 10927),
        "sha256": "d62a2adcd41d2861ad806b70f1a63195ac0393e116d1dbfc1d358307790ecb8a",
        "start": b"    /// Compare two checked fractions exactly",
        "following": b"    /// Form the exact complement",
    },
    "complement": {
        "source": "projection_enclosure.rs", "offsets": (10928, 11715),
        "sha256": "5c5adb30e9966de9e3ac61dfcd6de492e302fcf91ed76e9346541e1b72c5c4e9",
        "start": b"    /// Form the exact complement",
        "following": b"}\n\n/// Exact sign and magnitude",
    },
    "fraction_sum": {
        "source": "projection_enclosure.rs", "offsets": (16069, 16358),
        "sha256": "507cd2a0b0b0447bbbf859003ee3274423c14dfbcbbf5b901029e2485316762a",
        "start": b"/// Integral carry and residual-presence",
        "following": b"/// Classify a magnitude-fraction sum",
    },
    "addition": {
        "source": "projection_enclosure.rs", "offsets": (16359, 17495),
        "sha256": "ecdec0942af79afdecbb5ab207f921d40888c5905b4eb83f65ff965a4651f859",
        "start": b"/// Classify a magnitude-fraction sum",
        "following": b"/// Exact signed integer plus",
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
    "fraction_complement": {
        "always_error": {
            "before": "fn complement(self) -> Result<Self, FixtureEnclosureError> {",
            "after": "fn complement(self) -> Result<Self, FixtureEnclosureError> {\n"
                     "        return Err(FixtureEnclosureError::Invariant);",
            "harness": "fraction_arithmetic::fraction_complement_control_success",
            "intended_failure": "complement required success",
        },
        "wrong_numerator": {
            "before": "Self::new(numerator, self.shift)",
            "after": "Self::new(1, self.shift)",
            "harness": "fraction_arithmetic::fraction_complement_control_numerator",
            "intended_failure": "complement exact numerator relation",
        },
        "wrong_error": {
            "before": "return Err(FixtureEnclosureError::Invariant);",
            "after": "return Err(FixtureEnclosureError::Unrepresentable);",
            "harness": "fraction_arithmetic::fraction_complement_control_error",
            "intended_failure": "complement rejection classification",
        },
    },
    "fraction_addition": {
        "always_error": {
            "before": "fn add_fractions(left: Fraction, right: Fraction) -> Result<FractionSum, FixtureEnclosureError> {",
            "after": "fn add_fractions(left: Fraction, right: Fraction) -> Result<FractionSum, FixtureEnclosureError> {\n"
                     "    return Err(FixtureEnclosureError::Invariant);",
            "harness": "fraction_arithmetic::fraction_addition_control_success",
            "intended_failure": "addition required success",
        },
        "wrong_carry": {
            "before": "carry: comparison != Ordering::Less,",
            "after": "carry: comparison == Ordering::Greater,",
            "harness": "fraction_arithmetic::fraction_addition_control_carry",
            "intended_failure": "addition exact carry",
        },
        "wrong_residual": {
            "before": "residual: comparison != Ordering::Equal,",
            "after": "residual: comparison == Ordering::Greater,",
            "harness": "fraction_arithmetic::fraction_addition_control_residual",
            "intended_failure": "addition exact residual",
        },
    },
}
REQUIRED_ASSERTIONS = {
    "fraction_arithmetic::fraction_complement_full_domain": (
        "complement factory completeness", "complement required success",
        "complement rejection classification", "complement exact numerator relation",
        "complement shift retained", "complement positive output bound", "complement proper output",
    ),
    "fraction_arithmetic::fraction_addition_full_domain": (
        "addition left factory completeness", "addition right factory completeness",
        "addition required success", "addition exact carry", "addition exact residual",
        "addition oracle alignment exact", "addition oracle proper numerator bounds",
        "addition oracle sum representable", "addition high denominator positive",
        "addition global fraction maximum", "addition high numerator bound",
    ),
}
REQUIRED_COVERS = {
    "fraction_arithmetic::fraction_complement_full_domain": (
        "complement zero rejected", "complement quarter rejected",
        "complement below half neighbor rejected", "complement wide below half rejected",
        "complement minimum positive rejected", "complement maximum shift numerator rejected",
        "complement half succeeds", "complement unreduced half succeeds",
        "complement maximum half representation succeeds", "complement above half neighbor succeeds",
        "complement near one succeeds", "complement three quarters succeeds", "complement five eighths succeeds",
    ),
    "fraction_arithmetic::fraction_addition_full_domain": (
        "addition zero sum", "addition quarter plus zero", "addition zero plus quarter",
        "addition two quarters", "addition exact halves", "addition quarter plus three quarters",
        "addition three quarters plus quarter", "addition three quarters plus half",
        "addition half plus three quarters", "addition unreduced halves", "addition just below unit",
        "addition just above unit", "addition maximum numerator sum", "addition u128 edge left",
        "addition u128 edge right", "addition high denominator left", "addition high denominator right",
        "addition minimum positive plus zero", "addition zero plus minimum positive",
        "addition two minimum positives", "addition raw zeros canonicalize",
        "addition half plus quarter", "addition quarter plus half",
    ),
}


def digest(data: bytes) -> str:
    """Identify exact bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Ten sequential jobs: complete complement, then complete addition."""
    jobs = []
    for subject, controls in CONTROLS.items():
        harness = "fraction_arithmetic::" + subject + "_full_domain"
        jobs.append({"subject": subject, "case": "healthy", "harness": harness,
                     "intended_failure": None, "restored_healthy": False})
        for case, control in controls.items():
            jobs.append({"subject": subject, "case": case, "harness": control["harness"],
                         "intended_failure": control["intended_failure"], "restored_healthy": False})
        jobs.append({"subject": subject, "case": "healthy", "harness": harness,
                     "intended_failure": None, "restored_healthy": True})
    return jobs


def _extract(originals: dict[str, bytes], name: str) -> bytes:
    """Reject any ambiguity, shifted context or changed reviewed span bytes."""
    span = SPANS[name]
    source = originals[span["source"]]
    first, last = span["offsets"]
    if source.count(span["start"]) != 1 or source.count(span["following"]) != 1:
        raise ValueError("Actual arithmetic source extraction is ambiguous: " + name)
    selected = source[first:last]
    if (source.index(span["start"]) != first
            or source.index(span["following"]) < last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed arithmetic source span changed: " + name)
    return selected


def prepare_fraction_arithmetic(source_root: Path, inputs: Path, subject: str,
                                case: str, harness_path: Path) -> dict:
    """Copy exact owners, mutating only the selected arithmetic method if requested.

    Constructors, comparison and the other arithmetic method stay byte-exact.
    Source checks and output-collision checks finish before directory creation.
    """
    if subject not in CONTROLS:
        raise ValueError("Unknown fraction-arithmetic subject")
    if case != "healthy" and case not in CONTROLS[subject]:
        raise ValueError("Unknown fraction-arithmetic control")
    core = source_root / "crates/time-core/src"
    originals = {name: (core / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    owner = "complement" if subject == "fraction_complement" else "addition"
    compiled = dict(selected)
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[subject][case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if compiled[owner].count(before) != 1:
            raise ValueError("Reviewed arithmetic mutation point is not unique")
        compiled[owner] = compiled[owner].replace(before, after, 1)
    harness = harness_path.read_bytes()
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Byte-exact selected owners; correspondence.json records original locations.\n"
                   b"use std::cmp::Ordering;\n\n"
                   + compiled["interval_error"] + b"\n" + compiled["enclosure_error"] + b"\n"
                   + compiled["fraction"] + b"\nimpl Fraction {\n" + compiled["zero"]
                   + b"\n" + compiled["new"] + b"\n" + compiled["compare"]
                   + b"\n" + compiled["complement"] + b"}\n\n" + compiled["fraction_sum"]
                   + b"\n" + compiled["addition"])
    proof = (b"#![allow(dead_code)]\nmod fraction_arithmetic {\n"
             b'include!("fraction_arithmetic_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "fraction_arithmetic_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing fraction-arithmetic inputs")
    details = {
        "entrypoint": inputs / "proof.rs", "subject": subject,
        "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in SPANS.items()],
        "selected_arithmetic_owner": owner, "original_helper_preserved": case == "healthy",
        "compiled_helper_sha256": digest(compiled[owner]), "original_callees_and_other_arithmetic_preserved": True,
        "harness_inside_private_module": True, "harness_location": "proof.rs",
        "harness_sha256": digest(harness), "compiled_definitions_sha256": digest(definitions),
        "entrypoint_sha256": digest(proof), "mutation": mutation, "assumptions": [],
        "trusted_high_denominator_specification": "canonical M>=128 means a positive term <2^-75; every checked fraction <=1-2^-53, so sum is strictly between zero and one; typed supporting bounds are checked without a callee oracle",
        "omitted": ["binary64 decomposition and Garde/raw-operand admission",
                    "EndpointParts::new and complete enclosure composition",
                    "error formatting and error-trait implementations",
                    "chart roundoff, selection and complete Core/P1",
                    "production acquisition, native Windows, clock and release authority"],
        "scope": "all checked fractions/pairs through complete raw factories; exact complement and addition carry/residual only",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(definitions)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
