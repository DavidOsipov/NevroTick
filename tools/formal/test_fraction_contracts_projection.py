"""Source-only controls for the finite actual-callee contract preparation."""
from __future__ import annotations

from fractions import Fraction as Rational
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SPEC = importlib.util.spec_from_file_location("fraction_contracts_projection_test",
                                             HERE / "fraction_contracts_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class PreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-contracts-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        self.formal = self.source / "tools/formal"
        self.formal.mkdir(parents=True)
        for name in ("projection_enclosure.rs", "selection.rs"):
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        for name in (*PROJECTION.SUPPORT_SHA256, "fraction_contracts_spec.rs"):
            (self.formal / name).write_bytes((HERE / name).read_bytes())
        self.schedule = {job["id"]: job for job in PROJECTION.jobs()}

    def prepare(self, identity, folder=None):
        return PROJECTION.prepare(self.source, folder or self.base / identity,
                                  self.schedule[identity])

    def test_only_eleven_fixed_jobs_and_ordered_success_dependencies(self):
        self.assertEqual(list(self.schedule), [
            "mechanism_callee", "mechanism_caller", "mechanism_precondition",
            "mechanism_bad_callee", "constructor", "complement", "addition_healthy",
            "addition_reject_all", "addition_wrong_carry", "addition_wrong_residual",
            "addition_restored"])
        earlier = set()
        for job in self.schedule.values():
            self.assertTrue(set(job["dependencies"]) <= earlier)
            earlier.add(job["id"])
        self.assertEqual(self.schedule["constructor"]["expected_kind"],
                         'ProofForContract { target_fn: "Fraction::new" }')
        self.assertEqual(self.schedule["complement"]["dependencies"], ["constructor"])
        self.assertEqual(self.schedule["addition_healthy"]["dependencies"],
                         ["constructor", "complement"])
        self.assertEqual(self.schedule["constructor"]["required_contract_assertions"],
                         ["|result: &Result<Self, FixtureEnclosureError>| "
                          "constructor_post(numerator, shift, result)"])
        self.assertEqual(self.schedule["complement"]["required_contract_assertions"],
                         ["|result: &Result<Self, FixtureEnclosureError>| complement_post(self, result)"])
        primitive_clause = "|result: &u8| *result > value"
        self.assertEqual(self.schedule["mechanism_callee"]["required_contract_assertions"],
                         [primitive_clause])
        self.assertEqual(self.schedule["mechanism_bad_callee"]["intended_failure"], primitive_clause)
        self.assertEqual(self.schedule["mechanism_precondition"]["intended_failure"], "value < u8::MAX")
        self.assertEqual([(len(self.schedule[name]["expected_assertions"]),
                           len(self.schedule[name]["expected_covers"]))
                          for name in ("constructor", "complement", "addition_healthy")],
                         [(8, 14), (7, 13), (11, 23)])

    def test_all_nine_product_spans_preserved_except_annotations_and_selected_mutant(self):
        folder, arithmetic, _ = PROJECTION._support(self.source)
        del folder
        originals = {name: (self.core / name).read_bytes() for name in arithmetic.SOURCE_SHA256}
        spans = {name: arithmetic._extract(originals, name) for name in arithmetic.SPANS}
        for identity in list(self.schedule)[4:]:
            prepared = self.prepare(identity)
            definitions = (prepared["entrypoint"].parent / "fraction_contracts_selected.rs").read_bytes()
            stripped = definitions.replace(PROJECTION.NEW_ATTRIBUTE, b"").replace(
                PROJECTION.COMPLEMENT_ATTRIBUTES, b"")
            for name, original in spans.items():
                expected = original
                if name == "addition" and prepared["mutation"]:
                    mutation = prepared["mutation"]
                    expected = original.replace(mutation["before"].encode(), mutation["after"].encode(), 1)
                self.assertEqual(stripped.count(expected), 1, (identity, name))
            self.assertEqual(definitions.count(b"#[kani::ensures"), 2)
            self.assertEqual(definitions.count(b"#[kani::requires"), 1)
            self.assertIn(spans["compare"], definitions)
            self.assertEqual(prepared["compare_inline_unannotated_sha256"], arithmetic.SPANS["compare"]["sha256"])
            self.assertEqual(len(prepared["correspondence"]), 9)
            self.assertNotIn(b"stub_verified", definitions)

    def test_original_product_assertion_and_cover_bodies_are_retained(self):
        prepared = self.prepare("addition_healthy")
        proof = prepared["entrypoint"].read_bytes()
        constructor, arithmetic = PROJECTION._harnesses(self.formal)
        self.assertIn(constructor, proof)
        self.assertIn(arithmetic, proof)
        strip = lambda text: re.sub(rb"^#\[kani::(?:proof|proof_for_contract|stub_verified)[^\n]*\]\n",
                                    b"", text, flags=re.MULTILINE)
        old_constructor = (self.formal / "fraction_constructor_harness.rs").read_bytes().split(
            b"/// Reject-all loses", 1)[0]
        old_arithmetic = (self.formal / "fraction_arithmetic_harness.rs").read_bytes()
        old_arithmetic = old_arithmetic[old_arithmetic.index(b"const HALF_NUMERATOR:"):]
        self.assertEqual(strip(constructor), strip(old_constructor))
        self.assertEqual(strip(arithmetic), strip(old_arithmetic))
        self.assertEqual(proof.count(b"#[kani::proof_for_contract(Fraction::new)]"), 1)
        self.assertEqual(proof.count(b"#[kani::proof_for_contract(Fraction::complement)]"), 1)
        for identity in ("constructor", "complement", "addition_healthy"):
            job = self.schedule[identity]
            for label in (*job["expected_assertions"], *job["expected_covers"]):
                self.assertIn(('"' + label + '"').encode(), proof)
        self.assertNotIn(b"No Kani assumptions", proof)

    def test_callee_closures_survive_caller_only_mutation_and_restore_exact_inputs(self):
        prepared = {identity: self.prepare(identity) for identity in list(self.schedule)[4:]}
        reference = prepared["constructor"]["callee_bindings"]
        for value in prepared.values():
            self.assertEqual(value["callee_bindings"]["constructor"], reference["constructor"])
            self.assertEqual(value["callee_bindings"]["complement"], reference["complement"])
        first, restored = prepared["addition_healthy"], prepared["addition_restored"]
        self.assertEqual(first["input_files_sha256"], restored["input_files_sha256"])
        for name in first["input_files_sha256"]:
            self.assertEqual((first["entrypoint"].parent / name).read_bytes(),
                             (restored["entrypoint"].parent / name).read_bytes())
        self.assertEqual(len({prepared[name]["mutation"]["intended_failure"] for name in
                              ("addition_reject_all", "addition_wrong_carry", "addition_wrong_residual")}), 3)

    def test_specification_changes_invalidate_both_callee_bindings(self):
        first = self.prepare("constructor")
        spec = self.formal / "fraction_contracts_spec.rs"
        spec.write_bytes(spec.read_bytes() + b"\n// changed proof specification\n")
        changed = self.prepare("constructor", self.base / "changed-spec")
        self.assertNotEqual(first["callee_bindings"]["constructor"], changed["callee_bindings"]["constructor"])
        self.assertNotEqual(first["callee_bindings"]["complement"], changed["callee_bindings"]["complement"])

    def test_mechanism_good_sources_match_and_bad_callee_changes_actual_body(self):
        prepared = {name: self.prepare(name) for name in list(self.schedule)[:4]}
        self.assertEqual(prepared["mechanism_callee"]["input_files_sha256"],
                         prepared["mechanism_caller"]["input_files_sha256"])
        self.assertEqual(prepared["mechanism_callee"]["input_files_sha256"],
                         prepared["mechanism_precondition"]["input_files_sha256"])
        normal = prepared["mechanism_callee"]["entrypoint"].read_bytes()
        bad = prepared["mechanism_bad_callee"]["entrypoint"].read_bytes()
        changed = normal.replace(b"{ value + 1 }", b"{ value }", 1).replace(
            b'    kani::cover!(value == 254 && observed == 255, "mechanism callee admitted boundary");\n', b"")
        self.assertEqual(changed, bad)
        self.assertNotEqual(prepared["mechanism_callee"]["callee_bindings"]["mechanism_callee"],
                            prepared["mechanism_bad_callee"]["callee_bindings"]["mechanism_callee"])
        self.assertNotIn(b"kani::assume", normal)
        self.assertNotIn(b"assert!(observed > value", bad)

    def test_unknown_or_modified_jobs_rejected_before_output(self):
        for changes in ({"id": "retry"}, {"case": "wrong_carry"}, {"verified_stubs": []},
                        {"expected_covers": []}, {"expected_kind": "ProofForContract"}):
            job = {**self.schedule["addition_healthy"], **changes}
            output = self.base / "rejected"
            with self.assertRaisesRegex(ValueError, "Unknown or changed fixed"):
                PROJECTION.prepare(self.source, output, job)
            self.assertFalse(output.exists())

    def test_source_or_support_drift_rejected_before_any_output(self):
        for path in [self.core / "projection_enclosure.rs", self.core / "selection.rs",
                     *(self.formal / name for name in PROJECTION.SUPPORT_SHA256)]:
            original = path.read_bytes()
            path.write_bytes(original + b"\n// drift\n")
            output = self.base / "drift-output"
            with self.subTest(path=path.name), self.assertRaisesRegex(ValueError, "changed"):
                PROJECTION.prepare(self.source, output, self.schedule["constructor"])
            self.assertFalse(output.exists())
            path.write_bytes(original)

    def test_existing_files_or_symlinks_are_never_overwritten(self):
        for name in ("proof.rs", "fraction_contracts_selected.rs", "fraction_contracts_spec.rs", "correspondence.json"):
            for symlink in (False, True):
                output = self.base / (name + str(symlink))
                output.mkdir()
                preserved = output / name
                if symlink:
                    preserved.symlink_to(output / "missing")
                else:
                    preserved.write_bytes(b"retained")
                with self.assertRaisesRegex(ValueError, "overwrite"):
                    PROJECTION.prepare(self.source, output, self.schedule["constructor"])
                self.assertEqual(list(output.iterdir()), [preserved])
                if symlink:
                    self.assertTrue(preserved.is_symlink())
                else:
                    self.assertEqual(preserved.read_bytes(), b"retained")

    def test_correspondence_reports_assumptions_generators_and_exact_written_bytes(self):
        prepared = self.prepare("addition_healthy")
        folder = prepared["entrypoint"].parent
        recorded = json.loads((folder / "correspondence.json").read_text())
        self.assertEqual(recorded, {**prepared, "entrypoint": "proof.rs"})
        self.assertEqual(prepared["approved_callee_requires"],
                         {"constructor": [], "complement": ["checked_fraction(self)"]})
        for name, expected in prepared["input_files_sha256"].items():
            self.assertEqual(PROJECTION.digest((folder / name).read_bytes()), expected)
        spec = (folder / "fraction_contracts_spec.rs").read_text()
        for forbidden in ("kani::assume", "Fraction::new(", ".complement(", ".compare(", "add_fractions(", ".filter("):
            self.assertNotIn(forbidden, spec)
        self.assertEqual(spec.count("impl kani::Arbitrary for "), 3)
        for variant in ("Self::ZeroLabel", "Self::ReversedBounds", "Self::ValidationInvariant",
                        "Self::Unrepresentable", "Self::Invariant", "Self::Interval"):
            self.assertIn(variant, spec)
        self.assertIn("numerator: kani::any::<u64>()", spec)
        self.assertIn("shift: kani::any::<u32>()", spec)


class IndependentSpecificationFacts(unittest.TestCase):
    def test_factory_bit_length_spec_matches_proper_rationals_and_zero_normalization(self):
        values = set(range(66)) | {2**52 - 1, 2**52, 2**53 - 1, 2**53, 2**64 - 1}
        shifts = set(range(14)) | {52, 53, 54, 63, 64, 127, 128, 1041, 1042, 2**32 - 1}
        for numerator in values:
            for shift in shifts:
                admitted = shift <= 1041 and numerator.bit_length() <= 53 and (
                    numerator == 0 or numerator.bit_length() <= shift)
                independent = shift <= 1041 and numerator < 2**53 and (
                    numerator == 0 or (shift > 0 and numerator < 2**min(shift, 64)))
                self.assertEqual(admitted, independent, (numerator, shift))
                if admitted:
                    canonical = (numerator, 0 if numerator == 0 else shift)
                    self.assertTrue(canonical[0] != 0 or canonical[1] == 0)

    def test_complement_complete_threshold_and_retained_representation_against_rationals(self):
        values = set(range(1, 66)) | {2**52 - 1, 2**52, 2**52 + 1, 2**53 - 1}
        cases = [(0, 0)] + [(n, s) for n in values for s in
                              (1, 2, 3, 7, 52, 53, 54, 63, 64, 127, 128, 1041)
                              if n.bit_length() <= min(s, 53)]
        for n, s in cases:
            required = 1 <= s <= 53 and 2 * n >= 2**s
            self.assertEqual(required, Rational(n, 2**s) >= Rational(1, 2), (n, s))
            if required:
                result_n = 2**s - n
                self.assertTrue(1 <= result_n <= 2**52)
                self.assertLess(result_n, 2**s)
                self.assertEqual(Rational(result_n, 2**s) + Rational(n, 2**s), 1)
                self.assertLessEqual(result_n.bit_length(), min(s, 53))
        # The raw factory domain admits these; a checked object never retains them.
        self.assertLessEqual((0).bit_length(), 1041)
        self.assertNotEqual((0, 1041), (0, 0))


if __name__ == "__main__":
    unittest.main()
