"""Test proof preparation and independent fractions; run no compiler or solver."""
from __future__ import annotations

from fractions import Fraction
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("fraction_constructor_projection_test", HERE / "fraction_constructor_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-fraction-constructor-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "fraction_constructor_harness.rs"

    def prepare(self, case="healthy"):
        return PROJECTION.prepare_fraction_constructor(self.source, self.inputs, case, self.harness)

    def original_spans(self):
        return {name: (self.core / span["source"]).read_bytes()[slice(*span["offsets"])]
                for name, span in PROJECTION.SPANS.items()}

    def test_healthy_preserves_exact_private_fraction_zero_new_and_actual_error_enums(self):
        result = self.prepare()
        definitions = (self.inputs / "fraction_constructor_selected.rs").read_bytes()
        for name, span in self.original_spans().items():
            with self.subTest(name=name):
                self.assertEqual(definitions.count(span), 1)
        self.assertIn(b"\nstruct Fraction {", definitions)
        self.assertNotIn(b"pub struct Fraction", definitions)
        self.assertNotIn(b"pub fn new", definitions)
        self.assertNotIn(b"impl fmt::", definitions)
        self.assertNotIn(b"impl std::error::Error", definitions)
        self.assertNotIn(b"use garde", definitions)
        self.assertNotIn(b"fn compare", definitions)
        self.assertTrue(result["original_helper_preserved"])
        self.assertTrue(result["original_struct_zero_and_enums_preserved"])
        self.assertTrue(result["harness_inside_private_module"])
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertEqual(result["compiled_helper_sha256"], PROJECTION.SPANS["new"]["sha256"])
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions))
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})

    def test_harness_is_exact_and_lexically_inside_private_entrypoint_module(self):
        result = self.prepare()
        proof = (self.inputs / "proof.rs").read_bytes()
        definitions = (self.inputs / "fraction_constructor_selected.rs").read_bytes()
        harness = self.harness.read_bytes()
        self.assertEqual(proof, b'#![allow(dead_code)]\nmod fraction_constructor {\n'
                         b'include!("fraction_constructor_selected.rs");\n' + harness + b"\n}\n")
        self.assertEqual(result["harness_location"], "proof.rs")
        self.assertEqual(result["harness_sha256"], PROJECTION.digest(harness))
        self.assertEqual(result["entrypoint_sha256"], PROJECTION.digest(proof))
        self.assertEqual(proof.count(b"#[kani::proof]"), 4)
        self.assertNotIn(b"#[kani::proof]", definitions)
        for label in (*PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS],
                      *PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS],
                      *(control["intended_failure"] for control in PROJECTION.CONTROLS.values())):
            literal = ('"' + label + '"').encode()
            self.assertIn(literal, proof)
            self.assertNotIn(literal, definitions)

    def test_correspondence_maps_five_unique_actual_source_spans(self):
        result = self.prepare()
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)
        self.assertEqual(len(result["correspondence"]), 5)
        for item in result["correspondence"]:
            original = (self.source / item["source"]).read_bytes()
            selected = original[slice(*item["byte_offsets_half_open"])]
            self.assertEqual(original.count(selected), 1)
            self.assertEqual(PROJECTION.digest(selected), item["sha256"])

    def test_each_control_changes_only_one_constructor_block_and_preserves_zero(self):
        spans = self.original_spans()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = PROJECTION.prepare_fraction_constructor(self.source, folder, case, self.harness)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(spans["new"].count(before), 1)
                changed = spans["new"].replace(before, after, 1)
                definitions = (folder / "fraction_constructor_selected.rs").read_bytes()
                self.assertEqual(definitions.count(changed), 1)
                self.assertNotIn(spans["new"], definitions)
                for name, span in spans.items():
                    if name != "new":
                        self.assertEqual(definitions.count(span), 1)
                self.assertEqual(result["mutation"], control)
                self.assertEqual(result["compiled_helper_sha256"], PROJECTION.digest(changed))
                self.assertFalse(result["original_helper_preserved"])

    def test_complete_source_drift_rejected_before_any_output(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.core / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// source drift\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_span_guard_independently_rejects_changed_constructor(self):
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes().replace(b"shift > 1041 || numerator", b"shift > 1042 || numerator", 1)
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "source span changed: new"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_extraction_rejected_even_with_matching_complete_hash(self):
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes() + b"\n" + PROJECTION.SPANS["fraction"]["start"]
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous: fraction"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_control_and_existing_or_symlink_outputs_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("accept_all")
        self.assertFalse(self.inputs.exists())
        for name in ("proof.rs", "fraction_constructor_selected.rs", "correspondence.json"):
            for symlink in (False, True):
                folder = self.base / (name + ("-link" if symlink else "-file"))
                folder.mkdir()
                retained = folder / name
                if symlink:
                    retained.symlink_to(folder / "absent")
                else:
                    retained.write_bytes(b"retained evidence")
                with self.subTest(name=name, symlink=symlink), self.assertRaisesRegex(ValueError, "overwrite"):
                    PROJECTION.prepare_fraction_constructor(self.source, folder, "healthy", self.harness)
                self.assertEqual(list(folder.iterdir()), [retained])
                if symlink:
                    self.assertTrue(retained.is_symlink())
                else:
                    self.assertEqual(retained.read_bytes(), b"retained evidence")

    def test_five_job_schedule_restores_identical_healthy_bytes(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "always_error", "wrong_numerator", "wrong_error", "healthy"])
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertFalse(jobs[0]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        self.assertEqual(len({job["intended_failure"] for job in jobs[1:-1]}), 3)
        for index, job in enumerate(jobs):
            PROJECTION.prepare_fraction_constructor(self.source, self.base / str(index), job["case"], self.harness)
        for name in ("proof.rs", "fraction_constructor_selected.rs", "correspondence.json"):
            self.assertEqual((self.base / "0" / name).read_bytes(), (self.base / "4" / name).read_bytes())

    def test_harness_covers_full_raw_domain_with_independent_bit_length_oracle(self):
        harness = self.harness.read_text()
        self.assertEqual(harness.count("kani::any::<u64>()"), 1)
        self.assertEqual(harness.count("kani::any::<u32>()"), 1)
        self.assertNotIn("kani::assume", harness)
        self.assertIn("let bits = 64 - numerator.leading_zeros();", harness)
        self.assertIn("bits <= 53 && (numerator == 0 || bits <= shift)", harness)
        self.assertNotIn("1_u64 <<", harness)
        self.assertNotIn("shift < 64", harness)
        self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]), 8)
        self.assertEqual(len(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]), 14)
        omissions = " ".join(self.prepare()["omitted"])
        for name in ("compare", "complement", "Garde", "formatting", "Windows"):
            self.assertIn(name, omissions)


