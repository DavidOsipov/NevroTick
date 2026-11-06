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
    text = """running 38 tests
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
test processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice ... ok
test processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection ... ok
test processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates ... ok
test processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once ... ok
test measurement::tests::exact_dyadic_results_ignore_the_wire_token ... ok
test measurement::tests::peer_shift_changes_sign_without_changing_rtt ... ok
test measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved ... ok
test measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values ... ok
test measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals ... ok
test measurement::tests::absent_profile_is_a_distinct_transport_only_reason ... ok
test measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor ... ok
test measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors ... ok
test measurement::tests::reversed_brackets_and_processing_are_not_ages ... ok
test measurement::tests::out_of_scope_local_coordinate_is_not_rebased ... ok
test measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints ... ok
test measurement::tests::era_zero_and_multiple_candidates_remain_distinct ... ok
test measurement::tests::inert_header_does_not_bypass_the_time_reply_guard ... ok
test measurement::tests::profile_bounds_and_maximum_allowance_are_checked ... ok
test measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic ... ok
test measurement::tests::refusal_codes_and_display_remain_literal_and_redacted ... ok
test measurement::tests::software_window_retains_each_local_and_peer_extremum ... ok
test measurement::tests::coherence_helper_rejects_each_reversed_bracket ... ok
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
running 4 tests
test argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure ... ok
test argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows ... ok
test argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles ... ok
test argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
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
running 12 tests
test crates/nevrotick-engine/src/lib.rs - Config::new (line 168) ... ok
test crates/nevrotick-engine/src/lib.rs - ControlSchedule::new (line 1238) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 91) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 101) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 114) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 123) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 128) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 213) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 219) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 226) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 234) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 239) - compile fail ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"""
    return text


