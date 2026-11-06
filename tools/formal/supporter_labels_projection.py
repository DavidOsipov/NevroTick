"""Prepare the complete actual supporter-label caller and its reached generic sort."""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import time


PREPARATION_STARTED_UNIX_NS = 1793949009810488193
PREPARATION_STARTED_MONOTONIC_NS = 45677643916875
PREPARATION_DEADLINE_MONOTONIC_NS = 49277643916875
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
SOURCE_SHA256 = {
    "crates/time-core/src/selection.rs": "da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e",
    "crates/time-core/src/lib.rs": "1c6496d9a4a40f77fec75087dbe1811194fff262117e883f88fad6f90f9bb458",
    "crates/time-core/Cargo.toml": "8b51f9fdfb2b65c757a7ad9e9c592340e2873719db14a19fddeb6747b69bcea6",
}
SPANS = {
    "errors": {
        "source": "crates/time-core/src/selection.rs", "offsets": (2840, 3497),
        "sha256": "a170a559189752048cff5d05311e826a5fd8a139d867ba9f40ca8f429bf2c1fb",
        "start": b"/// Stage of a checked selection postcondition failure.",
        "following": b"/// Implement fixed-prefix formatting and leaf error chaining for typed failures.",
    },
    "slot_error": {
        "source": "crates/time-core/src/selection.rs", "offsets": (16930, 17112),
        "sha256": "e3cb9292a24cb15871f4bd13e9bb0ebfecaa09bd81c792a6b426045e0c4d7607",
        "start": b"/// Fixed-capacity storage or checked index failed its internal contract.",
        "following": b"/// Closed-real maximum-component construction failed its internal contract.",
    },
    "support_error": {
        "source": "crates/time-core/src/selection.rs", "offsets": (17307, 17497),
        "sha256": "cd3b4aef88c60e41793e8cf2efcd391f1fa0952f18bea08502f0da72769ccb17",
        "start": b"/// The complete witness support did not match the evaluated maximum depth.",
        "following": b"/// Supporters' common intersection did not equal the sole maximum component.",
    },
    "sort_prefix": {
        "source": "crates/time-core/src/selection.rs", "offsets": (18618, 19644),
        "sha256": "e33639178b0d2455fe2d74db3958f1cf78ef0aa68a1ce5faf68e4910acc1c8de",
        "start": b"/// Sort at most 32 initialized values in place without allocating or panicking.",
        "following": b"/// Collect, sort and deduplicate the two exact endpoints of each checked row.",
    },
    "implementation": {
        "source": "crates/time-core/src/selection.rs", "offsets": (25799, 26702),
        "sha256": "b28c586d1b85e12277240d3c4c91caa77d1bf3362e2875139ed599ad7cc4e5d9",
        "start": b"/// Project and sort only the populated nonzero supporter-label prefix.",
        "following": b"/// Build a witness from all intervals containing the sole component's left end.",
    },
}
FULL_HARNESS = "supporter_labels::supporter_labels_full_domain"
CONTROLS = {
    "empty_sort": {
        "harness": "supporter_labels::supporter_labels_control_empty_sort",
        "intended_failure": "supporter prefix ordered", "span": "implementation",
        "before": "sort_prefix(prefix)?;",
        "after": "sort_prefix(prefix.get_mut(..0).ok_or(SLOT_ERROR)?)?;",
    },
    "multiplicity": {
        "harness": "supporter_labels::supporter_labels_control_multiplicity",
        "intended_failure": "supporter prefix multiplicity", "span": "implementation",
        "before": "sort_prefix(prefix)?;",
        "after": "sort_prefix(prefix)?;\n    if let Some(repeated) = prefix.get(1).copied() {\n        *prefix.first_mut().ok_or(SLOT_ERROR)? = repeated;\n    }",
    },
    "wrong_tail": {
        "harness": "supporter_labels::supporter_labels_control_wrong_tail",
        "intended_failure": "supporter first-label tail", "span": "implementation",
        "before": "sort_prefix(prefix)?;",
        "after": "sort_prefix(prefix)?;\n    if count < 16 {\n        let different = NonZeroU64::new(if first.get() == 1 { 2 } else { 1 }).ok_or(SUPPORT_ERROR)?;\n        *labels.get_mut(count).ok_or(SLOT_ERROR)? = different;\n    }",
    },
}
REQUIRED_ASSERTIONS = {FULL_HARNESS: (
    "supporter success iff prefix contract",
    "supporter prefix ordered", "supporter prefix multiplicity", "supporter first-label tail",
    "supporter exact support error", "supporter complete input frame",
)}
REQUIRED_COVERS = {FULL_HARNESS: (
    "supporter one label succeeds", "supporter sixteen labels succeed",
    "supporter zero count rejects", "supporter seventeen count rejects", "supporter maximum count rejects",
    "supporter missing first rejects", "supporter missing interior rejects",
    "supporter duplicate prefix succeeds", "supporter descending sixteen succeeds",
    "supporter maximum labels succeed", "supporter irrelevant None tail succeeds",
    "supporter irrelevant Some tail succeeds",
)}


