"""Finite Gregorian-leaf preparation controls; execute no Rust, solver or native tool."""
from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("calendar_finite_laws_test", HERE / "calendar_finite_laws_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class FiniteLawPreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-finite-calendar-laws-")
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.source = self.folder / "source"
        names = (*PROJECTION.EXTRA_INPUTS, "crates/time-domain/src/lib.rs",
                 "crates/time-domain/src/leap.rs", "crates/time-domain/src/leap/validation.rs")
        for name in names:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.harness = self.source / "tools/formal/calendar_finite_laws_harness.rs"
        self.inputs = self.folder / "inputs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_calendar_finite_laws(
            self.source, self.inputs if inputs is None else inputs, case, self.harness)

    def test_exact_original_model_prefix_and_seven_original_laws_are_preserved(self):
        details = self.prepare()
        model = (self.source / PROJECTION.MODEL).read_bytes()
        original_prefix = model[:model.index(b"#[kani::proof]")]
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertTrue(proof.startswith(b"#![allow(dead_code)]\n" + original_prefix))
        self.assertEqual(len(original_prefix), 4059)
        self.assertEqual(hashlib.sha256(original_prefix).hexdigest(), PROJECTION.MODEL_PREFIX_SHA256)
        self.assertIn(PROJECTION.finite_body(model), proof)
        self.assertNotIn(b"fn calendar_gregorian_laws()", proof)
        self.assertNotIn(b"let cycles = kani::any::<u64>();", proof)
        self.assertNotIn(b'"arbitrary cycle forward law"', proof)
        self.assertIn(b'"laws maximum cycle model"', proof)
        self.assertTrue(details["actual_H_unchanged"])
        self.assertEqual(details["actual_H_structural_binding"]["status"], "checked-reviewed-byte-family")
        self.assertEqual(details["assumptions"], [])

    def test_each_auxiliary_model_fault_changes_one_expression_and_never_changes_H(self):
        model = (self.source / PROJECTION.MODEL).read_bytes()[:PROJECTION.MODEL_PREFIX_BYTES]
        healthy = self.folder / "healthy"
        self.prepare(inputs=healthy)
        actual_H = (healthy / "calendar_selected.rs").read_bytes()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                inputs = self.folder / case
                details = self.prepare(case, inputs)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(model.count(before), 1)
                changed = model.replace(before, after, 1)
                self.assertTrue((inputs / "proof.rs").read_bytes().startswith(b"#![allow(dead_code)]\n" + changed))
                self.assertEqual(details["compiled_model_prefix_sha256"], hashlib.sha256(changed).hexdigest())
                self.assertEqual((inputs / "calendar_selected.rs").read_bytes(), actual_H)
                self.assertIn("auxiliary Gregorian model", details["mutation"]["scope"])

    def test_full_model_projection_and_product_source_drift_reject_before_writing(self):
        for name in (PROJECTION.MODEL, PROJECTION.SOURCE_PROJECTION,
                     "crates/time-domain/src/leap/validation.rs"):
            path = self.source / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// unreviewed drift\n")
            try:
                with self.subTest(name=name), self.assertRaises(ValueError):
                    self.prepare()
                self.assertFalse(self.inputs.exists())
            finally:
                path.write_bytes(original)

    def test_changed_finite_claims_unknown_case_and_existing_evidence_are_rejected(self):
        original = self.harness.read_bytes()
        self.harness.write_bytes(original.replace(b'"March prefix origin"', b'"weakened origin"', 1))
        with self.assertRaisesRegex(ValueError, "exact seven remaining laws"):
            self.prepare()
        self.assertFalse(self.inputs.exists())
        self.harness.write_bytes(original)
        with self.assertRaisesRegex(ValueError, "Unknown finite Gregorian"):
            self.prepare("skip_a_law")
        self.inputs.mkdir()
        proof = self.inputs / "proof.rs"
        proof.write_bytes(b"retained evidence")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual(proof.read_bytes(), b"retained evidence")

    def test_five_jobs_and_all_actual_descriptions_remain_narrow_and_require_period(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "cycle_year_coefficient", "march_prefix_length", "forward_epoch_coefficient", "healthy"])
        self.assertFalse(jobs[0]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        text = self.harness.read_text()
        assertions = PROJECTION.REQUIRED_ASSERTIONS["calendar_finite_laws_full_domain"]
        covers = PROJECTION.REQUIRED_COVERS["calendar_finite_laws_full_domain"]
        self.assertEqual((len(assertions), len(covers)), (7, 4))
        for name in assertions + covers:
            self.assertIn('"' + name + '"', text)
        for index, job in enumerate(jobs, 1):
            metadata = self.prepare(job["case"], self.folder / f"job-{index}")
            self.assertEqual(metadata["required_separate_period_leaf"]["status"],
                             "not established by this finite leaf")
            self.assertIn("neither original D nor full calendar", metadata["qualification_scope"])
            self.assertEqual(metadata["required_separate_period_leaf"]["original_model_prefix_sha256"],
                             PROJECTION.MODEL_PREFIX_SHA256)

    def test_exact_additional_proof_closure_contains_existing_model_and_binding_owner(self):
        self.assertEqual(set(PROJECTION.EXTRA_INPUTS), {
            "tools/formal/calendar_finite_laws_harness.rs", "tools/formal/calendar_finite_laws_projection.py",
            "tools/formal/calendar_composition_harness.rs", "tools/formal/calendar_composition_projection.py"})


class IndependentFaultFacts(unittest.TestCase):
    def test_year_month_and_epoch_faults_violate_distinct_promises(self):
        self.assertNotEqual(364, 365)
        self.assertNotEqual(30, 31)
        good_epoch = 365 * 1899 + 1899 // 4 - 1899 // 100 + 1899 // 400 + 306
        bad_epoch = 364 * 1899 + 1899 // 4 - 1899 // 100 + 1899 // 400 + 306
        self.assertEqual(good_epoch, 693_901)
        self.assertNotEqual(bad_epoch, 693_901)


if __name__ == "__main__":
    unittest.main()
