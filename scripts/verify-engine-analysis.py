#!/usr/bin/env python3
"""Bounded host engine coverage and mutation; enumeration grants no qualification.

GPL-2.0-only. Reuse the admitted shared analysis namespace, tools, source custody
and native outcome accounting. No acquisition, new graph or profile exception.
"""
from __future__ import annotations

import argparse
import hashlib
from collections import Counter
import importlib.util
import json
import math
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib
from typing import NoReturn

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("engine_analysis_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
PRODUCT = COMMON.PRODUCT
Incomplete = COMMON.Incomplete
POLICY = "policy/engine-analysis.json"
WRAPPER = "scripts/verify-engine-analysis.py"
TESTS = "scripts/tests/test_verify_engine_analysis.py"
SUBJECTS = {
    "library": "crates/nevrotick-engine/src/lib.rs",
    "cli": "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
    "measurement": "crates/nevrotick-engine/src/measurement.rs",
    "relative": "crates/nevrotick-engine/src/relative.rs",
}
SCOPE = COMMON.Scope("nevrotick-engine", tuple(SUBJECTS.items()), POLICY)
CONTRACT_SCOPE = COMMON.Scope(
    "nevrotick-engine", (("library", "crates/nevrotick-engine/src/lib.rs"),), POLICY)
FIXED_CONTRACT_POLICY = {
    "schema_version": 1,
    "admission_status": "admitted-engine-external-contracts-only",
    "package": "nevrotick-engine", "source_file_count": 168,
    "critical_files": ["crates/nevrotick-engine/src/lib.rs"],
    "tool_admission_profile": "stable-core",
    "contracts_wall_seconds": 300, "jobs": 6, "test_threads": 1,
    "mutation_equivalences": [], "mutation_error_values": {},
}
BOUND_INPUTS = (
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml",
    *SUBJECTS.values(), "crates/nevrotick-engine/tests/runtime_boundary.rs",
    "crates/nevrotick-engine/tests/prepare_release.rs", "crates/nevrotick-engine/tests/prepare_entered.rs",
    "crates/nevrotick-engine/tests/execute_entered.rs",
    "crates/nevrotick-engine/tests/private/processing_guards.rs",
    "crates/nevrotick-engine/tests/private/argument_guards.rs",
    "crates/nevrotick-engine/tests/private/measurement.rs",
    "crates/nevrotick-engine/tests/private/relative.rs",
    "scripts/portable-analysis.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/prepare-product.py", "policy/analysis-tools.json", "policy/verification-manifest.toml",
)
REQUIRED_TESTS = frozenset({
    "processing_guard_tests::processing_uses_fresh_deadline_and_current_control",
    "processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct",
    "processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries",
    "processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays",
    "processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap",
    "processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections",
    "processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions",
    "processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor",
    "processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes",
    "processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port",
    "processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct",
    "processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history",
    "processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four",
    "processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize",
    "processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor",
    "processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error",
    "processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice",
    "processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection",
    "processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates",
    "processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once",
    "processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals",
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
    "measurement::tests::decimal_remainders_change_the_exact_nominal_phase",
    "measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values",
    "measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association",
    "measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result",
    "measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds",
    "measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection",
    "measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt",
    "measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint",
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
    "prepare_outside_runtime_owns_and_releases_standard_sockets",
    "prepare_rejects_entered_runtime_before_native_startup",
    "execute_rejects_entered_runtime_without_nested_runtime_creation",
    "cli_worker_binary_preserves_literal_transport_contracts",
    "cli_worker_binary_rejects_arguments_and_observes_sink_failures",
    "argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure",
    "argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows",
    "argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles",
    "argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries",
})
FIXED_POLICY = {
    "schema_version": 1, "package": "nevrotick-engine",
    "critical_files": list(SUBJECTS.values()),
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "maximum_mutants": 512, "maximum_listed_per_file": 4096, "maximum_listed_total": 8192,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240,
    "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
    "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
    "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
    "tool_admission_profile": "full", "mutation_equivalences": [],
}

# N3a has an explicit role. Legacy constants, admission and fixtures retain their
# historical scope; selecting this role never imports their scores or inventory.
SERIES_POLICY = "policy/series-analysis.json"
SERIES_TESTS = "scripts/tests/test_series_analysis.py"
SERIES_SUBJECTS = {
    "library": "crates/nevrotick-engine/src/lib.rs",
    "series": "crates/nevrotick-engine/src/series.rs",
}
SERIES_SCOPE = COMMON.Scope("nevrotick-engine", tuple(SERIES_SUBJECTS.items()), SERIES_POLICY)
SERIES_BOUND_INPUTS = tuple(dict.fromkeys((
    *BOUND_INPUTS, WRAPPER, SERIES_TESTS, "rust-toolchain.toml", ".cargo/config.toml",
    ".cargo/mutants.toml", "crates/nevrotick-engine/src/series.rs",
    "crates/nevrotick-engine/tests/private/series.rs", "crates/nevrotick-engine/tests/series.rs",
)))
# The frozen Q1 role below retains SERIES_BOUND_INPUTS. Only the current N3
# continuation adds the child bridge used by its new semantic-cause admission.
SERIES_PROCESS_SOURCE = "crates/nevrotick-engine/tests/private/series_process.rs"
SERIES_CURRENT_BOUND_INPUTS = (*SERIES_BOUND_INPUTS, SERIES_PROCESS_SOURCE, TESTS)
SERIES_REQUIRED_TESTS = frozenset({
    "series::tests::series_process::semantic_evidence_accepts_only_exact_named_admitted_sites",
    "series_process::semantic_evidence_accepts_only_exact_named_admitted_sites",
    "series::tests::all_series_debug_outputs_are_exact_fixed_redactions",
    "series::tests::entropy_failure_and_entered_runtime_precede_all_acquisition_work",
    "series::tests::selected_native_datagram_after_retirement_is_charged_without_publication",
    "series::tests::setup_and_registration_failures_preserve_actual_prior_completion",
    "series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor",
    "series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor",
    "series::tests::actual_publication_rechecks_control_and_deadline_after_staging",
    "series::tests::deadlines_and_controls_do_not_reset_the_series_origin",
    "series::tests::genuine_terminal_report_admits_only_the_three_position_domain",
    "series::tests::partial_report_preserves_primary_and_both_cleanup_causes",
    "series::tests::post_commit_control_preserves_header_and_measurement_refusal",
    "series::tests::raw_config_and_tokens_have_mandatory_bounded_admission",
    "series::tests::receiver_rejects_duplicate_foreign_and_reused_identity_atomically",
    "series::tests::transfer_control_and_irreversible_retirement_are_distinct",
    "invalidation_preserves_first_completion_and_marks_third_not_started",
    "ipv6_real_requests_retain_distinct_identity_and_one_exact_origin",
    "middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt",
    "stop_preserves_first_completion_and_marks_third_not_started",
    "three_real_requests_retain_distinct_identity_and_one_exact_origin",
    "total_deadline_preserves_first_completion_and_prevents_third_send",
    "unavailable_measurements_still_consume_once_after_rejected_neighbors",
})
SERIES_CHECKPOINT_REQUIRED_TESTS = frozenset({
    "series::tests::current_checkpoint_observes_new_controls_before_expired_deadline",
    "series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions",
})
SERIES_N3_DOCTESTS = {
    "crates/nevrotick-engine/src/series.rs - series::PreparedSeries (line 209) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::PreparedSeries (line 216) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::SeriesAttempt (line 336) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 81) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 87) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 91) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::SeriesConfig::new (line 111)": "legal_execution",
    "crates/nevrotick-engine/src/series.rs - series::SeriesReport (line 400) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/series.rs - series::execute_series (line 1071) - compile": "legal_no_run"
}
SERIES_INVENTORY_DIGESTS = {
    "ordinary": "c21b82fd32cea0a3a3eefddc8f84d3b43e8591d03e2286d3445f4d334a7ae3c3",
    "doctests": "5eaf6bcb3df3807ecf447ef66d256abb85511048fbd3939da951c21df379e48a",
    "baseline": "b4a41aeb46444047904e96043d94e390ab8e34bf7c01a829f1adf55869267ca4"
}
SERIES_EQUIVALENCE_SOURCE = "f14ab14174eab86abdd14b8b70a567e88deeda4062520ed13323e7c4a8d11622"
SERIES_ZERO_ADMISSION_SOURCE = "174ab3b4538356c5134cb850baac19640baf27c1e14ad3c12f953948549a43f7"
SERIES_EQUIVALENCE_DESCRIPTORS = {
    "ENGINE-SERIES-COMMIT-DURATION-FALLBACK-SUB": "2ab92195ec79b1c71bae647479ea1f5a82b5e6e0a0bfde2cbc3da0bd9067af1d",
    "ENGINE-SERIES-COMMIT-DURATION-FALLBACK-MUL": "5d7961f5476b58579bee819ba1a895a6da90fc0cfa4a10ddcb389bf82ce982f9",
    "ENGINE-SERIES-UNAVAILABLE-ADMITTED-ZERO": "f4b3fae4f21b063861301bae063b9115737894d7916b6583f896cf63ca5ec531",
    "ENGINE-SERIES-UNAVAILABLE-ELIGIBLE-ZERO": "cc59ea98db653c91db4af372b1f9a3db26c081098e00eb62588d1fb33f097298"
}
SERIES_FIXED_POLICY = {
    "schema_version": 1, "admission_status": "admitted-n3a-series-source",
    "package": "nevrotick-engine", "source_file_count": 421,
    "critical_files": list(SERIES_SUBJECTS.values()),
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 512,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240, "contracts_wall_seconds": 300,
    "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
    "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
    "mutation_jobs": 8, "mutation_process_limit": 512, "mutation_message_format": "json",
    "minimum_free_bytes": 5 * 1024**3, "tool_admission_profile": "full",
}

# Q1 is a separate one-owner role. Historical policies and receipts are not
# rewritten or credited to it; the ordinary receipt fixes its complete inventory.
QUALITY_POLICY = "policy/quality-analysis.json"
QUALITY_SUBJECTS = {"quality": "crates/nevrotick-engine/src/quality.rs"}
QUALITY_SCOPE = COMMON.Scope("nevrotick-engine", tuple(QUALITY_SUBJECTS.items()), QUALITY_POLICY)
QUALITY_BOUND_INPUTS = tuple(dict.fromkeys((
    *SERIES_BOUND_INPUTS, TESTS, *QUALITY_SUBJECTS.values(),
    "crates/nevrotick-engine/src/quality/tests.rs", "crates/nevrotick-engine/tests/quality.rs",
    "crates/nevrotick-engine/PROVENANCE.md", "crates/nevrotick-engine/COPYING",
    "tasks/s0.1-chrony-ledger.json",
    "crates/time-domain/src/rational_seconds.rs", "tasks/engine-analysis-equivalences.md",
)))
QUALITY_CANDIDATE = "e8f6ead34f41429033a8d6c0ebd823bc693dae8caa645d53a8eafedf20e8727d"
QUALITY_SOURCE_PINS = {
    "crates/nevrotick-engine/src/quality.rs": "f3abb4b00c769b3a4fb655cb923df7626c4209cb792594a55f065449051e2135",
    "crates/nevrotick-engine/src/quality/tests.rs": "d2053446b6deca39df565e67249ca53c6a0c7aead04bb9096ddf84b3d989fcb0",
    "crates/nevrotick-engine/tests/quality.rs": "83a50ff2b783fde80431f5a46eae632a2fd6e774ffc4fec756317221daee9343",
    "crates/time-domain/src/rational_seconds.rs": "2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438",
}
QUALITY_INVENTORY_DIGESTS = {
    "ordinary": "c21b82fd32cea0a3a3eefddc8f84d3b43e8591d03e2286d3445f4d334a7ae3c3",
    "doctests": "5eaf6bcb3df3807ecf447ef66d256abb85511048fbd3939da951c21df379e48a",
    "baseline": "b4a41aeb46444047904e96043d94e390ab8e34bf7c01a829f1adf55869267ca4",
}
QUALITY_ZERO_EQUIVALENCE_ID = "ENGINE-QUALITY-INTERVAL-ZERO-SIGN"
QUALITY_ZERO_EQUIVALENCE_DESCRIPTOR = "ab870f10eb8cde026ababd980eb91338a2026e753395d4cd178bd50b8b6393ec"
QUALITY_REQUIRED_TESTS = frozenset({
    "quality::tests::computed_policy_boundaries_preserve_strict_and_inclusive_comparisons",
    "quality::tests::declared_getters_and_constructor_precedence_preserve_original_bits",
    "quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions",
    "quality::tests::evaluator_failure_precedence_preserves_original_observation",
    "quality::tests::finite_input_overflow_is_refused_at_the_first_computed_stage",
    "quality::tests::generated_garde_dispatch_covers_nested_and_computed_fields",
    "quality::tests::gradual_underflow_does_not_claim_exact_zero_uncertainty",
    "quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields",
    "quality::tests::negative_local_wall_interval_grows_declared_frequency_dispersion",
    "quality::tests::negative_server_interval_processing_limit_uses_absolute_magnitude",
    "quality::tests::original_measurement_refusal_and_redacted_diagnostics_are_preserved",
    "quality::tests::precision_clamps_have_exact_boundary_and_neighbor_values",
    "quality::tests::raw_round_trip_is_converted_once_before_cancellation",
    "quality::tests::raw_scalar_and_interval_helpers_preserve_full_admitted_domains",
    "quality::tests::refusal_families_keep_bounded_diagnostics_and_typed_sources",
    "quality::tests::signed_server_interval_and_unsigned_root_words_are_retained",
    "quality::tests::zero_raw_rtt_uses_combined_precision_floor",
    "absent_measurement_refuses_quality_with_actual_valid_neighbor",
    "actual_quality_limits_refuse_without_rewriting_completion_or_age",
    "actual_series_quality_retains_signed_intervals_source_and_original_age",
})
QUALITY_DOCTESTS = {
    "crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityAssumptions (line 389)": "legal_execution",
    "crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line 749) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line 755) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line 760) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/quality.rs - quality::DeclaredFrequencyInterval (line 255) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityAssumptions (line 401) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityAssumptions (line 405) - compile fail": "compile_fail",
    "crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityLimits (line 305) - compile fail": "compile_fail",
}
QUALITY_FIXED_POLICY = {
    **SERIES_FIXED_POLICY,
    "admission_status": "admitted-q1-conditional-quality-source",
    "source_file_count": 421, "critical_files": list(QUALITY_SUBJECTS.values()),
}
QUALITY_EVIDENCE_ROOT = Path.home() / ".local/share/nevrotick/dependency-refresh-20261106-v1"
QUALITY_ORDINARY_RECEIPT = Path.home() / ".local/share/nevrotick/series-final-checkpoint-20261108-v1/green-ordinary-v3/verification.json"
QUALITY_ORDINARY_SHA256 = "887732785dca641646e1c72adf1dcaf6af9054cd3566b33487d9e4ee1fee58b3"
QUALITY_ORDINARY_RECIPES = {
    "quality_ordinary_v2.py": "717cc1902e7c01194967309375d8a1660910276ba4045e7efbe81fe9af743a1b",
    "quality_ordinary_v1.py": "bb903f4bd09b6e59ccc956f4de582eb9b421fa9d49ba926920318fefdda0fc08",
    "relative_bridge_ordinary.py": "937709d9bc11054253b2e0cd9ad05357792f81bcb6e1ff88f997ac8f2a8292a9",
    "conditioned_fit_ordinary_v2.py": "3062bb0bb1869161e273ad4e80b238a4d7ef94661a21ffa4127341832a9356d6",
    "series_ordinary_v1.py": "d3ab16cc3e08a03f950808cdaeb570dc86ba4aefec4a1c6bf9c0810631e64cc2",
    "series_ordinary_v3.py": "17574af8bb9e4b41b3f55337531a82e2c0a2a275741f255b3b3cea226002e568",
}


def invalid_number(value: str) -> NoReturn:
    """Reject nonfinite tokens at every depth of policy and native report input."""
    raise Incomplete("Engine JSON contains a nonfinite number.")


def finite_number(value: str) -> float:
    """Retain finite native timing values without accepting exponent overflow."""
    number = float(value)
    if not math.isfinite(number):
        invalid_number(value)
    return number


def read_json(path: Path, *, maximum: int = 524288) -> dict:
    """Read bounded regular, nonlinked JSON; reject duplicates and nonfinite values."""
    if path != path.resolve(strict=True):
        raise Incomplete("Engine JSON input cannot use a linked path.")
    data = COMMON.ANALYSIS.read_once(path, maximum=maximum)
    try:
        value = json.loads(data, object_pairs_hook=COMMON.ANALYSIS.unique_object,
                           parse_float=finite_number, parse_constant=invalid_number)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Invalid bounded engine JSON input.") from error
    if not isinstance(value, dict):
        raise Incomplete("Engine JSON input must be an object.")
    return value


def inventory_bindings(policy: dict, *, required: bool = False) -> dict:
    """Admit either no review yet or complete exact nonempty per-file review pins."""
    bindings = policy.get("mutation_inventories")
    if not isinstance(bindings, dict):
        raise Incomplete("Engine mutation inventory bindings must be an object.")
    if not bindings and not required:
        return {}
    if set(bindings) != set(SUBJECTS):
        raise Incomplete("Campaign requires reviewed nonempty inventories for all four engine files.")
    for binding in bindings.values():
        if (not isinstance(binding, dict) or set(binding) != {"count", "sha256"}
                or type(binding["count"]) is not int or not 0 < binding["count"] <= policy["maximum_mutants"]
                or not isinstance(binding["sha256"], str)
                or not re.fullmatch(r"[a-f0-9]{64}", binding["sha256"])):
            raise Incomplete("Engine inventory review requires exact positive counts and SHA-256 pins.")
    return bindings


def validate_equivalence_scope(entries: object) -> None:
    """Admit the historical measurement cohort or the exact host fallback pair.

    Shared admission still binds each complete descriptor, candidate, source and
    independent-review artifact. This scope check supplies no semantic verdict.
    """
    owners = {
        "ENGINE-COHERENCE-LOWER-EQUALITY": ("measurement", "coherence_residual", "<=", None),
        "ENGINE-COHERENCE-UPPER-EQUALITY": ("measurement", "coherence_residual", ">=", None),
        "ENGINE-NOMINAL-ZERO-SIGN": ("measurement", "nominal_seconds", "<=", None),
        "ENGINE-COMMIT-DURATION-FALLBACK-SUB": (
            "library", "commit_reply", "-=",
            "7367d98456163b7ac5194b8423d289121a3b30bc1e60910bc67f0f3dfb294c39"),
        "ENGINE-COMMIT-DURATION-FALLBACK-MUL": (
            "library", "commit_reply", "*=",
            "640fe4b14f5d217b3662ca08f74bcfee0ad41eff9d9a613434fee7e0f15178d0"),
    }
    if type(entries) is not list:
        raise Incomplete("Engine equivalences require an exact list.")
    if not entries:
        return
    if (any(type(entry) is not dict for entry in entries)
            or any(type(entry.get("id")) is not str for entry in entries)):
        raise Incomplete("Engine equivalences require exact reviewed IDs.")
    identifiers = {entry["id"] for entry in entries}
    coherence = {"ENGINE-COHERENCE-LOWER-EQUALITY", "ENGINE-COHERENCE-UPPER-EQUALITY"}
    fallback = {"ENGINE-COMMIT-DURATION-FALLBACK-SUB", "ENGINE-COMMIT-DURATION-FALLBACK-MUL"}
    if len(entries) != len(identifiers) or identifiers not in (
            coherence, coherence | {"ENGINE-NOMINAL-ZERO-SIGN"}, fallback):
        raise Incomplete("Engine equivalences require a complete reviewed measurement cohort or exact fallback pair.")
    for entry in entries:
        mutant, proof = entry.get("mutant"), entry.get("proof")
        subject, function, replacement, descriptor_digest = owners[entry["id"]]
        if (type(mutant) is not dict or type(mutant.get("function")) is not dict
                or mutant.get("file") != SUBJECTS[subject]
                or mutant["function"].get("function_name") != function
                or mutant.get("genre") != "BinaryOperator"
                or mutant.get("replacement") != replacement
                or type(proof) is not dict
                or proof.get("path") != "tasks/engine-analysis-equivalences.md"):
            raise Incomplete("Engine equivalence leaves its exact function/operator owner or review artifact.")
        if descriptor_digest is not None and (
                COMMON.canonical_sha256(mutant) != descriptor_digest
                or entry.get("source_sha256") !=
                "6f3c6eb2f491de7f27a89f8fba84556e818874a9c875643b08081206f5dd4d46"):
            raise Incomplete("Engine fallback equivalence requires its exact source and native descriptor.")


def read_policy(root: Path) -> dict:
    """Bind the fixed engine scope to the actual whole product and protected helpers."""
    policy = read_json(root / POLICY)
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "scope", "external_contracts"}
    if (set(policy) != set(FIXED_POLICY) | extras
            or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
                   for key, value in FIXED_POLICY.items() if key != "mutation_equivalences")
            or not isinstance(policy["scope"], str) or not 0 < len(policy["scope"]) <= 2048):
        raise Incomplete("Unreviewed engine analysis scope, threshold, budget or profile.")
    validate_equivalence_scope(policy["mutation_equivalences"])
    validate_contract_policy(policy["external_contracts"])
    _, identity = PRODUCT.admit_product(root)
    if (not isinstance(policy["candidate_sha256"], str)
            or policy["candidate_sha256"] != COMMON.canonical_sha256(identity)):
        raise Incomplete("Engine analysis policy differs from the complete current product candidate.")
    pins = policy["inputs_sha256"]
    if (not isinstance(pins, dict) or set(pins) != set(BOUND_INPUTS)
            or any(not isinstance(digest, str) or not re.fullmatch(r"[a-f0-9]{64}", digest)
                   or PRODUCT.sha256(root / name) != digest for name, digest in pins.items())):
        raise Incomplete("Engine analysis protected source/helper bindings changed.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == SCOPE.package and filename in module.get("files", [])
                       for module in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Engine file must have one matching critical/security owner.")
    inventory_bindings(policy)
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    return policy


def validate_contract_policy(value: dict) -> dict:
    """Admit only the current inert external-contract role, never old campaigns."""
    if (type(value) is not dict or set(value) != set(FIXED_CONTRACT_POLICY)
            or CONTRACT_SCOPE.subjects != COMMON.ENGINE_CONTRACT_SUBJECTS
            or any(type(value.get(key)) is not type(expected) or value[key] != expected
                   for key, expected in FIXED_CONTRACT_POLICY.items())):
        raise Incomplete("Engine external contracts require the exact current inert stable role.")
    return value


def read_contract_policy(root: Path) -> dict:
    """Keep the historical candidate labels separate from current source custody."""
    parent = read_json(root / POLICY)
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "scope", "external_contracts"}
    if set(parent) != set(FIXED_POLICY) | extras:
        raise Incomplete("Engine historical/contract policy fields are missing or unknown.")
    policy = validate_contract_policy(parent["external_contracts"])
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    filename = policy["critical_files"][0]
    if (manifest.get("critical_files", []).count(filename) != 1
            or sum(module.get("package") == CONTRACT_SCOPE.package and filename in module.get("files", [])
                   for module in manifest.get("security_modules", [])) != 1):
        raise Incomplete("Engine constructor owner lacks its exact critical/security registration.")
    return policy


