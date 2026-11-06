"""Source-only custody and semantic controls; no Rust compiler or solver runs."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SPEC = importlib.util.spec_from_file_location("composition_projection_test", HERE / "fraction_composition_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class CompositionPreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-composition-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in (*PROJECTION.input_names(), "crates/time-core/src/projection_enclosure.rs",
                     "crates/time-core/src/selection.rs"):
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.formal = self.source / "tools/formal"
        self.jobs = PROJECTION.schedule()

    def prepare(self, index, folder=None):
        return PROJECTION.prepare(self.source, folder or self.base / str(index), self.jobs[index])

    def test_fixed_eighteen_jobs_and_repository_source_inventory(self):
        self.assertEqual([job["id"] for job in self.jobs], [
            "mech_healthy", "mech_assert_then_assume", "constructor_bad_post", "constructor_wrong_seed",
            "constructor", "comparison_bad_post", "comparison_wrong_seed", "comparison",
            "complement_bad_post", "complement_wrong_seed", "complement",
            "comparison_bad_precondition", "complement_bad_precondition", "addition_healthy",
            "addition_reject_all", "addition_wrong_carry", "addition_wrong_residual", "addition_restored"])
        self.assertEqual([i + 1 for i, job in enumerate(self.jobs) if job["case"] == "healthy"], [1, 5, 8, 11, 14, 18])
        self.assertEqual([job["certificate_key"] for job in self.jobs if job["certificate_key"]],
                         ["constructor", "comparison", "complement"])
        for job in self.jobs:
            self.assertEqual(job["dependencies"], ["constructor", "comparison", "complement"]
                             if job["role"] in {"precondition", "addition"} else [])
        self.assertTrue(all((ROOT / name).is_file() for name in PROJECTION.input_names()))
        self.assertTrue(all(name.startswith("tools/formal/") for name in PROJECTION.input_names()))
        self.assertFalse(set(PROJECTION.input_names()) & set(PROJECTION.generated_input_names()))

    def test_all_nine_spans_exact_except_scheduled_actual_body_defects(self):
        _, arithmetic, _, _ = PROJECTION._support(self.source)
        originals = {name: (self.source / "crates/time-core/src" / name).read_bytes()
                     for name in arithmetic.SOURCE_SHA256}
        selected = {name: arithmetic._extract(originals, name) for name in arithmetic.SPANS}
        for index, job in enumerate(self.jobs):
            prepared = self.prepare(index)
            code = (prepared["entrypoint"].parent / "fraction_composition_selected.rs").read_bytes()
            self.assertEqual(len(prepared["correspondence"]), 9)
            for name, span in selected.items():
                expected = span
                if prepared["mutation"] and job["case"] not in {"wrong_seed", "healthy"}:
                    mutation = prepared["mutation"]
                    before, after = mutation["before"].encode(), mutation["after"].encode()
                    if before in span:
                        expected = span.replace(before, after, 1)
                self.assertEqual(code.count(expected), 1, (index, name))
            self.assertNotIn(b"kani::", code)
            self.assertEqual(set(prepared["input_files_sha256"]), set(PROJECTION.generated_input_names()))

    def test_healthy_callee_harnesses_keep_original_assertions_covers_and_two_refinement_facts(self):
        folder, arithmetic, constructor, comparison = PROJECTION._support(self.source)
        owners = ((4, "constructor", constructor), (7, "comparison", comparison), (10, "complement", arithmetic))
        for index, key, owner in owners:
            job = self.jobs[index]
            prepared = self.prepare(index)
            proof = prepared["entrypoint"].read_bytes()
            original = (folder / {"constructor": "fraction_constructor_harness.rs",
                                  "comparison": "fraction_comparison_harness.rs",
                                  "complement": "fraction_arithmetic_harness.rs"}[key]).read_bytes()
            augmented = PROJECTION._original_harness(folder, key)
            parameters = {"constructor": "numerator, shift", "comparison": "left, right", "complement": "input"}[key]
            self.assertEqual(augmented.replace(PROJECTION._assertions(key, parameters), b""), original)
            for label in (*owner.REQUIRED_ASSERTIONS[job["harness"]], *owner.REQUIRED_COVERS[job["harness"]],
                          *PROJECTION.REFINEMENT_ASSERTIONS[key]):
                self.assertIn(('"' + label + '"').encode(), proof)
            self.assertEqual(job["required_refinement_assertions"], list(PROJECTION.REFINEMENT_ASSERTIONS[key]))
            self.assertNotIn(b"#[kani::stub", proof)
            self.assertIn(b"fn composition_decode_fraction", proof)
            self.assertNotIn(b'include!("fraction_composition_spec.rs")', proof)

    def test_addition_original_body_oracle_labels_witnesses_and_exact_restoration(self):
        first, restored = self.prepare(13), self.prepare(17)
        self.assertEqual(first["input_files_sha256"], restored["input_files_sha256"])
        for name in PROJECTION.generated_input_names():
            self.assertEqual((first["entrypoint"].parent / name).read_bytes(),
                             (restored["entrypoint"].parent / name).read_bytes())
        proof = first["entrypoint"].read_bytes()
        original = (self.formal / "fraction_arithmetic_harness.rs").read_bytes().replace(
            b"// No Kani assumptions, callee stubs or unchecked private field assembly.\n", b"")
        # Removing exactly the three selected mapping declarations restores the
        # entire original harness, including the unmodified rational oracle.
        attributes = b"".join(("#[kani::stub(" + pair["original"] + ", " + pair["replacement"] + ")]\n").encode()
                              for pair in self.jobs[13]["expected_stubs"])
        self.assertIn(original, proof.replace(attributes, b""))
        _, arithmetic, _, _ = PROJECTION._support(self.source)
        self.assertEqual(len(arithmetic.REQUIRED_ASSERTIONS[self.jobs[13]["harness"]]), 11)
        self.assertEqual(len(arithmetic.REQUIRED_COVERS[self.jobs[13]["harness"]]), 23)
        for label in (*arithmetic.REQUIRED_ASSERTIONS[self.jobs[13]["harness"]],
                      *arithmetic.REQUIRED_COVERS[self.jobs[13]["harness"]]):
            self.assertIn(('"' + label + '"').encode(), proof)
        self.assertEqual(first["callee_bindings"], restored["callee_bindings"])
        for index in (14, 15, 16):
            mutant = self.prepare(index)
            self.assertEqual(mutant["callee_bindings"], first["callee_bindings"])
            self.assertEqual(mutant["mutation"], arithmetic.CONTROLS["fraction_addition"][self.jobs[index]["case"]])

    def test_control_source_diff_is_actual_body_or_encoder_only(self):
        healthy = self.prepare(4)
        for index, key in ((2, "constructor"), (5, "comparison"), (8, "complement")):
            prepared = self.prepare(index)
            mutation = prepared["mutation"]
            before = (healthy["entrypoint"].parent / "fraction_composition_selected.rs").read_bytes()
            after = (prepared["entrypoint"].parent / "fraction_composition_selected.rs").read_bytes()
            self.assertEqual(after, before.replace(mutation["before"].encode(), mutation["after"].encode(), 1))
            self.assertNotEqual(prepared["callee_bindings"][key], healthy["callee_bindings"][key])
        for index, key in ((3, "constructor"), (6, "comparison"), (9, "complement")):
            prepared = self.prepare(index)
            self.assertEqual(prepared["compiled_span_sha256"], healthy["compiled_span_sha256"])
            self.assertNotEqual(prepared["callee_bindings"][key], healthy["callee_bindings"][key])
            self.assertEqual(prepared["mutation"]["owner"], "proof encoder")

    def test_fully_qualified_unique_stub_inventory_and_real_precondition_order(self):
        for index, job in enumerate(self.jobs):
            prepared = self.prepare(index)
            proof = prepared["entrypoint"].read_text()
            declared = [{"original": left, "replacement": right} for left, right in
                        re.findall(r"#\[kani::stub\(([^,]+), ([^)]+)\)\]", proof)]
            self.assertEqual(declared, job["expected_stubs"])
            self.assertEqual(len({p["original"] for p in declared}), len(declared))
            for pair in declared:
                self.assertTrue(all(path.startswith("crate::fraction_arithmetic::") for path in pair.values()))
                self.assertNotIn(" ", pair["original"])
                self.assertNotIn(" ", pair["replacement"])
            self.assertNotIn("stub_verified", proof)
            self.assertNotIn("proof_for_contract", proof)
            self.assertNotRegex(proof, r"impl\s+kani::Arbitrary|use[^;]+\bas\b")
        model = (self.formal / "fraction_composition_harness.rs").read_text()
        for key, method in (("comparison", "compare"), ("complement", "complement")):
            body = model.split("fn composition_model_" + method + "(", 1)[1].split("\n}\n", 1)[0]
            self.assertLess(body.index(PROJECTION.PRECONDITIONS[key]), body.index("composition_decode_"))
            self.assertLess(body.index("composition_decode_"), body.index("kani::assume("))
            invalid = model.split("fn composition_" + key + "_bad_precondition()", 1)[1].split("\n}\n", 1)[0]
            self.assertRegex(invalid, r"Fraction\s*\{\s*numerator:\s*0,\s*shift:\s*1,?\s*\}")
            self.assertIn("kani::assert(", body)
        generic = model.split("fn composition_assert_then_assume()", 1)[1].split("\n}\n", 1)[0]
        self.assertIn("kani::any::<bool>()", generic)
        self.assertIn("kani::assert(", generic)
        self.assertIn("kani::assume(condition)", generic)
        self.assertNotIn("assert!(false", generic)

    def test_seed_inventory_totality_and_mechanism_registry_labels(self):
        spec = (self.formal / "fraction_composition_spec.rs").read_text()
        for name in ("fraction", "order"):
            decoder = spec.split("fn composition_decode_" + name, 1)[1].split("\n}\n", 1)[0]
            self.assertNotIn("Fraction::new", decoder)
            self.assertNotIn("assume", decoder)
            self.assertNotIn("assert", decoder)
            for leaf in ("Unrepresentable", "Invariant", "ZeroLabel", "ReversedBounds", "ValidationInvariant"):
                self.assertIn(leaf, decoder)
        self.assertNotIn("kani::", spec)
        self.assertIn("if !composition_checked(left) || !composition_checked(right)", spec)
        self.assertIn("if gap < 64", spec)
        self.assertRegex(spec, r"if !composition_checked\(input\)\s*\{\s*return false;\s*\}")
        prepared = self.prepare(0)
        proof = prepared["entrypoint"].read_text()
        assertions, covers = PROJECTION.expected_contract(PROJECTION.MECHANISM_SUBJECT, PROJECTION.MECHANISM_HARNESS)
        self.assertEqual(len(assertions), 11)
        self.assertEqual(len(covers), 22)
        for label in assertions | covers:
            self.assertIn('"' + label + '"', proof)
        with self.assertRaises(ValueError):
            PROJECTION.expected_contract("fraction_addition", PROJECTION.MECHANISM_HARNESS)

    def test_drift_job_tampering_and_existing_custody_fail_closed(self):
        for field, value in (("id", "retry"), ("expected_stubs", []),
                             ("required_refinement_assertions", ["weaker"]), ("expected_kind", "ProofForContract")):
            job = {**self.jobs[13], field: value}
            with self.assertRaisesRegex(ValueError, "Unknown or changed fixed"):
                PROJECTION.prepare(self.source, self.base / "denied", job)
            self.assertFalse((self.base / "denied").exists())
        prepared = self.prepare(0)
        before = prepared["entrypoint"].read_bytes()
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare(0)
        self.assertEqual(prepared["entrypoint"].read_bytes(), before)
        path = self.formal / "fraction_arithmetic_harness.rs"
        path.write_bytes(path.read_bytes() + b"\n// drift\n")
        with self.assertRaisesRegex(ValueError, "support changed"):
            self.prepare(1)
        self.assertFalse((self.base / "1").exists())

    def test_new_spec_and_type_source_drift_invalidate_binding_or_refuse(self):
        first = self.prepare(4)
        path = self.formal / "fraction_composition_spec.rs"
        path.write_bytes(path.read_bytes() + b"\n// proof predicate identity change\n")
        changed = self.prepare(4, self.base / "new-spec")
        for key in ("constructor", "comparison", "complement"):
            self.assertNotEqual(first["callee_bindings"][key], changed["callee_bindings"][key])
        product = self.source / "crates/time-core/src/selection.rs"
        product.write_bytes(product.read_bytes() + b"\n// type source drift\n")
        with self.assertRaisesRegex(ValueError, "complete composition source changed"):
            self.prepare(4, self.base / "new-type")
        self.assertFalse((self.base / "new-type").exists())


if __name__ == "__main__":
    unittest.main()
