"""Test actual-source preparation and independent references; run no Rust/solver."""
from __future__ import annotations

from fractions import Fraction
import importlib.util
import json
import math
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("scaled_value_representation_projection_test",
                                             HERE / "scaled_value_representation_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)
FROM_BITS_PATTERNS = (
    0x0000000000000000, 0x8000000000000000, 0x0000000000000001, 0x8000000000000001,
    0x000fffffffffffff, 0x0010000000000000, 0x3dd0000000000000, 0x3ddfffffffffffff,
    0x3de0000000000000, 0x3de0000000000001, 0x3d30000000000000, 0x3d20000000000000,
    0x41d0000000000000, 0xc1d0000000000000, 0x41cfffffffffffff, 0x41d0000000000001,
    0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000001, 0xfff0000000000001,
)


def ieee_integer_specification(bits):
    """Unbounded mathematical format definition, not execution of the Rust body."""
    magnitude = bits & (2**63 - 1)
    if magnitude > 0x41d0000000000000:
        return None
    exponent, mantissa = divmod(magnitude, 2**52)
    numerator = mantissa + (2**52 if exponent else 0)
    k = 1042 - max(exponent, 1)
    if k <= 0:
        whole, remainder, shift = numerator * 2**-k, 0, 0
    else:
        whole, remainder = divmod(numerator, 2**k)
        shift = k if remainder else 0
    return bool(bits >> 63), whole, remainder, shift


def binary64_rational_reference(bits):
    """Independent Python binary64 decoder and exact-rational reference.

    frexp supplies the normal-value exponent without copying the integer bit
    decoder; subnormal format uses the fixed original 2^-1074 quantum. This
    preserves original unreduced denominator exponents, including half-quantum.
    """
    value = struct.unpack(">d", bits.to_bytes(8, "big"))[0]
    if not math.isfinite(value) or abs(value) > 2**30:
        return None
    exact = Fraction.from_float(abs(value)) * 2**33
    whole = exact.numerator // exact.denominator
    residual = exact - whole
    if residual == 0:
        numerator, shift = 0, 0
    else:
        shift = 1041 if abs(value) < sys.float_info.min else 20 - math.frexp(abs(value))[1]
        original_numerator = residual * 2**shift
        if original_numerator.denominator != 1:
            raise AssertionError("Expected an integral dyadic numerator")
        numerator = original_numerator.numerator
    return math.copysign(1.0, value) < 0, whole, numerator, shift


def signed_witnesses():
    """Twenty distinct raw private-payload witnesses, not product admission."""
    whole_values = (0, 1, 2, 2**63 - 1, 2**63, 2**63 + 1, 2**64 - 2, 2**64 - 1)
    cases = [(whole, negative, 0, 0) for whole in whole_values for negative in (False, True)]
    return cases + [(0, False, 1, 1), (0, True, 1, 1),
                    (1, False, 2**64 - 1, 0), (1, True, 0, 2**32 - 1)]


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-scaled-representation-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "scaled_value_representation_harness.rs"

    def prepare(self, subject="signed_whole", case="healthy", inputs=None):
        return PROJECTION.prepare_scaled_value_representation(
            self.source, inputs or self.inputs, subject, case, self.harness)

    def original_spans(self):
        return {name: (self.core / span["source"]).read_bytes()[slice(*span["offsets"])]
                for name, span in PROJECTION.SPANS.items()}

    def test_healthy_preserves_all_ten_actual_owners_for_both_subjects(self):
        for subject in PROJECTION.SUBJECTS:
            folder = self.base / subject
            result = self.prepare(subject, inputs=folder)
            definitions = (folder / "scaled_value_representation_selected.rs").read_bytes()
            for name, span in self.original_spans().items():
                with self.subTest(subject=subject, owner=name):
                    self.assertEqual(definitions.count(span), 1)
            for absent in (b"pub struct Fraction", b"pub struct ScaledValue", b"pub fn from_bits",
                           b"pub fn signed_whole", b"impl fmt::", b"use garde", b"fn compare"):
                self.assertNotIn(absent, definitions)
            self.assertTrue(result["original_helper_preserved"])
            self.assertTrue(result["original_other_owners_preserved"])
            self.assertEqual(result["raw_private_payload_extension"], subject == "signed_whole")
            self.assertIsNone(result["mutation"])
            self.assertEqual(result["assumptions"], [])
            self.assertEqual(result["compiled_helper_sha256"], PROJECTION.SPANS[subject]["sha256"])
            self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions))
            self.assertEqual(json.loads((folder / "correspondence.json").read_text()),
                             {**result, "entrypoint": "proof.rs"})

    def test_entrypoint_keeps_all_eight_harnesses_and_obligations_inside_private_module(self):
        result = self.prepare()
        proof = (self.inputs / "proof.rs").read_bytes()
        definitions = (self.inputs / "scaled_value_representation_selected.rs").read_bytes()
        harness = self.harness.read_bytes()
        self.assertEqual(proof, b'#![allow(dead_code)]\nmod scaled_value_representation {\n'
                         b'include!("scaled_value_representation_selected.rs");\n' + harness + b"\n}\n")
        self.assertEqual(proof.count(b"#[kani::proof]"), 8)
        self.assertNotIn(b"#[kani::proof]", definitions)
        self.assertTrue(result["harness_inside_private_module"])
        self.assertEqual(result["harness_location"], "proof.rs")
        self.assertEqual(result["harness_sha256"], PROJECTION.digest(harness))
        self.assertEqual(result["entrypoint_sha256"], PROJECTION.digest(proof))
        for subject in PROJECTION.SUBJECTS:
            full = PROJECTION.FULL_HARNESSES[subject]
            labels = (*PROJECTION.REQUIRED_ASSERTIONS[full], *PROJECTION.REQUIRED_COVERS[full],
                      *(c["intended_failure"] for c in PROJECTION.CONTROLS[subject].values()))
            for label in labels:
                literal = ('"' + label + '"').encode()
                self.assertIn(literal, proof)
                self.assertNotIn(literal, definitions)

    def test_correspondence_records_exact_unique_ten_source_spans(self):
        result = self.prepare()
        self.assertEqual(len(result["correspondence"]), 10)
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)
        self.assertEqual({row["item"] for row in result["correspondence"]}, set(PROJECTION.SPANS))
        for row in result["correspondence"]:
            original = (self.source / row["source"]).read_bytes()
            selected = original[slice(*row["byte_offsets_half_open"])]
            self.assertEqual(original.count(selected), 1)
            self.assertEqual(PROJECTION.digest(selected), row["sha256"])

    def test_each_of_six_controls_changes_only_its_selected_method(self):
        spans = self.original_spans()
        for subject in PROJECTION.SUBJECTS:
            for case, control in PROJECTION.CONTROLS[subject].items():
                folder = self.base / subject / case
                result = self.prepare(subject, case, folder)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(spans[subject].count(before), 1)
                changed = spans[subject].replace(before, after, 1)
                definitions = (folder / "scaled_value_representation_selected.rs").read_bytes()
                self.assertEqual(definitions.count(changed), 1)
                self.assertNotIn(spans[subject], definitions)
                for name, span in spans.items():
                    if name != subject:
                        self.assertEqual(definitions.count(span), 1)
                self.assertEqual(result["mutation"], control)
                self.assertEqual(result["compiled_helper_sha256"], PROJECTION.digest(changed))
                self.assertEqual(result["compiled_methods_sha256"][subject], PROJECTION.digest(changed))
                self.assertFalse(result["original_helper_preserved"])

    def test_complete_source_drift_rejects_before_output_creation(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.core / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// source drift\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_each_span_guard_rejects_drift_even_with_rebound_complete_digest(self):
        for name, span in PROJECTION.SPANS.items():
            path = self.core / span["source"]
            original = path.read_bytes()
            location = span["offsets"][1] - 2
            changed = original[:location] + bytes((original[location] ^ 1,)) + original[location + 1:]
            path.write_bytes(changed)
            with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
                with self.subTest(name=name), self.assertRaisesRegex(ValueError, "source span changed: " + name):
                    self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)

    def test_ambiguous_extraction_and_missing_mutation_site_reject_without_outputs(self):
        path = self.core / "projection_enclosure.rs"
        original = path.read_bytes()
        changed = original + b"\n" + PROJECTION.SPANS["from_bits"]["start"]
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous: from_bits"):
                self.prepare()
        self.assertFalse(self.inputs.exists())
        path.write_bytes(original)
        control = PROJECTION.CONTROLS["signed_whole"]["omit_negation"]
        with patch.dict(control, {"before": "nonexistent mutation site"}):
            with self.assertRaisesRegex(ValueError, "mutation point is not unique"):
                self.prepare("signed_whole", "omit_negation")
        self.assertFalse(self.inputs.exists())

    def test_unknown_subject_case_and_existing_or_symlink_outputs_are_rejected(self):
        for subject, case in (("other", "healthy"), ("from_bits", "omit_negation"), ("signed_whole", "lose_sign")):
            with self.subTest(subject=subject, case=case), self.assertRaisesRegex(ValueError, "Unknown"):
                self.prepare(subject, case)
        self.assertFalse(self.inputs.exists())
        for name in ("proof.rs", "scaled_value_representation_selected.rs", "correspondence.json"):
            for symlink in (False, True):
                folder = self.base / (name + ("-link" if symlink else "-file"))
                folder.mkdir()
                retained = folder / name
                if symlink:
                    retained.symlink_to(folder / "absent")
                else:
                    retained.write_bytes(b"retained evidence")
                with self.assertRaisesRegex(ValueError, "overwrite"):
                    self.prepare(inputs=folder)
                self.assertEqual(list(folder.iterdir()), [retained])
                if symlink:
                    self.assertTrue(retained.is_symlink())
                else:
                    self.assertEqual(retained.read_bytes(), b"retained evidence")

    def test_redirected_input_directory_cannot_write_to_its_target(self):
        target = self.base / "protected"
        target.mkdir()
        self.inputs.symlink_to(target, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "directory redirects"):
            self.prepare()
        self.assertEqual(list(target.iterdir()), [])

    def test_ten_jobs_keep_two_independent_complete_groups_and_exact_restoration(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 10)
        self.assertEqual([row["subject"] for row in jobs], ["signed_whole"] * 5 + ["from_bits"] * 5)
        self.assertEqual([row["case"] for row in jobs], [
            "healthy", "omit_negation", "narrow_magnitude", "use_fraction", "healthy",
            "healthy", "wrong_subnormal_scale", "lose_sign", "wrong_error", "healthy"])
        self.assertEqual([i for i, row in enumerate(jobs, 1) if row["restored_healthy"]], [5, 10])
        for group in (jobs[:5], jobs[5:]):
            self.assertEqual(len({row["intended_failure"] for row in group[1:4]}), 3)
        for index, row in enumerate(jobs):
            self.prepare(row["subject"], row["case"], self.base / str(index))
        for first, restored in ((0, 4), (5, 9)):
            for name in ("proof.rs", "scaled_value_representation_selected.rs", "correspondence.json"):
                self.assertEqual((self.base / str(first) / name).read_bytes(),
                                 (self.base / str(restored) / name).read_bytes())
        # Subject selection does not choose different executable definitions.
        self.assertEqual((self.base / "0/scaled_value_representation_selected.rs").read_bytes(),
                         (self.base / "5/scaled_value_representation_selected.rs").read_bytes())

    def test_raw_domains_and_safe_signed_oracle_are_explicit_without_assumptions(self):
        text = self.harness.read_text()
        self.assertNotIn("kani::assume", text)
        self.assertNotIn("MAX_SCALED_MAGNITUDE", text)
        self.assertNotIn("f64::", text)
        self.assertNotIn(".abs()", text)
        self.assertNotIn("k < 53", text)
        self.assertIn("observed.unsigned_abs()", text)
        self.assertIn("let k = 1042_i64 - effective_exponent;", text)
        self.assertIn("} else if k < 64 {", text)
        self.assertLess(text.index("if required_success {"), text.index("let k = 1042_i64"))
        self.assertEqual(text.count("kani::any::<u64>()"), 3)
        self.assertEqual(text.count("kani::any::<bool>()"), 1)
        self.assertEqual(text.count("kani::any::<u32>()"), 1)
        self.assertEqual(text.count("kani::cover!("), 40)
        for subject, assertions in (("signed_whole", 5), ("from_bits", 15)):
            full = PROJECTION.FULL_HARNESSES[subject]
            self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS[full]), assertions)
            self.assertEqual(len(PROJECTION.REQUIRED_COVERS[full]), 20)
            for label in PROJECTION.REQUIRED_COVERS[full]:
                self.assertEqual(text.count('"' + label + '"'), 1)


