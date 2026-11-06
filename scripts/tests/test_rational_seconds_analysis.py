"""Independent singleton domain admission controls; no Rust/analyzer execution."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("rational_seconds_controls", ROOT / "scripts/verify-time-domain-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def digest(value):
    """Independently encode the documented canonical complete file map."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def literal_execution(*, doctests):
    """Literal records for seven raw guards, six public cases and five examples."""
    text = """running 7 tests
test rational_seconds::tests::transformed_seconds_faults_have_valid_scalar_neighbors ... ok
test rational_seconds::tests::transformed_bounds_require_power_and_exactness_association ... ok
test rational_seconds::tests::raw_roundoff_rule_rejects_invalid_normals_without_publication_guards ... ok
test rational_seconds::tests::zero_publication_rejects_noncanonical_associations ... ok
test rational_seconds::tests::checked_helper_faults_do_not_use_truncated_shifts ... ok
test rational_seconds::tests::diagnostics_omit_numeric_payloads_and_use_fixed_errors ... ok
test rational_seconds::tests::raw_unsigned_magnitude_is_admitted_before_sign_application ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
running 6 tests
test rounds_exact_ratio_once_without_operand_double_rounding ... ok
test literal_ratios_cover_full_integer_envelope_and_tie_parities ... ok
test adjacent_rational_neighbors_preserve_ties_and_original_binade_bound ... ok
test equivalent_unreduced_ratios_retain_original_tuples ... ok
test engine_denominator_literals_preserve_seconds_without_nanosecond_rounding ... ok
test counted_binary_scalings_span_full_unsigned_shift_domain ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"""
    if doctests:
        text += """running 5 tests
test crates/time-domain/src/rational_seconds.rs - rational_seconds::RoundedRationalSeconds (line 100) ... ok
test crates/time-domain/src/rational_seconds.rs - rational_seconds::RoundedRationalSeconds (line 110) - compile fail ... ok
test crates/time-domain/src/rational_seconds.rs - rational_seconds::RoundedRationalSeconds (line 120) - compile fail ... ok
test crates/time-domain/src/rational_seconds.rs - rational_seconds::RoundedRationalSeconds (line 130) - compile fail ... ok
test crates/time-domain/src/rational_seconds.rs - rational_seconds::RoundedRationalSeconds (line 140) - compile fail ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"""
    return text


