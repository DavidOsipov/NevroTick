"""Prepare the exact append-depth owner and its full raw-state contract only."""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import time


PREPARATION_STARTED_UNIX_NS = 1793940925449538908
PREPARATION_STARTED_MONOTONIC_NS = 37593282968081
PREPARATION_DEADLINE_MONOTONIC_NS = 41193282968081
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
SOURCE_SHA256 = {
    "crates/time-core/src/selection.rs": "da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e",
    "crates/time-core/src/lib.rs": "1c6496d9a4a40f77fec75087dbe1811194fff262117e883f88fad6f90f9bb458",
    "crates/time-core/Cargo.toml": "8b51f9fdfb2b65c757a7ad9e9c592340e2873719db14a19fddeb6747b69bcea6",
}
SPANS = {
    "errors": {
        "offsets": (2840, 3497),
        "sha256": "a170a559189752048cff5d05311e826a5fd8a139d867ba9f40ca8f429bf2c1fb",
        "start": b"/// Stage of a checked selection postcondition failure.",
        "following": b"/// Implement fixed-prefix formatting and leaf error chaining for typed failures.",
    },
    "slot_error": {
        "offsets": (16930, 17112),
        "sha256": "e3cb9292a24cb15871f4bd13e9bb0ebfecaa09bd81c792a6b426045e0c4d7607",
        "start": b"/// Fixed-capacity storage or checked index failed its internal contract.",
        "following": b"/// Closed-real maximum-component construction failed its internal contract.",
    },
    "cell_depths": {
        "offsets": (17970, 18320),
        "sha256": "3b4def7923570cf3dbf331bce306a7eff5ca022e7d83b21b2e112ff3fa953ed3",
        "start": b"/// Once-evaluated alternating point/open-span depths in real-axis order.",
        "following": b"/// Count of all maximum components and the first one's closed endpoints.",
    },
    "implementation": {
        "offsets": (21715, 22215),
        "sha256": "42bed5ef51a0655584140e7e4b47483f0fd23c07a5f42cf3eea53e6f5b341439",
        "start": b"/// Retain one computed depth and update its same-prefix maximum.",
        "following": b"/// Evaluate each supplied endpoint and intervening real open span exactly once.",
    },
}
FULL_HARNESS = "selection_depth::selection_depth_full_domain"
CONTROLS = {
    "wrong_slot": {
        "harness": "selection_depth::selection_depth_control_wrong_slot",
        "intended_failure": "append error probe unchanged",
        "before": "get_mut(cells.len)", "after": "get_mut(0)",
    },
    "wrong_length": {
        "harness": "selection_depth::selection_depth_control_wrong_length",
        "intended_failure": "append exact mathematical length",
        "before": "checked_add(1)", "after": "checked_add(0)",
    },
    "wrong_maximum": {
        "harness": "selection_depth::selection_depth_control_wrong_maximum",
        "intended_failure": "append independent maximum",
        "before": "cells.maximum.max(depth)", "after": "cells.maximum.min(depth)",
    },
    "wrong_error": {
        "harness": "selection_depth::selection_depth_control_wrong_error",
        "intended_failure": "append exact slot error",
        "before": "get_mut(cells.len).ok_or(SLOT_ERROR)?",
        "after": "get_mut(cells.len).ok_or(SelectionError::Invariant { stage: SelectionInvariantStage::Component })?",
    },
}
REQUIRED_ASSERTIONS = {FULL_HARNESS: (
    "append required success", "append exact selected slot", "append exact mathematical length",
    "append independent maximum", "append success probe unchanged", "append exact slot error",
    "append error length unchanged", "append error maximum unchanged", "append error probe unchanged",
)}
REQUIRED_COVERS = {FULL_HARNESS: (
    "append first slot succeeds", "append final slot succeeds", "append maximum grows to raw maximum",
    "append maximum retains raw maximum", "append equal maximum succeeds",
    "append first unavailable slot rejects", "append raw maximum length rejects",
    "append nonselected earlier slot retained", "append nonselected later slot retained",
    "append rejection retains distinct probe value",
)}


