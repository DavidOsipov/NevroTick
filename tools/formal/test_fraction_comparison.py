"""Check comparison proof preparation and rational facts; execute no Rust."""
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
SPEC = importlib.util.spec_from_file_location("fraction_comparison_projection_test", HERE / "fraction_comparison_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class ProjectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-fraction-comparison-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        for name in PROJECTION.SOURCE_SHA256:
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "fraction_comparison_harness.rs"

    def prepare(self, case="healthy", folder=None):
        return PROJECTION.prepare_fraction_comparison(self.source, folder or self.inputs, case, self.harness)

    def spans(self):
        return {name: (self.core / span["source"]).read_bytes()[slice(*span["offsets"])]
                for name, span in PROJECTION.SPANS.items()}

    def test_exact_private_sources_and_lexically_qualified_harness(self):
        result = self.prepare()
        definitions = (self.inputs / "fraction_comparison_selected.rs").read_bytes()
        proof = (self.inputs / "proof.rs").read_bytes()
        for name, source in self.spans().items():
            with self.subTest(name=name):
                self.assertEqual(definitions.count(source), 1)
        self.assertIn(b"\nstruct Fraction {", definitions)
        for forbidden in (b"pub struct Fraction", b"pub fn new", b"pub fn compare", b"impl fmt::",
                          b"impl std::error::Error", b"use garde", b"fn complement"):
            self.assertNotIn(forbidden, definitions)
        self.assertEqual(proof, b'#![allow(dead_code)]\nmod fraction_comparison {\n'
                         b'include!("fraction_comparison_selected.rs");\n' + self.harness.read_bytes() + b"\n}\n")
        self.assertTrue(result["original_helper_preserved"])
        self.assertTrue(result["original_struct_zero_constructor_and_enums_preserved"])
        self.assertTrue(result["harness_inside_private_module"])
        self.assertEqual(result["harness_location"], "proof.rs")
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertEqual(result["compiled_helper_sha256"], PROJECTION.SPANS["compare"]["sha256"])
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions))
        self.assertEqual(result["entrypoint_sha256"], PROJECTION.digest(proof))
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        self.assertEqual(len(result["correspondence"]), 6)
        for item in result["correspondence"]:
            original = (self.source / item["source"]).read_bytes()
            source = original[slice(*item["byte_offsets_half_open"])]
            self.assertEqual(original.count(source), 1)
            self.assertEqual(PROJECTION.digest(source), item["sha256"])
        for label in (*PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS],
                      *PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS],
                      *(control["intended_failure"] for control in PROJECTION.CONTROLS.values())):
            literal = ('"' + label + '"').encode()
            self.assertIn(literal, proof)
            self.assertNotIn(literal, definitions)

    def test_each_fault_changes_only_one_compare_block_and_preserves_actual_factory(self):
        originals = self.spans()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = self.prepare(case, folder)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(originals["compare"].count(before), 1)
                changed = originals["compare"].replace(before, after, 1)
                definitions = (folder / "fraction_comparison_selected.rs").read_bytes()
                self.assertEqual(definitions.count(changed), 1)
                self.assertNotIn(originals["compare"], definitions)
                for name, source in originals.items():
                    if name != "compare":
                        self.assertEqual(definitions.count(source), 1)
                self.assertFalse(result["original_helper_preserved"])
                self.assertEqual(result["compiled_helper_sha256"], PROJECTION.digest(changed))
                self.assertEqual(result["mutation"], control)

    def test_complete_source_drift_and_changed_span_fail_before_output(self):
        for name in PROJECTION.SOURCE_SHA256:
            path = self.core / name
            original = path.read_bytes()
            path.write_bytes(original + b"\n// changed\n")
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "complete source changed"):
                self.prepare()
            self.assertFalse(self.inputs.exists())
            path.write_bytes(original)
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes().replace(b"unequal => Ok(unequal),", b"unequal => Ok(unequal.reverse()),", 1)
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "source span changed: compare"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_context_rejected_even_with_matching_complete_source_hash(self):
        path = self.core / "projection_enclosure.rs"
        changed = path.read_bytes() + b"\n" + PROJECTION.SPANS["compare"]["following"]
        path.write_bytes(changed)
        with patch.dict(PROJECTION.SOURCE_SHA256, {path.name: PROJECTION.digest(changed)}):
            with self.assertRaisesRegex(ValueError, "extraction is ambiguous: compare"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_control_and_existing_or_symlink_output_custody(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("always_equal")
        self.assertFalse(self.inputs.exists())
        for name in ("proof.rs", "fraction_comparison_selected.rs", "correspondence.json"):
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

    def test_five_jobs_and_healthy_restoration_are_identical(self):
        jobs = PROJECTION.schedule()
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "always_error", "reverse_unequal", "zero_alignment", "healthy"])
        self.assertEqual(jobs[0]["harness"], jobs[-1]["harness"])
        self.assertFalse(jobs[0]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        self.assertEqual(len({job["intended_failure"] for job in jobs[1:-1]}), 3)
        for index, job in enumerate(jobs):
            self.prepare(job["case"], self.base / str(index))
        for name in ("proof.rs", "fraction_comparison_selected.rs", "correspondence.json"):
            self.assertEqual((self.base / "0" / name).read_bytes(), (self.base / "4" / name).read_bytes())

    def test_raw_factories_independent_oracle_and_explicit_nontruncation_lemma(self):
        harness = self.harness.read_text()
        self.assertEqual(harness.count("kani::any::<u64>()"), 2)
        self.assertEqual(harness.count("kani::any::<u32>()"), 2)
        self.assertNotIn("kani::assume", harness)
        self.assertNotIn("Fraction {", harness)
        self.assertNotIn(".unwrap", harness)
        self.assertIn("Fraction::new(n, s)", harness)
        self.assertIn("Fraction::new(m, t)", harness)
        self.assertEqual(harness.count("#[kani::proof]"), 4)
        oracle = harness[harness.index("fn comparison_rational_oracle"):harness.index("fn comparison_source_machine_bounds")]
        self.assertNotIn("leading_zeros", oracle)
        self.assertNotIn("checked_shl", oracle)
        self.assertIn("if gap < 64", oracle)
        self.assertIn("1_u128 << 116", oracle)
        self.assertEqual(harness.count("machine.map(u128::from) == Some(mathematical)"), 2)
        self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]), 12)
        self.assertEqual(len(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]), 20)


