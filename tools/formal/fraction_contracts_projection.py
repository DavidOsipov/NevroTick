"""Prepare the closed eleven-job verified-callee experiment without execution.

Only contract attributes and proof-only specifications/generators are added to
the exact selected product owners. Comparison always remains inline. Callee
bindings deliberately exclude the caller-only addition mutation.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path


SUPPORT_SHA256 = {
    "fraction_arithmetic_projection.py": "9092cf40b90f60971b835bf573070fb2faa10e6cc9826f485cb6e6919da80046",
    "fraction_arithmetic_harness.rs": "d640170cff40d9bebafa7318e4501c2db4f98382a523050d6bbb07c9a2319cea",
    "fraction_constructor_projection.py": "c1a4d5bbf21fad7bd41044d2b1c071035eaf6f4649293ceef42b3675abaf397a",
    "fraction_constructor_harness.rs": "80d5c9c4ed2bdc6cc6dc17e06562e720285734f8905e456894ca00a28d63c6b2",
}
NEW_ENSURES = ("|result: &Result<Self, FixtureEnclosureError>| "
               "constructor_post(numerator, shift, result)")
COMPLEMENT_ENSURES = ("|result: &Result<Self, FixtureEnclosureError>| "
                      "complement_post(self, result)")
COMPLEMENT_REQUIRES = "checked_fraction(self)"
NEW_ATTRIBUTE = (
    b"    #[kani::ensures(|result: &Result<Self, FixtureEnclosureError>| "
    b"constructor_post(numerator, shift, result))]\n"
)
COMPLEMENT_ATTRIBUTES = (
    b"    #[kani::requires(checked_fraction(self))]\n"
    b"    #[kani::ensures(|result: &Result<Self, FixtureEnclosureError>| "
    b"complement_post(self, result))]\n"
)


def digest(data: bytes) -> str:
    """Identify exact bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def _json_digest(value: dict) -> str:
    return digest(json.dumps(value, sort_keys=True, separators=(",", ":")).encode())