def contract_examples() -> dict:
    """Exercise private checked plans without creating any runtime or socket.

    Negative APIs are compile-only, including the uncalled schedule helper.
    Every causal statement remains on line four, with all setup values used.
    """
    setup = (
        "let peers = [SocketAddr::from(([127, 0, 0, 1], 41_001)), "
        "SocketAddr::from(([127, 0, 0, 1], 41_002))]; "
        "let timeout = Duration::from_millis(250); "
        "let config = Config::new(peers, timeout)?; "
        "let schedule = ControlSchedule::new(None, None)?; "
        "let _ = (&peers, timeout, &config, &schedule);"
    )
    marker = "legal-engine-contracts:checked-only-no-runtime"
    legal = (
        "assert!(Config::new(peers, Duration::from_secs(30)).is_ok()); "
        "assert!(matches!(Config::new([peers[0], peers[0]], timeout), Err(PilotError::DuplicatePeer))); "
        "assert!(matches!(Config::new(peers, Duration::ZERO), Err(PilotError::InvalidTimeout))); "
        "assert!(matches!(Config::new(peers, Duration::from_secs(30) + Duration::from_nanos(1)), "
        "Err(PilotError::InvalidTimeout))); "
        "let zero_port = SocketAddr::from(([127, 0, 0, 1], 0)); "
        "assert!(matches!(Config::new([zero_port, peers[1]], timeout), Err(PilotError::InvalidPeer))); "
        "assert!(ControlSchedule::new(None, None).is_ok()); "
        "assert!(ControlSchedule::new(Some(Duration::ZERO), Some(Duration::from_secs(30))).is_ok()); "
        "let excessive = Some(Duration::from_secs(30) + Duration::from_nanos(1)); "
        "assert!(matches!(ControlSchedule::new(excessive, None), Err(PilotError::InvalidTimeout))); "
        "assert!(matches!(ControlSchedule::new(None, excessive), Err(PilotError::InvalidTimeout))); "
        'println!("' + marker + '");'
    )
    examples = {
        "legal": {"statement": legal, "code": None, "tokens": [], "success_marker": marker},
        "config_construction": {
            "statement": "let _ = Config { peers: [peers[0], peers[0]], timeout: Duration::ZERO, "
                         "experimental_profile: None };",
            "code": "E0451", "tokens": ["peers", "timeout", "experimental_profile", "private"]},
        "schedule_construction": {
            "statement": "let _ = ControlSchedule { stop_after: Some(Duration::from_secs(31)), "
                         "invalidate_after: Some(Duration::from_secs(31)) };",
            "code": "E0451", "tokens": ["stop_after", "invalidate_after", "private"]},
        "raw_config_prepare": {
            "statement": "let _ = prepare(&(peers, timeout));",
            "code": "E0308", "tokens": ["Config", "mismatched types"]},
        "raw_schedule_execute": {
            "statement": "fn rejected(pilot: PreparedPilot) { let _ = execute(pilot, "
                         "(None::<Duration>, None::<Duration>)); }",
            "code": "E0308", "tokens": ["ControlSchedule", "mismatched types"]},
    }
    for name, value, field, replacement in (
            ("config_peers", "config", "peers", "[peers[0], peers[0]]"),
            ("config_timeout", "config", "timeout", "Duration::ZERO"),
            ("schedule_stop", "schedule", "stop_after", "Some(Duration::from_secs(31))"),
            ("schedule_invalidate", "schedule", "invalidate_after", "Some(Duration::from_secs(31))")):
        examples[name + "_mutation"] = {
            "statement": f"let mut value = {value}; value.{field} = {replacement}; let _ = &value;",
            "code": "E0616", "tokens": [field, "private"]}
    for name, typename, raw in (
            ("config", "Config", "(peers, timeout)"),
            ("schedule", "ControlSchedule", "(None::<Duration>, None::<Duration>)")):
        examples[name + "_default"] = {
            "statement": f"let _: {typename} = Default::default();",
            "code": "E0277", "tokens": [typename, "Default"]}
        examples[name + "_from"] = {
            "statement": f"let _: {typename} = {raw}.into();",
            "code": "E0277", "tokens": [typename, "From"]}
    for specimen in examples.values():
        specimen["source"] = "\n".join([
            "use nevrotick_engine::*; use std::{net::SocketAddr, time::Duration};",
            "fn main() -> Result<(), PilotError> {", setup, specimen.pop("statement"), "Ok(())", "}"]) + "\n"
        specimen["line"] = 4
    return examples