def digest(data: bytes) -> str:
    """Identify complete or selected original bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def remaining_budget() -> float:
    """Use the single backdated preparation clock and the worse elapsed time."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise ValueError("Selection-depth preparation belongs to a different host boot")
    monotonic_elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall_elapsed = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(monotonic_elapsed, wall_elapsed)
    if (monotonic_elapsed < 0 or wall_elapsed < 0
            or not math.isfinite(remaining) or remaining <= 0):
        raise ValueError("Selection-depth preparation/execution budget expired or clock moved backwards")
    return remaining


def schedule() -> list[dict]:
    """Return exactly healthy, four semantic faults and identical restoration."""
    jobs = [{"subject": "selection_depth", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "selection_depth", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "selection_depth", "case": "healthy", "harness": FULL_HARNESS,
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def harness_contracts() -> dict:
    """Declare all full assertions/covers and each sole intended control failure."""
    return {
        FULL_HARNESS: {"successful_assertions": list(REQUIRED_ASSERTIONS[FULL_HARNESS]),
                       "covers": list(REQUIRED_COVERS[FULL_HARNESS])},
        **{control["harness"]: {"intended_failure": control["intended_failure"], "covers": []}
           for control in CONTROLS.values()},
    }


def _extract(original: bytes, name: str) -> bytes:
    """Require unique reviewed anchors, exact offsets and the selected-byte pin."""
    span = SPANS[name]
    if original.count(span["start"]) != 1 or original.count(span["following"]) != 1:
        raise ValueError("Actual selection-depth source extraction is ambiguous: " + name)
    first, last = span["offsets"]
    selected = original[first:last]
    if (original.index(span["start"]) != first
            or original.index(span["following"]) != last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed selection-depth source span changed: " + name)
    return selected


def prepare_selection_depth(source_root: Path, inputs: Path, case: str,
                            harness_path: Path) -> dict:
    """Copy complete actual owners with at most one fixed helper-only defect.

    Raw scratch assembly assumes no completed CellDepths prefix invariant.
    This function executes no compiler, solver or frozen Python source. All
    source and output-collision checks precede writes.
    """
    if type(case) is not str or case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown selection-depth control")
    originals = {name: (source_root / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    source_name = "crates/time-core/src/selection.rs"
    selected = {name: _extract(originals[source_name], name) for name in SPANS}
    implementation = selected["implementation"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if implementation.count(before) != 1:
            raise ValueError("Reviewed selection-depth mutation point is not unique")
        implementation = implementation.replace(before, after, 1)
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Exact selected actual definitions; see correspondence.json.\n"
                   + selected["errors"] + selected["slot_error"]
                   + selected["cell_depths"] + implementation)
    harness = harness_path.read_bytes()
    proof = (b"#![allow(dead_code)]\nmod selection_depth {\n"
             b'include!("selection_depth_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "selection_depth_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Selection-depth output already exists")
    correspondence = [
        {"owner": name, "source": source_name,
         "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
         "selection": "exact original bytes before the named helper-only defect"}
        for name, span in SPANS.items()]
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "compiled_definitions_sha256": digest(definitions),
        "compiled_implementation_sha256": digest(implementation),
        "proof_spec_sha256": digest(harness), "proof_sha256": digest(proof),
        "mutation": mutation, "correspondence": correspondence,
        "preparation_api": "prepare_selection_depth(source_root, inputs, case, harness_path)",
        "harness_contracts": harness_contracts(), "recommended_schedule": schedule(),
        "machine_widths": {"usize": 64, "length_specification": 128},
        "domain": {"independent_array_values": 63, "independent_scalar_values": 4,
                   "raw_value_type": "usize", "assumed_prefix_invariants": []},
        "selected_original_bytes": sum(last - first for first, last in
                                        (span["offsets"] for span in SPANS.values())),
        "assumptions": [],
        "omitted": ["initialized-prefix validity and maximum truth", "interval-membership depth truth",
                    "sorted endpoint keys", "component topology and whole majority selection",
                    "other selection owners", "error formatting and error-trait implementations"],
        "scope": "full raw-state append-depth success/error transition and universal array frame",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    (inputs / "selection_depth_selected.rs").write_bytes(definitions)
    (inputs / "proof.rs").write_bytes(proof)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
