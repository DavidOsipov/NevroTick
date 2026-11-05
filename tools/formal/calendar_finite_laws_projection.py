"""Prepare the finite Gregorian Rust laws without the separate wide-period proof.

The existing model helper prefix is copied byte for byte from the frozen calendar
composition harness. The new healthy body deletes only the symbolic cycle input
and its arbitrary-cycle assertion. The original complete D theorem is not renamed
as complete. No compiler, solver or product source is changed or executed here.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path


MODEL = "tools/formal/calendar_composition_harness.rs"
MODEL_SHA256 = "5a8ab5f79eabcb619d58c0d902152293cfb7ffe71d39c5cd000b1dfa20ad8b00"
MODEL_PREFIX_BYTES = 4059
MODEL_PREFIX_SHA256 = "b88bd85b68baf3ca59089e77dffbb10c14adf7633cc164969b5c49d2d403e1b6"
SOURCE_PROJECTION = "tools/formal/calendar_composition_projection.py"
SOURCE_PROJECTION_SHA256 = "6a78ee16a929fb00faf45900ea23ab1d8a273c056835987451b571a9770fd766"
FINITE_BODY_SHA256 = "1847cb76361c711befab1c2b714b3d9d0ace28c93e3214a8b9e5a509dc9f479c"
SUBJECT = "calendar_finite_laws"
EXTRA_INPUTS = (
    "tools/formal/calendar_finite_laws_harness.rs",
    "tools/formal/calendar_finite_laws_projection.py", MODEL, SOURCE_PROJECTION,
)
REQUIRED_ASSERTIONS = {
    "calendar_finite_laws_full_domain": (
        "Gregorian segment year recurrence", "Gregorian March month recurrence",
        "March prefix origin", "March segment total length", "Gregorian cycle period",
        "NTP March epoch anchor", "NTP cycle epoch position",
    ),
}
REQUIRED_COVERS = {
    "calendar_finite_laws_full_domain": (
        "laws ordinary leap month", "laws non-400 century March",
        "laws 400-year century March", "laws maximum cycle model",
    ),
}
CONTROLS = {
    "cycle_year_coefficient": {
        "before": "365 * year + year / 4 - year / 100 + year / 400",
        "after": "364 * year + year / 4 - year / 100 + year / 400",
        "harness": "calendar_finite_laws_control_year",
        "intended_failure": "Gregorian segment year recurrence",
    },
    "march_prefix_length": {
        "before": "if month > 0 {\n        days += 31;\n    }",
        "after": "if month > 0 {\n        days += 30;\n    }",
        "harness": "calendar_finite_laws_control_month",
        "intended_failure": "Gregorian March month recurrence",
    },
    "forward_epoch_coefficient": {
        "before": "365 * years + years / 4 - years / 100 + years / 400",
        "after": "364 * years + years / 4 - years / 100 + years / 400",
        "harness": "calendar_finite_laws_control_epoch",
        "intended_failure": "NTP March epoch anchor",
    },
}


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Five commands: finite healthy, three auxiliary-model faults, restored healthy."""
    jobs = [{"subject": SUBJECT, "case": "healthy", "harness": "calendar_finite_laws_full_domain",
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": SUBJECT, "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": SUBJECT, "case": "healthy", "harness": "calendar_finite_laws_full_domain",
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def finite_body(model: bytes) -> bytes:
    """Make only the two named deletions and the distinct healthy harness name."""
    first = b"#[kani::proof]\nfn calendar_gregorian_laws() {"
    last = b"\n#[kani::proof]\nfn calendar_composition_control_success()"
    if model.count(first) != 1 or model.count(last) != 1:
        raise ValueError("Original Gregorian-law body selection is ambiguous")
    begin = model.index(first)
    end = model.index(last, begin)
    original = model[begin:end]
    symbolic = b"    let cycles = kani::any::<u64>();\n"
    assertion = b'''    assert_eq!(
        forward_start(400 * u128::from(cycles) + u128::from(year)),
        u128::from(CYCLE_DAYS) * u128::from(cycles) + u128::from(cycle_start(year)),
        "arbitrary cycle forward law"
    );
'''
    if original.count(symbolic) != 1 or original.count(assertion) != 1:
        raise ValueError("Original symbolic period obligation changed")
    selected = (original.replace(b"fn calendar_gregorian_laws()", b"fn calendar_finite_laws_full_domain()", 1)
                .replace(symbolic, b"", 1).replace(assertion, b"", 1))
    if digest(selected) != FINITE_BODY_SHA256:
        raise ValueError("Finite law body differs from the reviewed two-deletion transformation")
    return selected


def _source_projection(source_root: Path):
    """Load only the separately pinned, frozen existing H binding owner."""
    path = source_root / SOURCE_PROJECTION
    if digest(path.read_bytes()) != SOURCE_PROJECTION_SHA256:
        raise ValueError("Original calendar source-binding projection changed")
    specification = importlib.util.spec_from_file_location("calendar_finite_source_binding", path)
    if specification is None or specification.loader is None:
        raise ValueError("Cannot load the frozen calendar source binding")
    loaded = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(loaded)
    return loaded


def prepare_calendar_finite_laws(source_root: Path, inputs: Path, case: str,
                                harness_path: Path) -> dict:
    """Copy the exact old model and finite body, then reuse healthy actual H.

    The existing preparation owner rejects pre-existing proof inputs before any
    write. This function then replaces only its newly generated proof/metadata
    files with this narrower finite leaf; calendar_selected.rs stays unchanged.
    Faults alter only the auxiliary Gregorian model, not product source or H.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown finite Gregorian-law control")
    model = (source_root / MODEL).read_bytes()
    if digest(model) != MODEL_SHA256 or model.index(b"#[kani::proof]") != MODEL_PREFIX_BYTES:
        raise ValueError("Original calendar model source changed")
    prefix = model[:MODEL_PREFIX_BYTES]
    if digest(prefix) != MODEL_PREFIX_SHA256:
        raise ValueError("Original Gregorian helper prefix changed")
    expected_body = finite_body(model)
    harness = harness_path.read_bytes()
    first = b"#[kani::proof]\nfn calendar_finite_laws_full_domain() {"
    last = b"\n#[kani::proof]\nfn calendar_finite_laws_control_year()"
    if (harness.count(first) != 1 or harness.count(last) != 1
            or harness[harness.index(first):harness.index(last)] != expected_body):
        raise ValueError("New finite harness does not preserve the exact seven remaining laws")
    compiled_prefix = prefix
    mutation = None
    if case != "healthy":
        mutation = {**CONTROLS[case], "scope": "auxiliary Gregorian model copy only; actual H unchanged"}
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if compiled_prefix.count(before) != 1:
            raise ValueError("Finite model control point is not unique")
        compiled_prefix = compiled_prefix.replace(before, after, 1)
    binding = _source_projection(source_root).prepare_calendar_composition(
        source_root, inputs, "healthy", source_root / MODEL)
    proof = (b"#![allow(dead_code)]\n" + compiled_prefix
             + b'\n#[path="calendar_selected.rs"]\nmod calendar;\n' + harness)
    details = {
        "entrypoint": inputs / "proof.rs", "mutation": mutation, "assumptions": [],
        "correspondence": [{"source": MODEL, "selection": "exact auxiliary model helper prefix before first proof",
                            "byte_offsets_half_open": [0, MODEL_PREFIX_BYTES], "sha256": digest(prefix)},
                           {"source": MODEL, "selection": "old D body with only symbolic-cycle input and arbitrary-period assertion removed",
                            "compiled_finite_body_sha256": digest(expected_body)}],
        "original_model_sha256": digest(model), "compiled_model_prefix_sha256": digest(compiled_prefix),
        "new_harness_sha256": digest(harness), "source_projection_sha256": SOURCE_PROJECTION_SHA256,
        "actual_H_structural_binding": binding["structural_binding"],
        "actual_H_correspondence": binding["correspondence"],
        "actual_H_unchanged": binding["mutation"] is None,
        "required_separate_period_leaf": {
            "status": "not established by this finite leaf",
            "property": "for every u64 cycle count c and y in 0..399: F(400*c+y)=146097*c+G(y)",
            "obligations": "exact same F/G definitions and prefix identities; all Rust u128/u32 intermediate bounds and division semantics",
            "original_model_prefix_sha256": MODEL_PREFIX_SHA256,
        },
        "qualification_scope": "seven finite Gregorian laws and four original covers only; neither original D nor full calendar acceptance",
        "omitted": ["symbolic arbitrary-u64 period law and its companion machine-bounds proof",
                    "full-u64 calendar acceptance", "Garde/ledger/native/date-provenance obligations"],
    }
    (inputs / "proof.rs").write_bytes(proof)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