def copy_contract_dependencies(session, artifact: str) -> Path:
    """Freeze actual Cargo-emitted libraries as independent immutable copies."""
    source = session.work / artifact.removeprefix("/work/")
    if (source.resolve(strict=True) != source or not source.is_file()
            or not source.is_relative_to(session.work / "target")):
        raise Incomplete("Actual engine contract library escaped its emitted target.")
    destination = session.work / "contract-deps"
    destination.mkdir(mode=0o700)
    for item in sorted((session.work / "target/debug/deps").iterdir()):
        if item.suffix not in {".rlib", ".rmeta", ".so"}:
            continue
        digest = PRODUCT.sha256(item)
        target = destination / item.name
        shutil.copyfile(item, target)
        target.chmod(0o400)
        if PRODUCT.sha256(target) != digest:
            raise Incomplete("Engine contract dependency changed while copied.")
    library = destination / "libnevrotick_engine.rlib"
    digest = PRODUCT.sha256(source)
    shutil.copyfile(source, library)
    library.chmod(0o400)
    if PRODUCT.sha256(library) != digest:
        raise Incomplete("The copied engine library differs from its actual Cargo artifact.")
    session.protect(destination)
    return destination


def contracts(session, policy: dict, report: dict, *, examples=None, effect_scope=None,
              deadline: float | None = None) -> None:
    """Compile intended external causes and execute only checked constructors."""
    if deadline is None:
        deadline = time.monotonic() + policy["contracts_wall_seconds"]
    output = session.run(
        "contract-library", [str(session.stable / "bin/cargo"), "build", "--package", "nevrotick-engine",
        "--frozen", "--jobs", str(policy["jobs"]), "--lib", "--message-format=json"],
        COMMON.remaining_timeout(deadline, 120))
    artifact = COMMON.library_artifact(output, "nevrotick-engine",
                                       "/work/source/crates/nevrotick-engine/Cargo.toml")
    dependencies = copy_contract_dependencies(session, artifact)
    consumers = session.work / "contract-inputs"
    consumers.mkdir(mode=0o700)
    (session.work / "contracts").mkdir(mode=0o700)
    examples = contract_examples() if examples is None else examples
    for name, specimen in examples.items():
        (consumers / (name + ".rs")).write_text(specimen["source"])
    session.protect(consumers)
    for name, specimen in examples.items():
        command = [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", name,
                   "--error-format=json", "/work/contract-inputs/" + name + ".rs",
                   "--extern", "nevrotick_engine=/work/contract-deps/libnevrotick_engine.rlib",
                   "-L", "dependency=/work/contract-deps", "--out-dir", "/work/contracts"]
        session.run("external-consumer-" + name, command, COMMON.remaining_timeout(deadline, 60),
                    sealed=[consumers, dependencies], specimen=specimen if specimen["code"] else None)
        if specimen["code"] is None:
            text = session.run("external-legal-execution", ["/work/contracts/" + name],
                               COMMON.remaining_timeout(deadline, 30), sealed=[consumers, dependencies])
            if specimen["success_marker"] not in text:
                raise Incomplete("Legal engine consumer did not reach checked constructor neighbors.")
    report["contracts"] = {
        "legal_consumers_compiled_and_executed": 1,
        "library_artifact": artifact,
        "immutable_dependencies_sha256": PRODUCT.inventory(dependencies),
        "required_diagnostics": {name: {key: specimen[key] for key in ("code", "tokens", "line")}
                                 for name, specimen in examples.items() if specimen["code"]},
        "effect_scope": ("Config/ControlSchedule construction only; no runtime, socket or execute calls"
                         if effect_scope is None else effect_scope),
    }


def execute_contracts(args, report: dict) -> None:
    """Qualify only external admission contracts on a separately admitted candidate."""
    policy = read_contract_policy(ROOT)
    if args.subject not in {"library", "all"}:
        raise Incomplete("External engine contracts belong only to the library owner.")
    COMMON.admission_profile(args, CONTRACT_SCOPE, policy)
    report.update(subjects=CONTRACT_SCOPE.select(args.subject), qualification_passed=False,
                  contracts_qualification_passed=False, historical_campaign_bindings_reused=False)
    session = COMMON.Session(ROOT, args, report, CONTRACT_SCOPE, policy, (WRAPPER, TESTS))
    try:
        if read_contract_policy(session.source) != policy:
            raise Incomplete("Captured engine contract policy differs from its admitted current role.")
        report["candidate_sha256"] = COMMON.canonical_sha256(session.product_identity)
        contracts(session, policy, report)
    finally:
        session.finish()
    report["contracts_qualification_passed"] = True


def readiness_proposal() -> dict:
    """Describe a finite single-worker measurement; never execute or admit it."""
    return {
        "kind": "proposal", "executed": False, "outer_workers": 1,
        "inner_cargo_jobs": 1, "test_threads": 1,
        "build_seconds": 90, "test_seconds": 60,
        "commands": [
            ["cargo", "test", "--no-run", "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"],
            ["cargo", "test", "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"],
        ],
        "required_observations": ["actual selected tests", "peak allocated worker tree bytes",
                                  "minimum normal-user available bytes", "process custody and actual exits"],
        "ten_worker_readiness": "not assessed; reserve ten measured workers plus stop margin before execution",
        "limits": "existing rlimits and sampled storage guards are not a hard aggregate RAM/disk quota",
    }


def listed_summary(inventories: dict) -> dict:
    """Hash every observed descriptor field except its optional rendered diff."""
    return {name: {"count": len(items), "sha256": COMMON.canonical_sha256({
        key: {field: value for field, value in item.items() if field != "diff"}
        for key, item in items.items()})} for name, items in inventories.items()}


