"""Closed preparation for the source-bound pure-fraction composition card.

This module starts no compiler/solver and grants no callee certificate. Ordinary
stub metadata are source tokens, not resolved identities; activation controls and
the independently reviewed pure-call rule remain explicit assurance obligations.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path


PREPARATION_STARTED_UNIX_NS = 1793918875353739445
PREPARATION_STARTED_MONOTONIC_NS = 15543187166373
PREPARATION_DEADLINE_MONOTONIC_NS = 19143187166373
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
FIXED_OUTPUT_LEAF = "formal-core-fraction-composition-20261106-v1"
SUPPORT_SHA256 = {
    "fraction_arithmetic_projection.py": "9092cf40b90f60971b835bf573070fb2faa10e6cc9826f485cb6e6919da80046",
    "fraction_arithmetic_harness.rs": "d640170cff40d9bebafa7318e4501c2db4f98382a523050d6bbb07c9a2319cea",
    "fraction_constructor_projection.py": "c1a4d5bbf21fad7bd41044d2b1c071035eaf6f4649293ceef42b3675abaf397a",
    "fraction_constructor_harness.rs": "80d5c9c4ed2bdc6cc6dc17e06562e720285734f8905e456894ca00a28d63c6b2",
    "fraction_comparison_projection.py": "70666d777b0d275e63e35b7b22d8b6e4e0ae79678c1db44ba5526882d5e24997",
    "fraction_comparison_harness.rs": "34af14795b940f68abd56a03fcd00565cd174f9a59808b4b654b9a4091751254",
}
MECHANISM_SUBJECT = "fraction_composition_mechanism"
MECHANISM_HARNESS = "fraction_arithmetic::composition_mechanism_full_domain"
PRECONDITIONS = {
    "comparison": "composition comparison checked precondition",
    "complement": "composition complement checked precondition",
}
REFINEMENT_ASSERTIONS = {
    key: (f"composition {key} actual postcondition", f"composition {key} actual return seed")
    for key in ("constructor", "comparison", "complement")
}
MECHANISM_ASSERTIONS = (
    "composition constructor replacement active", "composition comparison replacement active",
    "composition complement replacement active", "composition complete fraction result codec",
    "composition complete native Ordering result codec", "composition constructor total predicate relation",
    "composition comparison total predicate relation", "composition complement total predicate relation",
    "composition real model valid neighbors", *PRECONDITIONS.values(),
)
MECHANISM_COVERS = (
    "composition all three replacements reached",
    *("composition codec fraction " + leaf for leaf in
      ("Ok", "Unrepresentable", "Invariant", "ZeroLabel", "ReversedBounds", "ValidationInvariant")),
    *("composition codec Ordering " + leaf for leaf in
      ("Less", "Equal", "Greater", "Unrepresentable", "Invariant", "ZeroLabel", "ReversedBounds", "ValidationInvariant")),
    *(f"composition raw {subject} Post {value}" for subject in
      ("constructor", "comparison", "complement") for value in ("true", "false")),
    "composition real model neighbors reached",
)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _json_digest(value) -> str:
    return digest(json.dumps(value, sort_keys=True, separators=(",", ":")).encode())


def _load(path: Path):
    spec = importlib.util.spec_from_file_location("composition_" + path.stem, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _support(source: Path):
    folder = source / "tools/formal"
    for name, identity in SUPPORT_SHA256.items():
        if digest((folder / name).read_bytes()) != identity:
            raise ValueError("Reviewed composition support changed: " + name)
    return (folder, _load(folder / "fraction_arithmetic_projection.py"),
            _load(folder / "fraction_constructor_projection.py"),
            _load(folder / "fraction_comparison_projection.py"))


def expected_contract(subject: str, harness: str) -> tuple[frozenset[str], frozenset[str]]:
    if (subject, harness) != (MECHANISM_SUBJECT, MECHANISM_HARNESS):
        raise ValueError("Unknown composition mechanism contract")
    return frozenset(MECHANISM_ASSERTIONS), frozenset(MECHANISM_COVERS)


def _pairs(*keys: str, activation=False) -> list[dict]:
    methods = {"constructor": "new", "comparison": "compare", "complement": "complement"}
    models = {"constructor": "new", "comparison": "compare", "complement": "complement"}
    return [{"original": "crate::fraction_arithmetic::Fraction::" + methods[key],
             "replacement": "crate::fraction_arithmetic::composition_"
             + ("activation_" if activation else "model_") + models[key]} for key in keys]


def schedule() -> list[dict]:
    """The sole eighteen jobs; controls issue no certificates and never retry."""
    result = []

    def add(identity, subject, case, harness, *, role, failure=None, key=None,
            pairs=(), extra=(), restored=False):
        dependencies = ["constructor", "comparison", "complement"] if role in {"precondition", "addition"} else []
        result.append({"id": identity, "subject": subject, "case": case, "harness": harness,
                       "intended_failure": failure, "restored_healthy": restored,
                       "role": role, "dependencies": dependencies, "certificate_key": key,
                       "issues_certificate": key, "expected_stubs": list(pairs),
                       "stub_pairs": list(pairs), "required_refinement_assertions": list(extra),
                       "extra_assertions": list(extra), "expected_kind": "Proof"})

    all_keys = ("constructor", "comparison", "complement")
    add("mech_healthy", MECHANISM_SUBJECT, "healthy", MECHANISM_HARNESS, role="mechanism",
        pairs=_pairs(*all_keys, activation=True))
    add("mech_assert_then_assume", MECHANISM_SUBJECT, "assert_then_assume",
        "fraction_arithmetic::composition_assert_then_assume", role="mechanism",
        failure="composition earlier assertion survives false assumption")
    full = {
        "constructor": ("fraction_constructor", "fraction_constructor::fraction_constructor_full_domain"),
        "comparison": ("fraction_comparison", "fraction_comparison::fraction_comparison_full_domain"),
        "complement": ("fraction_complement", "fraction_arithmetic::fraction_complement_full_domain"),
    }
    for key, (subject, harness) in full.items():
        for case, label in zip(("bad_post", "wrong_seed"), REFINEMENT_ASSERTIONS[key]):
            add(f"{key}_{case}", subject, case,
                f"fraction_arithmetic::composition_{key}_{case}", role="callee", failure=label)
        add(key, subject, "healthy", harness, role="callee", key=key,
            extra=REFINEMENT_ASSERTIONS[key])
    for key in ("comparison", "complement"):
        add(key + "_bad_precondition", "fraction_" + key, "bad_precondition",
            f"fraction_arithmetic::composition_{key}_bad_precondition", role="precondition",
            failure=PRECONDITIONS[key], pairs=_pairs(key))
    add("addition_healthy", "fraction_addition", "healthy", "fraction_arithmetic::fraction_addition_full_domain",
        role="addition", pairs=_pairs(*all_keys), extra=PRECONDITIONS.values())
    for identity, case, suffix, failure in (
            ("addition_reject_all", "always_error", "success", "addition required success"),
            ("addition_wrong_carry", "wrong_carry", "carry", "addition exact carry"),
            ("addition_wrong_residual", "wrong_residual", "residual", "addition exact residual")):
        add(identity, "fraction_addition", case, "fraction_arithmetic::fraction_addition_control_" + suffix,
            role="addition", failure=failure, pairs=_pairs(*all_keys))
    add("addition_restored", "fraction_addition", "healthy", "fraction_arithmetic::fraction_addition_full_domain",
        role="addition", pairs=_pairs(*all_keys), extra=PRECONDITIONS.values(), restored=True)
    return result


jobs = schedule


def generated_input_names() -> tuple[str, ...]:
    return ("proof.rs", "fraction_composition_selected.rs", "fraction_composition_spec.rs", "correspondence.json")


def input_names() -> tuple[str, ...]:
    """Repository-relative source closure, distinct from generated leaf names."""
    return tuple("tools/formal/" + name for name in (
        *SUPPORT_SHA256, "fraction_composition_projection.py", "fraction_composition_spec.rs",
        "fraction_composition_harness.rs", "test_fraction_composition.py"))


source_input_names = input_names


def _once(data: bytes, before: bytes, after: bytes) -> bytes:
    if data.count(before) != 1:
        raise ValueError("Composition rewrite is not unique: " + before.decode())
    return data.replace(before, after, 1)


def _assertions(key: str, parameters: str) -> bytes:
    post, seed = REFINEMENT_ASSERTIONS[key]
    if key == "comparison":
        predicate = f"composition_comparison_post({parameters}, &observed)"
        equal = "composition_order_result_equal(observed, composition_decode_order(composition_encode_order(observed)))"
    else:
        predicate = f"composition_{key}_post({parameters}, &observed)"
        equal = "composition_fraction_result_equal(observed, composition_decode_fraction(composition_encode_fraction(observed)))"
    return (f'\n    assert!({predicate}, "{post}");\n'
            f'    assert!({equal}, "{seed}");').encode()


def _original_harness(folder: Path, key: str) -> bytes:
    filename = {"constructor": "fraction_constructor_harness.rs",
                "comparison": "fraction_comparison_harness.rs", "complement": "fraction_arithmetic_harness.rs"}[key]
    body = (folder / filename).read_bytes()
    if key == "constructor":
        before = b"    let observed = Fraction::new(numerator, shift);"
        return _once(body, before, before + _assertions(key, "numerator, shift"))
    if key == "comparison":
        before = b"            let observed = left.compare(right);"
        return _once(body, before, before + _assertions(key, "left, right"))
    before = b"    let observed = input.complement();"
    return _once(body, before, before + _assertions(key, "input"))


def prepare(source: Path, inputs: Path, job: dict) -> dict:
    """Seal exact private source/model inputs without overwriting any custody."""
    scheduled = {item["id"]: item for item in schedule()}
    if not isinstance(job, dict) or job != scheduled.get(job.get("id")):
        raise ValueError("Unknown or changed fixed composition job")
    folder, arithmetic, constructor, comparison = _support(source)
    originals = {name: (source / "crates/time-core/src" / name).read_bytes()
                 for name in arithmetic.SOURCE_SHA256}
    for name, identity in arithmetic.SOURCE_SHA256.items():
        if digest(originals[name]) != identity:
            raise ValueError("Reviewed complete composition source changed: " + name)
    selected = {name: arithmetic._extract(originals, name) for name in arithmetic.SPANS}
    compiled = dict(selected)
    spec = (folder / "fraction_composition_spec.rs").read_bytes()
    controls = (folder / "fraction_composition_harness.rs").read_bytes()
    mutation = None
    if job["role"] == "callee" and job["case"] == "bad_post":
        key = job["id"].removesuffix("_bad_post")
        mutation = dict({"constructor": constructor.CONTROLS["wrong_numerator"],
                         "comparison": comparison.CONTROLS["reverse_unequal"],
                         "complement": arithmetic.CONTROLS["fraction_complement"]["wrong_numerator"]}[key])
        method = {"constructor": "new", "comparison": "compare", "complement": "complement"}[key]
        compiled[method] = _once(compiled[method], mutation["before"].encode(), mutation["after"].encode())
    elif job["role"] == "callee" and job["case"] == "wrong_seed":
        key = job["id"].removesuffix("_wrong_seed")
        before, after = (b"Ok(Ordering::Less) => 0,", b"Ok(Ordering::Less) => 2,") if key == "comparison" else (
            b"numerator: value.numerator,",
            b"numerator: value.numerator ^ 1,")
        spec = _once(spec, before, after)
        mutation = {"before": before.decode(), "after": after.decode(), "owner": "proof encoder"}
    elif job["role"] == "addition" and job["case"] != "healthy":
        mutation = dict(arithmetic.CONTROLS["fraction_addition"][job["case"]])
        compiled["addition"] = _once(compiled["addition"], mutation["before"].encode(), mutation["after"].encode())

    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\nuse std::cmp::Ordering;\n"
                   + compiled["interval_error"] + b"\n" + compiled["enclosure_error"] + b"\n"
                   + compiled["fraction"] + b"\nimpl Fraction {\n" + compiled["zero"]
                   + b"\n" + compiled["new"] + b"\n" + compiled["compare"]
                   + b"\n" + compiled["complement"] + b"}\n" + compiled["fraction_sum"]
                   + b"\n" + compiled["addition"])
    key = job["certificate_key"]
    if key:
        original = _original_harness(folder, key)
    elif job["role"] == "addition":
        original = (folder / "fraction_arithmetic_harness.rs").read_bytes()
    else:
        original = b""
    # The historical no-assumptions comment describes the ordinary old mode only.
    original = original.replace(b"// No Kani assumptions, callee stubs or unchecked private field assembly.\n", b"")
    module = job["harness"].split("::", 1)[0]
    harness = original + b"\n" + controls
    local_name = job["harness"].split("::", 1)[1]
    declaration = ("#[kani::proof]\nfn " + local_name + "(").encode()
    attributes = b"".join(("#[kani::stub(" + pair["original"] + ", "
                            + pair["replacement"] + ")]\n").encode() for pair in job["expected_stubs"])
    harness = _once(harness, declaration, b"#[kani::proof]\n" + attributes + ("fn " + local_name + "(").encode())
    proof = (b"#![allow(dead_code)]\nmod " + module.encode() + b" {\n"
             b'include!("fraction_composition_selected.rs");\n' + spec + b"\n" + harness + b"\n}\n")

    types = {name: digest(selected[name]) for name in ("fraction", "zero", "enclosure_error", "interval_error")}
    mapping = _pairs("constructor", "comparison", "complement")
    closures = {}
    for callee, body in (("constructor", "new"), ("comparison", "compare"), ("complement", "complement")):
        source_dependencies = {name: digest(compiled[name]) for name in
                               {"constructor": ("new",), "comparison": ("new", "compare"),
                                "complement": ("new", "compare", "complement")}[callee]}
        closures[callee] = {"body": digest(compiled[body]), "actual_source_dependencies": source_dependencies,
                           "types": types, "complete_original_sources": dict(arithmetic.SOURCE_SHA256),
                           "post_and_primitive_codec": digest(spec), "models_and_controls": digest(controls),
                           "source_projection": digest(Path(__file__).read_bytes()),
                           "original_harness": digest(_original_harness(folder, callee)),
                           "mapping": mapping, "domain": {"constructor": "all raw u64/u32",
                           "comparison": "all pairs of canonical checked Fraction",
                           "complement": "all canonical checked Fraction including rejection regions"}[callee]}
    bindings = {name: _json_digest(closure) for name, closure in closures.items()}
    payload = {"proof.rs": proof, "fraction_composition_selected.rs": definitions,
               "fraction_composition_spec.rs": spec}
    details = {"original_sources_sha256": dict(arithmetic.SOURCE_SHA256), "support_sha256": dict(SUPPORT_SHA256),
               "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                                   "byte_offsets_half_open": list(span["offsets"]), "sha256": digest(selected[name])}
                                  for name, span in arithmetic.SPANS.items()],
               "compiled_span_sha256": {name: digest(value) for name, value in compiled.items()},
               "expected_stubs": job["expected_stubs"], "stub_pairs": job["expected_stubs"],
               "stub_metadata_is_source_tokens_not_resolved_ids": True,
               "required_refinement_assertions": job["required_refinement_assertions"],
               "callee_bindings": bindings, "certificate_identity": bindings,
               "callee_binding_components": closures, "mutation": mutation,
               "assumptions": ["callee Post only; exact preconditions asserted before each assumption"]
               if job["expected_stubs"] or job["role"] == "mechanism" else [],
               "harness": job["harness"], "subject": job["subject"], "case": job["case"],
               "input_files_sha256": {name: digest(content) for name, content in payload.items()},
               "scope": "pure fraction source composition only; no full enclosure/Core/P1/native qualification"}
    payload["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    if inputs.is_symlink() or any((inputs / name).exists() or (inputs / name).is_symlink() for name in payload):
        raise ValueError("Refusing to overwrite composition input custody")
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name, content in payload.items():
        (inputs / name).write_bytes(content)
    return {**details, "entrypoint": inputs / "proof.rs", "job": job,
            "generated_input_files": list(generated_input_names()),
            "input_files_sha256": {name: digest(content) for name, content in payload.items()}}