class RationalSecondsAnalysisTests(unittest.TestCase):
    """Actual isolated source admission plus positive and intended rejection controls."""

    def fixture(self, root):
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        names = set(identity) | set(MODULE.RATIONAL_SECONDS_BOUND_INPUTS) | {MODULE.RATIONAL_SECONDS_POLICY}
        for name in sorted(names):
            target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
            if (ROOT / name).is_file():
                shutil.copyfile(ROOT / name, target)
            else:
                # Python-only source-custody control, never compiled or claimed as a product result.
                target.write_text("// Synthetic unexecuted source-admission fixture.\n")
        manifest = root / "policy/verification-manifest.toml"
        text = manifest.read_text()
        if MODULE.RATIONAL_SECONDS_FILE not in text:
            text = text.replace("security_modules = [", "security_modules = [\n"
                '    { name = "synthetic-rational-owner", package = "time-domain", files = ["'
                + MODULE.RATIONAL_SECONDS_FILE + '"], scope = "unexecuted admission control" },', 1)
            text = text.replace("critical_files = [", 'critical_files = [\n    "'
                                + MODULE.RATIONAL_SECONDS_FILE + '",', 1)
            manifest.write_text(text)
        _, identity = MODULE.PRODUCT.admit_product(root)
        policy = {**copy.deepcopy(MODULE.RATIONAL_SECONDS_FIXED),
                  "candidate_sha256": digest(identity),
                  "inputs_sha256": {name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                                    for name in MODULE.RATIONAL_SECONDS_BOUND_INPUTS},
                  "mutation_inventories": {}, "mutation_equivalences": [],
                  "scope": "Unexecuted isolated singleton admission controls."}
        self.write(root, policy)
        return policy

    def write(self, root, policy):
        (root / MODULE.RATIONAL_SECONDS_POLICY).write_text(json.dumps(policy))

    def test_scope_and_healthy_complete_candidate_use_real_product_admission(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy = self.fixture(root)
            self.assertEqual(len(MODULE.PRODUCT.admit_product(root)[1]), 374)
            self.assertEqual(MODULE.read_rational_seconds_policy(root), policy)
            self.assertEqual(MODULE.RATIONAL_SECONDS_SCOPE.select("all"),
                             {"rational_seconds": "crates/time-domain/src/rational_seconds.rs"})
            self.assertNotIn("rational_seconds", MODULE.SUBJECTS)
            self.assertEqual(MODULE.SCOPE.policy, "policy/time-domain-analysis.json")
            self.assertEqual(json.loads((ROOT / "policy/time-domain-analysis.json").read_text())["source_file_count"], 166)

    def test_scope_thresholds_profiles_budgets_types_and_unknown_fields_cannot_change(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy = self.fixture(root)
            for field, value in [("package", "nevrotick-engine"), ("source_file_count", 373),
                                 ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                                 ("tool_admission_profile", "stable-core"), ("jobs", True),
                                 ("mutation_jobs", 8), ("mutation_process_limit", 128),
                                 ("maximum_mutants", 513), ("maximum_listed_per_file", 512),
                                 ("coverage_wall_seconds", 301), ("critical_files", []), ("unknown", 1)]:
                bad = copy.deepcopy(healthy); bad[field] = value; self.write(root, bad)
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_rational_seconds_policy(root)
            self.write(root, healthy)
            self.assertEqual(MODULE.read_rational_seconds_policy(root), healthy)

    def test_candidate_missing_extra_input_and_refreshed_local_pins_remain_fail_closed(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy = self.fixture(root)
            source = root / MODULE.RATIONAL_SECONDS_TEST_FILE
            original = source.read_bytes(); source.unlink()
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_rational_seconds_policy(root)
            source.write_bytes(original)
            extra = root / "crates/time-domain/tests/unreviewed.rs"; extra.write_text("// unreviewed\n")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_rational_seconds_policy(root)
            extra.unlink()
            source.write_bytes(original + b"// changed fixture\n")
            bad = copy.deepcopy(healthy)
            bad["inputs_sha256"][MODULE.RATIONAL_SECONDS_TEST_FILE] = hashlib.sha256(source.read_bytes()).hexdigest()
            self.write(root, bad)
            with self.assertRaisesRegex(MODULE.Incomplete, "374-input product"):
                MODULE.read_rational_seconds_policy(root)

    def test_protected_helpers_missing_extra_and_actual_content_drift_reject(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy = self.fixture(root)
            for change in ("missing", "extra", "content"):
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["inputs_sha256"].pop("scripts/portable-analysis.py")
                if change == "extra": bad["inputs_sha256"]["unreviewed.py"] = "0" * 64
                if change == "content": (root / "scripts/portable-analysis.py").write_text("# changed helper\n")
                self.write(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "helper bindings"):
                    MODULE.read_rational_seconds_policy(root)

    def test_inventory_must_be_reviewed_nonempty_exact_and_bounded_before_mutation(self):
        policy = {**MODULE.RATIONAL_SECONDS_FIXED, "mutation_inventories": {}}
        self.assertEqual(MODULE.rational_seconds_inventory_bindings(policy), {})
        for bindings in ({}, {"foreign": {"count": 1, "sha256": "a" * 64}},
                         {"rational_seconds": {"count": 0, "sha256": "a" * 64}},
                         {"rational_seconds": {"count": True, "sha256": "a" * 64}},
                         {"rational_seconds": {"count": 513, "sha256": "a" * 64}},
                         {"rational_seconds": {"count": 1, "sha256": "bad"}}):
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.rational_seconds_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {"rational_seconds": {"count": 1, "sha256": "a" * 64}}
        self.assertEqual(MODULE.rational_seconds_inventory_bindings(policy, required=True), policy["mutation_inventories"])

    def test_wrong_role_and_empty_inventory_reject_before_session_or_legacy_admission(self):
        args = SimpleNamespace(subject="rational_seconds", lane="coverage", tool_admission_profile="full",
                               output_dir=Path("/unused/output"), vendor_dir=Path("/unused/vendor"))
        for change, value in [("tool_admission_profile", "stable-core"), ("lane", "contracts"),
                              ("subject", "all"), ("cohort_plan", Path("/foreign/plan"))]:
            bad = copy.copy(args); setattr(bad, change, value)
            with self.subTest(change=change), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete): MODULE.execute_rational_seconds(bad, {})
                session.assert_not_called()
        for lane in ("mutation", "all"):
            args.lane = lane
            with self.subTest(lane=lane), patch.object(MODULE, "read_rational_seconds_policy",
                  return_value={**MODULE.RATIONAL_SECONDS_FIXED, "mutation_inventories": {}}), \
                  patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete): MODULE.execute_rational_seconds(args, {})
                session.assert_not_called()

    def test_cli_dispatch_avoids_legacy_family_and_preserves_explicit_full_profile(self):
        def selected(args):
            self.assertEqual((args.subject, args.lane, args.tool_admission_profile),
                             ("rational_seconds", "enumerate", "full"))
            return 0
        with patch.object(MODULE, "main_rational_seconds", side_effect=selected), patch.object(MODULE, "execute") as legacy:
            self.assertEqual(MODULE.main(["--subject", "rational_seconds", "--lane", "enumerate",
                                          "--output-dir", "/new/output"]), 0)
            legacy.assert_not_called()
        args = SimpleNamespace(subject="rational_seconds", lane="all")
        with patch.object(MODULE, "execute_rational_seconds") as selected, patch.object(MODULE, "read_policy") as legacy:
            MODULE.execute(args, {})
            selected.assert_called_once()
            legacy.assert_not_called()

    def test_literal_coverage_counts_are_exact_owner_nonempty_and_per_file(self):
        report = {"data": [{"files": [{"filename": "/work/source/" + MODULE.RATIONAL_SECONDS_FILE,
                    "summary": {"lines": {"count": 10, "covered": 9}}}]}]}
        self.assertEqual(MODULE.COMMON.coverage_counts(MODULE.RATIONAL_SECONDS_SCOPE, report,
                         "/work/source/" + MODULE.RATIONAL_SECONDS_FILE)["line_percent"], 90)
        for change in ("below", "empty", "wrong", "duplicate"):
            bad = copy.deepcopy(report); files = bad["data"][0]["files"]
            if change == "below": files[0]["summary"]["lines"]["covered"] = 8
            if change == "empty": files[0]["summary"]["lines"] = {"count": 0, "covered": 0}
            if change == "wrong": files[0]["filename"] = "/foreign/rational_seconds.rs"
            if change == "duplicate": files.append(copy.deepcopy(files[0]))
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.coverage_counts(MODULE.RATIONAL_SECONDS_SCOPE, bad,
                                              "/work/source/" + MODULE.RATIONAL_SECONDS_FILE)

    def test_actual_baseline_requires_every_independent_external_case_and_five_doctests(self):
        ordinary = literal_execution(doctests=False)
        self.assertEqual(MODULE.rational_seconds_test_execution(ordinary, doctests=False)["tests_executed"], 13)
        self.assertEqual(MODULE.rational_seconds_test_execution(literal_execution(doctests=True), doctests=True)
                         ["selected_doctests"], 5)
        rows = [line for line in ordinary.splitlines() if line.startswith("test ") and line.endswith(" ... ok")]
        self.assertEqual(len(rows), 6)
        for row in rows:
            summary = ("7 passed;", "6 passed;") if "rational_seconds::tests::" in row else ("6 passed;", "5 passed;")
            bad = ordinary.replace(row + "\n", "").replace(*summary)
            with self.subTest(omitted=row), self.assertRaisesRegex(MODULE.Incomplete, "actual passing"):
                MODULE.rational_seconds_test_execution(bad, doctests=False)

    def instrumented_execution(self):
        """Synthetic native-format split completion plus independent required cases."""
        return """running 1 test
test rational_seconds::private_rules::validated_output ... RATIONAL_SECONDS_PROFILE case=projected_batch_private_admission garde_callbacks=3 entry_read_attempts=5
RATIONAL_SECONDS_PROFILE case=projected_batch_private_admission histories=1 appends=3 observations=3 contexts=1 monotonic=5 coordinates=12 exchanges=3 getters=6 terms=1 projections=1 enclosures=1 batches=3 minima=0 selections=0
ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
""" + literal_execution(doctests=True)

    def test_native_instrumented_test_completion_matches_its_named_summary(self):
        result = MODULE.rational_seconds_test_execution(self.instrumented_execution(), doctests=True)
        self.assertEqual(result["tests_executed"], 19)
        self.assertEqual(result["selected_doctests"], 5)

    def test_instrumented_test_requires_unique_named_complete_success(self):
        healthy = self.instrumented_execution()
        header = "test rational_seconds::private_rules::validated_output ... "
        for bad in (healthy.replace("\nok\n", "\n", 1),
                    healthy.replace("\nok\n", "\nFAILED\n", 1),
                    healthy.replace("\nok\n", "\nignored\n", 1),
                    healthy.replace("\nok\n", "\nunattributed output\nok\n", 1),
                    healthy.replace(header, "unattributed instrumentation ", 1),
                    healthy.replace("\nok\n", "\nok\nok\n", 1),
                    healthy.replace("\nok\n", "\ntest nested ... ok\nok\n", 1),
                    healthy + header + "CORE_PROFILE_COUNTS fits=0 points=8\n"):
            with self.subTest(bad=bad[-150:]), self.assertRaises(MODULE.Incomplete):
                MODULE.rational_seconds_test_execution(bad, doctests=True)
        self.assertEqual(MODULE.rational_seconds_test_execution(healthy, doctests=True)["tests_executed"], 19)

    def test_crate_level_doctest_counts_without_replacing_owned_privacy_examples(self):
        crate = """running 1 test
test crates/time-domain/src/lib.rs - (line 18) ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"""
        result = MODULE.rational_seconds_test_execution(crate + literal_execution(doctests=True), doctests=True)
        self.assertEqual((result["all_package_doctests"], result["selected_doctests"]), (6, 5))

    def test_baseline_rejects_zero_failed_ignored_filtered_printed_and_redirected_records(self):
        healthy = literal_execution(doctests=True)
        for bad in ("test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;",
                    healthy.replace(" ... ok", " ... ignored", 1),
                    healthy.replace("0 failed;", "1 failed;", 1),
                    healthy.replace("0 filtered out;", "1 filtered out;", 1),
                    "\n".join(MODULE.RATIONAL_SECONDS_REQUIRED_TESTS),
                    healthy.replace("src/rational_seconds.rs", "src/lib.rs"),
                    healthy.replace("(line 110) - compile fail", "(line 110)"),
                    healthy.replace("rational_seconds::RoundedRationalSeconds (line 110)",
                                    "unreviewed::Other (line 110)"),
                    literal_execution(doctests=False)):
            with self.subTest(bad=bad[-150:]), self.assertRaises(MODULE.Incomplete):
                MODULE.rational_seconds_test_execution(bad, doctests=True)
        with self.assertRaisesRegex(MODULE.Incomplete, "unexpectedly includes doctests"):
            MODULE.rational_seconds_test_execution(healthy, doctests=False)

    def test_actual_owned_baseline_log_rejects_missing_failed_duplicate_escape_and_link(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); directory = work / "mutation/rational_seconds/mutants.out"
            directory.mkdir(parents=True)
            (directory / "baseline.log").write_text(literal_execution(doctests=True))
            baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"}
            path = directory / "outcomes.json"
            path.write_text(json.dumps({"outcomes": [baseline]}))
            self.assertEqual(MODULE.rational_seconds_baseline(work)["tests_executed"], 18)
            for malformed in ([], {"outcomes": None}, {"outcomes": {}}):
                path.write_text(json.dumps(malformed))
                with self.subTest(malformed=malformed), self.assertRaisesRegex(MODULE.Incomplete, "invalid shape"):
                    MODULE.rational_seconds_baseline(work)
            for rows in ([], [baseline, baseline], [{**baseline, "summary": "Failure"}],
                         [{**baseline, "log_path": "../escape.log"}],
                         [{**baseline, "log_path": "/absolute.log"}]):
                path.write_text(json.dumps({"outcomes": rows}))
                with self.subTest(rows=rows), self.assertRaises(MODULE.Incomplete):
                    MODULE.rational_seconds_baseline(work)
            (directory / "linked.log").symlink_to(directory / "baseline.log")
            path.write_text(json.dumps({"outcomes": [{**baseline, "log_path": "linked.log"}]}))
            with self.assertRaisesRegex(MODULE.Incomplete, "links"):
                MODULE.rational_seconds_baseline(work)

    def mutant(self):
        """Synthetic complete descriptor controls supply no actual generated-mutant credit."""
        return {"name": "synthetic-rational-fault", "package": "time-domain", "file": MODULE.RATIONAL_SECONDS_FILE,
                "function": {"function_name": "derive", "return_type": "-> Result<(), Error>",
                             "span": {"start": {"line": 1, "column": 1}, "end": {"line": 3, "column": 2}}},
                "span": {"start": {"line": 2, "column": 1}, "end": {"line": 2, "column": 2}},
                "replacement": ">=", "genre": "BinaryOperator"}

    def test_listing_only_has_exact_owner_commands_complete_binding_and_no_campaign_credit(self):
        calls = []
        def listing(label, command, timeout):
            calls.append((label, command, timeout))
            return MODULE.COMMON.mutation_listing_warning(10) + json.dumps([self.mutant()])
        session = SimpleNamespace(tools=Path("/owned/tools"), run=listing)
        report = {}
        MODULE.enumerate_rational_seconds(session, MODULE.RATIONAL_SECONDS_FIXED, report)
        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0][1][-2:], ["--list", "--json"])
        self.assertNotIn("--baseline", calls[0][1])
        self.assertEqual(calls[0][1][calls[0][1].index("--file") + 1], MODULE.RATIONAL_SECONDS_FILE)
        self.assertEqual(report["listed_total"], 1)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        policy = {**MODULE.RATIONAL_SECONDS_FIXED, "mutation_inventories": report["listed_inventory_bindings"]}
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.RATIONAL_SECONDS_SCOPE, policy,
                                                         report["listed_inventories"])
        changed = copy.deepcopy(report["listed_inventories"])
        changed["rational_seconds"]["synthetic-rational-fault"]["replacement"] = "<="
        with self.assertRaisesRegex(MODULE.Incomplete, "differs from its reviewed"):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.RATIONAL_SECONDS_SCOPE, policy, changed)

    def equivalence_fixture(self, root):
        """Exercise actual shared custody with a synthetic review, never semantic E approval."""
        policy = self.fixture(root)
        proof = root / "tasks/rational-seconds-equivalences.md"; proof.parent.mkdir(exist_ok=True)
        proof.write_text("## RATIONAL-SYNTHETIC-E\nUnexecuted admission control, not product equivalence.\n")
        entry = {"id": "RATIONAL-SYNTHETIC-E", "candidate_sha256": policy["candidate_sha256"],
                 "source_sha256": hashlib.sha256((root / MODULE.RATIONAL_SECONDS_FILE).read_bytes()).hexdigest(),
                 "mutant": self.mutant(), "requirement": "CHECK-7/8/25", "rationale": "Synthetic source-custody control.",
                 "proof": {"path": "tasks/rational-seconds-equivalences.md",
                           "sha256": hashlib.sha256(proof.read_bytes()).hexdigest(), "section": "RATIONAL-SYNTHETIC-E",
                           "reviewer": "independent synthetic control", "verdict": "independently-justified-equivalent"}}
        policy["mutation_equivalences"] = [entry]; self.write(root, policy)
        return policy, proof

    def test_exact_equivalence_requires_actual_source_candidate_proof_and_review(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, proof = self.equivalence_fixture(root)
            self.assertEqual(MODULE.read_rational_seconds_policy(root), healthy)
            for change in ("candidate", "source", "proof", "owner", "duplicate", "section", "reviewer", "descriptor"):
                bad = copy.deepcopy(healthy); case = bad["mutation_equivalences"][0]
                if change == "candidate": case["candidate_sha256"] = "0" * 64
                if change == "source": case["source_sha256"] = "0" * 64
                if change == "proof": case["proof"]["sha256"] = "0" * 64
                if change == "owner": case["mutant"]["file"] = "crates/time-domain/src/regression.rs"
                if change == "duplicate": bad["mutation_equivalences"].append(copy.deepcopy(case))
                if change == "section": case["proof"]["section"] = "MISSING"
                if change == "reviewer": case["proof"]["reviewer"] = ""
                if change == "descriptor": case["mutant"].pop("span")
                self.write(root, bad)
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_rational_seconds_policy(root)
            self.write(root, healthy); proof.write_text("## RATIONAL-SYNTHETIC-E\nChanged review.\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                MODULE.read_rational_seconds_policy(root)

    def test_all_lane_uses_one_session_coverage_before_mutation_and_preserves_budgets(self):
        for covered in (9, 8):
            with self.subTest(covered=covered), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); policy = self.fixture(root)
                policy["mutation_inventories"] = {"rational_seconds": {"count": 1, "sha256": "a" * 64}}
                self.write(root, policy)
                _, identity = MODULE.PRODUCT.admit_product(root)
                work = root / "owned-work"; work.mkdir()
                events = []
                def run(label, command, timeout, **options):
                    self.assertEqual(label, "coverage")
                    self.assertEqual(timeout, 300)
                    self.assertEqual(command[command.index("--test") + 1], "rational_seconds")
                    self.assertNotIn("--fail-under-file-lines", command)
                    events.append("coverage")
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [{
                        "filename": "/work/source/" + MODULE.RATIONAL_SECONDS_FILE,
                        "summary": {"lines": {"count": 10, "covered": covered}}}]}]}))
                    return literal_execution(doctests=False)
                def session_factory(actual_root, args, report, scope, admitted, extra):
                    self.assertEqual((actual_root, args.lane, scope), (root, "all", MODULE.RATIONAL_SECONDS_SCOPE))
                    events.append("session")
                    return SimpleNamespace(product_identity=identity, source=root, work=work,
                                           tools=root / "owned-tools", run=run,
                                           finish=lambda: events.append("finish"))
                def mutation(scope, subjects, admitted, actual_work, tools, execute, report, lane):
                    self.assertEqual((lane, admitted["mutation_wall_seconds"]), ("mutation", 1200))
                    self.assertEqual(subjects, {"rational_seconds": MODULE.RATIONAL_SECONDS_FILE})
                    self.assertEqual(report["coverage"]["rational_seconds"]["line_percent"], 90)
                    events.append("mutation")
                args = SimpleNamespace(subject="rational_seconds", lane="all", tool_admission_profile="full",
                                       output_dir=root / "output", vendor_dir=root / "vendor")
                report = {}
                with patch.object(MODULE, "ROOT", root), patch.object(MODULE.COMMON, "Session", side_effect=session_factory) as sessions, \
                     patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=mutation) as mutations:
                    if covered == 9:
                        MODULE.execute_rational_seconds(args, report)
                        self.assertTrue(report["qualification_passed"])
                        self.assertEqual(events, ["session", "coverage", "mutation", "finish"])
                        mutations.assert_called_once()
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, "below 90"):
                            MODULE.execute_rational_seconds(args, report)
                        self.assertFalse(report["qualification_passed"])
                        self.assertEqual(events, ["session", "coverage", "finish"])
                        mutations.assert_not_called()
                    sessions.assert_called_once()

    def test_json_admission_rejects_duplicate_nonfinite_overflow_and_linked_policy(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); path = root / "policy.json"
            for text in ('{"x":1,"x":2}', '{"x":NaN}', '{"x":1e999}'):
                path.write_text(text)
                with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                    MODULE.rational_seconds_document(path)
            path.write_text('{"x":0.5}')
            self.assertEqual(MODULE.rational_seconds_document(path), {"x": 0.5})
            path.write_text(" " * (512 * 1024 + 1))
            with self.assertRaises(MODULE.Incomplete): MODULE.rational_seconds_document(path)
            path.write_text('{"x":0.5}')
            linked = root / "linked.json"; linked.symlink_to(path)
            with self.assertRaises(MODULE.Incomplete): MODULE.rational_seconds_document(linked)

    def test_inline_test_padding_and_missing_security_owner_reject_even_with_refreshed_pins(self):
        for change in ("inline", "owner"):
            with self.subTest(change=change), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); policy = self.fixture(root)
                if change == "inline":
                    source = root / MODULE.RATIONAL_SECONDS_FILE
                    source.write_text(source.read_text() + "\n#[test]\nfn synthetic_padding() {}\n")
                else:
                    manifest = root / "policy/verification-manifest.toml"
                    lines = manifest.read_text().splitlines(keepends=True)
                    index = next(index for index, line in enumerate(lines)
                                 if MODULE.RATIONAL_SECONDS_FILE in line and "package" in line)
                    lines[index] = lines[index].replace('package = "time-domain"', 'package = "time-core"')
                    manifest.write_text("".join(lines))
                _, identity = MODULE.PRODUCT.admit_product(root)
                policy["candidate_sha256"] = digest(identity)
                policy["inputs_sha256"] = {name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                                          for name in MODULE.RATIONAL_SECONDS_BOUND_INPUTS}
                self.write(root, policy)
                with self.assertRaisesRegex(MODULE.Incomplete, "inline test|critical/security"):
                    MODULE.read_rational_seconds_policy(root)


if __name__ == "__main__":
    unittest.main()
