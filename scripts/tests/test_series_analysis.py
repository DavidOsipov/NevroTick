"""Independent N3 profile admission controls; no compiler/analyzer execution.

Current native identities are copied from the actual final-checkpoint ordinary-v3
receipt (88773278...), including Q1 and both checkpoint tests. Synthetic records exercise observer rejection only and claim no product
execution. Historical engine fixtures are not changed or reused as an oracle.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("series_analysis_test", ROOT / "scripts/verify-engine-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

ORDINARY_NAMES = ('absent_measurement_refuses_quality_with_actual_valid_neighbor', 'actual_quality_limits_refuse_without_rewriting_completion_or_age', 'actual_series_quality_retains_signed_intervals_source_and_original_age', 'argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows', 'argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles', 'argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries', 'argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure', 'cli_worker_binary_preserves_literal_transport_contracts', 'cli_worker_binary_rejects_arguments_and_observes_sink_failures', 'execute_rejects_entered_runtime_without_nested_runtime_creation', 'invalidation_preserves_first_completion_and_marks_third_not_started', 'ipv6_real_requests_retain_distinct_identity_and_one_exact_origin', 'measurement::tests::absent_profile_is_a_distinct_transport_only_reason', 'measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values', 'measurement::tests::coherence_helper_rejects_each_reversed_bracket', 'measurement::tests::decimal_remainders_change_the_exact_nominal_phase', 'measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor', 'measurement::tests::era_zero_and_multiple_candidates_remain_distinct', 'measurement::tests::exact_dyadic_results_ignore_the_wire_token', 'measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values', 'measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic', 'measurement::tests::inert_header_does_not_bypass_the_time_reply_guard', 'measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved', 'measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result', 'measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection', 'measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds', 'measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint', 'measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt', 'measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors', 'measurement::tests::out_of_scope_local_coordinate_is_not_rebased', 'measurement::tests::peer_shift_changes_sign_without_changing_rtt', 'measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals', 'measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association', 'measurement::tests::profile_bounds_and_maximum_allowance_are_checked', 'measurement::tests::refusal_codes_and_display_remain_literal_and_redacted', 'measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding', 'measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign', 'measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks', 'measurement::tests::reversed_brackets_and_processing_are_not_ages', 'measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints', 'measurement::tests::software_window_retains_each_local_and_peer_extremum', 'middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt', 'prepare_outside_runtime_owns_and_releases_standard_sockets', 'prepare_rejects_entered_runtime_before_native_startup', 'processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor', 'processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection', 'processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error', 'processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history', 'processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries', 'processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays', 'processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes', 'processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections', 'processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor', 'processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port', 'processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once', 'processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions', 'processing_guard_tests::processing_uses_fresh_deadline_and_current_control', 'processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals', 'processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct', 'processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates', 'processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four', 'processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize', 'processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice', 'processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct', 'processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap', 'quality::tests::computed_policy_boundaries_preserve_strict_and_inclusive_comparisons', 'quality::tests::declared_getters_and_constructor_precedence_preserve_original_bits', 'quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions', 'quality::tests::evaluator_failure_precedence_preserves_original_observation', 'quality::tests::finite_input_overflow_is_refused_at_the_first_computed_stage', 'quality::tests::generated_garde_dispatch_covers_nested_and_computed_fields', 'quality::tests::gradual_underflow_does_not_claim_exact_zero_uncertainty', 'quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields', 'quality::tests::negative_local_wall_interval_grows_declared_frequency_dispersion', 'quality::tests::negative_server_interval_processing_limit_uses_absolute_magnitude', 'quality::tests::original_measurement_refusal_and_redacted_diagnostics_are_preserved', 'quality::tests::precision_clamps_have_exact_boundary_and_neighbor_values', 'quality::tests::raw_round_trip_is_converted_once_before_cancellation', 'quality::tests::raw_scalar_and_interval_helpers_preserve_full_admitted_domains', 'quality::tests::refusal_families_keep_bounded_diagnostics_and_typed_sources', 'quality::tests::signed_server_interval_and_unsigned_root_words_are_retained', 'quality::tests::zero_raw_rtt_uses_combined_precision_floor', 'relative::tests::duration_nanoseconds_admits_full_u64_and_rejects_next_tick', 'relative::tests::midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints', 'relative::tests::midpoint_publication_enforces_raw_association_and_garde_rule', 'relative::tests::reference_instants_check_order_before_duration_conversion', 'relative::tests::relative_diagnostics_preserve_fixed_text_and_original_causes', 'relative::tests::relative_phase_raw_i128_endpoints_keep_u128_magnitude', 'relative::tests::relative_reference_differences_keep_direction_and_half_ticks', 'relative::tests::relative_seconds_scale_requires_nonzero_with_healthy_neighbors', 'relative::tests::shared_origin_distinguishes_equal_values_from_run_identity', 'series::tests::actual_publication_rechecks_control_and_deadline_after_staging', 'series::tests::all_series_debug_outputs_are_exact_fixed_redactions', 'series::tests::current_checkpoint_observes_new_controls_before_expired_deadline', 'series::tests::deadlines_and_controls_do_not_reset_the_series_origin', 'series::tests::entropy_failure_and_entered_runtime_precede_all_acquisition_work', 'series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions', 'series::tests::genuine_terminal_report_admits_only_the_three_position_domain', 'series::tests::partial_report_preserves_primary_and_both_cleanup_causes', 'series::tests::post_commit_control_preserves_header_and_measurement_refusal', 'series::tests::raw_config_and_tokens_have_mandatory_bounded_admission', 'series::tests::receiver_rejects_duplicate_foreign_and_reused_identity_atomically', 'series::tests::selected_native_datagram_after_retirement_is_charged_without_publication', 'series::tests::series_process::semantic_evidence_accepts_only_exact_named_admitted_sites', 'series::tests::setup_and_registration_failures_preserve_actual_prior_completion', 'series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor', 'series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor', 'series::tests::transfer_control_and_irreversible_retirement_are_distinct', 'series_process::semantic_evidence_accepts_only_exact_named_admitted_sites', 'stop_preserves_first_completion_and_marks_third_not_started', 'three_real_requests_retain_distinct_identity_and_one_exact_origin', 'total_deadline_preserves_first_completion_and_prevents_third_send', 'unavailable_measurements_still_consume_once_after_rejected_neighbors')

DOCTEST_NAMES = ('crates/nevrotick-engine/src/lib.rs - Config::new (line 194)', 'crates/nevrotick-engine/src/lib.rs - ControlSchedule::new (line 1583)', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 216)', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 222) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 229) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 237) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 242) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 104) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 117) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 126) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 131) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 94)', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 610)', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 618) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 623) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 631) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 636) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 680)', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 688) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 693) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 701) - compile fail', 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 706) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line 749) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line 755) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line 760) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::DeclaredFrequencyInterval (line 255) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityAssumptions (line 389)', 'crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityAssumptions (line 401) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityAssumptions (line 405) - compile fail', 'crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityLimits (line 305) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 101) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 106) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 111) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 116) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 92)', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 157)', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 169) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 185) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 192) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 197) - compile fail', 'crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 204) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::PreparedSeries (line 209) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::PreparedSeries (line 216) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::SeriesAttempt (line 336) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 81) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 87) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 91) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig::new (line 111)', 'crates/nevrotick-engine/src/series.rs - series::SeriesReport (line 400) - compile fail', 'crates/nevrotick-engine/src/series.rs - series::execute_series (line 1071) - compile')

def digest(value):
    """Independent documented JSON digest, not the production helper."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def policy_fixture():
    """A mutable metadata fixture; no execution result or campaign is inherited."""
    return json.loads((ROOT / "policy/series-analysis.json").read_text())


