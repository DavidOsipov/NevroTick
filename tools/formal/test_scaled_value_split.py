"""Check source-bound split preparation and independent arithmetic; run no Rust."""
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
SPEC = importlib.util.spec_from_file_location("scaled_value_split_projection_test",
                                             HERE / "scaled_value_split_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)
MAXIMUM = 2**53 - 1


def rational_expectation(significand, scale):
    """Independent unbounded-rational specification, not product execution.

    Gate unsupported raw inputs before exponentiation/negation. Rational floor
    and subtraction determine the result without the Rust cutoff/mask algorithm.
    Preserve the original denominator exponent for a nonzero fraction.
    """
    if significand > MAXIMUM or scale < -1041 or scale > 11:
        return None
    exact = Fraction(significand) * (2**scale if scale >= 0 else Fraction(1, 2**-scale))
    whole = exact.numerator // exact.denominator
    residual = exact - whole
    if residual == 0:
        return whole, 0, 0
    exponent = -scale
    raw_remainder = residual * 2**exponent
    if raw_remainder.denominator != 1:
        raise AssertionError("Expected an exact raw dyadic numerator")
    return whole, raw_remainder.numerator, exponent


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-scaled-split-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "scaled_value_split_harness.rs"

    def prepare(self, case="healthy"):
        return PROJECTION.prepare_scaled_value_split(self.source, self.inputs, case, self.harness)

    def original_spans(self):
        return {name: (self.core / span["source"]).read_bytes()[slice(*span["offsets"])]
                for name, span in PROJECTION.SPANS.items()}

    def test_healthy_preserves_actual_private_split_factory_zero_types_and_errors(self):
        result = self.prepare()
        definitions = (self.inputs / "scaled_value_split_selected.rs").read_bytes()
        for name, span in self.original_spans().items():
            with self.subTest(name=name):
                self.assertEqual(definitions.count(span), 1)
        self.assertIn(b"\nstruct Fraction {", definitions)
        self.assertIn(b"\nstruct ScaledValue {", definitions)
        for absent in (b"pub struct Fraction", b"pub struct ScaledValue", b"pub fn split",
                       b"impl fmt::", b"impl std::error::Error", b"use garde",
                       b"fn compare", b"fn from_bits", b"fn signed_whole"):
            self.assertNotIn(absent, definitions)
        self.assertTrue(result["original_helper_preserved"])
        self.assertTrue(result["original_struct_zero_constructor_and_enums_preserved"])
        self.assertTrue(result["harness_inside_private_module"])
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertEqual(result["compiled_helper_sha256"], PROJECTION.SPANS["split"]["sha256"])
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions))
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})

    def test_exact_harness_and_obligations_are_in_private_entrypoint_module(self):
        result = self.prepare()
        proof = (self.inputs / "proof.rs").read_bytes()
        definitions = (self.inputs / "scaled_value_split_selected.rs").read_bytes()
        harness = self.harness.read_bytes()
        self.assertEqual(proof, b'#![allow(dead_code)]\nmod scaled_value_split {\n'
                         b'include!("scaled_value_split_selected.rs");\n' + harness + b"\n}\n")
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

    def test_correspondence_maps_seven_unique_actual_source_spans(self):
        result = self.prepare()
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)
        self.assertEqual(len(result["correspondence"]), 7)
        self.assertEqual({item["item"] for item in result["correspondence"]},
                         {"fraction", "zero", "new", "scaled_value", "split", "enclosure_error", "interval_error"})
        for item in result["correspondence"]:
            original = (self.source / item["source"]).read_bytes()
            selected = original[slice(*item["byte_offsets_half_open"])]
            self.assertEqual(original.count(selected), 1)
            self.assertEqual(PROJECTION.digest(selected), item["sha256"])

    def test_each_control_changes_only_one_split_block_preserving_actual_factory(self):
        spans = self.original_spans()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = PROJECTION.prepare_scaled_value_split(self.source, folder, case, self.harness)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(spans["split"].count(before), 1)
                changed = spans["split"].replace(before, after, 1)
                definitions = (folder / "scaled_value_split_selected.rs").read_bytes()
                self.assertEqual(definitions.count(changed), 1)
                self.assertNotIn(spans["split"], definitions)
                for name, span in spans.items():
                    if name != "split":
                        self.assertEqual(definitions.count(span), 1)
                self.assertEqual(result["mutation"], control)
                self.assertEqual(result["compiled_helper_sha256"], PROJECTION.digest(changed))
                self.assertFalse(result["original_helper_preserved"])

    def test_complete_source_drift_rejected_before_creating_outputs(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.core / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// source drift\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_each_span_guard_rejects_drift_even_when_complete_hash_is_rebound(self):
        for name, span in PROJECTION.SPANS.items():
            path = self.core / span["source"]
            original = path.read_bytes()
            first, last = span["offsets"]
            # Change one byte inside each selected definition without changing
            # file length or the unique extraction boundary comments.
            location = last - 2
            changed = original[:location] + bytes((original[location] ^ 1,)) + original[location + 1:]
            path.write_bytes(changed)
            with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
                with self.subTest(name=name), self.assertRaisesRegex(ValueError, "source span changed: " + name):
                    self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_ambiguous_extraction_rejected_with_matching_complete_hash(self):
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes() + b"\n" + PROJECTION.SPANS["split"]["start"]
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous: split"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_control_and_existing_or_symlink_outputs_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("accept_all")
        self.assertFalse(self.inputs.exists())
        for name in ("proof.rs", "scaled_value_split_selected.rs", "correspondence.json"):
            for symlink in (False, True):
                folder = self.base / (name + ("-link" if symlink else "-file"))
                folder.mkdir()
                retained = folder / name
                if symlink:
                    retained.symlink_to(folder / "absent")
                else:
                    retained.write_bytes(b"retained evidence")
                with self.subTest(name=name, symlink=symlink), self.assertRaisesRegex(ValueError, "overwrite"):
                    PROJECTION.prepare_scaled_value_split(self.source, folder, "healthy", self.harness)
                self.assertEqual(list(folder.iterdir()), [retained])
                if symlink:
                    self.assertTrue(retained.is_symlink())
                else:
                    self.assertEqual(retained.read_bytes(), b"retained evidence")

    def test_redirected_or_regular_input_directory_is_rejected(self):
        target = self.base / "retained"
        target.mkdir()
        self.inputs.symlink_to(target, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "directory redirects"):
            self.prepare()
        self.assertEqual(list(target.iterdir()), [])
        self.inputs.unlink()
        self.inputs.write_bytes(b"retained")
        with self.assertRaisesRegex(ValueError, "not a directory"):
            self.prepare()
        self.assertEqual(self.inputs.read_bytes(), b"retained")

    def test_five_job_schedule_restores_identical_healthy_bytes(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "reject_all", "zero_remainder", "wrong_error", "healthy"])
        self.assertEqual({job["subject"] for job in jobs}, {"scaled_value_split"})
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertEqual(len({job["intended_failure"] for job in jobs[1:-1]}), 3)
        for index, job in enumerate(jobs):
            PROJECTION.prepare_scaled_value_split(self.source, self.base / str(index), job["case"], self.harness)
        for name in ("proof.rs", "scaled_value_split_selected.rs", "correspondence.json"):
            self.assertEqual((self.base / "0" / name).read_bytes(), (self.base / "4" / name).read_bytes())

    def test_full_raw_domain_uses_independent_cutoff_and_no_assumptions_or_narrow_whole_bound(self):
        harness = self.harness.read_text()
        self.assertEqual(harness.count("kani::any::<u64>()"), 1)
        self.assertEqual(harness.count("kani::any::<i64>()"), 1)
        self.assertNotIn("kani::assume", harness)
        self.assertNotIn("1041..=11", harness)
        self.assertNotIn("denominator_shift < 53", harness)
        self.assertNotIn("significand &", harness)
        self.assertNotIn("MAX_SCALED_MAGNITUDE", harness)
        self.assertNotIn("1_u64 << 63", harness)
        self.assertIn("denominator_shift < 64", harness)
        self.assertIn("let reconstructed = (u128::from(whole) << denominator_shift)", harness)
        self.assertLess(harness.index("if required_success {"), harness.index("(-scale) as u32"))
        self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]), 13)
        self.assertEqual(len(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]), 20)
        self.assertEqual(harness.count("kani::cover!("), 20)
        omissions = " ".join(self.prepare()["omitted"])
        for name in ("from_bits", "signed_whole", "compare", "complement", "Garde", "formatting", "Windows"):
            self.assertIn(name, omissions)


