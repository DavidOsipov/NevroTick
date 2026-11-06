"""Exercise source preparation and independent facts; run no Rust or solver."""
from __future__ import annotations

from fractions import Fraction
import importlib.util
import json
import math
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("endpoint_finish_projection_test", HERE / "endpoint_finish_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-endpoint-finish-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "endpoint_finish_harness.rs"

    def prepare(self, case="healthy"):
        return PROJECTION.prepare_endpoint_finish(self.source, self.inputs, case, self.harness)

    def original_spans(self):
        return {name: (self.core / span["source"]).read_bytes()[slice(*span["offsets"])]
                for name, span in PROJECTION.SPANS.items()}

    def test_healthy_preserves_actual_private_struct_method_and_all_three_enums(self):
        result = self.prepare()
        module = (self.inputs / "endpoint_finish_selected.rs").read_bytes()
        originals = self.original_spans()
        for name, original in originals.items():
            with self.subTest(name=name):
                self.assertEqual(module.count(original), 1)
        self.assertIn(b"\nstruct EndpointParts {", module)
        self.assertIn(b"\nimpl EndpointParts {\n" + originals["finish"] + b"}\n", module)
        self.assertNotIn(b"pub struct EndpointParts", module)
        self.assertNotIn(b"pub fn finish", module)
        self.assertNotIn(b"impl fmt::", module)
        self.assertNotIn(b"impl std::error::Error", module)
        self.assertNotIn(b"use garde", module)
        self.assertNotIn(b"fn new(", module)
        self.assertNotIn(self.harness.read_bytes(), module)
        self.assertTrue(result["original_helper_preserved"])
        self.assertTrue(result["original_struct_and_enums_preserved"])
        self.assertTrue(result["harness_inside_private_module"])
        self.assertEqual(result["compiled_helper_sha256"], PROJECTION.SPANS["finish"]["sha256"])
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(module))
        self.assertEqual(result["assumptions"], [])
        self.assertIsNone(result["mutation"])
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertEqual(proof, b'#![allow(dead_code)]\nmod endpoint_finish {\n'
                         b'include!("endpoint_finish_selected.rs");\n'
                         + self.harness.read_bytes() + b"\n}\n")
        self.assertEqual(result["harness_location"], "proof.rs")
        self.assertEqual(result["entrypoint_sha256"], PROJECTION.digest(proof))

    def test_all_harness_assertions_are_lexically_in_qualified_entrypoint(self):
        self.prepare()
        proof = (self.inputs / "proof.rs").read_text()
        definitions = (self.inputs / "endpoint_finish_selected.rs").read_text()
        self.assertIn('mod endpoint_finish {\ninclude!("endpoint_finish_selected.rs");', proof)
        self.assertEqual(proof.count("#[kani::proof]"), 4)
        self.assertNotIn("#[kani::proof]", definitions)
        labels = (*PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS],
                  *PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS],
                  *(control["intended_failure"] for control in PROJECTION.CONTROLS.values()))
        for label in labels:
            self.assertIn('"' + label + '"', proof)
            self.assertNotIn('"' + label + '"', definitions)

    def test_correspondence_identifies_exact_complete_sources_and_unique_spans(self):
        result = self.prepare()
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)
        self.assertEqual(len(result["correspondence"]), 5)
        for record in result["correspondence"]:
            original = (self.source / record["source"]).read_bytes()
            span = original[slice(*record["byte_offsets_half_open"])]
            self.assertEqual(PROJECTION.digest(span), record["sha256"])
            self.assertEqual(original.count(span), 1)

    def test_each_control_changes_only_one_actual_method_expression(self):
        originals = self.original_spans()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = PROJECTION.prepare_endpoint_finish(self.source, folder, case, self.harness)
                method = originals["finish"]
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(method.count(before), 1)
                changed = method.replace(before, after, 1)
                module = (folder / "endpoint_finish_selected.rs").read_bytes()
                self.assertEqual(module.count(changed), 1)
                self.assertNotIn(method, module)
                for name, original in originals.items():
                    if name != "finish":
                        self.assertEqual(module.count(original), 1)
                self.assertEqual(result["mutation"], control)
                self.assertEqual(result["compiled_helper_sha256"], PROJECTION.digest(changed))
                self.assertFalse(result["original_helper_preserved"])

    def test_every_complete_source_drift_fails_before_output_creation(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.core / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// changed source\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_span_guard_independently_rejects_changed_body(self):
        path = self.core / "projection_enclosure.rs"
        original = path.read_bytes()
        changed = original.replace(b"(true, Ordering::Less) => -1,", b"(true, Ordering::Less) =>  0,", 1)
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "source span changed: finish"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_source_marker_is_rejected_even_with_matching_complete_hash(self):
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes() + b"\n/// Complete duplicate marker\n" + PROJECTION.SPANS["parts"]["start"]
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous: parts"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_case_and_existing_file_or_dangling_symlink_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("silently_clamp")
        self.assertFalse(self.inputs.exists())
        for name in ("proof.rs", "endpoint_finish_selected.rs", "correspondence.json"):
            for symlink in (False, True):
                folder = self.base / (name + ("-symlink" if symlink else "-file"))
                folder.mkdir()
                retained = folder / name
                if symlink:
                    retained.symlink_to(folder / "absent-target")
                else:
                    retained.write_bytes(b"retained evidence")
                with self.subTest(name=name, symlink=symlink), self.assertRaisesRegex(ValueError, "overwrite"):
                    PROJECTION.prepare_endpoint_finish(self.source, folder, "healthy", self.harness)
                self.assertEqual(list(folder.iterdir()), [retained])
                if symlink:
                    self.assertTrue(retained.is_symlink())
                else:
                    self.assertEqual(retained.read_bytes(), b"retained evidence")

    def test_schedule_and_restored_healthy_inputs_are_identical(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "reject_all", "omit_negative_floor", "range_as_invariant", "healthy"])
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertFalse(jobs[0]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        self.assertEqual(len({job["intended_failure"] for job in jobs[1:-1]}), 3)
        for index, job in enumerate(jobs):
            PROJECTION.prepare_endpoint_finish(self.source, self.base / str(index), job["case"], self.harness)
        for name in ("proof.rs", "endpoint_finish_selected.rs", "correspondence.json"):
            self.assertEqual((self.base / "0" / name).read_bytes(), (self.base / "4" / name).read_bytes())

    def test_harness_has_full_raw_domain_without_assumptions_and_fixed_obligations(self):
        harness = self.harness.read_text()
        self.assertIn("kani::any::<i128>()", harness)
        self.assertEqual(harness.count("kani::any::<bool>()"), 3)
        self.assertNotIn("kani::assume", harness)
        self.assertNotIn("EndpointParts::new", harness)
        self.assertEqual(harness.count("#[kani::proof]"), 4)
        self.assertEqual(len(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]), 10)
        for label in (*PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS],
                      *PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS],
                      *(control["intended_failure"] for control in PROJECTION.CONTROLS.values())):
            self.assertIn('"' + label + '"', harness)
        self.assertLess(harness.index("if required_success {"), harness.index("let twice_value = whole * 2 + sign;"))
        omissions = " ".join(self.prepare()["omitted"])
        for name in ("EndpointParts::new", "fraction", "Garde", "formatting", "Windows"):
            self.assertIn(name, omissions)


