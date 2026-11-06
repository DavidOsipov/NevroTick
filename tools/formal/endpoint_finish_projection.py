"""Prepare the byte-exact private endpoint completion and its full-domain harness.

This copies only the reviewed struct, selected method and actual error/rounding
enums. Proof-only assembly and harnesses stay inside a private projected module;
the product source, visibility, constructors and validation paths are unchanged.
Preparation starts no compiler, solver, admission or budget.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE_SHA256 = {
    "projection_enclosure.rs": "6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023",
    "selection.rs": "da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e",
}
# Every selected span includes its original documentation, attributes and final
# newline. Fixed offsets, unique contextual markers and hashes must all agree.
SPANS = {
    "rounding": {
        "source": "projection_enclosure.rs", "offsets": (1268, 1607),
        "sha256": "e26870116ae163556941b721cda8f34f9707b76051a92705e8c2b2fd178fbf69",
        "start": b"/// Whether one exact represented endpoint",
        "following": b"/// Bounded enclosure failure",
    },
    "enclosure_error": {
        "source": "projection_enclosure.rs", "offsets": (1608, 2231),
        "sha256": "628d29b282da93e9bd20b15fb6b7af99a82e0985ac8330705bb6518dcffea901",
        "start": b"/// Bounded enclosure failure",
        "following": b"impl fmt::Debug for FixtureEnclosureError",
    },
    "parts": {
        "source": "projection_enclosure.rs", "offsets": (17496, 17798),
        "sha256": "3a72471263a45f08a425d23796372e6673cf65f45c9791e1169f861472af0e5f",
        "start": b"/// Exact signed integer plus",
        "following": b"impl EndpointParts {",
    },
    "finish": {
        "source": "projection_enclosure.rs", "offsets": (19512, 20597),
        "sha256": "380ee7ec568d0fe73ed5e5d4b188e5cd0c6bf3456d8003045a858896612411af",
        "start": b"    /// Complete outward floor or ceiling",
        "following": b"}\n\n/// Completed private endpoints",
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
        "before": "Ok((integer, rounding))",
        "after": "Err(FixtureEnclosureError::Unrepresentable)",
        "harness": "endpoint_finish::endpoint_finish_control_success",
        "intended_failure": "endpoint required success",
    },
    "omit_negative_floor": {
        "before": "(true, Ordering::Less) => -1,",
        "after": "(true, Ordering::Less) => 0,",
        "harness": "endpoint_finish::endpoint_finish_control_negative_floor",
        "intended_failure": "endpoint exact negative floor",
    },
    "range_as_invariant": {
        "before": "i64::try_from(integer).map_err(|_| FixtureEnclosureError::Unrepresentable)?",
        "after": "i64::try_from(integer).map_err(|_| FixtureEnclosureError::Invariant)?",
        "harness": "endpoint_finish::endpoint_finish_control_range_error",
        "intended_failure": "endpoint range rejection classification",
    },
}
FULL_HARNESS = "endpoint_finish::endpoint_finish_full_domain"
REQUIRED_ASSERTIONS = {
    FULL_HARNESS: (
        "endpoint required success", "endpoint checked overflow classification",
        "endpoint range rejection classification", "endpoint never interval error",
        "endpoint oracle success envelope", "endpoint exact floor relation",
        "endpoint exact ceiling relation", "endpoint residual rounding classification",
    ),
}
REQUIRED_COVERS = {
    FULL_HARNESS: (
        "endpoint raw minimum negative floor is invariant",
        "endpoint raw maximum positive ceiling is invariant",
        "endpoint exact minimum succeeds", "endpoint exact maximum succeeds",
        "endpoint negative floor rounds into range", "endpoint positive ceiling rounds into range",
        "endpoint exact lower neighbor is unrepresentable",
        "endpoint exact upper neighbor is unrepresentable",
        "endpoint negative ceiling needs no adjustment", "endpoint positive floor needs no adjustment",
    ),
}


def digest(data: bytes) -> str:
    """Identify exact bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Return the fixed healthy/three-defect/restored five-command checkpoint."""
    jobs = [{"subject": "endpoint_finish", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "endpoint_finish", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "endpoint_finish", "case": "healthy", "harness": FULL_HARNESS,
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def _extract(originals: dict[str, bytes], name: str) -> bytes:
    """Fail closed unless unique reviewed markers, offsets and span hash agree."""
    span = SPANS[name]
    source = originals[span["source"]]
    first, last = span["offsets"]
    if source.count(span["start"]) != 1 or source.count(span["following"]) != 1:
        raise ValueError("Actual endpoint source extraction is ambiguous: " + name)
    selected = source[first:last]
    if (source.index(span["start"]) != first
            or source.index(span["following"]) < last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed endpoint source span changed: " + name)
    return selected


def prepare_endpoint_finish(source_root: Path, inputs: Path, case: str,
                            harness_path: Path) -> dict:
    """Preserve exact source spans and apply at most one documented local defect.

    All source and output-collision checks precede directory creation. The
    selected method is compiled inside its private type's module; unchecked
    proof assembly confers no guarantee on the omitted product constructor.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown endpoint-finish control")
    core = source_root / "crates/time-core/src"
    originals = {name: (core / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    method = selected["finish"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if method.count(before) != 1:
            raise ValueError("Reviewed endpoint mutation point is not unique")
        method = method.replace(before, after, 1)
    harness = harness_path.read_bytes()
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Byte-exact selected owners; correspondence.json records original locations.\n"
                   b"use std::cmp::Ordering;\n\n"
                   + selected["interval_error"] + b"\n" + selected["rounding"] + b"\n"
                   + selected["enclosure_error"] + b"\n" + selected["parts"]
                   + b"\nimpl EndpointParts {\n" + method + b"}\n")
    # Assertions must be lexically in proof.rs for the runner's existing source
    # qualification. Both included definitions and harness remain private peers.
    proof = (b"#![allow(dead_code)]\nmod endpoint_finish {\n"
             b'include!("endpoint_finish_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "endpoint_finish_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing endpoint-finish inputs")
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in SPANS.items()],
        "original_helper_preserved": case == "healthy", "compiled_helper_sha256": digest(method),
        "original_struct_and_enums_preserved": True, "harness_inside_private_module": True,
        "harness_sha256": digest(harness), "harness_location": "proof.rs",
        "compiled_definitions_sha256": digest(definitions), "entrypoint_sha256": digest(proof),
        "mutation": mutation, "assumptions": [],
        "omitted": ["EndpointParts::new and its reachable-state invariants",
                    "fraction comparison, addition and binary64 decomposition",
                    "Garde/raw-operand admission and checked interval construction",
                    "error formatting and error-trait implementations",
                    "complete enclosure composition, chart roundoff and selection",
                    "production acquisition, native Windows, clock and release authority"],
        "scope": "full raw i128/Ordering/bool EndpointParts::finish success, exact rounding and errors only",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(definitions)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
