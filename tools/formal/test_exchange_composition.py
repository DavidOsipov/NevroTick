"""Exchange decomposition preparation controls; run no solver or Rust compiler."""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("exchange_composition_test", HERE / "exchange_composition_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class CompositionPreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-exchange-composition-")
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.source = self.folder / "source"
        self.domain = self.source / "crates/time-domain/src"
        self.domain.mkdir(parents=True)
        self.original = {}
        for name in ("coordinate.rs", "exchange.rs", "rounding.rs", "lib.rs"):
            data = (ROOT / "crates/time-domain/src" / name).read_bytes()
            self.original[name] = data
            (self.domain / name).write_bytes(data)
        self.harness = HERE / "exchange_composition_harness.rs"
        self.inputs = self.folder / "inputs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_exchange_composition(
            self.source, self.inputs if inputs is None else inputs, case, self.harness)

    def test_healthy_preserves_complete_actual_modules_and_exact_scaling_expression(self):
        details = self.prepare()
        for name in ("coordinate.rs", "exchange.rs"):
            self.assertEqual((self.inputs / name).read_bytes(), self.original[name])
        rounding = (self.inputs / "rounding.rs").read_bytes()
        self.assertTrue(rounding.startswith(self.original["rounding.rs"]))
        wrapper = rounding[len(self.original["rounding.rs"]):]
        expression = b"numerator\n        .checked_mul(1_000_000_000)"
        self.assertIn(b"fn proof_nanosecond_scale(numerator: i128) -> Option<i128>", wrapper)
        self.assertIn(expression, wrapper)
        self.assertNotIn(b".ok_or(", wrapper)
        selected = details["scaling_expression"]
        first, last = selected["source_byte_offsets_half_open"]
        self.assertEqual(self.original["rounding.rs"][first:last], expression)
        self.assertEqual(selected["original_sha256"], hashlib.sha256(expression).hexdigest())
        self.assertEqual(selected["compiled_sha256"], selected["original_sha256"])
        self.assertIn("no complete round() proof", selected["selection"])
        self.assertEqual(details["assumptions"], [])
        self.assertIsNone(details["mutation"])
        self.assertEqual(details["entrypoint"], self.inputs / "proof.rs")
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**details, "entrypoint": "proof.rs"})

    def test_three_exchange_controls_change_one_actual_source_expression(self):
        expectations = {
            "always_error": (b"if !relative_in_range(offset_numerator) || !relative_in_range(round_trip_numerator) {", b"if true {"),
            "wrong_phase": (b".checked_add(transmit_delta)", b".checked_sub(transmit_delta)"),
            "wrong_rtt": (b".checked_sub(peer_elapsed)", b".checked_add(peer_elapsed)"),
        }
        for case, (before, after) in expectations.items():
            with self.subTest(case=case):
                inputs = self.folder / case
                details = self.prepare(case, inputs)
                self.assertEqual(self.original["exchange.rs"].count(before), 1)
                self.assertEqual((inputs / "exchange.rs").read_bytes(),
                                 self.original["exchange.rs"].replace(before, after, 1))
                self.assertEqual((inputs / "coordinate.rs").read_bytes(), self.original["coordinate.rs"])
                self.assertTrue((inputs / "rounding.rs").read_bytes().startswith(self.original["rounding.rs"]))
                self.assertEqual(details["scaling_expression"]["compiled_sha256"],
                                 details["scaling_expression"]["original_sha256"])

    def test_scaling_control_changes_only_projected_copy_and_has_real_overflow_witness(self):
        details = self.prepare("wrong_scale")
        compiled = (self.inputs / "rounding.rs").read_bytes()
        self.assertTrue(compiled.startswith(self.original["rounding.rs"]))
        wrapper = compiled[len(self.original["rounding.rs"]):]
        self.assertIn(b"numerator\n        .checked_mul(2_000_000_000)", wrapper)
        self.assertNotIn(b".checked_mul(1_000_000_000)", wrapper)
        self.assertNotEqual(details["scaling_expression"]["compiled_sha256"],
                            details["scaling_expression"]["original_sha256"])
        self.assertEqual((self.inputs / "exchange.rs").read_bytes(), self.original["exchange.rs"])
        limit = 2 ** 97 - 2
        self.assertLessEqual(limit * 1_000_000_000, 2 ** 127 - 1)
        self.assertGreater(limit * 2_000_000_000, 2 ** 127 - 1)
        self.assertEqual(details["mutation"]["intended_failure"], "bounded scaling succeeds")

    def test_full_module_drift_is_rejected_before_any_output(self):
        for name in ("coordinate.rs", "exchange.rs", "rounding.rs"):
            with self.subTest(name=name):
                target = self.domain / name
                target.write_bytes(self.original[name] + b"\n// unreviewed drift\n")
                try:
                    with self.assertRaisesRegex(ValueError, "Reviewed complete module changed"):
                        self.prepare()
                    self.assertFalse(self.inputs.exists())
                finally:
                    target.write_bytes(self.original[name])

    def test_error_enum_drift_and_ambiguity_are_rejected(self):
        target = self.domain / "lib.rs"
        for data in (self.original["lib.rs"].replace(b"    ArithmeticOverflow,", b"    DifferentOverflow,", 1),
                     self.original["lib.rs"] + b"\n/// Fixed redacted domain failures\n"):
            target.write_bytes(data)
            with self.assertRaisesRegex(ValueError, "DomainError"):
                self.prepare()
            self.assertFalse(self.inputs.exists())

    def test_expression_selection_rejects_drift_or_ambiguity(self):
        original = self.original["rounding.rs"]
        for data in (original.replace(b".checked_mul(1_000_000_000)", b".checked_mul(2_000_000_000)", 1),
                     original + b"\n    let scaled = duplicate;\n"):
            with self.assertRaises(ValueError):
                PROJECTION.scaling_expression(data)

    def test_unknown_case_and_changed_harness_bound_create_no_inputs(self):
        with self.assertRaisesRegex(ValueError, "Unknown exchange-composition control"):
            self.prepare("weaken_required_success")
        self.assertFalse(self.inputs.exists())
        altered = self.folder / "altered-harness.rs"
        altered.write_bytes(self.harness.read_bytes().replace(
            b"158_456_325_028_528_675_187_087_900_670", b"158_456_325_028_528_675_187_087_900_671"))
        self.harness = altered
        with self.assertRaisesRegex(ValueError, "caller bound differs"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_existing_proof_and_dangling_symlink_are_never_overwritten(self):
        self.inputs.mkdir()
        target = self.inputs / "proof.rs"
        target.write_bytes(b"retained evidence")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual(target.read_bytes(), b"retained evidence")
        target.unlink()
        target.symlink_to(self.folder / "missing-external-target")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertTrue(target.is_symlink())
        self.assertFalse((self.folder / "missing-external-target").exists())

    def test_ten_job_composite_requires_every_healthy_control_and_restoration(self):
        jobs = PROJECTION.schedule()
        healthy = ["exchange_phase_full_domain", "exchange_rtt_full_domain", "exchange_scaling_full_domain"]
        self.assertEqual(len(jobs), 10)
        self.assertEqual([job["harness"] for job in jobs[:3]], healthy)
        self.assertEqual([job["harness"] for job in jobs[-3:]], healthy)
        self.assertTrue(all(job["case"] == "healthy" and not job["restored_healthy"] for job in jobs[:3]))
        self.assertTrue(all(job["case"] == "healthy" and job["restored_healthy"] for job in jobs[-3:]))
        self.assertEqual([job["case"] for job in jobs[3:7]],
                         ["always_error", "wrong_phase", "wrong_rtt", "wrong_scale"])
        self.assertEqual([job["intended_failure"] for job in jobs[3:7]],
                         ["phase required success", "exact phase numerator", "exact signed round trip", "bounded scaling succeeds"])
        details = self.prepare()
        composite = details["composite_contract"]
        self.assertEqual(composite["required_healthy_theorems"], healthy)
        self.assertEqual(composite["required_restored_theorems"], healthy)
        self.assertEqual(composite["fixed_jobs"], jobs)
        self.assertEqual(composite["relative_limit"], 2 ** 97 - 2)
        self.assertEqual(composite["nanoseconds_per_second"], 1_000_000_000)
        self.assertIn("no individual theorem", composite["acceptance"])

    def test_all_ten_preparations_keep_matching_original_composition_identities(self):
        baseline = None
        for index, job in enumerate(PROJECTION.schedule(), 1):
            with self.subTest(index=index, case=job["case"]):
                details = self.prepare(job["case"], self.folder / f"job-{index}")
                if baseline is None:
                    baseline = details["composite_contract"]
                self.assertEqual(details["composite_contract"], baseline)
                self.assertEqual(details["mutation"] is None, job["case"] == "healthy")
                self.assertEqual(details["harness_sha256"], hashlib.sha256(self.harness.read_bytes()).hexdigest())

    def test_harness_promises_are_nonempty_and_full_domains_have_no_assumptions(self):
        text = self.harness.read_text()
        inventory = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", text))
        self.assertEqual(inventory, {job["harness"] for job in PROJECTION.schedule()})
        self.assertEqual(len(inventory), 7)
        self.assertNotIn("kani::assume", text)
        self.assertNotRegex(text, r"\bUtcCoordinateQ32\s*\{\s*ticks\s*:")
        self.assertEqual(text.count("let t1 = admitted_ticks();"), 2)
        self.assertEqual(text.count("let t4 = admitted_ticks();"), 2)
        self.assertIn("scaling_contract(kani::any::<i128>());", text)
        self.assertIn("if numerator >= -RELATIVE_LIMIT && numerator <= RELATIVE_LIMIT", text)
        self.assertIn("(t2 + t3) - (t1 + t4)", text)
        self.assertIn("(t2 + t4) - (t1 + t3)", text)
        self.assertEqual([len(names) for names in PROJECTION.REQUIRED_ASSERTIONS.values()], [3, 3, 1])
        for harness, names in PROJECTION.REQUIRED_ASSERTIONS.items():
            self.assertEqual(len(PROJECTION.REQUIRED_COVERS[harness]), 4)
            for name in names + PROJECTION.REQUIRED_COVERS[harness]:
                self.assertIn(f'"{name}"', text)


if __name__ == "__main__":
    unittest.main()