def mutation_prefix(tools: Path, subject: str, policy: dict) -> list[str]:
    """Use the unchanged exact shared campaign validator for the listing prefix."""
    filename = SCOPE.select(subject)[subject]
    prefix = [str(tools / "cargo-mutants"), "mutants", "--package", SCOPE.package,
              "--file", filename, "--test-workspace", "false", "--cap-lints", "false",
              "--jobs", str(policy["mutation_jobs"]), "--jobserver", "true",
              "--jobserver-tasks", str(policy["mutation_jobs"]), "--cargo-arg=--message-format=json"]
    campaign = [*prefix, "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                "--output", "/work/mutation/" + subject]
    COMMON.mutation_campaign_command(SCOPE, tools, subject, campaign,
                                    expected_jobs=policy["mutation_jobs"], expected_message_format="json")
    return prefix


def enumerate_only(session, subjects: dict, policy: dict, report: dict) -> None:
    """Retain complete bounded listings; launch no baseline or mutation campaign."""
    deadline = time.monotonic() + policy["enumeration_wall_seconds"]
    inventories = {}
    observations = report["mutation_enumeration"] = {}
    for name, filename in subjects.items():
        command = [*mutation_prefix(session.tools, name, policy), "--list", "--json"]
        raw = session.run("mutation-enumeration:" + name, command, COMMON.remaining_timeout(deadline, 120))
        observations[name] = {"raw_output": raw}
        items, warning = COMMON.parse_mutation_listing(raw, expected_jobs=policy["mutation_jobs"])
        observations[name]["warning"] = warning
        inventories[name] = COMMON.mutation_inventory(SCOPE, items, policy["maximum_listed_per_file"], filename)
    total = COMMON.combined_mutant_count(inventories, policy["maximum_listed_total"])
    report["listed_inventories"] = inventories
    report["listed_inventory_bindings"] = listed_summary(inventories)
    report["listed_total"] = total
    report["campaign_cap_exceeded"] = total > policy["maximum_mutants"]
    report["campaign_admission"] = False
    report["qualification_passed"] = False


def validate_test_execution(text: str, *, doctests: bool) -> dict:
    """Require actual successful libtest records, summaries and the required cases."""
    records = re.findall(r"^test ([^\r\n]+?) \.\.\. ok\r?$", text, re.MULTILINE)
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    pattern = r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*"
    counts = [re.fullmatch(pattern, line) for line in summaries]
    if (not summaries or any(match is None for match in counts) or len(records) != len(set(records))
            or sum(int(match[1]) for match in counts if match is not None) != len(records)
            or not REQUIRED_TESTS <= set(records)):
        raise Incomplete("Engine execution omits required actual passing unit/integration records or complete summaries.")
    docs = [name for name in records if re.fullmatch(
        r"[^\r\n]+\.rs - [^\r\n]+ \(line [1-9][0-9]*\)(?: - compile fail)?", name)]
    doc_pattern = re.compile(
        r"(?:[^\r\n]+/)?crates/nevrotick-engine/src/(lib|measurement|relative)\.rs - "
        r"(Config::new|ControlSchedule::new|measurement::ExperimentalProfile|"
        r"measurement::ExperimentalMeasurement|measurement::NominalPhaseNs|"
        r"measurement::NominalRoundTripNs|relative::ExperimentalReferenceNs|"
        r"relative::ExperimentalRelativeMeasurement) \(line [1-9][0-9]*\)( - compile fail)?")
    signatures = Counter((match[1], match[2], bool(match[3]))
                         for name in docs if (match := doc_pattern.fullmatch(name)) is not None)
    expected = Counter({("lib", "Config::new", False): 1,
                        ("lib", "ControlSchedule::new", False): 1,
                        ("measurement", "measurement::ExperimentalProfile", False): 1,
                        ("measurement", "measurement::ExperimentalProfile", True): 4,
                        ("measurement", "measurement::ExperimentalMeasurement", False): 1,
                        ("measurement", "measurement::ExperimentalMeasurement", True): 4,
                        ("measurement", "measurement::NominalPhaseNs", False): 1,
                        ("measurement", "measurement::NominalPhaseNs", True): 4,
                        ("measurement", "measurement::NominalRoundTripNs", False): 1,
                        ("measurement", "measurement::NominalRoundTripNs", True): 4,
                        ("relative", "relative::ExperimentalReferenceNs", False): 1,
                        ("relative", "relative::ExperimentalReferenceNs", True): 4,
                        ("relative", "relative::ExperimentalRelativeMeasurement", False): 1,
                        ("relative", "relative::ExperimentalRelativeMeasurement", True): 5})
    if doctests and (signatures != expected or len(docs) != sum(expected.values())):
        raise Incomplete("Engine mutation baseline omits its actual construction/privacy doctests.")
    if not doctests and docs:
        raise Incomplete("Coverage selection unexpectedly includes doctests.")
    return {"tests_executed": len(records), "required_tests": sorted(REQUIRED_TESTS), "doctests": len(docs)}


def validate_baseline(work: Path, subject: str) -> dict:
    """Inspect the tool's actual source-bound baseline before accepting any score."""
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    baseline = [item for item in outcomes.get("outcomes", [])
                if isinstance(item, dict) and item.get("scenario") == "Baseline"]
    if len(baseline) != 1 or baseline[0].get("summary") != "Success":
        raise Incomplete("Engine mutation requires exactly one successful actual baseline.")
    relative = COMMON.analysis_output_path(baseline[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Engine baseline log escapes its owned output.")
    path = directory / relative
    if path.is_symlink():
        raise Incomplete("Engine baseline log cannot use a linked path.")
    data = COMMON.read_analysis_bytes(path, maximum=16 * 1024 * 1024)
    return {**validate_test_execution(data.decode("utf-8"), doctests=True), "log_sha256": hashlib.sha256(data).hexdigest()}


def series_inventory_bindings(policy: dict, *, required: bool = False) -> dict:
    """Keep discovery separate from exact, complete two-owner campaign admission."""
    bindings = policy.get("mutation_inventories")
    if type(bindings) is not dict:
        raise Incomplete("Series mutation inventory bindings must be an object.")
    if not bindings and not required:
        return {}
    if set(bindings) != set(SERIES_SUBJECTS):
        raise Incomplete("Series mutation needs reviewed nonempty inventories for both owners.")
    for binding in bindings.values():
        if (type(binding) is not dict or set(binding) != {"count", "sha256"}
                or type(binding["count"]) is not int or not 0 < binding["count"] <= 512
                or type(binding["sha256"]) is not str
                or re.fullmatch(r"[a-f0-9]{64}", binding["sha256"]) is None):
            raise Incomplete("Series mutation requires exact positive inventory counts and hashes.")
    return bindings


def series_test_inventory(policy: dict) -> tuple[list[str], list[str]]:
    """Admit all current package identities, not only a minimum N3 name subset."""
    inventory = policy.get("test_inventory")
    if (type(inventory) is not dict
            or set(inventory) != {"ordinary", "doctests", "baseline_count", "sha256", "n3_doctests"}
            or type(inventory["baseline_count"]) is not int or inventory["baseline_count"] != 163
            or inventory["sha256"] != SERIES_INVENTORY_DIGESTS
            or inventory["n3_doctests"] != SERIES_N3_DOCTESTS):
        raise Incomplete("Series test inventory or legal/compile-only contract identities changed.")
    ordinary, docs = inventory["ordinary"], inventory["doctests"]
    for names, count, kind in ((ordinary, 113, "ordinary"), (docs, 50, "doctests")):
        if (type(names) is not list or len(names) != count
                or any(type(name) is not str or not name or len(name) > 1024 for name in names)
                or len(set(names)) != count or names != sorted(names)
                or COMMON.canonical_sha256(names) != SERIES_INVENTORY_DIGESTS[kind]):
            raise Incomplete("Series requires its complete unique frozen package test identities.")
    if (set(ordinary) != REQUIRED_TESTS | SERIES_REQUIRED_TESTS | SERIES_CHECKPOINT_REQUIRED_TESTS | QUALITY_REQUIRED_TESTS
            or not set(SERIES_N3_DOCTESTS) <= set(docs)
            or COMMON.canonical_sha256(sorted(ordinary + docs)) != SERIES_INVENTORY_DIGESTS["baseline"]):
        raise Incomplete("Series package inventory omits an existing or new actual control.")
    return ordinary, docs


def validate_series_equivalence_scope(entries: object) -> None:
    """Allow no E or the exact reviewed fallback and unavailable-admission pairs.

    This does not inherit the historical engine cohort. Shared admission still
    checks actual candidate/source/proof bytes and complete native missed faults;
    its timeout, native-exit and score gates remain unchanged.
    """
    if type(entries) is not list:
        raise Incomplete("Series equivalences require an exact list.")
    if not entries:
        return
    required = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    if (len(entries) != 4 or any(type(entry) is not dict or set(entry) != required for entry in entries)
            or any(type(entry["id"]) is not str for entry in entries)
            or {entry["id"] for entry in entries} != set(SERIES_EQUIVALENCE_DESCRIPTORS)):
        raise Incomplete("Series equivalences require the complete exact current four-ID N3 cohort.")
    for entry in entries:
        mutant, proof = entry["mutant"], entry["proof"]
        expected_source = (SERIES_ZERO_ADMISSION_SOURCE
                           if entry["id"].startswith("ENGINE-SERIES-UNAVAILABLE-")
                           else SERIES_EQUIVALENCE_SOURCE)
        if (entry["source_sha256"] != expected_source
                or type(mutant) is not dict
                or COMMON.canonical_sha256(mutant) != SERIES_EQUIVALENCE_DESCRIPTORS[entry["id"]]
                or type(entry["candidate_sha256"]) is not str
                or re.fullmatch(r"[a-f0-9]{64}", entry["candidate_sha256"]) is None
                or type(proof) is not dict
                or set(proof) != {"path", "sha256", "section", "reviewer", "verdict"}
                or proof["path"] != "tasks/engine-analysis-equivalences.md"
                or proof["section"] != entry["id"]
                or proof["verdict"] != "independently-justified-equivalent"
                or type(proof["sha256"]) is not str
                or re.fullmatch(r"[a-f0-9]{64}", proof["sha256"]) is None
                or type(proof["reviewer"]) is not str or not 0 < len(proof["reviewer"].strip()) <= 128
                or any(type(entry[key]) is not str or not 0 < len(entry[key].strip()) <= maximum
                       for key, maximum in (("requirement", 256), ("rationale", 4096)))):
            raise Incomplete("Series equivalence leaves its exact current source, native descriptor or review identity.")


def validate_series_policy(policy: dict) -> dict:
    """Admit only the explicit N3 role and its finite, unchanged gate settings."""
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "mutation_equivalences", "scope", "test_inventory"}
    if (type(policy) is not dict or set(policy) != set(SERIES_FIXED_POLICY) | extras
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in SERIES_FIXED_POLICY.items())
            or type(policy.get("scope")) is not str or not 0 < len(policy["scope"]) <= 2048
            or type(policy.get("candidate_sha256")) is not str
            or re.fullmatch(r"[a-f0-9]{64}", policy["candidate_sha256"]) is None):
        raise Incomplete("Unreviewed series scope, thresholds, budgets or full tool profile.")
    pins = policy["inputs_sha256"]
    if (type(pins) is not dict or set(pins) != set(SERIES_CURRENT_BOUND_INPUTS)
            or any(type(h) is not str or re.fullmatch(r"[a-f0-9]{64}", h) is None for h in pins.values())):
        raise Incomplete("Series protected input bindings are incomplete or malformed.")
    series_test_inventory(policy)
    series_inventory_bindings(policy)
    validate_series_equivalence_scope(policy["mutation_equivalences"])
    if any(entry["candidate_sha256"] != policy["candidate_sha256"] for entry in policy["mutation_equivalences"]):
        raise Incomplete("Series equivalence candidate differs from its complete policy candidate.")
    return policy


def read_series_policy(root: Path) -> dict:
    """Bind the new role to actual product, helpers and existing manifest owners."""
    policy = validate_series_policy(read_json(root / SERIES_POLICY))
    _, identity = PRODUCT.admit_product(root)
    if (len(identity) != policy["source_file_count"]
            or COMMON.canonical_sha256(identity) != policy["candidate_sha256"]):
        raise Incomplete("Series policy differs from the complete current product candidate.")
    if any(PRODUCT.sha256(root / name) != digest for name, digest in policy["inputs_sha256"].items()):
        raise Incomplete("Series protected source, test or helper bytes changed.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SERIES_SUBJECTS.values():
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == SERIES_SCOPE.package and filename in module.get("files", [])
                       for module in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Each series owner needs one existing critical/security registration.")
    COMMON.admit_mutation_equivalences(root, SERIES_SCOPE, policy)
    return policy


def validate_series_test_execution(text: str, policy: dict, *, doctests: bool) -> dict:
    """Reconcile exact unique native records and complete zero-failure summaries."""
    ordinary, docs = series_test_inventory(policy)
    if type(text) is not str:
        raise Incomplete("Series execution output must be captured text.")
    records = []
    for line in text.splitlines():
        if line.startswith("test ") and not line.startswith("test result:"):
            match = re.fullmatch(r"test ([^\r\n]+?) \.\.\. ok", line)
            if match is None:
                raise Incomplete("Series native record lacks its exact successful completion.")
            records.append(match[1])
        elif line in {"ok", "FAILED", "ignored"}:
            raise Incomplete("Series native completion lacks its test identity.")
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    counts = [re.fullmatch(
        r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*", line)
        for line in summaries]
    expected = sorted(ordinary + docs) if doctests else ordinary
    if (not records or not summaries or any(row is None for row in counts)
            or len(records) != len(set(records)) or sorted(records) != expected
            or sum(int(row[1]) for row in counts if row is not None) != len(records)):
        raise Incomplete("Series execution lacks its complete actual package inventory or summaries.")
    return {"tests_executed": len(records), "ordinary_tests": len(ordinary),
            "doctests": len(docs) if doctests else 0,
            "test_names": records, "inventory_sha256": COMMON.canonical_sha256(sorted(records)),
            "required_n3": sorted(SERIES_REQUIRED_TESTS | SERIES_CHECKPOINT_REQUIRED_TESTS),
            "n3_doctests": SERIES_N3_DOCTESTS if doctests else {},
            "no_run_meaning": "execute_series no_run is compiler evidence; its network body is not executed"}


def series_checkpoint_assertion_sites(source_root: Path, policy: dict) -> dict:
    """Bind the two new controls and closed semantic helpers to admitted bytes.

    Peer/channel/worker errors are not sites. The child bridge relays one fixed
    independent fact; its parent assertion alone supplies no semantic credit.
    """
    filename = "crates/nevrotick-engine/tests/private/series.rs"
    sources = {}
    for name in (filename, SERIES_PROCESS_SOURCE):
        data = COMMON.read_analysis_bytes(source_root / name, maximum=512 * 1024)
        if hashlib.sha256(data).hexdigest() != policy["inputs_sha256"].get(name):
            raise Incomplete("Series checkpoint semantic source differs from admitted bytes.")
        sources[name] = data.decode("utf-8")
    source = sources[filename]
    masked, functions = quality_rust_functions(source)
    allowed_helpers = {"final_checkpoint_case", "check_checkpoint_facts", "serve_checkpoint", "checkpoint_queries"}
    result = {"candidate_sha256": policy["candidate_sha256"],
              "source_sha256": {name: policy["inputs_sha256"][name] for name in sources}, "tests": {}}
    for test in sorted(SERIES_CHECKPOINT_REQUIRED_TESTS):
        pending, visited, sites = [test.removeprefix("series::tests::")], set(), []
        while pending:
            owner = pending.pop()
            if owner in visited:
                continue
            if owner not in functions:
                raise Incomplete("Series checkpoint test/helper has no admitted function body.")
            visited.add(owner)
            start, end = functions[owner]["start"], functions[owner]["end"]
            code = masked[start:end]
            pending.extend(helper for helper in sorted(allowed_helpers)
                           if re.search(r"\b" + helper + r"\s*\(", code))
            for match in re.finditer(r"\b(assert|assert_eq|assert_ne)!\s*\(", code):
                offset = start + match.start()
                marker = {"assert": "assertion failed:", "assert_eq": "assertion `left == right` failed",
                          "assert_ne": "assertion `left != right` failed"}[match[1]]
                opening = start + match.end() - 1
                depth, close = 1, opening + 1
                while close < end and depth:
                    depth += (masked[close] == "(") - (masked[close] == ")")
                    close += 1
                strings = re.findall(r'"((?:\\.|[^"\\])*)"', source[opening:close])
                if depth or (strings and (match[1] != "assert" or len(strings) != 1
                                          or "\\" in strings[0] or "{" in strings[0])):
                    raise Incomplete("Series checkpoint assertion has an unreviewed message shape.")
                if strings:
                    marker = strings[0]
                sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                              "column": offset - source.rfind("\n", 0, offset), "owner": owner,
                              "kind": "panic_assertion", "marker": marker})
            for match in re.finditer(r"\bcheckpoint_fact\s*\(", code):
                offset, opening = start + match.start(), start + match.end() - 1
                depth, close = 1, opening + 1
                while close < end and depth:
                    depth += (masked[close] == "(") - (masked[close] == ")")
                    close += 1
                strings = re.findall(r'"((?:\\.|[^"\\])*)"', source[opening:close])
                if depth or len(strings) != 1 or "\\" in strings[0]:
                    raise Incomplete("Series checkpoint fact lacks one exact static source marker.")
                sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                              "column": offset - source.rfind("\n", 0, offset), "owner": owner,
                              "kind": "static_checkpoint_fact", "marker": strings[0]})
        if not sites:
            raise Incomplete("Series checkpoint control has no admitted semantic assertion.")
        result["tests"][test] = sorted(sites, key=lambda row: (row["line"], row["column"]))
    process = sources[SERIES_PROCESS_SOURCE]
    bridge = list(re.finditer(r'assert!\(\s*completion == Completion::Passed,\s*"N3 exact child semantic failure assertion"', process))
    if len(bridge) != 1:
        raise Incomplete("Series checkpoint child bridge assertion changed.")
    offset = bridge[0].start()
    result["bridge"] = {"source": SERIES_PROCESS_SOURCE, "line": process.count("\n", 0, offset) + 1,
                        "column": offset - process.rfind("\n", 0, offset),
                        "marker": "N3 exact child semantic failure assertion"}
    return result