def literal_execution(names):
    """One explicit native record per independent identity and its exact summary."""
    return ("running " + str(len(names)) + " tests\n"
            + "".join("test " + name + " ... ok\n" for name in names)
            + "test result: ok. " + str(len(names))
            + " passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")


class SeriesAnalysisControls(unittest.TestCase):
    def test_literal_inventory_and_legacy_defaults_remain_distinct(self):
        self.assertEqual(len(ORDINARY_NAMES), 113)
        self.assertEqual(len(DOCTEST_NAMES), 50)
        self.assertEqual(len(MODULE.REQUIRED_TESTS), 68)
        self.assertEqual(len(MODULE.SERIES_REQUIRED_TESTS), 23)
        self.assertEqual(len(MODULE.SERIES_CHECKPOINT_REQUIRED_TESTS), 2)
        self.assertEqual(set(ORDINARY_NAMES), MODULE.REQUIRED_TESTS | MODULE.SERIES_REQUIRED_TESTS
                         | MODULE.SERIES_CHECKPOINT_REQUIRED_TESTS | MODULE.QUALITY_REQUIRED_TESTS)
        self.assertEqual(MODULE.FIXED_POLICY["mutation_jobs"], 10)
        self.assertNotIn("contracts_wall_seconds", MODULE.FIXED_POLICY)
        self.assertNotIn("mutation_equivalences", MODULE.SERIES_FIXED_POLICY)
        self.assertEqual(MODULE.SERIES_FIXED_POLICY["mutation_jobs"], 8)
        self.assertEqual(MODULE.SERIES_SCOPE.select("all"), {
            "library": "crates/nevrotick-engine/src/lib.rs",
            "series": "crates/nevrotick-engine/src/series.rs"})
        self.assertEqual(digest(list(ORDINARY_NAMES)), "c21b82fd32cea0a3a3eefddc8f84d3b43e8591d03e2286d3445f4d334a7ae3c3")
        self.assertEqual(digest(list(DOCTEST_NAMES)), "5eaf6bcb3df3807ecf447ef66d256abb85511048fbd3939da951c21df379e48a")

    def test_exact_healthy_coverage_and_baseline_records(self):
        policy = policy_fixture()
        coverage = MODULE.validate_series_test_execution(literal_execution(ORDINARY_NAMES), policy, doctests=False)
        baseline = MODULE.validate_series_test_execution(literal_execution(ORDINARY_NAMES + DOCTEST_NAMES), policy, doctests=True)
        self.assertEqual(coverage["tests_executed"], 113)
        self.assertEqual(coverage["doctests"], 0)
        self.assertEqual(baseline["tests_executed"], 163)
        self.assertEqual(baseline["doctests"], 50)
        self.assertEqual(baseline["inventory_sha256"], "b4a41aeb46444047904e96043d94e390ab8e34bf7c01a829f1adf55869267ca4")
        kinds = list(baseline["n3_doctests"].values())
        self.assertEqual(kinds.count("compile_fail"), 7)
        self.assertEqual(kinds.count("legal_execution"), 1)
        self.assertEqual(kinds.count("legal_no_run"), 1)
        self.assertIn("network body is not executed", baseline["no_run_meaning"])

    def test_old_package_identity_cannot_be_omitted_behind_new_minimum(self):
        names = list(ORDINARY_NAMES)
        names.remove("prepare_outside_runtime_owns_and_releases_standard_sockets")
        self.assertTrue(MODULE.SERIES_REQUIRED_TESTS <= set(names))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_series_test_execution(literal_execution(names), policy_fixture(), doctests=False)
        names.append("invented_same_count_replacement")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_series_test_execution(literal_execution(names), policy_fixture(), doctests=False)

    def test_each_n3_identity_and_no_run_identity_is_required(self):
        for missing in MODULE.SERIES_REQUIRED_TESTS | MODULE.SERIES_CHECKPOINT_REQUIRED_TESTS:
            with self.subTest(missing=missing), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_test_execution(literal_execution([n for n in ORDINARY_NAMES if n != missing]), policy_fixture(), doctests=False)
        for missing in ("crates/nevrotick-engine/src/series.rs - series::SeriesConfig::new (line 111)",
                        "crates/nevrotick-engine/src/series.rs - series::execute_series (line 1071) - compile"):
            with self.subTest(missing=missing), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_test_execution(literal_execution([n for n in ORDINARY_NAMES + DOCTEST_NAMES if n != missing]), policy_fixture(), doctests=True)

    def test_duplicate_filtered_ignored_zero_and_truncated_native_records_refuse(self):
        healthy = literal_execution(ORDINARY_NAMES)
        bad = [literal_execution(ORDINARY_NAMES + (ORDINARY_NAMES[0],)),
               healthy.replace("0 ignored", "1 ignored"),
               healthy.replace("0 filtered out", "1 filtered out"),
               healthy.replace(" ... ok", " ... ignored", 1),
               healthy.replace("113 passed", "114 passed"),
               healthy[:healthy.index("test result:")],
               healthy + "ok\n", literal_execution(())]
        for text in bad:
            with self.subTest(text=text[-120:]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_test_execution(text, policy_fixture(), doctests=False)

    def test_policy_cannot_redefine_test_truth_or_weaken_fixed_role(self):
        MODULE.validate_series_policy(policy_fixture())
        mutations = [("jobs", True), ("mutation_jobs", 9), ("mutation_process_limit", 128),
                     ("minimum_free_bytes", 4 * 1024**3), ("source_file_count", 387),
                     ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                     ("tool_admission_profile", "stable-core"), ("package", "time-core"),
                     ("mutation_equivalences", [{"id": "historical"}]),
                     ("mutation_error_values", {})]
        for key, value in mutations:
            candidate = policy_fixture(); candidate[key] = value
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_policy(candidate)
        for field in ["ordinary", "doctests"]:
            candidate = policy_fixture(); candidate["test_inventory"][field][0] = "changed_identity"
            candidate["test_inventory"][field].sort()
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_policy(candidate)
        candidate = policy_fixture(); candidate["test_inventory"]["ordinary"][0] = candidate["test_inventory"]["ordinary"][1]
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_series_policy(candidate)
        candidate = policy_fixture(); candidate["test_inventory"]["n3_doctests"] = {}
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_series_policy(candidate)

    def test_empty_or_partial_mutation_review_fails_closed(self):
        policy = policy_fixture(); policy["mutation_inventories"] = {}
        self.assertEqual(MODULE.series_inventory_bindings(policy), {})
        with self.assertRaises(MODULE.Incomplete): MODULE.series_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {"library": {"count": 1, "sha256": "1" * 64}}
        with self.assertRaises(MODULE.Incomplete): MODULE.series_inventory_bindings(policy)
        policy["mutation_inventories"]["series"] = {"count": 1, "sha256": "2" * 64}
        self.assertEqual(set(MODULE.series_inventory_bindings(policy, required=True)), {"library", "series"})
        policy["mutation_inventories"]["series"]["count"] = 0
        with self.assertRaises(MODULE.Incomplete): MODULE.series_inventory_bindings(policy, required=True)

    def test_mutation_empty_review_and_five_gib_preflight_precede_session(self):
        args = SimpleNamespace(lane="mutation", subject="all", output_dir=Path("/not-created/owned"))
        held_policy = policy_fixture(); held_policy["mutation_inventories"] = {}
        with patch.object(MODULE, "read_series_policy", return_value=held_policy), patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaises(MODULE.Incomplete): MODULE.execute_series_profile(args, {})
            session.assert_not_called()
        args.lane = "coverage"
        with patch.object(MODULE, "read_series_policy", return_value=policy_fixture()), patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5 * 1024**3 - 1)), patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "five GiB"): MODULE.execute_series_profile(args, {})
            session.assert_not_called()

    def test_profile_route_is_explicit_and_legacy_default_is_preserved(self):
        args = SimpleNamespace(profile="series")
        with patch.object(MODULE, "execute_series_profile") as selected, patch.object(MODULE, "read_policy") as legacy:
            MODULE.execute(args, {})
            selected.assert_called_once(); legacy.assert_not_called()
        observed = []
        def capture(args, report): observed.append(args.profile)
        with patch.object(MODULE, "execute", side_effect=capture), patch("sys.stdout", new_callable=io.StringIO):
            MODULE.main(["--lane", "coverage", "--vendor-dir", "/not-used", "--output-dir", "/not-created/legacy"])
            MODULE.main(["--profile", "series", "--lane", "coverage", "--vendor-dir", "/not-used", "--output-dir", "/not-created/series"])
        self.assertEqual(observed, ["legacy", "series"])

    def test_selected_library_keeps_the_full_series_custody_scope(self):
        class ReachedCustodian(Exception):
            pass
        observed = []
        def capture(root, args, report, scope, policy, inputs):
            observed.append((args.lane, scope.select("all"), inputs))
            raise ReachedCustodian()
        args = SimpleNamespace(lane="enumerate", subject="library", output_dir=Path("/not-created/owned"))
        with patch.object(MODULE, "read_series_policy", return_value=policy_fixture()), patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5 * 1024**3)), patch.object(MODULE.COMMON, "Session", side_effect=capture):
            with self.assertRaises(ReachedCustodian): MODULE.execute_series_profile(args, {})
        self.assertEqual(args.lane, "enumerate")
        self.assertEqual(observed[0][0], "mutation")
        self.assertEqual(observed[0][1], {"library": "crates/nevrotick-engine/src/lib.rs", "series": "crates/nevrotick-engine/src/series.rs"})
        self.assertIn("scripts/tests/test_series_analysis.py", observed[0][2])

    def test_mutation_order_preserves_selected_owners_and_coverage(self):
        """Synthetic routing only; no Cargo or product execution is represented."""
        policy = policy_fixture()
        bindings = {
            "library": {"count": 1, "sha256": "1" * 64},
            "series": {"count": 1, "sha256": "2" * 64}}
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        observed, finished = [], []
        fake = SimpleNamespace(product_identity=identity, source=ROOT,
                               work=Path("/unused/not-created"), tools={},
                               finish=lambda: finished.append(True))
        for subject in ("all", "library", "series"):
            args = SimpleNamespace(lane="all", subject=subject,
                                   output_dir=Path("/unused/not-created/result"))
            def capture(_scope, selected, _policy, _work, _tools, _run, _report, lane):
                observed.append((subject, lane, list(selected)))
            with (patch.object(MODULE, "read_series_policy", return_value=policy),
                  patch.object(MODULE, "series_inventory_bindings", return_value=bindings),
                  patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5 * 1024**3 + 1)),
                  patch.object(MODULE.COMMON, "Session", return_value=fake),
                  patch.object(MODULE, "contracts") as contracts,
                  patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=capture)):
                MODULE.execute_series_profile(args, {})
            contracts.assert_called_once()
        self.assertEqual(observed, [
            ("all", "coverage", ["library", "series"]),
            ("all", "mutation", ["series", "library"]),
            ("library", "coverage", ["library"]),
            ("library", "mutation", ["library"]),
            ("series", "coverage", ["series"]),
            ("series", "mutation", ["series"])])
        self.assertEqual(finished, [True, True, True])

    def test_bounded_baseline_reads_parse_and_hash_the_same_bytes(self):
        with tempfile.TemporaryDirectory() as td:
            work = Path(td); out = work / "mutation/series/mutants.out"; (out / "log").mkdir(parents=True)
            log = out / "log/baseline.log"; raw = literal_execution(ORDINARY_NAMES + DOCTEST_NAMES).encode(); log.write_bytes(raw)
            (out / "outcomes.json").write_text(json.dumps({"outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log"}]}))
            original = MODULE.COMMON.read_analysis_bytes
            with (patch.object(MODULE.COMMON, "read_analysis_bytes", wraps=original) as reads,
                  patch.object(MODULE, "series_checkpoint_assertion_sites", return_value={})):
                observed = MODULE.validate_series_baseline(work, "series", policy_fixture())
            self.assertEqual(observed["tests_executed"], 163)
            self.assertEqual(observed["log_sha256"], hashlib.sha256(raw).hexdigest())
            self.assertEqual(sum(call.args[0] == log for call in reads.call_args_list), 1)
            for relative in ["../escape.log", "/tmp/escape.log"]:
                (out / "outcomes.json").write_text(json.dumps({"outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": relative}]}))
                with self.assertRaises(MODULE.Incomplete): MODULE.validate_series_baseline(work, "series", policy_fixture())

    def test_actual_baseline_rejects_marked_child_harness_failure_as_caught(self):
        with tempfile.TemporaryDirectory() as td:
            work = Path(td); out = work / "mutation/series/mutants.out"; (out / "log").mkdir(parents=True)
            (out / "log/baseline.log").write_text(literal_execution(ORDINARY_NAMES + DOCTEST_NAMES))
            log = out / "log/child.log"
            rows = [{"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log"},
                    {"scenario": {"Mutant": {"name": "selected-lab-fault"}},
                     "summary": "CaughtMutant", "log_path": "log/child.log"}]
            (out / "outcomes.json").write_text(json.dumps({"outcomes": rows}))
            healthy = b"named child reached its independent semantic assertion\n"
            log.write_bytes(healthy)
            with patch.object(MODULE, "series_checkpoint_assertion_sites", return_value={}):
                result = MODULE.validate_series_baseline(work, "series", policy_fixture())
            self.assertEqual(result["caught_cause_admission"]["checked_caught_logs"],
                             {"selected-lab-fault": hashlib.sha256(healthy).hexdigest()})
            log.write_bytes(healthy + b"N3_HARNESS_INCOMPLETE\nfixture failure is not a behavioral kill\n")
            with (patch.object(MODULE, "series_checkpoint_assertion_sites", return_value={}),
                  self.assertRaisesRegex(MODULE.Incomplete, "harness failure")):
                MODULE.validate_series_baseline(work, "series", policy_fixture())
            log.unlink(); log.symlink_to(out / "log/baseline.log")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_baseline(work, "series", policy_fixture())
            rows[1]["scenario"]["Mutant"]["name"] = "x" * 4097
            with self.assertRaisesRegex(MODULE.Incomplete, "identity"):
                MODULE.validate_series_caught_causes(out, {"outcomes": rows})

    def test_checkpoint_map_binds_static_product_facts_and_direct_assertions(self):
        mapping = MODULE.series_checkpoint_assertion_sites(ROOT, policy_fixture())
        final = "series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions"
        direct = "series::tests::current_checkpoint_observes_new_controls_before_expired_deadline"
        self.assertEqual(set(mapping["tests"]), {final, direct})
        facts = mapping["tests"][final]
        self.assertTrue(all(site["kind"] == "static_checkpoint_fact" for site in facts))
        self.assertIn("final series summary ignored its serialized current control", {site["marker"] for site in facts})
        self.assertFalse(any("short native response" in site["marker"] or "acknowledged" in site["marker"] for site in facts))
        self.assertTrue(all(site["kind"] == "panic_assertion" for site in mapping["tests"][direct]))
        self.assertEqual(mapping["bridge"], {
            "source": "crates/nevrotick-engine/tests/private/series_process.rs", "line": 429, "column": 9,
            "marker": "N3 exact child semantic failure assertion"})
        self.assertEqual(len(mapping["tests"][direct]), 4)
        self.assertIn("fresh checkpoint control rejected its first generation change",
                      {site["marker"] for site in mapping["tests"][direct]})
        policy = policy_fixture(); policy["inputs_sha256"][MODULE.SERIES_PROCESS_SOURCE] = "a" * 64
        with self.assertRaisesRegex(MODULE.Incomplete, "admitted bytes"):
            MODULE.series_checkpoint_assertion_sites(ROOT, policy)

    @staticmethod
    def checkpoint_failure(test, site, *, fact=None):
        """Synthetic failed-test section with literal source coordinates, no native claim."""
        prefix = "" if fact is None else "N3_SEMANTIC_EVIDENCE independent fact: " + fact + "\n"
        return ("running 1 test\ntest " + test + " ... FAILED\n\nfailures:\n\n---- " + test
                + " stdout ----\n" + prefix + "thread '" + test
                + "' (7) panicked at " + site["source"] + ":" + str(site["line"]) + ":"
                + str(site["column"]) + ":\n" + site["marker"] + "\n\nfailures:\n\n    " + test
                + "\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")

    def test_checkpoint_caught_requires_source_fact_bridge_or_direct_assertion(self):
        mapping = MODULE.series_checkpoint_assertion_sites(ROOT, policy_fixture())
        final = "series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions"
        direct = "series::tests::current_checkpoint_observes_new_controls_before_expired_deadline"
        fact = "final series summary ignored its serialized current control"
        healthy = self.checkpoint_failure(final, mapping["bridge"], fact=fact)
        causes = MODULE.series_checkpoint_caught_causes(healthy, mapping)
        self.assertEqual(len(causes), 1)
        self.assertEqual((causes[0]["owner"], causes[0]["line"]), ("final_checkpoint_case", 1981))
        for site in mapping["tests"][direct]:
            self.assertEqual(len(MODULE.series_checkpoint_caught_causes(self.checkpoint_failure(direct, site), mapping)), 1)
        for bad in [healthy.replace(fact, "checkpoint worker panicked"),
                    healthy.replace("N3_SEMANTIC_EVIDENCE independent fact: " + fact + "\n", ""),
                    healthy.replace(":429:9:", ":423:9:"),
                    healthy.replace("N3 exact child semantic failure assertion", "N3 series owner noncompletion assertion"),
                    healthy.replace("' (7) panicked", "-foreign' (7) panicked"),
                    healthy.replace("series_process.rs:429", "foreign.rs:429"),
                    healthy[:healthy.rindex("\n\nfailures:")],
                    healthy[:healthy.rindex("test result:")],
                    healthy.replace("0 passed; 1 failed", "1 passed; 1 failed"),
                    healthy.replace("running 1 test", "running 2 tests"),
                    healthy + "test " + final + " ... FAILED\n",
                    healthy + "running 1 test\ntest " + final + " ... FAILED\n",
                    healthy + "error: test failed; process did not exit successfully (signal: 9, SIGKILL: kill)\n",
                    healthy.replace("\n\nfailures:", "\nError: Elapsed(())\n\nfailures:"),
                    healthy.replace("\n\nfailures:", '\nError: Custom { kind: Other, error: "checkpoint worker panicked" }\n\nfailures:')]:
            with self.subTest(log=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.series_checkpoint_caught_causes(bad, mapping)
        with self.assertRaises(MODULE.Incomplete): MODULE.series_checkpoint_caught_causes(healthy, None)
        with self.assertRaisesRegex(MODULE.Incomplete, "complete semantic section"):
            MODULE.series_checkpoint_caught_causes("test " + final + " ... FAILED\n", mapping)

    def test_checkpoint_harness_failure_refuses_even_beside_a_genuine_primary(self):
        mapping = MODULE.series_checkpoint_assertion_sites(ROOT, policy_fixture())
        direct = "series::tests::current_checkpoint_observes_new_controls_before_expired_deadline"
        text = self.checkpoint_failure(direct, mapping["tests"][direct][-1])
        with tempfile.TemporaryDirectory() as td:
            directory = Path(td); log = directory / "mixed.log"
            rows = [{"scenario": {"Mutant": {"name": "synthetic-early-stop"}},
                     "summary": "CaughtMutant", "log_path": "mixed.log"}]
            log.write_text(text)
            result = MODULE.validate_series_caught_causes(directory, {"outcomes": rows}, mapping)
            self.assertEqual(len(result["checkpoint_semantic_causes"]["synthetic-early-stop"]), 1)
            log.write_text(text + "N3_HARNESS_INCOMPLETE\nError: Elapsed(())\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "harness failure"):
                MODULE.validate_series_caught_causes(directory, {"outcomes": rows}, mapping)

    def test_source_candidate_and_manifest_owners_are_actual_boundaries(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td); policy = policy_fixture()
            policy["mutation_equivalences"] = []
            for name in MODULE.SERIES_CURRENT_BOUND_INPUTS:
                path = root / name; path.parent.mkdir(parents=True, exist_ok=True); path.write_text("fixture:" + name + "\n")
            manifest = root / "policy/verification-manifest.toml"
            manifest.write_text('critical_files = ["crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/series.rs"]\n[[security_modules]]\npackage = "nevrotick-engine"\nfiles = ["crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/series.rs"]\n')
            identity = {"fixture-" + str(i): "1" * 64 for i in range(421)}
            policy["candidate_sha256"] = digest(identity)
            policy["inputs_sha256"] = {n: hashlib.sha256((root / n).read_bytes()).hexdigest() for n in MODULE.SERIES_CURRENT_BOUND_INPUTS}
            target = root / MODULE.SERIES_POLICY; target.write_text(json.dumps(policy))
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.read_series_policy(root), policy)
                changed = root / "scripts/verify-engine-analysis.py"; changed.write_text("changed protected helper")
                with self.assertRaisesRegex(MODULE.Incomplete, "bytes changed"): MODULE.read_series_policy(root)
                changed.write_text("fixture:scripts/verify-engine-analysis.py\n")
                identity["extra"] = "1" * 64
                with self.assertRaisesRegex(MODULE.Incomplete, "complete current product"): MODULE.read_series_policy(root)
                identity.pop("extra")
                manifest.write_text('critical_files = ["crates/nevrotick-engine/src/lib.rs"]\n[[security_modules]]\npackage = "nevrotick-engine"\nfiles = ["crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/series.rs"]\n')
                policy["inputs_sha256"]["policy/verification-manifest.toml"] = hashlib.sha256(manifest.read_bytes()).hexdigest()
                target.write_text(json.dumps(policy))
                with self.assertRaisesRegex(MODULE.Incomplete, "critical/security registration"): MODULE.read_series_policy(root)

    @staticmethod
    def current_fallback_entries(candidate="1" * 64, *, owner=None):
        """Literal current native identities; no semantic or execution oracle."""
        filename = "crates/nevrotick-engine/src/lib.rs"
        proof_path = "tasks/engine-analysis-equivalences.md"
        proof_digest = hashlib.sha256((ROOT / proof_path).read_bytes()).hexdigest()
        entries = []
        for suffix, replacement in (("SUB", "-="), ("MUL", "*=")):
            identifier = "ENGINE-SERIES-COMMIT-DURATION-FALLBACK-" + suffix
            entries.append({
                "id": identifier, "candidate_sha256": candidate,
                "source_sha256": "f14ab14174eab86abdd14b8b70a567e88deeda4062520ed13323e7c4a8d11622",
                "mutant": {
                    "name": f"{filename}:1350:23: replace += with {replacement} in commit_reply",
                    "package": "nevrotick-engine", "file": filename,
                    "function": {"function_name": "commit_reply", "return_type": "",
                                 "span": {"start": {"line": 1330, "column": 1},
                                          "end": {"line": 1379, "column": 2}}},
                    "span": {"start": {"line": 1350, "column": 23},
                             "end": {"line": 1350, "column": 25}},
                    "replacement": replacement, "genre": "BinaryOperator"},
                "requirement": "CHECK-6/7/8/11/14/25/27",
                "rationale": "Admission control for an exact independently reviewed source/descriptor pair; no synthetic execution credit.",
                "proof": {"path": proof_path, "sha256": proof_digest, "section": identifier,
                          "reviewer": "independent source review; synthetic observer control",
                          "verdict": "independently-justified-equivalent"}})
        filename = "crates/nevrotick-engine/src/series.rs"
        for suffix, function, line, start, end in (
                ("ADMITTED", "admitted_sample_count", 469, 466, 472),
                ("ELIGIBLE", "eligible_sample_count", 476, 473, 479)):
            identifier = "ENGINE-SERIES-UNAVAILABLE-" + suffix + "-ZERO"
            entries.append({
                "id": identifier, "candidate_sha256": candidate,
                "source_sha256": "174ab3b4538356c5134cb850baac19640baf27c1e14ad3c12f953948549a43f7",
                "mutant": {
                    "name": f"{filename}:{line}:9: replace SeriesReport::{function} -> usize with 0",
                    "package": "nevrotick-engine", "file": filename,
                    "function": {"function_name": "SeriesReport::" + function, "return_type": "-> usize",
                                 "span": {"start": {"line": start, "column": 5},
                                          "end": {"line": end, "column": 6}}},
                    "span": {"start": {"line": line, "column": 9},
                             "end": {"line": line + 2, "column": 10}},
                    "replacement": "0", "genre": "FnValue"},
                "requirement": "CHECK-6/7/8/11/14/25/27; unavailable N3 sample admission",
                "rationale": "Exact independently reviewed unavailable-only getter domain; no synthetic execution credit.",
                "proof": {"path": proof_path, "sha256": proof_digest, "section": identifier,
                          "reviewer": "temporal_control_review; synthetic observer control",
                          "verdict": "independently-justified-equivalent"}})
        if owner is not None:
            selected = "crates/nevrotick-engine/src/" + ("lib.rs" if owner == "library" else "series.rs")
            return [entry for entry in entries if entry["mutant"]["file"] == selected]
        return entries

    def test_series_equivalence_opt_in_is_only_the_complete_current_cohort(self):
        entries = self.current_fallback_entries()
        MODULE.validate_series_equivalence_scope([])
        MODULE.validate_series_equivalence_scope(entries)
        self.assertEqual([digest(entry["mutant"]) for entry in entries], [
            "2ab92195ec79b1c71bae647479ea1f5a82b5e6e0a0bfde2cbc3da0bd9067af1d",
            "5d7961f5476b58579bee819ba1a895a6da90fc0cfa4a10ddcb389bf82ce982f9",
            "f4b3fae4f21b063861301bae063b9115737894d7916b6583f896cf63ca5ec531",
            "cc59ea98db653c91db4af372b1f9a3db26c081098e00eb62588d1fb33f097298"])
        for owner in ("library", "series"):
            with self.subTest(partial=owner), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_equivalence_scope(self.current_fallback_entries(owner=owner))
        for index in range(4):
            bad = copy.deepcopy(entries); bad[index]["source_sha256"] = "a" * 64
            with self.subTest(source_owner=index), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_equivalence_scope(bad)
        swapped = copy.deepcopy(entries)
        swapped[2]["mutant"], swapped[3]["mutant"] = swapped[3]["mutant"], swapped[2]["mutant"]
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_series_equivalence_scope(swapped)
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_equivalence_scope(entries)
        for change in ("list_type", "single", "duplicate", "legacy", "extra", "source", "old_source",
                       "line", "span", "operator", "package", "file", "function", "return_type", "genre",
                       "proof_path", "proof_section", "verdict", "proof_hash", "reviewer", "candidate",
                       "rationale", "unknown_field"):
            bad = copy.deepcopy(entries)
            if change == "list_type": bad = {}
            elif change == "single": bad.pop()
            elif change == "duplicate": bad[1] = copy.deepcopy(bad[0])
            elif change == "legacy": bad[0]["id"] = "ENGINE-COMMIT-DURATION-FALLBACK-SUB"
            elif change == "extra": bad.append(copy.deepcopy(bad[0]))
            elif change == "source": bad[0]["source_sha256"] = "a" * 64
            elif change == "old_source": bad[0]["source_sha256"] = "6f3c6eb2f491de7f27a89f8fba84556e818874a9c875643b08081206f5dd4d46"
            elif change == "line": bad[0]["mutant"]["span"]["start"]["line"] = 1340
            elif change == "span": bad[0]["mutant"]["function"]["span"]["end"]["line"] += 1
            elif change == "operator": bad[0]["mutant"]["replacement"] = "+="
            elif change == "package": bad[0]["mutant"]["package"] = "time-core"
            elif change == "file": bad[0]["mutant"]["file"] = "crates/nevrotick-engine/src/series.rs"
            elif change == "function": bad[0]["mutant"]["function"]["function_name"] = "process_reply"
            elif change == "return_type": bad[0]["mutant"]["function"]["return_type"] = "-> ()"
            elif change == "genre": bad[0]["mutant"]["genre"] = "FnValue"
            elif change == "proof_path": bad[0]["proof"]["path"] = "tasks/foreign.md"
            elif change == "proof_section": bad[0]["proof"]["section"] = "ENGINE-COMMIT-DURATION-FALLBACK-SUB"
            elif change == "verdict": bad[0]["proof"]["verdict"] = "unreviewed"
            elif change == "proof_hash": bad[0]["proof"]["sha256"] = "invalid"
            elif change == "reviewer": bad[0]["proof"]["reviewer"] = " "
            elif change == "candidate": bad[0]["candidate_sha256"] = "invalid"
            elif change == "rationale": bad[0]["rationale"] = " "
            else: bad[0]["unknown"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_series_equivalence_scope(bad)
        policy = policy_fixture()
        policy["mutation_equivalences"] = self.current_fallback_entries(policy["candidate_sha256"])
        MODULE.validate_series_policy(policy)
        policy["mutation_equivalences"][0]["candidate_sha256"] = "a" * 64
        with self.assertRaisesRegex(MODULE.Incomplete, "complete policy candidate"):
            MODULE.validate_series_policy(policy)
        policy["mutation_equivalences"] = []
        MODULE.validate_series_policy(policy)
        policy.pop("mutation_equivalences")
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_series_policy(policy)

    def test_series_equivalence_common_custody_binds_actual_source_candidate_and_review(self):
        """Copy actual admitted bytes; exercise custody without running a mutant."""
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            _, current = MODULE.PRODUCT.admit_product(ROOT)
            proof_name = "tasks/engine-analysis-equivalences.md"
            for name in sorted(set(current) | set(MODULE.SERIES_CURRENT_BOUND_INPUTS) | {MODULE.SERIES_POLICY, proof_name}):
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, target)
            _, identity = MODULE.PRODUCT.admit_product(root)
            entries = self.current_fallback_entries(digest(identity))
            policy = policy_fixture(); policy["candidate_sha256"] = digest(identity)
            policy["mutation_equivalences"] = entries
            # Independent literal records must match the current copied source;
            # this custody fixture supplies no mutant execution or kill credit.
            for entry in entries:
                self.assertEqual(entry["source_sha256"], hashlib.sha256((root / entry["mutant"]["file"]).read_bytes()).hexdigest())
            MODULE.validate_series_equivalence_scope(entries)
            self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SERIES_SCOPE, policy), entries)
            bad = copy.deepcopy(policy); bad["mutation_equivalences"][0]["candidate_sha256"] = "a" * 64
            with self.assertRaisesRegex(MODULE.Incomplete, "complete candidate"):
                MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SERIES_SCOPE, bad)
            proof = root / proof_name; proof_bytes = proof.read_bytes()
            proof.write_bytes(proof_bytes + b"\nChanged review bytes.\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SERIES_SCOPE, policy)
            proof.write_bytes(proof_bytes.replace(b"## ENGINE-SERIES-COMMIT-DURATION-FALLBACK-SUB\n",
                                                b"## Changed section\n"))
            for entry in entries: entry["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
            with self.assertRaisesRegex(MODULE.Incomplete, "exact section"):
                MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SERIES_SCOPE, policy)
            proof.write_bytes(proof_bytes)
            for entry in entries: entry["proof"]["sha256"] = hashlib.sha256(proof_bytes).hexdigest()
            source = root / "crates/nevrotick-engine/src/lib.rs"
            source.write_bytes(source.read_bytes() + b"\n// Changed source custody.\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "source or complete candidate"):
                MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SERIES_SCOPE, policy)

    def test_series_equivalence_native_observer_retains_misses_kills_and_timeouts(self):
        """Synthetic native records test accounting only; no execution credit."""
        for owner in ("library", "series"):
            entries = self.current_fallback_entries(owner=owner)
            filename = "crates/nevrotick-engine/src/" + ("lib.rs" if owner == "library" else "series.rs")
            control = {"name": filename + ":53:43: replace + with -", "package": "nevrotick-engine",
                       "file": filename, "function": None, "genre": "BinaryOperator", "replacement": "-",
                       "span": {"start": {"line": 53, "column": 43}, "end": {"line": 53, "column": 44}}}
            expected = {item["name"]: item for item in [control, *(entry["mutant"] for entry in entries)]}
            def phases(summary):
                result = []
                for phase in (["Build"] if summary == "Unviable" else ["Build", "Test"]):
                    failed = summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test")
                    result.append({"phase": phase, "duration": 0.01,
                                   "process_status": {"Failure": 101} if failed else "Success",
                                   "argv": ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                                            "--verbose", "--package=nevrotick-engine@0.0.0", "--message-format=json", "--frozen"]})
                return result
            outcomes = [{"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success")},
                        {"scenario": {"Mutant": control}, "summary": "CaughtMutant", "phase_results": phases("CaughtMutant")}]
            outcomes.extend({"scenario": {"Mutant": entry["mutant"]}, "summary": "MissedMutant",
                             "phase_results": phases("MissedMutant")} for entry in entries)
            report = {"cargo_mutants_version": "27.1.0", "total_mutants": 3, "caught": 1, "missed": 2,
                      "unviable": 0, "timeout": 0, "success": 0, "outcomes": outcomes}
            def observe(data, native=2, cohort=entries):
                return MODULE.COMMON.mutation_counts(
                    MODULE.SERIES_SCOPE, data, expected, filename, native_exit_code=native,
                    equivalences=cohort, defer_gate=True)
            healthy = observe(report)
            self.assertEqual((healthy["killed"], healthy["survived"], healthy["excluded"],
                              healthy["raw_denominator"], healthy["adjusted_denominator"]), (1, 2, 2, 3, 1))
            self.assertTrue(healthy["gate_passed"])
            self.assertFalse(observe(report, cohort=[])["gate_passed"])
            with self.assertRaisesRegex(MODULE.Incomplete, "native exit"): observe(report, native=0)
            for summary, key in (("CaughtMutant", "caught"), ("Unviable", "unviable"), ("Timeout", "timeout")):
                bad = copy.deepcopy(report); bad["outcomes"][2]["summary"] = summary
                bad["outcomes"][2]["phase_results"] = phases(summary)
                bad["missed"] -= 1; bad[key] += 1
                with self.subTest(summary=summary), self.assertRaises(MODULE.Incomplete): observe(bad)
            bad = copy.deepcopy(report); bad["outcomes"].pop()
            with self.assertRaisesRegex(MODULE.Incomplete, "complete frozen enumeration"): observe(bad)
            bad_entries = copy.deepcopy(entries); bad_entries[0]["mutant"]["span"]["start"]["line"] = 1340
            with self.assertRaisesRegex(MODULE.Incomplete, "not an actual missed mutant"):
                observe(report, cohort=bad_entries)

    def test_external_contracts_have_specific_owned_causes_and_inert_legal_control(self):
        examples = MODULE.series_contract_examples()
        self.assertEqual(len(examples), 14)
        expected = {"E0451": 4, "E0616": 4, "E0277": 4, "E0382": 1}
        observed = {code: sum(x["code"] == code for x in examples.values()) for code in expected}
        self.assertEqual(observed, expected)
        legal = examples["series_legal"]
        self.assertIsNone(legal["code"])
        self.assertIn("SeriesConfig::new", legal["source"])
        self.assertNotIn("prepare_series(", legal["source"])
        self.assertNotIn("execute_series(", legal["source"])
        for name, specimen in examples.items():
            self.assertEqual(specimen["line"], 4)
            if specimen["code"]: self.assertTrue(specimen["tokens"])
        consumed = examples["series_prepared_consumed"]
        self.assertEqual(consumed["code"], "E0382")
        self.assertEqual(consumed["source"].splitlines()[3].count("execute_series("), 2)
        self.assertEqual(consumed["tokens"], ["plan", "moved"])


if __name__ == "__main__":
    unittest.main()
