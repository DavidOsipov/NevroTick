"""Prepare exact private Fraction construction/comparison, without execution.

The actual type, ZERO, new, compare and error enums retain their reviewed bytes.
Harness assertions stay lexically in proof.rs inside their private module.
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
        "before": "fn compare(self, other: Self) -> Result<Ordering, FixtureEnclosureError> {",
        "after": "fn compare(self, other: Self) -> Result<Ordering, FixtureEnclosureError> {\n"
                 "        return Err(FixtureEnclosureError::Invariant);",
        "harness": "fraction_comparison::fraction_comparison_control_success",
        "intended_failure": "comparison required success",
    },
    "reverse_unequal": {
        "before": "unequal => Ok(unequal),",
        "after": "unequal => Ok(unequal.reverse()),",
        "harness": "fraction_comparison::fraction_comparison_control_unequal",
        "intended_failure": "comparison strict unequal order",
    },
    "zero_alignment": {
        "before": ".checked_shl(other.shift - self.shift)",
        "after": ".checked_shl(0)",
        "harness": "fraction_comparison::fraction_comparison_control_alignment",
        "intended_failure": "comparison unreduced equality",
    },
}
FULL_HARNESS = "fraction_comparison::fraction_comparison_full_domain"
REQUIRED_ASSERTIONS = {
    FULL_HARNESS: (
        "comparison left factory completeness", "comparison right factory completeness",
        "comparison required success", "comparison exact rational ordering",
        "comparison oracle gap bound", "comparison oracle scaled width",
        "comparison positive numerator bit bounds", "comparison leading exponent bounds",
        "comparison equal power alignment relation", "comparison alignment count bound",
        "comparison alignment retains numerator bits", "comparison unequal powers strict order",
    ),
}
REQUIRED_COVERS = {
    FULL_HARNESS: (
        "comparison zero equality", "comparison zero below minimum positive",
        "comparison minimum positive above zero", "comparison identical one half",
        "comparison quarter below half", "comparison half above quarter",
        "comparison same shift less", "comparison same shift greater",
        "comparison unreduced equality left", "comparison unreduced equality right",
        "comparison maximum alignment left", "comparison maximum alignment right",
        "comparison maximum alignment less", "comparison maximum alignment greater",
        "comparison gap63 greater", "comparison gap63 less",
        "comparison gap64 greater", "comparison gap64 less",
        "comparison maximum numerator wide gap", "comparison maximum shift gap",
    ),
}


def digest(data: bytes) -> str:
    """Identify exact source bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Five fixed jobs: healthy, three distinct faults, then identical restoration."""
    jobs = [{"subject": "fraction_comparison", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "fraction_comparison", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "fraction_comparison", "case": "healthy", "harness": FULL_HARNESS,
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


def prepare_fraction_comparison(source_root: Path, inputs: Path, case: str,
                                 harness_path: Path) -> dict:
    """Copy exact definitions and at most one reviewed comparison-only mutation.

    All source/collision checks precede output creation. This prepares a private
    comparison proof; it asserts no completion of callers or other operations.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown fraction-comparison control")
    core = source_root / "crates/time-core/src"
    originals = {name: (core / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    comparison = selected["compare"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if comparison.count(before) != 1:
            raise ValueError("Reviewed fraction mutation point is not unique")
        comparison = comparison.replace(before, after, 1)
    harness = harness_path.read_bytes()
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Byte-exact selected owners; correspondence.json records original locations.\n"
                   b"use std::cmp::Ordering;\n\n"
                   + selected["interval_error"] + b"\n" + selected["enclosure_error"] + b"\n"
                   + selected["fraction"] + b"\nimpl Fraction {\n" + selected["zero"]
                   + b"\n" + selected["new"] + b"\n" + comparison + b"}\n")
    proof = (b"#![allow(dead_code)]\nmod fraction_comparison {\n"
             b'include!("fraction_comparison_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "fraction_comparison_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing fraction-comparison inputs")
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in SPANS.items()],
        "original_helper_preserved": case == "healthy", "compiled_helper_sha256": digest(comparison),
        "original_struct_zero_constructor_and_enums_preserved": True, "harness_inside_private_module": True,
        "harness_sha256": digest(harness), "harness_location": "proof.rs",
        "compiled_definitions_sha256": digest(definitions), "entrypoint_sha256": digest(proof),
        "mutation": mutation, "assumptions": [],
        "machine_lemma_correspondence": "source-equivalent bit lengths/leading exponents and branch alignment formulas; actual checked_shl equals bounded u128 alignment",
        "omitted": ["Fraction::complement",
                    "fraction addition and binary64 decomposition",
                    "EndpointParts::new and complete enclosure composition",
                    "Garde/raw-operand admission and checked interval construction",
                    "error formatting and error-trait implementations",
                    "chart roundoff, selection and complete Core/P1",
                    "production acquisition, native Windows, clock and release authority"],
        "scope": "all checked Fraction pairs through complete raw factories; exact rational comparison, required success and source machine bounds",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(definitions)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