def series_checkpoint_caught_causes(text: str, semantic_sites: dict | None) -> list[dict]:
    """Require every failed checkpoint control to have its own mapped cause.

    A genuine assertion elsewhere cannot hide a checkpoint fixture timeout.
    The legacy non-checkpoint semantic review remains independently required.
    """
    if re.search(r"signal:\s*\d|SIG(?:KILL|ABRT|SEGV|BUS|ILL)|process aborted|fatal runtime error", text):
        raise Incomplete("Series checkpoint log contains an aborted native test process.")
    blocks = list(re.finditer(
        r"^running (\d{1,6}) tests?\n(.*?)^test result: (ok|FAILED)\. "
        r"(\d{1,6}) passed; (\d{1,6}) failed; (\d{1,6}) ignored; "
        r"(\d{1,6}) measured; (\d{1,6}) filtered out;[^\r\n]*$", text,
        re.MULTILINE | re.DOTALL))
    sections = re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)", text,
                           re.MULTILINE | re.DOTALL)
    causes, seen = [], set()
    for section in sections:
        test, body = section.groups()
        if test not in SERIES_CHECKPOINT_REQUIRED_TESTS:
            continue
        if (test in seen or type(semantic_sites) is not dict
                or set(semantic_sites.get("tests", {})) != set(SERIES_CHECKPOINT_REQUIRED_TESTS)):
            raise Incomplete("Series checkpoint cause needs its complete source-bound site map.")
        seen.add(test)
        containing = [block for block in blocks if block.start() <= section.start() < block.end()]
        if len(containing) != 1:
            raise Incomplete("Series checkpoint failure lacks a completed native test block.")
        block = containing[0]
        running, block_body, status, passed, failed, ignored, measured, filtered = block.groups()
        counts = tuple(map(int, (running, passed, failed, ignored, measured, filtered)))
        records = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", block_body)
        failed_names = [name for name, result in records if result == "FAILED"]
        listed = block_body.rsplit("\nfailures:\n", 1)
        listing = listed[-1].strip("\n").splitlines()
        section_names = re.findall(r"(?m)^---- ([^\r\n]+) stdout ----$", block_body)
        if (status != "FAILED" or not counts[2] or any(counts[3:])
                or counts[0] != counts[1] + counts[2] or len(records) != counts[0]
                or len({name for name, _ in records}) != len(records)
                or sum(result == "ok" for _, result in records) != counts[1]
                or len(failed_names) != counts[2] or test not in failed_names
                or len(listed) != 2 or any(not line.startswith("    ") or not line[4:] for line in listing)
                or sorted(line[4:] for line in listing) != sorted(failed_names)
                or sorted(section_names) != sorted(failed_names)
                or re.search(r"(?m)^running \d+ tests?", block_body)):
            raise Incomplete("Series checkpoint failure lacks consistent completed libtest records.")
        if re.search(r"(?m)^Error: ", body):
            raise Incomplete("Series checkpoint failure contains a fixture, channel, worker or I/O error.")
        admitted = semantic_sites["tests"][test]
        fact_prefix = "N3_SEMANTIC_EVIDENCE independent fact: "
        facts = [line.removeprefix(fact_prefix) for line in body.splitlines() if line.startswith(fact_prefix)]
        if body.count("N3_SEMANTIC_EVIDENCE") != len(facts) or len(facts) > 1:
            raise Incomplete("Series checkpoint child lacks one exact static semantic fact.")
        fact_sites = [site for fact in facts for site in admitted
                      if site["kind"] == "static_checkpoint_fact" and site["marker"] == fact]
        if len(fact_sites) != len(facts):
            raise Incomplete("Series checkpoint child fact is outside its admitted source owner.")
        locations = list(re.finditer(r"(?m)^thread '([^'\r\n]+)'(?: \(\d+\))? panicked at "
                                    r"(?:/work/source/)?([^\r\n]+):(\d+):(\d+):\n([^\r\n]+)", body))
        if not locations or body.count("panicked at ") != len(locations):
            raise Incomplete("Series checkpoint failure lacks an exact semantic assertion location.")
        section_causes, bridges = [], 0
        for location in locations:
            thread, source, line, column, message = location.groups()
            source = {"crates/nevrotick-engine/src/../tests/private/series.rs":
                          "crates/nevrotick-engine/tests/private/series.rs",
                      "crates/nevrotick-engine/src/../tests/private/series_process.rs":
                          SERIES_PROCESS_SOURCE}.get(source, source)
            coordinates = source, int(line), int(column)
            bridge = semantic_sites["bridge"]
            if (thread == test and coordinates == (bridge["source"], bridge["line"], bridge["column"])
                    and message == bridge["marker"] and len(fact_sites) == 1):
                bridges += 1
                continue
            sites = [site for site in admitted if site["kind"] == "panic_assertion"
                     and coordinates == (site["source"], site["line"], site["column"])
                     and message.startswith(site["marker"])]
            if thread != test or len(sites) != 1:
                raise Incomplete("Series checkpoint panic is not its named source-owned semantic assertion.")
            section_causes.append({"test": test, **sites[0]})
        if facts and bridges != 1:
            raise Incomplete("Series checkpoint static fact lacks its exact completed child bridge.")
        section_causes.extend({"test": test, **site} for site in fact_sites)
        if not section_causes:
            raise Incomplete("Series checkpoint failure has no admitted semantic cause.")
        causes.extend(section_causes)
    named_failures = [name for name in re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. FAILED$", text)
                      if name in SERIES_CHECKPOINT_REQUIRED_TESTS]
    if len(named_failures) != len(set(named_failures)) or set(named_failures) != seen:
        raise Incomplete("Series named checkpoint failure lacks its complete semantic section.")
    return causes


def validate_series_caught_causes(directory: Path, outcomes: dict, semantic_sites: dict | None = None) -> dict:
    """Refuse marked N3 harness failures before they can qualify as caught faults.

    Use bounded admitted bytes for every selected native log. This checks the
    explicit containment failure marker and the new checkpoint source-owned
    assertions; independent review of other causes remains required.
    """
    rows = outcomes.get("outcomes")
    if (type(rows) is not list or len(rows) > 513
            or any(type(row) is not dict for row in rows)):
        raise Incomplete("Series caught-cause records are malformed or exceed their bounded inventory.")
    checked, checkpoint_causes = {}, {}
    for row in rows:
        if row.get("summary") != "CaughtMutant":
            continue
        scenario = row.get("scenario")
        mutant = scenario.get("Mutant") if type(scenario) is dict else None
        name = mutant.get("name") if type(mutant) is dict else None
        if type(name) is not str or not 0 < len(name) <= 4096 or name in checked:
            raise Incomplete("Series caught-cause identity is missing or duplicated.")
        relative = COMMON.analysis_output_path(row.get("log_path", ""))
        if relative.is_absolute() or not relative.parts or ".." in relative.parts:
            raise Incomplete("Series caught-cause log escapes its owned output.")
        data = COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)
        if b"N3_HARNESS_INCOMPLETE" in data.splitlines():
            raise Incomplete("Series caught fault contains an unclassified harness failure.")
        causes = series_checkpoint_caught_causes(data.decode("utf-8"), semantic_sites)
        if causes:
            checkpoint_causes[name] = causes
        checked[name] = hashlib.sha256(data).hexdigest()
    return {"checked_caught_logs": checked, "marked_harness_failures": 0,
            "checkpoint_semantic_causes": checkpoint_causes,
            "independent_semantic_review_required": True}


def validate_series_baseline(work: Path, subject: str, policy: dict) -> dict:
    """Read one bounded actual baseline and hash exactly the parsed log bytes."""
    if subject not in SERIES_SUBJECTS:
        raise Incomplete("Series baseline needs one exact critical owner.")
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    rows = [x for x in outcomes.get("outcomes", []) if type(x) is dict and x.get("scenario") == "Baseline"]
    if len(rows) != 1 or rows[0].get("summary") != "Success":
        raise Incomplete("Series mutation requires exactly one successful actual baseline.")
    relative = COMMON.analysis_output_path(rows[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Series baseline log escapes its owned output.")
    data = COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)
    return {**validate_series_test_execution(data.decode("utf-8"), policy, doctests=True),
            "log_sha256": hashlib.sha256(data).hexdigest(),
            "caught_cause_admission": validate_series_caught_causes(directory, outcomes,
                series_checkpoint_assertion_sites(work / "source", policy))}


def series_contract_examples() -> dict:
    """Use the existing external-consumer harness for exact N3 causes on line four.

    The legal body exercises inert constructor neighbors and type-checks function
    pointers only. Neither preparation entropy nor execute_series is invoked.
    Negative helper bodies are compiler-only, including the double-consumption case.
    """
    setup = ("let peer = SocketAddr::from(([127, 0, 0, 1], 41001)); "
             "let request = Duration::from_millis(100); let total = Duration::from_secs(1); "
             "let config = SeriesConfig::new(peer, request, total)?; let _ = &config;")
    marker = "legal-series-contracts:checked-only-no-preparation-or-network"
    legal = ("assert!(SeriesConfig::new(peer, request, total).is_ok()); "
             "assert!(SeriesConfig::new(peer, Duration::ZERO, total).is_err()); "
             "assert!(SeriesConfig::new(peer, total, request).is_err()); "
             "assert!(SeriesConfig::new(SocketAddr::from(([127,0,0,1],0)), request, total).is_err()); "
             "let _: fn(&SeriesConfig) -> Result<(PreparedSeries, ControlHandle), PilotError> = prepare_series; "
             "let _: fn(PreparedSeries, &ControlSchedule) -> Result<SeriesReport, PilotError> = execute_series; "
             'println!("' + marker + '");')
    examples = {
        "series_legal": {"statement": legal, "code": None, "tokens": [], "success_marker": marker},
        "series_config_construction": {
            "statement": "fn bypass(base: SeriesConfig) { let _ = SeriesConfig { request_timeout: Duration::ZERO, ..base }; }",
            "code": "E0451", "tokens": ["request_timeout", "private"]},
        "series_prepared_construction": {
            "statement": "fn bypass(base: PreparedSeries, peer: SocketAddr) { let _ = PreparedSeries { peer, ..base }; }",
            "code": "E0451", "tokens": ["peer", "private"]},
        "series_attempt_construction": {
            "statement": "fn bypass(base: SeriesAttempt) { let _ = SeriesAttempt { index: 0, ..base }; }",
            "code": "E0451", "tokens": ["index", "private"]},
        "series_report_construction": {
            "statement": "fn bypass(base: SeriesReport) { let _ = SeriesReport { primary_failure: None, ..base }; }",
            "code": "E0451", "tokens": ["primary_failure", "private"]},
        "series_config_mutation": {
            "statement": "let mut value = config; value.series_timeout = Duration::ZERO; let _ = &value;",
            "code": "E0616", "tokens": ["series_timeout", "private"]},
        "series_prepared_mutation": {
            "statement": "fn mutate(mut value: PreparedSeries) { value.series_timeout = Duration::ZERO; let _ = &value; }",
            "code": "E0616", "tokens": ["series_timeout", "private"]},
        "series_attempt_mutation": {
            "statement": "fn mutate(mut value: SeriesAttempt) { value.index = 0; let _ = &value; }",
            "code": "E0616", "tokens": ["index", "private"]},
        "series_report_mutation": {
            "statement": "fn mutate(mut value: SeriesReport) { value.primary_failure = None; let _ = &value; }",
            "code": "E0616", "tokens": ["primary_failure", "private"]},
        "series_prepared_consumed": {
            "statement": "fn reuse(plan: PreparedSeries, first: &ControlSchedule, second: &ControlSchedule) { let _ = execute_series(plan, first); let _ = execute_series(plan, second); }",
            "code": "E0382", "tokens": ["plan", "moved"]},
    }
    for kind, typename in (("config", "SeriesConfig"), ("prepared", "PreparedSeries"),
                           ("attempt", "SeriesAttempt"), ("report", "SeriesReport")):
        examples["series_" + kind + "_default"] = {
            "statement": f"let _: {typename} = Default::default();",
            "code": "E0277", "tokens": [typename, "Default"]}
    for specimen in examples.values():
        specimen["source"] = "\n".join([
            "use nevrotick_engine::*; use std::{net::SocketAddr, time::Duration};",
            "fn main() -> Result<(), PilotError> {", setup, specimen.pop("statement"), "Ok(())", "}"]) + "\n"
        specimen["line"] = 4
    return examples


def execute_series_profile(args, report: dict) -> None:
    """Run only the separately admitted N3 coverage/listing/mutation role."""
    if args.lane not in {"contracts", "enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Unknown series analysis lane.")
    policy = read_series_policy(ROOT)
    subjects = SERIES_SCOPE.select(args.subject)
    if args.lane in {"mutation", "all"}:
        bindings = series_inventory_bindings(policy, required=True)
        if sum(bindings[name]["count"] for name in subjects) > policy["maximum_mutants"]:
            raise Incomplete("Complete selected series mutation inventory exceeds 512 variants.")
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Series analysis requires five GiB of available preflight space.")
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"  # Tool custody only; no campaign is launched.
    report.update(subjects=subjects, candidate_sha256=policy["candidate_sha256"],
                  qualification_passed=False, profile="series",
                  scope="Bounded host N3 series " + args.lane,
                  generation_gap_controls="Separate function/macro inventory and direct controls required; no score inferred.")
    session = COMMON.Session(ROOT, session_args, report, SERIES_SCOPE, policy,
                             (WRAPPER, SERIES_TESTS, *SERIES_CURRENT_BOUND_INPUTS))

    def run(label, command, timeout, **options):
        result = session.run(label, command, timeout, **options)
        if label == "coverage":
            report["coverage_execution"] = validate_series_test_execution(result, policy, doctests=False)
        elif label.startswith("mutation-campaign:"):
            subject = label.removeprefix("mutation-campaign:")
            report.setdefault("baseline_execution", {})[subject] = validate_series_baseline(session.work, subject, policy)
        return result

    try:
        if COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]:
            raise Incomplete("Captured series Session differs from the admitted product.")
        if any(PRODUCT.sha256(session.source / name) != digest
               for name, digest in policy["inputs_sha256"].items()):
            raise Incomplete("Captured series protected input bytes changed.")
        if COMMON.canonical_sha256(read_json(session.source / SERIES_POLICY)) != COMMON.canonical_sha256(policy):
            raise Incomplete("Captured series policy differs from admitted policy.")
        if args.lane in {"contracts", "all"}:
            contracts(session, policy, report, examples=series_contract_examples(),
                      effect_scope="SeriesConfig constructors execute; preparation/execute signatures and negative consumption are compile-only. No entropy, runtime, socket or network body.")
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(
                SERIES_SCOPE, subjects, policy, session.tools, run, report,
                deadline=time.monotonic() + policy["enumeration_wall_seconds"],
                maximum_per_file=policy["maximum_listed_per_file"], maximum_total=policy["maximum_listed_total"])
            report.update(listed_inventories=listing.inventories,
                          listed_inventory_bindings=listed_summary(listing.inventories),
                          listed_total=listing.total, campaign_cap_exceeded=listing.campaign_budget_exceeded,
                          campaign_admission=False)
        elif args.lane != "contracts":
            for lane in (("coverage", "mutation") if args.lane == "all" else (args.lane,)):
                # Expose receiving-boundary failures before repeating the library.
                selected = ({name: subjects[name] for name in ("series", "library") if name in subjects}
                            if lane == "mutation" else subjects)
                COMMON.run_analysis_lanes(SERIES_SCOPE, selected, policy, session.work,
                                          session.tools, run, report, lane)
    finally:
        session.finish()
    if args.lane in {"contracts", "all"}:
        report["contracts_qualification_passed"] = True
    if args.lane not in {"enumerate", "contracts"}:
        report["qualification_passed"] = True


