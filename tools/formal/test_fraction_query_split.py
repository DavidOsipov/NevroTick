"""Source-only query preservation, direct witnesses and restoration controls."""
from __future__ import annotations

import sys

sys.dont_write_bytecode = True

import hashlib
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SPEC = importlib.util.spec_from_file_location("query_split_projection_test", HERE / "fraction_query_split_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


class QuerySplitPreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-query-split-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in (*PROJECTION.input_names(), "crates/time-core/src/projection_enclosure.rs",
                     "crates/time-core/src/selection.rs"):
            path = self.source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes((ROOT / name).read_bytes())
        self.formal = self.source / "tools/formal"
        self.jobs = PROJECTION.schedule()
        self.original = PROJECTION._original(self.source)
        self.old_jobs = {job["id"]: job for job in self.original.schedule()}

    def prepare(self, index, folder=None):
        return PROJECTION.prepare(self.source, folder or self.base / f"new-{index}", self.jobs[index])

    def base_addition(self):
        return self.original.prepare(self.source, self.base / "old-addition", self.old_jobs["addition_healthy"])

    def test_fixed_twenty_job_schedule_preserves_prefix_faults_and_two_restorations(self):
        original = self.original.schedule()
        self.assertEqual(len(self.jobs), 20)
        self.assertEqual(self.jobs[:13], original[:13])
        self.assertEqual(self.jobs[14:17], original[14:17])
        canonical = lambda value: hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        self.assertEqual(canonical(self.jobs[:13]), "ff1d5e2118dacde0f5aaf25eec94d40ac2a9dbf3419bb039926515e0c05a3a3e")
        self.assertEqual(canonical(self.jobs[14:17]), "a2f091edd0ae2f3567392a1ce1b91f4cb3f2a2f868419baa7d3122d2f6402da7")
        self.assertEqual([self.jobs[i]["id"] for i in (13, 17, 18, 19)],
                         ["addition_universal_healthy", "addition_universal_restored",
                          "addition_witnesses_healthy", "addition_witnesses_restored"])
        for first, restored in ((13, 17), (18, 19)):
            self.assertEqual({k: v for k, v in self.jobs[first].items() if k not in {"id", "restored_healthy"}},
                             {k: v for k, v in self.jobs[restored].items() if k not in {"id", "restored_healthy"}})
            self.assertFalse(self.jobs[first]["restored_healthy"])
            self.assertTrue(self.jobs[restored]["restored_healthy"])
        self.assertEqual(self.jobs[13]["dependencies"], ["constructor", "comparison", "complement"])
        self.assertEqual(len(self.jobs[13]["expected_stubs"]), 3)
        self.assertEqual(self.jobs[18]["dependencies"], [])
        self.assertEqual(self.jobs[18]["expected_stubs"], [])
        self.assertEqual(self.jobs[18]["required_refinement_assertions"], [])

    def test_original_files_source_inventory_and_bytecode_safe_project_loading(self):
        for name, expected in PROJECTION.PRESERVED_SHA256.items():
            self.assertEqual(PROJECTION.digest((HERE / name).read_bytes()), expected, name)
        self.assertTrue(all((ROOT / name).is_file() for name in PROJECTION.input_names()))
        self.assertTrue(all(name.startswith("tools/formal/") for name in PROJECTION.input_names()))
        self.assertFalse(set(PROJECTION.input_names()) & set(PROJECTION.generated_input_names()))
        self.assertTrue(sys.dont_write_bytecode)
        self.original._support(self.source)
        self.assertFalse(list(self.source.rglob("__pycache__")))
        self.assertFalse(list(self.source.rglob("*.pyc")))

    def test_prefix_and_fault_payloads_are_exact_original_preparations(self):
        for index in (*range(13), 14, 15, 16):
            job = self.jobs[index]
            first = self.prepare(index)
            old = self.original.prepare(self.source, self.base / f"old-{index}", self.old_jobs[job["id"]])
            self.assertEqual(first["input_files_sha256"], old["input_files_sha256"])
            for name in PROJECTION.generated_input_names():
                self.assertEqual((first["entrypoint"].parent / name).read_bytes(),
                                 (old["entrypoint"].parent / name).read_bytes(), (index, name))

    def test_universal_is_only_original_query_rename_and_twenty_two_cover_removals(self):
        old = self.base_addition()
        old_proof = old["entrypoint"].read_bytes()
        raw = (self.formal / "fraction_arithmetic_harness.rs").read_bytes()
        function, witnesses = PROJECTION.original_query(raw)
        new = self.prepare(13)
        proof = new["entrypoint"].read_bytes()
        transformed = PROJECTION.universal_function(function, witnesses)
        self.assertEqual(proof, old_proof.replace(function, transformed, 1))
        # Removing the single retained cover and undoing the function rename
        # yields precisely the original unrestricted raw-input query prefix.
        without_covers = function
        for item in witnesses:
            without_covers = without_covers.replace(item["block"], b"", 1)
        zero = next(item["block"] for item in witnesses if item["label"] == "addition zero sum")
        self.assertEqual(transformed.replace(zero, b"", 1).replace(
            b"fn fraction_addition_universal_full_domain()", b"fn fraction_addition_full_domain()", 1), without_covers)
        self.assertEqual(transformed.count(b"kani::any::<u64>()"), 2)
        self.assertEqual(transformed.count(b"kani::any::<u32>()"), 2)
        self.assertEqual(transformed.count(b"kani::cover!"), 1)
        self.assertNotIn(b"kani::assume", transformed)
        self.assertEqual(new["compiled_span_sha256"], old["compiled_span_sha256"])

    def test_whole_original_helpers_duplicate_assertion_sites_and_calls_are_preserved(self):
        old = self.base_addition()
        new = self.prepare(13)
        first, second = old["entrypoint"].read_bytes(), new["entrypoint"].read_bytes()
        raw = (self.formal / "fraction_arithmetic_harness.rs").read_bytes()
        start = raw.index(b"const HALF_NUMERATOR:")
        stop = raw.index(b"#[kani::proof]")
        helpers = raw[start:stop]
        self.assertEqual(first.count(helpers), 1)
        self.assertEqual(second.count(helpers), 1)
        self.assertEqual(helpers.count(b'"addition global fraction maximum"'), 2)
        self.assertEqual(helpers.count(b"addition_global_fraction_maximum(left);"), 1)
        self.assertEqual(helpers.count(b"addition_global_fraction_maximum(right);"), 1)
        for label in PROJECTION.ALL_ASSERTIONS:
            self.assertEqual(first.count(('"' + label + '"').encode()), second.count(('"' + label + '"').encode()))

    def test_direct_witnesses_keep_all_original_tuple_answer_conjunctions_without_stubs(self):
        old = self.base_addition()
        raw = (self.formal / "fraction_arithmetic_harness.rs").read_bytes()
        function, witnesses = PROJECTION.original_query(raw)
        generated = PROJECTION.witness_function(witnesses)
        prepared = self.prepare(18)
        proof = prepared["entrypoint"].read_bytes()
        expected = old["entrypoint"].read_bytes().replace(function, generated, 1)
        for pair in self.old_jobs["addition_healthy"]["expected_stubs"]:
            declaration = ("#[kani::stub(" + pair["original"] + ", " + pair["replacement"] + ")]\n").encode()
            expected = expected.replace(declaration, b"", 1)
        self.assertEqual(proof, expected)
        self.assertEqual(generated.count(b"addition_contract(n, s, m, t);"), 23)
        self.assertEqual(generated.count(b"kani::cover!"), 23)
        self.assertNotIn(b"kani::any", generated)
        self.assertNotIn(b"kani::assume", generated)
        self.assertNotIn(b"composition_", generated)
        self.assertNotIn(b"#[kani::stub", proof)
        for item in witnesses:
            self.assertEqual(generated.count(item["block"]), 1, item["label"])
            self.assertIn(b"let (n, s, m, t) = " + item["tuple_source"].encode() + b";", generated)
        selected = (prepared["entrypoint"].parent / "fraction_composition_selected.rs").read_bytes()
        self.assertNotIn(b"kani::", selected)
        helpers = raw[raw.index(b"fn addition_global_fraction_maximum("):raw.index(b"#[kani::proof]")]
        self.assertNotIn(b"kani::any", helpers)
        self.assertNotIn(b"kani::assume", helpers)
        self.assertNotIn(b"composition_", helpers)
        self.assertEqual(prepared["assumptions"], [])

    def test_both_lanes_restore_all_generated_bytes_and_fresh_callee_bindings_match(self):
        prepared = {i: self.prepare(i) for i in (4, 7, 10, 13, 14, 15, 16, 17, 18, 19)}
        for first, restored in ((13, 17), (18, 19)):
            self.assertEqual(prepared[first]["input_files_sha256"], prepared[restored]["input_files_sha256"])
            for name in PROJECTION.generated_input_names():
                self.assertEqual((prepared[first]["entrypoint"].parent / name).read_bytes(),
                                 (prepared[restored]["entrypoint"].parent / name).read_bytes(), name)
        for key, index in (("constructor", 4), ("comparison", 7), ("complement", 10)):
            for caller in (13, 14, 15, 16, 17, 18, 19):
                self.assertEqual(prepared[index]["callee_bindings"][key], prepared[caller]["callee_bindings"][key])
        for index in (13, 18):
            correspondence = json.loads((prepared[index]["entrypoint"].parent / "correspondence.json").read_text())
            self.assertEqual(len(correspondence["query_split"]["witness_ledger"]), 23)
            self.assertEqual(correspondence["query_split"]["canonical_base_job"], "addition_healthy")
            self.assertEqual(correspondence["harness"], self.jobs[index]["harness"])

    def test_new_contracts_keep_nonempty_cover_policy_and_full_property_union(self):
        universal = PROJECTION.expected_contract(PROJECTION.UNIVERSAL_SUBJECT, PROJECTION.UNIVERSAL_HARNESS)
        witnesses = PROJECTION.expected_contract(PROJECTION.WITNESS_SUBJECT, PROJECTION.WITNESS_HARNESS)
        _, arithmetic, _, _ = self.original._support(self.source)
        old_harness = self.old_jobs["addition_healthy"]["harness"]
        self.assertEqual(universal[0], witnesses[0])
        self.assertEqual(universal[0], frozenset(arithmetic.REQUIRED_ASSERTIONS[old_harness]))
        self.assertEqual(universal[1], frozenset(("addition zero sum",)))
        self.assertEqual(witnesses[1], frozenset(arithmetic.REQUIRED_COVERS[old_harness]))
        self.assertEqual((len(universal[0]), len(universal[1]), len(witnesses[1])), (11, 1, 23))
        for subject, harness in ((PROJECTION.UNIVERSAL_SUBJECT, PROJECTION.WITNESS_HARNESS),
                                 (PROJECTION.WITNESS_SUBJECT, PROJECTION.UNIVERSAL_HARNESS),
                                 ("fraction_addition", PROJECTION.UNIVERSAL_HARNESS)):
            with self.assertRaises(ValueError):
                PROJECTION.expected_contract(subject, harness)

    def test_unknown_weakened_jobs_and_source_or_witness_drift_refused_before_output(self):
        for field, value in (("id", "retry"), ("expected_stubs", []), ("dependencies", []),
                             ("required_refinement_assertions", []), ("harness", PROJECTION.WITNESS_HARNESS)):
            job = {**self.jobs[13], field: value}
            with self.assertRaisesRegex(ValueError, "Unknown or changed fixed"):
                PROJECTION.prepare(self.source, self.base / "denied", job)
            self.assertFalse((self.base / "denied").exists())
        raw = (self.formal / "fraction_arithmetic_harness.rs").read_bytes()
        for before, after in ((b"observed == Some((false, false))", b"observed == Some((true, false))"),
                              (b"addition_global_fraction_maximum(right);", b"addition_global_fraction_maximum(left);")):
            with self.assertRaisesRegex(ValueError, "Original addition harness changed"):
                PROJECTION.original_query(raw.replace(before, after, 1))
        (self.formal / "fraction_arithmetic_harness.rs").write_bytes(raw + b"\n// drift\n")
        with self.assertRaisesRegex(ValueError, "Original addition harness changed"):
            self.prepare(13)
        self.assertFalse((self.base / "new-13").exists())

    def test_existing_input_custody_and_old_kit_parity_are_preserved(self):
        before = {name: (HERE / name).read_bytes() for name in PROJECTION.PRESERVED_SHA256}
        prepared = self.prepare(13)
        snapshots = {name: (prepared["entrypoint"].parent / name).read_bytes()
                     for name in PROJECTION.generated_input_names()}
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare(13)
        self.assertEqual(snapshots, {name: (prepared["entrypoint"].parent / name).read_bytes()
                                     for name in PROJECTION.generated_input_names()})
        self.assertEqual(before, {name: (HERE / name).read_bytes() for name in before})
        self.assertFalse(list(self.source.rglob("__pycache__")))


if __name__ == "__main__":
    unittest.main()
