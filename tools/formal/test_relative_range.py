"""Raw relative-predicate proof preparation controls; execute no Rust/solver."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("relative_range_test", HERE / "relative_range_projection.py")
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("Cannot load local relative-range projection")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    """Verify exact module prefixes, controls, closure and fail-closed preparation."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-relative-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.domain = self.source / "crates/time-domain/src"
        self.domain.mkdir(parents=True)
        for name in (*PROJECTION.SOURCE_SHA256, "lib.rs"):
            (self.domain / name).write_bytes((ROOT / "crates/time-domain/src" / name).read_bytes())
        self.harness = HERE / "relative_range_harness.rs"
        self.inputs = self.base / "inputs"

    def prepare(self, case="healthy"):
        return PROJECTION.prepare_relative_range(self.source, self.inputs, case, self.harness)

    def test_healthy_preserves_complete_original_exchange_prefix(self):
        result = self.prepare()
        original = (self.domain / "exchange.rs").read_bytes()
        generated = (self.inputs / "exchange.rs").read_bytes()
        self.assertEqual(generated, original + PROJECTION.WRAPPER)
        self.assertTrue(result["exchange_original_prefix_preserved"])
        self.assertEqual(result["exchange_original_prefix_bytes"], len(original))
        self.assertEqual(result["exchange_compiled_prefix_bytes"], len(original))
        self.assertEqual(result["exchange_compiled_prefix_sha256"], PROJECTION.digest(original))
        self.assertEqual(result["appended_wrapper_sha256"], PROJECTION.digest(PROJECTION.WRAPPER))
        self.assertEqual(result["assumptions"], [])
        self.assertIsNone(result["mutation"])
        for name in ("coordinate.rs", "rounding.rs"):
            self.assertEqual((self.inputs / name).read_bytes(), (self.domain / name).read_bytes())
        receipt = json.loads((self.inputs / "correspondence.json").read_text())
        self.assertEqual(receipt, {**result, "entrypoint": "proof.rs"})

    def test_wrapper_calls_actual_private_predicate_without_constructing_state(self):
        self.assertEqual(PROJECTION.WRAPPER.count(b"relative_in_range(value)"), 1)
        self.assertNotIn(b"UtcCoordinateQ32 {", PROJECTION.WRAPPER)
        self.assertNotIn(b"PhaseOffsetQ33 {", PROJECTION.WRAPPER)
        self.assertNotIn(b"SignedRoundTripQ32 {", PROJECTION.WRAPPER)

    def test_all_controls_change_only_original_predicate_before_identical_wrapper(self):
        expected = {
            "reject_all": ("relative_control_success", "required relative success"),
            "accept_all": ("relative_control_upper", "upper relative rejection"),
            "widen_lower_edge": ("relative_control_lower", "lower relative rejection"),
        }
        original = (self.domain / "exchange.rs").read_bytes()
        for case, (harness, intended) in expected.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = PROJECTION.prepare_relative_range(self.source, folder, case, self.harness)
                mutation = result["mutation"]
                self.assertEqual((mutation["harness"], mutation["intended_failure"]), (harness, intended))
                before, after = mutation["before"].encode(), mutation["after"].encode()
                self.assertEqual(original.count(before), 1)
                expected_prefix = original.replace(before, after, 1)
                self.assertEqual((folder / "exchange.rs").read_bytes(), expected_prefix + PROJECTION.WRAPPER)
                self.assertFalse(result["exchange_original_prefix_preserved"])
                self.assertEqual(result["exchange_compiled_prefix_sha256"], PROJECTION.digest(expected_prefix))
                self.assertEqual(result["exchange_compiled_prefix_bytes"], len(expected_prefix))
                for name in ("coordinate.rs", "rounding.rs"):
                    self.assertEqual((folder / name).read_bytes(), (self.domain / name).read_bytes())

    def test_all_module_source_drift_is_rejected_before_output(self):
        for name in PROJECTION.SOURCE_SHA256:
            with self.subTest(name=name):
                path = self.domain / name
                original = path.read_bytes()
                path.write_bytes(original + b"\n// source drift\n")
                try:
                    with self.assertRaisesRegex(ValueError, "Reviewed complete module changed"):
                        self.prepare()
                    self.assertFalse(self.inputs.exists())
                finally:
                    path.write_bytes(original)

    def test_error_enum_drift_is_rejected_before_output(self):
        path = self.domain / "lib.rs"
        path.write_bytes(path.read_bytes().replace(b"    ArithmeticOverflow,", b"    OtherOverflow,", 1))
        with self.assertRaisesRegex(ValueError, "Actual DomainError enum differs"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_control_is_rejected_before_output(self):
        with self.assertRaisesRegex(ValueError, "Unknown relative-range control"):
            self.prepare("widen_upper_edge")
        self.assertFalse(self.inputs.exists())

    def test_existing_input_is_preserved(self):
        self.inputs.mkdir()
        target = self.inputs / "exchange.rs"
        target.write_bytes(b"existing evidence")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual(target.read_bytes(), b"existing evidence")
        self.assertFalse((self.inputs / "proof.rs").exists())

    def test_existing_dangling_symlink_is_not_followed(self):
        self.inputs.mkdir()
        target = self.inputs / "proof.rs"
        target.symlink_to(self.base / "missing-target")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertTrue(target.is_symlink())
        self.assertFalse((self.base / "missing-target").exists())

    def test_empty_harness_is_rejected_before_output(self):
        self.harness = self.base / "empty.rs"
        self.harness.write_bytes(b" \n")
        with self.assertRaisesRegex(ValueError, "harness is empty"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_five_jobs_include_all_distinct_controls_and_restored_healthy(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "reject_all", "accept_all", "widen_lower_edge", "healthy"])
        self.assertFalse(jobs[0]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertEqual(len({job["intended_failure"] for job in jobs[1:-1]}), 3)

    def test_harness_metadata_selects_exact_assertions_covers_and_entrypoints(self):
        text = self.harness.read_text()
        names = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", text))
        self.assertEqual(names, {job["harness"] for job in PROJECTION.schedule()})
        for name in (*PROJECTION.REQUIRED_ASSERTIONS["relative_full_domain"],
                     *PROJECTION.REQUIRED_COVERS["relative_full_domain"]):
            self.assertIn(f'"{name}"', text)
        self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS["relative_full_domain"]), 3)
        self.assertEqual(len(PROJECTION.REQUIRED_COVERS["relative_full_domain"]), 7)
        self.assertNotIn("kani::assume", text)

    def test_oracle_limit_and_control_neighbors_have_independent_integer_facts(self):
        literal = re.search(r"const RELATIVE_LIMIT: i128 = ([\d_]+);", self.harness.read_text())
        self.assertIsNotNone(literal)
        limit = int(literal.group(1).replace("_", ""))
        self.assertEqual(limit, 2 * ((2 ** 95 - 1) - (-(2 ** 95))))
        self.assertTrue(-limit <= 0 <= limit)
        self.assertFalse(-limit <= 2 ** 127 - 1 <= limit)
        self.assertFalse(-limit <= -limit - 1 <= limit)
        self.assertTrue(-limit - 1 <= -limit - 1 <= limit)


if __name__ == "__main__":
    unittest.main()
