"""Prepare the closed universal/direct-witness fraction query split.

The original eighteen-job source kit and controller are immutable dependencies.
This wrapper changes only the selected healthy addition query; it starts no
native tool, imports no certificate and changes no original domain or property.
"""
from __future__ import annotations

import sys

# This must precede every project-module import, including a caller's schedule().
sys.dont_write_bytecode = True

import hashlib
import importlib.util
import json
from pathlib import Path
import re


PREPARATION_STARTED_UNIX_NS = 1793921076355277638
PREPARATION_STARTED_MONOTONIC_NS = 17744188704676
PREPARATION_DEADLINE_MONOTONIC_NS = 21344188704676
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
FIXED_OUTPUT_LEAF = "formal-core-fraction-query-split-20261106-v1"
PRESERVED_SHA256 = {
    "fraction_composition_projection.py": "440333800ddd4a046ad30bc483a782ec737e5a53b8b41bce83e750b4baa650c7",
    "fraction_composition_spec.rs": "fa2d61b0f428f572150303ce29b98f57bcf044769a5aba02b50d1b5c259c39b0",
    "fraction_composition_harness.rs": "511e28b069cccfee06f0f50d479dd689a521638ac6f542efac8f21e8ab428cd6",
    "test_fraction_composition.py": "2c58683a65e4203de0fc2fa3bcd59cb2127937385433f9086b42a5e0b1d14c77",
    "verify_fraction_composition.py": "767df3c5e956a52eaaf0b1378882b4ca4d1932746ceb674e6f3ed05fd3035c3d",
    "test_verify_fraction_composition.py": "9fafc445028ad0156894b42c179ba17117e7051e76bf42169025809c130ac375",
}
ORIGINAL_ARITHMETIC_HARNESS_SHA256 = "d640170cff40d9bebafa7318e4501c2db4f98382a523050d6bbb07c9a2319cea"
ORIGINAL_FULL_FUNCTION_SHA256 = "93a97157e5c6fb9b4e7dcc73fe821cf30f5576d1c02a3f9be3d0af6b310d84bf"
UNIVERSAL_SUBJECT = "fraction_addition_universal"
UNIVERSAL_HARNESS = "fraction_arithmetic::fraction_addition_universal_full_domain"
WITNESS_SUBJECT = "fraction_addition_witnesses"
WITNESS_HARNESS = "fraction_arithmetic::fraction_addition_witnesses_full_domain"
ALL_ASSERTIONS = (
    "addition left factory completeness", "addition right factory completeness",
    "addition required success", "addition exact carry", "addition exact residual",
    "addition oracle alignment exact", "addition oracle proper numerator bounds",
    "addition oracle sum representable", "addition high denominator positive",
    "addition global fraction maximum", "addition high numerator bound",
)
ALL_COVERS = (
    "addition zero sum", "addition quarter plus zero", "addition zero plus quarter",
    "addition two quarters", "addition exact halves", "addition quarter plus three quarters",
    "addition three quarters plus quarter", "addition three quarters plus half",
    "addition half plus three quarters", "addition unreduced halves", "addition just below unit",
    "addition just above unit", "addition maximum numerator sum", "addition u128 edge left",
    "addition u128 edge right", "addition high denominator left", "addition high denominator right",
    "addition minimum positive plus zero", "addition zero plus minimum positive",
    "addition two minimum positives", "addition raw zeros canonicalize", "addition half plus quarter",
    "addition quarter plus half",
)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _original(source: Path):
    """Load only the pinned preparation dependency, with bytecode disabled."""
    if not sys.dont_write_bytecode:
        raise ValueError("Query-split project imports require bytecode suppression")
    folder = source / "tools/formal"
    for name, expected in PRESERVED_SHA256.items():
        if digest((folder / name).read_bytes()) != expected:
            raise ValueError("Original closed composition kit changed: " + name)
    path = folder / "fraction_composition_projection.py"
    spec = importlib.util.spec_from_file_location("query_split_original_composition", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def schedule() -> list[dict]:
    original = _original(Path(__file__).resolve().parents[2]).schedule()
    result = original[:13]
    universal = {**original[13], "id": "addition_universal_healthy",
                 "subject": UNIVERSAL_SUBJECT, "harness": UNIVERSAL_HARNESS}
    result.append(universal)
    result.extend(original[14:17])
    result.append({**universal, "id": "addition_universal_restored", "restored_healthy": True})
    witnesses = {**original[13], "id": "addition_witnesses_healthy", "subject": WITNESS_SUBJECT,
                 "harness": WITNESS_HARNESS, "role": "witness", "dependencies": [],
                 "expected_stubs": [], "stub_pairs": [], "required_refinement_assertions": [],
                 "extra_assertions": []}
    result.append(witnesses)
    result.append({**witnesses, "id": "addition_witnesses_restored", "restored_healthy": True})
    return result


jobs = schedule


def generated_input_names() -> tuple[str, ...]:
    return ("proof.rs", "fraction_composition_selected.rs", "fraction_composition_spec.rs", "correspondence.json")


def input_names() -> tuple[str, ...]:
    root = Path(__file__).resolve().parents[2]
    names = set(_original(root).input_names())
    names.update("tools/formal/" + name for name in PRESERVED_SHA256)
    names.update(("tools/formal/fraction_query_split_projection.py", "tools/formal/test_fraction_query_split.py"))
    return tuple(sorted(names))


source_input_names = input_names


def expected_contract(subject: str, harness: str) -> tuple[frozenset[str], frozenset[str]]:
    if (subject, harness) == (UNIVERSAL_SUBJECT, UNIVERSAL_HARNESS):
        return frozenset(ALL_ASSERTIONS), frozenset(("addition zero sum",))
    if (subject, harness) == (WITNESS_SUBJECT, WITNESS_HARNESS):
        return frozenset(ALL_ASSERTIONS), frozenset(ALL_COVERS)
    raise ValueError("Unknown query-split healthy proof contract")


def original_query(harness: bytes) -> tuple[bytes, tuple[dict, ...]]:
    """Extract pinned whole function and every exact tuple/answer cover block."""
    if digest(harness) != ORIGINAL_ARITHMETIC_HARNESS_SHA256:
        raise ValueError("Original addition harness changed")
    start = harness.index(b"fn fraction_addition_full_domain() {")
    end = harness.index(b"\n}\n", start) + 3
    function = harness[start:end]
    if digest(function) != ORIGINAL_FULL_FUNCTION_SHA256:
        raise ValueError("Original full-domain addition query changed")
    blocks = re.findall(rb"    kani::cover!\(\n.*?    \);\n", function, re.DOTALL)
    if len(blocks) != 23:
        raise ValueError("Original witness blocks are incomplete")
    witnesses = []
    for block in blocks:
        match = re.fullmatch(rb'    kani::cover!\(\n        inputs == (\([^\n]+\)) && observed == Some\((\((?:true|false), (?:true|false)\))\),\n        "([^"\n]+)"\n    \);\n', block)
        if match is None:
            raise ValueError("Original witness conjunction changed shape")
        inputs, answer, label = match.groups()
        witnesses.append({"tuple_source": inputs.decode(), "answer_source": answer.decode(),
                          "label": label.decode(), "block": block, "sha256": digest(block)})
    if tuple(item["label"] for item in witnesses) != ALL_COVERS:
        raise ValueError("Original witness inventory changed")
    return function, tuple(witnesses)


def universal_function(function: bytes, witnesses: tuple[dict, ...]) -> bytes:
    """Delete exactly 22 original cover blocks and rename only the selected fn."""
    result = function.replace(b"fn fraction_addition_full_domain()", b"fn fraction_addition_universal_full_domain()", 1)
    for item in witnesses:
        if item["label"] != "addition zero sum":
            if result.count(item["block"]) != 1:
                raise ValueError("Universal cover removal is ambiguous")
            result = result.replace(item["block"], b"", 1)
    return result


def witness_function(witnesses: tuple[dict, ...]) -> bytes:
    """Unroll complete original conjunctions around actual constant-input calls."""
    result = b"fn fraction_addition_witnesses_full_domain() {\n"
    for item in witnesses:
        result += (b"    {\n        let (n, s, m, t) = " + item["tuple_source"].encode()
                   + b";\n        let inputs = (n, s, m, t);\n"
                   b"        let observed = addition_contract(n, s, m, t);\n"
                   + item["block"] + b"    }\n")
    return result + b"}\n"


def prepare(source: Path, inputs: Path, job: dict) -> dict:
    """Prepare one fixed fresh job; never mutate old source or prior evidence."""
    scheduled = {item["id"]: item for item in schedule()}
    if not isinstance(job, dict) or job != scheduled.get(job.get("id")):
        raise ValueError("Unknown or changed fixed query-split job")
    original = _original(source)
    old_jobs = {item["id"]: item for item in original.schedule()}
    if job["subject"] not in {UNIVERSAL_SUBJECT, WITNESS_SUBJECT}:
        # Preserve both job declarations and all four payload bytes for 1–13/15–17.
        if job != old_jobs.get(job["id"]):
            raise ValueError("Original prefix or fault job changed")
        return original.prepare(source, inputs, job)

    raw_harness = (source / "tools/formal/fraction_arithmetic_harness.rs").read_bytes()
    function, witnesses = original_query(raw_harness)
    lane = "universal" if job["subject"] == UNIVERSAL_SUBJECT else "witnesses"
    transformed = universal_function(function, witnesses) if lane == "universal" else witness_function(witnesses)
    # The sole base is the canonical original healthy addition job, also for both
    # restorations. Its job ID/flag never leaks into generated restored identities.
    prepared = original.prepare(source, inputs, old_jobs["addition_healthy"])
    proof_path = inputs / "proof.rs"
    proof = proof_path.read_bytes()
    if proof.count(function) != 1:
        raise ValueError("Original generated addition query is not unique")
    proof = proof.replace(function, transformed, 1)
    if lane == "witnesses":
        for pair in old_jobs["addition_healthy"]["expected_stubs"]:
            declaration = ("#[kani::stub(" + pair["original"] + ", " + pair["replacement"] + ")]\n").encode()
            if proof.count(declaration) != 1:
                raise ValueError("Direct witness stub removal is ambiguous")
            proof = proof.replace(declaration, b"", 1)
        if b"#[kani::stub" in proof:
            raise ValueError("Direct-source witness contains a replacement declaration")

    details = json.loads((inputs / "correspondence.json").read_text())
    details.update(harness=job["harness"], subject=job["subject"], case=job["case"],
                   expected_stubs=job["expected_stubs"], stub_pairs=job["expected_stubs"],
                   required_refinement_assertions=job["required_refinement_assertions"],
                   assumptions=prepared["assumptions"] if lane == "universal" else [],
                   scope="full raw-domain universal addition and separate direct-source original witnesses; whole matrix required")
    details["query_split"] = {
        "lane": lane, "projection_sha256": digest(Path(__file__).read_bytes()),
        "original_harness_sha256": digest(raw_harness), "original_full_function_sha256": digest(function),
        "transformed_function_sha256": digest(transformed), "canonical_base_job": "addition_healthy",
        "witness_ledger": [{key: value for key, value in item.items() if key != "block"} for item in witnesses],
        "retained_cover_labels": ["addition zero sum"] if lane == "universal" else list(ALL_COVERS),
        "universal_domain": "all raw u64/u32/u64/u32 without additional assumptions",
        "witness_actual_callgraph": lane == "witnesses",
    }
    payload = {name: (inputs / name).read_bytes() for name in generated_input_names() if name != "correspondence.json"}
    payload["proof.rs"] = proof
    details["input_files_sha256"] = {name: digest(content) for name, content in payload.items()}
    payload["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    for name, content in payload.items():
        (inputs / name).write_bytes(content)
    return {**details, "entrypoint": proof_path, "job": job,
            "generated_input_files": list(generated_input_names()),
            "input_files_sha256": {name: digest(content) for name, content in payload.items()}}