def digest(data: bytes) -> str:
    """Identify complete or selected original bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def remaining_budget() -> float:
    """Use the single backdated preparation clock and the worse elapsed time."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise ValueError("Supporter-label preparation belongs to a different host boot")
    monotonic_elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall_elapsed = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(monotonic_elapsed, wall_elapsed)
    if (monotonic_elapsed < 0 or wall_elapsed < 0
            or not math.isfinite(remaining) or remaining <= 0):
        raise ValueError("Supporter-label preparation/execution budget expired or clock moved backwards")
    return remaining


def schedule() -> list[dict]:
    """Return exactly healthy, three semantic faults and identical restoration."""
    jobs = [{"subject": "supporter_labels", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "supporter_labels", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "supporter_labels", "case": "healthy", "harness": FULL_HARNESS,
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
        raise ValueError("Actual supporter-labels source extraction is ambiguous: " + name)
    first, last = span["offsets"]
    selected = original[first:last]
    if (original.index(span["start"]) != first
            or original.index(span["following"]) != last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed supporter-labels source span changed: " + name)
    return selected


def prepare_supporter_labels(source_root: Path, inputs: Path, case: str,
                             harness_path: Path) -> dict:
    """Copy all five actual spans, preserving standard calls, with one caller-only defect.

    Only reviewed module/include framing is added. No compiler, solver or frozen
    Python source is executed. Source and output checks precede writes.
    """
    if type(case) is not str or case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown supporter-labels control")
    originals = {name: (source_root / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals[span["source"]], name)
                for name, span in SPANS.items()}
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        span = mutation["span"]
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if selected[span].count(before) != 1:
            raise ValueError("Reviewed supporter-labels mutation point is not unique")
        selected[span] = selected[span].replace(before, after, 1)
    definitions = (b"use std::num::NonZeroU64;\n" + selected["errors"]
                   + selected["slot_error"] + selected["support_error"] + selected["sort_prefix"])
    implementation = selected["implementation"]
    harness = harness_path.read_bytes()
    proof = (
        b"#![allow(dead_code)]\n"
        b"// SPDX-License-Identifier: GPL-2.0-only\n"
        b"mod supporter_labels {\n"
        b'include!("supporter_labels_selected.rs");\n'
        b'include!("supporter_labels_owner.rs");\n' + harness + b"\n}\n")
    outputs = {"proof.rs": proof, "supporter_labels_selected.rs": definitions,
               "supporter_labels_owner.rs": implementation}
    destinations = [inputs / name for name in (*outputs, "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Supporter-label output already exists")
    correspondence = [
        {"owner": name, "source": span["source"],
         "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
         "selection": "exact original bytes before the named single source defect"}
        for name, span in SPANS.items()]
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "compiled_definitions_sha256": digest(definitions + implementation),
        "compiled_implementation_sha256": digest(implementation),
        "compiled_files_sha256": {name: digest(data) for name, data in outputs.items()},
        "proof_spec_sha256": digest(harness), "proof_sha256": digest(proof),
        "mutation": mutation, "correspondence": correspondence,
        "preparation_api": "prepare_supporter_labels(source_root, inputs, case, harness_path)",
        "harness_contracts": harness_contracts(), "recommended_schedule": schedule(),
        "machine_widths": {"usize": 64, "raw_label": 64, "multiplicity_census": 8},
        "domain": {"independent_array_values": 16, "independent_scalar_values": 3,
                   "primitive_any_calls": 19, "option_mapping": "real NonZeroU64::new per raw u64",
                   "raw_value_types": ["u64", "usize", "u64", "usize"],
                   "assumed_input_invariants": []},
        "effective_harness_unwind": 17,
        "selected_original_bytes": sum(last - first for first, last in
                                        (span["offsets"] for span in SPANS.values())),
        "assumptions": [],
        "omitted": ["generic Ord laws beyond the reached NonZeroU64 specialization",
                    "standalone sorting including lengths 17 through 32 and invalid slices",
                    "i64 endpoint sorting", "whole selection", "error formatting and error-trait implementations"],
        "scope": "full raw option-array/count supporter_labels admission, order, multiplicity, tail, error and input frame",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name, data in outputs.items():
        (inputs / name).write_bytes(data)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