def order(left, right):
    return (left > right) - (left < right)


def rational_oracle(n, s, m, t):
    """Python rendition of the spec only; never substitutes for actual Rust."""
    if n == 0 or m == 0:
        return order(n, m)
    if s == t:
        return order(n, m)
    if s < t:
        gap = t - s
        return order(n << gap, m) if gap < 64 else 1
    gap = s - t
    return order(n, m << gap) if gap < 64 else -1


class IndependentSpecificationFacts(unittest.TestCase):
    def representations(self):
        a, b = 2**52, 2**53 - 1
        numerators = [0, 1, 2, 3, a - 1, a, a + 1, b]
        shifts = [0, 1, 2, 3, 52, 53, 54, 63, 64, 65, 116, 117, 1041]
        return [(n, s) for n in numerators for s in shifts
                if (n == 0 and s == 0) or 0 < n < 2**53 and n < 2**s]

    def test_cross_scaling_and_machine_lemmas_against_unbounded_rationals(self):
        for n, s in self.representations():
            for m, t in self.representations():
                expected = order(Fraction(n, 2**s), Fraction(m, 2**t))
                self.assertEqual(rational_oracle(n, s, m, t), expected)
                if n == 0 or m == 0:
                    continue
                bits, other_bits = n.bit_length(), m.bit_length()
                self.assertTrue(1 <= bits <= 53 and 1 <= other_bits <= 53)
                leading, other_leading = bits - s, other_bits - t
                self.assertTrue(-1040 <= leading <= 0 and -1040 <= other_leading <= 0)
                self.assertLessEqual(abs(s - t), 1040)
                if abs(s - t) < 64:
                    self.assertLess((n if s < t else m) << abs(s - t), 2**116)
                if leading == other_leading:
                    gap = abs(s - t)
                    self.assertEqual(gap, abs(bits - other_bits))
                    self.assertLessEqual(gap, 52)
                    mathematical = (n if s < t else m) << gap
                    self.assertLess(mathematical, 2**53)
                    self.assertEqual(mathematical & (2**64 - 1), mathematical)
                else:
                    self.assertEqual(order(leading, other_leading), expected)

    def test_twenty_cover_witnesses_and_three_semantic_defects(self):
        a, b = 2**52, 2**53 - 1
        witnesses = [((0, 0, 0, 0), 0), ((0, 0, 1, 1041), -1), ((1, 1041, 0, 0), 1),
                     ((1, 1, 1, 1), 0), ((1, 2, 1, 1), -1), ((1, 1, 1, 2), 1),
                     ((2, 3, 3, 3), -1), ((3, 3, 2, 3), 1), ((1, 1, 2, 2), 0),
                     ((2, 2, 1, 1), 0), ((1, 1, a, 53), 0), ((a, 53, 1, 1), 0),
                     ((1, 1, a + 1, 53), -1), ((a + 1, 53, 1, 1), 1),
                     ((a, 53, a, 116), 1), ((a, 116, a, 53), -1),
                     ((a, 53, a, 117), 1), ((a, 117, a, 53), -1),
                     ((b, 1041, b, 53), -1), ((1, 1, 1, 1041), 1)]
        self.assertEqual(len(witnesses), 20)
        for (n, s, m, t), expected in witnesses:
            self.assertTrue(0 <= Fraction(n, 2**s) < 1 and 0 <= Fraction(m, 2**t) < 1)
            self.assertEqual(order(Fraction(n, 2**s), Fraction(m, 2**t)), expected)
        self.assertEqual(Fraction(1, 2), Fraction(1, 2))  # required successful equality
        self.assertNotEqual(rational_oracle(1, 2, 1, 1), 1)  # reversed unequal powers
        self.assertEqual(Fraction(1, 2), Fraction(2, 4))
        self.assertNotEqual(order(1 << 0, 2), 0)  # zeroed alignment count loses equality


if __name__ == "__main__":
    unittest.main()
