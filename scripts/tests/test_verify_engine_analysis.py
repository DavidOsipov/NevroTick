"""Independent engine analysis admission controls; no compiler/analyzer execution."""
from __future__ import annotations

import copy
from contextlib import contextmanager
import hashlib
import importlib.util
import io
import json
import os
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
    text = """running 59 tests
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
test measurement::tests::decimal_remainders_change_the_exact_nominal_phase ... ok
test measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values ... ok
test measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association ... ok
test measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result ... ok
test measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds ... ok
test measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection ... ok
test measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt ... ok
test measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint ... ok
test processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals ... ok
test measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks ... ok
test measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding ... ok
test measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign ... ok
test relative::tests::midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints ... ok
test relative::tests::midpoint_publication_enforces_raw_association_and_garde_rule ... ok
test relative::tests::duration_nanoseconds_admits_full_u64_and_rejects_next_tick ... ok
test relative::tests::reference_instants_check_order_before_duration_conversion ... ok
test relative::tests::relative_reference_differences_keep_direction_and_half_ticks ... ok
test relative::tests::relative_phase_raw_i128_endpoints_keep_u128_magnitude ... ok
test relative::tests::shared_origin_distinguishes_equal_values_from_run_identity ... ok
test relative::tests::relative_seconds_scale_requires_nonzero_with_healthy_neighbors ... ok
test relative::tests::relative_diagnostics_preserve_fixed_text_and_original_causes ... ok
test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
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
running 33 tests
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
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 600) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 610) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 620) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 630) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 640) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 660) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 670) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 680) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 690) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 700) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 90) ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 101) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 106) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 111) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 116) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 155) ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 169) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 185) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 192) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 197) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 204) - compile fail ... ok
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"""
    return text


