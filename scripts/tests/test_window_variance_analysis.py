"""Independent singleton Core admission controls; no Rust/analyzer execution."""
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
SPEC = importlib.util.spec_from_file_location("window_variance_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def digest(value):
    """Independently encode the documented canonical complete file map."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def literal_execution(*, doctests):
    """Literal native-format controls for 34 reviewed cases and six examples."""
    text = """running 34 tests
test debug_projection_omits_distances_and_precision ... ok
test error_diagnostics_are_bounded_without_validation_payloads ... ok
test whole_window_quality_uses_precision_and_squared_scale ... ok
test upper_clamped_spread_has_exact_dyadic_divisors ... ok
test interior_spread_uses_the_point_seven_coefficient ... ok
test precision_above_minimum_is_legal_and_wins_the_clamp ... ok
test legal_divisors_are_not_capped_at_sixteen ... ok
test even_median_preserves_original_order_and_storage ... ok
test permutations_preserve_distance_to_divisor_correspondence ... ok
test changing_the_window_reweights_retained_rows ... ok
test safe_power_of_two_unit_scales_preserve_divisor_bits ... ok
test excess_threshold_has_exact_neighbor_results ... ok
test lower_spread_clamp_has_a_plateau_and_valid_upper_neighbor ... ok
test upper_spread_clamp_has_a_plateau_and_valid_lower_neighbor ... ok
test two_rows_are_a_legal_weighting_window ... ok
test sixty_four_rows_retain_the_last_distance_and_divisor ... ok
test empty_and_sixty_five_row_windows_are_rejected ... ok
test count_errors_precede_scalar_validation ... ok
test every_invalid_precision_class_is_rejected ... ok
test every_invalid_distance_class_is_rejected_at_every_position ... ok
test precision_error_precedes_distance_validation ... ok
test original_distance_index_precedes_later_errors_and_arithmetic ... ok
test positive_precision_extremes_are_accepted ... ok
test positive_distance_extremes_are_accepted ... ok
test positive_subnormal_inputs_produce_exact_nonunit_divisors ... ok
test subnormal_even_median_preserves_sum_then_half_rounding ... ok
test subnormal_even_median_does_not_split_half_products ... ok
test even_median_sum_overflow_is_rejected_with_healthy_controls ... ok
test spread_overflow_cannot_be_hidden_by_the_clamp ... ok
test threshold_overflow_is_rejected_with_a_finite_neighbor ... ok
test ratio_overflow_is_rejected_with_a_subnormal_healthy_control ... ok
test square_overflow_is_rejected_with_a_finite_large_neighbor ... ok
test minimum_normal_reciprocal_boundary_is_accepted ... ok
test positive_subnormal_reciprocals_do_not_narrow_output_admission ... ok
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"""
    if doctests:
        text += """running 6 tests
test crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 100) ... ok
test crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 110) - compile fail ... ok
test crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 120) - compile fail ... ok
test crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 130) - compile fail ... ok
test crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 140) - compile fail ... ok
test crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 150) - compile fail ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"""
    return text


class WindowVarianceAnalysisTests(unittest.TestCase):
    """Actual isolated source admission plus positive and intended rejection controls."""

    def fixture(self, root):
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        names = set(identity) | set(MODULE.WINDOW_VARIANCE_BOUND_INPUTS) | {MODULE.WINDOW_VARIANCE_POLICY}
        for name in sorted(names):
            target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
            if (ROOT / name).is_file():
                shutil.copyfile(ROOT / name, target)
            else:
                # Python-only source-custody control, never compiled or claimed as a product result.
                target.write_text("// Synthetic unexecuted source-admission fixture.\n")
        manifest = root / "policy/verification-manifest.toml"
        text = manifest.read_text()
        if MODULE.WINDOW_VARIANCE_FILE not in text:
            text = text.replace("security_modules = [", "security_modules = [\n"
                '    { name = "synthetic-window-owner", package = "time-core", files = ["'
                + MODULE.WINDOW_VARIANCE_FILE + '"], scope = "unexecuted admission control" },', 1)
            text = text.replace("critical_files = [", 'critical_files = [\n    "'
                                + MODULE.WINDOW_VARIANCE_FILE + '",', 1)
            manifest.write_text(text)
        _, identity = MODULE.PRODUCT.admit_product(root)
        policy = {**copy.deepcopy(MODULE.WINDOW_VARIANCE_FIXED),
                  "candidate_sha256": digest(identity),
                  "inputs_sha256": {name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                                    for name in MODULE.WINDOW_VARIANCE_BOUND_INPUTS},
                  "mutation_inventories": {}, "mutation_equivalences": [],
                  "scope": "Unexecuted isolated singleton admission controls."}
        self.write(root, policy)
        return policy

    def write(self, root, policy):
        (root / MODULE.WINDOW_VARIANCE_POLICY).write_text(json.dumps(policy))

    def test_scope_and_healthy_complete_candidate_use_real_product_admission(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy = self.fixture(root)
            self.assertEqual(len(MODULE.PRODUCT.admit_product(root)[1]), 370)
            self.assertEqual(MODULE.read_window_variance_policy(root), policy)
            self.assertEqual(MODULE.WINDOW_VARIANCE_SCOPE.select("all"),
                             {"window_variance": "crates/time-core/src/window_variance.rs"})
            self.assertNotIn("window_variance", MODULE.SUBJECTS)
            self.assertEqual(MODULE.SCOPE.policy, "policy/time-core-analysis.json")
            self.assertEqual(MODULE.FIXED["source_file_count"], 158)

    def test_scope_thresholds_profiles_budgets_types_and_unknown_fields_cannot_change(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy = self.fixture(root)
            for field, value in [("package", "nevrotick-engine"), ("source_file_count", 369),
                                 ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                                 ("tool_admission_profile", "stable-core"), ("jobs", True),
                                 ("mutation_jobs", 8), ("mutation_process_limit", 128),
                                 ("maximum_mutants", 513), ("maximum_listed_per_file", 512),
                                 ("coverage_wall_seconds", 301), ("critical_files", []), ("unknown", 1)]:
                bad = copy.deepcopy(healthy); bad[field] = value; self.write(root, bad)
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_window_variance_policy(root)
            self.write(root, healthy)
            self.assertEqual(MODULE.read_window_variance_policy(root), healthy)

    def test_candidate_missing_extra_input_and_refreshed_local_pins_remain_fail_closed(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy = self.fixture(root)
            source = root / MODULE.WINDOW_VARIANCE_TEST_FILE
            original = source.read_bytes(); source.unlink()
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_window_variance_policy(root)
            source.write_bytes(original)
            extra = root / "crates/time-core/tests/unreviewed.rs"; extra.write_text("// unreviewed\n")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_window_variance_policy(root)
            extra.unlink()
            source.write_bytes(original + b"// changed fixture\n")
            bad = copy.deepcopy(healthy)
            bad["inputs_sha256"][MODULE.WINDOW_VARIANCE_TEST_FILE] = hashlib.sha256(source.read_bytes()).hexdigest()
            self.write(root, bad)
            with self.assertRaisesRegex(MODULE.Incomplete, "370-input product"):
                MODULE.read_window_variance_policy(root)

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
                    MODULE.read_window_variance_policy(root)

    def test_inventory_must_be_reviewed_nonempty_exact_and_bounded_before_mutation(self):
        policy = {**MODULE.WINDOW_VARIANCE_FIXED, "mutation_inventories": {}}
        self.assertEqual(MODULE.window_variance_inventory_bindings(policy), {})
        for bindings in ({}, {"foreign": {"count": 1, "sha256": "a" * 64}},
                         {"window_variance": {"count": 0, "sha256": "a" * 64}},
                         {"window_variance": {"count": True, "sha256": "a" * 64}},
                         {"window_variance": {"count": 513, "sha256": "a" * 64}},
                         {"window_variance": {"count": 1, "sha256": "bad"}}):
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.window_variance_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {"window_variance": {"count": 1, "sha256": "a" * 64}}
        self.assertEqual(MODULE.window_variance_inventory_bindings(policy, required=True), policy["mutation_inventories"])

    def test_wrong_role_and_empty_inventory_reject_before_session_or_legacy_admission(self):
        args = SimpleNamespace(subject="window_variance", lane="coverage", tool_admission_profile="full",
                               output_dir=Path("/unused/output"), vendor_dir=Path("/unused/vendor"))
        for change, value in [("tool_admission_profile", "stable-core"), ("lane", "contracts"),
                              ("subject", "all"), ("cohort_plan", Path("/foreign/plan"))]:
            bad = copy.copy(args); setattr(bad, change, value)
            with self.subTest(change=change), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete): MODULE.execute_window_variance(bad, {})
                session.assert_not_called()
        for lane in ("mutation", "all"):
            args.lane = lane
            with self.subTest(lane=lane), patch.object(MODULE, "read_window_variance_policy",
                  return_value={**MODULE.WINDOW_VARIANCE_FIXED, "mutation_inventories": {}}), \
                  patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete): MODULE.execute_window_variance(args, {})
                session.assert_not_called()

    def test_cli_dispatch_avoids_legacy_family_and_preserves_explicit_full_profile(self):
        def selected(args):
            self.assertEqual((args.subject, args.lane, args.tool_admission_profile),
                             ("window_variance", "enumerate", "full"))
            return 0
        with patch.object(MODULE, "main_window_variance", side_effect=selected), patch.object(MODULE, "execute") as legacy:
            self.assertEqual(MODULE.main(["--subject", "window_variance", "--lane", "enumerate",
                                          "--output-dir", "/new/output"]), 0)
            legacy.assert_not_called()
        args = SimpleNamespace(subject="window_variance", lane="all")
        with patch.object(MODULE, "execute_window_variance") as selected, patch.object(MODULE, "read_policy") as legacy:
            MODULE.execute(args, {})
            selected.assert_called_once()
            legacy.assert_not_called()

    def test_literal_coverage_counts_are_exact_owner_nonempty_and_per_file(self):
        report = {"data": [{"files": [{"filename": "/work/source/" + MODULE.WINDOW_VARIANCE_FILE,
                    "summary": {"lines": {"count": 10, "covered": 9}}}]}]}
        self.assertEqual(MODULE.COMMON.coverage_counts(MODULE.WINDOW_VARIANCE_SCOPE, report,
                         "/work/source/" + MODULE.WINDOW_VARIANCE_FILE)["line_percent"], 90)
        for change in ("below", "empty", "wrong", "duplicate"):
            bad = copy.deepcopy(report); files = bad["data"][0]["files"]
            if change == "below": files[0]["summary"]["lines"]["covered"] = 8
            if change == "empty": files[0]["summary"]["lines"] = {"count": 0, "covered": 0}
            if change == "wrong": files[0]["filename"] = "/foreign/window_variance.rs"
            if change == "duplicate": files.append(copy.deepcopy(files[0]))
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.coverage_counts(MODULE.WINDOW_VARIANCE_SCOPE, bad,
                                              "/work/source/" + MODULE.WINDOW_VARIANCE_FILE)

    def test_actual_baseline_requires_every_independent_external_case_and_six_doctests(self):
        ordinary = literal_execution(doctests=False)
        self.assertEqual(MODULE.window_variance_test_execution(ordinary, doctests=False)["tests_executed"], 34)
        self.assertEqual(MODULE.window_variance_test_execution(literal_execution(doctests=True), doctests=True)
                         ["selected_doctests"], 6)
        rows = [line for line in ordinary.splitlines() if line.startswith("test ") and line.endswith(" ... ok")]
        self.assertEqual(len(rows), 34)
        for row in rows:
            bad = ordinary.replace(row + "\n", "").replace("34 passed;", "33 passed;")
            with self.subTest(omitted=row), self.assertRaisesRegex(MODULE.Incomplete, "actual passing"):
                MODULE.window_variance_test_execution(bad, doctests=False)

    def instrumented_execution(self):
        """Literal split completion from the actual v1 native baseline, plus required controls."""
        return """running 1 test