class IndependentSpecificationFacts(unittest.TestCase):
    """Unbounded integers/rationals check the specification, not Rust execution."""

    def test_bit_length_admission_matches_exact_proper_dyadics_at_machine_neighbors(self):
        numerators = [0, 1, 2, 3, 4, 2**52 - 1, 2**52, 2**53 - 1, 2**53,
                      2**53 + 1, 2**63 - 1, 2**63, 2**64 - 1]
        shifts = [0, 1, 2, 3, 52, 53, 54, 63, 64, 65, 1040, 1041, 1042, 2**32 - 1]
        for numerator in numerators:
            for shift in shifts:
                with self.subTest(numerator=numerator, shift=shift):
                    bits = numerator.bit_length()
                    expected = shift <= 1041 and bits <= 53 and (numerator == 0 or bits <= shift)
                    # Reject the unsupported exponent before forming a gigantic
                    # denominator; this is the actual specified admission limit.
                    rational = Fraction(numerator, 2**shift) if shift <= 1041 else None
                    independently_admitted = (rational is not None and 0 <= rational < 1
                                              and numerator <= 9_007_199_254_740_991)
                    self.assertEqual(expected, independently_admitted)
                    if expected:
                        canonical = (0, 0) if numerator == 0 else (numerator, shift)
                        self.assertEqual(Fraction(canonical[0], 2**canonical[1]), rational)

    def test_fourteen_planned_covers_have_independent_expected_outcomes(self):
        maximum = 2**53 - 1
        witnesses = [(0, 0, True), (0, 1041, True), (1, 1, True), (maximum, 53, True),
                     (1, 1041, True), (0, 1042, False), (0, 2**32 - 1, False),
                     (2**53, 54, False), (2**64 - 1, 1041, False), (1, 0, False),
                     (4, 2, False), (3, 2, True), (maximum, 63, True), (maximum, 64, True)]
        self.assertEqual(len(witnesses), 14)
        for numerator, shift, expected in witnesses:
            with self.subTest(numerator=numerator, shift=shift):
                if shift > 1041:
                    self.assertFalse(expected)
                else:
                    self.assertEqual(Fraction(numerator, 2**shift) < 1
                                     and numerator <= 9_007_199_254_740_991, expected)

    def test_three_controls_discriminate_success_retention_and_error_priority(self):
        self.assertTrue(0 < Fraction(1, 2) < 1)  # required success, despite reject-all
        healthy, wrong = Fraction(3, 4), Fraction(2, 4)
        self.assertTrue(0 < healthy < 1 and 0 < wrong < 1)
        self.assertNotEqual(healthy, wrong)  # validity alone misses wrong retention
        self.assertEqual(Fraction(0, 2**1042), 0)
        self.assertGreater(1042, 1041)  # raw shift rejection precedes zero canonicalization


if __name__ == "__main__":
    unittest.main()