class IndependentSpecificationFacts(unittest.TestCase):
    """These reference observations are not executed product proofs."""

    def test_ieee_integer_oracle_matches_independent_binary64_rationals_at_neighbors(self):
        exponents = (0, 1, 2, 977, 978, 979, 988, 989, 990, 1041, 1042, 1052, 1053, 1054, 2046, 2047)
        mantissas = (0, 1, 2, 2**51 - 1, 2**51, 2**52 - 2, 2**52 - 1)
        for exponent in exponents:
            for mantissa in mantissas:
                for negative in (False, True):
                    bits = (int(negative) << 63) | (exponent << 52) | mantissa
                    with self.subTest(bits=f"{bits:016x}"):
                        expected = binary64_rational_reference(bits)
                        self.assertEqual(ieee_integer_specification(bits), expected)
                        if expected is None:
                            continue
                        sign, whole, numerator, shift = expected
                        self.assertEqual(sign, negative)
                        self.assertLessEqual(whole, 2**63)
                        self.assertLess(numerator, 2**53)
                        self.assertTrue(numerator == 0 and shift == 0
                                        or numerator != 0 and 1 <= shift <= 1041)
                        self.assertTrue(0 <= Fraction(numerator, 2**shift) < 1)

    def test_forty_witnesses_are_distinct_and_have_the_declared_outcomes(self):
        self.assertEqual(len(set(FROM_BITS_PATTERNS)), 20)
        admitted = 0
        for bits in FROM_BITS_PATTERNS:
            expected = binary64_rational_reference(bits)
            self.assertEqual(ieee_integer_specification(bits), expected)
            admitted += expected is not None
        self.assertEqual(admitted, 15)
        witnesses = signed_witnesses()
        self.assertEqual(len(witnesses), 20)
        self.assertEqual(len(set(witnesses)), 20)
        for whole, negative, numerator, shift in witnesses:
            self.assertTrue(0 <= numerator < 2**64 and 0 <= shift < 2**32)
            expected = (-1 if negative else 1) * whole
            self.assertEqual(abs(expected), whole)
            self.assertEqual(expected < 0, negative and whole != 0)
            self.assertEqual(expected == 0, whole == 0)
            self.assertTrue(-(2**64 - 1) <= expected <= 2**64 - 1)

    def test_half_quantum_keeps_unreduced_fraction_and_boundaries_keep_i128_range(self):
        self.assertEqual(binary64_rational_reference(0x3dd0000000000000), (False, 0, 2**52, 53))
        positive_boundary = binary64_rational_reference(0x41d0000000000000)
        self.assertEqual(positive_boundary, (False, 2**63, 0, 0))
        self.assertEqual(binary64_rational_reference(0xc1d0000000000000), (True, 2**63, 0, 0))
        self.assertIsNone(binary64_rational_reference(0x41d0000000000001))
        self.assertGreater(positive_boundary[1], 2**63 - 1)  # needs i128, not i64

    def test_oracle_bounds_remain_safe_for_arbitrary_faulty_output_fields(self):
        reconstruction = (2**64 - 1) * 2**63 + (2**64 - 1)
        self.assertEqual(reconstruction, 2**127 + 2**63 - 1)
        self.assertLess(reconstruction, 2**128)
        self.assertLess(abs(-2**127), 2**128)  # unsigned_abs handles faulty i128::MIN
        self.assertGreater(abs(-2**127), 2**64 - 1)
        for exponent in (0, 1, 1041, 1042, 1052, 1053):
            k = 1042 - max(exponent, 1)
            self.assertTrue(-11 <= k <= 1041)

    def test_control_witnesses_challenge_sign_magnitude_independence_and_scaling(self):
        expected_negative = -1
        missing_negation = 1
        self.assertEqual(abs(expected_negative), abs(missing_negation))
        self.assertNotEqual(expected_negative < 0, missing_negation < 0)
        self.assertNotEqual(abs(-1), 2**64 - 1)  # u64::MAX narrowed through i64
        self.assertNotEqual(0, 0 + 1)  # fractional numerator contaminates the whole
        self.assertEqual(ieee_integer_specification(1), (False, 0, 1, 1041))
        self.assertNotEqual(Fraction(1, 2**1041), Fraction(1, 2**1040))
        negative_zero = ieee_integer_specification(0x8000000000000000)
        self.assertEqual(negative_zero, (True, 0, 0, 0))
        self.assertNotEqual(negative_zero, (False, 0, 0, 0))
        self.assertIsNone(ieee_integer_specification(0x41d0000000000001))


if __name__ == "__main__":
    unittest.main()
