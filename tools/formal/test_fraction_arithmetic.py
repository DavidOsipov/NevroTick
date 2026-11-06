"""Prepare actual arithmetic sources; validate spec facts without Rust execution."""
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
SPEC = importlib.util.spec_from_file_location("fraction_arithmetic_projection_test", HERE / "fraction_arithmetic_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-fraction-arithmetic-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "fraction_arithmetic_harness.rs"

    def prepare(self, subject="fraction_complement", case="healthy", folder=None):
        return PROJECTION.prepare_fraction_arithmetic(self.source, folder or self.inputs,
                                                     subject, case, self.harness)

    def spans(self):
        return {name: (self.core / span["source"]).read_bytes()[slice(*span["offsets"])]
                for name, span in PROJECTION.SPANS.items()}

    def test_nine_actual_spans_preserved_and_assertions_lexically_in_private_entrypoint(self):
        result = self.prepare()
        definitions = (self.inputs / "fraction_arithmetic_selected.rs").read_bytes()
        proof = (self.inputs / "proof.rs").read_bytes()
        for name, source in self.spans().items():
            with self.subTest(name=name):
                self.assertEqual(definitions.count(source), 1)
        for forbidden in (b"pub struct Fraction", b"pub fn new", b"pub fn complement",
                          b"pub fn add_fractions", b"impl fmt::", b"impl std::error::Error", b"use garde"):
            self.assertNotIn(forbidden, definitions)
        self.assertIn(b"\nstruct FractionSum {", definitions)
        self.assertEqual(proof, b'#![allow(dead_code)]\nmod fraction_arithmetic {\n'
                         b'include!("fraction_arithmetic_selected.rs");\n' + self.harness.read_bytes() + b"\n}\n")
        self.assertTrue(result["original_helper_preserved"])
        self.assertTrue(result["original_callees_and_other_arithmetic_preserved"])
        self.assertEqual(result["harness_location"], "proof.rs")
        self.assertEqual(result["assumptions"], [])
        self.assertIsNone(result["mutation"])
        self.assertIn("canonical M>=128", result["trusted_high_denominator_specification"])
        self.assertEqual(result["compiled_helper_sha256"], PROJECTION.SPANS["complement"]["sha256"])
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions))
        self.assertEqual(result["entrypoint_sha256"], PROJECTION.digest(proof))
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        self.assertEqual(len(result["correspondence"]), 9)
        for item in result["correspondence"]:
            original = (self.source / item["source"]).read_bytes()
            selected = original[slice(*item["byte_offsets_half_open"])]
            self.assertEqual(original.count(selected), 1)
            self.assertEqual(PROJECTION.digest(selected), item["sha256"])
        for harness in PROJECTION.REQUIRED_ASSERTIONS:
            for label in (*PROJECTION.REQUIRED_ASSERTIONS[harness], *PROJECTION.REQUIRED_COVERS[harness]):
                literal = ('"' + label + '"').encode()
                self.assertIn(literal, proof)
                self.assertNotIn(literal, definitions)

    def test_all_six_mutations_change_only_their_selected_actual_method(self):
        originals = self.spans()
        for subject, controls in PROJECTION.CONTROLS.items():
            owner = "complement" if subject == "fraction_complement" else "addition"
            for case, control in controls.items():
                with self.subTest(subject=subject, case=case):
                    folder = self.base / (subject + "-" + case)
                    result = self.prepare(subject, case, folder)
                    before, after = control["before"].encode(), control["after"].encode()
                    self.assertEqual(originals[owner].count(before), 1)
                    changed = originals[owner].replace(before, after, 1)
                    definitions = (folder / "fraction_arithmetic_selected.rs").read_bytes()
                    self.assertEqual(definitions.count(changed), 1)
                    self.assertNotIn(originals[owner], definitions)
                    for name, source in originals.items():
                        if name != owner:
                            self.assertEqual(definitions.count(source), 1)
                    self.assertEqual(result["selected_arithmetic_owner"], owner)
                    self.assertEqual(result["compiled_helper_sha256"], PROJECTION.digest(changed))
                    self.assertEqual(result["mutation"], control)
                    self.assertFalse(result["original_helper_preserved"])
                    self.assertIn(('"' + control["intended_failure"] + '"').encode(),
                                  (folder / "proof.rs").read_bytes())

    def test_complete_source_and_selected_span_drift_fail_before_output_creation(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.core / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// drift\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes().replace(b"carry: comparison != Ordering::Less,",
                                           b"carry: comparison == Ordering::Less,", 1)
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "source span changed: addition"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_context_rejected_even_with_matching_complete_hash(self):
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes() + b"\n" + PROJECTION.SPANS["addition"]["following"]
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous: addition"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_subject_cross_subject_case_and_existing_output_custody(self):
        for subject, case in (("fraction_division", "healthy"), ("fraction_complement", "wrong_carry"),
                              ("fraction_addition", "wrong_error")):
            with self.subTest(subject=subject, case=case), self.assertRaisesRegex(ValueError, "Unknown"):
                self.prepare(subject, case)
            self.assertFalse(self.inputs.exists())
        for name in ("proof.rs", "fraction_arithmetic_selected.rs", "correspondence.json"):
            for symlink in (False, True):
                folder = self.base / (name + ("-link" if symlink else "-file"))
                folder.mkdir()
                retained = folder / name
                if symlink:
                    retained.symlink_to(folder / "absent")
                else:
                    retained.write_bytes(b"retained evidence")
                with self.subTest(name=name, symlink=symlink), self.assertRaisesRegex(ValueError, "overwrite"):
                    self.prepare(folder=folder)
                self.assertEqual(list(folder.iterdir()), [retained])
                if symlink:
                    self.assertTrue(retained.is_symlink())
                else:
                    self.assertEqual(retained.read_bytes(), b"retained evidence")

    def test_ten_jobs_restore_identical_artifacts_for_both_subjects(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 10)
        self.assertEqual([job["subject"] for job in jobs], ["fraction_complement"] * 5 + ["fraction_addition"] * 5)
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "always_error", "wrong_numerator", "wrong_error", "healthy",
                          "healthy", "always_error", "wrong_carry", "wrong_residual", "healthy"])
        for index, job in enumerate(jobs):
            self.prepare(job["subject"], job["case"], self.base / str(index))
        for first, last in ((0, 4), (5, 9)):
            self.assertEqual(jobs[first]["harness"], jobs[last]["harness"])
            self.assertFalse(jobs[first]["restored_healthy"])
            self.assertTrue(jobs[last]["restored_healthy"])
            self.assertEqual(len({job["intended_failure"] for job in jobs[first + 1:last]}), 3)
            for name in ("proof.rs", "fraction_arithmetic_selected.rs", "correspondence.json"):
                self.assertEqual((self.base / str(first) / name).read_bytes(), (self.base / str(last) / name).read_bytes())

    def test_full_raw_factories_and_canonical_field_oracles_without_callee_stubs(self):
        harness = self.harness.read_text()
        self.assertEqual(harness.count("kani::any::<u64>()"), 3)
        self.assertEqual(harness.count("kani::any::<u32>()"), 3)
        self.assertEqual(harness.count("#[kani::proof]"), 8)
        for forbidden in ("kani::assume", "Fraction {", ".unwrap", "Result<FractionSum"):
            self.assertNotIn(forbidden, harness)
        self.assertIn("let maximum = left.shift.max(right.shift);", harness)
        self.assertIn("left_scaled >> left_gap == u128::from(left.numerator)", harness)
        self.assertIn("left_scaled <= u128::MAX - right_scaled", harness)
        oracle = harness[harness.index("fn addition_rational_oracle"):harness.index("fn addition_contract")]
        self.assertNotIn(".compare(", oracle)
        self.assertNotIn(".complement(", oracle)
        self.assertNotIn("add_fractions(", oracle)
        self.assertIn("Trusted reviewed specification correspondence", oracle)
        sizes = [(len(PROJECTION.REQUIRED_ASSERTIONS[key]), len(PROJECTION.REQUIRED_COVERS[key]))
                 for key in PROJECTION.REQUIRED_ASSERTIONS]
        self.assertEqual(sizes, [(7, 13), (11, 23)])