def quality_inventory_bindings(policy: dict, *, required: bool = False) -> dict:
    """Permit discovery without pins; campaigns require the complete reviewed owner."""
    bindings = policy.get("mutation_inventories")
    if type(bindings) is not dict:
        raise Incomplete("Quality mutation inventory bindings must be an object.")
    if not bindings and not required:
        return {}
    if set(bindings) != set(QUALITY_SUBJECTS):
        raise Incomplete("Quality mutation requires its reviewed nonempty selected inventory.")
    binding = bindings["quality"]
    if (type(binding) is not dict or set(binding) != {"count", "sha256"}
            or type(binding["count"]) is not int or not 0 < binding["count"] <= 512
            or type(binding["sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", binding["sha256"]) is None):
        raise Incomplete("Quality inventory requires an exact positive count and SHA-256 pin.")
    return bindings


def quality_test_inventory(policy: dict) -> tuple[list[str], list[str]]:
    """Require the frozen complete package inventory and every Q1 contract identity."""
    inventory = policy.get("test_inventory")
    if (type(inventory) is not dict
            or set(inventory) != {"ordinary", "doctests", "baseline_count", "sha256", "q1_doctests"}
            or type(inventory["baseline_count"]) is not int or inventory["baseline_count"] != 163
            or inventory["sha256"] != QUALITY_INVENTORY_DIGESTS
            or inventory["q1_doctests"] != QUALITY_DOCTESTS):
        raise Incomplete("Quality test inventory or actual legal/negative contract identities changed.")
    ordinary, docs = inventory["ordinary"], inventory["doctests"]
    for names, count, kind in ((ordinary, 113, "ordinary"), (docs, 50, "doctests")):
        if (type(names) is not list or len(names) != count
                or any(type(name) is not str or not name or len(name) > 1024 for name in names)
                or len(set(names)) != count or names != sorted(names)
                or COMMON.canonical_sha256(names) != QUALITY_INVENTORY_DIGESTS[kind]):
            raise Incomplete("Quality needs its exact complete unique native test identities.")
    if (set(ordinary) != REQUIRED_TESTS | SERIES_REQUIRED_TESTS | SERIES_CHECKPOINT_REQUIRED_TESTS | QUALITY_REQUIRED_TESTS
            or not set(QUALITY_DOCTESTS) <= set(docs)
            or COMMON.canonical_sha256(sorted(ordinary + docs)) != QUALITY_INVENTORY_DIGESTS["baseline"]):
        raise Incomplete("Quality inventory omits a selected or existing actual control.")
    return ordinary, docs


def validate_quality_equivalence_scope(entries: object) -> None:
    """Permit only no E or the reviewed exact zero-sign fault; never an N3 cohort.

    Converter/source custody and actual native MissedMutant adjudication remain
    mandatory in the policy reader and shared analysis path. No kill is inferred.
    """
    if type(entries) is not list:
        raise Incomplete("Quality equivalences require an exact list.")
    if not entries:
        return
    required = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    if len(entries) != 1 or type(entries[0]) is not dict or set(entries[0]) != required:
        raise Incomplete("Quality permits only its exact single reviewed zero-sign equivalence.")
    entry = entries[0]
    proof, mutant = entry["proof"], entry["mutant"]
    if (entry["id"] != QUALITY_ZERO_EQUIVALENCE_ID
            or entry["source_sha256"] != QUALITY_SOURCE_PINS[QUALITY_SUBJECTS["quality"]]
            or type(mutant) is not dict or COMMON.canonical_sha256(mutant) != QUALITY_ZERO_EQUIVALENCE_DESCRIPTOR
            or type(entry["candidate_sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", entry["candidate_sha256"]) is None
            or type(proof) is not dict or set(proof) != {"path", "sha256", "section", "reviewer", "verdict"}
            or proof["path"] != "tasks/engine-analysis-equivalences.md"
            or proof["section"] != QUALITY_ZERO_EQUIVALENCE_ID
            or proof["verdict"] != "independently-justified-equivalent"
            or type(proof["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", proof["sha256"]) is None
            or type(proof["reviewer"]) is not str or not 0 < len(proof["reviewer"].strip()) <= 128
            or any(type(entry[key]) is not str or not 0 < len(entry[key].strip()) <= maximum
                   for key, maximum in (("requirement", 256), ("rationale", 4096)))):
        raise Incomplete("Quality equivalence leaves its exact source, descriptor or reviewed dossier identity.")


def validate_quality_policy(policy: dict) -> dict:
    """Admit current Q1 with no inherited scores and only its exact reviewed E."""
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories",
              "mutation_equivalences", "scope", "test_inventory", "ordinary_evidence"}
    if (type(policy) is not dict or set(policy) != set(QUALITY_FIXED_POLICY) | extras
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in QUALITY_FIXED_POLICY.items())
            or policy.get("candidate_sha256") != QUALITY_CANDIDATE
            or type(policy.get("scope")) is not str or not 0 < len(policy["scope"]) <= 2048):
        raise Incomplete("Unreviewed quality source, scope, threshold, budget or exclusion.")
    pins = policy["inputs_sha256"]
    if (type(pins) is not dict or set(pins) != set(QUALITY_BOUND_INPUTS)
            or any(type(h) is not str or re.fullmatch(r"[a-f0-9]{64}", h) is None for h in pins.values())
            or any(pins.get(name) != digest for name, digest in QUALITY_SOURCE_PINS.items())):
        raise Incomplete("Quality protected source, test or helper bindings are invalid.")
    expected_evidence = {"receipt": str(QUALITY_ORDINARY_RECEIPT),
                         "sha256": QUALITY_ORDINARY_SHA256, "recipes_sha256": QUALITY_ORDINARY_RECIPES}
    if policy["ordinary_evidence"] != expected_evidence:
        raise Incomplete("Quality ordinary receipt or immutable recipe binding changed.")
    quality_test_inventory(policy)
    quality_inventory_bindings(policy)
    validate_quality_equivalence_scope(policy["mutation_equivalences"])
    if any(entry["candidate_sha256"] != policy["candidate_sha256"] for entry in policy["mutation_equivalences"]):
        raise Incomplete("Quality equivalence differs from its complete policy candidate.")
    return policy


def admit_quality_ordinary(policy: dict) -> dict:
    """Re-read exact historical ordinary bytes/recipes; do not infer a fresh pass."""
    data = COMMON.read_analysis_bytes(QUALITY_ORDINARY_RECEIPT, maximum=512 * 1024)
    if hashlib.sha256(data).hexdigest() != policy["ordinary_evidence"]["sha256"]:
        raise Incomplete("Quality ordinary receipt bytes changed.")
    try:
        receipt = json.loads(data.decode("utf-8"), object_pairs_hook=COMMON.ANALYSIS.unique_object,
                             parse_float=finite_number, parse_constant=invalid_number)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Quality ordinary receipt is not strict bounded UTF-8 JSON.") from error
    if type(receipt) is not dict:
        raise Incomplete("Quality ordinary receipt must be an object.")
    recipes = {str(QUALITY_EVIDENCE_ROOT / name): digest for name, digest in QUALITY_ORDINARY_RECIPES.items()}
    if (receipt.get("status") != "passed-q1-quality-all" or receipt.get("bookends_unchanged") is not True
            or receipt.get("candidate_sha256") != policy["candidate_sha256"]
            or type(receipt.get("source_sha256")) is not dict
            or len(receipt["source_sha256"]) != policy["source_file_count"]
            or COMMON.canonical_sha256(receipt["source_sha256"]) != policy["candidate_sha256"]
            or receipt.get("recipes_sha256") != recipes
            or any(Path(path) != Path(path).resolve(strict=True) or PRODUCT.sha256(Path(path)) != digest
                   for path, digest in recipes.items())):
        raise Incomplete("Quality ordinary candidate, recipes or unchanged bookends are unsupported.")
    ordinary, docs = quality_test_inventory(policy)
    for lane, expected in (("engine-tests", ordinary), ("engine-optimized-tests", ordinary), ("engine-doctests", docs)):
        rows = [row for row in receipt.get("lanes", []) if type(row) is dict and row.get("lane") == lane]
        if (len(rows) != 1 or type(rows[0].get("tests")) is not dict
                or rows[0]["tests"].get("executed") != len(expected)
                or sorted(rows[0]["tests"].get("test_names", [])) != expected):
            raise Incomplete("Quality ordinary receipt lacks its actual complete selected native execution.")
    return {"receipt": str(QUALITY_ORDINARY_RECEIPT), "sha256": hashlib.sha256(data).hexdigest(),
            "candidate_sha256": receipt["candidate_sha256"], "fresh_execution": False}


def read_quality_policy(root: Path) -> dict:
    """Bind Q1 product/helpers, exact actual baseline and existing critical owners."""
    policy = validate_quality_policy(read_json(root / QUALITY_POLICY))
    _, identity = PRODUCT.admit_product(root)
    if len(identity) != policy["source_file_count"] or COMMON.canonical_sha256(identity) != policy["candidate_sha256"]:
        raise Incomplete("Quality policy differs from the complete current product candidate.")
    if any(PRODUCT.sha256(root / name) != digest for name, digest in policy["inputs_sha256"].items()):
        raise Incomplete("Quality protected source, test or analysis helper bytes changed.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    filename = QUALITY_SUBJECTS["quality"]
    if (manifest.get("critical_files", []).count(filename) != 1
            or sum(module.get("package") == QUALITY_SCOPE.package and filename in module.get("files", [])
                   for module in manifest.get("security_modules", [])) != 1):
        raise Incomplete("Quality needs exactly one current critical/security owner registration.")
    admit_quality_ordinary(policy)
    COMMON.admit_mutation_equivalences(root, QUALITY_SCOPE, policy)
    return policy


def validate_quality_test_execution(text: str, policy: dict, *, doctests: bool) -> dict:
    """Require every actual selected native identity and zero-failure summary."""
    ordinary, docs = quality_test_inventory(policy)
    if type(text) is not str:
        raise Incomplete("Quality native output must be bounded captured text.")
    records = []
    for line in text.splitlines():
        if line.startswith("test ") and not line.startswith("test result:"):
            match = re.fullmatch(r"test ([^\r\n]+?) \.\.\. ok", line)
            if match is None:
                raise Incomplete("Quality native record lacks its named successful completion.")
            records.append(match[1])
        elif line in {"ok", "FAILED", "ignored"}:
            raise Incomplete("Quality native completion lacks its selected identity.")
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    counts = [re.fullmatch(r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*", line)
              for line in summaries]
    expected = sorted(ordinary + docs) if doctests else ordinary
    if (not records or not summaries or any(row is None for row in counts)
            or len(set(records)) != len(records) or sorted(records) != expected
            or sum(int(row[1]) for row in counts if row is not None) != len(expected)):
        raise Incomplete("Quality execution lacks its complete actual native inventory or summaries.")
    return {"tests_executed": len(records), "ordinary_tests": len(ordinary),
            "doctests": len(docs) if doctests else 0, "test_names": records,
            "inventory_sha256": COMMON.canonical_sha256(sorted(records)),
            "required_q1": sorted(QUALITY_REQUIRED_TESTS), "q1_doctests": QUALITY_DOCTESTS if doctests else {}}


def quality_rust_functions(source: str) -> tuple[str, dict[str, dict]]:
    """Extract bounded top-level test/helper bodies, masking comments and strings.

    This intentionally handles the admitted Q1 and selected N3 checkpoint test
    syntax only. Changed shapes, duplicates or unmatched delimiters fail admission.
    """
    masked = re.sub(r'//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"',
                    lambda match: re.sub(r"[^\n]", " ", match[0]), source, flags=re.DOTALL)
    functions = {}
    for match in re.finditer(r"(?m)^fn ([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^\n]*>)?\s*\(", masked):
        opening = masked.find("{", match.end())
        if opening < 0 or match[1] in functions:
            raise Incomplete("Quality semantic function inventory changed or is malformed.")
        depth, end = 1, opening + 1
        while end < len(masked) and depth:
            depth += (masked[end] == "{") - (masked[end] == "}")
            end += 1
        if depth:
            raise Incomplete("Quality semantic function body is incomplete.")
        functions[match[1]] = {"start": opening + 1, "end": end - 1}
    return masked, functions


def quality_semantic_assertion_sites(source_root: Path, policy: dict) -> dict:
    """Bind named semantic sites to exactly the admitted immutable test bytes.

    Only explicit numeric helpers are traversed. Acquisition, peer, original
    transport and I/O helpers are excluded. The external check helper returns a
    unique static io::Error fact instead of a panic location; its exact marker
    binds the admitted source line. None of this supplies independent mutant
    equivalence or causality review.
    """
    private = "crates/nevrotick-engine/src/quality/tests.rs"
    external = "crates/nevrotick-engine/tests/quality.rs"
    helpers = {private: {"exact"}, external: {"check_quality", "check_ratio", "check_near"}}
    # These checks describe predecessor/peer fixture truth, not mutated Q1 output.
    excluded_facts = {
        "conditional quality changed literal peer claims",
        "quality completion lost the literal peer timestamps",
        "original nominal RTT did not reconstruct from actual wall captures",
    }
    result = {"candidate_sha256": policy["candidate_sha256"], "source_sha256": {}, "tests": {}}
    for filename, allowed_helpers in helpers.items():
        data = COMMON.read_analysis_bytes(source_root / filename, maximum=512 * 1024)
        digest = hashlib.sha256(data).hexdigest()
        if digest != policy["inputs_sha256"].get(filename):
            raise Incomplete("Quality semantic assertion source differs from its admitted bytes.")
        result["source_sha256"][filename] = digest
        source = data.decode("utf-8")
        masked, functions = quality_rust_functions(source)
        selected = [name for name in QUALITY_REQUIRED_TESTS if name.startswith("quality::tests::") == (filename == private)]
        for test in selected:
            name = test.removeprefix("quality::tests::")
            if name not in functions:
                raise Incomplete("Quality selected test has no admitted semantic function body.")
            pending, visited, sites = [name], set(), []
            while pending:
                owner = pending.pop()
                if owner in visited:
                    continue
                visited.add(owner)
                body = functions[owner]
                start, end = body["start"], body["end"]
                code = masked[start:end]
                for helper in allowed_helpers:
                    if re.search(r"\b" + helper + r"\s*\(", code):
                        if helper not in functions:
                            raise Incomplete("Quality numeric helper has no admitted source body.")
                        pending.append(helper)
                for match in re.finditer(r"\b(assert|assert_eq|assert_ne)!\s*\(", code):
                    offset = start + match.start()
                    line = source.count("\n", 0, offset) + 1
                    column = offset - source.rfind("\n", 0, offset)
                    marker = {"assert": "assertion failed:", "assert_eq": "assertion `left == right` failed",
                              "assert_ne": "assertion `left != right` failed"}[match[1]]
                    sites.append({"source": filename, "line": line, "column": column,
                                  "owner": owner, "kind": "panic_assertion", "marker": marker})
                if filename == external:
                    for match in re.finditer(r"\bcheck\s*\(", code):
                        offset = start + match.start()
                        opening = start + match.end() - 1
                        depth, close = 1, opening + 1
                        while close < end and depth:
                            depth += (masked[close] == "(") - (masked[close] == ")")
                            close += 1
                        if depth:
                            raise Incomplete("Quality semantic check call is incomplete.")
                        strings = re.findall(r'"((?:\\.|[^"\\])*)"', source[opening:close])
                        if len(strings) != 1 or "\\" in strings[0]:
                            raise Incomplete("Quality semantic check requires one literal fixed fact.")
                        fact = strings[0]
                        if fact in excluded_facts:
                            continue
                        sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                            "column": offset - source.rfind("\n", 0, offset), "owner": owner,
                            "kind": "typed_semantic_assertion",
                            "marker": 'Error: Custom { kind: Other, error: ' + json.dumps(fact) + ' }'})
            if not sites:
                raise Incomplete("Quality selected test has no admitted semantic assertion site.")
            result["tests"][test] = sorted(sites, key=lambda row: (row["source"], row["line"], row["column"]))
    if set(result["tests"]) != set(QUALITY_REQUIRED_TESTS):
        raise Incomplete("Quality semantic site inventory is incomplete.")
    return result


def quality_secondary_product_refusal(line: str) -> bool:
    """Admit only closed payload-free Q1 refusal Debug forms as secondary evidence.

    Such a refusal never earns a kill itself. Any I/O, peer, resource or unknown
    error remains a harness/cause gap, even beside a valid semantic assertion.
    """
    assumption = ("HistoricalQuantum|TransmitCaptureAllowance|ReceiveCaptureAllowance|FrequencyInterval|"
                  "AdvertisedDistanceLimit|DelayLimit|ServerProcessingLimit")
    interval = "Local|Server|RawRoundTrip"
    measurement = "Unavailable|InvalidProfile|CaptureOrder|MappingDiscontinuity|DateScope|EraNoCandidate|EraAmbiguous|Arithmetic|Protocol"
    stage = ("CombinedPrecision|FrequencyWidth|FrequencyHalfWidth|FrequencyDispersion|PeerDispersion|"
             "AdvertisedRootDelay|AdvertisedRootDispersion|AdvertisedDistance|RootDelay|RootDispersion|"
             "PeerDistance|RootDistance|AdjustedDelay")
    pattern = (r"Error: (?:InvalidAssumption\((?:" + assumption + r")\)|Measurement\((?:" + measurement
        + r")\)|ExactArithmetic\((?:" + interval + r")\)|NonFinite\((?:" + stage
        + r")\)|Rational \{ interval: (?:" + interval + r"), source: (?:ArithmeticInvariant|OutputInvariant) \}"
        + r"|AdvertisedDistance|PrecisionDelay|AdjustedDelay|ServerProcessing|OutputInvariant|ValidationInvariant)")
    return re.fullmatch(pattern, line) is not None


def validate_quality_caught_causes(directory: Path, outcomes: dict, semantic_sites: dict) -> dict:
    """Require source-bound primary assertions; unrelated failures earn no kill."""
    if (type(semantic_sites) is not dict or set(semantic_sites) != {"candidate_sha256", "source_sha256", "tests"}
            or type(semantic_sites["tests"]) is not dict or set(semantic_sites["tests"]) != set(QUALITY_REQUIRED_TESTS)):
        raise Incomplete("Quality caught causes require the complete admitted semantic site map.")
    rows = outcomes.get("outcomes")
    if type(rows) is not list or len(rows) > 513 or any(type(row) is not dict for row in rows):
        raise Incomplete("Quality caught-cause records are malformed or oversized.")
    checked = {}
    for row in rows:
        if row.get("summary") != "CaughtMutant":
            continue
        scenario = row.get("scenario")
        mutant = scenario.get("Mutant") if type(scenario) is dict else None
        name = mutant.get("name") if type(mutant) is dict else None
        if (type(name) is not str or not 0 < len(name) <= 4096 or name in checked
                or mutant.get("file") != QUALITY_SUBJECTS["quality"] or mutant.get("package") != QUALITY_SCOPE.package):
            raise Incomplete("Quality caught identity is missing, repeated or outside its selected owner.")
        relative = COMMON.analysis_output_path(row.get("log_path", ""))
        if relative.is_absolute() or not relative.parts or ".." in relative.parts:
            raise Incomplete("Quality caught log escapes its owned output.")
        data = COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)
        text = data.decode("utf-8")
        if ("N3_HARNESS_INCOMPLETE" in text or "Q1_HARNESS_INCOMPLETE" in text
                or re.search(r"error\[E\d+\]|could not compile|process didn't exit successfully.*rustc|"
                             r"timed out|no space left|cannot allocate memory|out of memory|"
                             r"failed to spawn|resource temporarily unavailable|signal:\s*\d|"
                             r"SIG(?:KILL|ABRT|SEGV|BUS|ILL)|process aborted|fatal runtime error", text, re.IGNORECASE)):
            raise Incomplete("Quality caught fault contains compiler or unclassified harness failure.")
        causes, secondary = [], []
        sections = set()
        failed_sections = list(re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)", text, re.MULTILINE | re.DOTALL))
        blocks = list(re.finditer(
            r"^running (\d{1,6}) tests?\n(.*?)^test result: (ok|FAILED)\. "
            r"(\d{1,6}) passed; (\d{1,6}) failed; (\d{1,6}) ignored; "
            r"(\d{1,6}) measured; (\d{1,6}) filtered out;[^\r\n]*$", text,
            re.MULTILINE | re.DOTALL))
        records = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", text)
        failed_names = [test for test, status in records if status == "FAILED"]
        if (not blocks or len(re.findall(r"(?m)^running ", text)) != len(blocks)
                or len(re.findall(r"(?m)^test result:", text)) != len(blocks)
                or len(failed_names) != len(set(failed_names))
                or sorted(match[1] for match in failed_sections) != sorted(failed_names)
                or any(sum(block.start() <= match.start() < block.end() for block in blocks) != 1
                       for match in failed_sections)):
            raise Incomplete("Quality caught fault lacks complete unique native failure records.")
        recorded = 0
        for block in blocks:
            running, body, status, passed, failed, ignored, measured, filtered = block.groups()
            counts = tuple(map(int, (running, passed, failed, ignored, measured, filtered)))
            block_records = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", body)
            block_failed = [test for test, result in block_records if result == "FAILED"]
            block_sections = re.findall(r"(?m)^---- ([^\r\n]+) stdout ----$", body)
            listed = body.rsplit("\nfailures:\n", 1)
            listing = listed[-1].strip("\n").splitlines()
            if (status != ("FAILED" if counts[2] else "ok") or any(counts[3:])
                    or counts[0] != counts[1] + counts[2] or len(block_records) != counts[0]
                    or len({test for test, _ in block_records}) != len(block_records)
                    or sum(result == "ok" for _, result in block_records) != counts[1]
                    or len(block_failed) != counts[2] or sorted(block_sections) != sorted(block_failed)
                    or re.search(r"(?m)^running ", body)
                    or (counts[2] and (len(listed) != 2
                        or any(not line.startswith("    ") or not line[4:] for line in listing)
                        or sorted(line[4:] for line in listing) != sorted(block_failed)))
                    or (not counts[2] and len(listed) != 1)):
                raise Incomplete("Quality caught fault has inconsistent completed libtest records.")
            recorded += len(block_records)
        if recorded != len(records):
            raise Incomplete("Quality caught fault has test records outside completed native blocks.")
        if (text.count("panicked at ") != sum(match[2].count("panicked at ") for match in failed_sections)
                or len(re.findall(r"(?m)^Error: ", text)) != sum(len(re.findall(r"(?m)^Error: ", match[2])) for match in failed_sections)):
            raise Incomplete("Quality caught fault has an error or panic outside a named failed-test section.")
        for match in failed_sections:
            test, body = match.groups()
            if test not in semantic_sites["tests"] or test in sections:
                raise Incomplete("Quality caught fault has an unrelated or repeated failed test section.")
            sections.add(test)
            admitted = semantic_sites["tests"][test]
            section_causes = []
            locations = list(re.finditer(r"(?m)^thread '([^'\r\n]+)'(?: \(\d+\))? panicked at "
                                        r"(?:/work/source/)?([^\r\n]+):(\d+):(\d+):\n([^\r\n]+)", body))
            if body.count("panicked at ") != len(locations):
                raise Incomplete("Quality caught panic lacks an exact admitted source location.")
            for location in locations:
                thread, source, line, column, message = location.groups()
                sites = [site for site in admitted if site["kind"] == "panic_assertion"
                         and (site["source"], site["line"], site["column"]) == (source, int(line), int(column))
                         and message.startswith(site["marker"])]
                if thread != test or len(sites) != 1:
                    raise Incomplete("Quality caught panic is not its named test's admitted semantic assertion site.")
                section_causes.append({"test": test, **sites[0]})
            for line in body.splitlines():
                if not line.startswith("Error: "):
                    continue
                sites = [site for site in admitted if site["kind"] == "typed_semantic_assertion" and line == site["marker"]]
                if len(sites) == 1:
                    section_causes.append({"test": test, **sites[0]})
                elif not sites and quality_secondary_product_refusal(line):
                    secondary.append({"test": test, "refusal": line})
                else:
                    raise Incomplete("Quality caught fault contains an unrelated fixture, peer or I/O error.")
            if not section_causes and not any(row["test"] == test for row in secondary):
                raise Incomplete("Quality failed test has no admitted semantic cause or closed product refusal.")
            causes.extend(section_causes)
        if not causes:
            raise Incomplete("Quality caught fault lacks a selected Q1 assertion with its actual owned source cause.")
        checked[name] = {"log_sha256": hashlib.sha256(data).hexdigest(), "causes": causes,
                         "secondary_product_refusals_without_credit": secondary}
    return {"checked_caught_logs": checked, "compiler_or_harness_kill_credit": 0,
            "semantic_site_map_sha256": COMMON.canonical_sha256(semantic_sites),
            "scope": "Named Q1 assertions only; independent descriptor/cause review remains required."}