def literal_inventories():
    """Synthetic listing controls are not actual product mutant observations."""
    result = {}
    for subject, filename in [
        ("library", "crates/nevrotick-engine/src/lib.rs"),
        ("cli", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs"),
        ("measurement", "crates/nevrotick-engine/src/measurement.rs"),
        ("relative", "crates/nevrotick-engine/src/relative.rs"),
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

    def test_nominal_type_doctests_cannot_be_omitted_with_consistent_summaries(self):
        healthy = literal_execution(doctests=True)
        for owner in ("NominalPhaseNs", "NominalRoundTripNs"):
            line = next(row for row in healthy.splitlines()
                        if "measurement::" + owner + " " in row and "compile fail" in row)
            bad = healthy.replace(line + "\n", "", 1).replace("ok. 33 passed;", "ok. 32 passed;")
            with self.subTest(owner=owner), self.assertRaisesRegex(MODULE.Incomplete, "construction/privacy"):
                MODULE.validate_test_execution(bad, doctests=True)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 33)

    def test_four_source_owners_have_exact_selections_and_command_targets(self):
        expected = {"library": "crates/nevrotick-engine/src/lib.rs",
                    "cli": "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
                    "measurement": "crates/nevrotick-engine/src/measurement.rs",
                    "relative": "crates/nevrotick-engine/src/relative.rs"}
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
        # This test checks census membership. Exact cohort counts belong to each
        # profile's admission, which must refuse unrelated concurrent source drift.
        self.assertNotIn(MODULE.QUALITY_POLICY, identity)
        self.assertIn("scripts/tests/test_verify_vendor_admission.py", identity)
        self.assertIn("crates/time-core/src/window_variance.rs", identity)
        self.assertIn("crates/time-core/tests/window_variance.rs", identity)
        self.assertIn("crates/nevrotick-engine/src/measurement.rs", identity)
        self.assertIn("crates/nevrotick-engine/src/relative.rs", identity)
        self.assertIn("crates/nevrotick-engine/tests/private/relative.rs", identity)
        self.assertIn("crates/nevrotick-engine/src/series.rs", identity)
        self.assertIn("crates/nevrotick-engine/tests/private/series.rs", identity)
        self.assertIn("crates/nevrotick-engine/tests/series.rs", identity)
        for name in ["scripts/verify-engine-analysis.py", "scripts/tests/test_verify_engine_analysis.py",
                     "policy/engine-analysis.json", MODULE.SERIES_POLICY, MODULE.SERIES_TESTS]:
            self.assertNotIn(name, identity)

    def test_healthy_policy_is_full_profile_and_campaign_initially_held(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); _, identity = self.fixture(root)
            policy = self.admit_fixture(root, identity)
        self.assertEqual(policy["critical_files"], [
            "crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
            "crates/nevrotick-engine/src/measurement.rs", "crates/nevrotick-engine/src/relative.rs"])
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
            self.assertEqual(MODULE.PRODUCT.admit_product(root)[1],
                             MODULE.PRODUCT.admit_product(ROOT)[1])
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

    def test_zero_sign_equivalence_scope_is_an_exact_optional_third_case(self):
        """Synthetic scope controls; actual source/review custody is checked separately."""
        entries = [
            {"id": identifier, "mutant": {
                "file": "crates/nevrotick-engine/src/measurement.rs",
                "function": {"function_name": owner}, "genre": "BinaryOperator",
                "replacement": replacement},
             "proof": {"path": "tasks/engine-analysis-equivalences.md"}}
            for identifier, owner, replacement in [
                ("ENGINE-COHERENCE-LOWER-EQUALITY", "coherence_residual", "<="),
                ("ENGINE-COHERENCE-UPPER-EQUALITY", "coherence_residual", ">="),
                ("ENGINE-NOMINAL-ZERO-SIGN", "nominal_seconds", "<="),
            ]
        ]
        MODULE.validate_equivalence_scope([])
        MODULE.validate_equivalence_scope(entries[:2])
        MODULE.validate_equivalence_scope(entries)
        for change in ("zero_only", "missing_lower", "missing_upper", "duplicate", "foreign_id",
                       "function", "file", "operator", "genre", "proof_path"):
            bad = copy.deepcopy(entries)
            if change == "zero_only": bad = bad[2:]
            if change == "missing_lower": bad.pop(0)
            if change == "missing_upper": bad.pop(1)
            if change == "duplicate": bad.append(copy.deepcopy(bad[2]))
            if change == "foreign_id": bad[2]["id"] = "ENGINE-UNREVIEWED-ZERO"
            if change == "function": bad[2]["mutant"]["function"]["function_name"] = "coherence_residual"
            if change == "file": bad[2]["mutant"]["file"] = "crates/nevrotick-engine/src/lib.rs"
            if change == "operator": bad[2]["mutant"]["replacement"] = ">="
            if change == "genre": bad[2]["mutant"]["genre"] = "FnValue"
            if change == "proof_path": bad[2]["proof"]["path"] = "tasks/foreign.md"
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_equivalence_scope(bad)

    @staticmethod
    def commit_fallback_entries():
        """Literal native descriptor identities; not a semantic-equivalence oracle."""
        filename = "crates/nevrotick-engine/src/lib.rs"
        entries = []
        for identifier, replacement in (
            ("ENGINE-COMMIT-DURATION-FALLBACK-SUB", "-="),
            ("ENGINE-COMMIT-DURATION-FALLBACK-MUL", "*="),
        ):
            entries.append({"id": identifier,
                "source_sha256": "6f3c6eb2f491de7f27a89f8fba84556e818874a9c875643b08081206f5dd4d46",
                "mutant": {
                "name": f"{filename}:1335:23: replace += with {replacement} in commit_reply",
                "package": "nevrotick-engine", "file": filename,
                "function": {"function_name": "commit_reply", "return_type": "",
                             "span": {"start": {"line": 1315, "column": 1},
                                      "end": {"line": 1364, "column": 2}}},
                "span": {"start": {"line": 1335, "column": 23},
                         "end": {"line": 1335, "column": 25}},
                "replacement": replacement, "genre": "BinaryOperator"},
                "proof": {"path": "tasks/engine-analysis-equivalences.md"}})
        return entries

    def test_commit_fallback_scope_requires_complete_exact_native_pair(self):
        entries = self.commit_fallback_entries()
        failure = None
        try:
            MODULE.validate_equivalence_scope(entries)
        except MODULE.Incomplete as error:
            failure = str(error)
        self.assertIsNone(failure, "Both exact reviewed fallback descriptors must be admitted.")
        for change in ("single", "duplicate", "foreign_id", "function", "file", "operator",
                       "genre", "proof_path", "reachable_counter", "function_span", "source", "mixed"):
            bad = copy.deepcopy(entries)
            if change == "single": bad.pop()
            elif change == "duplicate": bad.append(copy.deepcopy(bad[0]))
            elif change == "foreign_id": bad[0]["id"] = "ENGINE-UNREVIEWED-FALLBACK"
            elif change == "function": bad[0]["mutant"]["function"]["function_name"] = "stage_reply"
            elif change == "file": bad[0]["mutant"]["file"] = "crates/nevrotick-engine/src/measurement.rs"
            elif change == "operator": bad[0]["mutant"]["replacement"] = "+="
            elif change == "genre": bad[0]["mutant"]["genre"] = "FnValue"
            elif change == "proof_path": bad[0]["proof"]["path"] = "tasks/foreign.md"
            elif change == "reachable_counter":
                mutant = bad[0]["mutant"]
                mutant["name"] = mutant["name"].replace(":1335:", ":1331:")
                mutant["span"]["start"]["line"] = 1331
                mutant["span"]["end"]["line"] = 1331
            elif change == "function_span": bad[0]["mutant"]["function"]["span"]["end"]["line"] += 1
            elif change == "source": bad[0]["source_sha256"] = "a" * 64
            else:
                bad.append({"id": "ENGINE-COHERENCE-LOWER-EQUALITY"})
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_equivalence_scope(bad)

    def test_commit_fallback_review_uses_actual_product_and_shared_custody(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy, proof = self.equivalence_fixture(root)
            entries = self.commit_fallback_entries()
            proof.write_text("# Synthetic fallback admission controls, not product qualification\n" +
                             "".join("\n## " + item["id"] + "\nSynthetic review fixture.\n" for item in entries))
            for entry in entries:
                entry.update(candidate_sha256=policy["candidate_sha256"],
                             source_sha256=MODULE.PRODUCT.sha256(root / entry["mutant"]["file"]),
                             requirement="CHECK-6/25/27",
                             rationale="Synthetic exact native-descriptor custody control.")
                entry["proof"].update(sha256=MODULE.PRODUCT.sha256(proof), section=entry["id"],
                                      reviewer="independent synthetic control",
                                      verdict="independently-justified-equivalent")
            policy["mutation_equivalences"] = entries
            self.write_policy(root, policy)
            historical_source = self.commit_fallback_entries()[0]["source_sha256"]
            if entries[0]["source_sha256"] == historical_source:
                self.assertEqual(MODULE.read_policy(root), policy)
                self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SCOPE, policy), entries)
            else:
                # A copied current tree is not the historical reviewed library.
                # Relabelling its source/candidate cannot reuse old native spans.
                with self.assertRaisesRegex(MODULE.Incomplete, "exact source and native descriptor"):
                    MODULE.read_policy(root)

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

    def test_measurement_and_relative_scope_reject_missing_extra_and_redirected_selection(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            for filename in ("crates/nevrotick-engine/src/measurement.rs",
                             "crates/nevrotick-engine/src/relative.rs"):
                for change in ("missing", "extra", "redirected"):
                    bad = copy.deepcopy(healthy)
                    if change == "missing": bad["critical_files"].remove(filename)
                    elif change == "extra": bad["critical_files"].append("crates/nevrotick-engine/src/foreign.rs")
                    else: bad["critical_files"][bad["critical_files"].index(filename)] = "crates/nevrotick-engine/src/foreign.rs"
                    self.write_policy(root, bad)
                    with self.subTest(filename=filename, change=change), self.assertRaisesRegex(MODULE.Incomplete, "scope"):
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

    def test_relative_owner_requires_one_critical_and_security_registration_after_repinning(self):
        """A fresh manifest pin cannot authorize an absent or ambiguous owner."""
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            self.assertEqual(self.admit_fixture(root, identity), healthy)
            path = root / "policy/verification-manifest.toml"
            original = path.read_text()
            critical = '    "crates/nevrotick-engine/src/relative.rs",\n'
            security = next(line + "\n" for line in original.splitlines()
                            if 'name = "engine-experimental-relative-measurement"' in line)
            self.assertEqual(original.count(critical), 1)
            for change in ("critical_missing", "critical_duplicate", "critical_redirected",
                           "security_missing", "security_duplicate", "security_redirected", "security_package"):
                if change == "critical_missing": changed = original.replace(critical, "", 1)
                elif change == "critical_duplicate": changed = original.replace(critical, critical * 2, 1)
                elif change == "critical_redirected": changed = original.replace(critical, critical.replace("relative.rs", "foreign.rs"), 1)
                elif change == "security_missing": changed = original.replace(security, "", 1)
                elif change == "security_duplicate": changed = original.replace(security, security * 2, 1)
                elif change == "security_redirected": changed = original.replace(security, security.replace("relative.rs", "foreign.rs"), 1)
                else: changed = original.replace(security, security.replace('package = "nevrotick-engine"', 'package = "wrong"'), 1)
                path.write_text(changed)
                bad = copy.deepcopy(healthy)
                bad["inputs_sha256"]["policy/verification-manifest.toml"] = hashlib.sha256(path.read_bytes()).hexdigest()
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "matching critical/security owner"):
                    self.admit_fixture(root, identity)
            path.write_text(original)
            self.write_policy(root, healthy)
            self.assertEqual(self.admit_fixture(root, identity), healthy)

    def test_relative_private_tests_require_exact_protected_bytes_and_path(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            name = "crates/nevrotick-engine/tests/private/relative.rs"
            self.assertIn(name, healthy["inputs_sha256"])
            path = root / name; original = path.read_bytes()
            for change in ("missing", "redirected", "content"):
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["inputs_sha256"].pop(name)
                elif change == "redirected": bad["inputs_sha256"][name.replace("relative.rs", "foreign.rs")] = bad["inputs_sha256"].pop(name)
                else: path.write_bytes(original + b"\n// changed protected test bytes\n")
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "bindings changed"):
                    self.admit_fixture(root, identity)
                path.write_bytes(original)
            self.write_policy(root, healthy)
            self.assertEqual(self.admit_fixture(root, identity), healthy)

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

    def test_campaign_bindings_must_cover_four_nonempty_exact_inventories(self):
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        for bindings in [{}, {"library": {"count": 1, "sha256": "a" * 64}},
                         {name: {"count": count, "sha256": "a" * 64}
                          for name, count in [("library", 1), ("cli", 1), ("measurement", 0), ("relative", 1)]},
                         {name: {"count": True, "sha256": "a" * 64}
                          for name in ["library", "cli", "measurement", "relative"]},
                         {name: {"count": 1, "sha256": "not-a-hash"}
                          for name in ["library", "cli", "measurement", "relative"]}]:
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.inventory_bindings(policy, required=True)
        healthy = {name: {"count": 1, "sha256": "a" * 64} for name in ["library", "cli", "measurement", "relative"]}
        policy["mutation_inventories"] = healthy
        self.assertEqual(len(MODULE.inventory_bindings(policy, required=True)), 4)
        for change in ("missing", "missing_relative", "extra", "redirected"):
            bad = copy.deepcopy(healthy)
            if change == "missing": bad.pop("measurement")
            elif change == "missing_relative": bad.pop("relative")
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
                         {"library", "cli", "measurement", "relative"})
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
                         ["tests_executed"], 68)
        full = MODULE.validate_test_execution(literal_execution(doctests=True), doctests=True)
        self.assertEqual((full["tests_executed"], full["doctests"]), (101, 33))

    def test_relative_bridge_cases_reject_individual_omissions_with_consistent_counts(self):
        """Independent literal cases require the real bridge and each numeric boundary."""
        healthy = literal_execution(doctests=False)
        required = (
            "processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals",
            "measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks",
            "measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding",
            "measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign",
            "relative::tests::midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints",
            "relative::tests::midpoint_publication_enforces_raw_association_and_garde_rule",
            "relative::tests::duration_nanoseconds_admits_full_u64_and_rejects_next_tick",
            "relative::tests::reference_instants_check_order_before_duration_conversion",
            "relative::tests::relative_reference_differences_keep_direction_and_half_ticks",
            "relative::tests::relative_phase_raw_i128_endpoints_keep_u128_magnitude",
            "relative::tests::shared_origin_distinguishes_equal_values_from_run_identity",
            "relative::tests::relative_seconds_scale_requires_nonzero_with_healthy_neighbors",
            "relative::tests::relative_diagnostics_preserve_fixed_text_and_original_causes",
        )
        self.assertEqual(len(required), 13)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
        for name in required:
            record = "test " + name + " ... ok\n"
            self.assertEqual(healthy.count(record), 1)
            bad = healthy.replace(record, "", 1).replace("running 59 tests", "running 58 tests", 1)
            bad = bad.replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaisesRegex(MODULE.Incomplete, "required actual"):
                MODULE.validate_test_execution(bad, doctests=False)

    def test_relative_doctests_require_both_legal_views_and_all_nine_privacy_failures(self):
        """Missing cases still fail when the native doctest counts agree with the log."""
        healthy = literal_execution(doctests=True)
        docs = [line for line in healthy.splitlines()
                if line.startswith("test crates/nevrotick-engine/src/relative.rs - ")]
        self.assertEqual(len(docs), 11)
        self.assertEqual(sum("relative::ExperimentalReferenceNs " in line and "compile fail" not in line
                             for line in docs), 1)
        self.assertEqual(sum("relative::ExperimentalReferenceNs " in line and "compile fail" in line
                             for line in docs), 4)
        self.assertEqual(sum("relative::ExperimentalRelativeMeasurement " in line and "compile fail" not in line
                             for line in docs), 1)
        self.assertEqual(sum("relative::ExperimentalRelativeMeasurement " in line and "compile fail" in line
                             for line in docs), 5)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 33)
        for line in docs:
            bad = healthy.replace(line + "\n", "", 1).replace("running 33 tests", "running 32 tests", 1)
            bad = bad.replace("33 passed;", "32 passed;", 1)
            with self.subTest(omitted=line), self.assertRaisesRegex(MODULE.Incomplete, "construction/privacy"):
                MODULE.validate_test_execution(bad, doctests=True)
        for change, before, after in (
            ("file", "src/relative.rs - relative::ExperimentalReferenceNs",
             "src/lib.rs - relative::ExperimentalReferenceNs"),
            ("owner", "relative::ExperimentalRelativeMeasurement (line 204)",
             "relative::ExperimentalReferenceNs (line 204)"),
            ("compile_failure", "relative::ExperimentalRelativeMeasurement (line 204) - compile fail",
             "relative::ExperimentalRelativeMeasurement (line 204)"),
        ):
            self.assertEqual(healthy.count(before), 1 if change != "file" else 5)
            bad = healthy.replace(before, after, 1)
            with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "construction/privacy"):
                MODULE.validate_test_execution(bad, doctests=True)

    def test_constructor_and_guard_alternatives_cannot_be_omitted_from_baseline(self):
        """A healthy full transcript discriminates the earlier seven-case baseline."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
        added = (
            "processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries",
            "processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays",
            "processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap",
            "processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections",
            "processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions",
            "processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor",
        )
        for name in added:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_argument_budget_cases_cannot_be_omitted_from_baseline(self):
        """Checked resource owners require their actual native unit records."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
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
            "measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt",
            "measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint",
        )
        for name in required:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaisesRegex(MODULE.Incomplete, "required actual"):
                MODULE.validate_test_execution(text, doctests=False)

    def test_measurement_doctests_require_success_and_all_private_construction_failures(self):
        healthy = literal_execution(doctests=True)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 33)
        docs = [line for line in healthy.splitlines() if line.startswith("test crates/")]
        for line in docs:
            text = healthy.replace(line + "\n", "").replace("33 passed;", "32 passed;", 1)
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
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
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
            text = healthy.replace("test " + name + " ... ok\n", "").replace("59 passed;", "58 passed;", 1)
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
                     literal_execution(doctests=False).replace("59 passed;", "0 passed;", 1)]:
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
            self.assertEqual(MODULE.validate_baseline(work, "library")["tests_executed"], 101)
            bad = json.loads(path.read_text()); bad["outcomes"][0]["log_path"] = "../escape.log"
            path.write_text(json.dumps(bad))
            with self.assertRaisesRegex(MODULE.Incomplete, "escapes"):
                MODULE.validate_baseline(work, "library")

    def test_baseline_hash_uses_retained_bytes_without_reopening_the_log(self):
        """The baseline digest and validated transcript identify one snapshot."""
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); directory = work / "mutation/library/mutants.out"
            (directory / "log").mkdir(parents=True)
            raw = literal_execution(doctests=True).encode("utf-8")
            baseline = directory / "log/baseline.log"
            baseline.write_bytes(raw)
            (directory / "outcomes.json").write_text(json.dumps({"outcomes": [
                {"scenario": "Baseline", "summary": "Success", "duration": 0.25,
                 "log_path": "log/baseline.log"}]}))
            resolved = []
            original_resolve = Path.resolve
            def observe_resolve(path, *arguments, **options):
                resolved.append(path)
                return original_resolve(path, *arguments, **options)
            with patch.object(MODULE.PRODUCT, "sha256", wraps=MODULE.PRODUCT.sha256) as hashing, \
                    patch.object(Path, "resolve", observe_resolve):
                result = MODULE.validate_baseline(work, "library")
            self.assertEqual(result["tests_executed"], 101)
            self.assertEqual(result["log_sha256"], hashlib.sha256(raw).hexdigest())
            self.assertFalse(any(call.args and call.args[0] == baseline
                                 for call in hashing.call_args_list),
                             "The admitted baseline bytes must not be reopened for hashing.")
            self.assertNotIn(baseline, resolved,
                             "Generated paths must not follow links before bounded admission.")

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
        self.assertEqual(len(calls), 4)
        self.assertTrue(all(command[-2:] == ["--list", "--json"] and "--baseline" not in command
                            for _, command, _ in calls))
        self.assertEqual(report["listed_total"], 4)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        self.assertEqual(finished, [True])

    def test_combined_lane_keeps_coverage_first_and_one_final_custody_boundary(self):
        """Control-plane checks only: mocked commands supply no native qualification."""
        for failure_at in (None, "coverage", "finish", "inventory"):
            with self.subTest(failure_at=failure_at), tempfile.TemporaryDirectory() as temp:
                source = Path(temp); policy, identity = self.fixture(source)
                if failure_at != "inventory":
                    policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64}
                                                       for name in MODULE.SUBJECTS}
                self.write_policy(source, policy)
                report, calls, finished, sessions = {}, [], [], []

                class FakeSession:
                    def __init__(self, root, args, received_report, scope, chosen_policy, extra):
                        sessions.append(args.lane)
                        self.source, self.product_identity = source, identity
                        self.work, self.tools = Path("/owned/work"), Path("/owned/tools")
                    def run(self, *args, **options):
                        raise AssertionError("Synthetic control must not execute a command")
                    def finish(self):
                        finished.append(True)
                        if failure_at == "finish":
                            raise MODULE.Incomplete("final custody failure")

                def selected(scope, subjects, chosen, work, tools, run, received_report, lane):
                    calls.append(lane)
                    if lane == failure_at:
                        raise MODULE.Incomplete("selected coverage failure")

                with patch.object(MODULE, "read_policy", return_value=policy), \
                        patch.object(MODULE.COMMON, "Session", FakeSession), \
                        patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=selected):
                    if failure_at is None:
                        MODULE.execute(SimpleNamespace(lane="all", subject="all"), report)
                        self.assertTrue(report["qualification_passed"])
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute(SimpleNamespace(lane="all", subject="all"), report)
                        self.assertFalse(report.get("qualification_passed", False))
                self.assertEqual(sessions, [] if failure_at == "inventory" else ["all"])
                self.assertEqual(finished, [] if failure_at == "inventory" else [True])
                self.assertEqual(calls, [] if failure_at == "inventory" else
                                 ["coverage"] if failure_at == "coverage" else ["coverage", "mutation"])

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

    def test_cli_selectors_route_measurement_and_relative_without_changing_profile_or_lane(self):
        for subject in ("measurement", "relative"):
            def selected(args, report):
                self.assertEqual((args.lane, args.subject, args.tool_admission_profile),
                                 ("coverage", subject, "full"))
            with self.subTest(subject=subject), patch.object(MODULE, "execute", side_effect=selected), \
                    patch("sys.stdout", new_callable=io.StringIO):
                self.assertEqual(MODULE.main(["--lane", "coverage", "--subject", subject,
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


class QualityAnalysisTests(unittest.TestCase):
    """Q1 registration controls use synthetic native logs; no analyzer is invoked."""

    @staticmethod
    def assertion_sources():
        """Small parser fixtures, not historical Rust source or executed product tests."""
        private = """fn exact() { assert_eq!(17, 17); }
fn facts() { assert_eq!(41, 41); }
fn computed_policy_boundaries_preserve_strict_and_inclusive_comparisons() { assert_eq!(1, 1); }
fn declared_getters_and_constructor_precedence_preserve_original_bits() { assert_eq!(1, 1); }
fn dyadic_calculation_borrows_original_facts_and_assumptions() { facts(); exact(); assert!(true); }
fn evaluator_failure_precedence_preserves_original_observation() { assert_eq!(1, 1); }
fn finite_input_overflow_is_refused_at_the_first_computed_stage() { assert_eq!(1, 1); }
fn generated_garde_dispatch_covers_nested_and_computed_fields() { assert_eq!(1, 1); }
fn gradual_underflow_does_not_claim_exact_zero_uncertainty() { assert_eq!(1, 1); }
fn mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields() { facts(); assert!(true); }
fn negative_local_wall_interval_grows_declared_frequency_dispersion() { assert_eq!(1, 1); }
fn negative_server_interval_processing_limit_uses_absolute_magnitude() { assert_eq!(1, 1); }
fn original_measurement_refusal_and_redacted_diagnostics_are_preserved() { assert_eq!(1, 1); }
fn precision_clamps_have_exact_boundary_and_neighbor_values() { assert_eq!(1, 1); }
fn raw_round_trip_is_converted_once_before_cancellation() { assert_eq!(1, 1); }
fn raw_scalar_and_interval_helpers_preserve_full_admitted_domains() { assert_eq!(1, 1); }
fn refusal_families_keep_bounded_diagnostics_and_typed_sources() { assert_eq!(1, 1); }
fn signed_server_interval_and_unsigned_root_words_are_retained() { assert_eq!(1, 1); }
fn zero_raw_rtt_uses_combined_precision_floor() { assert_eq!(1, 1); }
"""
        external = """fn check(_value: bool, _fact: &str) {}
fn facts() { assert_eq!(41, 41); }
fn run_series() { assert_eq!(43, 43); }
fn check_report() { assert_eq!(47, 47); }
fn diagnostic() { assert_eq!(53, 53); }
fn serve() { assert_eq!(59, 59); }
fn check_ratio() { assert_eq!(2, 2); }
fn check_near() { check(true, "fixture quality arithmetic differs"); }
fn check_quality() { check_ratio(); check_near(); }
fn absent_measurement_refuses_quality_with_actual_valid_neighbor() { check_quality(); }
fn actual_quality_limits_refuse_without_rewriting_completion_or_age() { check_quality(); }
fn actual_series_quality_retains_signed_intervals_source_and_original_age() {
    facts(); run_series(); check_report(); diagnostic(); serve(); check_quality();
    check(true, "conditional quality changed literal peer claims");
}
"""
        return {
            "crates/nevrotick-engine/src/quality/tests.rs": private.encode(),
            "crates/nevrotick-engine/tests/quality.rs": external.encode(),
        }

    def semantic_policy(self):
        """Bind only synthetic assertion inputs at this unit boundary, never production policy."""
        policy = self.policy()
        pins = {name: hashlib.sha256(data).hexdigest()
                for name, data in self.assertion_sources().items()}
        policy["inputs_sha256"].update(pins)
        policy["candidate_sha256"] = independent_digest(pins)
        return policy

    def write_assertion_sources(self, root):
        """Materialize the two fixed parser inputs without reading private evidence."""
        for filename, data in self.assertion_sources().items():
            target = root / filename
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)

    @contextmanager
    def ordinary_fixture(self):
        """Temporary synthetic receipt/recipes exercise the real reader, not product evidence."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            recipes = root / "recipes"
            recipes.mkdir()
            recipe_names = (
                "quality_ordinary_v2.py", "quality_ordinary_v1.py", "relative_bridge_ordinary.py",
                "conditioned_fit_ordinary_v2.py", "series_ordinary_v1.py", "series_ordinary_v3.py",
            )
            recipe_pins = {}
            for name in recipe_names:
                data = ("# Synthetic reader fixture only: " + name + "\n").encode()
                (recipes / name).write_bytes(data)
                recipe_pins[name] = hashlib.sha256(data).hexdigest()
            policy = self.policy()
            source = {f"synthetic/source-{index:03}.rs": f"{index:064x}" for index in range(421)}
            policy["candidate_sha256"] = independent_digest(source)
            inventory = policy["test_inventory"]
            receipt_data = {
                "scope": "Synthetic receipt-reader fixture; no Rust execution or qualification.",
                "status": "passed-q1-quality-all", "bookends_unchanged": True,
                "candidate_sha256": policy["candidate_sha256"], "source_sha256": source,
                "recipes_sha256": {str(recipes / name): digest for name, digest in recipe_pins.items()},
                "lanes": [{"lane": lane, "tests": {"executed": len(inventory[key]),
                    "test_names": inventory[key]}} for lane, key in (
                        ("engine-tests", "ordinary"), ("engine-optimized-tests", "ordinary"),
                        ("engine-doctests", "doctests"))],
            }
            receipt = root / "receipt.json"
            raw = json.dumps(receipt_data, sort_keys=True, separators=(",", ":")).encode()
            receipt.write_bytes(raw)
            policy["ordinary_evidence"] = {"receipt": str(receipt),
                "sha256": hashlib.sha256(raw).hexdigest(), "recipes_sha256": recipe_pins}
            with patch.object(MODULE, "QUALITY_EVIDENCE_ROOT", recipes), \
                 patch.object(MODULE, "QUALITY_ORDINARY_RECEIPT", receipt), \
                 patch.object(MODULE, "QUALITY_ORDINARY_RECIPES", recipe_pins):
                yield policy, receipt, recipes

    def test_current_roles_keep_distinct_source_scopes_and_bound_input_sets(self):
        """Current package identities do not merge the two qualification owners."""
        policy = self.policy()
        MODULE.validate_quality_policy(policy)
        ordinary, docs = MODULE.quality_test_inventory(policy)
        self.assertEqual((len(ordinary), len(docs)), (113, 50))
        self.assertEqual(len(MODULE.SERIES_REQUIRED_TESTS), 23)
        self.assertTrue(MODULE.SERIES_CHECKPOINT_REQUIRED_TESTS <= set(ordinary))
        self.assertNotIn(MODULE.SERIES_PROCESS_SOURCE, MODULE.QUALITY_BOUND_INPUTS)
        self.assertIn(MODULE.SERIES_PROCESS_SOURCE, MODULE.SERIES_CURRENT_BOUND_INPUTS)
        self.assertEqual(MODULE.QUALITY_FIXED_POLICY["source_file_count"], 421)

    def policy(self):
        return copy.deepcopy(json.loads((ROOT / MODULE.QUALITY_POLICY).read_text()))

    def execution(self, *, doctests=True):
        """Render declared parser-fixture identities, never claim product execution."""
        inventory = self.policy()["test_inventory"]
        names = inventory["ordinary"] + (inventory["doctests"] if doctests else [])
        return ("\n".join("test " + name + " ... ok" for name in names)
                + f"\ntest result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def phases(self, summary):
        """Independent literal native phase shape for accounting fixtures."""
        phases = ["Build"] if summary == "Unviable" else ["Build", "Test"]
        return [{"phase": phase, "duration": 0.1,
            "argv": ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                     "--verbose", "--package=nevrotick-engine@0.0.0", "--message-format=json", "--frozen"],
            "process_status": {"Failure": 101} if summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test") else "Success"}
            for phase in phases]

    def descriptor(self, name="quality synthetic fault"):
        """Complete synthetic fault descriptor, not actual cargo-mutants enumeration."""
        return {"name": name, "package": "nevrotick-engine", "file": MODULE.QUALITY_SUBJECTS["quality"],
            "function": {"function_name": "fixture_only", "return_type": "bool",
                         "span": {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 1}}},
            "span": {"start": {"line": 1, "column": 2}, "end": {"line": 1, "column": 4}},
            "replacement": "!=", "genre": "BinaryOperator"}

    def baseline_fixture(self, root):
        self.write_assertion_sources(root / "source")
        directory = root / "mutation/quality/mutants.out"
        directory.mkdir(parents=True)
        (directory / "baseline.log").write_text(self.execution())
        report = {"outcomes": [{"scenario": "Baseline", "summary": "Success",
            "phase_results": self.phases("Success"), "log_path": "baseline.log"}]}
        (directory / "outcomes.json").write_text(json.dumps(report))
        return directory, report

    def semantic_sites(self):
        """Use fixed synthetic parser bytes; no historical product-source claim follows."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.write_assertion_sources(root)
            return MODULE.quality_semantic_assertion_sites(root, self.semantic_policy())

    @staticmethod
    def failed_execution(failures):
        """Literal completed libtest failure records; no product execution claimed."""
        noun = "test" if len(failures) == 1 else "tests"
        return (f"running {len(failures)} {noun}\n"
                + "".join(f"test {test} ... FAILED\n" for test, _ in failures)
                + "\nfailures:\n\n"
                + "".join(f"---- {test} stdout ----\n{body}\n\n" for test, body in failures)
                + "failures:\n\n" + "".join(f"    {test}\n" for test, _ in failures)
                + f"\ntest result: FAILED. 0 passed; {len(failures)} failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")

    def caught_fixture(self, directory):
        test = "quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        site=next(site for site in self.semantic_sites()["tests"][test] if site["owner"]=="exact")
        body = (f"thread '{test}' panicked at "
                f"{site['source']}:{site['line']}:{site['column']}:\n"
                f"{site['marker']}\n  left: 17\n right: 19")
        text = self.failed_execution([(test, body)])
        (directory / "caught.log").write_text(text)
        row = {"scenario": {"Mutant": self.descriptor()}, "summary": "CaughtMutant",
               "phase_results": self.phases("CaughtMutant"), "log_path": "caught.log"}
        return text, {"outcomes": [row]}

    def test_quality_role_selects_only_one_owner_with_unchanged_finite_gates(self):
        policy = self.policy()
        self.assertEqual(MODULE.validate_quality_policy(policy), policy)
        self.assertEqual(MODULE.QUALITY_SCOPE.select("all"),
                         {"quality": "crates/nevrotick-engine/src/quality.rs"})
        for subject in ["library", "series", "relative", "measurement", "cli"]:
            with self.subTest(subject=subject), self.assertRaises(MODULE.Incomplete):
                MODULE.QUALITY_SCOPE.select(subject)
        for key, value in {"coverage_minimum_line_percent":90, "mutation_minimum_viable_killed_percent":95,
            "mutation_jobs":8, "jobs":6, "test_threads":1, "minimum_free_bytes":5*1024**3,
            "tool_admission_profile":"full"}.items():
            self.assertEqual(policy[key], value)
        self.assertEqual([entry["id"] for entry in policy["mutation_equivalences"]],
                         ["ENGINE-QUALITY-INTERVAL-ZERO-SIGN"])
        self.assertEqual(policy["mutation_inventories"], {"quality":{"count":167,
            "sha256":"52851ec7a7408ed86034de072648f0e3071e3f4b314142f9ee529fb9b90971f5"}})
        self.assertEqual(policy["source_file_count"],421)
        self.assertEqual(policy["test_inventory"]["baseline_count"],163)
        self.assertEqual(len([name for name in MODULE.QUALITY_REQUIRED_TESTS if name.startswith("quality::tests::")]),17)
        self.assertEqual(len(MODULE.QUALITY_REQUIRED_TESTS),20)
        self.assertEqual(len(policy["ordinary_evidence"]["recipes_sha256"]),6)
        self.assertIn("quality_ordinary_v2.py",policy["ordinary_evidence"]["recipes_sha256"])
        readiness={"declared_getters_and_constructor_precedence_preserve_original_bits",
            "evaluator_failure_precedence_preserves_original_observation",
            "generated_garde_dispatch_covers_nested_and_computed_fields",
            "negative_local_wall_interval_grows_declared_frequency_dispersion",
            "negative_server_interval_processing_limit_uses_absolute_magnitude",
            "raw_scalar_and_interval_helpers_preserve_full_admitted_domains",
            "refusal_families_keep_bounded_diagnostics_and_typed_sources",
            "zero_raw_rtt_uses_combined_precision_floor"}
        self.assertTrue({"quality::tests::"+name for name in readiness}<=MODULE.QUALITY_REQUIRED_TESTS)

    def test_quality_policy_rejects_changed_source_recipe_count_and_exclusions(self):
        changes = [lambda p:p.update(source_file_count=411), lambda p:p.update(mutation_jobs=10),
            lambda p:p.update(tool_admission_profile="stable-core"),
            lambda p:p.update(mutation_equivalences=[{"inherited":"series"}]),
            lambda p:p["inputs_sha256"].update({MODULE.QUALITY_SUBJECTS["quality"]:"a"*64}),
            lambda p:p["ordinary_evidence"]["recipes_sha256"].update({"quality_ordinary_v1.py":"a"*64}),
            lambda p:p["ordinary_evidence"]["recipes_sha256"].update({"quality_ordinary_v2.py":"a"*64}),
            lambda p:p.update(candidate_sha256="a"*64),
            lambda p:p["test_inventory"].update(baseline_count=True)]
        for change in changes:
            policy = self.policy(); change(policy)
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_policy(policy)

    @staticmethod
    def zero_sign_entry():
        """Independent literal reviewed fault; this fixture supplies no native outcome."""
        identifier = "ENGINE-QUALITY-INTERVAL-ZERO-SIGN"
        return {
            "id": identifier,
            "candidate_sha256": "e8f6ead34f41429033a8d6c0ebd823bc693dae8caa645d53a8eafedf20e8727d",
            "source_sha256": "f3abb4b00c769b3a4fb655cb923df7626c4209cb792594a55f065449051e2135",
            "mutant": {
                "file": "crates/nevrotick-engine/src/quality.rs", "package": "nevrotick-engine",
                "name": "crates/nevrotick-engine/src/quality.rs:524:29: replace < with <= in interval_seconds",
                "function": {"function_name": "interval_seconds",
                    "return_type": "-> Result<RoundedRationalSeconds, QualityRefusal>",
                    "span": {"start": {"line": 514, "column": 1}, "end": {"line": 531, "column": 2}}},
                "span": {"start": {"line": 524, "column": 29}, "end": {"line": 524, "column": 30}},
                "replacement": "<=", "genre": "BinaryOperator"},
            "requirement": "REQ-8/BND-1; CHECK-25/27; canonical rational zero",
            "rationale": "Exact reviewed full typed-domain constructor canonicalization; synthetic admission control only.",
            "proof": {"path": "tasks/engine-analysis-equivalences.md",
                "sha256": hashlib.sha256((ROOT / "tasks/engine-analysis-equivalences.md").read_bytes()).hexdigest(),
                "section": identifier, "reviewer": "independent zero-sign source review; observer fixture",
                "verdict": "independently-justified-equivalent"}}

    def test_quality_equivalence_opt_in_is_only_the_exact_zero_sign_fault(self):
        entry = self.zero_sign_entry()
        self.assertEqual(independent_digest(entry["mutant"]),
                         "ab870f10eb8cde026ababd980eb91338a2026e753395d4cd178bd50b8b6393ec")
        MODULE.validate_quality_equivalence_scope([])
        MODULE.validate_quality_equivalence_scope([entry])
        for change in ("list", "duplicate", "foreign", "source", "line", "span", "replacement",
                       "function", "return_type", "genre", "package", "file", "proof_path", "proof_section",
                       "proof_hash", "verdict", "reviewer", "candidate", "rationale", "extra"):
            bad = copy.deepcopy(entry)
            if change == "list": bad = None
            elif change == "foreign": bad["id"] = "ENGINE-SERIES-UNAVAILABLE-ADMITTED-ZERO"
            elif change == "source": bad["source_sha256"] = "a" * 64
            elif change == "line": bad["mutant"]["span"]["start"]["line"] = 525
            elif change == "span": bad["mutant"]["function"]["span"]["end"]["line"] = 532
            elif change == "replacement": bad["mutant"]["replacement"] = ">="
            elif change == "function": bad["mutant"]["function"]["function_name"] = "exact_intervals"
            elif change == "return_type": bad["mutant"]["function"]["return_type"] = "-> f64"
            elif change == "genre": bad["mutant"]["genre"] = "FnValue"
            elif change == "package": bad["mutant"]["package"] = "time-domain"
            elif change == "file": bad["mutant"]["file"] = "crates/time-domain/src/rational_seconds.rs"
            elif change == "proof_path": bad["proof"]["path"] = "tasks/foreign.md"
            elif change == "proof_section": bad["proof"]["section"] = "ENGINE-NOMINAL-ZERO-SIGN"
            elif change == "proof_hash": bad["proof"]["sha256"] = "bad"
            elif change == "verdict": bad["proof"]["verdict"] = "unreviewed"
            elif change == "reviewer": bad["proof"]["reviewer"] = " "
            elif change == "candidate": bad["candidate_sha256"] = False
            elif change == "rationale": bad["rationale"] = " "
            elif change == "extra": bad["unknown"] = True
            cohort = None if change == "list" else [bad, copy.deepcopy(bad)] if change == "duplicate" else [bad]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_equivalence_scope(cohort)
        policy = self.policy(); policy["mutation_equivalences"] = []
        MODULE.validate_quality_policy(policy)
        policy["mutation_equivalences"] = [entry]
        MODULE.validate_quality_policy(policy)
        policy["mutation_equivalences"][0]["candidate_sha256"] = "a" * 64
        with self.assertRaisesRegex(MODULE.Incomplete, "complete policy candidate"):
            MODULE.validate_quality_policy(policy)

    def test_quality_converter_and_dossier_are_explicit_protected_inputs(self):
        converter, dossier = "crates/time-domain/src/rational_seconds.rs", "tasks/engine-analysis-equivalences.md"
        policy = self.policy()
        self.assertIn(converter, MODULE.QUALITY_BOUND_INPUTS)
        self.assertIn(dossier, MODULE.QUALITY_BOUND_INPUTS)
        self.assertEqual(policy["inputs_sha256"][converter],
                         "2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438")
        self.assertEqual(policy["inputs_sha256"][dossier], policy["mutation_equivalences"][0]["proof"]["sha256"])
        policy["inputs_sha256"][converter] = "a" * 64
        with self.assertRaisesRegex(MODULE.Incomplete, "bindings are invalid"):
            MODULE.validate_quality_policy(policy)

    def test_quality_equivalent_native_miss_is_excluded_without_kill_credit(self):
        entry = self.zero_sign_entry(); equivalent = entry["mutant"]
        caught = self.descriptor("quality synthetic caught companion")
        expected = {row["name"]: row for row in (caught, equivalent)}
        report = {"cargo_mutants_version": "27.1.0", "total_mutants": 2,
            "caught": 1, "missed": 1, "unviable": 0, "timeout": 0, "success": 0,
            "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": self.phases("Success")},
                {"scenario": {"Mutant": caught}, "summary": "CaughtMutant", "phase_results": self.phases("CaughtMutant")},
                {"scenario": {"Mutant": equivalent}, "summary": "MissedMutant", "phase_results": self.phases("MissedMutant")}]}
        def observe(data, native=2):
            return MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE, data, expected,
                "crates/nevrotick-engine/src/quality.rs", native_exit_code=native,
                equivalences=[entry], defer_gate=True)
        result = observe(report)
        self.assertEqual((result["killed"], result["survived"], result["excluded"],
                          result["raw_denominator"], result["adjusted_denominator"]), (1, 1, 1, 2, 1))
        with self.assertRaisesRegex(MODULE.Incomplete, "native exit"): observe(report, native=0)
        bad = copy.deepcopy(report); bad["outcomes"][-1].update(summary="CaughtMutant", phase_results=self.phases("CaughtMutant"))
        bad.update(caught=2, missed=0)
        with self.assertRaisesRegex(MODULE.Incomplete, "not an actual missed mutant"):
            observe(bad, native=0)

    def test_quality_ordinary_evidence_rejects_changed_actual_receipt_and_recipe_bytes(self):
        with self.ordinary_fixture() as (policy, receipt, recipes):
            admitted = MODULE.admit_quality_ordinary(policy)
            self.assertFalse(admitted["fresh_execution"])
            original = receipt.read_bytes()
            receipt.write_bytes(b"changed synthetic receipt")
            with self.assertRaisesRegex(MODULE.Incomplete,"receipt bytes"):
                MODULE.admit_quality_ordinary(policy)
            receipt.write_bytes(original)
            recipe = recipes / "quality_ordinary_v2.py"
            original_recipe = recipe.read_bytes()
            recipe.write_bytes(b"# changed synthetic recipe\n")
            with self.assertRaisesRegex(MODULE.Incomplete,"recipes"):
                MODULE.admit_quality_ordinary(policy)
            recipe.write_bytes(original_recipe)
            self.assertFalse(MODULE.admit_quality_ordinary(policy)["fresh_execution"])
            with self.assertRaisesRegex(MODULE.Incomplete, "Unreviewed quality"):
                MODULE.validate_quality_policy(policy)
        policy = self.policy()
        with patch.object(MODULE,"read_json",return_value=policy), \
             patch.object(MODULE.PRODUCT,"admit_product",return_value=({}, {"new-source.rs":"a"*64})):
            with self.assertRaisesRegex(MODULE.Incomplete,"complete current product"):
                MODULE.read_quality_policy(ROOT)

    def test_quality_ordinary_parses_only_the_exact_hashed_raw_byte_observation(self):
        with self.ordinary_fixture() as (policy, receipt, _recipes):
            original=receipt.read_bytes()
            read=MODULE.COMMON.read_analysis_bytes
            def replaced_after_read(path,**kwargs):
                data=read(path,**kwargs)
                receipt.write_text('{"status":"replaced-after-hash"}')
                return data
            with patch.object(MODULE.COMMON,"read_analysis_bytes",side_effect=replaced_after_read), \
                 patch.object(MODULE,"read_json",side_effect=AssertionError("receipt was reopened")):
                admitted=MODULE.admit_quality_ordinary(policy)
            self.assertEqual(admitted["sha256"],hashlib.sha256(original).hexdigest())
            self.assertEqual(json.loads(receipt.read_text())["status"],"replaced-after-hash")
        for raw in [b'{"a":1,"a":2}',b'{"n":NaN}',b'{"n":1e9999}',b'[]',b'\xff']:
            bad=self.policy();bad["ordinary_evidence"]["sha256"]=hashlib.sha256(raw).hexdigest()
            with patch.object(MODULE.COMMON,"read_analysis_bytes",return_value=raw):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_quality_ordinary(bad)

    def test_quality_inventory_starts_empty_and_requires_exact_complete_nonempty_campaign(self):
        policy = self.policy()
        policy["mutation_inventories"] = {}
        self.assertEqual(MODULE.quality_inventory_bindings(policy), {})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.quality_inventory_bindings(policy, required=True)
        descriptor = self.descriptor()
        inventory = {descriptor["name"]:descriptor}
        binding = {"quality":{"count":1,"sha256":independent_digest(inventory)}}
        policy["mutation_inventories"] = binding
        self.assertEqual(MODULE.quality_inventory_bindings(policy, required=True), binding)
        admitted = MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.QUALITY_SCOPE, policy, {"quality":inventory})
        self.assertEqual(admitted, binding)
        for invalid in [{"series":binding["quality"]}, {"quality":{"count":0,"sha256":"a"*64}},
                        {"quality":{"count":True,"sha256":"a"*64}}, {"quality":{"count":513,"sha256":"a"*64}}]:
            policy["mutation_inventories"] = invalid
            with self.assertRaises(MODULE.Incomplete):
                MODULE.quality_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {"quality":{"count":2,"sha256":independent_digest(inventory)}}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.QUALITY_SCOPE, policy, {"quality":inventory})

    def test_quality_execution_requires_complete_unique_actual_native_records(self):
        policy = self.policy()
        healthy = self.execution()
        self.assertEqual(MODULE.validate_quality_test_execution(healthy, policy, doctests=True)["tests_executed"],
                         policy["test_inventory"]["baseline_count"])
        ordinary = self.execution(doctests=False)
        self.assertEqual(MODULE.validate_quality_test_execution(ordinary, policy, doctests=False)["ordinary_tests"],
                         len(policy["test_inventory"]["ordinary"]))
        first = healthy.splitlines()[0]
        for invalid in ["", healthy.replace(first+"\n", "", 1), first+"\n"+healthy,
                        healthy.replace(" ... ok", " ... FAILED", 1),
                        healthy.replace("0 ignored", "1 ignored"), healthy.replace("0 filtered out", "1 filtered out")]:
            with self.subTest(invalid=invalid[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_test_execution(invalid, policy, doctests=True)

    def test_quality_baseline_requires_selected_successful_full_native_execution(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory, report = self.baseline_fixture(root)
            result = MODULE.validate_quality_baseline(root, "quality", self.semantic_policy())
            self.assertEqual(result["tests_executed"], self.policy()["test_inventory"]["baseline_count"])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_baseline(root, "library", self.semantic_policy())
            for rows in [[], report["outcomes"]*2, [{**report["outcomes"][0],"summary":"CaughtMutant"}]]:
                (directory/"outcomes.json").write_text(json.dumps({"outcomes":rows}))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_baseline(root, "quality", self.semantic_policy())
            (directory/"outcomes.json").write_text(json.dumps(report))
            (directory/"baseline.log").write_text("test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\n")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_baseline(root, "quality", self.semantic_policy())

    def test_quality_caught_causes_require_owned_selected_behavioral_assertions(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            healthy, outcomes = self.caught_fixture(directory)
            result = MODULE.validate_quality_caught_causes(directory, outcomes, self.semantic_sites())
            self.assertEqual(len(result["checked_caught_logs"]), 1)
            for text in [healthy.replace("src/quality/tests.rs", "src/series.rs"),
                         healthy.replace("assertion `left == right` failed", "Error: fixture setup failed"),
                         healthy+"Q1_HARNESS_INCOMPLETE\n", healthy+"N3_HARNESS_INCOMPLETE\n",
                         healthy+"error[E0308]: mismatched types\n",
                         healthy+'Error: Custom { kind: Other, error: "unrelated peer failure" }\n',
                         healthy.replace("quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions", "processing_guard_tests::fixture")]:
                (directory/"caught.log").write_text(text)
                with self.subTest(text=text[:60]), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory, outcomes, self.semantic_sites())
            (directory/"caught.log").write_text(healthy)
            for change in [lambda p:p["outcomes"].append(copy.deepcopy(p["outcomes"][0])),
                lambda p:p["outcomes"][0].update(log_path="../outside.log"),
                lambda p:p["outcomes"][0]["scenario"]["Mutant"].update(file="crates/nevrotick-engine/src/lib.rs")]:
                bad = copy.deepcopy(outcomes); change(bad)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory, bad, self.semantic_sites())

    def test_quality_caught_causes_require_complete_native_failure_records(self):
        mapping = self.semantic_sites()
        test = "quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            healthy, outcomes = self.caught_fixture(directory)
            prior = ("running 1 test\ntest fixture::healthy ... ok\n"
                     "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")
            for complete in (healthy, prior + healthy):
                (directory / "caught.log").write_text(complete)
                result = MODULE.validate_quality_caught_causes(directory, outcomes, mapping)
                self.assertEqual(len(next(iter(result["checked_caught_logs"].values()))["causes"]), 1)
            section = healthy[healthy.index("---- "):healthy.rindex("\nfailures:\n")]
            foreign = healthy.replace("running 1 test\n", "running 2 tests\n").replace(
                f"test {test} ... FAILED\n", f"test {test} ... FAILED\ntest fixture::peer ... FAILED\n"
            ).replace("0 passed; 1 failed", "0 passed; 2 failed").replace(
                "\n\ntest result:", "\n    fixture::peer\n\ntest result:")
            bad = {
                "section only": section,
                "truncated after assertion": healthy[:healthy.rindex("\nfailures:\n")],
                "truncated before summary": healthy[:healthy.rindex("test result:")],
                "signal": healthy + "error: process did not exit successfully (signal: 9, SIGKILL: kill)\n",
                "abort": healthy + "fatal runtime error: thread stack overflow, aborting\n",
                "duplicate failed row": healthy + f"test {test} ... FAILED\n",
                "duplicate truncated run": healthy + f"running 1 test\ntest {test} ... FAILED\n",
                "truncated later passing block": healthy + "running 1 test\ntest fixture::healthy ... ok\n",
                "inconsistent summary": healthy.replace("0 passed; 1 failed", "1 passed; 1 failed"),
                "wrong running count": healthy.replace("running 1 test\n", "running 2 tests\n"),
                "foreign failed row without section": foreign,
            }
            for label, text in bad.items():
                (directory / "caught.log").write_text(text)
                with self.subTest(label=label), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory, outcomes, mapping)

    def test_quality_semantic_sites_are_bound_to_source_and_reachable_helpers(self):
        mapping=self.semantic_sites()
        test="quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        self.assertTrue(any(site["owner"]=="exact" for site in mapping["tests"][test]))
        constructor="quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields"
        self.assertFalse(any(site["owner"]=="exact" for site in mapping["tests"][constructor]))
        self.assertTrue(all(site["owner"] not in {"facts","run_series","check_report","diagnostic","serve"}
            for sites in mapping["tests"].values() for site in sites))
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            self.write_assertion_sources(root)
            self.assertEqual(MODULE.quality_semantic_assertion_sites(root,self.semantic_policy()),mapping)
            private=root/"crates/nevrotick-engine/src/quality/tests.rs"
            private.write_text("\n"+private.read_text())
            with self.assertRaisesRegex(MODULE.Incomplete,"admitted bytes"):
                MODULE.quality_semantic_assertion_sites(root,self.semantic_policy())

    def test_quality_correct_filename_requires_exact_named_site_and_assertion_kind(self):
        mapping=self.semantic_sites()
        test="quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        exact=next(site for site in mapping["tests"][test] if site["owner"]=="exact")
        foreign=next(site for site in mapping["tests"][
            "quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields"]
            if site["kind"]=="panic_assertion")
        with tempfile.TemporaryDirectory() as temporary:
            directory=Path(temporary);healthy,outcomes=self.caught_fixture(directory)
            location=f"{exact['source']}:{exact['line']}:{exact['column']}"
            for replacement in [f"{exact['source']}:0:{exact['column']}",
                f"{exact['source']}:999999:{exact['column']}",f"{exact['source']}:{exact['line']}:0",
                f"{exact['source']}:{exact['line']}:999999",
                f"{foreign['source']}:{foreign['line']}:{foreign['column']}"]:
                (directory/"caught.log").write_text(healthy.replace(location,replacement))
                with self.subTest(location=replacement), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
            for changed in [healthy.replace(exact["marker"],"assertion `left != right` failed"),
                            healthy.replace("thread '"+test+"'","thread 'unrelated-worker'")]:
                (directory/"caught.log").write_text(changed)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)

    def test_quality_primary_assertion_allows_only_closed_secondary_product_refusals(self):
        mapping=self.semantic_sites()
        secondary="quality::tests::precision_clamps_have_exact_boundary_and_neighbor_values"
        with tempfile.TemporaryDirectory() as temporary:
            directory=Path(temporary);healthy,outcomes=self.caught_fixture(directory)
            primary="quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
            body=healthy.split(f"---- {primary} stdout ----\n",1)[1].rsplit("\nfailures:\n",1)[0].rstrip("\n")
            def mixed(error):
                return self.failed_execution([(primary,body),(secondary,error)])
            for error in ["Error: InvalidAssumption(HistoricalQuantum)","Error: NonFinite(FrequencyWidth)"]:
                (directory/"caught.log").write_text(mixed(error))
                result=MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
                row=next(iter(result["checked_caught_logs"].values()))
                self.assertEqual(len(row["causes"]),1)
                self.assertEqual(len(row["secondary_product_refusals_without_credit"]),1)
            for error in ['Error: Custom { kind: Other, error: "peer fixture failed" }',
                'Error: Custom { kind: TimedOut, error: "peer timed out" }',"Error: Unavailable",
                "Error: InvalidAssumption(UnknownField)","no cause recorded"]:
                (directory/"caught.log").write_text(mixed(error))
                with self.subTest(error=error), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
            (directory/"caught.log").write_text(self.failed_execution([
                (secondary,"Error: InvalidAssumption(HistoricalQuantum)")]))
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_caught_causes(directory,outcomes,mapping)

    def test_quality_external_static_fact_is_source_bound_and_peer_failure_is_rejected(self):
        mapping=self.semantic_sites()
        test="actual_series_quality_retains_signed_intervals_source_and_original_age"
        site=next(site for site in mapping["tests"][test] if site["owner"]=="check_near")
        with tempfile.TemporaryDirectory() as temporary:
            directory=Path(temporary);_,outcomes=self.caught_fixture(directory)
            healthy=self.failed_execution([(test,site["marker"])])
            (directory/"caught.log").write_text(healthy)
            result=MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
            cause=next(iter(result["checked_caught_logs"].values()))["causes"][0]
            self.assertEqual((cause["source"],cause["line"]),(site["source"],site["line"]))
            for marker in [site["marker"].replace("arithmetic","unregistered arithmetic"),
                'Error: Custom { kind: Other, error: "conditional quality changed literal peer claims" }']:
                (directory/"caught.log").write_text(healthy.replace(site["marker"],marker))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)

    def test_quality_score_cannot_count_compiler_harness_timeout_or_missing_inventory(self):
        expected = {}
        rows = [{"scenario":"Baseline","summary":"Success","phase_results":self.phases("Success")}]
        for number in range(20):
            descriptor = self.descriptor("synthetic quality fault "+str(number))
            expected[descriptor["name"]] = descriptor
            rows.append({"scenario":{"Mutant":descriptor},"summary":"CaughtMutant",
                         "phase_results":self.phases("CaughtMutant")})
        outcomes = {"cargo_mutants_version":"27.1.0","outcomes":rows,"total_mutants":20,
                    "caught":20,"missed":0,"unviable":0,"timeout":0,"success":0}
        result = MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE, outcomes, expected,
            MODULE.QUALITY_SUBJECTS["quality"], native_exit_code=0)
        self.assertEqual(result["viable_killed_percent"], 100)
        for change in [lambda p:p.update(caught=19), lambda p:p["outcomes"].pop(),
                       lambda p:p["outcomes"][1].update(summary="Timeout"),
                       lambda p:p["outcomes"][1]["phase_results"][0].update(process_status={"Failure":101})]:
            bad=copy.deepcopy(outcomes); change(bad)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE,bad,expected,
                    MODULE.QUALITY_SUBJECTS["quality"],native_exit_code=0)
        bad=copy.deepcopy(outcomes)
        bad.update(caught=18,missed=2)
        for row in bad["outcomes"][-2:]:
            row.update(summary="MissedMutant",phase_results=self.phases("MissedMutant"))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE,bad,expected,
                MODULE.QUALITY_SUBJECTS["quality"],native_exit_code=2)

    def test_quality_external_probes_and_constructor_omissions_keep_real_boundaries(self):
        examples = MODULE.quality_contract_examples()
        self.assertEqual(sum(item["code"] is None for item in examples.values()),1)
        self.assertTrue({"E0451","E0616","E0308","E0277","E0515"} <= {item["code"] for item in examples.values()})
        self.assertEqual(examples["quality_assumptions_substitution"]["code"],"E0616")
        self.assertEqual(examples["quality_assumptions_substitution"]["tokens"],["assumptions","private"])
        for owner in ("facts","assumptions"):
            case=examples["quality_"+owner+"_owner_escape"]
            self.assertEqual(case["code"],"E0515")
            self.assertEqual(case["tokens"],["return value","function parameter",owner])
        self.assertIn("facts: TransportFacts, assumptions: &'a DeclaredQualityAssumptions",
                      examples["quality_facts_owner_escape"]["source"])
        self.assertIn("facts: &'a TransportFacts, assumptions: DeclaredQualityAssumptions",
                      examples["quality_assumptions_owner_escape"]["source"])
        self.assertIn("fn retained<'a>",examples["quality_legal"]["source"])
        self.assertIn("evaluate_conditional_quality(facts, assumptions)",examples["quality_legal"]["source"])
        for specimen in examples.values():
            self.assertEqual(specimen["line"],4)
            self.assertTrue(specimen["source"].splitlines()[3])
            self.assertNotIn("execute_series(",specimen["source"])
        source=(ROOT/MODULE.QUALITY_SUBJECTS["quality"]).read_text()
        variants=MODULE.quality_validation_variants(source)
        self.assertEqual(set(variants),{"frequency","limits","assumptions"})
        for body in variants.values():
            self.assertEqual(source.count("candidate.validate()")-body.count("candidate.validate()"),1)
        with self.assertRaises((MODULE.Incomplete,ValueError)):
            MODULE.quality_validation_variants(source.replace("candidate.validate()","candidate.check()"))

    def test_quality_contract_phases_share_the_original_cumulative_wall_budget(self):
        examples=MODULE.quality_contract_examples()
        # Twenty seconds have already elapsed since the caller's original epoch.
        now=[25.0];deadline=305.0;observed=[];calls=[]
        def remaining(actual,maximum):
            observed.append(actual)
            self.assertEqual(actual,deadline)
            value=int(actual-now[0])
            if value<=0:
                raise MODULE.Incomplete("shared contract deadline exhausted")
            return min(maximum,value)
        def run(label,command,timeout,**kwargs):
            calls.append((label,timeout))
            now[0]+=min(timeout,5 if label.startswith("external-") else 80)
            if label=="external-legal-execution":
                return examples["quality_legal"]["success_marker"]
            if label=="quality-validation-execute:healthy":
                return "Q1_VALIDATION_CONTROL:healthy"
            return ""
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary)
            deps=work/"target/debug/deps";deps.mkdir(parents=True)
            (work/"target/debug/libnevrotick_engine.rlib").write_bytes(b"synthetic emitted library")
            (deps/"libgarde-control.rlib").write_bytes(b"synthetic derive dependency")
            (deps/"libtime_domain-control.rlib").write_bytes(b"synthetic rational dependency")
            session=SimpleNamespace(work=work,source=ROOT,stable=work/"stable",run=run,protect=lambda _:None)
            report={}
            with patch.object(MODULE.time,"monotonic",side_effect=lambda:now[0]), \
                 patch.object(MODULE.COMMON,"remaining_timeout",side_effect=remaining), \
                 patch.object(MODULE.COMMON,"library_artifact",return_value="/work/target/debug/libnevrotick_engine.rlib"):
                MODULE.contracts(session,self.policy(),report,examples=examples,deadline=deadline)
                with self.assertRaisesRegex(MODULE.Incomplete,"shared contract deadline"):
                    MODULE.quality_validation_contracts(session,report,deadline=deadline)
            self.assertTrue(any(label.startswith("quality-validation-") for label,_ in calls))
            self.assertTrue(observed)
            self.assertEqual(set(observed),{deadline})
            self.assertEqual(now[0],deadline)
            self.assertNotIn("quality_validation_controls",report)

    def test_quality_preflight_and_unknown_subject_cannot_launch_a_native_session(self):
        with tempfile.TemporaryDirectory() as temporary:
            args=SimpleNamespace(profile="quality",lane="enumerate",subject="quality",
                output_dir=Path(temporary)/"output",tool_admission_profile="full")
            with patch.object(MODULE,"read_quality_policy",return_value=self.policy()), \
                 patch.object(MODULE.shutil,"disk_usage",return_value=SimpleNamespace(free=5*1024**3-1)), \
                 patch.object(MODULE.COMMON,"Session") as session:
                with self.assertRaisesRegex(MODULE.Incomplete,"five GiB"):
                    MODULE.execute(args,{})
                session.assert_not_called()
                args.subject="library"
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute(args,{})
                session.assert_not_called()



