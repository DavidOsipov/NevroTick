#!/usr/bin/env python3
"""Cheap preparation/correspondence controls; never execute Rust or a solver."""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest
from unittest import mock


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SPEC = importlib.util.spec_from_file_location(
    "coordinate_exchange_projection", HERE / "coordinate_exchange_projection.py"
)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("Cannot load local coordinate/exchange projection")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    """Check real source identity, exact copies and each intended compiled edit."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-coordinate-proof-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.source = self.work / "source"
        self.domain = self.source / "crates/time-domain/src"
        self.domain.mkdir(parents=True)
        for name in (*PROJECTION.SOURCE_SHA256, "lib.rs"):
            original = ROOT / "crates/time-domain/src" / name
            (self.domain / name).write_bytes(original.read_bytes())
        self.inputs = self.work / "inputs"
        self.harness = HERE / "coordinate_exchange_harness.rs"

    def prepare(self, subject="coordinate", case="healthy"):
        return PROJECTION.prepare_coordinate_exchange(
            self.source, self.inputs, subject, case, self.harness
        )

    def test_healthy_coordinate_preserves_complete_modules(self):
        self.check_healthy("coordinate")

    def test_healthy_exchange_preserves_complete_modules(self):
        self.check_healthy("exchange")

    def check_healthy(self, subject):
        details = self.prepare(subject)
        self.assertEqual(details["entrypoint"], self.inputs / "proof.rs")
        self.assertEqual(details["subject"], subject)
        self.assertIsNone(details["mutation"])
        self.assertEqual(details["assumptions"], [])
        for name, expected_hash in PROJECTION.SOURCE_SHA256.items():
            observed = (self.inputs / name).read_bytes()
            self.assertEqual(observed, (self.domain / name).read_bytes())
            self.assertEqual(hashlib.sha256(observed).hexdigest(), expected_hash)
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertIn(b"pub enum DomainError {", proof)
        self.assertIn(b"mod coordinate;", proof)
        self.assertIn(b"mod exchange;", proof)
        self.assertIn(b"mod rounding;", proof)
        self.assertIn(self.harness.read_bytes().replace(b"//!", b"//"), proof)
        receipt = json.loads((self.inputs / "correspondence.json").read_text())
        self.assertEqual(receipt, {**details, "entrypoint": "proof.rs"})
        self.assertEqual(
            {item["compiled_module"] for item in receipt["correspondence"]},
            {"coordinate.rs", "exchange.rs", "rounding.rs"},
        )

    def test_each_control_changes_only_its_named_actual_module(self):
        # Fixed expectations keep the detecting assertion independent of the
        # production projection's control metadata.
        cases = (
            ("coordinate", "always_error", "coordinate.rs",
             "coordinate_control_zero", "required coordinate success"),
            ("coordinate", "wrong_coordinate", "coordinate.rs",
             "coordinate_control_retained", "exact retained coordinate"),
            ("coordinate", "wrong_error", "coordinate.rs",
             "coordinate_control_rejection", "coordinate rejection classification"),
            ("exchange", "always_error", "exchange.rs",
             "exchange_control_zero", "required exchange success"),
            ("exchange", "wrong_offset", "exchange.rs",
             "exchange_control_phase", "exact phase numerator"),
            ("exchange", "wrong_rtt", "exchange.rs",
             "exchange_control_round_trip", "exact signed round trip"),
        )
        for subject, case, target, harness, assertion in cases:
            with self.subTest(subject=subject, case=case):
                destination = self.work / f"{subject}-{case}"
                details = PROJECTION.prepare_coordinate_exchange(
                    self.source, destination, subject, case, self.harness
                )
                mutation = details["mutation"]
                self.assertEqual(mutation["module"], target)
                self.assertEqual(mutation["harness"], harness)
                self.assertEqual(mutation["intended_failure"], assertion)
                self.assertIn(f"fn {harness}()", self.harness.read_text())
                self.assertIn(f'"{assertion}"', self.harness.read_text())
                changed = []
                for name in PROJECTION.SOURCE_SHA256:
                    original = (self.domain / name).read_bytes()
                    compiled = (destination / name).read_bytes()
                    if original != compiled:
                        changed.append(name)
                        before, after = mutation["before"].encode(), mutation["after"].encode()
                        self.assertEqual(original.count(before), 1)
                        self.assertEqual(compiled, original.replace(before, after, 1))
                    entry = next(item for item in details["correspondence"]
                                 if item["compiled_module"] == name)
                    self.assertEqual(entry["compiled_sha256"], hashlib.sha256(compiled).hexdigest())
                self.assertEqual(changed, [target])

    def test_changed_coordinate_is_rejected_before_any_output(self):
        self.check_source_drift("coordinate.rs")

    def test_changed_exchange_is_rejected_before_any_output(self):
        self.check_source_drift("exchange.rs")

    def test_changed_rounding_is_rejected_before_any_output(self):
        self.check_source_drift("rounding.rs")

    def check_source_drift(self, name):
        path = self.domain / name
        path.write_bytes(path.read_bytes() + b"\n// unexpected source drift\n")
        with self.assertRaisesRegex(ValueError, "Reviewed complete module changed"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_changed_actual_error_enum_is_rejected_before_any_output(self):
        path = self.domain / "lib.rs"
        path.write_bytes(path.read_bytes().replace(b"    ArithmeticOverflow,", b"    DifferentOverflow,", 1))
        with self.assertRaisesRegex(ValueError, "Actual DomainError enum differs"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_error_enum_extraction_is_rejected_before_output(self):
        path = self.domain / "lib.rs"
        path.write_bytes(path.read_bytes() + b"\n/// Fixed redacted domain failures\n")
        with self.assertRaisesRegex(ValueError, "Actual DomainError extraction is ambiguous"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_subject_is_rejected_before_output(self):
        with self.assertRaisesRegex(ValueError, "Unknown coordinate/exchange proof subject"):
            self.prepare("era")
        self.assertFalse(self.inputs.exists())

    def test_unknown_control_is_rejected_before_output(self):
        with self.assertRaisesRegex(ValueError, "Unknown coordinate/exchange proof control"):
            self.prepare("exchange", "wrong_error")
        self.assertFalse(self.inputs.exists())

    def test_empty_harness_is_rejected_before_output(self):
        self.harness = self.work / "empty.rs"
        self.harness.write_bytes(b" \n")
        with self.assertRaisesRegex(ValueError, "proof harness is empty"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_missing_control_point_is_rejected_before_output(self):
        control = dict(PROJECTION.CONTROLS["coordinate"]["always_error"])
        control["before"] = "not an actual source expression"
        with mock.patch.dict(PROJECTION.CONTROLS["coordinate"], {"always_error": control}):
            with self.assertRaisesRegex(ValueError, "mutation point is not unique"):
                self.prepare("coordinate", "always_error")
        self.assertFalse(self.inputs.exists())

    def test_oracle_literal_endpoints_follow_independent_integer_derivation(self):
        text = self.harness.read_text()
        values = {
            name: int(value.replace("_", ""))
            for name, value in re.findall(r"const (\w+): i128 = (-?[\d_]+);", text)
        }
        self.assertEqual(values["COORDINATE_LOWER"], -(2 ** 95))
        self.assertEqual(values["COORDINATE_UPPER"], 2 ** 95 - 1)
        self.assertEqual(values["RELATIVE_LIMIT"], 2 * ((2 ** 95 - 1) - (-(2 ** 95))))
        self.assertLessEqual(values["RELATIVE_LIMIT"] * 1_000_000_000, 2 ** 127 - 1)


if __name__ == "__main__":
    unittest.main()
