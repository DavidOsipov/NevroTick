"""Preparation controls for the clock proofs; no compiler or solver execution."""
from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path
import re
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("clock_projection_test", Path(__file__).with_name("clock_projection.py"))
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)
HARNESS = Path(__file__).with_name("clock_harness.rs")


class ClockProjectionTests(unittest.TestCase):
    """Preserve actual source correspondence and finite control selection."""

    def fixture(self, folder):
        source = folder / "source"
        domain = source / "crates/time-domain/src"
        domain.mkdir(parents=True)
        for name in ("clock.rs", "lib.rs"):
            (domain / name).write_bytes((ROOT / "crates/time-domain/src" / name).read_bytes())
        return source, domain, folder / "inputs"

    def test_complete_healthy_module_is_byte_exact_and_source_is_unchanged(self):
        with tempfile.TemporaryDirectory() as directory:
            source, domain, inputs = self.fixture(Path(directory))
            original = (domain / "clock.rs").read_bytes()
            details = PROJECTION.prepare_clock(source, inputs, "healthy", HARNESS)
            self.assertEqual((inputs / "clock.rs").read_bytes(), original)
            self.assertEqual((domain / "clock.rs").read_bytes(), original)
            self.assertEqual(details["entrypoint"], inputs / "proof.rs")
            self.assertEqual(details["compiled_subject_sha256"], hashlib.sha256(original).hexdigest())
            self.assertIsNone(details["mutation"])
            self.assertEqual(details["assumptions"], [])
            self.assertEqual(details["omitted"], [])
            self.assertIn(b"pub enum DomainError {", (inputs / "proof.rs").read_bytes())

    def test_clock_drift_rejects_before_emitting_inputs(self):
        with tempfile.TemporaryDirectory() as directory:
            source, domain, inputs = self.fixture(Path(directory))
            target = domain / "clock.rs"
            target.write_bytes(target.read_bytes() + b"\n")
            with self.assertRaisesRegex(ValueError, "Clock source differs"):
                PROJECTION.prepare_clock(source, inputs, "healthy", HARNESS)
            self.assertFalse(inputs.exists())

    def test_actual_error_enum_drift_rejects_before_emitting_inputs(self):
        with tempfile.TemporaryDirectory() as directory:
            source, domain, inputs = self.fixture(Path(directory))
            target = domain / "lib.rs"
            target.write_bytes(target.read_bytes().replace(b"    DurationOutOfOrder,", b"    DifferentDurationError,", 1))
            with self.assertRaisesRegex(ValueError, "DomainError enum changed"):
                PROJECTION.prepare_clock(source, inputs, "healthy", HARNESS)
            self.assertFalse(inputs.exists())

    def test_ambiguous_error_enum_selection_rejects(self):
        with tempfile.TemporaryDirectory() as directory:
            source, domain, inputs = self.fixture(Path(directory))
            target = domain / "lib.rs"
            target.write_bytes(target.read_bytes() + b"\n/// Fixed redacted domain failures\n")
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous"):
                PROJECTION.prepare_clock(source, inputs, "healthy", HARNESS)
            self.assertFalse(inputs.exists())

    def test_unknown_control_rejects_without_outputs(self):
        with tempfile.TemporaryDirectory() as directory:
            source, _, inputs = self.fixture(Path(directory))
            with self.assertRaisesRegex(ValueError, "Unknown clock control"):
                PROJECTION.prepare_clock(source, inputs, "active_awake_unreviewed", HARNESS)
            self.assertFalse(inputs.exists())

    def test_each_defect_changes_one_expression_in_only_its_domain(self):
        for owner, reading, duration, other in (
            ("monotonic", "MonotonicNs", "MonotonicDurationNs", "ActiveAwakeNs"),
            ("active_awake", "ActiveAwakeNs", "ActiveAwakeDurationNs", "MonotonicNs"),
        ):
            for defect in ("always_error", "wrong_duration", "wrong_error"):
                with self.subTest(owner=owner, defect=defect), tempfile.TemporaryDirectory() as directory:
                    source, domain, inputs = self.fixture(Path(directory))
                    original = (domain / "clock.rs").read_bytes()
                    details = PROJECTION.prepare_clock(source, inputs, f"{owner}_{defect}", HARNESS)
                    changed = (inputs / "clock.rs").read_bytes()
                    start = original.index(f"impl {reading} {{".encode())
                    stop = original.index(f"impl {duration} {{".encode())
                    self.assertEqual(changed[:start], original[:start])
                    self.assertTrue(changed.endswith(original[stop:]))
                    if defect == "wrong_error":
                        before = b".ok_or(DomainError::ArithmeticOverflow)?"
                        after = b".ok_or(DomainError::DurationOutOfOrder)?"
                    else:
                        before = f"Ok({duration} {{ nanoseconds }})".encode()
                        after = (b"Err(DomainError::DurationOutOfOrder)" if defect == "always_error" else
                                 f"Ok({duration} {{ nanoseconds: nanoseconds ^ 1 }})".encode())
                    expected = original[:start] + original[start:stop].replace(before, after, 1) + original[stop:]
                    self.assertEqual(changed, expected)
                    self.assertEqual(details["mutation"]["owner_impl"], reading)
                    self.assertEqual((domain / "clock.rs").read_bytes(), original)
                    self.assertIn(f"impl {other} {{".encode(), changed)

    def test_schedule_preserves_both_healthy_and_restored_proofs(self):
        jobs = PROJECTION.recommended_schedule()
        self.assertEqual(len(jobs), 10)
        self.assertEqual([(j["case"], j["harness"], j["restored_healthy"]) for j in jobs[:2]],
                         [("healthy", "monotonic_full_domain", False),
                          ("healthy", "active_awake_full_domain", False)])
        self.assertEqual([(j["case"], j["harness"], j["restored_healthy"]) for j in jobs[-2:]],
                         [("healthy", "monotonic_full_domain", True),
                          ("healthy", "active_awake_full_domain", True)])
        self.assertEqual({j["case"] for j in jobs[2:-2]}, {
            "monotonic_always_error", "monotonic_wrong_duration", "monotonic_wrong_error",
            "active_awake_always_error", "active_awake_wrong_duration", "active_awake_wrong_error"})
        self.assertTrue(all(j["intended_failure"] for j in jobs[2:-2]))

    def test_harness_inventory_and_nonvacuous_contract_metadata(self):
        source = HARNESS.read_text()
        contracts = PROJECTION.harness_contracts()
        harnesses = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", source))
        self.assertEqual(harnesses, set(contracts))
        self.assertEqual(len(harnesses), 8)
        self.assertNotIn("kani::assume", source)
        self.assertNotIn("MonotonicDurationNs {", source)
        self.assertNotIn("ActiveAwakeDurationNs {", source)
        self.assertNotIn(".checked_sub(", source)
        self.assertIn("u128::from(a) - u128::from(b)", source)
        self.assertIn("u128::from(a) + u128::from(delta)", source)
        for harness, contract in contracts.items():
            if harness.endswith("full_domain"):
                self.assertEqual(len(contract["successful_assertions"]), 6)
                self.assertEqual(len(contract["covers"]), 6)
                for name in contract["successful_assertions"] + contract["covers"]:
                    self.assertIn(f'"{name}"', source)
            else:
                self.assertEqual(contract["covers"], [])
                self.assertIn(f'"{contract["intended_failure"]}"', source)


if __name__ == "__main__":
    unittest.main()
