"""Prepare the actual private ScaledValue::split and its actual fraction factory.

Exact source spans retain the private types, ZERO, Fraction::new and error enums.
This module runs no compiler/solver and changes no application source. The full
raw-input oracle and semantic controls stay lexically in proof.rs.
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
    "scaled_value": {
        "source": "projection_enclosure.rs", "offsets": (11718, 12113),
        "sha256": "f6f5a446aea1b101939f8d861185a61d4d5ceb9795497e82a803b7a648aa93d8",
        "start": b"/// Exact sign and magnitude of one admitted operand multiplied by",
        "following": b"impl ScaledValue {",
    },
    "split": {
        "source": "projection_enclosure.rs", "offsets": (13289, 14550),
        "sha256": "b1bb449fc7055801f776adc2b048a54de9ee3b02922af78da7c44f0690d6ed9f",
        "start": b"    /// Split an IEEE significand and scale",
        "following": b"    /// Copy the exact signed integral component;",
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
    "reject_all": {
        "before": "if significand >= (1_u64 << 53) || !(-1041..=11).contains(&scale) {",
        "after": "if true || significand >= (1_u64 << 53) || !(-1041..=11).contains(&scale) {",
        "harness": "scaled_value_split::scaled_value_split_control_success",
        "intended_failure": "split required success",
    },
    "zero_remainder": {
        "before": "let numerator = significand & ((1_u64 << shift) - 1);",
        "after": "let numerator = 0_u64;",
        "harness": "scaled_value_split::scaled_value_split_control_remainder",
        "intended_failure": "split exact quotient remainder",
    },
    "wrong_error": {
        "before": "if significand >= (1_u64 << 53) || !(-1041..=11).contains(&scale) {\n"
                  "            return Err(FixtureEnclosureError::Invariant);\n        }",
        "after": "if significand >= (1_u64 << 53) || !(-1041..=11).contains(&scale) {\n"
                 "            return Err(FixtureEnclosureError::Unrepresentable);\n        }",
        "harness": "scaled_value_split::scaled_value_split_control_error",
        "intended_failure": "split rejection classification",
    },
}
FULL_HARNESS = "scaled_value_split::scaled_value_split_full_domain"
REQUIRED_ASSERTIONS = {
    FULL_HARNESS: (
        "split required success", "split rejection classification",
        "split exact nonnegative whole", "split nonnegative zero fraction",
        "split exact quotient remainder", "split strict remainder bound",
        "split large denominator whole zero", "split large denominator numerator retained",
        "split zero canonicalization", "split nonzero shift retained",
        "split output numerator bound", "split output shift bound", "split output proper dyadic",
    ),
}
REQUIRED_COVERS = {
    FULL_HARNESS: (
        "split zero at scale zero", "split zero at minimum scale", "split zero at negative sixty four",
        "split integral unit", "split maximum raw whole", "split exact negative scale",
        "split fractional negative scale", "split below source cutoff", "split at source cutoff",
        "split below oracle cutoff", "split at oracle cutoff", "split maximum fraction shift",
        "split oversized proper significand rejected", "split maximum raw significand rejected",
        "split zero beyond upper scale rejected", "split nonzero beyond upper scale rejected",
        "split zero beyond lower scale rejected", "split nonzero beyond lower scale rejected",
        "split minimum raw scale rejected", "split maximum raw scale rejected",
    ),
}


def digest(data: bytes) -> str:
    """Identify executable and correspondence bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Five fixed jobs: full raw domain, three distinct faults, exact restoration."""
    jobs = [{"subject": "scaled_value_split", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "scaled_value_split", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "scaled_value_split", "case": "healthy", "harness": FULL_HARNESS,
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def _extract(originals: dict[str, bytes], name: str) -> bytes:
    """Require unique reviewed context, fixed offsets and byte-exact source spans."""
    span = SPANS[name]
    source = originals[span["source"]]
    first, last = span["offsets"]
    if source.count(span["start"]) != 1 or source.count(span["following"]) != 1:
        raise ValueError("Actual split source extraction is ambiguous: " + name)
    selected = source[first:last]
    if (source.index(span["start"]) != first
            or source.index(span["following"]) < last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed split source span changed: " + name)
    return selected


def prepare_scaled_value_split(source_root: Path, inputs: Path, case: str,
                               harness_path: Path) -> dict:
    """Copy actual split/callees, mutating only one reviewed split-body site.

    Reject source/collision drift before creating outputs. No valid-input
    assumption substitutes for the full raw-input domain in the separate harness.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown scaled-value-split control")
    core = source_root / "crates/time-core/src"
    originals = {name: (core / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    split = selected["split"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if split.count(before) != 1:
            raise ValueError("Reviewed split mutation point is not unique")
        split = split.replace(before, after, 1)
    harness = harness_path.read_bytes()
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Byte-exact selected owners; correspondence.json records original locations.\n"
                   + selected["interval_error"] + b"\n" + selected["enclosure_error"] + b"\n"
                   + selected["fraction"] + b"\nimpl Fraction {\n" + selected["zero"]
                   + b"\n" + selected["new"] + b"}\n" + selected["scaled_value"]
                   + b"\nimpl ScaledValue {\n" + split + b"}\n")
    proof = (b"#![allow(dead_code)]\nmod scaled_value_split {\n"
             b'include!("scaled_value_split_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "scaled_value_split_selected.rs", "correspondence.json")]
    if inputs.is_symlink() or (inputs.exists() and not inputs.is_dir()):
        raise ValueError("Split input directory redirects or is not a directory")
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing scaled-value-split inputs")
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in SPANS.items()],
        "original_helper_preserved": case == "healthy", "compiled_helper_sha256": digest(split),
        "original_struct_zero_constructor_and_enums_preserved": True,
        "harness_inside_private_module": True,
        "harness_sha256": digest(harness), "harness_location": "proof.rs",
        "compiled_definitions_sha256": digest(definitions), "entrypoint_sha256": digest(proof),
        "mutation": mutation, "assumptions": [],
        "omitted": ["ScaledValue::from_bits and signed_whole",
                    "Fraction::compare, Fraction::complement and fraction addition",
                    "EndpointParts construction/rounding and complete enclosure composition",
                    "Garde/raw-operand admission and checked interval construction",
                    "error formatting and error-trait implementations",
                    "chart roundoff, selection and complete Core/P1",
                    "production acquisition, native Windows, clock and release authority"],
        "scope": "full raw u64/i64 ScaledValue::split admission, exact decomposition, fraction invariants and errors only",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(definitions)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
