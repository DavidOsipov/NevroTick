"""Independent engine analysis admission controls; no compiler/analyzer execution."""
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
SPEC = importlib.util.spec_from_file_location("engine_analysis_test", ROOT / "scripts/verify-engine-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def independent_digest(value):
    """Bind independently declared fixture values using the documented JSON encoding."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def literal_execution(*, doctests):
    """Literal native libtest records, independent of the runner's required-name set."""
    text = """running 16 tests
test processing_guard_tests::processing_uses_fresh_deadline_and_current_control ... ok
test processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct ... ok
test processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries ... ok
test processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays ... ok
test processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap ... ok
test processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections ... ok
test processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions ... ok
test processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor ... ok
test processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes ... ok
test processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port ... ok
test processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct ... ok
test processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history ... ok
test processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four ... ok
test processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize ... ok
test processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor ... ok
test processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error ... ok
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
running 3 tests
test argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure ... ok
test argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows ... ok
test argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 5 tests
test prepare_outside_runtime_owns_and_releases_standard_sockets ... ok
test prepare_rejects_entered_runtime_before_native_startup ... ok
test execute_rejects_entered_runtime_without_nested_runtime_creation ... ok
test cli_worker_binary_preserves_literal_transport_contracts ... ok
test cli_worker_binary_rejects_arguments_and_observes_sink_failures ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s
"""
    if doctests:
        text += """   Doc-tests nevrotick_engine
running 2 tests
test crates/nevrotick-engine/src/lib.rs - Config::new (line 168) ... ok
test crates/nevrotick-engine/src/lib.rs - ControlSchedule::new (line 1238) ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"""
    return text


def literal_inventories():
    """Synthetic listing controls are not actual product mutant observations."""
    result = {}
    for subject, filename in [
        ("library", "crates/nevrotick-engine/src/lib.rs"),
        ("cli", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs"),
    ]:
        name = subject + " synthetic fault"
        result[subject] = {name: {
            "name": name, "package": "nevrotick-engine", "file": filename,
            "function": {"function_name": "fixture_only", "return_type": "bool",
                         "span": {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 1}}},
            "span": {"start": {"line": 1, "column": 2}, "end": {"line": 1, "column": 4}},
            "replacement": "!=", "genre": "BinaryOperator",
        }}
    return result


class EngineAnalysisTests(unittest.TestCase):
    """Healthy neighbors discriminate scope, candidate, listing and baseline gates."""

    def fixture(self, root):
        for name in (MODULE.POLICY, *MODULE.BOUND_INPUTS):
            destination = root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        identity = {"owned-fixture-input.rs": "a" * 64}
        policy = json.loads((root / MODULE.POLICY).read_text())
        policy["candidate_sha256"] = independent_digest(identity)
        policy["mutation_inventories"] = {}  # This literal fixture models the held state.
        policy["inputs_sha256"] = {
            name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in MODULE.BOUND_INPUTS}
        self.write_policy(root, policy)
        return policy, identity

    def write_policy(self, root, policy):
        (root / MODULE.POLICY).write_text(json.dumps(policy))

    def admit_fixture(self, root, identity):
        with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
            return MODULE.read_policy(root)

    def test_new_analysis_files_are_outside_the_frozen_product_census(self):
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        self.assertEqual(len(identity), 119)
        for name in ["scripts/verify-engine-analysis.py", "scripts/tests/test_verify_engine_analysis.py",
                     "policy/engine-analysis.json"]:
            self.assertNotIn(name, identity)

    def test_healthy_policy_is_full_profile_and_campaign_initially_held(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); _, identity = self.fixture(root)
            policy = self.admit_fixture(root, identity)
        self.assertEqual(policy["critical_files"], [
            "crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs"])
        self.assertEqual(policy["mutation_inventories"], {})
        self.assertEqual(policy["mutation_equivalences"], [])
        self.assertEqual(policy["tool_admission_profile"], "full")
        self.assertEqual(policy["mutation_jobs"], 10)

    def test_wrong_scope_threshold_budget_type_profile_and_equivalence_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            changes = [("package", "time-core"), ("critical_files", healthy["critical_files"][:1]),
                       ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                       ("maximum_mutants", 1), ("jobs", True), ("coverage_test_threads", 12),
                       ("tool_admission_profile", "stable-core"), ("mutation_jobs", 1),
                       ("mutation_equivalences", [{}]), ("unknown", True)]
            for key, value in changes:
                bad = copy.deepcopy(healthy); bad[key] = value; self.write_policy(root, bad)
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    self.admit_fixture(root, identity)

    def test_candidate_drift_and_refreshed_local_pins_do_not_evade_whole_candidate(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            with self.assertRaisesRegex(MODULE.Incomplete, "complete current product candidate"):
                self.admit_fixture(root, {**identity, "new-input.rs": "b" * 64})
            bad = copy.deepcopy(healthy); bad["candidate_sha256"] = "0" * 64; self.write_policy(root, bad)
            with self.assertRaisesRegex(MODULE.Incomplete, "complete current product candidate"):
                self.admit_fixture(root, identity)

    def test_protected_input_missing_extra_and_actual_content_drift_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            for change in ["missing", "extra", "content"]:
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["inputs_sha256"].pop("Cargo.lock")
                elif change == "extra": bad["inputs_sha256"]["not-admitted"] = "0" * 64
                else: (root / "Cargo.lock").write_text("changed fixture input")
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "bindings changed"):
                    self.admit_fixture(root, identity)

    def test_critical_owner_removal_rejects_even_with_refreshed_local_pin(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); policy, identity = self.fixture(root)
            path = root / "policy/verification-manifest.toml"
            path.write_text(path.read_text().replace('package = "nevrotick-engine"', 'package = "wrong"'))
            policy["inputs_sha256"]["policy/verification-manifest.toml"] = hashlib.sha256(path.read_bytes()).hexdigest()
            self.write_policy(root, policy)
            with self.assertRaisesRegex(MODULE.Incomplete, "matching critical/security owner"):
                self.admit_fixture(root, identity)

    def test_json_rejects_duplicates_nonfinite_overflow_link_and_size(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); path = root / "input.json"
            for text in ['{"x":1,"x":2}', '{"x":{"x":1,"x":2}}', '{"x":NaN}', '{"x":1e999}']:
                path.write_text(text)
                with self.subTest(text=text), self.assertRaises(MODULE.Incomplete): MODULE.read_json(path)
            path.write_text('{"duration":0.25}')
            self.assertEqual(MODULE.read_json(path), {"duration": 0.25})
            with self.assertRaises(MODULE.Incomplete): MODULE.read_json(path, maximum=2)
            linked = root / "linked.json"; linked.symlink_to(path)
            with self.assertRaises(MODULE.Incomplete): MODULE.read_json(linked)

    def test_campaign_bindings_must_cover_both_nonempty_exact_inventories(self):
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        for bindings in [{}, {"library": {"count": 1, "sha256": "a" * 64}},
                         {name: {"count": count, "sha256": "a" * 64} for name, count in [("library", 0), ("cli", 1)]},
                         {name: {"count": True, "sha256": "a" * 64} for name in ["library", "cli"]},
                         {name: {"count": 1, "sha256": "not-a-hash"} for name in ["library", "cli"]}]:
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64} for name in ["library", "cli"]}
        self.assertEqual(len(MODULE.inventory_bindings(policy, required=True)), 2)

    def test_fresh_descriptor_metadata_must_match_review_while_rendered_diff_is_ignored(self):
        observed = literal_inventories()
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        policy["mutation_inventories"] = {
            name: {"count": 1, "sha256": independent_digest(items)} for name, items in observed.items()}
        self.assertEqual(set(MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, observed)),
                         {"library", "cli"})
        changed = copy.deepcopy(observed); changed["library"]["library synthetic fault"]["diff"] = "rendering only"
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, changed)
        changed["library"]["library synthetic fault"]["function"]["return_type"] = "u8"
        with self.assertRaisesRegex(MODULE.Incomplete, "differs from its reviewed"):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, changed)

    def test_empty_and_foreign_actual_listing_are_rejected(self):
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_inventory(MODULE.SCOPE, [], 4096, MODULE.SUBJECTS["library"])
        item = next(iter(literal_inventories()["cli"].values()))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_inventory(MODULE.SCOPE, [item], 4096, MODULE.SUBJECTS["library"])

    def test_baseline_has_healthy_coverage_and_full_cargo_neighbors(self):
        self.assertEqual(MODULE.validate_test_execution(literal_execution(doctests=False), doctests=False)
                         ["tests_executed"], 24)
        full = MODULE.validate_test_execution(literal_execution(doctests=True), doctests=True)
        self.assertEqual((full["tests_executed"], full["doctests"]), (26, 2))

    def test_constructor_and_guard_alternatives_cannot_be_omitted_from_baseline(self):
        """A healthy full transcript discriminates the earlier seven-case baseline."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 24)
        added = (
            "processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries",
            "processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays",
            "processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap",
            "processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections",
            "processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions",
            "processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor",
        )
        for name in added:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("16 passed;", "15 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_argument_budget_cases_cannot_be_omitted_from_baseline(self):
        """Checked resource owners require their actual native unit records."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 24)
        for name in (
            "argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure",
            "argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows",
            "argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles",
        ):
            text = healthy.replace("test " + name + " ... ok\n", "").replace("3 passed;", "2 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_library_native_result_alternatives_cannot_be_omitted(self):
        """Every newly required admission/completion owner has a native record."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 24)
        for name in (
            "processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes",
            "processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port",
            "processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct",
            "processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history",
            "processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four",
            "processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize",
            "processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor",
            "processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error",
        ):
            text = healthy.replace("test " + name + " ... ok\n", "").replace("16 passed;", "15 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_old_runtime_only_baseline_and_printed_names_cannot_replace_execution(self):
        old = """running 3 tests
test prepare_outside_runtime_owns_and_releases_standard_sockets ... ok
test prepare_rejects_entered_runtime_before_native_startup ... ok
test execute_rejects_entered_runtime_without_nested_runtime_creation ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"""
        for text in [old, old + "\n".join(MODULE.REQUIRED_TESTS),
                     literal_execution(doctests=False).replace("16 passed;", "0 passed;", 1)]:
            with self.subTest(text=text[:40]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_failed_skipped_filtered_or_missing_doctest_baselines_reject(self):
        healthy = literal_execution(doctests=True)
        for text in [healthy.replace(" ... ok", " ... ignored", 1),
                     healthy.replace("0 failed;", "1 failed;", 1),
                     healthy.replace("0 filtered out;", "1 filtered out;", 1),
                     literal_execution(doctests=False)]:
            with self.subTest(text=text[:40]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=True)

    def test_actual_baseline_log_and_finite_native_timings_are_read(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); directory = work / "mutation/library/mutants.out"
            (directory / "log").mkdir(parents=True)
            (directory / "log/baseline.log").write_text(literal_execution(doctests=True))
            path = directory / "outcomes.json"
            path.write_text(json.dumps({"outcomes": [{"scenario": "Baseline", "summary": "Success",
                                                    "duration": 0.25, "log_path": "log/baseline.log"}]}))
            self.assertEqual(MODULE.validate_baseline(work, "library")["tests_executed"], 26)
            bad = json.loads(path.read_text()); bad["outcomes"][0]["log_path"] = "../escape.log"
            path.write_text(json.dumps(bad))
            with self.assertRaisesRegex(MODULE.Incomplete, "escapes"):
                MODULE.validate_baseline(work, "library")

    def test_mutation_without_review_fails_before_any_session_or_command(self):
        policy = copy.deepcopy(MODULE.FIXED_POLICY); policy["mutation_inventories"] = {}
        args = SimpleNamespace(lane="mutation", subject="all")
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "requires reviewed nonempty"):
                MODULE.execute(args, {})
            session.assert_not_called()

    def test_direct_unknown_lane_cannot_manufacture_a_zero_command_pass(self):
        with patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "Unknown engine analysis lane"):
                MODULE.execute(SimpleNamespace(lane="not-a-lane", subject="all"), {})
            session.assert_not_called()

    def test_enumerate_only_launches_lists_and_never_claims_campaign_or_qualification(self):
        observed = literal_inventories(); calls = []; finished = []

        class FakeSession:
            def __init__(self, root, args, report, scope, chosen_policy, extra):
                self.tools = Path("/owned/tools")
                self.work = Path("/owned/work")
                self.selected_lane = args.lane
                self.source = source
                self.product_identity = identity

            def run(self, label, command, timeout):
                calls.append((label, command, timeout))
                subject = label.removeprefix("mutation-enumeration:")
                return json.dumps(list(observed[subject].values()))

            def finish(self):
                finished.append(True)

        args = SimpleNamespace(lane="enumerate", subject="all")
        report = {}
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp); policy, identity = self.fixture(source)
            with patch.object(MODULE, "read_policy", return_value=policy), \
                    patch.object(MODULE.COMMON, "Session", FakeSession), \
                    patch.object(MODULE.COMMON, "run_analysis_lanes") as campaign:
                MODULE.execute(args, report)
                campaign.assert_not_called()
        self.assertEqual(len(calls), 2)
        self.assertTrue(all(command[-2:] == ["--list", "--json"] and "--baseline" not in command
                            for _, command, _ in calls))
        self.assertEqual(report["listed_total"], 2)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        self.assertEqual(finished, [True])

    def test_failed_final_custody_cannot_leave_a_qualification_claim(self):
        report = {}
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp); policy, identity = self.fixture(source)
            fake = SimpleNamespace(source=source, product_identity=identity,
                                   work=Path("/owned/work"), tools=Path("/owned/tools"),
                                   run=lambda *args, **kwargs: "", finish=lambda: (_ for _ in ()).throw(
                                       MODULE.Incomplete("custody changed")))
            with patch.object(MODULE, "read_policy", return_value=policy), \
                    patch.object(MODULE.COMMON, "Session", return_value=fake), \
                    patch.object(MODULE.COMMON, "run_analysis_lanes"):
                with self.assertRaisesRegex(MODULE.Incomplete, "custody changed"):
                    MODULE.execute(SimpleNamespace(lane="coverage", subject="all"), report)
        self.assertFalse(report["qualification_passed"])

    def snapshot_control(self, change):
        """Exercise actual execute with a separately captured synthetic Session."""
        commands = []; finished = []; report = {}; failure = None
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp); policy, identity = self.fixture(source)

            class FakeSession:
                def __init__(self, root, args, received_report, scope, chosen_policy, extra):
                    self.source = source
                    self.work = Path("/owned/work")
                    self.tools = Path("/owned/tools")
                    self.product_identity = dict(identity)
                    if change == "candidate":
                        self.product_identity["new-session-input.rs"] = "b" * 64
                    elif change == "pin":
                        (source / "Cargo.lock").write_text("different copied lock bytes")
                    elif change == "policy":
                        copied = copy.deepcopy(policy); copied["scope"] = "different copied policy"
                        (source / MODULE.POLICY).write_text(json.dumps(copied))

                def run(self, label, command, timeout, **options):
                    commands.append((label, command))
                    return literal_execution(doctests=False)

                def finish(self):
                    finished.append(True)

            def selected_lane(scope, subjects, chosen_policy, work, tools, run, received_report, lane):
                run("coverage", ["fixture-only-coverage-command"], 1)

            with patch.object(MODULE, "read_policy", return_value=policy), \
                    patch.object(MODULE.COMMON, "Session", FakeSession), \
                    patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=selected_lane):
                try:
                    MODULE.execute(SimpleNamespace(lane="coverage", subject="all"), report)
                except MODULE.Incomplete as error:
                    failure = error
        return report, failure, commands, finished

    def test_execute_rejects_session_candidate_change_before_any_command(self):
        report, failure, commands, finished = self.snapshot_control("candidate")
        self.assertEqual(finished, [True])
        self.assertEqual(commands, [])
        self.assertIsInstance(failure, MODULE.Incomplete)
        self.assertFalse(report["qualification_passed"])

    def test_execute_rejects_copied_input_pin_drift_before_any_command(self):
        report, failure, commands, finished = self.snapshot_control("pin")
        self.assertEqual(finished, [True])
        self.assertEqual(commands, [])
        self.assertIsInstance(failure, MODULE.Incomplete)
        self.assertFalse(report["qualification_passed"])

    def test_execute_rejects_copied_policy_drift_before_any_command(self):
        report, failure, commands, finished = self.snapshot_control("policy")
        self.assertEqual(finished, [True])
        self.assertEqual(commands, [])
        self.assertIsInstance(failure, MODULE.Incomplete)
        self.assertFalse(report["qualification_passed"])

    def test_execute_healthy_captured_session_is_admitted_with_final_custody(self):
        report, failure, commands, finished = self.snapshot_control("healthy")
        self.assertIsNone(failure)
        self.assertEqual(commands, [("coverage", ["fixture-only-coverage-command"])])
        self.assertEqual(finished, [True])
        self.assertTrue(report["qualification_passed"])

    def test_readiness_is_a_finite_unexecuted_single_worker_proposal(self):
        proposal = MODULE.readiness_proposal()
        self.assertEqual((proposal["kind"], proposal["executed"], proposal["outer_workers"]),
                         ("proposal", False, 1))
        self.assertEqual((proposal["build_seconds"], proposal["test_seconds"]), (90, 60))
        self.assertIn("not assessed", proposal["ten_worker_readiness"])

    def test_enumeration_cli_status_remains_distinct_from_passed_qualification(self):
        observed = []

        def complete_listing(args, report):
            observed.append(report)
            report.update(qualification_passed=False, campaign_admission=False)

        with patch.object(MODULE, "execute", side_effect=complete_listing), patch("sys.stdout", new_callable=io.StringIO):
            status = MODULE.main(["--lane", "enumerate", "--vendor-dir", "/admitted/vendor",
                                  "--output-dir", "/new/owned/output"])
        self.assertEqual(status, 0)
        self.assertEqual(observed[0]["status"], "enumerated")
        self.assertFalse(observed[0]["qualification_passed"])


if __name__ == "__main__":
    unittest.main()