def validate_quality_baseline(work: Path, subject: str, policy: dict) -> dict:
    """Require one complete successful actual selected baseline before fault scoring."""
    if subject != "quality":
        raise Incomplete("Quality baseline requires only its selected quality owner.")
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    rows = outcomes.get("outcomes")
    if type(rows) is not list or any(type(row) is not dict for row in rows):
        raise Incomplete("Quality baseline outcome inventory is malformed.")
    baseline = [row for row in rows if row.get("scenario") == "Baseline"]
    if len(baseline) != 1 or baseline[0].get("summary") != "Success":
        raise Incomplete("Quality mutation requires one successful actual selected native baseline.")
    COMMON.mutation_phases(QUALITY_SCOPE, baseline[0])
    relative = COMMON.analysis_output_path(baseline[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Quality baseline log escapes its owned output.")
    data = COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)
    return {**validate_quality_test_execution(data.decode("utf-8"), policy, doctests=True),
            "log_sha256": hashlib.sha256(data).hexdigest(),
            "caught_cause_admission": validate_quality_caught_causes(directory, outcomes,
                quality_semantic_assertion_sites(work / "source", policy))}


def quality_contract_examples() -> dict:
    """Execute checked constructor neighbors; compile actual external field/borrow bypasses."""
    setup = ("let prior = DeclaredFrequencyInterval::new(-0.0, 0.0)?; "
             "let limits = DeclaredQualityLimits::new(1.0, 2.0, 3.0)?; "
             "let assumptions = DeclaredQualityAssumptions::new(0.25, -0.0, 0.0, prior, limits)?; "
             "let _ = (&prior, &limits, &assumptions);")
    marker = "legal-quality-contracts:explicit-checked-assumptions-only"
    examples = {
        "quality_legal": {"statement": 'assert!(DeclaredFrequencyInterval::new(1.0, -1.0).is_err()); '
            'assert!(DeclaredQualityLimits::new(0.0, 2.0, 3.0).is_err()); '
            'assert!(DeclaredQualityAssumptions::new(0.0, 0.0, 0.0, prior, limits).is_err()); '
            'assert_eq!(assumptions.transmit_capture_allowance_seconds().to_bits(), (-0.0_f64).to_bits()); '
            "fn retained<'a>(facts: &'a TransportFacts, assumptions: &'a DeclaredQualityAssumptions) "
            "-> Result<ConditionalQuality<'a>, QualityRefusal> { evaluate_conditional_quality(facts, assumptions) } "
            "let _ = retained; "
            'println!("' + marker + '");', "code": None, "tokens": [], "success_marker": marker},
        "quality_frequency_construction": {"statement": "let _ = DeclaredFrequencyInterval { endpoints: [1.0, -1.0] };", "code": "E0451", "tokens": ["endpoints", "private"]},
        "quality_limits_construction": {"statement": "let _ = DeclaredQualityLimits { advertised_distance: 0.0, delay: 0.0, server_processing: 0.0 };", "code": "E0451", "tokens": ["advertised_distance", "private"]},
        "quality_frequency_mutation": {"statement": "let mut value = prior; value.endpoints = [1.0, -1.0]; let _ = value;", "code": "E0616", "tokens": ["endpoints", "private"]},
        "quality_assumptions_mutation": {"statement": "let mut value = assumptions; value.local_quantum = 0.0; let _ = value;", "code": "E0616", "tokens": ["local_quantum", "private"]},
        "quality_facts_substitution": {"statement": "fn substitute<'a>(mut value: ConditionalQuality<'a>, other: &'a TransportFacts) { value.facts = other; let _ = value; }", "code": "E0616", "tokens": ["facts", "private"]},
        "quality_assumptions_substitution": {"statement": "fn substitute<'a>(mut value: ConditionalQuality<'a>, other: &'a DeclaredQualityAssumptions) { value.assumptions = other; let _ = value; }", "code": "E0616", "tokens": ["assumptions", "private"]},
        "quality_facts_owner_escape": {"statement": "fn escape<'a>(facts: TransportFacts, assumptions: &'a DeclaredQualityAssumptions) -> Result<ConditionalQuality<'a>, QualityRefusal> { evaluate_conditional_quality(&facts, assumptions) }", "code": "E0515", "tokens": ["return value", "function parameter", "facts"]},
        "quality_assumptions_owner_escape": {"statement": "fn escape<'a>(facts: &'a TransportFacts, assumptions: DeclaredQualityAssumptions) -> Result<ConditionalQuality<'a>, QualityRefusal> { evaluate_conditional_quality(facts, &assumptions) }", "code": "E0515", "tokens": ["return value", "function parameter", "assumptions"]},
        "quality_raw_facts": {"statement": "let _ = evaluate_conditional_quality(&0_u8, &assumptions);", "code": "E0308", "tokens": ["TransportFacts", "mismatched types"]},
        "quality_default": {"statement": "let _: DeclaredQualityAssumptions = Default::default();", "code": "E0277", "tokens": ["DeclaredQualityAssumptions", "Default"]},
        "quality_raw_conversion": {"statement": "let _: DeclaredFrequencyInterval = (1.0, -1.0).into();", "code": "E0277", "tokens": ["DeclaredFrequencyInterval", "From"]},
    }
    for specimen in examples.values():
        specimen["source"] = "\n".join(["use nevrotick_engine::*;", "fn main() -> Result<(), QualityRefusal> {",
            setup, specimen.pop("statement"), "Ok(())", "}"]) + "\n"
        specimen["line"] = 4
    return examples


def quality_validation_variants(source: str) -> dict[str, str]:
    """Omit exactly one actual constructor validation, leaving immutable originals intact."""
    variants = {}
    for name, owner in (("frequency", "DeclaredFrequencyInterval"), ("limits", "DeclaredQualityLimits"),
                        ("assumptions", "DeclaredQualityAssumptions")):
        owner_start = source.index("impl " + owner + " {")
        start = source.index("        candidate.validate().map_err(|_| {", owner_start)
        end = source.index("        Ok(candidate)", start)
        omitted = source[start:end]
        if omitted.count("candidate.validate()") != 1 or not omitted.rstrip().endswith("})?;"):
            raise Incomplete("Quality validation omission no longer matches its exact constructor guard.")
        variants[name] = source[:start] + source[end:]
    return variants


def quality_validation_contracts(session, report: dict, *, deadline: float) -> None:
    """Run actual validation-omission failures against sealed standalone source copies."""
    source = (session.source / QUALITY_SUBJECTS["quality"]).read_text()
    variants = quality_validation_variants(source)
    dependencies = session.work / "contract-deps"
    externs = []
    for package in ("garde", "time_domain"):
        matches = sorted(dependencies.glob("lib" + package + "-*.rlib"))
        if len(matches) != 1:
            raise Incomplete("Quality validation probes require one actual copied dependency artifact.")
        externs += ["--extern", package + "=/work/contract-deps/" + matches[0].name]
    directory = session.work / "quality-validation-inputs"
    directory.mkdir(mode=0o700)
    checks = {
        "frequency": "assert!(quality::DeclaredFrequencyInterval::new(1.0, -1.0).is_err(), \"Q1_VALIDATION_OMISSION:frequency\");",
        "limits": "assert!(quality::DeclaredQualityLimits::new(0.0, 1.0, 1.0).is_err(), \"Q1_VALIDATION_OMISSION:limits\");",
        "assumptions": "let prior = quality::DeclaredFrequencyInterval::new(0.0, 0.0)?; let limits = quality::DeclaredQualityLimits::new(1.0, 1.0, 1.0)?; assert!(quality::DeclaredQualityAssumptions::new(0.0, 0.0, 0.0, prior, limits).is_err(), \"Q1_VALIDATION_OMISSION:assumptions\");",
    }
    for name, body in {"healthy": source, **variants}.items():
        (directory / (name + "_module.rs")).write_text(body)
        statements = " ".join(checks.values()) if name == "healthy" else checks[name]
        (directory / (name + ".rs")).write_text(
            "use nevrotick_engine::{MeasurementRefusal, TransportFacts};\n"
            '#[path="' + name + '_module.rs"] mod quality;\n'
            "fn main() -> Result<(), quality::QualityRefusal> { " + statements
            + ' println!("Q1_VALIDATION_CONTROL:healthy"); Ok(()) }\n')
    session.protect(directory)
    for name in ("healthy", *variants):
        command = [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", "quality_validation_" + name,
            "/work/quality-validation-inputs/" + name + ".rs", "--extern",
            "nevrotick_engine=/work/contract-deps/libnevrotick_engine.rlib", *externs,
            "-L", "dependency=/work/contract-deps", "--out-dir", "/work/contracts"]
        session.run("quality-validation-compile:" + name, command, COMMON.remaining_timeout(deadline, 60),
                    sealed=[directory, dependencies])
        marker = None if name == "healthy" else "Q1_VALIDATION_OMISSION:" + name
        output = session.run("quality-validation-execute:" + name,
            ["/work/contracts/quality_validation_" + name], COMMON.remaining_timeout(deadline, 30),
            sealed=[directory, dependencies], negative=marker)
        if name == "healthy" and "Q1_VALIDATION_CONTROL:healthy" not in output:
            raise Incomplete("Quality healthy constructor validation companion did not execute.")
    report["quality_validation_controls"] = {"healthy_executed": 1, "intended_runtime_failures": list(variants),
        "source_sha256": PRODUCT.sha256(session.source / QUALITY_SUBJECTS["quality"]),
        "scope": "Three actual constructor validation omissions; no mutation score or producer qualification credit."}


def execute_quality_profile(args, report: dict) -> None:
    """Use the existing custodian for one frozen Q1 owner and all exact native controls."""
    if args.lane not in {"contracts", "enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Unknown quality analysis lane.")
    subjects = QUALITY_SCOPE.select(args.subject)
    policy = read_quality_policy(ROOT)
    if args.lane in {"mutation", "all"}:
        quality_inventory_bindings(policy, required=True)
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Quality analysis requires five GiB of available preflight space.")
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"
    report.update(subjects=subjects, candidate_sha256=policy["candidate_sha256"], qualification_passed=False,
        profile="quality", scope="Bounded host Q1 conditional quality " + args.lane,
        ordinary_evidence=admit_quality_ordinary(policy),
        generation_gap_controls="Separate function/Garde inventory and direct controls remain required; no score inferred.")
    session = COMMON.Session(ROOT, session_args, report, QUALITY_SCOPE, policy, QUALITY_BOUND_INPUTS)

    def run(label, command, timeout, **options):
        result = session.run(label, command, timeout, **options)
        if label == "coverage":
            report["coverage_execution"] = validate_quality_test_execution(result, policy, doctests=False)
        elif label.startswith("mutation-campaign:"):
            subject = label.removeprefix("mutation-campaign:")
            report.setdefault("baseline_execution", {})[subject] = validate_quality_baseline(session.work, subject, policy)
        return result

    try:
        if COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]:
            raise Incomplete("Captured quality Session differs from its admitted product candidate.")
        if any(PRODUCT.sha256(session.source / name) != digest for name, digest in policy["inputs_sha256"].items()):
            raise Incomplete("Captured quality protected source or helper bytes changed.")
        if COMMON.canonical_sha256(read_json(session.source / QUALITY_POLICY)) != COMMON.canonical_sha256(policy):
            raise Incomplete("Captured quality policy differs from admitted policy.")
        if args.lane in {"contracts", "all"}:
            contracts_deadline = time.monotonic() + policy["contracts_wall_seconds"]
            contracts(session, policy, report, examples=quality_contract_examples(),
                effect_scope="Explicit quality assumption constructors only; actual facts/result/borrow probes compile only. No clocks or network.",
                deadline=contracts_deadline)
            quality_validation_contracts(session, report, deadline=contracts_deadline)
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(QUALITY_SCOPE, subjects, policy, session.tools, run, report,
                deadline=time.monotonic() + policy["enumeration_wall_seconds"],
                maximum_per_file=policy["maximum_listed_per_file"], maximum_total=policy["maximum_listed_total"])
            report.update(listed_inventories=listing.inventories, listed_inventory_bindings=listed_summary(listing.inventories),
                listed_total=listing.total, campaign_cap_exceeded=listing.campaign_budget_exceeded, campaign_admission=False)
        elif args.lane != "contracts":
            for lane in (("coverage", "mutation") if args.lane == "all" else (args.lane,)):
                COMMON.run_analysis_lanes(QUALITY_SCOPE, subjects, policy, session.work, session.tools, run, report, lane)
    finally:
        session.finish()
        admit_quality_ordinary(policy)
    if args.lane in {"contracts", "all"}:
        report["contracts_qualification_passed"] = True
    if args.lane not in {"enumerate", "contracts"}:
        report["qualification_passed"] = True