class EngineKeAnalysisRoleTests(unittest.TestCase):
    """Synthetic admission controls only; no product, compiler or analyzer runs."""

    HOST = {
        "ke_core": "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs",
        "ke_association": "crates/nevrotick-engine/src/nts_ke_experiment/association.rs",
        "ke_process": "crates/nevrotick-engine/src/nts_ke_experiment/process.rs",
        "ke_handoff": "crates/nevrotick-engine/src/nts_ke_experiment/handoff.rs",
        "ke_request": "crates/nevrotick-engine/src/nts_ke_experiment/request.rs",
        "ke_fault": "crates/nevrotick-engine/src/nts_ke_experiment/transport_fault.rs",
        "ke_ledger": "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs",
        "ke_retirement": "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/retirement.rs",
        "ke_response": "crates/nevrotick-engine/src/nts_ke_experiment/response.rs",
        "ke_datagram": "crates/nevrotick-engine/src/nts_ke_experiment/datagram.rs",
        "ke_recovery": "crates/nevrotick-engine/src/nts_ke_experiment/recovery.rs",
    }

    class SessionReached(Exception):
        """A healthy admission reached Session; stop before any execution."""

    def policy_fixture(self):
        """Independently specified limits and clearly synthetic 348/170 identities."""
        critical = ["crates/nevrotick-engine/src/lib.rs", *self.HOST.values(),
            "crates/nevrotick-engine/src/nts_ke_experiment/profile.rs",
            "crates/nevrotick-engine/src/nts_ke_experiment/transport.rs",
            "tools/crypto-cng-lifecycle/src/bin/nts-ke-probe.rs"]
        names = sorted([f"ordinary_fixture_{index:03}" for index in range(178)] +
                       [f"nts_ke_experiment::fixture_case_{index:02}" for index in range(170)])
        return {
            "schema_version": 1, "package": "nevrotick-engine",
            "admission_status": "admitted-engine-ke-source-analysis",
            "subjects": self.HOST.copy(), "critical_files": critical,
            "features": ["ke-association-prototype"], "test_workspace": False,
            "tool_admission_profile": "stable-core",
            "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
            "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 3072,
            "baseline_wall_seconds": 300, "coverage_wall_seconds": 300,
            "enumeration_wall_seconds": 240, "mutation_wall_seconds": 1200,
            "mutation_build_seconds": 90, "mutation_test_seconds": 60,
            "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
            "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
            "minimum_free_bytes": 5 * 1024**3, "stop_free_bytes": 4 * 1024**3,
            "owned_output_maximum_allocated_bytes": 768 * 1024**2,
            "owned_output_maximum_entries": 32768, "mutation_equivalences": [],
            "campaigns_admitted": False, "mutation_inventories": {},
            "binding": {"candidate_sha256": "a" * 64, "source_file_count": 2,
                "inputs_sha256": {name: "b" * 64 for name in MODULE.ENGINE_KE_BOUND_INPUTS},
                "test_inventory": names, "source_context_sha256": "c" * 64,
                "ordinary_evidence": {"path": "/synthetic/ordinary.json", "sha256": "d" * 64},
                "root_graph_evidence": {"path": "/synthetic/graph.json", "sha256": "e" * 64}},
        }

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / "source"
        self.root.mkdir(mode=0o700)
        self.policy = self.policy_fixture()
        for name in MODULE.ENGINE_KE_BOUND_INPUTS:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("Synthetic admitted source marker; never compiled.\n")
        critical = self.policy["critical_files"]
        manifest = ("critical_files = " + json.dumps(critical) + "\nsecurity_modules = [\n" +
            '{package="nevrotick-engine",files=' + json.dumps(critical[:-1]) + '},\n' +
            '{package="nevrotick-crypto-cng-lifecycle",files=' + json.dumps(critical[-1:]) + '},\n]\n')
        (self.root / "policy/verification-manifest.toml").write_text(manifest)
        self.identity = {name: MODULE.PRODUCT.sha256(self.root / name)
                         for name in ("Cargo.lock", "crates/nevrotick-engine/Cargo.toml")}
        binding = self.policy["binding"]
        binding["candidate_sha256"] = independent_digest(self.identity)
        binding["inputs_sha256"] = {name: MODULE.PRODUCT.sha256(self.root / name)
                                    for name in MODULE.ENGINE_KE_BOUND_INPUTS}
        for field in ("ordinary_evidence", "root_graph_evidence"):
            path = self.base / (field + ".json")
            path.write_text('{"scope":"synthetic reference; no product evidence"}\n')
            binding[field] = {"path": str(path), "sha256": MODULE.PRODUCT.sha256(path)}
        self.args = SimpleNamespace(profile="ke_association", lane="baseline", subject="all",
            tool_admission_profile="stable-core", output_dir=self.base / "new-output", vendor_dir=self.base / "vendor")

    def invoke(self, *, accepted=False, free=5 * 1024**3):
        """Exercise real pre-Session guards with assumed prior product admission."""
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        dependency = {"engine_ke_association": {"rows": {}}}
        with patch.object(MODULE, "ROOT", self.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=(dependency, self.identity)), \
                patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=free)), \
                patch.object(MODULE.COMMON, "Session", side_effect=self.SessionReached) as session:
            with self.assertRaises(self.SessionReached if accepted else MODULE.Incomplete):
                MODULE.execute_engine_ke_profile(self.args, {})
            if accepted:
                session.assert_called_once()
            else:
                session.assert_not_called()

    def test_closed_scope_features_and_gates_keep_healthy_neighbor(self):
        self.assertEqual(MODULE.ENGINE_KE_SCOPE.select("all"), self.HOST)
        MODULE.validate_engine_ke_policy(self.policy)
        for field, value in (("features", []), ("features", ["foreign"]),
            ("features", ["ke-association-prototype", "foreign"]), ("test_workspace", True),
            ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
            ("jobs", 12), ("critical_files", self.policy["critical_files"][:-1])):
            bad = copy.deepcopy(self.policy); bad[field] = value
            with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_policy(bad)
        for scope in (MODULE.COMMON.Scope("foreign", tuple(self.HOST.items()), MODULE.ENGINE_KE_POLICY),
                      MODULE.COMMON.Scope("nevrotick-engine", tuple(self.HOST.items()), "policy/foreign.json"),
                      MODULE.COMMON.Scope("nevrotick-engine", tuple(self.HOST.items())[:-1], MODULE.ENGINE_KE_POLICY)):
            with self.subTest(scope=scope), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.require_engine_ke_role(scope, self.policy)
        self.invoke(accepted=True)

    def test_source_helper_and_candidate_pin_drift_refuse_before_session(self):
        for name in ("scripts/portable-analysis.py", "crates/nevrotick-engine/src/nts_ke_experiment/association.rs"):
            path = self.root / name; original = path.read_bytes()
            path.write_bytes(original + b"changed bytes\n")
            with self.subTest(source=name): self.invoke()
            path.write_bytes(original)
        original = self.policy["binding"]["candidate_sha256"]
        self.policy["binding"]["candidate_sha256"] = "f" * 64
        self.invoke()
        self.policy["binding"]["candidate_sha256"] = original
        self.invoke(accepted=True)

    def test_reference_byte_drift_refuses_before_session(self):
        for field in ("ordinary_evidence", "root_graph_evidence"):
            path = Path(self.policy["binding"][field]["path"]); original = path.read_bytes()
            path.write_bytes(original + b" ")
            with self.subTest(reference=field): self.invoke()
            path.write_bytes(original)
        self.invoke(accepted=True)

    def test_five_gib_startup_boundary_refuses_below_and_accepts_exact(self):
        self.invoke(free=5 * 1024**3 - 1)
        self.invoke(accepted=True, free=5 * 1024**3)

    def test_premature_campaign_and_empty_review_refuse_before_session(self):
        for lane in ("mutation", "all"):
            self.args.lane = lane
            with self.subTest(lane=lane): self.invoke()
        self.policy["campaigns_admitted"] = True
        self.invoke()
        self.args.lane = "baseline"; self.policy["campaigns_admitted"] = False
        self.invoke(accepted=True)

    def test_malformed_complete_inventory_cannot_admit_campaign(self):
        self.policy["campaigns_admitted"] = True
        for record in ({}, {"count": 0, "sha256": "a" * 64},
                       {"count": True, "sha256": "a" * 64}, {"count": 1, "sha256": "bad"}):
            self.policy["mutation_inventories"] = {name: copy.deepcopy(record) for name in self.HOST}
            with self.subTest(record=record), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_policy(self.policy, campaign=True)

    def phases(self, *, feature=True):
        """Pinned native Cargo argument format; synthetic phase observations only."""
        return {"summary": "Success", "phase_results": [
            {"phase": phase, "argv": ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                "--verbose", "--package=nevrotick-engine@0.0.0",
                *(["--features=ke-association-prototype"] if feature else []),
                "--message-format=json", "--frozen"], "duration": 0.01, "process_status": "Success"}
            for phase in ("Build", "Test")]}

    def test_emitted_feature_argv_requires_exact_pinned_order_and_legacy_stays_closed(self):
        healthy = self.phases()
        MODULE.COMMON.mutation_phases(MODULE.ENGINE_KE_SCOPE, healthy)
        MODULE.COMMON.mutation_phases(MODULE.SCOPE, self.phases(feature=False))
        for change in ("omitted", "foreign", "all", "order"):
            bad = copy.deepcopy(healthy)
            for phase in bad["phase_results"]:
                args = phase["argv"]; index = args.index("--features=ke-association-prototype")
                if change == "omitted": del args[index]
                elif change == "foreign": args[index] = "--features=foreign"
                elif change == "all": args[index] = "--all-features"
                else: args.append(args.pop(index))
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_phases(MODULE.ENGINE_KE_SCOPE, bad)
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.mutation_phases(MODULE.SCOPE, healthy)

    def test_campaign_effect_boundary_refuses_before_any_execution(self):
        tools = Path("/owned/synthetic-tools")
        command = [str(tools / "cargo-mutants"), "mutants", "--package", "nevrotick-engine",
            "--file", self.HOST["ke_association"], "--test-workspace", "false", "--cap-lints", "false",
            "--jobs", "10", "--jobserver", "true", "--jobserver-tasks", "10",
            "--cargo-arg=--message-format=json", "--features", "ke-association-prototype",
            "--baseline", "run", "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/ke_association"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", command,
                expected_jobs=10, expected_message_format="json", policy=self.policy)
        self.policy["campaigns_admitted"] = True
        self.policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64} for name in self.HOST}
        MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", command,
            expected_jobs=10, expected_message_format="json", policy=self.policy)
        bad = command.copy(); bad[bad.index("--test-workspace") + 1] = "true"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", bad,
                expected_jobs=10, expected_message_format="json", policy=self.policy)

    def execution(self, names):
        """Build synthetic complete entered libtest output, never product evidence."""
        return (f"running {len(names)} tests\n" + "".join(f"test {name} ... ok\n" for name in names) +
                f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def test_entered_blocks_require_all_348_cases_and_170_ke_neighbors(self):
        binding = self.policy["binding"]; names = binding["test_inventory"]
        healthy = self.execution(names)
        self.assertEqual(MODULE.validate_engine_ke_execution(healthy, binding)["tests_executed"], 348)
        self.assertEqual(MODULE.validate_engine_ke_execution(healthy, binding)["ke_tests_executed"], 170)
        missing = [name for name in names if name != "nts_ke_experiment::fixture_case_00"]
        old = [name for name in names if not name.startswith("nts_ke_experiment::")]
        variants = [self.execution([]), self.execution(old), self.execution(missing),
            self.execution([*names, names[0]]), healthy.replace(" ... ok", " ... FAILED", 1),
            healthy.replace("0 filtered out", "1 filtered out"), healthy.replace("running 348", "running 347"),
            healthy.replace("test result: ok. 348 passed", "test result: ok. 347 passed"),
            "\n".join(line for line in healthy.splitlines() if not line.startswith("running "))]
        for text in variants:
            with self.subTest(sample=text[-120:]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_execution(text, binding)
        altered = copy.deepcopy(binding); altered["test_inventory"][0] = "nts_ke_experiment::unexecuted_other_case"
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_engine_ke_execution(healthy, altered)

    def test_doctest_forms_require_opt_in_and_preserve_the_complete_ordinary_inventory(self):
        binding = self.policy["binding"]; names = binding["test_inventory"]
        ordinary = self.execution(names)
        # Rustdoc no_run success is compilation-only evidence, not execution of its body.
        doc_forms = ["src/lib.rs - Example::checked (line 10)",
                     "src/lib.rs - Example::private (line 20) - compile fail",
                     "src/lib.rs - Example::prepared (line 30) - compile"]
        for doc in doc_forms:
            text = ordinary + self.execution([doc])
            with self.subTest(doctest=doc):
                result = MODULE.validate_engine_ke_execution(text, binding, include_doctests=True)
                self.assertEqual(result["additional_doctests"], [doc])
                self.assertEqual(result["tests_executed"], 349)
                self.assertEqual(result["ke_tests_executed"], 170)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_engine_ke_execution(text, binding)
        invalid = [self.execution(names[1:]) + self.execution([doc_forms[0]]),
                   ordinary + self.execution([doc_forms[0] + " - execute"]),
                   ordinary + self.execution([doc_forms[2], doc_forms[2]]),
                   ordinary + self.execution([doc_forms[1]]) + self.execution([doc_forms[1]])]
        for text in invalid:
            with self.subTest(sample=text[-160:]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_execution(text, binding, include_doctests=True)

    def test_reviewed_descriptors_must_match_actual_listing_not_only_count(self):
        inventories = {name: {name + " synthetic fault": {"name": name + " synthetic fault",
            "file": path, "package": "nevrotick-engine", "replacement": "false"}}
            for name, path in self.HOST.items()}
        self.policy["mutation_inventories"] = {name: {"count": 1, "sha256": independent_digest(items)}
            for name, items in inventories.items()}
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.ENGINE_KE_SCOPE, self.policy, inventories)
        bad = copy.deepcopy(inventories); bad["ke_association"]["ke_association synthetic fault"]["replacement"] = "true"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.ENGINE_KE_SCOPE, self.policy, bad)

    def test_historical_six_owner_scope_cannot_qualify_new_owners(self):
        self.assertIn("ke_ledger", MODULE.ENGINE_KE_HOST_SUBJECTS)
        self.assertIn("ke_response", MODULE.ENGINE_KE_HOST_SUBJECTS)
        self.assertIn("ke_datagram", MODULE.ENGINE_KE_HOST_SUBJECTS)
        bad = copy.deepcopy(self.policy)
        for name in ("ke_ledger", "ke_response"):
            del bad["subjects"][name]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)
        bad = copy.deepcopy(self.policy)
        del bad["subjects"]["ke_datagram"]
        bad["critical_files"].remove(self.HOST["ke_datagram"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)
        bad = copy.deepcopy(self.policy)
        bad["binding"]["test_inventory"] = bad["binding"]["test_inventory"][:233]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)

    def test_doctest_lane_preserves_compilation_only_meaning(self):
        names = ["src/lib.rs - Example::new (line 1)", "src/lib.rs - Example::no_run (line 2) - compile",
                 "src/lib.rs - Example::private (line 3) - compile fail"]
        result = MODULE.validate_engine_ke_doctests(self.execution(names))
        self.assertEqual(result["tests_executed"], 3)
        self.assertEqual(result["compilation_only"], names[1:])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_doctests(self.execution(["ordinary_case"]))

    def test_baseline_keeps_full_release_docs_and_windows_source_checks_separate(self):
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        calls = []
        outer = self
        class Session:
            def __init__(self, root, _args, report, *_arguments):
                self.product_identity = outer.identity
                self.source = root
                self.output = outer.args.output_dir
                self.output.mkdir(mode=0o700)
                self.work = self.output / "work"
                self.work.mkdir(mode=0o700)
                (self.work / "target").mkdir(mode=0o700)
                self.release = self.work / "target/release"
                self.release.mkdir(mode=0o700)
                self.keep = [self.work / "target/debug/keep", self.work / "target/unrelated/keep",
                             self.output / "raw.log", self.source / "source-sentinel"]
                for path in self.keep:
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_text("retained synthetic sentinel\n")
                self.report = report
                report["lanes"] = []
            def run(self, label, argv, timeout):
                calls.append((label, argv, timeout))
                self.report["lanes"].append({"lane": label, "exit_code": 0, "failure": None})
                if label == "ke-release":
                    (self.release / "completed-artifact").write_bytes(b"x" * 65536)
                if label == "ke-doctests":
                    outer.assertFalse(self.release.exists(), "completed release target must retire before doctests")
                    for path in self.keep:
                        outer.assertEqual(path.read_text(), "retained synthetic sentinel\n")
                if label in {"ke-baseline", "ke-release"}:
                    return outer.execution(outer.policy["binding"]["test_inventory"])
                if label == "ke-doctests":
                    return outer.execution(["src/lib.rs - Example::no_run (line 1) - compile"])
                return "Synthetic compiler result only; no product execution.\n"
            def finish(self):
                calls.append(("finish", [], 0))
        report = {}
        with patch.object(MODULE, "ROOT", self.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                patch.object(MODULE.COMMON, "Session", Session):
            MODULE.execute_engine_ke_profile(self.args, report)
        self.assertEqual([row[0] for row in calls], ["ke-format", "ke-windows-check", "ke-baseline", "ke-clippy",
            "ke-release", "ke-doctests", "ke-windows-clippy", "finish"])
        self.assertIn("--release", calls[4][1])
        self.assertIn("--message-format=json", calls[1][1])
        for label, argv, _timeout in (calls[1], calls[6]):
            self.assertEqual(argv[argv.index("--target") + 1], "x86_64-pc-windows-msvc")
            self.assertIn("--all-targets", argv)
            self.assertNotIn("build", argv)
            self.assertNotIn("run", argv)
        self.assertEqual(report["baseline_execution"]["tests_executed"], 348)
        self.assertEqual(report["release_execution"]["ke_tests_executed"], 170)
        self.assertEqual(report["doctest_execution"]["compilation_only"],
                         ["src/lib.rs - Example::no_run (line 1) - compile"])
        retired = report["release_target_retirement"]
        self.assertEqual((retired["path"], retired["completed_lane"], retired["removed"]),
                         ("work/target/release", "ke-release", True))
        self.assertGreaterEqual(retired["before"]["allocated_bytes"], 65536)
        self.assertEqual([row["phase"] for row in report["storage_samples"]
                          if row["lane"] == "ke-release-target-retirement"], ["before", "after"])
        self.assertFalse(report["qualification_passed"])

    def release_retirement_fixture(self):
        """Real private files plus explicitly synthetic completed-child observations."""
        temporary = tempfile.TemporaryDirectory(prefix="retirement-", dir=self.base)
        self.addCleanup(temporary.cleanup)
        output = Path(temporary.name)
        work = output / "work"; work.mkdir(mode=0o700)
        target = work / "target"; target.mkdir(mode=0o700)
        release = target / "release"; release.mkdir(mode=0o700)
        (release / "artifact").write_bytes(b"owned release artifact")
        (output / "raw.log").write_text("retained raw child output")
        session = SimpleNamespace(output=output, work=work)
        report = {"lanes": [{"lane": "ke-release", "exit_code": 0, "failure": None}],
                  "release_execution": {"tests_executed": 348, "ke_tests_executed": 170}}
        MODULE.address_storage_checkpoint(session, self.policy, report, "ke-release", "after")
        return session, report, release

    def test_release_retirement_refuses_links_special_and_noncanonical_or_public_parents(self):
        for case in ("release-link", "child-link", "fifo", "target-link", "noncanonical", "public-work"):
            with self.subTest(case=case):
                session, report, release = self.release_retirement_fixture()
                outside = session.output.parent / (session.output.name + "-outside")
                outside.mkdir(mode=0o700)
                self.addCleanup(shutil.rmtree, outside)
                sentinel = outside / "release/keep"; sentinel.parent.mkdir()
                sentinel.write_text("unrelated external sentinel")
                if case == "release-link":
                    shutil.rmtree(release)
                    release.symlink_to(outside / "release", target_is_directory=True)
                elif case == "child-link":
                    (release / "linked").symlink_to(outside, target_is_directory=True)
                elif case == "fifo":
                    os.mkfifo(release / "pipe", mode=0o600)
                elif case == "target-link":
                    release.parent.rename(session.work / "saved-target")
                    (session.work / "target").symlink_to(outside, target_is_directory=True)
                elif case == "noncanonical":
                    (session.output / "alias").mkdir(mode=0o700)
                    session.work = session.output / "alias/../work"
                else:
                    session.work.chmod(0o755)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.retire_engine_ke_release_target(session, self.policy, report)
                self.assertEqual(sentinel.read_text(), "unrelated external sentinel")
                self.assertEqual((session.output / "raw.log").read_text(), "retained raw child output")
                self.assertNotIn("release_target_retirement", report)

    def test_failed_release_validation_or_postlane_cap_prevents_retirement_and_later_lanes(self):
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        for mode in ("command", "validation", "storage"):
            with self.subTest(mode=mode):
                self.args.output_dir = self.base / ("negative-release-" + mode)
                calls = []; report = {}; outer = self
                artifact = self.args.output_dir / "work/target/release/artifact"
                class Session:
                    def __init__(self, root, _args, report, *_arguments):
                        self.product_identity = outer.identity; self.source = root
                        self.output = outer.args.output_dir; self.output.mkdir(mode=0o700)
                        self.work = self.output / "work"; self.work.mkdir(mode=0o700)
                        (self.work / "target").mkdir(mode=0o700)
                        artifact.parent.mkdir(mode=0o700)
                        self.report = report; report["lanes"] = []
                    def run(self, label, argv, timeout):
                        calls.append(label)
                        self.report["lanes"].append({"lane": label, "exit_code": 1 if label == "ke-release" and mode == "command" else 0,
                                                     "failure": None})
                        if label == "ke-release":
                            artifact.write_text("completed or failed release evidence")
                            if mode == "command":
                                raise MODULE.Incomplete("Synthetic release child failure")
                            names = outer.policy["binding"]["test_inventory"]
                            return outer.execution(names[:-1] if mode == "validation" else names)
                        if label == "ke-baseline":
                            return outer.execution(outer.policy["binding"]["test_inventory"])
                        return "Synthetic compiler result only; no product execution.\n"
                    def finish(self):
                        calls.append("finish")
                original_usage = MODULE.address_storage_usage
                def measured_usage(directory):
                    usage = original_usage(directory)
                    if mode == "storage" and artifact.exists():
                        return {**usage, "allocated_bytes": 805306368}
                    return usage
                with patch.object(MODULE, "ROOT", self.root), \
                        patch.object(MODULE.PRODUCT, "admit_product", return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                        patch.object(MODULE.COMMON, "Session", Session), \
                        patch.object(MODULE, "address_storage_usage", side_effect=measured_usage):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.execute_engine_ke_profile(self.args, report)
                self.assertEqual(calls, ["ke-format", "ke-windows-check", "ke-baseline", "ke-clippy", "ke-release", "finish"])
                self.assertEqual(artifact.read_text(), "completed or failed release evidence")
                self.assertNotIn("release_target_retirement", report)
                if mode == "storage":
                    self.assertEqual((report["storage_samples"][-1]["lane"], report["storage_samples"][-1]["phase"],
                                      report["storage_samples"][-1]["allocated_bytes"]), ("ke-release", "after", 805306368))

class EngineKeFixtureLaneTests(unittest.TestCase):
    """One public generator, never synthetic Engine qualification evidence."""

    NAMES = ("HEALTHY ZERO_COOKIES EMPTY_COOKIE TWO_COOKIES EIGHT_COOKIES NINE_COOKIES "
        "MISSING_UID SHORT_UID DUPLICATE_UID CLEAR_COOKIE CLEAR_PLACEHOLDER INNER_UID INNER_AUTH "
        "INNER_PLACEHOLDER UNKNOWN_FIELDS_AND_SUFFIX SUFFIX_DUPLICATE_UID_AUTH MALFORMED_INNER "
        "MAX_INNER_FIELDS TOO_MANY_INNER MAX_OUTER_FIELDS TOO_MANY_OUTER MAX_PACKET_BYTES "
        "TOO_MANY_PACKET_BYTES WRONG_UID WRONG_ORIGIN WRONG_VERSION WRONG_MODE UNSYNCHRONIZED "
        "STRATUM_ZERO STRATUM_SIXTEEN NONCE_ZERO NONCE_ONE NONCE_TWELVE").split()

    def output(self, names=None, body="    0x00, 0xff,\n"):
        """Clearly synthetic public literals inside a complete captured libtest block."""
        source = ("// SPDX-License-Identifier: GPL-2.0-only\n"
            "//! Public component fixtures generated through the existing maintained AES-SIV API.\n"
            "//! No TLS/exporter/native/entropy or production authentication provenance is claimed.\n")
        for name in self.NAMES if names is None else names:
            source += (f"/// Public independently generated valid-tag fixture: {name}.\n"
                f"pub(in crate::nts_ke_experiment) const {name}: &[u8] = &[\n{body}];\n\n")
        return ("running 1 test\ntest generate_ep2_response_fixtures ... ok\n\nsuccesses:\n\n"
            "---- generate_ep2_response_fixtures stdout ----\n" + source +
            "\nsuccesses:\n    generate_ep2_response_fixtures\n\n"
            "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def test_exact_public_roster_and_complete_actual_case(self):
        source, facts = MODULE.validate_engine_ke_fixture_output(self.output())
        self.assertEqual(set(facts["fixtures"]), set(self.NAMES))
        self.assertTrue(facts["generated"])
        self.assertEqual(facts["tests_executed"], 1)
        self.assertEqual(facts["payload_bytes"], 66)
        self.assertTrue(source.startswith("// SPDX-License-Identifier:"))
        self.assertNotIn("successes:", source)
        self.assertNotIn("FIXTURES_GENERATED", source)
        self.assertEqual(facts["fixtures"]["HEALTHY"],
            {"bytes": 2, "sha256": hashlib.sha256(bytes([0, 255])).hexdigest()})

    def test_missing_extra_duplicate_malformed_and_incomplete_refuse(self):
        healthy = self.output()
        bad = [self.output(self.NAMES[:-1]), self.output([*self.NAMES, "FOREIGN"]),
            self.output([*self.NAMES, self.NAMES[0]]), self.output(body=""),
            self.output(body="    0xgg,\n"), self.output(body="    255,\n"),
            self.output(body="    0x00,,\n"), self.output(body="    include_bytes!(\"secret\"),\n"),
            healthy.replace("running 1 test\n", ""), healthy.replace(" ... ok", " ... FAILED"),
            healthy.replace("0 filtered out", "1 filtered out"),
            healthy.replace("fixture: HEALTHY.", "fixture: FOREIGN.")]
        for text in bad:
            with self.subTest(tail=text[-100:]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_fixture_output(text)

    def test_finite_byte_and_source_bounds_keep_healthy_neighbor(self):
        MODULE.validate_engine_ke_fixture_output(self.output().replace(
            "    0x00, 0xff,\n", "    " + "0x00, " * 4100 + "\n", 1))
        for text in (self.output().replace("    0x00, 0xff,\n", "    " + "0x00, " * 4107 + "\n", 1),
                     self.output(body="    " + "0x00, " * 4100 + "\n")):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_fixture_output(text)

    def fixture_context(self):
        """Reuse admission fixtures, deliberately leaving the ordinary binding stale."""
        support = EngineKeAnalysisRoleTests()
        support.setUp()
        self.addCleanup(support.doCleanups)
        support.args.lane = "fixtures"
        support.policy["binding"] = {"deliberately_stale": True}
        generator = support.root / MODULE.ENGINE_KE_FIXTURE_SOURCE
        generator.parent.mkdir(parents=True, exist_ok=True)
        generator.write_text("Synthetic public generator, never compiled.\n")
        support.identity[MODULE.ENGINE_KE_FIXTURE_SOURCE] = MODULE.PRODUCT.sha256(generator)
        lock = ('version = 4\n[[package]]\nname="nevrotick-s115-aes-siv-fixture"\nversion="0.0.0"\n'
            '[[package]]\nname="ntp-wire"\nversion="0.0.0"\n[[package]]\nname="aes-siv"\nversion="0.8.0"\n'
            'source="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="' + "a" * 64 + '"\n')
        (support.root / "Cargo.lock").write_text(lock)
        (support.root / MODULE.ENGINE_KE_FIXTURE_LOCK).write_text(lock)
        support.identity["Cargo.lock"] = MODULE.PRODUCT.sha256(support.root / "Cargo.lock")
        support.identity[MODULE.ENGINE_KE_FIXTURE_LOCK] = MODULE.PRODUCT.sha256(support.root / MODULE.ENGINE_KE_FIXTURE_LOCK)
        (support.root / MODULE.ENGINE_KE_FIXTURE_MANIFEST).write_text("Synthetic manifest marker; never compiled.\n")
        support.fixture_dependency = {"engine_ke_association": {"rows": {}},
            "packages": [{"name": "aes-siv", "version": "0.8.0", "checksum": "a" * 64}]}
        return support

    def test_fixed_commands_source_bookends_and_no_engine_credit(self):
        support = self.fixture_context()
        (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
        calls = []
        outer = self
        class Session:
            def __init__(self, root, args, report, scope, policy, inputs):
                outer.assertEqual(args.lane, "baseline")
                outer.assertIn(MODULE.ENGINE_KE_FIXTURE_SOURCE, inputs)
                self.product_identity = support.identity
                self.source = root
                self.output = support.args.output_dir
                self.output.mkdir(mode=0o700)
            def run(self, label, argv, timeout):
                calls.append((label, argv, timeout))
                return "nevrotick-s115-aes-siv-fixture v0.0.0\n" if label == "fixture-tree" else outer.output()
            def finish(self):
                calls.append(("finish",))
        report = {}
        with patch.object(MODULE, "ROOT", support.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=(support.fixture_dependency, support.identity)), \
                patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE, "validate_engine_ke_policy", side_effect=AssertionError("stale ordinary binding used")):
            MODULE.execute_engine_ke_fixtures(support.args, report)
        self.assertEqual(calls, [("fixture-tree", ["cargo", "tree", "--package", "nevrotick-s115-aes-siv-fixture",
            "--manifest-path", "/work/source/tools/crypto-aes-siv-witness/Cargo.toml",
            "--target", "x86_64-unknown-linux-gnu", "--frozen", "--edges", "normal,build,features"], 120),
            ("fixture-generator", ["cargo", "test", "--package", "nevrotick-s115-aes-siv-fixture",
                "--manifest-path", "/work/source/tools/crypto-aes-siv-witness/Cargo.toml", "--test",
                "ep2_response_fixtures", "--frozen", "--jobs", "1", "--", "--exact",
                "generate_ep2_response_fixtures", "--show-output", "--test-threads=1"], 120), ("finish",)])
        self.assertTrue(report["fixture_generation"]["generated"])
        self.assertFalse(report["qualification_passed"])
        self.assertFalse(report["selected_host_metrics_passed"])
        artifact = support.args.output_dir / "ep2-response-fixtures.rs"
        self.assertEqual(MODULE.PRODUCT.sha256(artifact), report["fixture_generation"]["source_sha256"])

    def test_wrong_role_lane_subject_or_open_campaign_refuses_before_session(self):
        for change in (lambda s: setattr(s.args, "profile", "legacy"), lambda s: setattr(s.args, "lane", "baseline"),
                lambda s: setattr(s.args, "subject", "ke_core"), lambda s: setattr(s.args, "tool_admission_profile", "full"),
                lambda s: s.policy.update(campaigns_admitted=True), lambda s: s.policy.update(jobs=1),
                lambda s: s.policy.update(features=["foreign"]),
                lambda s: s.policy.update(mutation_inventories={name: {"count": 1, "sha256": "a" * 64}
                    for name in EngineKeAnalysisRoleTests.HOST})):
            support = self.fixture_context()
            change(support)
            (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
            with patch.object(MODULE, "ROOT", support.root), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute_engine_ke_fixtures(support.args, {})
                session.assert_not_called()

    def test_cli_cannot_borrow_fixture_lane_for_other_profile_or_subject(self):
        for extra in (["--profile", "legacy"], ["--profile", "ke_association", "--subject", "ke_core"]):
            with patch.object(MODULE.COMMON, "Session") as session, patch.object(MODULE.sys, "stderr", io.StringIO()):
                with self.assertRaises(SystemExit) as error:
                    MODULE.main(["--lane", "fixtures", "--output-dir", "/unused/synthetic", *extra])
                self.assertEqual(error.exception.code, 2)
                session.assert_not_called()

    def test_fixture_startup_free_space_is_enforced_before_session(self):
        support = self.fixture_context()
        (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
        for free, accepted in ((5 * 1024**3 - 1, False), (5 * 1024**3, True)):
            with patch.object(MODULE, "ROOT", support.root), \
                    patch.object(MODULE.PRODUCT, "admit_product", return_value=(support.fixture_dependency, support.identity)), \
                    patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=free)), \
                    patch.object(MODULE.COMMON, "Session", side_effect=EngineKeAnalysisRoleTests.SessionReached) as session:
                with self.assertRaises(EngineKeAnalysisRoleTests.SessionReached if accepted else MODULE.Incomplete):
                    MODULE.execute_engine_ke_fixtures(support.args, {})
                self.assertEqual(session.call_count, int(accepted))

    def test_standalone_lock_requires_exact_selected_root_identities(self):
        support = self.fixture_context()
        facts = MODULE.admit_engine_ke_fixture_lock(support.root, support.fixture_dependency)
        self.assertEqual(facts["registry_packages"], ["aes-siv@0.8.0"])
        lock = support.root / MODULE.ENGINE_KE_FIXTURE_LOCK
        original = lock.read_text()
        for changed in (original.replace('version="0.8.0"', 'version="0.7.0"'),
                        original.replace("a" * 64, "b" * 64),
                        original.replace('name="ntp-wire"', 'name="foreign-owner"'),
                        original + '[[package]]\nname="ntp-wire"\nversion="0.0.0"\n'):
            lock.write_text(changed)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_engine_ke_fixture_lock(support.root, support.fixture_dependency)
        lock.write_text(original)
        dependency = copy.deepcopy(support.fixture_dependency)
        dependency["packages"] = []
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_engine_ke_fixture_lock(support.root, dependency)

    def test_invalid_capture_writes_no_artifact_and_finishes_custody(self):
        support = self.fixture_context()
        (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
        finished = []
        class Session:
            def __init__(self, root, *_arguments):
                self.product_identity = support.identity
                self.source = root
                self.output = support.args.output_dir
                self.output.mkdir(mode=0o700)
            def run(self, *_arguments):
                return "Invalid synthetic output; never a product result.\n"
            def finish(self):
                finished.append(True)
        report = {}
        with patch.object(MODULE, "ROOT", support.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=(support.fixture_dependency, support.identity)), \
                patch.object(MODULE.COMMON, "Session", Session):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.execute_engine_ke_fixtures(support.args, report)
        self.assertEqual(finished, [True])
        self.assertFalse(report["fixture_generation"]["generated"])
        self.assertFalse((support.args.output_dir / "ep2-response-fixtures.rs").exists())


if __name__ == "__main__":
    unittest.main()