def literal_inventories():
    """Synthetic listing controls are not actual product mutant observations."""
    result = {}
    for subject, filename in [
        ("library", "crates/nevrotick-engine/src/lib.rs"),
        ("cli", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs"),
        ("measurement", "crates/nevrotick-engine/src/measurement.rs"),
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

    def test_three_source_owners_have_exact_selections_and_command_targets(self):
        expected = {"library": "crates/nevrotick-engine/src/lib.rs",
                    "cli": "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
                    "measurement": "crates/nevrotick-engine/src/measurement.rs"}
        self.assertEqual(MODULE.SCOPE.select("all"), expected)
        for subject, filename in expected.items():
            self.assertEqual(MODULE.SCOPE.select(subject), {subject: filename})
            command = MODULE.mutation_prefix(Path("/owned/tools"), subject, MODULE.FIXED_POLICY)
            self.assertEqual(command[command.index("--file") + 1], filename)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.SCOPE.select("foreign")
        self.assertEqual(MODULE.CONTRACT_SCOPE.select("all"), {"library": expected["library"]})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.CONTRACT_SCOPE.select("measurement")

    def fixture(self, root):
        for name in (MODULE.POLICY, *MODULE.BOUND_INPUTS):
            destination = root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        identity = {"owned-fixture-input.rs": "a" * 64}
        policy = json.loads((root / MODULE.POLICY).read_text())
        policy["candidate_sha256"] = independent_digest(identity)
        policy["mutation_inventories"] = {}  # This literal fixture models the held state.
        policy["mutation_equivalences"] = []
        policy["inputs_sha256"] = {
            name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in MODULE.BOUND_INPUTS}
        self.write_policy(root, policy)
        return policy, identity

    def write_policy(self, root, policy):
        (root / MODULE.POLICY).write_text(json.dumps(policy))

    def admit_fixture(self, root, identity):
        with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
            return MODULE.read_policy(root)

    def test_new_analysis_files_are_outside_the_current_product_census(self):
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        self.assertEqual(len(identity), 168)
        self.assertIn("crates/nevrotick-engine/src/measurement.rs", identity)
        for name in ["scripts/verify-engine-analysis.py", "scripts/tests/test_verify_engine_analysis.py",
                     "policy/engine-analysis.json"]:
            self.assertNotIn(name, identity)

    def test_healthy_policy_is_full_profile_and_campaign_initially_held(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); _, identity = self.fixture(root)
            policy = self.admit_fixture(root, identity)
        self.assertEqual(policy["critical_files"], [
            "crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
            "crates/nevrotick-engine/src/measurement.rs"])
        self.assertEqual(policy["mutation_inventories"], {})
        self.assertEqual(policy["mutation_equivalences"], [])
        self.assertEqual(policy["tool_admission_profile"], "full")
        self.assertEqual(policy["mutation_jobs"], 10)

    def equivalence_fixture(self, root):
        """Use real product/source admission with synthetic, explicitly scoped E controls."""
        _, current = MODULE.PRODUCT.admit_product(ROOT)
        for name in sorted(set(current) | set(MODULE.BOUND_INPUTS) | {MODULE.POLICY}):
            target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        _, identity = MODULE.PRODUCT.admit_product(root)
        policy = json.loads((root / MODULE.POLICY).read_text())
        policy["candidate_sha256"] = independent_digest(identity)
        policy["inputs_sha256"] = {
            name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in MODULE.BOUND_INPUTS}
        policy["mutation_inventories"] = {}
        filename = "crates/nevrotick-engine/src/measurement.rs"
        lines = (root / filename).read_text().splitlines()
        start = next(index for index, line in enumerate(lines, 1) if line.startswith("fn coherence_residual("))
        end = next(index for index, line in enumerate(lines, 1) if index > start and line == "}")
        cases = [("ENGINE-COHERENCE-LOWER-EQUALITY", "<", "<=", "wall_delta < lower"),
                 ("ENGINE-COHERENCE-UPPER-EQUALITY", ">", ">=", "wall_delta > upper")]
        proof = root / "tasks/engine-analysis-equivalences.md"; proof.parent.mkdir(exist_ok=True)
        proof.write_text("# Synthetic admission controls, not product qualification\n" + "".join(
            "\n## " + identifier + "\nSynthetic review-section fixture.\n" for identifier, *_ in cases))
        entries = []
        for identifier, original, replacement, expression in cases:
            line = next(index for index, text in enumerate(lines, 1) if expression in text)
            column = lines[line - 1].index(original) + 1
            mutant = {
                "name": f"{filename}:{line}:{column}: replace {original} with {replacement} in coherence_residual",
                "package": "nevrotick-engine", "file": filename,
                "function": {"function_name": "coherence_residual", "return_type": "-> Result<u64, MeasurementRefusal>",
                             "span": {"start": {"line": start, "column": 1}, "end": {"line": end, "column": 2}}},
                "span": {"start": {"line": line, "column": column}, "end": {"line": line, "column": column + 1}},
                "replacement": replacement, "genre": "BinaryOperator"}
            entries.append({"id": identifier, "candidate_sha256": policy["candidate_sha256"],
                            "source_sha256": identity[filename], "mutant": mutant,
                            "requirement": "CHECK-7/8/25", "rationale": "Synthetic exact-custody admission control.",
                            "proof": {"path": "tasks/engine-analysis-equivalences.md",
                                      "sha256": hashlib.sha256(proof.read_bytes()).hexdigest(),
                                      "section": identifier, "reviewer": "independent synthetic control",
                                      "verdict": "independently-justified-equivalent"}})
        policy["mutation_equivalences"] = entries
        self.write_policy(root, policy)
        return policy, proof

    def test_exact_engine_equivalences_bind_real_product_source_and_shared_review_custody(self):
        """No product or equivalence-admission mocks stand in for the healthy control."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, _ = self.equivalence_fixture(root)
            self.assertEqual(len(MODULE.PRODUCT.admit_product(root)[1]), 168)
            self.assertEqual(MODULE.read_policy(root), policy)
            self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SCOPE, policy),
                             policy["mutation_equivalences"])
            policy["mutation_equivalences"] = []
            self.write_policy(root, policy)
            self.assertEqual(MODULE.read_policy(root), policy)

    def test_engine_equivalences_reject_partial_foreign_duplicate_stale_and_malformed_reviews(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, _ = self.equivalence_fixture(root)
            for change in ("missing", "foreign", "duplicate", "owner", "function", "path", "operator", "genre",
                           "candidate", "source", "proof", "section", "reviewer", "verdict", "descriptor",
                           "unknown", "id_type", "list_type", "missing_key"):
                bad = copy.deepcopy(healthy); entries = bad["mutation_equivalences"]; case = entries[0]
                if change == "missing": entries.pop()
                if change == "foreign": case["id"] = "FOREIGN-EQUALITY"
                if change == "duplicate": entries[1] = copy.deepcopy(case)
                if change == "owner": case["mutant"]["file"] = "crates/nevrotick-engine/src/lib.rs"
                if change == "function": case["mutant"]["function"]["function_name"] = "measure"
                if change == "path": case["proof"]["path"] = "tasks/foreign.md"
                if change == "operator": case["mutant"]["replacement"] = ">="
                if change == "genre": case["mutant"]["genre"] = "FnValue"
                if change == "candidate": case["candidate_sha256"] = "0" * 64
                if change == "source": case["source_sha256"] = "0" * 64
                if change == "proof": case["proof"]["sha256"] = "0" * 64
                if change == "section": case["proof"]["section"] = "UNREVIEWED"
                if change == "reviewer": case["proof"]["reviewer"] = ""
                if change == "verdict": case["proof"]["verdict"] = "proposed"
                if change == "descriptor": case["mutant"].pop("span")
                if change == "unknown": case["skip_function"] = "coherence_residual"
                if change == "id_type": case["id"] = []
                if change == "list_type": bad["mutation_equivalences"] = {}
                if change == "missing_key": bad.pop("mutation_equivalences")
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root)
            self.write_policy(root, healthy)
            self.assertEqual(MODULE.read_policy(root), healthy)

    def test_engine_equivalence_review_bytes_and_missing_sections_cannot_drift(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, proof = self.equivalence_fixture(root)
            healthy = copy.deepcopy(policy)
            original = proof.read_text()
            proof.write_text(original + "Changed after independent review.\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                MODULE.read_policy(root)
            proof.write_text("# No reviewed sections\n")
            for entry in policy["mutation_equivalences"]:
                entry["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
            self.write_policy(root, policy)
            with self.assertRaisesRegex(MODULE.Incomplete, "exact section"):
                MODULE.read_policy(root)
            proof.unlink()
            with self.assertRaises(FileNotFoundError):
                MODULE.read_policy(root)
            proof.write_text(original)
            self.write_policy(root, healthy)
            self.assertEqual(MODULE.read_policy(root), healthy)

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

    def test_critical_measurement_scope_rejects_missing_extra_and_redirected_selection(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            for change in ("missing", "extra", "redirected"):
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["critical_files"].remove("crates/nevrotick-engine/src/measurement.rs")
                elif change == "extra": bad["critical_files"].append("crates/nevrotick-engine/src/foreign.rs")
                else: bad["critical_files"][-1] = "crates/nevrotick-engine/src/foreign.rs"
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "scope"):
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

    def test_campaign_bindings_must_cover_three_nonempty_exact_inventories(self):
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        for bindings in [{}, {"library": {"count": 1, "sha256": "a" * 64}},
                         {name: {"count": count, "sha256": "a" * 64}
                          for name, count in [("library", 1), ("cli", 1), ("measurement", 0)]},
                         {name: {"count": True, "sha256": "a" * 64}
                          for name in ["library", "cli", "measurement"]},
                         {name: {"count": 1, "sha256": "not-a-hash"}
                          for name in ["library", "cli", "measurement"]}]:
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.inventory_bindings(policy, required=True)
        healthy = {name: {"count": 1, "sha256": "a" * 64} for name in ["library", "cli", "measurement"]}
        policy["mutation_inventories"] = healthy
        self.assertEqual(len(MODULE.inventory_bindings(policy, required=True)), 3)
        for change in ("missing", "extra", "redirected"):
            bad = copy.deepcopy(healthy)
            if change == "missing": bad.pop("measurement")
            elif change == "extra": bad["foreign"] = {"count": 1, "sha256": "a" * 64}
            else: bad["redirected"] = bad.pop("measurement")
            policy["mutation_inventories"] = bad
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.inventory_bindings(policy, required=True)

    def test_fresh_descriptor_metadata_must_match_review_while_rendered_diff_is_ignored(self):
        observed = literal_inventories()
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        policy["mutation_inventories"] = {
            name: {"count": 1, "sha256": independent_digest(items)} for name, items in observed.items()}
        self.assertEqual(set(MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, observed)),
                         {"library", "cli", "measurement"})
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
                         ["tests_executed"], 47)
        full = MODULE.validate_test_execution(literal_execution(doctests=True), doctests=True)
        self.assertEqual((full["tests_executed"], full["doctests"]), (59, 12))

    def test_constructor_and_guard_alternatives_cannot_be_omitted_from_baseline(self):
        """A healthy full transcript discriminates the earlier seven-case baseline."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 47)
        added = (
            "processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries",
            "processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays",
            "processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap",
            "processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections",
            "processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions",
            "processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor",
        )
        for name in added:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("38 passed;", "37 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_argument_budget_cases_cannot_be_omitted_from_baseline(self):
        """Checked resource owners require their actual native unit records."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 47)
        for name in (
            "argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure",
            "argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows",
            "argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles",
            "argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries",
        ):
            text = healthy.replace("test " + name + " ... ok\n", "").replace("4 passed;", "3 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_measurement_and_owner_cases_cannot_be_omitted_from_baseline(self):
        """A counted healthy transcript rejects every new semantic/ownership omission."""
        healthy = literal_execution(doctests=False)
        required = (
            "processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice",
            "processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection",
            "processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates",
            "processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once",
            "measurement::tests::exact_dyadic_results_ignore_the_wire_token",
            "measurement::tests::peer_shift_changes_sign_without_changing_rtt",
            "measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved",
            "measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values",
            "measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals",
            "measurement::tests::absent_profile_is_a_distinct_transport_only_reason",
            "measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor",
            "measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors",
            "measurement::tests::reversed_brackets_and_processing_are_not_ages",
            "measurement::tests::out_of_scope_local_coordinate_is_not_rebased",
            "measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints",
            "measurement::tests::era_zero_and_multiple_candidates_remain_distinct",
            "measurement::tests::inert_header_does_not_bypass_the_time_reply_guard",
            "measurement::tests::profile_bounds_and_maximum_allowance_are_checked",
            "measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic",
            "measurement::tests::refusal_codes_and_display_remain_literal_and_redacted",
            "measurement::tests::software_window_retains_each_local_and_peer_extremum",
            "measurement::tests::coherence_helper_rejects_each_reversed_bracket",
        )
        for name in required:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("38 passed;", "37 passed;", 1)
            with self.subTest(name=name), self.assertRaisesRegex(MODULE.Incomplete, "required actual"):
                MODULE.validate_test_execution(text, doctests=False)

    def test_measurement_doctests_require_success_and_all_private_construction_failures(self):
        healthy = literal_execution(doctests=True)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 12)
        docs = [line for line in healthy.splitlines() if line.startswith("test crates/")]
        for line in docs:
            text = healthy.replace(line + "\n", "").replace("12 passed;", "11 passed;", 1)
            with self.subTest(omitted=line), self.assertRaisesRegex(MODULE.Incomplete, "doctest"):
                MODULE.validate_test_execution(text, doctests=True)
        for text in (
            healthy.replace("measurement::ExperimentalProfile (line 101) - compile fail",
                            "measurement::ExperimentalProfile (line 101)"),
            healthy.replace("src/measurement.rs - measurement::ExperimentalMeasurement",
                            "src/lib.rs - measurement::ExperimentalMeasurement", 1),
            healthy.replace("measurement::ExperimentalProfile (line 128)", "unrelated::Example (line 128)"),
        ):
            with self.subTest(text=text[-500:]), self.assertRaisesRegex(MODULE.Incomplete, "doctest"):
                MODULE.validate_test_execution(text, doctests=True)
        with self.assertRaisesRegex(MODULE.Incomplete, "unexpectedly includes doctests"):
            MODULE.validate_test_execution(healthy, doctests=False)

    def test_library_native_result_alternatives_cannot_be_omitted(self):
        """Every newly required admission/completion owner has a native record."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 47)
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
            text = healthy.replace("test " + name + " ... ok\n", "").replace("38 passed;", "37 passed;", 1)
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
                     literal_execution(doctests=False).replace("38 passed;", "0 passed;", 1)]:
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
            self.assertEqual(MODULE.validate_baseline(work, "library")["tests_executed"], 59)
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
        self.assertEqual(len(calls), 3)
        self.assertTrue(all(command[-2:] == ["--list", "--json"] and "--baseline" not in command
                            for _, command, _ in calls))
        self.assertEqual(report["listed_total"], 3)
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

    def test_cli_selector_routes_measurement_without_changing_profile_or_lane(self):
        def selected(args, report):
            self.assertEqual((args.lane, args.subject, args.tool_admission_profile),
                             ("coverage", "measurement", "full"))
        with patch.object(MODULE, "execute", side_effect=selected), patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(MODULE.main(["--lane", "coverage", "--subject", "measurement",
                                          "--vendor-dir", "/admitted/vendor", "--output-dir", "/owned/output"]), 0)


class EngineExternalContractTests(unittest.TestCase):
    """Current inert external contracts never inherit the historical campaigns."""

    def fixture(self, root):
        for name in (MODULE.POLICY,"policy/verification-manifest.toml"):
            target=root/name;target.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(ROOT/name,target)
        parent=json.loads((root/MODULE.POLICY).read_text())
        parent["mutation_equivalences"]=[]
        (root/MODULE.POLICY).write_text(json.dumps(parent))
        return parent

    def test_configured_contract_cli_has_a_distinct_scoped_result(self):
        reports=[]
        def completed(args, report):
            self.assertEqual(args.tool_admission_profile,"stable-core")
            reports.append(report)
            report.update(contracts_qualification_passed=True,qualification_passed=False)
        with patch.object(MODULE,"execute",side_effect=completed),patch("sys.stdout",new_callable=io.StringIO):
            status=MODULE.main(["--lane","contracts","--tool-admission-profile","stable-core",
                                "--vendor-dir","/admitted/vendor","--output-dir","/new/owned/output"])
        self.assertEqual(status,0)
        self.assertEqual(reports[0]["status"],"passed")
        self.assertFalse(reports[0]["qualification_passed"])
        self.assertTrue(reports[0]["contracts_qualification_passed"])

    def test_current_contract_role_is_independent_of_stale_historical_candidate(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);parent=self.fixture(root)
            for name in MODULE.BOUND_INPUTS:
                target=root/name;target.parent.mkdir(parents=True,exist_ok=True)
                shutil.copyfile(ROOT/name,target)
            identity={"owned-fixture-input.rs":"a"*64}
            parent["candidate_sha256"]=independent_digest(identity)
            parent["mutation_inventories"]={}
            parent["mutation_equivalences"]=[]
            parent["inputs_sha256"]={name:hashlib.sha256((root/name).read_bytes()).hexdigest()
                                      for name in MODULE.BOUND_INPUTS}
            path=root/MODULE.POLICY
            path.write_text(json.dumps(parent))
            with patch.object(MODULE.PRODUCT,"admit_product",return_value=({},identity)):
                self.assertEqual(MODULE.read_policy(root),parent)
                policy=MODULE.read_contract_policy(root)
                stale=copy.deepcopy(parent);stale["candidate_sha256"]="0"*64
                path.write_text(json.dumps(stale))
                original=path.read_bytes()
                self.assertEqual(MODULE.read_contract_policy(root),policy)
                with self.assertRaisesRegex(MODULE.Incomplete,"complete current product"):
                    MODULE.read_policy(root)
                self.assertEqual(path.read_bytes(),original)
                path.write_text(json.dumps(parent))
                self.assertEqual(MODULE.read_policy(root),parent)
                self.assertEqual(MODULE.read_contract_policy(root),policy)
        self.assertEqual(policy["source_file_count"],168)
        self.assertEqual(policy["critical_files"],["crates/nevrotick-engine/src/lib.rs"])
        self.assertEqual(policy["jobs"],6)
        self.assertEqual(policy["test_threads"],1)
        self.assertEqual(policy["tool_admission_profile"],"stable-core")
        self.assertEqual(MODULE.CONTRACT_SCOPE.subjects,
                         (("library","crates/nevrotick-engine/src/lib.rs"),))
        self.assertEqual(MODULE.COMMON.admission_profile(
            SimpleNamespace(lane="contracts",tool_admission_profile="stable-core"),
            MODULE.CONTRACT_SCOPE,policy),"stable-core")

    def test_child_role_rejects_poison_fields_without_rebinding_parent(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);parent=self.fixture(root)
            historical={key:copy.deepcopy(value) for key,value in parent.items() if key!="external_contracts"}
            healthy=MODULE.read_contract_policy(root)
            for key,value in (("schema_version",2),("admission_status","passed"),
                              ("source_file_count",True),("source_file_count",167),
                              ("source_file_count",169),("jobs",12),("test_threads",12),
                              ("tool_admission_profile","full"),("critical_files",list(MODULE.SUBJECTS.values())),
                              ("mutation_equivalences",[{}]),("mutation_error_values",{"library":["panic!()"]}),
                              ("contracts_wall_seconds",0),("unexpected",1)):
                bad=copy.deepcopy(parent);bad["external_contracts"][key]=value
                (root/MODULE.POLICY).write_text(json.dumps(bad))
                with self.subTest(key=key),self.assertRaises(MODULE.Incomplete):
                    MODULE.read_contract_policy(root)
            (root/MODULE.POLICY).write_text(json.dumps(parent))
            self.assertEqual(MODULE.read_contract_policy(root),healthy)
            self.assertEqual({key:value for key,value in MODULE.read_json(root/MODULE.POLICY).items()
                              if key!="external_contracts"},historical)

    def test_privacy_trait_and_raw_api_fixtures_have_owned_causal_lines(self):
        cases=MODULE.contract_examples()
        self.assertEqual(len(cases),13)
        self.assertIsNone(cases["legal"]["code"])
        expected={
            "config_construction":"E0451","schedule_construction":"E0451",
            "config_peers_mutation":"E0616","config_timeout_mutation":"E0616",
            "schedule_stop_mutation":"E0616","schedule_invalidate_mutation":"E0616",
            "config_default":"E0277","schedule_default":"E0277",
            "config_from":"E0277","schedule_from":"E0277",
            "raw_config_prepare":"E0308","raw_schedule_execute":"E0308"}
        for name,code in expected.items():
            case=cases[name]
            with self.subTest(case=name):
                self.assertEqual(case["code"],code)
                self.assertEqual(case["line"],4)
                self.assertEqual(len(case["source"].splitlines()),6)
                self.assertNotIn("\\\\n",case["source"])
                self.assertTrue(case["tokens"])
        legal=cases["legal"]["source"]
        self.assertIn("Config::new",legal)
        self.assertIn("ControlSchedule::new",legal)
        self.assertIn("PilotError::DuplicatePeer",legal)
        self.assertIn("PilotError::InvalidTimeout",legal)
        self.assertNotIn("prepare(",legal)
        self.assertNotIn("execute(",legal)
        self.assertNotIn("Runtime",legal)
        self.assertIn("fn rejected(pilot: PreparedPilot)",cases["raw_schedule_execute"]["source"])
        self.assertTrue({"peers","timeout","experimental_profile"} <= set(cases["config_construction"]["tokens"]))
        self.assertEqual(cases["config_construction"]["source"].splitlines()[3],
                         "let _ = Config { peers: [peers[0], peers[0]], timeout: Duration::ZERO, "
                         "experimental_profile: None };")
        self.assertTrue({"stop_after","invalidate_after"} <= set(cases["schedule_construction"]["tokens"]))
        self.assertTrue(all("let _ = &value;" in cases[name]["source"].splitlines()[3]
                            for name in expected if name.endswith("_mutation")))

    def test_contract_library_copy_and_command_dispatch_preserve_real_artifact(self):
        cases=MODULE.contract_examples()
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary);deps=work/"target/debug/deps";deps.mkdir(parents=True)
            artifact=work/"target/debug/libnevrotick_engine.rlib";artifact.write_bytes(b"actual emitted library")
            (deps/"libtokio.rlib").write_bytes(b"actual dependency")
            calls=[]
            def run(label,command,timeout,**kwargs):
                calls.append((label,command,kwargs.get("specimen")))
                if label=="contract-library":
                    self.assertEqual(command[command.index("--jobs")+1],"6")
                if label=="external-legal-execution":
                    return cases["legal"]["success_marker"]
                return ""
            session=SimpleNamespace(work=work,stable=work/"stable",run=run,protect=lambda _:None)
            report={}
            with patch.object(MODULE.COMMON,"library_artifact",
                              return_value="/work/target/debug/libnevrotick_engine.rlib"):
                MODULE.contracts(session,MODULE.read_contract_policy(ROOT),report)
            self.assertEqual(report["contracts"]["legal_consumers_compiled_and_executed"],1)
            self.assertEqual(len([case for _,_,case in calls if case is not None]),12)
            copied=work/"contract-deps/libnevrotick_engine.rlib"
            self.assertEqual(copied.read_bytes(),artifact.read_bytes())
            self.assertNotEqual(copied.stat().st_ino,artifact.stat().st_ino)
            self.assertEqual(copied.stat().st_mode&0o777,0o400)
            self.assertTrue(all("--error-format=json" in command for label,command,_ in calls
                                if label.startswith("external-consumer-")))
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary);(work/"target/debug/deps").mkdir(parents=True)
            (work/"target/debug/libnevrotick_engine.rlib").write_bytes(b"emitted library")
            def missing_marker(label,command,timeout,**kwargs):
                return "not the expected actual consumer marker" if label=="external-legal-execution" else ""
            session=SimpleNamespace(work=work,stable=work/"stable",run=missing_marker,protect=lambda _:None)
            report={}
            with patch.object(MODULE.COMMON,"library_artifact",
                              return_value="/work/target/debug/libnevrotick_engine.rlib"):
                with self.assertRaisesRegex(MODULE.Incomplete,"Legal engine consumer"):
                    MODULE.contracts(session,MODULE.read_contract_policy(ROOT),report)
            self.assertNotIn("contracts",report)

    def test_contracts_reject_wrong_profile_or_subject_before_session(self):
        for lane,subject,profile in (("contracts","all","full"),("contracts","cli","stable-core"),
                                     ("contracts","measurement","stable-core")):
            with self.subTest(subject=subject,profile=profile),patch.object(MODULE.COMMON,"Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute(SimpleNamespace(lane=lane,subject=subject,tool_admission_profile=profile),{})
                session.assert_not_called()

    def test_failed_current_custody_never_publishes_contract_qualification(self):
        with tempfile.TemporaryDirectory() as temporary:
            source=Path(temporary);self.fixture(source)
            fake=SimpleNamespace(source=source,product_identity={"fixture.rs":"a"*64},
                                 finish=lambda:(_ for _ in ()).throw(MODULE.Incomplete("final custody failure")))
            report={}
            with patch.object(MODULE.COMMON,"Session",return_value=fake), \
                 patch.object(MODULE,"contracts"):
                with self.assertRaisesRegex(MODULE.Incomplete,"final custody failure"):
                    MODULE.execute(SimpleNamespace(lane="contracts",subject="all",
                                   tool_admission_profile="stable-core"),report)
            self.assertFalse(report["qualification_passed"])
            self.assertFalse(report["contracts_qualification_passed"])


if __name__ == "__main__":
    unittest.main()
