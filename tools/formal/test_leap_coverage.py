"""Preparation controls and independent finite facts; never execute Rust/Kani."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent


class IndependentCoverageFacts(unittest.TestCase):
    """Enumerated point sets are independent of the selected Boolean formula."""

    @staticmethod
    def unknown(point, history, expiry, continuous):
        reviewed = point < history
        imported = expiry is not None and point < expiry
        future = continuous is not None and point >= continuous
        return not (reviewed or imported or future)

    @classmethod
    def witness(cls, lower, upper, history, expiry, continuous):
        candidate = lower
        if candidate < history:
            candidate = history
        if expiry is not None and candidate < expiry:
            candidate = expiry
        return candidate if candidate <= upper and cls.unknown(
            candidate, history, expiry, continuous) else None

    def test_every_small_interval_and_optional_endpoint_matches_point_coverage(self):
        for history in (2, 3):
            for expiry in (None, 0, 2, 4):
                for continuous in (None, 0, 2, 3, 5):
                    cutoff = max(history, expiry if expiry is not None else history)
                    for lower in range(1, 6):
                        for upper in range(lower, 6):
                            offset = all(not self.unknown(p, history, expiry, continuous)
                                         for p in range(1, upper + 1))
                            event = all(not self.unknown(p, history, expiry, continuous)
                                        for p in range(lower, upper + 1))
                            bridged = continuous is not None and continuous <= cutoff
                            self.assertEqual(offset, upper < cutoff or bridged)
                            self.assertEqual(event, offset or (
                                continuous is not None and lower >= continuous))
                            unknown_points = [p for p in range(lower, upper + 1)
                                              if self.unknown(p, history, expiry, continuous)]
                            self.assertEqual(self.witness(lower, upper, history, expiry, continuous),
                                             unknown_points[0] if unknown_points else None)

    def test_three_faults_have_distinct_boundary_counterexamples_and_neighbors(self):
        # Endpoint equality is unknown, with known/unknown immediate neighbors.
        self.assertEqual([self.unknown(p, 3, None, None) for p in (2, 3, 4)],
                         [False, True, True])
        # An unbridged continuous interval is event-complete at its first point.
        self.assertEqual([self.unknown(p, 3, None, 5) for p in (4, 5, 6)],
                         [True, False, False])
        # No continuous rule may bridge even a one-unit uncovered gap.
        self.assertTrue(self.unknown(3, 3, None, 4))
        self.assertFalse(self.unknown(3, 3, None, 3))
        self.assertTrue(self.unknown(3, 3, None, 5))

    def test_fractional_edges_and_zero_are_not_normalized_away(self):
        unit = 1 << 32
        self.assertFalse(self.unknown(3 * unit - 1, 3 * unit, None, None))
        self.assertTrue(self.unknown(3 * unit, 3 * unit, None, None))
        self.assertTrue(self.unknown(3 * unit + 1, 3 * unit, None, 4 * unit))
        self.assertFalse(self.unknown(4 * unit, 3 * unit, None, 4 * unit))
        self.assertFalse(self.unknown(3 * unit, 3 * unit, 0, 0))
        self.assertFalse(self.unknown(3 * unit, 3 * unit, 4 * unit, None))


class ProjectionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location("leap_coverage_projection_test",
                                                    HERE / "leap_coverage_projection.py")
        cls.projection = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.projection)

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-leap-coverage-preparation-")
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.source, self.inputs = self.base / "source", self.base / "inputs"
        for name in self.projection.SOURCE_SHA256:
            path = self.source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes((ROOT / name).read_bytes())
        self.harness = HERE / "leap_coverage_harness.rs"

    def prepare(self, case="healthy", folder=None):
        return self.projection.prepare_leap_coverage(
            self.source, folder or self.inputs, case, self.harness)

    def test_healthy_prefix_and_actual_dependencies_are_preserved(self):
        result = self.prepare()
        source = (self.source / self.projection.SOURCE).read_bytes()
        prefix, suffix = self.projection.reviewed_parts(source)
        selected = (self.inputs / "coverage_selected.rs").read_bytes()
        self.assertEqual(source[813:1668], prefix)
        self.assertIn(prefix, selected)
        self.assertIn(self.projection.OBSERVATION_TAIL, selected)
        self.assertNotIn(suffix, selected)
        self.assertTrue(result["original_prefix_preserved"])
        self.assertFalse(result["structural_binding"]["machine_whole_interpret_equivalence"])
        self.assertEqual((self.inputs / "coordinate.rs").read_bytes(),
                         (self.source / "crates/time-domain/src/coordinate.rs").read_bytes())
        self.assertIn("Garde", " ".join(result["omitted"]))
        self.assertIn("all Some(0)", " ".join(result["admitted_domain"]))
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})

    def test_control_changes_only_its_actual_source_expression(self):
        original, _ = self.projection.reviewed_parts((self.source / self.projection.SOURCE).read_bytes())
        for case, control in self.projection.CONTROLS.items():
            with self.subTest(case=case):
                result = self.prepare(case, self.base / case)
                expected = original.replace(control["before"].encode(), control["after"].encode(), 1)
                self.assertEqual(original.count(control["before"].encode()), 1)
                self.assertIn(expected, (self.base / case / "coverage_selected.rs").read_bytes())
                self.assertEqual(result["compiled_prefix_sha256"], self.projection.digest(expected))
                self.assertFalse(result["original_prefix_preserved"])
                self.assertEqual(result["mutation"], control)

    def test_each_context_drift_rejects_before_any_output(self):
        for name in self.projection.SOURCE_SHA256:
            path = self.source / name
            before = path.read_bytes()
            path.write_bytes(before + b"\n// drift\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(before)

    def test_reviewed_cut_refuses_changed_suffix_or_prefix(self):
        source = (self.source / self.projection.SOURCE).read_bytes()
        for before, after in [(b"let mut initial_offset = 10", b"let mut initial_offset = 11"),
                              (b"latest < known_before", b"latest <= known_before")]:
            with self.subTest(before=before), self.assertRaisesRegex(ValueError, "source changed"):
                self.projection.reviewed_parts(source.replace(before, after, 1))

    def test_changed_assumption_owner_guards_are_rejected(self):
        name = "crates/time-domain/src/leap/validation.rs"
        self.assertIn(name, self.projection.SOURCE_SHA256)
        path = self.source / name
        original = path.read_bytes()
        for before, after in [
            (b"coordinate_ticks(dates.earliest()) < baseline_ticks()",
             b"coordinate_ticks(dates.earliest()) < 0"),
            (b"reviewed_before <= LEAP_BASELINE_NTP_SECONDS", b"reviewed_before == 0"),
        ]:
            with self.subTest(before=before):
                self.assertEqual(original.count(before), 1)
                path.write_bytes(original.replace(before, after, 1))
                with self.assertRaisesRegex(ValueError, "source changed"):
                    self.prepare()
                self.assertFalse(self.inputs.exists())
        path.write_bytes(original)
        result = self.prepare()
        self.assertTrue(any(record["source"] == name for record in result["correspondence"]))

    def test_existing_or_empty_inputs_are_not_replaced(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("always_known")
        self.assertFalse(self.inputs.exists())
        empty = self.base / "empty.rs"
        empty.write_bytes(b"")
        with self.assertRaisesRegex(ValueError, "empty"):
            self.projection.prepare_leap_coverage(self.source, self.inputs, "healthy", empty)
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        sentinel = self.inputs / "proof.rs"
        sentinel.write_bytes(b"retained proof input")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual(sentinel.read_bytes(), b"retained proof input")

    def test_contract_names_schedule_and_assumptions_are_explicit(self):
        jobs = self.projection.schedule()
        self.assertEqual([j["case"] for j in jobs],
                         ["healthy", "include_exclusive_end", "exclude_continuous_start",
                          "bridge_one_second_gap", "healthy"])
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        text = self.harness.read_text()
        for phrase in (*self.projection.REQUIRED_ASSERTIONS["leap_coverage_full_domain"],
                       *self.projection.REQUIRED_COVERS["leap_coverage_full_domain"],
                       *(j["intended_failure"] for j in jobs[1:-1])):
            self.assertIn('"' + phrase + '"', text)
        self.assertIn("unknown_at", text)
        self.assertIn("gap_witness", text)
        self.assertIn("i32", text)
        self.assertNotIn("assume(continuous", text)
        self.assertNotIn("assume(expiry", text)
        self.assertEqual(text.count("kani::assume("), 4)
        self.assertNotIn("assume(probe", text)
        self.assertEqual(len(self.projection.REQUIRED_ASSERTIONS["leap_coverage_full_domain"]), 9)
        self.assertEqual(len(self.projection.REQUIRED_COVERS["leap_coverage_full_domain"]), 13)
        names = self.projection.input_names()
        self.assertIn(self.projection.SOURCE, names)
        self.assertIn("tools/formal/leap_coverage_projection.py", names)
        self.assertIn("tools/formal/leap_coverage_harness.rs", names)
        self.assertIn("tools/formal/test_leap_coverage.py", names)


if __name__ == "__main__":
    unittest.main()
