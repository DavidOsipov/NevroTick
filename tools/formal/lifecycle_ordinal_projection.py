"""Prepare the exact lifecycle-ordinal owner and its full raw-state contract only."""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import time


PREPARATION_STARTED_UNIX_NS = 1793943385621323997
PREPARATION_STARTED_MONOTONIC_NS = 40053454753050
PREPARATION_DEADLINE_MONOTONIC_NS = 43653454753050
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
SOURCE_SHA256 = {'crates/time-core/src/combination/lifecycle.rs': '6e415208ccfd93ebe976c09c887c8c42090f85bfec313a17c19594a891dc5081',
 'crates/time-core/src/principal.rs': 'cf4962c8d3536a528b7c783543379bdaeaf2e4070355fbeb7921fb5329b63d1d',
 'crates/time-core/src/combination.rs': 'd33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326',
 'crates/time-core/src/regression.rs': '52223fcf848927b344673ec810ca0c784045c8e61835a01e9451de236d8f28d4',
 'crates/time-core/src/selection.rs': 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e',
 'crates/time-core/src/lib.rs': '1c6496d9a4a40f77fec75087dbe1811194fff262117e883f88fad6f90f9bb458',
 'crates/time-core/Cargo.toml': '8b51f9fdfb2b65c757a7ad9e9c592340e2873719db14a19fddeb6747b69bcea6'}
SPANS = {'lifecycle_errors': {'source': 'crates/time-core/src/combination/lifecycle.rs',
                      'offsets': (6261, 9644),
                      'sha256': 'a046ca293c8107e68e3272b47397eb4e8542e69b7a3ddf1e1163a26c96e852e2',
                      'start': b'/// Explicit uncalibrated fixture parameter '
                               b'whose constructor rejected input.',
                      'following': b'impl fmt::Debug for FixtureCombinationLi'
                                   b'fecycleError {'},
 'implementation': {'source': 'crates/time-core/src/combination/lifecycle.rs',
                    'offsets': (22383, 22780),
                    'sha256': '75c2c84828f21cb724d933972c0c3680c8917a00511ed7af6bb11aae259ec7bb',
                    'start': b'/// Prepare a nonwrapping next ordinal over the '
                             b'complete raw u64 domain.',
                    'following': b'/// Bounded event/owner relationship, separa'
                                 b'te from raw-ordinal construction.'},
 'principal_errors': {'source': 'crates/time-core/src/principal.rs',
                      'offsets': (2988, 7799),
                      'sha256': '62c2cda5cb9499047bab014ccd21e672264d7f110bf454fc9be4dc6ff10ee29e',
                      'start': b'/// Explicit parameter whose shared scalar r'
                               b'ule rejected a raw value.',
                      'following': b'impl fmt::Debug for FixturePrincipalErro'
                                   b'r {'},
 'combination_errors': {'source': 'crates/time-core/src/combination.rs',
                        'offsets': (4658, 10807),
                        'sha256': '41d9727c848f4c56d15d1485be410a4f2e3161c91d71d7998c497412fc781b10',
                        'start': b'/// Supplied uncertainty role, in its constr'
                                 b"uctor's first-error order.",
                        'following': b'impl fmt::Debug for FixtureCombinationEr'
                                     b'ror {'},
 'regression_errors': {'source': 'crates/time-core/src/regression.rs',
                       'offsets': (4900, 9375),
                       'sha256': '1b4afdc9cd09e8d1d7f80e436b5f5eb39fd2502d30c95bee8bbabe50ccf0f7ca',
                       'start': b'/// Constructor argument role, in the declar'
                                b'ed first-error precedence.',
                       'following': b'impl fmt::Display for FitError {'},
 'selection_errors': {'source': 'crates/time-core/src/selection.rs',
                      'offsets': (2840, 3497),
                      'sha256': 'a170a559189752048cff5d05311e826a5fd8a139d867ba9f40ca8f429bf2c1fb',
                      'start': b'/// Stage of a checked selection postconditi'
                               b'on failure.',
                      'following': b'/// Implement fixed-prefix formatting an'
                                   b'd leaf error chaining for typed failures'
                                   b'.'}}
