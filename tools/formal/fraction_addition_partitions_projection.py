"""Source-bound full-domain ordinary-Kani addition partition preparation."""
from __future__ import annotations

PREPARATION_STARTED_UNIX_NS = 1793913403729453001
PREPARATION_STARTED_MONOTONIC_NS = 10071562880982
PREPARATION_BOOT_ID = 'd3fc5615-e867-45cd-b74f-4a7413afc5ab'

import hashlib
import importlib.util
import json
import math
from pathlib import Path
import re
import time


PREPARATION_DEADLINE_MONOTONIC_NS = 13671562880982
TOTAL_SECONDS = 3600
FIXED_OUTPUT_LEAF = "formal-core-fraction-addition-partitions-20261106-v1"
SUPPORT_SHA256 = {
    "fraction_arithmetic_projection.py": "9092cf40b90f60971b835bf573070fb2faa10e6cc9826f485cb6e6919da80046",
    "fraction_arithmetic_harness.rs": "d640170cff40d9bebafa7318e4501c2db4f98382a523050d6bbb07c9a2319cea",
}
MODULE = "fraction_addition_partitions::"
UNION = MODULE + "fraction_addition_partition_union"
ALL_ASSERTIONS = (
    "addition left factory completeness", "addition right factory completeness",
    "addition required success", "addition exact carry", "addition exact residual",
    "addition oracle alignment exact", "addition oracle proper numerator bounds",
    "addition oracle sum representable", "addition high denominator positive",
    "addition global fraction maximum", "addition high numerator bound",
)
UNION_ASSERTIONS = (
    "addition partition exactly one guard", "addition partition raw rejection equivalence",
    "addition partition raw success equivalence", "addition partition left numerator exact",
    "addition partition left canonical shift exact", "addition partition right numerator exact",
    "addition partition right canonical shift exact",
)
LEAF_COVERS = {
    0: ("addition partition invalid left factory", "addition partition invalid right factory"),
    1: ("addition zero sum", "addition quarter plus zero", "addition zero plus quarter",
        "addition two quarters", "addition minimum positive plus zero",
        "addition zero plus minimum positive", "addition two minimum positives",
        "addition raw zeros canonicalize"),
    2: ("addition u128 edge left", "addition high denominator left"),
    3: ("addition exact halves", "addition three quarters plus quarter",
        "addition three quarters plus half", "addition half plus three quarters",
        "addition unreduced halves", "addition just below unit", "addition just above unit",
        "addition maximum numerator sum", "addition half plus quarter"),
    4: ("addition u128 edge right", "addition high denominator right"),
    5: ("addition quarter plus three quarters", "addition quarter plus half"),
}
REQUIRED_ASSERTIONS = {UNION: UNION_ASSERTIONS}
REQUIRED_ASSERTIONS.update({
    MODULE + "fraction_addition_partition_" + str(index):
        ALL_ASSERTIONS[:2] if index == 0 else ALL_ASSERTIONS[:8] if index in (3, 5) else ALL_ASSERTIONS
    for index in range(6)
})
REQUIRED_COVERS = {
    UNION: tuple("addition partition guard " + str(index) + " reachable" for index in range(6))
        + ("addition partition raw left rejection neighbor", "addition partition raw left acceptance neighbor",
           "addition partition raw right rejection neighbor", "addition partition raw right acceptance neighbor"),
    **{MODULE + "fraction_addition_partition_" + str(index): covers
       for index, covers in LEAF_COVERS.items()},
}
GUARDS = (
    "!A",
    "A && !L(left) && !L(right)",
    "A && L(left) && canonical_right_shift >= 106",
    "A && L(left) && canonical_right_shift < 106",
    "A && !L(left) && L(right) && canonical_left_shift >= 106",
    "A && !L(left) && L(right) && canonical_left_shift < 106",
)


def digest(data: bytes) -> str:
    """Identify exact bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def remaining_budget() -> float:
    """Enforce the once-recorded preparation-to-decision wall/monotonic cap."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise ValueError("Partition preparation belongs to a different host boot")
    monotonic_elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall_elapsed = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(monotonic_elapsed, wall_elapsed)
    if (monotonic_elapsed < 0 or wall_elapsed < 0
            or not math.isfinite(remaining) or remaining <= 0):
        raise ValueError("Partition preparation/execution budget expired or clock moved backwards")
    return remaining


def schedule() -> list[dict]:
    """One union theorem, six complete leaves, three original faults, six restorations."""
    jobs = [{"subject": "fraction_addition_partition_union", "case": "healthy",
             "harness": UNION, "intended_failure": None, "restored_healthy": False}]
    for index in range(6):
        subject = "fraction_addition_partition_" + str(index)
        jobs.append({"subject": subject, "case": "healthy", "harness": MODULE + subject,
                     "intended_failure": None, "restored_healthy": False})
    for case, suffix, failure in (
            ("always_error", "success", "addition required success"),
            ("wrong_carry", "carry", "addition exact carry"),
            ("wrong_residual", "residual", "addition exact residual")):
        jobs.append({"subject": "fraction_addition_partition_fault", "case": case,
                     "harness": MODULE + "fraction_addition_control_" + suffix,
                     "intended_failure": failure, "restored_healthy": False})
    jobs.extend({**job, "restored_healthy": True} for job in jobs[1:7])
    return jobs


def _support(source_root: Path):
    folder = source_root / "tools/formal"
    for name, expected in SUPPORT_SHA256.items():
        if digest((folder / name).read_bytes()) != expected:
            raise ValueError("Reviewed partition support changed: " + name)
    path = folder / "fraction_arithmetic_projection.py"
    spec = importlib.util.spec_from_file_location("partition_original_arithmetic", path)
    original = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(original)
    return folder, original