class IndependentSpecificationFacts(unittest.TestCase):
    """Unbounded rationals check the specification, not the copied Rust body."""

    def test_success_intervals_and_half_integer_relations_at_all_machine_neighbors(self):
        m, maximum = -(2**63), 2**63 - 1
        wholes = [-(2**127), -(2**127) + 1, m - 2, m - 1, m, m + 1,
                  -1, 0, 1, maximum - 1, maximum, maximum + 1, maximum + 2,
                  2**127 - 2, 2**127 - 1]
        residuals = [Fraction(-7, 8), Fraction(-1, 2), Fraction(-1, 100),
                     Fraction(0), Fraction(1, 100), Fraction(1, 2), Fraction(7, 8)]
        for whole in wholes:
            for residual in residuals:
                sign = (residual > 0) - (residual < 0)
                for lower in (False, True):
                    with self.subTest(whole=whole, residual=residual, lower=lower):
                        q = math.floor(whole + residual) if lower else math.ceil(whole + residual)
                        left, right = ((m + 1, maximum + 1) if lower and sign < 0 else
                                       (m - 1, maximum - 1) if not lower and sign > 0 else (m, maximum))
                        self.assertEqual(m <= q <= maximum, left <= whole <= right)
                        invariant = (whole == -(2**127) and lower and sign < 0
                                     or whole == 2**127 - 1 and not lower and sign > 0)
                        self.assertEqual(not -(2**127) <= q < 2**127, invariant)
                        if left <= whole <= right:
                            twice_value = 2 * whole + sign
                            if lower:
                                self.assertTrue(2 * q <= twice_value < 2 * q + 2)
                            else:
                                self.assertTrue(2 * q - 2 < twice_value <= 2 * q)
                            self.assertTrue(-(2**127) <= twice_value < 2**127)

    def test_each_control_has_a_distinct_literal_witness(self):
        zero = math.floor(Fraction(0))
        self.assertEqual(zero, 0)
        self.assertTrue(-(2**63) <= zero < 2**63)  # reject-all loses required success
        self.assertEqual(math.floor(Fraction(-1, 2)), -1)
        self.assertNotEqual(math.floor(Fraction(-1, 2)), 0)
        whole = 2**63
        self.assertFalse(-(2**63) <= whole < 2**63)
        self.assertTrue(-(2**127) <= whole < 2**127)  # range, not i128 overflow
        self.assertEqual((2**29 + 2**29) * 2**33, whole)


if __name__ == "__main__":
    unittest.main()