class IndependentSpecificationFacts(unittest.TestCase):
    """Unbounded rational observations do not execute or qualify actual Rust."""

    def test_machine_neighbors_obey_euclidean_and_fraction_factory_contracts(self):
        numerators = [0, 1, 2, 3, 4, 2**51, 2**52 - 1, 2**52, MAXIMUM,
                      2**53, 2**53 + 1, 2**63, 2**64 - 1]
        scales = [-2**63, -1042, -1041, -1040, -65, -64, -63, -54, -53,
                  -52, -51, -2, -1, 0, 1, 10, 11, 12, 2**63 - 1]
        for numerator in numerators:
            for scale in scales:
                with self.subTest(numerator=numerator, scale=scale):
                    observed = rational_expectation(numerator, scale)
                    admitted = numerator < 2**53 and -1041 <= scale <= 11
                    self.assertEqual(observed is not None, admitted)
                    if not admitted:
                        continue
                    whole, remainder, exponent = observed
                    self.assertLessEqual(whole, 2**64 - 2048)
                    if scale >= 0:
                        self.assertEqual(whole, numerator * 2**scale)
                        self.assertEqual((remainder, exponent), (0, 0))
                    else:
                        k = -scale
                        # Division in unbounded integers is independent of both
                        # the production cutoff/mask and the Rust reconstruction.
                        quotient, residual = divmod(numerator, 2**k)
                        self.assertEqual((whole, remainder), (quotient, residual))
                        self.assertEqual(exponent, k if remainder else 0)
                        if k < 64:
                            self.assertEqual((whole << k) + remainder, numerator)
                            self.assertLess(remainder, 2**k)
                        else:
                            self.assertEqual((whole, remainder), (0, numerator))
                    # The actual Fraction::new receives only this valid subset.
                    self.assertLess(remainder, 2**53)
                    self.assertLessEqual(exponent, 1041)
                    self.assertTrue(remainder == 0 or 0 < Fraction(remainder, 2**exponent) < 1)

    def test_twenty_planned_witnesses_have_independent_exact_outcomes(self):
        accepted = [
            (0, 0, (0, 0, 0)), (0, -1041, (0, 0, 0)), (0, -64, (0, 0, 0)),
            (1, 0, (1, 0, 0)), (MAXIMUM, 11, (2**64 - 2048, 0, 0)),
            (2, -1, (1, 0, 0)), (3, -1, (1, 1, 1)),
            (MAXIMUM, -52, (1, 2**52 - 1, 52)), (MAXIMUM, -53, (0, MAXIMUM, 53)),
            (MAXIMUM, -63, (0, MAXIMUM, 63)), (MAXIMUM, -64, (0, MAXIMUM, 64)),
            (1, -1041, (0, 1, 1041)),
        ]
        rejected = [(2**53, -54), (2**64 - 1, -1041), (0, 12), (1, 12),
                    (0, -1042), (1, -1042), (0, -2**63), (0, 2**63 - 1)]
        self.assertEqual(len(accepted) + len(rejected), 20)
        for numerator, scale, expected in accepted:
            self.assertEqual(rational_expectation(numerator, scale), expected)
        for numerator, scale in rejected:
            self.assertIsNone(rational_expectation(numerator, scale))

    def test_raw_whole_maximum_is_larger_than_the_separate_struct_field_limit(self):
        expected = rational_expectation(MAXIMUM, 11)
        self.assertEqual(expected, (18_446_744_073_709_549_568, 0, 0))
        self.assertGreater(expected[0], 2**63)
        self.assertLess(expected[0], 2**64)

    def test_reconstruction_cannot_overflow_u128_for_arbitrary_u64_outputs(self):
        maximum_u64 = 2**64 - 1
        reconstructed = maximum_u64 * 2**63 + maximum_u64
        self.assertEqual(reconstructed, 2**127 + 2**63 - 1)
        self.assertLess(reconstructed, 2**128)

    def test_three_controls_discriminate_success_exact_value_and_error_classification(self):
        self.assertEqual(rational_expectation(1, 0), (1, 0, 0))  # reject-all is wrong
        self.assertEqual(rational_expectation(3, -1), (1, 1, 1))
        self.assertTrue(0 <= Fraction(0, 1) < 1)  # wrong residual still has valid form
        self.assertNotEqual(Fraction(3, 2), 1 + Fraction(0, 1))
        self.assertEqual(rational_expectation(2, -1), (1, 0, 0))  # valid zero-residual neighbor
        self.assertIsNone(rational_expectation(0, 12))  # Invariant, not Unrepresentable
        self.assertEqual(rational_expectation(0, 11), (0, 0, 0))


if __name__ == "__main__":
    unittest.main()
