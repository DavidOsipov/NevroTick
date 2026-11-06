"""Prepare two exact source-field spans and a complete independent tuple contract."""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import time


PREPARATION_STARTED_UNIX_NS = 1793944926242703570
PREPARATION_STARTED_MONOTONIC_NS = 41594076128365
PREPARATION_DEADLINE_MONOTONIC_NS = 45194076128365
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
SOURCE_SHA256 = {
    "crates/time-core/src/combination/lifecycle.rs":
        "6e415208ccfd93ebe976c09c887c8c42090f85bfec313a17c19594a891dc5081",
}
SPANS = {
    "constants": {
        "source": "crates/time-core/src/combination/lifecycle.rs",
        "offsets": (3114, 3310),
        "sha256": "1e17644c506c8ccaae958f5c844f42efc39bf07a24264ba454eb3f77e42a1cb9",
        "start": b"/// Chrony reachability-history width used only for fixture maturity.",
        "following": b"\n/// Additive tracking lifecycle failure without raw entries or state payloads.",
    },
    "implementation": {
        "source": "crates/time-core/src/combination/lifecycle.rs",
        "offsets": (18571, 18849),
        "sha256": "7607cd68f02e8979c86cc67a74dae22bbee67df1de495e9b3918ab6ebaa78545",
        "start": b"/// State rules cover the relationship between maturity, watermark and penalty.",
        "following": b"/// Delegate the complete bounded source-state relationship to one predicate.",
    },
}
FULL_HARNESS = "lifecycle_source_fields::lifecycle_source_fields_full_domain"
CONTROLS = {
    "strict_penalty": {
        "harness": "lifecycle_source_fields::lifecycle_source_fields_control_strict_penalty",
        "intended_failure": "source fields penalty classification",
        "span": "implementation",
        "before": "value.1 <= DISTANT_PENALTY", "after": "value.1 < DISTANT_PENALTY",
    },
    "wrong_maturity": {
        "harness": "lifecycle_source_fields::lifecycle_source_fields_control_wrong_maturity",
        "intended_failure": "source fields maturity classification",
        "span": "constants",
        "before": "const MATURE_UPDATES: u8 = 8;", "after": "const MATURE_UPDATES: u8 = 7;",
    },
    "loose_relationship": {
        "harness": "lifecycle_source_fields::lifecycle_source_fields_control_loose_relationship",
        "intended_failure": "source fields required rejection",
        "span": "implementation",
        "before": "u64::from(value.0) == value.2.min(u64::from(MATURE_UPDATES))",
        "after": "u64::from(value.0) <= value.2.min(u64::from(MATURE_UPDATES))",
    },
}
REQUIRED_ASSERTIONS = {FULL_HARNESS: (
    "source fields exact classification", "source fields penalty classification",
    "source fields maturity classification", "source fields required rejection",
    "source fields input retained",
)}
REQUIRED_COVERS = {FULL_HARNESS: (
    "source fields zero admitted", "source fields last immature admitted",
    "source fields exact maturity and penalty admitted", "source fields after maturity admitted",
    "source fields maximum ordinal admitted", "source fields low mature value rejected",
    "source fields first excess penalty rejected", "source fields maximum penalty rejected",
    "source fields first excess maturity rejected", "source fields maximum maturity rejected",
    "source fields low relationship rejected", "source fields high relationship rejected",
)}


def digest(data: bytes) -> str:
    """Identify complete or selected original bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def remaining_budget() -> float:
    """Use the single backdated preparation clock and the worse elapsed time."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise ValueError("Lifecycle source-fields preparation belongs to a different host boot")
    monotonic_elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall_elapsed = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(monotonic_elapsed, wall_elapsed)
    if (monotonic_elapsed < 0 or wall_elapsed < 0
            or not math.isfinite(remaining) or remaining <= 0):
        raise ValueError("Lifecycle source-fields preparation/execution budget expired or clock moved backwards")
    return remaining


def schedule() -> list[dict]:
    """Return exactly healthy, three semantic faults and identical restoration."""
    jobs = [{"subject": "lifecycle_source_fields", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "lifecycle_source_fields", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "lifecycle_source_fields", "case": "healthy", "harness": FULL_HARNESS,
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
        raise ValueError("Actual lifecycle-source-fields source extraction is ambiguous: " + name)
    first, last = span["offsets"]
    selected = original[first:last]
    if (original.index(span["start"]) != first
            or original.index(span["following"]) != last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed lifecycle-source-fields source span changed: " + name)
    return selected


def prepare_lifecycle_source_fields(source_root: Path, inputs: Path, case: str,
                                    harness_path: Path) -> dict:
    """Copy two exact actual spans, preserving standard calls, with one fixed defect.

    Only reviewed module/include framing is added. No compiler, solver or frozen
    Python source is executed. Source and output checks precede writes.
    """
    if type(case) is not str or case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown lifecycle-source-fields control")
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
            raise ValueError("Reviewed lifecycle-source-fields mutation point is not unique")
        selected[span] = selected[span].replace(before, after, 1)
    definitions, implementation = selected["constants"], selected["implementation"]
    harness = harness_path.read_bytes()
    proof = (
        b"#![allow(dead_code)]\n"
        b"// SPDX-License-Identifier: GPL-2.0-only\n"
        b"mod lifecycle_source_fields {\n"
        b'include!("lifecycle_source_fields_selected.rs");\n'
        b'include!("lifecycle_source_fields_owner.rs");\n' + harness + b"\n}\n")
    outputs = {"proof.rs": proof, "lifecycle_source_fields_selected.rs": definitions,
               "lifecycle_source_fields_owner.rs": implementation}
    destinations = [inputs / name for name in (*outputs, "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Lifecycle source-fields output already exists")
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
        "preparation_api": "prepare_lifecycle_source_fields(source_root, inputs, case, harness_path)",
        "harness_contracts": harness_contracts(), "recommended_schedule": schedule(),
        "machine_widths": {"maturity": 8, "penalty": 8, "ordinal": 64, "specification": 128},
        "domain": {"independent_scalar_values": 3, "raw_value_types": ["u8", "u8", "u64"],
                   "raw_tuple_count": 2 ** 80, "assumed_input_invariants": []},
        "selected_original_bytes": sum(last - first for first, last in
                                        (span["offsets"] for span in SPANS.values())),
        "assumptions": [],
        "omitted": ["source_fields_rule and Garde composition", "private source construction",
                    "with_penalty and record_reachability transitions", "saturation and rollback",
                    "reset and currentness authority", "whole lifecycle"],
        "scope": "full raw-u8/u8/u64 source_fields_valid Boolean equivalence and unchanged inputs",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name, data in outputs.items():
        (inputs / name).write_bytes(data)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
