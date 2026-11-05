"""Prepare exact leap-ticks proof inputs; execute no Rust compiler or solver."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("leap_ticks_projection_test", HERE / "leap_ticks_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-leap-ticks-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.domain = self.source / "crates/time-domain/src"
        self.domain.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.domain / name).write_bytes((ROOT / "crates/time-domain/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "leap_ticks_harness.rs"

    def prepare(self, case="healthy"):
        return PROJECTION.prepare_leap_ticks(self.source, self.inputs, case, self.harness)

    def test_healthy_preserves_exact_helper_coordinate_and_error_enum(self):
        result = self.prepare()
        original = (self.domain / "leap.rs").read_bytes()[19_736:19_818]
        self.assertEqual((self.inputs / "leap_ticks_selected.rs").read_bytes(),
                         original + b"\n" + PROJECTION.WRAPPER)
        self.assertEqual((self.inputs / "coordinate.rs").read_bytes(),
                         (self.domain / "coordinate.rs").read_bytes())
        self.assertTrue(result["original_helper_preserved"])
        self.assertTrue(result["complete_coordinate_module_preserved"])
        self.assertEqual(result["assumptions"], [])
        self.assertIsNone(result["mutation"])
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertIn(b"pub enum DomainError", proof)
        self.assertIn(b"    CoordinateOutOfRange,", proof)
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})

    def test_wrapper_and_harness_reach_actual_functions_without_private_assembly(self):
        self.assertEqual(PROJECTION.WRAPPER.count(b"second_ticks(seconds)"), 1)
        harness = self.harness.read_text()
        self.assertIn("coordinate::UtcCoordinateQ32::new(observed)", harness)
        self.assertIn("u128::from(seconds) << 32", harness)
        self.assertNotIn("kani::assume", harness)
        self.assertNotIn("UtcCoordinateQ32 {", harness)
        self.assertNotIn("LeapSecond {", harness)
        self.assertIn("Garde", " ".join(self.prepare()["omitted"]))

    def test_each_control_changes_only_one_reviewed_expression(self):
        original = (self.domain / "leap.rs").read_bytes()[19_736:19_818]
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = PROJECTION.prepare_leap_ticks(self.source, folder, case, self.harness)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(original.count(before), 1)
                self.assertEqual((folder / "leap_ticks_selected.rs").read_bytes(),
                                 original.replace(before, after, 1) + b"\n" + PROJECTION.WRAPPER)
                self.assertFalse(result["original_helper_preserved"])
                self.assertEqual(result["mutation"], control)
                self.assertEqual((folder / "coordinate.rs").read_bytes(),
                                 (self.domain / "coordinate.rs").read_bytes())

    def test_all_original_source_drift_fails_before_output_creation(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.domain / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// drift\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_unknown_control_and_existing_outputs_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("clamp_seconds")
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        (self.inputs / "proof.rs").write_bytes(b"retained evidence")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual((self.inputs / "proof.rs").read_bytes(), b"retained evidence")

    def test_schedule_preserves_three_distinct_faults_and_restored_full_domain(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "return_zero", "half_scale", "truncate_seconds", "healthy"])
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertFalse(jobs[0]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        self.assertEqual(len({job["intended_failure"] for job in jobs[1:-1]}), 3)
        text = self.harness.read_text()
        for description in (*PROJECTION.REQUIRED_ASSERTIONS["leap_ticks_full_domain"],
                            *PROJECTION.REQUIRED_COVERS["leap_ticks_full_domain"]):
            self.assertIn('"' + description + '"', text)


class IndependentPreparationFacts(unittest.TestCase):
    """Byte-concatenation facts check the specification, not Rust execution."""

    def encode(self, seconds):
        return int.from_bytes(seconds.to_bytes(8, "big") + bytes(4), "big")

    def test_literal_unit_era_and_machine_neighbors(self):
        self.assertEqual(self.encode(0), 0)
        self.assertEqual(self.encode(1), 4_294_967_296)
        self.assertEqual(self.encode(4_294_967_296), 18_446_744_073_709_551_616)
        last = self.encode(9_223_372_036_854_775_807)
        first_outside = self.encode(9_223_372_036_854_775_808)
        self.assertLessEqual(last, 2**95 - 1)
        self.assertEqual(first_outside, 2**95)
        self.assertEqual(self.encode(2**64 - 1), 2**96 - 2**32)
        self.assertLess(self.encode(2**64 - 1), 2**127)

    def test_each_planned_fault_has_a_discriminating_arithmetic_witness(self):
        self.assertNotEqual(0, self.encode(1))
        self.assertNotEqual((2**31) & (2**32 - 1), 0)
        truncated_second = 4_294_967_296 & (2**32 - 1)
        self.assertNotEqual(self.encode(truncated_second) >> 32, 4_294_967_296)


if __name__ == "__main__":
    unittest.main()