def _original_harness_parts(harness: bytes) -> tuple[bytes, bytes, dict[str, bytes]]:
    """Keep helper and defect bodies intact; identify all original witness blocks."""
    helpers = harness[harness.index(b"const HALF_NUMERATOR:"):harness.index(b"#[kani::proof]")]
    controls = harness[harness.index(b"#[kani::proof]\nfn fraction_addition_control_success"):]
    full = harness.split(b"#[kani::proof]\nfn fraction_addition_full_domain() {", 1)[1]
    full = full.split(b"\n#[kani::proof]", 1)[0]
    blocks = re.findall(rb"    kani::cover!\(\n.*?    \);\n", full, re.DOTALL)
    if len(blocks) != 23:
        raise ValueError("Original full-domain addition witnesses changed")
    witnesses = {}
    for block in blocks:
        strings = re.findall(rb'"([^"\n]*)"', block)
        if not strings or strings[-1].decode() in witnesses:
            raise ValueError("Original addition witness label is ambiguous")
        witnesses[strings[-1].decode()] = block
    return helpers, controls, witnesses


def prepare_fraction_addition_partitions(source_root: Path, inputs: Path, subject: str,
                                        case: str, harness_path: Path) -> dict:
    """Prepare exact source owners and ordinary proofs; invoke no native tool."""
    jobs = [job for job in schedule() if (job["subject"], job["case"]) == (subject, case)]
    if not jobs:
        raise ValueError("Unknown fraction addition partition subject/control")
    folder, original = _support(source_root)
    if harness_path != folder / "fraction_addition_partitions_harness.rs":
        raise ValueError("Partition harness must come from its fixed frozen source path")
    originals = {name: (source_root / "crates/time-core/src" / name).read_bytes()
                 for name in original.SOURCE_SHA256}
    for name, expected in original.SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: original._extract(originals, name) for name in original.SPANS}
    compiled = dict(selected)
    mutation = None
    if subject == "fraction_addition_partition_fault":
        mutation = dict(original.CONTROLS["fraction_addition"][case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if compiled["addition"].count(before) != 1:
            raise ValueError("Original addition mutation point is not unique")
        compiled["addition"] = compiled["addition"].replace(before, after, 1)
    harness = harness_path.read_bytes()
    helpers, controls, witnesses = _original_harness_parts(
        (folder / "fraction_arithmetic_harness.rs").read_bytes())
    allocated = [label for index in range(1, 6) for label in LEAF_COVERS[index]]
    if len(allocated) != len(set(allocated)) or set(allocated) != set(witnesses):
        raise ValueError("Partition cover ledger omits or duplicates an original witness")
    for label, block in witnesses.items():
        if harness.count(block) != 1:
            raise ValueError("Original addition cover expression changed: " + label)
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\nuse std::cmp::Ordering;\n"
                   + compiled["interval_error"] + b"\n" + compiled["enclosure_error"] + b"\n"
                   + compiled["fraction"] + b"\nimpl Fraction {\n" + compiled["zero"]
                   + b"\n" + compiled["new"] + b"\n" + compiled["compare"]
                   + b"\n" + compiled["complement"] + b"}\n" + compiled["fraction_sum"]
                   + b"\n" + compiled["addition"])
    proof = (b"#![allow(dead_code)]\nmod fraction_addition_partitions {\n"
             b'include!("fraction_addition_partitions_selected.rs");\n'
             + helpers + b"\n" + harness + b"\n" + controls + b"\n}\n")
    leaf = int(subject.rsplit("_", 1)[1]) if subject.rsplit("_", 1)[1].isdigit() else None
    details = {
        "entrypoint": inputs / "proof.rs", "subject": subject,
        "original_sources_sha256": dict(original.SOURCE_SHA256),
        "support_sha256": dict(SUPPORT_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in original.SPANS.items()],
        "selected_arithmetic_owner": "addition", "original_helper_preserved": mutation is None,
        "compiled_helper_sha256": digest(compiled["addition"]), "mutation": mutation,
        "original_callees_and_other_arithmetic_preserved": True,
        "original_math_helpers_sha256": digest(helpers), "original_fault_harnesses_sha256": digest(controls),
        "partition_harness_sha256": digest(harness), "compiled_definitions_sha256": digest(definitions),
        "entrypoint_sha256": digest(proof), "harness_inside_private_module": True,
        "harness_location": "proof.rs", "partition_guard": None if leaf is None else GUARDS[leaf],
        "assumptions": [] if leaf is None else ["partition_guards(n, s, m, t)[" + str(leaf) + "]"],
        "guard_definitions": list(GUARDS), "raw_admission_precedes_zero_normalization": True,
        "original_cover_allocation": {str(index): list(LEAF_COVERS[index]) for index in range(1, 6)},
        "aggregate_requires": "union theorem and all six healthy/restored leaves; three original semantic defects",
        "trusted_high_denominator_specification": "canonical M>=128 means a positive term <2^-75; every checked fraction <=1-2^-53, so sum is strictly between zero and one; typed supporting bounds are checked without a callee oracle",
        "omitted": ["Garde and caller admission", "EndpointParts construction and complete enclosure", "complete Core/P1", "Windows, clock and release authority"],
        "scope": "complete original raw factory input union; unchanged checked-fraction addition success/carry/residual contract",
    }
    files = {"proof.rs": proof, "fraction_addition_partitions_selected.rs": definitions,
             "correspondence.json": (json.dumps({**details, "entrypoint": "proof.rs"},
                                                 indent=2) + "\n").encode()}
    if inputs.is_symlink() or any((inputs / name).exists() or (inputs / name).is_symlink() for name in files):
        raise ValueError("Refusing to overwrite existing partition inputs")
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name, data in files.items():
        (inputs / name).write_bytes(data)
    return details