def execute(args, report: dict) -> None:
    """Execute only the fixed selected lane, with full shared custody bookends."""
    profile = getattr(args, "profile", "legacy")
    if profile not in {"legacy", "series", "quality"}:
        raise Incomplete("Unknown engine analysis profile.")
    if profile == "quality":
        execute_quality_profile(args, report)
        return
    if profile == "series":
        execute_series_profile(args, report)
        return
    if args.lane == "contracts":
        execute_contracts(args, report)
        return
    if args.lane not in {"enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Unknown engine analysis lane.")
    policy = read_policy(ROOT)
    subjects = SCOPE.select(args.subject)
    if args.lane in {"mutation", "all"}:
        bindings = inventory_bindings(policy, required=True)
        if sum(bindings[name]["count"] for name in subjects) > policy["maximum_mutants"]:
            raise Incomplete("Selected complete mutation inventory exceeds the shared campaign cap.")
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"  # Select only the admitted extension, not campaign execution.
    report.update(subjects=subjects, candidate_sha256=policy["candidate_sha256"],
                  qualification_passed=False, readiness_proposal=readiness_proposal())
    session = COMMON.Session(ROOT, session_args, report, SCOPE, policy, (WRAPPER, TESTS, *BOUND_INPUTS))

    def run(label, command, timeout, **options):
        result = session.run(label, command, timeout, **options)
        if label == "coverage":
            report["coverage_execution"] = validate_test_execution(result, doctests=False)
        elif label.startswith("mutation-campaign:"):
            subject = label.removeprefix("mutation-campaign:")
            report.setdefault("baseline_execution", {})[subject] = validate_baseline(session.work, subject)
        return result

    try:
        if COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]:
            raise Incomplete("Captured engine Session differs from the policy's product candidate.")
        if any(PRODUCT.sha256(session.source / name) != policy["inputs_sha256"][name]
               for name in BOUND_INPUTS):
            raise Incomplete("Captured engine Session protected input pins changed.")
        if COMMON.canonical_sha256(read_json(session.source / POLICY)) != COMMON.canonical_sha256(policy):
            raise Incomplete("Captured engine Session policy differs from the admitted policy.")
        if args.lane == "enumerate":
            enumerate_only(session, subjects, policy, report)
        else:
            # Share the exact source/tool snapshot and final custody boundary.
            # A failed coverage gate never starts a mutation campaign.
            lanes = ("coverage", "mutation") if args.lane == "all" else (args.lane,)
            for lane in lanes:
                COMMON.run_analysis_lanes(SCOPE, subjects, policy, session.work, session.tools, run, report, lane)
    finally:
        session.finish()
    if args.lane != "enumerate":
        report["qualification_passed"] = True


def main(argv=None) -> int:
    """Keep listing completion distinct from selected host coverage/mutation gates."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--profile", choices=("legacy", "series", "quality"), default="legacy")
    parser.add_argument("--lane", choices=("contracts", "enumerate", "coverage", "mutation", "all"), required=True)
    parser.add_argument("--subject", choices=(*SUBJECTS, "series", "quality", "all"), default="all")
    parser.add_argument("--tool-admission-profile", choices=("full", "stable-core"), default="full")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    if args.profile == "legacy" and args.subject == "series":
        parser.error("--subject series requires --profile series")
    if args.profile == "legacy" and args.subject == "quality":
        parser.error("--subject quality requires --profile quality")
    if args.profile == "series" and args.subject not in {*SERIES_SUBJECTS, "all"}:
        parser.error("--profile series selects only library, series or all")
    if args.profile == "quality" and args.subject not in {"quality", "all"}:
        parser.error("--profile quality selects only quality or all")
    report = {"status": "incomplete", "scope": "Bounded host engine " + args.lane,
              "selected_lane": args.lane, "qualification_passed": False,
              "native_windows_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(read_json(PRODUCT.POINTER)["vendor_dir"])
        execute(args, report)
        report["status"] = "enumerated" if args.lane == "enumerate" else "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["qualification_passed"] = False
        report["contracts_qualification_passed"] = False
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] in {"enumerated", "passed"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