test projected_batch::projected_batch_guards::projected_batch_private_admission ... CORE_PROJECTED_BATCH_GUARD_PROFILE case=projected_batch_private_admission garde_callbacks=3 entry_read_attempts=5
CORE_PROJECTED_BATCH_PROFILE case=projected_batch_private_admission histories=1 appends=3 observations=3 contexts=1 monotonic=5 coordinates=12 exchanges=3 getters=6 terms=1 projections=1 enclosures=1 batches=3 minima=0 selections=0
ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
""" + literal_execution(doctests=True)

    def test_native_instrumented_test_completion_matches_its_named_summary(self):
        result = MODULE.window_variance_test_execution(self.instrumented_execution(), doctests=True)
        self.assertEqual(result["tests_executed"], 41)
        self.assertEqual(result["selected_doctests"], 6)

    def test_instrumented_test_requires_unique_named_complete_success(self):
        healthy = self.instrumented_execution()
        header = "test projected_batch::projected_batch_guards::projected_batch_private_admission ... "
        for bad in (healthy.replace("\nok\n", "\n", 1),
                    healthy.replace("\nok\n", "\nFAILED\n", 1),
                    healthy.replace("\nok\n", "\nignored\n", 1),
                    healthy.replace("\nok\n", "\nunattributed output\nok\n", 1),
                    healthy.replace(header, "unattributed instrumentation ", 1),
                    healthy.replace("\nok\n", "\nok\nok\n", 1),
                    healthy.replace("\nok\n", "\ntest nested ... ok\nok\n", 1),
                    healthy + header + "CORE_PROFILE_COUNTS fits=0 points=8\n"):
            with self.subTest(bad=bad[-150:]), self.assertRaises(MODULE.Incomplete):
                MODULE.window_variance_test_execution(bad, doctests=True)
        self.assertEqual(MODULE.window_variance_test_execution(healthy, doctests=True)["tests_executed"], 41)

    def test_crate_level_doctest_counts_without_replacing_owned_privacy_examples(self):
        crate = """running 1 test
