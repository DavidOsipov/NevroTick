"""Challenge exact source-field preparation without a compiler or solver."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent


def load(name, path):
    """Load only the selected current local Python source for inert controls."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Local projection is unavailable")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


PROJECTION = load("lifecycle_source_fields_projection_test", HERE / "lifecycle_source_fields_projection.py")
SUITES = load("lifecycle_source_fields_registry_test", HERE / "additional_suites.py")


class LifecycleSourceFieldsTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-lifecycle-source-fields-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in PROJECTION.SOURCE_SHA256:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "lifecycle_source_fields_harness.rs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_lifecycle_source_fields(
            self.source, inputs or self.inputs, case, self.harness)

    def test_two_exact_original_spans_constants_and_actual_standard_calls(self):
        result = self.prepare()
        definitions = (self.inputs / "lifecycle_source_fields_selected.rs").read_bytes()
        owner = (self.inputs / "lifecycle_source_fields_owner.rs").read_bytes()
        self.assertEqual(result["selected_original_bytes"], 474)
        self.assertEqual([(item["source"], item["byte_offsets_half_open"], item["sha256"])
                          for item in result["correspondence"]], [
            ("crates/time-core/src/combination/lifecycle.rs", [3114, 3310],
             "1e17644c506c8ccaae958f5c844f42efc39bf07a24264ba454eb3f77e42a1cb9"),
            ("crates/time-core/src/combination/lifecycle.rs", [18571, 18849],
             "7607cd68f02e8979c86cc67a74dae22bbee67df1de495e9b3918ab6ebaa78545")])
        for item, actual in zip(result["correspondence"], (definitions, owner), strict=True):
            raw = (ROOT / item["source"]).read_bytes()
            self.assertEqual(actual, raw[slice(*item["byte_offsets_half_open"])])
            self.assertEqual(PROJECTION.digest(actual), item["sha256"])
        self.assertIn(b"const MATURE_UPDATES: u8 = 8;", definitions)
        self.assertIn(b"const DISTANT_PENALTY: u8 = 32;", definitions)
        self.assertIn(b"u64::from(value.0) == value.2.min(u64::from(MATURE_UPDATES))", owner)
        proof = (self.inputs / "proof.rs").read_text()
        self.assertIn('mod lifecycle_source_fields {\ninclude!("lifecycle_source_fields_selected.rs");', proof)
        self.assertIn('include!("lifecycle_source_fields_owner.rs");', proof)
        self.assertEqual(proof.count(self.harness.read_text()), 1)
        for replacement in (b"fn min", b"impl From", b"impl Ord", b"enum ", b"struct ", b"garde"):
            self.assertNotIn(replacement, definitions + owner)

    def test_correspondence_hashes_full_machine_domain_and_no_assumptions(self):
        result = self.prepare()
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        for name, expected in result["compiled_files_sha256"].items():
            self.assertEqual(PROJECTION.digest((self.inputs / name).read_bytes()), expected)
        definitions = (self.inputs / "lifecycle_source_fields_selected.rs").read_bytes()
        owner = (self.inputs / "lifecycle_source_fields_owner.rs").read_bytes()
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions + owner))
        self.assertEqual(result["machine_widths"], {"maturity": 8, "penalty": 8, "ordinal": 64,
                                                    "specification": 128})
        self.assertEqual(result["domain"], {"independent_scalar_values": 3,
                         "raw_value_types": ["u8", "u8", "u64"], "raw_tuple_count": 2 ** 80,
                         "assumed_input_invariants": []})
        self.assertEqual(result["assumptions"], [])
        self.assertIsNone(result["mutation"])
        self.assertIn("source_fields_rule and Garde composition", result["omitted"])
        self.assertIn("private source construction", result["omitted"])

    def test_three_controls_each_change_exactly_one_real_source_point(self):
        self.prepare()
        expected = {
            "strict_penalty": ("implementation", "value.1 <= DISTANT_PENALTY", "value.1 < DISTANT_PENALTY"),
            "wrong_maturity": ("constants", "const MATURE_UPDATES: u8 = 8;", "const MATURE_UPDATES: u8 = 7;"),
            "loose_relationship": ("implementation", "u64::from(value.0) == value.2.min(u64::from(MATURE_UPDATES))",
                                   "u64::from(value.0) <= value.2.min(u64::from(MATURE_UPDATES))"),
        }
        self.assertEqual(set(PROJECTION.CONTROLS), set(expected))
        for case, (span, before, after) in expected.items():
            with self.subTest(case=case):
                control = PROJECTION.CONTROLS[case]
                self.assertEqual((control["span"], control["before"], control["after"]), (span, before, after))
                folder = self.base / case
                result = self.prepare(case, folder)
                self.assertEqual(result["mutation"], control)
                changed_name = ("lifecycle_source_fields_selected.rs" if span == "constants"
                                else "lifecycle_source_fields_owner.rs")
                for name in ("proof.rs", "lifecycle_source_fields_selected.rs", "lifecycle_source_fields_owner.rs"):
                    original = (self.inputs / name).read_bytes()
                    if name == changed_name:
                        self.assertEqual(original.count(before.encode()), 1)
                        self.assertEqual((folder / name).read_bytes(), original.replace(before.encode(), after.encode(), 1))
                    else:
                        self.assertEqual((folder / name).read_bytes(), original)

    def test_mutation_missing_and_nonunique_points_fail_before_outputs(self):
        for before in ("unreviewed mutation point", " "):
            with self.subTest(before=before), patch.dict(PROJECTION.CONTROLS, {
                    "strict_penalty": {**PROJECTION.CONTROLS["strict_penalty"], "before": before}}):
                with self.assertRaisesRegex(ValueError, "mutation point is not unique"):
                    self.prepare("strict_penalty")
                self.assertFalse(self.inputs.exists())

    def test_restoration_reproduces_all_four_generated_files_byte_for_byte(self):
        self.prepare()
        restored = self.base / "restored"
        self.prepare(inputs=restored)
        names = {path.name for path in self.inputs.iterdir()}
        self.assertEqual(names, {"proof.rs", "lifecycle_source_fields_selected.rs",
                                 "lifecycle_source_fields_owner.rs", "correspondence.json"})
        self.assertEqual(names, {path.name for path in restored.iterdir()})
        for name in names:
            self.assertEqual((self.inputs / name).read_bytes(), (restored / name).read_bytes())

    def test_complete_source_drift_rejected_before_any_output(self):
        path = self.source / next(iter(PROJECTION.SOURCE_SHA256))
        path.write_bytes(path.read_bytes() + b"\n")
        with self.assertRaisesRegex(ValueError, "complete source changed"):
            self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_each_span_pin_and_both_anchor_identities_reject_rebound_drift(self):
        for name, span in PROJECTION.SPANS.items():
            with self.subTest(name=name):
                path = self.source / span["source"]
                original = path.read_bytes()
                position = span["offsets"][0] + len(span["start"]) + 2
                changed = original[:position] + b" " + original[position + 1:]
                variants = [(changed, "source span changed"),
                            (original + b"\n" + span["start"], "ambiguous"),
                            (original + b"\n" + span["following"], "ambiguous")]
                for candidate, failure in variants:
                    path.write_bytes(candidate)
                    with patch.dict(PROJECTION.SOURCE_SHA256, {span["source"]: PROJECTION.digest(candidate)}):
                        with self.assertRaisesRegex(ValueError, failure):
                            self.prepare()
                    self.assertFalse(self.inputs.exists())
                path.write_bytes(original)

    def test_unknown_cases_and_all_output_collisions_fail_without_replacement(self):
        for case in ("other", None, True, 1):
            with self.subTest(case=case), self.assertRaisesRegex(ValueError, "Unknown lifecycle-source-fields control"):
                self.prepare(case)
        self.inputs.mkdir()
        for name in ("proof.rs", "lifecycle_source_fields_selected.rs", "lifecycle_source_fields_owner.rs", "correspondence.json"):
            target = self.inputs / name
            target.write_bytes(b"owned original\n")
            with self.assertRaisesRegex(ValueError, "already exists"):
                self.prepare()
            self.assertEqual(target.read_bytes(), b"owned original\n")
            target.unlink()
        target = self.inputs / "proof.rs"
        target.symlink_to("nonexistent-original")
        with self.assertRaisesRegex(ValueError, "already exists"):
            self.prepare()
        self.assertTrue(target.is_symlink())

    def test_full_tuple_domain_and_independent_widened_piecewise_specification(self):
        source = self.harness.read_text()
        healthy = source.split("fn lifecycle_source_fields_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        self.assertEqual(healthy.count("kani::any::<u8>()"), 2)
        self.assertEqual(healthy.count("kani::any::<u64>()"), 1)
        self.assertEqual(len(re.findall(r"kani::any", healthy)), 3)
        self.assertIn("(ordinal < 8 && maturity as u128 == ordinal as u128)", healthy)
        self.assertIn("|| (ordinal >= 8 && maturity == 8)", healthy)
        self.assertIn("let penalty_relation = penalty <= 32;", healthy)
        self.assertIn("let required = penalty_relation && maturity_relation;", healthy)
        self.assertIn("let observed = source_fields_valid(input);", healthy)
        self.assertIn('assert_eq!(observed, required, "source fields exact classification");', healthy)
        self.assertIn("if maturity_relation {", healthy)
        self.assertIn("if penalty_relation {", healthy)
        self.assertIn('assert!(!observed, "source fields required rejection");', healthy)
        for forbidden in ("kani::assume", "#[kani::unwind", ".min(", "u64::from", "MATURE_UPDATES",
                          "DISTANT_PENALTY", "for ", "while ", ".unwrap(", ".expect(", "return ", "garde"):
            self.assertNotIn(forbidden, source)

    def test_five_assertions_twelve_literal_tuple_covers_and_focused_controls(self):
        source = self.harness.read_text()
        healthy = source.split("fn lifecycle_source_fields_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        self.assertEqual(len(re.findall(r"\bassert(?:_eq)?!", healthy)), 5)
        self.assertEqual(healthy.count("kani::cover!("), 12)
        for label in PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]:
            self.assertEqual(healthy.count('"' + label + '"'), 1)
        for label in PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]:
            self.assertEqual(healthy.count('"' + label + '"'), 1)
        covers = re.findall(r"kani::cover!\(\s*input == \((.*?)\) && (!?observed),", healthy)
        self.assertEqual(covers, [("0, 0, 0", "observed"), ("7, 31, 7", "observed"),
                         ("8, 32, 8", "observed"), ("8, 0, 9", "observed"),
                         ("8, 32, u64::MAX", "observed"), ("7, 32, 8", "!observed"),
                         ("8, 33, 8", "!observed"), ("8, 255, 8", "!observed"),
                         ("9, 0, 9", "!observed"), ("255, 0, u64::MAX", "!observed"),
                         ("0, 0, 1", "!observed"), ("1, 0, 0", "!observed")])
        harnesses = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", source))
        self.assertEqual(harnesses, {job["harness"].split("::")[-1] for job in PROJECTION.schedule()})
        for case, literal in (("strict_penalty", "source_fields_valid((8, 32, 8))"),
                              ("wrong_maturity", "source_fields_valid((8, 0, 8))"),
                              ("loose_relationship", "!source_fields_valid((0, 0, 1))")):
            block = source.split("fn lifecycle_source_fields_control_" + case + "() {", 1)[1].split("#[kani::proof]", 1)[0]
            self.assertEqual(block.count("assert!("), 1)
            self.assertIn(literal, block)
            self.assertIn('"' + PROJECTION.CONTROLS[case]["intended_failure"] + '"', block)
            self.assertNotIn("full_domain", block)
            self.assertNotIn("kani::any", block)

    def test_registry_schedule_contract_and_previous_finite_counts(self):
        jobs = SUITES.jobs("lifecycle-source-fields")
        self.assertEqual(jobs, PROJECTION.schedule())
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "strict_penalty", "wrong_maturity", "loose_relationship", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertEqual(SUITES.expected_contract("lifecycle_source_fields", PROJECTION.FULL_HARNESS),
                         (frozenset(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]),
                          frozenset(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS])))
        for job in jobs[1:4]:
            with self.assertRaises(ValueError):
                SUITES.expected_contract(job["subject"], job["harness"])
        jobs[0]["case"] = "caller modification"
        self.assertEqual(SUITES.jobs("lifecycle-source-fields")[0]["case"], "healthy")
        self.assertEqual([len(SUITES.jobs(selector)) for selector in
                         ("request-timing", "selection-depth", "lifecycle-ordinal", "fraction-addition-partitions")],
                         [5, 6, 5, 16])
        self.assertEqual(SUITES.MAX_JOBS, 12)

    def test_registry_freezes_exact_input_inventory(self):
        self.assertEqual(SUITES.input_names("lifecycle-source-fields"), [
            "tools/formal/additional_suites.py", "tools/formal/lifecycle_source_fields_harness.rs",
            "tools/formal/lifecycle_source_fields_projection.py", "tools/formal/test_lifecycle_source_fields.py",
            "tools/formal/verify_lifecycle_source_fields.py", "tools/formal/test_verify_lifecycle_source_fields.py",
            "crates/time-core/src/combination/lifecycle.rs"])
        for selector in ("lifecycle_source_fields", "other", None, True):
            with self.subTest(selector=selector), self.assertRaises(ValueError):
                SUITES.input_names(selector)

    def test_registry_rejects_redirects_and_changed_jobs_before_loading(self):
        job = SUITES.jobs("lifecycle-source-fields")[0]
        invalid = [(job, 0), (job, True), (job, 2), (job, 6)]
        invalid += [(dict(job, **{field: value}), 1) for field, value in
                    (("case", "strict_penalty"), ("subject", "request_timing"), ("harness", "other"),
                     ("restored_healthy", True), ("intended_failure", "other"), ("extra", True))]
        def forbidden_loader(name, path):
            self.fail("An altered job reached source loading")
        for altered, index in invalid:
            with self.subTest(job=altered, index=index), self.assertRaises(ValueError):
                SUITES.prepare(forbidden_loader, self.source, self.inputs, altered, index)

    def test_registry_uses_only_supplied_frozen_loader_and_exact_entrypoint(self):
        calls = []
        def controlled_loader(name, path):
            calls.append((name, path))
            self.assertEqual(path, self.source / "tools/formal/lifecycle_source_fields_projection.py")
            return PROJECTION
        harness_target = self.source / "tools/formal/lifecycle_source_fields_harness.rs"
        harness_target.parent.mkdir(parents=True)
        harness_target.write_bytes(self.harness.read_bytes())
        with patch.object(SUITES.importlib.util, "spec_from_file_location",
                          side_effect=AssertionError("Unexpected registry loader")):
            result = SUITES.prepare(controlled_loader, self.source, self.inputs,
                                    SUITES.jobs("lifecycle-source-fields")[0], 1)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result["entrypoint"], self.inputs / "proof.rs")

    def test_registry_rejects_projection_entrypoint_redirect(self):
        class RedirectedProjection:
            @staticmethod
            def prepare_lifecycle_source_fields(*args):
                return {"entrypoint": self.base / "outside.rs"}
        with self.assertRaisesRegex(ValueError, "inputs/proof.rs"):
            SUITES.prepare(lambda name, path: RedirectedProjection, self.source, self.inputs,
                           SUITES.jobs("lifecycle-source-fields")[0], 1)

    def test_fixed_nonresetting_clock_uses_worse_elapsed_and_rejects_invalid_time(self):
        self.assertEqual(PROJECTION.PREPARATION_STARTED_UNIX_NS, 1793944926242703570)
        self.assertEqual(PROJECTION.PREPARATION_STARTED_MONOTONIC_NS, 41594076128365)
        self.assertEqual(PROJECTION.PREPARATION_DEADLINE_MONOTONIC_NS, 45194076128365)
        self.assertEqual(PROJECTION.TOTAL_SECONDS, 3600)
        self.assertEqual(PROJECTION.PREPARATION_BOOT_ID, "d3fc5615-e867-45cd-b74f-4a7413afc5ab")
        for boot, mono_delta, wall_delta, expected in (
                (PROJECTION.PREPARATION_BOOT_ID, 700, 800, 2800),
                ("foreign boot", 1, 1, None), (PROJECTION.PREPARATION_BOOT_ID, 3600, 1, None),
                (PROJECTION.PREPARATION_BOOT_ID, 1, 3600, None),
                (PROJECTION.PREPARATION_BOOT_ID, -1, 1, None),
                (PROJECTION.PREPARATION_BOOT_ID, 1, -1, None)):
            with self.subTest(boot=boot, mono=mono_delta, wall=wall_delta), \
                 patch.object(Path, "read_text", return_value=boot), \
                 patch.object(PROJECTION.time, "monotonic_ns", return_value=
                              PROJECTION.PREPARATION_STARTED_MONOTONIC_NS + mono_delta * 1_000_000_000), \
                 patch.object(PROJECTION.time, "time_ns", return_value=
                              PROJECTION.PREPARATION_STARTED_UNIX_NS + wall_delta * 1_000_000_000):
                if expected is None:
                    with self.assertRaises(ValueError):
                        PROJECTION.remaining_budget()
                else:
                    self.assertEqual(PROJECTION.remaining_budget(), expected)


if __name__ == "__main__":
    unittest.main()