FULL_HARNESS = "lifecycle_ordinal::lifecycle_ordinal_full_domain"
CONTROLS = {
    "always_error": {
        "harness": "lifecycle_ordinal::lifecycle_ordinal_control_always_error",
        "intended_failure": "ordinal success iff bound",
        "before": ".and_then(NonZeroU64::new)", "after": ".and_then(|_| None)",
    },
    "wrong_increment": {
        "harness": "lifecycle_ordinal::lifecycle_ordinal_control_wrong_increment",
        "intended_failure": "ordinal exact successor",
        "before": ".checked_add(1)", "after": ".checked_add(2)",
    },
    "wrong_error": {
        "harness": "lifecycle_ordinal::lifecycle_ordinal_control_wrong_error",
        "intended_failure": "ordinal exact exhaustion error",
        "before": ".ok_or(FixtureCombinationLifecycleError::CounterExhausted)",
        "after": ".ok_or(FixtureCombinationLifecycleError::Invariant)",
    },
}
REQUIRED_ASSERTIONS = {FULL_HARNESS: (
    "ordinal success iff bound", "ordinal exact successor", "ordinal nonzero output",
    "ordinal exact exhaustion error", "ordinal exhaustion iff max", "ordinal input retained",
)}
REQUIRED_COVERS = {FULL_HARNESS: (
    "ordinal zero reaches one", "ordinal one reaches two", "ordinal high bit successor",
    "ordinal final successor", "ordinal maximum exhausts",
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
    """Return exactly healthy, three semantic faults and identical restoration."""
    jobs = [{"subject": "lifecycle_ordinal", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "lifecycle_ordinal", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "lifecycle_ordinal", "case": "healthy", "harness": FULL_HARNESS,
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
        raise ValueError("Actual lifecycle-ordinal source extraction is ambiguous: " + name)
    first, last = span["offsets"]
    selected = original[first:last]
    if (original.index(span["start"]) != first
            or original.index(span["following"]) != last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed lifecycle-ordinal source span changed: " + name)
    return selected


def prepare_lifecycle_ordinal(source_root: Path, inputs: Path, case: str,
                              harness_path: Path) -> dict:
    """Copy six exact actual spans and their full type closure, with one fixed defect.

    Only reviewed module/import framing is added. No compiler, solver or frozen
    Python source is executed. All source/output checks precede writes.
    """
    if type(case) is not str or case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown lifecycle-ordinal control")
    originals = {name: (source_root / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals[span["source"]], name)
                for name, span in SPANS.items()}
    implementation = selected["implementation"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if implementation.count(before) != 1:
            raise ValueError("Reviewed lifecycle-ordinal mutation point is not unique")
        implementation = implementation.replace(before, after, 1)
    definitions = (
        b"// SPDX-License-Identifier: GPL-2.0-only\n"
        b"// Exact actual enum closure; source correspondence retains original notices.\n"
        b"mod regression {\n" + selected["regression_errors"] + b"}\n"
        b"pub use regression::FitError;\n"
        b"mod selection {\n" + selected["selection_errors"] + b"}\n"
        b"pub use selection::SelectionError;\n"
        b"mod principal {\nuse crate::{FitError, SelectionError};\n"
        + selected["principal_errors"] + b"}\n"
        b"pub use principal::FixturePrincipalError;\n"
        b"mod combination {\nuse crate::FitError;\n"
        + selected["combination_errors"] + b"}\n")
    owner = selected["lifecycle_errors"] + implementation
    harness = harness_path.read_bytes()
    proof = (
        b"#![allow(dead_code)]\n"
        b'include!("lifecycle_ordinal_selected.rs");\n'
        b"mod lifecycle_ordinal {\nuse std::num::NonZeroU64;\n"
        b"use crate::{FitError, FixturePrincipalError};\n"
        b"use crate::combination::FixtureCombinationError;\n"
        b'include!("lifecycle_ordinal_owner.rs");\n' + harness + b"\n}\n")
    outputs = {"proof.rs": proof, "lifecycle_ordinal_selected.rs": definitions,
               "lifecycle_ordinal_owner.rs": owner}
    destinations = [inputs / name for name in (*outputs, "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Lifecycle-ordinal output already exists")
    correspondence = [
        {"owner": name, "source": span["source"],
         "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
         "selection": "exact original bytes before the named helper-only defect"}
        for name, span in SPANS.items()]
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "compiled_definitions_sha256": digest(definitions + owner),
        "compiled_implementation_sha256": digest(implementation),
        "compiled_files_sha256": {name: digest(data) for name, data in outputs.items()},
        "proof_spec_sha256": digest(harness), "proof_sha256": digest(proof),
        "mutation": mutation, "correspondence": correspondence,
        "preparation_api": "prepare_lifecycle_ordinal(source_root, inputs, case, harness_path)",
        "harness_contracts": harness_contracts(), "recommended_schedule": schedule(),
        "machine_widths": {"u64": 64, "successor_specification": 128},
        "domain": {"independent_scalar_values": 1, "raw_value_type": "u64",
                   "assumed_input_invariants": [], "actual_enum_declarations": 23},
        "selected_original_bytes": sum(last - first for first, last in
                                        (span["offsets"] for span in SPANS.values())),
        "assumptions": [],
        "omitted": ["global and cross-owner uniqueness", "event and currentness authority",
                    "transactional admit_event and emit_round callers", "counter reset authority",
                    "error formatting and error-trait implementations"],
        "scope": "full raw-u64 checked_next success/error relation and unchanged input",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name, data in outputs.items():
        (inputs / name).write_bytes(data)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