def _load(path: Path):
    spec = importlib.util.spec_from_file_location("fraction_contracts_" + path.stem, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _support(source_root: Path):
    folder = source_root / "tools/formal"
    for name, expected in SUPPORT_SHA256.items():
        if digest((folder / name).read_bytes()) != expected:
            raise ValueError("Reviewed contract support changed: " + name)
    return folder, _load(folder / "fraction_arithmetic_projection.py"), _load(
        folder / "fraction_constructor_projection.py")


def jobs() -> list[dict]:
    """Return the sole finite schedule; no retries or alternative selectors."""
    # Read only the already pinned preparation modules for their original labels.
    folder, arithmetic, constructor = _support(Path(__file__).resolve().parents[2])
    del folder
    result = []

    def add(identity, subject, case, harness, *, kind="Proof", stubs=(), dependencies=(),
            assertions=(), covers=(), failure=None, contract=(), preconditions=()):
        result.append({"id": identity, "subject": subject, "case": case,
                       "harness": "fraction_contracts::" + harness,
                       "expected_kind": kind, "verified_stubs": list(stubs),
                       "dependencies": list(dependencies),
                       "expected_assertions": list(assertions), "expected_covers": list(covers),
                       "intended_failure": failure,
                       "required_contract_assertions": list(contract),
                       "required_precondition_assertions": list(preconditions)})

    primitive_kind = 'ProofForContract { target_fn: "primitive_next" }'
    add("mechanism_callee", "mechanism", "healthy", "primitive_checker",
        kind=primitive_kind,
        covers=("mechanism callee admitted boundary",), contract=("|result: &u8| *result > value",))
    add("mechanism_caller", "mechanism", "healthy", "primitive_caller",
        stubs=("primitive_next",), dependencies=("mechanism_callee",),
        assertions=("mechanism verified caller",), covers=("mechanism caller reached verified return",),
        preconditions=("value < u8::MAX",))
    add("mechanism_precondition", "mechanism", "bad_precondition", "primitive_bad_precondition",
        stubs=("primitive_next",), dependencies=("mechanism_callee",),
        failure="value < u8::MAX")
    add("mechanism_bad_callee", "mechanism", "bad_callee", "primitive_checker",
        kind=primitive_kind, failure="|result: &u8| *result > value",
        contract=("|result: &u8| *result > value",))
    constructor_harness = constructor.FULL_HARNESS
    add("constructor", "constructor", "healthy", "fraction_constructor_full_domain",
        kind='ProofForContract { target_fn: "Fraction::new" }',
        assertions=constructor.REQUIRED_ASSERTIONS[constructor_harness],
        covers=constructor.REQUIRED_COVERS[constructor_harness], contract=(NEW_ENSURES,))
    complement_harness = "fraction_arithmetic::fraction_complement_full_domain"
    add("complement", "complement", "healthy", "fraction_complement_full_domain",
        kind='ProofForContract { target_fn: "Fraction::complement" }',
        stubs=("Fraction::new",), dependencies=("constructor",),
        assertions=arithmetic.REQUIRED_ASSERTIONS[complement_harness],
        covers=arithmetic.REQUIRED_COVERS[complement_harness], contract=(COMPLEMENT_ENSURES,))
    addition_harness = "fraction_arithmetic::fraction_addition_full_domain"
    common = {"stubs": ("Fraction::new", "Fraction::complement"),
              "dependencies": ("constructor", "complement")}
    add("addition_healthy", "addition", "healthy", "fraction_addition_full_domain", **common,
        assertions=arithmetic.REQUIRED_ASSERTIONS[addition_harness],
        covers=arithmetic.REQUIRED_COVERS[addition_harness],
        preconditions=(COMPLEMENT_REQUIRES,))
    for identity, case in (("addition_reject_all", "always_error"),
                           ("addition_wrong_carry", "wrong_carry"),
                           ("addition_wrong_residual", "wrong_residual")):
        control = arithmetic.CONTROLS["fraction_addition"][case]
        add(identity, "addition", case, control["harness"].split("::", 1)[1], **common,
            failure=control["intended_failure"])
    add("addition_restored", "addition", "healthy", "fraction_addition_full_domain", **common,
        assertions=arithmetic.REQUIRED_ASSERTIONS[addition_harness],
        covers=arithmetic.REQUIRED_COVERS[addition_harness], preconditions=(COMPLEMENT_REQUIRES,))
    return result


def _primitive(case: str) -> bytes:
    body = b"value" if case == "bad_callee" else b"value + 1"
    checker_cover = (b"" if case == "bad_callee" else
                     b'    kani::cover!(value == 254 && observed == 255, "mechanism callee admitted boundary");\n')
    return (b"#[kani::requires(value < u8::MAX)]\n"
            b"#[kani::ensures(|result: &u8| *result > value)]\n"
            b"fn primitive_next(value: u8) -> u8 { " + body + b" }\n"
            b"#[kani::proof_for_contract(primitive_next)]\n"
            b"fn primitive_checker() {\n"
            b"    let value = kani::any::<u8>();\n"
            b"    let observed = primitive_next(value);\n" + checker_cover
            + b"}\n#[kani::proof]\n#[kani::stub_verified(primitive_next)]\n"
            b"fn primitive_caller() {\n"
            b"    let observed = primitive_next(7);\n"
            b'    assert!(observed > 7, "mechanism verified caller");\n'
            b'    kani::cover!(observed > 7, "mechanism caller reached verified return");\n'
            b"}\n#[kani::proof]\n#[kani::stub_verified(primitive_next)]\n"
            b"fn primitive_bad_precondition() { let _value = primitive_next(u8::MAX); }\n")


def _harnesses(folder: Path) -> tuple[bytes, bytes]:
    constructor = (folder / "fraction_constructor_harness.rs").read_bytes()
    constructor = constructor[:constructor.index(b"/// Reject-all loses")]
    constructor = constructor.replace(b"#[kani::proof]\nfn fraction_constructor_full_domain",
                                      b"#[kani::proof_for_contract(Fraction::new)]\n"
                                      b"fn fraction_constructor_full_domain", 1)
    arithmetic = (folder / "fraction_arithmetic_harness.rs").read_bytes()
    # The previous no-assumptions/no-stubs comment does not describe this mode.
    arithmetic = arithmetic[arithmetic.index(b"const HALF_NUMERATOR:"):]
    arithmetic = arithmetic.replace(b"#[kani::proof]\nfn fraction_complement_full_domain",
                                    b"#[kani::proof_for_contract(Fraction::complement)]\n"
                                    b"#[kani::stub_verified(Fraction::new)]\n"
                                    b"fn fraction_complement_full_domain", 1)
    for name in ("fraction_addition_full_domain", "fraction_addition_control_success",
                 "fraction_addition_control_carry", "fraction_addition_control_residual"):
        before = ("#[kani::proof]\nfn " + name).encode()
        if arithmetic.count(before) != 1:
            raise ValueError("Original addition harness is not unique: " + name)
        arithmetic = arithmetic.replace(before, b"#[kani::proof]\n"
                                        b"#[kani::stub_verified(Fraction::new)]\n"
                                        b"#[kani::stub_verified(Fraction::complement)]\nfn "
                                        + name.encode(), 1)
    return constructor, arithmetic


def prepare(source_root: Path, inputs: Path, job: dict) -> dict:
    """Seal one exact fixed job, refusing drift and preexisting output custody."""
    scheduled = {item["id"]: item for item in jobs()}
    expected = scheduled.get(job.get("id"))
    if expected is None or any(job.get(key) != value for key, value in expected.items()):
        raise ValueError("Unknown or changed fixed fraction-contract job")
    folder, arithmetic, _constructor = _support(source_root)
    originals = {name: (source_root / "crates/time-core/src" / name).read_bytes()
                 for name in arithmetic.SOURCE_SHA256}
    for name, expected_hash in arithmetic.SOURCE_SHA256.items():
        if digest(originals[name]) != expected_hash:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: arithmetic._extract(originals, name) for name in arithmetic.SPANS}
    spec = (folder / "fraction_contracts_spec.rs").read_bytes()
    constructor_harness, arithmetic_harness = _harnesses(folder)
    types = {name: digest(selected[name]) for name in
             ("fraction", "zero", "interval_error", "enclosure_error")}
    constructor_closure = {"target": "Fraction::new", "body": digest(selected["new"]),
                           "annotation": digest(NEW_ATTRIBUTE), "types": types,
                           "spec_and_generators": digest(spec),
                           "checker": digest(constructor_harness)}
    complement_closure = {"target": "Fraction::complement",
                          "body": digest(selected["complement"]),
                          "annotation": digest(COMPLEMENT_ATTRIBUTES), "types": types,
                          "spec_and_generators": digest(spec),
                          "compare_inline": digest(selected["compare"]),
                          "constructor_dependency": _json_digest(constructor_closure),
                          "checker_and_original_math": digest(arithmetic_harness)}
    bindings = {"mechanism_callee": digest(_primitive(job["case"])),
                "constructor": _json_digest(constructor_closure),
                "complement": _json_digest(complement_closure)}
    compiled = dict(selected)
    mutation = None
    if job["subject"] == "addition" and job["case"] != "healthy":
        mutation = dict(arithmetic.CONTROLS["fraction_addition"][job["case"]])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if compiled["addition"].count(before) != 1:
            raise ValueError("Reviewed addition mutation is not unique")
        compiled["addition"] = compiled["addition"].replace(before, after, 1)
    for name, signature, attributes in (
            ("new", b"    fn new(", NEW_ATTRIBUTE),
            ("complement", b"    fn complement(", COMPLEMENT_ATTRIBUTES)):
        if compiled[name].count(signature) != 1:
            raise ValueError("Reviewed annotated method is not unique: " + name)
        compiled[name] = compiled[name].replace(signature, attributes + signature, 1)
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\nuse std::cmp::Ordering;\n"
                   + compiled["interval_error"] + b"\n" + compiled["enclosure_error"] + b"\n"
                   + compiled["fraction"] + b"\nimpl Fraction {\n" + compiled["zero"]
                   + b"\n" + compiled["new"] + b"\n" + compiled["compare"]
                   + b"\n" + compiled["complement"] + b"}\n" + compiled["fraction_sum"]
                   + b"\n" + compiled["addition"])
    if job["subject"] == "mechanism":
        proof = b"#![allow(dead_code)]\nmod fraction_contracts {\n" + _primitive(job["case"]) + b"}\n"
        files = {"proof.rs": proof}
    else:
        proof = (b"#![allow(dead_code)]\nmod fraction_contracts {\n"
                 b'include!("fraction_contracts_selected.rs");\n'
                 b'include!("fraction_contracts_spec.rs");\n'
                 + constructor_harness + b"\n" + arithmetic_harness + b"\n}\n")
        files = {"proof.rs": proof, "fraction_contracts_selected.rs": definitions,
                 "fraction_contracts_spec.rs": spec}
    details = {"entrypoint": inputs / "proof.rs", "job": job,
               "original_sources_sha256": dict(arithmetic.SOURCE_SHA256),
               "support_sha256": dict(SUPPORT_SHA256),
               "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                                   "byte_offsets_half_open": list(span["offsets"]),
                                   "sha256": digest(selected[name])}
                                  for name, span in arithmetic.SPANS.items()],
               "callee_bindings": bindings,
               "callee_binding_components": {"constructor": constructor_closure,
                                               "complement": complement_closure},
               "annotations_only_on": ["Fraction::new", "Fraction::complement"],
               "compare_inline_unannotated_sha256": digest(selected["compare"]),
               "approved_callee_requires": {"constructor": [], "complement": [COMPLEMENT_REQUIRES]},
               "contract_postconditions": {"constructor": NEW_ENSURES, "complement": COMPLEMENT_ENSURES},
               "requires_are_assumed_only_in_checking_harness": True,
               "requires_are_asserted_at_verified_stub_calls": True,
               "unrestricted_return_generators": ["Fraction", "FixtureEnclosureError", "IntervalError"],
               "mutation": mutation,
               "input_files_sha256": {name: digest(content) for name, content in files.items()},
               "scope": "checked Fraction addition through complete raw factories; Garde/caller admission and full enclosure remain open"}
    files["correspondence.json"] = (json.dumps({**details, "entrypoint": "proof.rs"},
                                             indent=2, sort_keys=True) + "\n").encode()
    if inputs.is_symlink() or any((inputs / name).exists() or (inputs / name).is_symlink() for name in files):
        raise ValueError("Refusing to overwrite fraction-contract input custody")
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name, content in files.items():
        (inputs / name).write_bytes(content)
    return details