def canonical_raw(n, s):
    """Specification constructor, used only for independent rational facts."""
    if s > 1041 or n.bit_length() > 53 or n and n.bit_length() > s:
        return None
    return (0, 0) if n == 0 else (n, s)


def addition_oracle(left, right):
    n, s = left
    m, t = right
    maximum = max(s, t)
    if maximum >= 128:
        return False, True
    denominator = 2**maximum
    total = n * 2**(maximum - s) + m * 2**(maximum - t)
    return total >= denominator, total != 0 and total != denominator


class IndependentSpecificationFacts(unittest.TestCase):
    def representations(self):
        a, b = 2**52, 2**53 - 1
        raw = [(0, 0), (0, 1041), (1, 1), (2, 2), (1, 2), (3, 2), (5, 3),
               (a - 1, 53), (a, 53), (a + 1, 53), (b, 53), (b, 54), (b, 127),
               (b, 128), (1, 1041), (b, 1041)]
        return raw

    def test_complement_contract_and_thirteen_cover_witnesses(self):
        a, b = 2**52, 2**53 - 1
        witnesses = [((0, 0), None), ((1, 2), None), ((a - 1, 53), None), ((b, 54), None),
                     ((1, 1041), None), ((b, 1041), None), ((1, 1), (1, 1)), ((2, 2), (2, 2)),
                     ((a, 53), (a, 53)), ((a + 1, 53), (a - 1, 53)), ((b, 53), (1, 53)),
                     ((3, 2), (1, 2)), ((5, 3), (3, 3))]
        self.assertEqual(len(witnesses), 13)
        for raw, expected in witnesses:
            n, s = canonical_raw(*raw)
            rational = Fraction(n, 2**s)
            required = 1 <= s <= 53 and 2 * n >= 2**s
            self.assertEqual(required, rational >= Fraction(1, 2))
            self.assertEqual(required, expected is not None)
            if required:
                output = (2**s - n, s)
                self.assertEqual(output, expected)
                self.assertTrue(1 <= output[0] <= a and output[0] < 2**s)
                self.assertEqual(Fraction(output[0], 2**output[1]) + rational, 1)

    def test_exact_sum_alignment_width_and_large_denominator_bound_against_rationals(self):
        maximum_fraction = Fraction(2**53 - 1, 2**53)
        for raw_left in self.representations():
            for raw_right in self.representations():
                left, right = canonical_raw(*raw_left), canonical_raw(*raw_right)
                n, s = left
                m, t = right
                x, y = Fraction(n, 2**s), Fraction(m, 2**t)
                exact = x + y
                self.assertEqual(addition_oracle(left, right), (exact >= 1, exact.denominator != 1))
                self.assertTrue(x <= maximum_fraction and y <= maximum_fraction)
                maximum = max(s, t)
                if maximum < 128:
                    denominator = 2**maximum
                    left_scaled, right_scaled = n << (maximum - s), m << (maximum - t)
                    self.assertTrue(left_scaled < denominator and right_scaled < denominator)
                    self.assertEqual(left_scaled >> (maximum - s), n)
                    self.assertEqual(right_scaled >> (maximum - t), m)
                    self.assertLess(left_scaled + right_scaled, 2**128)
                else:
                    high = x if s >= 128 else y
                    self.assertTrue(0 < high < Fraction(1, 2**75))
                    self.assertTrue(0 < exact < 1)

    def test_twenty_three_addition_covers_and_raw_zero_canonicalization(self):
        a, b = 2**52, 2**53 - 1
        witnesses = [((0, 0, 0, 0), (False, False)), ((1, 2, 0, 0), (False, True)),
                     ((0, 0, 1, 2), (False, True)), ((1, 2, 1, 2), (False, True)),
                     ((1, 1, 1, 1), (True, False)), ((1, 2, 3, 2), (True, False)),
                     ((3, 2, 1, 2), (True, False)), ((3, 2, 1, 1), (True, True)),
                     ((1, 1, 3, 2), (True, True)), ((2, 2, 2, 2), (True, False)),
                     ((1, 1, a - 1, 53), (False, True)), ((1, 1, a + 1, 53), (True, True)),
                     ((b, 53, b, 53), (True, True)), ((b, 53, b, 127), (False, True)),
                     ((b, 127, b, 53), (False, True)), ((b, 53, b, 128), (False, True)),
                     ((b, 128, b, 53), (False, True)), ((1, 1041, 0, 0), (False, True)),
                     ((0, 0, 1, 1041), (False, True)), ((1, 1041, 1, 1041), (False, True)),
                     ((0, 1041, 0, 1041), (False, False)), ((1, 1, 1, 2), (False, True)),
                     ((1, 2, 1, 1), (False, True))]
        self.assertEqual(len(witnesses), 23)
        for (n, s, m, t), expected in witnesses:
            left, right = canonical_raw(n, s), canonical_raw(m, t)
            exact = Fraction(left[0], 2**left[1]) + Fraction(right[0], 2**right[1])
            self.assertEqual((exact >= 1, exact.denominator != 1), expected)
            self.assertEqual(addition_oracle(left, right), expected)
        self.assertEqual(canonical_raw(0, 1041), (0, 0))
        self.assertEqual(addition_oracle(canonical_raw(0, 1041), canonical_raw(0, 1041)), (False, False))
        self.assertNotEqual(addition_oracle((0, 1041), (0, 1041)), (False, False))

    def test_six_semantic_fault_witnesses_violate_distinct_promised_observations(self):
        self.assertGreaterEqual(Fraction(1, 2), Fraction(1, 2))  # reject-all loses complement success
        self.assertEqual(Fraction(5, 8) + Fraction(3, 8), 1)
        self.assertNotEqual(Fraction(5, 8) + Fraction(1, 8), 1)
        self.assertLess(Fraction(1, 4), Fraction(1, 2))  # specified Invariant rejection
        self.assertEqual(Fraction(0) + Fraction(0), 0)  # addition must still succeed
        self.assertEqual(Fraction(1, 2) + Fraction(1, 2), 1)  # equality carries
        self.assertEqual((Fraction(1, 2) + Fraction(1, 4)).denominator, 4)  # below-unit residual retained


if __name__ == "__main__":
    unittest.main()
