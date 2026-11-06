"""Source preparation and independent specification controls; no Rust execution."""
from __future__ import annotations

from fractions import Fraction as Rational
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SPEC = importlib.util.spec_from_file_location("fraction_partitions_projection_test",
                                             HERE / "fraction_addition_partitions_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class PreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-addition-partitions-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.core = self.source / "crates/time-core/src"
        self.core.mkdir(parents=True)
        self.formal = self.source / "tools/formal"
        self.formal.mkdir(parents=True)
        for name in ("projection_enclosure.rs", "selection.rs"):
            (self.core / name).write_bytes((ROOT / "crates/time-core/src" / name).read_bytes())
        for name in (*PROJECTION.SUPPORT_SHA256, "fraction_addition_partitions_harness.rs"):
            (self.formal / name).write_bytes((HERE / name).read_bytes())
        self.harness = self.formal / "fraction_addition_partitions_harness.rs"
        self.jobs = PROJECTION.schedule()

    def prepare(self, index=0, folder=None):
        job = self.jobs[index]
        return PROJECTION.prepare_fraction_addition_partitions(
            self.source, folder or self.base / str(index), job["subject"], job["case"], self.harness)

    def test_exact_sixteen_job_schedule_and_closed_ordinary_harnesses(self):
        self.assertEqual(len(self.jobs), 16)
        self.assertEqual([j["subject"] for j in self.jobs[:7]],
                         ["fraction_addition_partition_union"]
                         + ["fraction_addition_partition_" + str(i) for i in range(6)])
        self.assertEqual([j["case"] for j in self.jobs],
                         ["healthy"] * 7 + ["always_error", "wrong_carry", "wrong_residual"]
                         + ["healthy"] * 6)
        self.assertEqual([j["restored_healthy"] for j in self.jobs], [False] * 10 + [True] * 6)
        self.assertEqual(self.jobs[1:7], [{**j, "restored_healthy": False} for j in self.jobs[10:]])
        for job in [*self.jobs[:7], *self.jobs[10:]]:
            self.assertEqual(job["harness"], PROJECTION.MODULE + job["subject"])
        self.assertEqual(PROJECTION.FIXED_OUTPUT_LEAF,
                         "formal-core-fraction-addition-partitions-20261106-v1")

    def test_all_original_spans_and_inline_callees_match_for_every_job(self):
        _, original = PROJECTION._support(self.source)
        originals = {name: (self.core / name).read_bytes() for name in original.SOURCE_SHA256}
        spans = {name: original._extract(originals, name) for name in original.SPANS}
        for index in range(16):
            prepared = self.prepare(index)
            selected = (prepared["entrypoint"].parent / "fraction_addition_partitions_selected.rs").read_bytes()
            for name, span in spans.items():
                expected = span
                if name == "addition" and prepared["mutation"]:
                    mutation = prepared["mutation"]
                    expected = span.replace(mutation["before"].encode(), mutation["after"].encode(), 1)
                self.assertEqual(selected.count(expected), 1, (index, name))
            self.assertEqual(len(prepared["correspondence"]), 9)
            self.assertTrue(prepared["original_callees_and_other_arithmetic_preserved"])
            self.assertEqual(prepared["compiled_helper_sha256"], PROJECTION.digest(
                spans["addition"] if prepared["mutation"] is None else spans["addition"].replace(
                    prepared["mutation"]["before"].encode(), prepared["mutation"]["after"].encode(), 1)))
            self.assertNotIn(b"kani::", selected)

    def test_unchanged_original_math_helpers_controls_and_all_twenty_three_witnesses(self):
        prepared = self.prepare()
        proof = prepared["entrypoint"].read_bytes()
        helpers, controls, witnesses = PROJECTION._original_harness_parts(
            (self.formal / "fraction_arithmetic_harness.rs").read_bytes())
        self.assertEqual(proof.count(helpers), 1)
        self.assertEqual(proof.count(controls), 1)
        self.assertIn(b"Trusted reviewed specification correspondence", helpers)
        for label, block in witnesses.items():
            self.assertEqual(proof.count(block), 1, label)
        self.assertEqual(proof.count(b"#[kani::proof]"), 10)
        self.assertEqual(proof.count(b"kani::assume("), 6)
        for forbidden in (b"stub_verified", b"kani::stub", b"proof_for_contract", b"kani::requires",
                          b"kani::ensures", b"impl kani::Arbitrary", b"No Kani assumptions"):
            self.assertNotIn(forbidden, proof)

    def test_each_leaf_declares_only_its_membership_and_restores_all_bytes(self):
        prepared = {index: self.prepare(index) for index in range(16)}
        for index in (0, 7, 8, 9):
            self.assertEqual(prepared[index]["assumptions"], [])
        for leaf in range(6):
            first, restored = prepared[leaf + 1], prepared[leaf + 10]
            self.assertEqual(first["assumptions"], [f"partition_guards(n, s, m, t)[{leaf}]"])
            self.assertEqual(first["partition_guard"], PROJECTION.GUARDS[leaf])
            for name in ("proof.rs", "fraction_addition_partitions_selected.rs", "correspondence.json"):
                self.assertEqual((first["entrypoint"].parent / name).read_bytes(),
                                 (restored["entrypoint"].parent / name).read_bytes(), (leaf, name))
        self.assertEqual(len({prepared[i]["mutation"]["intended_failure"] for i in (7, 8, 9)}), 3)

    def test_required_reachable_ledgers_and_cover_allocation(self):
        assertions = PROJECTION.REQUIRED_ASSERTIONS
        covers = PROJECTION.REQUIRED_COVERS
        self.assertEqual(len(assertions[PROJECTION.UNION]), 7)
        self.assertEqual(len(covers[PROJECTION.UNION]), 10)
        self.assertEqual([len(assertions[PROJECTION.MODULE + "fraction_addition_partition_" + str(i)])
                          for i in range(6)], [2, 11, 11, 8, 11, 8])
        self.assertEqual([len(PROJECTION.LEAF_COVERS[i]) for i in range(6)], [2, 8, 2, 9, 2, 2])
        _, original = PROJECTION._support(self.source)
        old_harness = "fraction_arithmetic::fraction_addition_full_domain"
        self.assertEqual(set().union(*(set(v) for v in assertions.values())) - set(PROJECTION.UNION_ASSERTIONS),
                         set(original.REQUIRED_ASSERTIONS[old_harness]))
        original_labels = [label for i in range(1, 6) for label in PROJECTION.LEAF_COVERS[i]]
        self.assertEqual(len(original_labels), len(set(original_labels)))
        self.assertEqual(set(original_labels), set(original.REQUIRED_COVERS[old_harness]))
        text = self.harness.read_text()
        for leaf in range(6):
            body = text.split("fn fraction_addition_partition_" + str(leaf) + "() {", 1)[1]
            body = body.split("\n#[kani::proof]", 1)[0]
            for label in PROJECTION.LEAF_COVERS[leaf]:
                self.assertIn('"' + label + '"', body)

    def test_source_support_and_cover_expression_drift_fail_before_output(self):
        for path in [self.core / "projection_enclosure.rs", self.core / "selection.rs",
                     *(self.formal / name for name in PROJECTION.SUPPORT_SHA256)]:
            original = path.read_bytes()
            path.write_bytes(original + b"\n// drift\n")
            with self.subTest(path=path.name), self.assertRaisesRegex(ValueError, "changed"):
                self.prepare()
            self.assertFalse((self.base / "0").exists())
            path.write_bytes(original)
        self.harness.write_bytes(self.harness.read_bytes().replace(
            b"inputs == (1, 1, 1, 1) && observed", b"inputs == (1, 1, 1, 2) && observed", 1))
        with self.assertRaisesRegex(ValueError, "cover expression changed"):
            self.prepare()
        self.assertFalse((self.base / "0").exists())

    def test_unknown_subject_case_or_nonfixed_harness_rejected(self):
        for subject, case in (("retry", "healthy"), ("fraction_addition_partition_6", "healthy"),
                              ("fraction_addition_partition_0", "always_error"),
                              ("fraction_addition_partition_fault", "healthy")):
            with self.assertRaisesRegex(ValueError, "Unknown"):
                PROJECTION.prepare_fraction_addition_partitions(
                    self.source, self.base / "unknown", subject, case, self.harness)
        with self.assertRaisesRegex(ValueError, "fixed frozen source path"):
            PROJECTION.prepare_fraction_addition_partitions(self.source, self.base / "other",
                self.jobs[0]["subject"], "healthy", HERE / "fraction_addition_partitions_harness.rs")

    def test_existing_inputs_never_overwritten_and_correspondence_matches(self):
        for name in ("proof.rs", "fraction_addition_partitions_selected.rs", "correspondence.json"):
            for symlink in (False, True):
                output = self.base / (name + str(symlink))
                output.mkdir()
                retained = output / name
                if symlink:
                    retained.symlink_to(output / "missing")
                else:
                    retained.write_bytes(b"retained evidence")
                with self.assertRaisesRegex(ValueError, "overwrite"):
                    self.prepare(folder=output)
                self.assertEqual(list(output.iterdir()), [retained])
        prepared = self.prepare()
        self.assertEqual(json.loads((prepared["entrypoint"].parent / "correspondence.json").read_text()),
                         {**prepared, "entrypoint": "proof.rs"})


class BudgetTests(unittest.TestCase):
    def test_preserved_clock_identity_and_remaining_time_use_both_clocks(self):
        self.assertEqual(PROJECTION.PREPARATION_STARTED_UNIX_NS, 1793913403729453001)
        self.assertEqual(PROJECTION.PREPARATION_STARTED_MONOTONIC_NS, 10071562880982)
        self.assertEqual(PROJECTION.PREPARATION_DEADLINE_MONOTONIC_NS, 13671562880982)
        with patch.object(Path, "read_text", return_value=PROJECTION.PREPARATION_BOOT_ID), \
                patch.object(PROJECTION.time, "time_ns", return_value=PROJECTION.PREPARATION_STARTED_UNIX_NS + 20_000_000_000), \
                patch.object(PROJECTION.time, "monotonic_ns", return_value=PROJECTION.PREPARATION_STARTED_MONOTONIC_NS + 10_000_000_000):
            self.assertEqual(PROJECTION.remaining_budget(), 3580)
        with patch.object(Path, "read_text", return_value="other boot"), self.assertRaisesRegex(ValueError, "different host boot"):
            PROJECTION.remaining_budget()

    def test_expiry_or_backwards_either_clock_never_creates_a_fresh_budget(self):
        for wall, monotonic in ((3600, 20), (20, 3600), (-1, 20), (20, -1)):
            with patch.object(Path, "read_text", return_value=PROJECTION.PREPARATION_BOOT_ID), \
                    patch.object(PROJECTION.time, "time_ns", return_value=PROJECTION.PREPARATION_STARTED_UNIX_NS + wall * 10**9), \
                    patch.object(PROJECTION.time, "monotonic_ns", return_value=PROJECTION.PREPARATION_STARTED_MONOTONIC_NS + monotonic * 10**9), \
                    self.assertRaisesRegex(ValueError, "expired|backwards"):
                PROJECTION.remaining_budget()


def guards(n, s, m, t):
    """Reviewed machine-width predicate rendered in Python for independent facts."""
    admitted = lambda numerator, shift: shift <= 1041 and numerator.bit_length() <= 53 and (
        numerator == 0 or numerator.bit_length() <= shift)
    if not admitted(n, s) or not admitted(m, t):
        return [True, False, False, False, False, False]
    a, b = (0 if n == 0 else s), (0 if m == 0 else t)
    large = lambda numerator, shift: 1 <= shift <= 53 and 2 * numerator >= 2**shift
    left, right = large(n, a), large(m, b)
    return [False, not left and not right, left and b >= 106, left and b < 106,
            not left and right and a >= 106, not left and right and a < 106]


class IndependentSpecificationFacts(unittest.TestCase):
    def test_disjoint_raw_union_against_rational_half_and_actual_factory_specification(self):
        raw = [(0, 0), (0, 1041), (0, 1042), (1, 0), (1, 1), (1, 2), (2, 2),
               (3, 2), (1, 105), (1, 106), (1, 127), (1, 128), (1, 1041),
               (2**52 - 1, 53), (2**52, 53), (2**52 + 1, 53), (2**53 - 1, 53),
               (2**53 - 1, 105), (2**53 - 1, 106), (2**53 - 1, 1041),
               (2**53, 54), (2**64 - 1, 1041), (0, 2**32 - 1)]
        seen = set()
        for n, s in raw:
            for m, t in raw:
                observed = guards(n, s, m, t)
                self.assertEqual(sum(observed), 1)
                seen.add(observed.index(True))
                def reference(numerator, shift):
                    if shift > 1041 or numerator >= 2**53:
                        return None
                    value = Rational(numerator, 2**shift)
                    return value if value < 1 else None
                left, right = reference(n, s), reference(m, t)
                self.assertEqual(observed[0], left is None or right is None)
                if left is not None and right is not None:
                    a, b = (0 if n == 0 else s), (0 if m == 0 else t)
                    expected = 1 if left < Rational(1, 2) and right < Rational(1, 2) else (
                        2 if b >= 106 else 3) if left >= Rational(1, 2) else (4 if a >= 106 else 5)
                    self.assertTrue(observed[expected])
                    if expected in (2, 4):
                        self.assertTrue(0 < left + right < 1)
                    if expected in (3, 5):
                        self.assertLessEqual(max(a, b), 105)
        self.assertEqual(seen, set(range(6)))

    def test_all_original_witnesses_have_exactly_their_allocated_guard(self):
        a, b = 2**52, 2**53 - 1
        witnesses = [(0, 0, 0, 0), (1, 2, 0, 0), (0, 0, 1, 2), (1, 2, 1, 2),
            (1, 1, 1, 1), (1, 2, 3, 2), (3, 2, 1, 2), (3, 2, 1, 1), (1, 1, 3, 2),
            (2, 2, 2, 2), (1, 1, a - 1, 53), (1, 1, a + 1, 53), (b, 53, b, 53),
            (b, 53, b, 127), (b, 127, b, 53), (b, 53, b, 128), (b, 128, b, 53),
            (1, 1041, 0, 0), (0, 0, 1, 1041), (1, 1041, 1, 1041), (0, 1041, 0, 1041),
            (1, 1, 1, 2), (1, 2, 1, 1)]
        _, original = PROJECTION._support(ROOT)
        labels = original.REQUIRED_COVERS["fraction_arithmetic::fraction_addition_full_domain"]
        self.assertEqual(len(witnesses), len(labels))
        allocation = {label: leaf for leaf in range(1, 6) for label in PROJECTION.LEAF_COVERS[leaf]}
        for witness, label in zip(witnesses, labels, strict=True):
            self.assertEqual(guards(*witness).index(True), allocation[label], label)


if __name__ == "__main__":
    unittest.main()