test crates/time-core/src/lib.rs - (line 18) ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"""
        result = MODULE.window_variance_test_execution(crate + literal_execution(doctests=True), doctests=True)
        self.assertEqual((result["all_package_doctests"], result["selected_doctests"]), (7, 6))

    def test_baseline_rejects_zero_failed_ignored_filtered_printed_and_redirected_records(self):
        healthy = literal_execution(doctests=True)
        for bad in ("test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;",
                    healthy.replace(" ... ok", " ... ignored", 1),
                    healthy.replace("0 failed;", "1 failed;", 1),
                    healthy.replace("0 filtered out;", "1 filtered out;", 1),
                    "\n".join(MODULE.WINDOW_VARIANCE_REQUIRED_TESTS),
                    healthy.replace("src/window_variance.rs", "src/lib.rs"),
                    healthy.replace("(line 110) - compile fail", "(line 110)"),
                    healthy.replace("window_variance::WindowVarianceDivisors (line 110)",
                                    "unreviewed::Other (line 110)"),
                    literal_execution(doctests=False)):
            with self.subTest(bad=bad[-150:]), self.assertRaises(MODULE.Incomplete):
                MODULE.window_variance_test_execution(bad, doctests=True)
        with self.assertRaisesRegex(MODULE.Incomplete, "unexpectedly includes doctests"):
            MODULE.window_variance_test_execution(healthy, doctests=False)

    def test_actual_owned_baseline_log_rejects_missing_failed_duplicate_escape_and_link(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); directory = work / "mutation/window_variance/mutants.out"
            directory.mkdir(parents=True)
            (directory / "baseline.log").write_text(literal_execution(doctests=True))
            baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"}
            path = directory / "outcomes.json"
            path.write_text(json.dumps({"outcomes": [baseline]}))
            self.assertEqual(MODULE.window_variance_baseline(work)["tests_executed"], 40)
            for rows in ([], [baseline, baseline], [{**baseline, "summary": "Failure"}],
                         [{**baseline, "log_path": "../escape.log"}],
                         [{**baseline, "log_path": "/absolute.log"}]):
                path.write_text(json.dumps({"outcomes": rows}))
                with self.subTest(rows=rows), self.assertRaises(MODULE.Incomplete):
                    MODULE.window_variance_baseline(work)
            (directory / "linked.log").symlink_to(directory / "baseline.log")
            path.write_text(json.dumps({"outcomes": [{**baseline, "log_path": "linked.log"}]}))
            with self.assertRaisesRegex(MODULE.Incomplete, "links"):
                MODULE.window_variance_baseline(work)

    def mutant(self):
        """Synthetic complete descriptor controls supply no actual generated-mutant credit."""
        return {"name": "synthetic-window-fault", "package": "time-core", "file": MODULE.WINDOW_VARIANCE_FILE,
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
        MODULE.enumerate_window_variance(session, MODULE.WINDOW_VARIANCE_FIXED, report)
        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0][1][-2:], ["--list", "--json"])
        self.assertNotIn("--baseline", calls[0][1])
        self.assertEqual(calls[0][1][calls[0][1].index("--file") + 1], MODULE.WINDOW_VARIANCE_FILE)
        self.assertEqual(report["listed_total"], 1)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        policy = {**MODULE.WINDOW_VARIANCE_FIXED, "mutation_inventories": report["listed_inventory_bindings"]}
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.WINDOW_VARIANCE_SCOPE, policy,
                                                         report["listed_inventories"])
        changed = copy.deepcopy(report["listed_inventories"])
        changed["window_variance"]["synthetic-window-fault"]["replacement"] = "<="
        with self.assertRaisesRegex(MODULE.Incomplete, "differs from its reviewed"):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.WINDOW_VARIANCE_SCOPE, policy, changed)

    def equivalence_fixture(self, root):
        """Exercise actual shared custody with a synthetic review, never semantic E approval."""
        policy = self.fixture(root)
        proof = root / "tasks/window-variance-equivalences.md"; proof.parent.mkdir(exist_ok=True)
        proof.write_text("## WINDOW-SYNTHETIC-E\nUnexecuted admission control, not product equivalence.\n")
        entry = {"id": "WINDOW-SYNTHETIC-E", "candidate_sha256": policy["candidate_sha256"],
                 "source_sha256": hashlib.sha256((root / MODULE.WINDOW_VARIANCE_FILE).read_bytes()).hexdigest(),
                 "mutant": self.mutant(), "requirement": "CHECK-7/8/25", "rationale": "Synthetic source-custody control.",
                 "proof": {"path": "tasks/window-variance-equivalences.md",
                           "sha256": hashlib.sha256(proof.read_bytes()).hexdigest(), "section": "WINDOW-SYNTHETIC-E",
                           "reviewer": "independent synthetic control", "verdict": "independently-justified-equivalent"}}
        policy["mutation_equivalences"] = [entry]; self.write(root, policy)
        return policy, proof

    def test_exact_equivalence_requires_actual_source_candidate_proof_and_review(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, proof = self.equivalence_fixture(root)
            self.assertEqual(MODULE.read_window_variance_policy(root), healthy)
            for change in ("candidate", "source", "proof", "owner", "duplicate", "section", "reviewer", "descriptor"):
                bad = copy.deepcopy(healthy); case = bad["mutation_equivalences"][0]
                if change == "candidate": case["candidate_sha256"] = "0" * 64
                if change == "source": case["source_sha256"] = "0" * 64
                if change == "proof": case["proof"]["sha256"] = "0" * 64
                if change == "owner": case["mutant"]["file"] = "crates/time-core/src/regression.rs"
                if change == "duplicate": bad["mutation_equivalences"].append(copy.deepcopy(case))
                if change == "section": case["proof"]["section"] = "MISSING"
                if change == "reviewer": case["proof"]["reviewer"] = ""
                if change == "descriptor": case["mutant"].pop("span")
                self.write(root, bad)
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_window_variance_policy(root)
            self.write(root, healthy); proof.write_text("## WINDOW-SYNTHETIC-E\nChanged review.\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                MODULE.read_window_variance_policy(root)

    def test_all_lane_uses_one_session_coverage_before_mutation_and_preserves_budgets(self):
        for covered in (9, 8):
            with self.subTest(covered=covered), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); policy = self.fixture(root)
                policy["mutation_inventories"] = {"window_variance": {"count": 1, "sha256": "a" * 64}}
                self.write(root, policy)
                _, identity = MODULE.PRODUCT.admit_product(root)
                work = root / "owned-work"; work.mkdir()
                events = []
                def run(label, command, timeout, **options):
                    self.assertEqual(label, "coverage")
                    self.assertEqual(timeout, 300)
                    self.assertEqual(command[command.index("--test") + 1], "window_variance")
                    self.assertNotIn("--fail-under-file-lines", command)
                    events.append("coverage")
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [{
                        "filename": "/work/source/" + MODULE.WINDOW_VARIANCE_FILE,
                        "summary": {"lines": {"count": 10, "covered": covered}}}]}]}))
                    return literal_execution(doctests=False)
                def session_factory(actual_root, args, report, scope, admitted, extra):
                    self.assertEqual((actual_root, args.lane, scope), (root, "all", MODULE.WINDOW_VARIANCE_SCOPE))
                    events.append("session")
                    return SimpleNamespace(product_identity=identity, source=root, work=work,
                                           tools=root / "owned-tools", run=run,
                                           finish=lambda: events.append("finish"))
                def mutation(scope, subjects, admitted, actual_work, tools, execute, report, lane):
                    self.assertEqual((lane, admitted["mutation_wall_seconds"]), ("mutation", 1200))
                    self.assertEqual(subjects, {"window_variance": MODULE.WINDOW_VARIANCE_FILE})
                    self.assertEqual(report["coverage"]["window_variance"]["line_percent"], 90)
                    events.append("mutation")
                args = SimpleNamespace(subject="window_variance", lane="all", tool_admission_profile="full",
                                       output_dir=root / "output", vendor_dir=root / "vendor")
                report = {}
                with patch.object(MODULE, "ROOT", root), patch.object(MODULE.COMMON, "Session", side_effect=session_factory) as sessions, \
                     patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=mutation) as mutations:
                    if covered == 9:
                        MODULE.execute_window_variance(args, report)
                        self.assertTrue(report["qualification_passed"])
                        self.assertEqual(events, ["session", "coverage", "mutation", "finish"])
                        mutations.assert_called_once()
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, "below 90"):
                            MODULE.execute_window_variance(args, report)
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
                    MODULE.inventory_document(path)
            path.write_text('{"x":0.5}')
            self.assertEqual(MODULE.inventory_document(path), {"x": 0.5})
            linked = root / "linked.json"; linked.symlink_to(path)
            with self.assertRaises(MODULE.Incomplete): MODULE.inventory_document(linked)

    def test_inline_test_padding_and_missing_security_owner_reject_even_with_refreshed_pins(self):
        for change in ("inline", "owner"):
            with self.subTest(change=change), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); policy = self.fixture(root)
                if change == "inline":
                    source = root / MODULE.WINDOW_VARIANCE_FILE
                    source.write_text(source.read_text() + "\n#[test]\nfn synthetic_padding() {}\n")
                else:
                    manifest = root / "policy/verification-manifest.toml"
                    lines = manifest.read_text().splitlines(keepends=True)
                    index = next(index for index, line in enumerate(lines)
                                 if MODULE.WINDOW_VARIANCE_FILE in line and "package" in line)
                    lines[index] = lines[index].replace('package = "time-core"', 'package = "time-domain"')
                    manifest.write_text("".join(lines))
                _, identity = MODULE.PRODUCT.admit_product(root)
                policy["candidate_sha256"] = digest(identity)
                policy["inputs_sha256"] = {name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                                          for name in MODULE.WINDOW_VARIANCE_BOUND_INPUTS}
                self.write(root, policy)
                with self.assertRaisesRegex(MODULE.Incomplete, "inline test|critical/security"):
                    MODULE.read_window_variance_policy(root)


if __name__ == "__main__":
    unittest.main()
