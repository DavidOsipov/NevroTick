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
import os
from pathlib import Path
import re
import shutil
import stat
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
    "package": "nevrotick-engine", **COMMON.CONTRACT_PRODUCT_BINDING,
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


def read_json(path: Path, *, maximum: int = 524288, expected_sha256: str | None = None,
              owned_evidence: bool = False) -> dict:
    """Read bounded JSON with the selected file custody; reject duplicates and nonfinite values."""
    if owned_evidence:
        data = COMMON.read_analysis_bytes(path, maximum=maximum)
    else:
        if path != path.resolve(strict=True):
            raise Incomplete("Engine JSON input cannot use a linked path.")
        data = COMMON.ANALYSIS.read_once(path, maximum=maximum)
    if expected_sha256 is not None and (
            type(expected_sha256) is not str
            or re.fullmatch(r"[a-f0-9]{64}", expected_sha256) is None
            or hashlib.sha256(data).hexdigest() != expected_sha256):
        raise Incomplete("Engine JSON snapshot bytes differ from their exact binding.")
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
    manifest = tomllib.loads(COMMON.ANALYSIS.read_once(
        root / "policy/verification-manifest.toml", maximum=524288).decode())
    filename = policy["critical_files"][0]
    if (manifest.get("critical_files", []).count(filename) != 1
            or sum(module.get("package") == CONTRACT_SCOPE.package and filename in module.get("files", [])
                   for module in manifest.get("security_modules", [])) != 1):
        raise Incomplete("Engine constructor owner lacks its exact critical/security registration.")
    COMMON.require_contract_candidate(PRODUCT.admit_product(root)[1], policy)
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


def copy_contract_dependencies(session, artifact: str, *, cargo_output: str) -> Path:
    """Seal only complete libraries attributed to the actual Cargo build."""
    return COMMON.copy_cargo_contract_dependencies(
        session, cargo_output, {"libnevrotick_engine.rlib": artifact})


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
    dependencies = copy_contract_dependencies(session, artifact, cargo_output=output)
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


# Singleton persistent-session qualification. Ordinary execution must freeze this
# binding before any role lane is admitted; no historical Q1/N3 result transfers.
SESSION_POLICY = "policy/session-analysis.json"
SESSION_TESTS = "scripts/tests/test_session_analysis.py"
SESSION_FILE = "crates/nevrotick-engine/src/session.rs"
SESSION_PRIVATE = "crates/nevrotick-engine/tests/private/session.rs"
SESSION_EXTERNAL = "crates/nevrotick-engine/tests/session.rs"
SESSION_SUBJECTS = {
    "session": SESSION_FILE,
    "series": "crates/nevrotick-engine/src/series.rs",
    "measurement": "crates/nevrotick-engine/src/measurement.rs",
}
SESSION_SCOPE = COMMON.Scope("nevrotick-engine", tuple(SESSION_SUBJECTS.items()), SESSION_POLICY)
SESSION_WINDOWS_CONTROL = "policy/measurement-coherence-windows-control.rs"
SESSION_EQUIVALENCE_PROOF = "tasks/engine-analysis-equivalences.md"
# Full native identities exclude the rendered diff; no historical cohort transfers.
SESSION_EQUIVALENCE_DESCRIPTORS = {
    "ENGINE-SESSION-SERIES-UNAVAILABLE-ADMITTED-ZERO": (
        SESSION_SUBJECTS["series"], "3444eeb5e950c77ac738bbb268ef43e2dd68b6984adac1454e2d8754b174da7c",
        "8b3ff21dbdb4481bd500c4a6afc55933af64e5f954bc7e41824f453aef84e8af"),
    "ENGINE-SESSION-SERIES-UNAVAILABLE-ELIGIBLE-ZERO": (
        SESSION_SUBJECTS["series"], "3444eeb5e950c77ac738bbb268ef43e2dd68b6984adac1454e2d8754b174da7c",
        "1bb74259a1ad856d2c4d85150a1fb75787ec932cc116910bcf3bf1c8ce90f2ef"),
    "ENGINE-SESSION-COHERENCE-LOWER-EQUALITY": (
        SESSION_SUBJECTS["measurement"], "0b97439bbb6b6e83db74b046a7140b0030e3c97b7aa69b67423f60c454889ff7",
        "bd833b6be02ec7ea0e551202bc0ba3982d6a587c5ae694d6d504030a95065e0d"),
    "ENGINE-SESSION-COHERENCE-UPPER-EQUALITY": (
        SESSION_SUBJECTS["measurement"], "0b97439bbb6b6e83db74b046a7140b0030e3c97b7aa69b67423f60c454889ff7",
        "da36761fb7f15b660253819ecfd4d29a868bf422d9a54ce49f845331bcd33dcc"),
    "ENGINE-SESSION-NOMINAL-ZERO-SIGN": (
        SESSION_SUBJECTS["measurement"], "0b97439bbb6b6e83db74b046a7140b0030e3c97b7aa69b67423f60c454889ff7",
        "85e4e17d40dbe6e7a5ed1e147a2caed88565d9e49f8255ce33249134ce321a3e"),
}
# Supplemental executions never revise or credit a rejected full-suite log.
# These are the six reviewed, already-generated Series faults, not a retry filter.
SESSION_WITNESS_COUNTER_TEST = "series::tests::synchronous_counter_boundaries_do_not_panic_and_preserve_exact_effects"
SESSION_WITNESS_CASES = {
    "crates/nevrotick-engine/src/series.rs:171:25: replace == with != in token_issue": (
        "057546eea791b5f3617793730eab05355a6d867621f1f864a60d6d5801966a16",
        "series::tests::raw_config_and_tokens_have_mandatory_bounded_admission"),
    "crates/nevrotick-engine/src/series.rs:171:51: replace == with != in token_issue": (
        "88218aa722db50cb2cb49acf67c7f418161f438be0a91e6bd92dd59e7431c8b6",
        "series::tests::raw_config_and_tokens_have_mandatory_bounded_admission"),
    "crates/nevrotick-engine/src/series.rs:171:77: replace == with != in token_issue": (
        "490d828a33a2e9be16c8bff75cb19fd802c3731c6cbc75bc85df6d783741e074",
        "series::tests::raw_config_and_tokens_have_mandatory_bounded_admission"),
    "crates/nevrotick-engine/src/series.rs:743:12: replace >= with < in refresh_slot": (
        "c1d19da02aaf6b5616706445ade365a706739078249d8fea28501211b87a432b",
        "series::tests::actual_publication_rechecks_control_and_deadline_after_staging"),
    "crates/nevrotick-engine/src/series.rs:821:27: replace += with -= in publish_series_reply": (
        "415b333e2c60d8faf007e76474451d5128c39007624c2defbfa41e38ac9befc8",
        SESSION_WITNESS_COUNTER_TEST),
    "crates/nevrotick-engine/src/series.rs:843:27: replace += with -= in handle_series_read": (
        "f52338e9e5f8bdb1327d5515541705f58e2836d42aeddca6057178c939bf7966",
        SESSION_WITNESS_COUNTER_TEST),
}
SESSION_WITNESS_LIMITS = {"maximum_cases": 6, "case_wall_seconds": 120,
                          "total_wall_seconds": 360, "jobs": 1, "test_threads": 1}
# Coordinator binds only after the final ordinary candidate/receipt exists.
SESSION_WITNESS_ADMISSION = {'maximum_cases': 6,
 'case_wall_seconds': 120,
 'total_wall_seconds': 360,
 'jobs': 1,
 'test_threads': 1,
 'candidate_sha256': '6d4cc1560b7974129d75462cdb7b46411f97784bf8442ac64b88896468a5acb4',
 'ordinary_receipt_sha256': 'fc6bf6fddef8cb47e168c62aa6a4f1a2e41853c8759c26c74e7536491629d0ec',
 'cases': {'crates/nevrotick-engine/src/series.rs:171:25: replace == with != in token_issue': {'descriptor_sha256': '057546eea791b5f3617793730eab05355a6d867621f1f864a60d6d5801966a16',
                                                                                               'test': 'series::tests::raw_config_and_tokens_have_mandatory_bounded_admission'},
           'crates/nevrotick-engine/src/series.rs:171:51: replace == with != in token_issue': {'descriptor_sha256': '88218aa722db50cb2cb49acf67c7f418161f438be0a91e6bd92dd59e7431c8b6',
                                                                                               'test': 'series::tests::raw_config_and_tokens_have_mandatory_bounded_admission'},
           'crates/nevrotick-engine/src/series.rs:171:77: replace == with != in token_issue': {'descriptor_sha256': '490d828a33a2e9be16c8bff75cb19fd802c3731c6cbc75bc85df6d783741e074',
                                                                                               'test': 'series::tests::raw_config_and_tokens_have_mandatory_bounded_admission'},
           'crates/nevrotick-engine/src/series.rs:743:12: replace >= with < in refresh_slot': {'descriptor_sha256': 'c1d19da02aaf6b5616706445ade365a706739078249d8fea28501211b87a432b',
                                                                                               'test': 'series::tests::actual_publication_rechecks_control_and_deadline_after_staging'},
           'crates/nevrotick-engine/src/series.rs:821:27: replace += with -= in publish_series_reply': {'descriptor_sha256': '415b333e2c60d8faf007e76474451d5128c39007624c2defbfa41e38ac9befc8',
                                                                                                        'test': 'series::tests::synchronous_counter_boundaries_do_not_panic_and_preserve_exact_effects'},
           'crates/nevrotick-engine/src/series.rs:843:27: replace += with -= in handle_series_read': {'descriptor_sha256': 'f52338e9e5f8bdb1327d5515541705f58e2836d42aeddca6057178c939bf7966',
                                                                                                      'test': 'series::tests::synchronous_counter_boundaries_do_not_panic_and_preserve_exact_effects'}}}
SESSION_BOUND_INPUTS = tuple(dict.fromkeys((
    *BOUND_INPUTS, WRAPPER, SESSION_TESTS, "rust-toolchain.toml", ".cargo/mutants.toml",
    "scripts/analysis-controls.py",
    *SESSION_SUBJECTS.values(), SESSION_PRIVATE, SESSION_EXTERNAL, SERIES_PROCESS_SOURCE,
    "crates/nevrotick-engine/tests/private/series.rs", "crates/nevrotick-engine/tests/series.rs",
    "crates/nevrotick-engine/src/quality/tests.rs", "crates/nevrotick-engine/tests/quality.rs",
    SESSION_WINDOWS_CONTROL, SESSION_EQUIVALENCE_PROOF,
)))
SESSION_FIXED_POLICY = {
    **{key: value for key, value in SERIES_FIXED_POLICY.items()
       if key not in {"source_file_count", "critical_files", "admission_status"}},
    "admission_status": "admitted-acquisition-session-source",
    "critical_files": list(SESSION_SUBJECTS.values()), "mutation_equivalences": [],
}
# Bound only after the coordinator supplies the actual completed ordinary receipt.
# Keys: candidate_sha256, source_file_count, ordinary_evidence, test_inventory,
# semantic_tests. The latter maps exact executed names to source + helper names.
SESSION_ADMISSION = {'candidate_sha256': '6d4cc1560b7974129d75462cdb7b46411f97784bf8442ac64b88896468a5acb4',
 'source_file_count': 424,
 'ordinary_evidence': {'receipt': '/home/david_osipov/.local/share/nevrotick/acquisition-session-20261108-v1/coherence-ordinary-v10/verification.json',
                       'sha256': 'fc6bf6fddef8cb47e168c62aa6a4f1a2e41853c8759c26c74e7536491629d0ec',
                       'status': 'passed-session-all',
                       'recipes_sha256': {'/home/david_osipov/.local/share/nevrotick/acquisition-session-20261108-v1/coherence_ordinary_v3.py': '7240b69ce85cc92e27eba2e26ecfa6b33323e0974a3a7e1904f2c205cd1e8c6b',
                                          '/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/relative_bridge_ordinary.py': '937709d9bc11054253b2e0cd9ad05357792f81bcb6e1ff88f997ac8f2a8292a9',
                                          '/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/conditioned_fit_ordinary_v2.py': '3062bb0bb1869161e273ad4e80b238a4d7ef94661a21ffa4127341832a9356d6',
                                          '/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/series_ordinary_v1.py': 'd3ab16cc3e08a03f950808cdaeb570dc86ba4aefec4a1c6bf9c0810631e64cc2',
                                          '/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/series_ordinary_v3.py': '17574af8bb9e4b41b3f55337531a82e2c0a2a275741f255b3b3cea226002e568',
                                          '/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/quality_ordinary_v1.py': 'bb903f4bd09b6e59ccc956f4de582eb9b421fa9d49ba926920318fefdda0fc08',
                                          '/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/quality_ordinary_v2.py': '717cc1902e7c01194967309375d8a1660910276ba4045e7efbe81fe9af743a1b',
                                          '/home/david_osipov/.local/share/nevrotick/acquisition-session-20261108-v1/session_ordinary_v1.py': '3e666c8a81b422d4b8df3a42f387c5be6970de753d18f99838d3430aaa3b7052',
                                          '/home/david_osipov/.local/share/nevrotick/acquisition-session-20261108-v1/coherence_ordinary_v1.py': 'c18aeb7449021933a7b5170e75168710bd4714e55e9b6c71d5722154f27f1a7e',
                                          '/home/david_osipov/.local/share/nevrotick/acquisition-session-20261108-v1/coherence_ordinary_v2.py': '90756105680c4dd0214449c4fdcd2547524f3c3dca57aabde80665cf29177ec5'}},
 'test_inventory': {'ordinary': ['absent_measurement_refuses_quality_with_actual_valid_neighbor',
                                 'actual_quality_limits_refuse_without_rewriting_completion_or_age',
                                 'actual_series_quality_retains_signed_intervals_source_and_original_age',
                                 'argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows',
                                 'argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles',
                                 'argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries',
                                 'argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure',
                                 'clean_stop_and_real_deadline_each_allow_a_fresh_preparation',
                                 'cli_worker_binary_preserves_literal_transport_contracts',
                                 'cli_worker_binary_rejects_arguments_and_observes_sink_failures',
                                 'execute_rejects_entered_runtime_without_nested_runtime_creation',
                                 'invalidation_preserves_first_completion_and_marks_third_not_started',
                                 'ipv6_real_requests_retain_distinct_identity_and_one_exact_origin',
                                 'measurement::tests::absent_profile_is_a_distinct_transport_only_reason',
                                 'measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values',
                                 'measurement::tests::coherence_helper_crossed_brackets_keep_touching_and_separated_neighbors',
                                 'measurement::tests::coherence_helper_rejects_each_reversed_bracket',
                                 'measurement::tests::decimal_remainders_change_the_exact_nominal_phase',
                                 'measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor',
                                 'measurement::tests::era_zero_and_multiple_candidates_remain_distinct',
                                 'measurement::tests::exact_dyadic_results_ignore_the_wire_token',
                                 'measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values',
                                 'measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic',
                                 'measurement::tests::inert_header_does_not_bypass_the_time_reply_guard',
                                 'measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved',
                                 'measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result',
                                 'measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection',
                                 'measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds',
                                 'measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint',
                                 'measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt',
                                 'measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors',
                                 'measurement::tests::out_of_scope_local_coordinate_is_not_rebased',
                                 'measurement::tests::peer_shift_changes_sign_without_changing_rtt',
                                 'measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals',
                                 'measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association',
                                 'measurement::tests::profile_bounds_and_maximum_allowance_are_checked',
                                 'measurement::tests::refusal_codes_and_display_remain_literal_and_redacted',
                                 'measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding',
                                 'measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign',
                                 'measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks',
                                 'measurement::tests::reversed_brackets_and_processing_are_not_ages',
                                 'measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints',
                                 'measurement::tests::series_coherence_all_nine_positions_influence_the_maximum',
                                 'measurement::tests::series_coherence_empty_single_and_holes_keep_original_positions',
                                 'measurement::tests::series_coherence_fixed_first_baseline_detects_accumulated_drift',
                                 'measurement::tests::series_coherence_full_order_validation_precedes_earlier_arithmetic',
                                 'measurement::tests::series_coherence_full_u64_residual_keeps_its_next_rejection',
                                 'measurement::tests::series_coherence_late_arithmetic_failure_wins_over_an_earlier_gap',
                                 'measurement::tests::series_coherence_rejects_each_own_and_sequential_order_violation',
                                 'measurement::tests::software_window_retains_each_local_and_peer_extremum',
                                 'middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt',
                                 'prepare_outside_runtime_owns_and_releases_standard_sockets',
                                 'prepare_rejects_entered_runtime_before_native_startup',
                                 'processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor',
                                 'processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection',
                                 'processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error',
                                 'processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history',
                                 'processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries',
                                 'processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays',
                                 'processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes',
                                 'processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections',
                                 'processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor',
                                 'processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port',
                                 'processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once',
                                 'processing_guard_tests::observed_measurement_prerequisites_keep_broad_date_and_model_boundaries',
                                 'processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions',
                                 'processing_guard_tests::processing_uses_fresh_deadline_and_current_control',
                                 'processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals',
                                 'processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct',
                                 'processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates',
                                 'processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four',
                                 'processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize',
                                 'processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice',
                                 'processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct',
                                 'processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap',
                                 'quality::tests::computed_policy_boundaries_preserve_strict_and_inclusive_comparisons',
                                 'quality::tests::declared_getters_and_constructor_precedence_preserve_original_bits',
                                 'quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions',
                                 'quality::tests::evaluator_failure_precedence_preserves_original_observation',
                                 'quality::tests::finite_input_overflow_is_refused_at_the_first_computed_stage',
                                 'quality::tests::generated_garde_dispatch_covers_nested_and_computed_fields',
                                 'quality::tests::gradual_underflow_does_not_claim_exact_zero_uncertainty',
                                 'quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields',
                                 'quality::tests::negative_local_wall_interval_grows_declared_frequency_dispersion',
                                 'quality::tests::negative_server_interval_processing_limit_uses_absolute_magnitude',
                                 'quality::tests::original_measurement_refusal_and_redacted_diagnostics_are_preserved',
                                 'quality::tests::precision_clamps_have_exact_boundary_and_neighbor_values',
                                 'quality::tests::raw_round_trip_is_converted_once_before_cancellation',
                                 'quality::tests::raw_scalar_and_interval_helpers_preserve_full_admitted_domains',
                                 'quality::tests::refusal_families_keep_bounded_diagnostics_and_typed_sources',
                                 'quality::tests::signed_server_interval_and_unsigned_root_words_are_retained',
                                 'quality::tests::zero_raw_rtt_uses_combined_precision_floor',
                                 'relative::tests::duration_nanoseconds_admits_full_u64_and_rejects_next_tick',
                                 'relative::tests::midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints',
                                 'relative::tests::midpoint_publication_enforces_raw_association_and_garde_rule',
                                 'relative::tests::reference_instants_check_order_before_duration_conversion',
                                 'relative::tests::relative_diagnostics_preserve_fixed_text_and_original_causes',
                                 'relative::tests::relative_phase_raw_i128_endpoints_keep_u128_magnitude',
                                 'relative::tests::relative_reference_differences_keep_direction_and_half_ticks',
                                 'relative::tests::relative_seconds_scale_requires_nonzero_with_healthy_neighbors',
                                 'relative::tests::shared_origin_distinguishes_equal_values_from_run_identity',
                                 'series::tests::actual_publication_rechecks_control_and_deadline_after_staging',
                                 'series::tests::all_series_debug_outputs_are_exact_fixed_redactions',
                                 'series::tests::checkpoint_capture_prerequisites_keep_boundary_and_invalid_observations_distinct',
                                 'series::tests::checkpoint_failure_taxonomy_keeps_native_context_uncredited',
                                 'series::tests::current_checkpoint_observes_new_controls_before_expired_deadline',
                                 'series::tests::deadlines_and_controls_do_not_reset_the_series_origin',
                                 'series::tests::entropy_failure_and_entered_runtime_precede_all_acquisition_work',
                                 'series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions',
                                 'series::tests::genuine_terminal_report_admits_only_the_three_position_domain',
                                 'series::tests::partial_report_preserves_primary_and_both_cleanup_causes',
                                 'series::tests::post_commit_control_preserves_header_and_measurement_refusal',
                                 'series::tests::raw_config_and_tokens_have_mandatory_bounded_admission',
                                 'series::tests::receiver_rejects_duplicate_foreign_and_reused_identity_atomically',
                                 'series::tests::selected_native_datagram_after_retirement_is_charged_without_publication',
                                 'series::tests::series_process::semantic_evidence_accepts_only_exact_named_admitted_sites',
                                 'series::tests::setup_and_registration_failures_preserve_actual_prior_completion',
                                 'series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor',
                                 'series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor',
                                 'series::tests::synchronous_counter_boundaries_do_not_panic_and_preserve_exact_effects',
                                 'series::tests::transfer_control_and_irreversible_retirement_are_distinct',
                                 'series_process::semantic_evidence_accepts_only_exact_named_admitted_sites',
                                 'session::tests::completion_classification_covers_all_end_and_failure_combinations',
                                 'session::tests::entered_runtime_refusals_preserve_prepare_rollback_and_retire_consumed_execution',
                                 'session::tests::equal_origin_foreign_owners_produce_distinct_genuine_report_memberships',
                                 'session::tests::forgotten_plan_keeps_busy_until_idempotent_terminal_retirement',
                                 'session::tests::private_busy_and_retired_states_reject_before_driver_preparation',
                                 'session::tests::private_drop_contradictions_retire_without_entering_execution',
                                 'session::tests::private_plan_state_chart_and_missing_inner_contradictions_retire',
                                 'session::tests::reference_refusal_diagnostics_and_exact_typed_sources_are_fixed',
                                 'session::tests::series_coherence_debug_contains_only_fixed_scalar_causes',
                                 'session::tests::session_debug_and_driver_error_sources_are_fixed_and_typed',
                                 'session::tests::synthetic_capture_coherence_preserves_nonzero_limit_and_checked_failure_precedence',
                                 'session::tests::synthetic_execution_unwind_retires_and_releases_prepared_resources',
                                 'session::tests::synthetic_started_guard_drop_retires_without_reopening_capacity',
                                 'session::tests::unused_plan_drop_releases_capacity_without_replacing_the_chart',
                                 'session_reference_refusals_preserve_original_causes_and_checked_positions',
                                 'stale_execution_retires_owner_without_refreshing_retained_facts',
                                 'stop_preserves_first_completion_and_marks_third_not_started',
                                 'three_real_requests_retain_distinct_identity_and_one_exact_origin',
                                 'total_deadline_preserves_first_completion_and_prevents_third_send',
                                 'two_requested_series_share_chart_keep_distinct_runs_and_old_handles_isolated',
                                 'unavailable_measurements_still_consume_once_after_rejected_neighbors'],
                    'doctests': ['crates/nevrotick-engine/src/lib.rs - Config::new (line 199)',
                                 'crates/nevrotick-engine/src/lib.rs - ControlSchedule::new (line 1588)',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalMeasurement (line 216)',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalMeasurement (line 222) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalMeasurement (line 229) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalMeasurement (line 237) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalMeasurement (line 242) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalProfile (line 104) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalProfile (line 117) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalProfile (line 126) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalProfile (line 131) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::ExperimentalProfile (line 94)',
                                 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs '
                                 '(line 655)',
                                 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs '
                                 '(line 663) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs '
                                 '(line 668) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs '
                                 '(line 676) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs '
                                 '(line 681) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::NominalRoundTripNs (line 725)',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::NominalRoundTripNs (line 733) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::NominalRoundTripNs (line 738) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::NominalRoundTripNs (line 746) - compile fail',
                                 'crates/nevrotick-engine/src/measurement.rs - '
                                 'measurement::NominalRoundTripNs (line 751) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line '
                                 '749) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line '
                                 '755) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - quality::ConditionalQuality (line '
                                 '760) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - '
                                 'quality::DeclaredFrequencyInterval (line 255) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - '
                                 'quality::DeclaredQualityAssumptions (line 389)',
                                 'crates/nevrotick-engine/src/quality.rs - '
                                 'quality::DeclaredQualityAssumptions (line 401) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - '
                                 'quality::DeclaredQualityAssumptions (line 405) - compile fail',
                                 'crates/nevrotick-engine/src/quality.rs - quality::DeclaredQualityLimits '
                                 '(line 305) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalReferenceNs (line 101) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalReferenceNs (line 106) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalReferenceNs (line 111) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalReferenceNs (line 116) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalReferenceNs (line 92)',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalRelativeMeasurement (line 157)',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalRelativeMeasurement (line 169) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalRelativeMeasurement (line 185) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalRelativeMeasurement (line 192) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalRelativeMeasurement (line 197) - compile fail',
                                 'crates/nevrotick-engine/src/relative.rs - '
                                 'relative::ExperimentalRelativeMeasurement (line 204) - compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::PreparedSeries (line 216) '
                                 '- compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::PreparedSeries (line 223) '
                                 '- compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::SeriesAttempt (line 343) - '
                                 'compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 81) - '
                                 'compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 87) - '
                                 'compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig (line 91) - '
                                 'compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::SeriesConfig::new (line '
                                 '111)',
                                 'crates/nevrotick-engine/src/series.rs - series::SeriesReport (line 407) - '
                                 'compile fail',
                                 'crates/nevrotick-engine/src/series.rs - series::execute_series (line 1078) '
                                 '- compile',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionReference '
                                 '(line 600)',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionReference '
                                 '(line 609) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionReference '
                                 '(line 619) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionReference '
                                 '(line 626) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionReport (line '
                                 '438) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionReport (line '
                                 '444) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - '
                                 'session::AcquisitionReport::coherence (line 470) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionSession (line '
                                 '112) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionSession (line '
                                 '118) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionSession (line '
                                 '124) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::AcquisitionSession (line '
                                 '94)',
                                 'crates/nevrotick-engine/src/session.rs - session::PreparedAcquisition '
                                 '(line 205) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::PreparedAcquisition '
                                 '(line 213) - compile fail',
                                 'crates/nevrotick-engine/src/session.rs - session::PreparedAcquisition '
                                 '(line 222) - compile fail'],
                    'sha256': {'ordinary': '47f7924a9bd5c8eb79e33a207afb6cf8a7114bd1dd03a357eb69db555ce72120',
                               'doctests': 'c0f44db972f2395731f503278e53cdd6bb5981beecb28efde85db11d2f9ef2ac',
                               'baseline': 'eadd3c3ad17cf72670d8f3e084fd12860cfa685ba670a1e0793a23a4ddbcad73'}},
 'semantic_tests': {'absent_measurement_refuses_quality_with_actual_valid_neighbor': {'source': 'crates/nevrotick-engine/tests/quality.rs',
                                                                                      'helpers': ['assumptions',
                                                                                                  'check_near',
                                                                                                  'check_quality',
                                                                                                  'check_ratio',
                                                                                                  'check_report',
                                                                                                  'diagnostic',
                                                                                                  'facts',
                                                                                                  'product_ok',
                                                                                                  'run_series',
                                                                                                  'serve',
                                                                                                  'slot']},
                    'actual_quality_limits_refuse_without_rewriting_completion_or_age': {'source': 'crates/nevrotick-engine/tests/quality.rs',
                                                                                         'helpers': ['assumptions',
                                                                                                     'check_near',
                                                                                                     'check_quality',
                                                                                                     'check_ratio',
                                                                                                     'check_report',
                                                                                                     'diagnostic',
                                                                                                     'facts',
                                                                                                     'product_ok',
                                                                                                     'run_series',
                                                                                                     'serve',
                                                                                                     'slot']},
                    'actual_series_quality_retains_signed_intervals_source_and_original_age': {'source': 'crates/nevrotick-engine/tests/quality.rs',
                                                                                               'helpers': ['assumptions',
                                                                                                           'check_near',
                                                                                                           'check_quality',
                                                                                                           'check_ratio',
                                                                                                           'check_report',
                                                                                                           'diagnostic',
                                                                                                           'facts',
                                                                                                           'product_ok',
                                                                                                           'run_series',
                                                                                                           'serve',
                                                                                                           'slot']},
                    'argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows': {'source': 'crates/nevrotick-engine/tests/private/argument_guards.rs',
                                                                                                                 'helpers': ['product_ok']},
                    'argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles': {'source': 'crates/nevrotick-engine/tests/private/argument_guards.rs',
                                                                                               'helpers': ['product_ok']},
                    'argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries': {'source': 'crates/nevrotick-engine/tests/private/argument_guards.rs',
                                                                                                   'helpers': ['product_ok']},
                    'argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure': {'source': 'crates/nevrotick-engine/tests/private/argument_guards.rs',
                                                                                                     'helpers': ['product_ok']},
                    'clean_stop_and_real_deadline_each_allow_a_fresh_preparation': {'source': 'crates/nevrotick-engine/tests/session.rs',
                                                                                    'helpers': ['actual_report',
                                                                                                'checked_measurement',
                                                                                                'checked_reference',
                                                                                                'config',
                                                                                                'fact',
                                                                                                'idle_plan',
                                                                                                'inspect',
                                                                                                'inspect_reference',
                                                                                                'product_ok',
                                                                                                'run_three',
                                                                                                'snapshots',
                                                                                                'unchanged']},
                    'cli_worker_binary_preserves_literal_transport_contracts': {'source': 'crates/nevrotick-engine/tests/runtime_boundary.rs',
                                                                                'helpers': ['argument_rejection',
                                                                                            'bounded_fields',
                                                                                            'cli_argument_boundary_rejections',
                                                                                            'cli_case',
                                                                                            'experimental_argument_rejections',
                                                                                            'fields',
                                                                                            'integer',
                                                                                            'measurement_arithmetic',
                                                                                            'measurement_record',
                                                                                            'measurement_transcript_controls',
                                                                                            'non_text_arguments',
                                                                                            'product_ok',
                                                                                            'result_record',
                                                                                            'signed_integer',
                                                                                            'timeout_elapsed_controls',
                                                                                            'transcript',
                                                                                            'transcript_controls']},
                    'cli_worker_binary_rejects_arguments_and_observes_sink_failures': {'source': 'crates/nevrotick-engine/tests/runtime_boundary.rs',
                                                                                       'helpers': ['argument_rejection',
                                                                                                   'bounded_fields',
                                                                                                   'cli_argument_boundary_rejections',
                                                                                                   'cli_case',
                                                                                                   'experimental_argument_rejections',
                                                                                                   'fields',
                                                                                                   'integer',
                                                                                                   'measurement_arithmetic',
                                                                                                   'measurement_record',
                                                                                                   'measurement_transcript_controls',
                                                                                                   'non_text_arguments',
                                                                                                   'product_ok',
                                                                                                   'result_record',
                                                                                                   'signed_integer',
                                                                                                   'timeout_elapsed_controls',
                                                                                                   'transcript',
                                                                                                   'transcript_controls']},
                    'invalidation_preserves_first_completion_and_marks_third_not_started': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                            'helpers': ['check_exact_measurement',
                                                                                                        'check_ratio',
                                                                                                        'common_report_checks',
                                                                                                        'facts',
                                                                                                        'healthy_report_checks',
                                                                                                        'no_extra_query',
                                                                                                        'product_ok',
                                                                                                        'profile',
                                                                                                        'run_bound_case',
                                                                                                        'run_case',
                                                                                                        'serve',
                                                                                                        'slot',
                                                                                                        'terminal_case']},
                    'ipv6_real_requests_retain_distinct_identity_and_one_exact_origin': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                         'helpers': ['check_exact_measurement',
                                                                                                     'check_ratio',
                                                                                                     'common_report_checks',
                                                                                                     'facts',
                                                                                                     'healthy_report_checks',
                                                                                                     'no_extra_query',
                                                                                                     'product_ok',
                                                                                                     'profile',
                                                                                                     'run_bound_case',
                                                                                                     'run_case',
                                                                                                     'serve',
                                                                                                     'slot',
                                                                                                     'terminal_case']},
                    'measurement::tests::absent_profile_is_a_distinct_transport_only_reason': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                               'helpers': ['assert_relative_phase',
                                                                                                           'nominal_relative_fixture',
                                                                                                           'product_ok']},
                    'measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                         'helpers': ['assert_relative_phase',
                                                                                                                     'nominal_relative_fixture',
                                                                                                                     'product_ok']},
                    'measurement::tests::coherence_helper_crossed_brackets_keep_touching_and_separated_neighbors': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                                    'helpers': ['assert_relative_phase',
                                                                                                                                'nominal_relative_fixture',
                                                                                                                                'product_ok']},
                    'measurement::tests::coherence_helper_rejects_each_reversed_bracket': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                           'helpers': ['assert_relative_phase',
                                                                                                       'nominal_relative_fixture',
                                                                                                       'product_ok']},
                    'measurement::tests::decimal_remainders_change_the_exact_nominal_phase': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                              'helpers': ['assert_relative_phase',
                                                                                                          'nominal_relative_fixture',
                                                                                                          'product_ok']},
                    'measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                   'helpers': ['assert_relative_phase',
                                                                                                               'nominal_relative_fixture',
                                                                                                               'product_ok']},
                    'measurement::tests::era_zero_and_multiple_candidates_remain_distinct': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                             'helpers': ['assert_relative_phase',
                                                                                                         'nominal_relative_fixture',
                                                                                                         'product_ok']},
                    'measurement::tests::exact_dyadic_results_ignore_the_wire_token': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                       'helpers': ['assert_relative_phase',
                                                                                                   'nominal_relative_fixture',
                                                                                                   'product_ok']},
                    'measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                       'helpers': ['assert_relative_phase',
                                                                                                                   'nominal_relative_fixture',
                                                                                                                   'product_ok']},
                    'measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                  'helpers': ['assert_relative_phase',
                                                                                                              'nominal_relative_fixture',
                                                                                                              'product_ok']},
                    'measurement::tests::inert_header_does_not_bypass_the_time_reply_guard': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                              'helpers': ['assert_relative_phase',
                                                                                                          'nominal_relative_fixture',
                                                                                                          'product_ok']},
                    'measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                              'helpers': ['assert_relative_phase',
                                                                                                          'nominal_relative_fixture',
                                                                                                          'product_ok']},
                    'measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                                      'helpers': ['assert_relative_phase',
                                                                                                                                  'nominal_relative_fixture',
                                                                                                                                  'product_ok']},
                    'measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                                     'helpers': ['assert_relative_phase',
                                                                                                                                 'nominal_relative_fixture',
                                                                                                                                 'product_ok']},
                    'measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                       'helpers': ['assert_relative_phase',
                                                                                                                   'nominal_relative_fixture',
                                                                                                                   'product_ok']},
                    'measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                                 'helpers': ['assert_relative_phase',
                                                                                                                             'nominal_relative_fixture',
                                                                                                                             'product_ok']},
                    'measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                            'helpers': ['assert_relative_phase',
                                                                                                                        'nominal_relative_fixture',
                                                                                                                        'product_ok']},
                    'measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                        'helpers': ['assert_relative_phase',
                                                                                                                    'nominal_relative_fixture',
                                                                                                                    'product_ok']},
                    'measurement::tests::out_of_scope_local_coordinate_is_not_rebased': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                         'helpers': ['assert_relative_phase',
                                                                                                     'nominal_relative_fixture',
                                                                                                     'product_ok']},
                    'measurement::tests::peer_shift_changes_sign_without_changing_rtt': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                         'helpers': ['assert_relative_phase',
                                                                                                     'nominal_relative_fixture',
                                                                                                     'product_ok']},
                    'measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                 'helpers': ['assert_relative_phase',
                                                                                                             'nominal_relative_fixture',
                                                                                                             'product_ok']},
                    'measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                        'helpers': ['assert_relative_phase',
                                                                                                                    'nominal_relative_fixture',
                                                                                                                    'product_ok']},
                    'measurement::tests::profile_bounds_and_maximum_allowance_are_checked': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                             'helpers': ['assert_relative_phase',
                                                                                                         'nominal_relative_fixture',
                                                                                                         'product_ok']},
                    'measurement::tests::refusal_codes_and_display_remain_literal_and_redacted': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                  'helpers': ['assert_relative_phase',
                                                                                                              'nominal_relative_fixture',
                                                                                                              'product_ok']},
                    'measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                      'helpers': ['assert_relative_phase',
                                                                                                                  'nominal_relative_fixture',
                                                                                                                  'product_ok']},
                    'measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                       'helpers': ['assert_relative_phase',
                                                                                                                   'nominal_relative_fixture',
                                                                                                                   'product_ok']},
                    'measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                     'helpers': ['assert_relative_phase',
                                                                                                                 'nominal_relative_fixture',
                                                                                                                 'product_ok']},
                    'measurement::tests::reversed_brackets_and_processing_are_not_ages': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                          'helpers': ['assert_relative_phase',
                                                                                                      'nominal_relative_fixture',
                                                                                                      'product_ok']},
                    'measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                         'helpers': ['assert_relative_phase',
                                                                                                                     'nominal_relative_fixture',
                                                                                                                     'product_ok']},
                    'measurement::tests::series_coherence_all_nine_positions_influence_the_maximum': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                      'helpers': ['assert_relative_phase',
                                                                                                                  'nominal_relative_fixture',
                                                                                                                  'product_ok']},
                    'measurement::tests::series_coherence_empty_single_and_holes_keep_original_positions': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                            'helpers': ['assert_relative_phase',
                                                                                                                        'nominal_relative_fixture',
                                                                                                                        'product_ok']},
                    'measurement::tests::series_coherence_fixed_first_baseline_detects_accumulated_drift': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                            'helpers': ['assert_relative_phase',
                                                                                                                        'nominal_relative_fixture',
                                                                                                                        'product_ok']},
                    'measurement::tests::series_coherence_full_order_validation_precedes_earlier_arithmetic': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                               'helpers': ['assert_relative_phase',
                                                                                                                           'nominal_relative_fixture',
                                                                                                                           'product_ok']},
                    'measurement::tests::series_coherence_full_u64_residual_keeps_its_next_rejection': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                        'helpers': ['assert_relative_phase',
                                                                                                                    'nominal_relative_fixture',
                                                                                                                    'product_ok']},
                    'measurement::tests::series_coherence_late_arithmetic_failure_wins_over_an_earlier_gap': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                              'helpers': ['assert_relative_phase',
                                                                                                                          'nominal_relative_fixture',
                                                                                                                          'product_ok']},
                    'measurement::tests::series_coherence_rejects_each_own_and_sequential_order_violation': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                             'helpers': ['assert_relative_phase',
                                                                                                                         'nominal_relative_fixture',
                                                                                                                         'product_ok']},
                    'measurement::tests::software_window_retains_each_local_and_peer_extremum': {'source': 'crates/nevrotick-engine/tests/private/measurement.rs',
                                                                                                 'helpers': ['assert_relative_phase',
                                                                                                             'nominal_relative_fixture',
                                                                                                             'product_ok']},
                    'middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                                'helpers': ['check_exact_measurement',
                                                                                                            'check_ratio',
                                                                                                            'common_report_checks',
                                                                                                            'facts',
                                                                                                            'healthy_report_checks',
                                                                                                            'no_extra_query',
                                                                                                            'product_ok',
                                                                                                            'profile',
                                                                                                            'run_bound_case',
                                                                                                            'run_case',
                                                                                                            'serve',
                                                                                                            'slot',
                                                                                                            'terminal_case']},
                    'processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                 'helpers': ['assert_observed_difference',
                                                                                                                             'check_actual_measurement_record',
                                                                                                                             'check_commit_processing_boundaries',
                                                                                                                             'check_completed_cleanup_error',
                                                                                                                             'check_fact_getters',
                                                                                                                             'check_failure_projection',
                                                                                                                             'check_guard',
                                                                                                                             'check_independent_staged_context_guards',
                                                                                                                             'check_io_report',
                                                                                                                             'check_literal_stage_rejections',
                                                                                                                             'check_observed_measurement',
                                                                                                                             'check_outcomes',
                                                                                                                             'check_partial_measurement_write_failure',
                                                                                                                             'check_receive_cap',
                                                                                                                             'check_relative_refusals',
                                                                                                                             'check_report_projection',
                                                                                                                             'check_retired_selected_read',
                                                                                                                             'check_send_completion',
                                                                                                                             'check_shared_measurement_writer',
                                                                                                                             'check_target',
                                                                                                                             'check_typed_oversize_accounting',
                                                                                                                             'commit_neighbor',
                                                                                                                             'inject_io_case',
                                                                                                                             'observed_reference_ticks',
                                                                                                                             'product_ok',
                                                                                                                             'two_origin_rejections']},
                    'processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                'helpers': ['assert_observed_difference',
                                                                                                                            'check_actual_measurement_record',
                                                                                                                            'check_commit_processing_boundaries',
                                                                                                                            'check_completed_cleanup_error',
                                                                                                                            'check_fact_getters',
                                                                                                                            'check_failure_projection',
                                                                                                                            'check_guard',
                                                                                                                            'check_independent_staged_context_guards',
                                                                                                                            'check_io_report',
                                                                                                                            'check_literal_stage_rejections',
                                                                                                                            'check_observed_measurement',
                                                                                                                            'check_outcomes',
                                                                                                                            'check_partial_measurement_write_failure',
                                                                                                                            'check_receive_cap',
                                                                                                                            'check_relative_refusals',
                                                                                                                            'check_report_projection',
                                                                                                                            'check_retired_selected_read',
                                                                                                                            'check_send_completion',
                                                                                                                            'check_shared_measurement_writer',
                                                                                                                            'check_target',
                                                                                                                            'check_typed_oversize_accounting',
                                                                                                                            'commit_neighbor',
                                                                                                                            'inject_io_case',
                                                                                                                            'observed_reference_ticks',
                                                                                                                            'product_ok',
                                                                                                                            'two_origin_rejections']},
                    'processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                      'helpers': ['assert_observed_difference',
                                                                                                                                  'check_actual_measurement_record',
                                                                                                                                  'check_commit_processing_boundaries',
                                                                                                                                  'check_completed_cleanup_error',
                                                                                                                                  'check_fact_getters',
                                                                                                                                  'check_failure_projection',
                                                                                                                                  'check_guard',
                                                                                                                                  'check_independent_staged_context_guards',
                                                                                                                                  'check_io_report',
                                                                                                                                  'check_literal_stage_rejections',
                                                                                                                                  'check_observed_measurement',
                                                                                                                                  'check_outcomes',
                                                                                                                                  'check_partial_measurement_write_failure',
                                                                                                                                  'check_receive_cap',
                                                                                                                                  'check_relative_refusals',
                                                                                                                                  'check_report_projection',
                                                                                                                                  'check_retired_selected_read',
                                                                                                                                  'check_send_completion',
                                                                                                                                  'check_shared_measurement_writer',
                                                                                                                                  'check_target',
                                                                                                                                  'check_typed_oversize_accounting',
                                                                                                                                  'commit_neighbor',
                                                                                                                                  'inject_io_case',
                                                                                                                                  'observed_reference_ticks',
                                                                                                                                  'product_ok',
                                                                                                                                  'two_origin_rejections']},
                    'processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                 'helpers': ['assert_observed_difference',
                                                                                                                             'check_actual_measurement_record',
                                                                                                                             'check_commit_processing_boundaries',
                                                                                                                             'check_completed_cleanup_error',
                                                                                                                             'check_fact_getters',
                                                                                                                             'check_failure_projection',
                                                                                                                             'check_guard',
                                                                                                                             'check_independent_staged_context_guards',
                                                                                                                             'check_io_report',
                                                                                                                             'check_literal_stage_rejections',
                                                                                                                             'check_observed_measurement',
                                                                                                                             'check_outcomes',
                                                                                                                             'check_partial_measurement_write_failure',
                                                                                                                             'check_receive_cap',
                                                                                                                             'check_relative_refusals',
                                                                                                                             'check_report_projection',
                                                                                                                             'check_retired_selected_read',
                                                                                                                             'check_send_completion',
                                                                                                                             'check_shared_measurement_writer',
                                                                                                                             'check_target',
                                                                                                                             'check_typed_oversize_accounting',
                                                                                                                             'commit_neighbor',
                                                                                                                             'inject_io_case',
                                                                                                                             'observed_reference_ticks',
                                                                                                                             'product_ok',
                                                                                                                             'two_origin_rejections']},
                    'processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                       'helpers': ['assert_observed_difference',
                                                                                                                   'check_actual_measurement_record',
                                                                                                                   'check_commit_processing_boundaries',
                                                                                                                   'check_completed_cleanup_error',
                                                                                                                   'check_fact_getters',
                                                                                                                   'check_failure_projection',
                                                                                                                   'check_guard',
                                                                                                                   'check_independent_staged_context_guards',
                                                                                                                   'check_io_report',
                                                                                                                   'check_literal_stage_rejections',
                                                                                                                   'check_observed_measurement',
                                                                                                                   'check_outcomes',
                                                                                                                   'check_partial_measurement_write_failure',
                                                                                                                   'check_receive_cap',
                                                                                                                   'check_relative_refusals',
                                                                                                                   'check_report_projection',
                                                                                                                   'check_retired_selected_read',
                                                                                                                   'check_send_completion',
                                                                                                                   'check_shared_measurement_writer',
                                                                                                                   'check_target',
                                                                                                                   'check_typed_oversize_accounting',
                                                                                                                   'commit_neighbor',
                                                                                                                   'inject_io_case',
                                                                                                                   'observed_reference_ticks',
                                                                                                                   'product_ok',
                                                                                                                   'two_origin_rejections']},
                    'processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                         'helpers': ['assert_observed_difference',
                                                                                                                     'check_actual_measurement_record',
                                                                                                                     'check_commit_processing_boundaries',
                                                                                                                     'check_completed_cleanup_error',
                                                                                                                     'check_fact_getters',
                                                                                                                     'check_failure_projection',
                                                                                                                     'check_guard',
                                                                                                                     'check_independent_staged_context_guards',
                                                                                                                     'check_io_report',
                                                                                                                     'check_literal_stage_rejections',
                                                                                                                     'check_observed_measurement',
                                                                                                                     'check_outcomes',
                                                                                                                     'check_partial_measurement_write_failure',
                                                                                                                     'check_receive_cap',
                                                                                                                     'check_relative_refusals',
                                                                                                                     'check_report_projection',
                                                                                                                     'check_retired_selected_read',
                                                                                                                     'check_send_completion',
                                                                                                                     'check_shared_measurement_writer',
                                                                                                                     'check_target',
                                                                                                                     'check_typed_oversize_accounting',
                                                                                                                     'commit_neighbor',
                                                                                                                     'inject_io_case',
                                                                                                                     'observed_reference_ticks',
                                                                                                                     'product_ok',
                                                                                                                     'two_origin_rejections']},
                    'processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                           'helpers': ['assert_observed_difference',
                                                                                                                       'check_actual_measurement_record',
                                                                                                                       'check_commit_processing_boundaries',
                                                                                                                       'check_completed_cleanup_error',
                                                                                                                       'check_fact_getters',
                                                                                                                       'check_failure_projection',
                                                                                                                       'check_guard',
                                                                                                                       'check_independent_staged_context_guards',
                                                                                                                       'check_io_report',
                                                                                                                       'check_literal_stage_rejections',
                                                                                                                       'check_observed_measurement',
                                                                                                                       'check_outcomes',
                                                                                                                       'check_partial_measurement_write_failure',
                                                                                                                       'check_receive_cap',
                                                                                                                       'check_relative_refusals',
                                                                                                                       'check_report_projection',
                                                                                                                       'check_retired_selected_read',
                                                                                                                       'check_send_completion',
                                                                                                                       'check_shared_measurement_writer',
                                                                                                                       'check_target',
                                                                                                                       'check_typed_oversize_accounting',
                                                                                                                       'commit_neighbor',
                                                                                                                       'inject_io_case',
                                                                                                                       'observed_reference_ticks',
                                                                                                                       'product_ok',
                                                                                                                       'two_origin_rejections']},
                    'processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                            'helpers': ['assert_observed_difference',
                                                                                                                        'check_actual_measurement_record',
                                                                                                                        'check_commit_processing_boundaries',
                                                                                                                        'check_completed_cleanup_error',
                                                                                                                        'check_fact_getters',
                                                                                                                        'check_failure_projection',
                                                                                                                        'check_guard',
                                                                                                                        'check_independent_staged_context_guards',
                                                                                                                        'check_io_report',
                                                                                                                        'check_literal_stage_rejections',
                                                                                                                        'check_observed_measurement',
                                                                                                                        'check_outcomes',
                                                                                                                        'check_partial_measurement_write_failure',
                                                                                                                        'check_receive_cap',
                                                                                                                        'check_relative_refusals',
                                                                                                                        'check_report_projection',
                                                                                                                        'check_retired_selected_read',
                                                                                                                        'check_send_completion',
                                                                                                                        'check_shared_measurement_writer',
                                                                                                                        'check_target',
                                                                                                                        'check_typed_oversize_accounting',
                                                                                                                        'commit_neighbor',
                                                                                                                        'inject_io_case',
                                                                                                                        'observed_reference_ticks',
                                                                                                                        'product_ok',
                                                                                                                        'two_origin_rejections']},
                    'processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                               'helpers': ['assert_observed_difference',
                                                                                                                           'check_actual_measurement_record',
                                                                                                                           'check_commit_processing_boundaries',
                                                                                                                           'check_completed_cleanup_error',
                                                                                                                           'check_fact_getters',
                                                                                                                           'check_failure_projection',
                                                                                                                           'check_guard',
                                                                                                                           'check_independent_staged_context_guards',
                                                                                                                           'check_io_report',
                                                                                                                           'check_literal_stage_rejections',
                                                                                                                           'check_observed_measurement',
                                                                                                                           'check_outcomes',
                                                                                                                           'check_partial_measurement_write_failure',
                                                                                                                           'check_receive_cap',
                                                                                                                           'check_relative_refusals',
                                                                                                                           'check_report_projection',
                                                                                                                           'check_retired_selected_read',
                                                                                                                           'check_send_completion',
                                                                                                                           'check_shared_measurement_writer',
                                                                                                                           'check_target',
                                                                                                                           'check_typed_oversize_accounting',
                                                                                                                           'commit_neighbor',
                                                                                                                           'inject_io_case',
                                                                                                                           'observed_reference_ticks',
                                                                                                                           'product_ok',
                                                                                                                           'two_origin_rejections']},
                    'processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                 'helpers': ['assert_observed_difference',
                                                                                                                             'check_actual_measurement_record',
                                                                                                                             'check_commit_processing_boundaries',
                                                                                                                             'check_completed_cleanup_error',
                                                                                                                             'check_fact_getters',
                                                                                                                             'check_failure_projection',
                                                                                                                             'check_guard',
                                                                                                                             'check_independent_staged_context_guards',
                                                                                                                             'check_io_report',
                                                                                                                             'check_literal_stage_rejections',
                                                                                                                             'check_observed_measurement',
                                                                                                                             'check_outcomes',
                                                                                                                             'check_partial_measurement_write_failure',
                                                                                                                             'check_receive_cap',
                                                                                                                             'check_relative_refusals',
                                                                                                                             'check_report_projection',
                                                                                                                             'check_retired_selected_read',
                                                                                                                             'check_send_completion',
                                                                                                                             'check_shared_measurement_writer',
                                                                                                                             'check_target',
                                                                                                                             'check_typed_oversize_accounting',
                                                                                                                             'commit_neighbor',
                                                                                                                             'inject_io_case',
                                                                                                                             'observed_reference_ticks',
                                                                                                                             'product_ok',
                                                                                                                             'two_origin_rejections']},
                    'processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                         'helpers': ['assert_observed_difference',
                                                                                                                     'check_actual_measurement_record',
                                                                                                                     'check_commit_processing_boundaries',
                                                                                                                     'check_completed_cleanup_error',
                                                                                                                     'check_fact_getters',
                                                                                                                     'check_failure_projection',
                                                                                                                     'check_guard',
                                                                                                                     'check_independent_staged_context_guards',
                                                                                                                     'check_io_report',
                                                                                                                     'check_literal_stage_rejections',
                                                                                                                     'check_observed_measurement',
                                                                                                                     'check_outcomes',
                                                                                                                     'check_partial_measurement_write_failure',
                                                                                                                     'check_receive_cap',
                                                                                                                     'check_relative_refusals',
                                                                                                                     'check_report_projection',
                                                                                                                     'check_retired_selected_read',
                                                                                                                     'check_send_completion',
                                                                                                                     'check_shared_measurement_writer',
                                                                                                                     'check_target',
                                                                                                                     'check_typed_oversize_accounting',
                                                                                                                     'commit_neighbor',
                                                                                                                     'inject_io_case',
                                                                                                                     'observed_reference_ticks',
                                                                                                                     'product_ok',
                                                                                                                     'two_origin_rejections']},
                    'processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                 'helpers': ['assert_observed_difference',
                                                                                                                             'check_actual_measurement_record',
                                                                                                                             'check_commit_processing_boundaries',
                                                                                                                             'check_completed_cleanup_error',
                                                                                                                             'check_fact_getters',
                                                                                                                             'check_failure_projection',
                                                                                                                             'check_guard',
                                                                                                                             'check_independent_staged_context_guards',
                                                                                                                             'check_io_report',
                                                                                                                             'check_literal_stage_rejections',
                                                                                                                             'check_observed_measurement',
                                                                                                                             'check_outcomes',
                                                                                                                             'check_partial_measurement_write_failure',
                                                                                                                             'check_receive_cap',
                                                                                                                             'check_relative_refusals',
                                                                                                                             'check_report_projection',
                                                                                                                             'check_retired_selected_read',
                                                                                                                             'check_send_completion',
                                                                                                                             'check_shared_measurement_writer',
                                                                                                                             'check_target',
                                                                                                                             'check_typed_oversize_accounting',
                                                                                                                             'commit_neighbor',
                                                                                                                             'inject_io_case',
                                                                                                                             'observed_reference_ticks',
                                                                                                                             'product_ok',
                                                                                                                             'two_origin_rejections']},
                    'processing_guard_tests::processing_uses_fresh_deadline_and_current_control': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                   'helpers': ['assert_observed_difference',
                                                                                                               'check_actual_measurement_record',
                                                                                                               'check_commit_processing_boundaries',
                                                                                                               'check_completed_cleanup_error',
                                                                                                               'check_fact_getters',
                                                                                                               'check_failure_projection',
                                                                                                               'check_guard',
                                                                                                               'check_independent_staged_context_guards',
                                                                                                               'check_io_report',
                                                                                                               'check_literal_stage_rejections',
                                                                                                               'check_observed_measurement',
                                                                                                               'check_outcomes',
                                                                                                               'check_partial_measurement_write_failure',
                                                                                                               'check_receive_cap',
                                                                                                               'check_relative_refusals',
                                                                                                               'check_report_projection',
                                                                                                               'check_retired_selected_read',
                                                                                                               'check_send_completion',
                                                                                                               'check_shared_measurement_writer',
                                                                                                               'check_target',
                                                                                                               'check_typed_oversize_accounting',
                                                                                                               'commit_neighbor',
                                                                                                               'inject_io_case',
                                                                                                               'observed_reference_ticks',
                                                                                                               'product_ok',
                                                                                                               'two_origin_rejections']},
                    'processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                    'helpers': ['assert_observed_difference',
                                                                                                                                'check_actual_measurement_record',
                                                                                                                                'check_commit_processing_boundaries',
                                                                                                                                'check_completed_cleanup_error',
                                                                                                                                'check_fact_getters',
                                                                                                                                'check_failure_projection',
                                                                                                                                'check_guard',
                                                                                                                                'check_independent_staged_context_guards',
                                                                                                                                'check_io_report',
                                                                                                                                'check_literal_stage_rejections',
                                                                                                                                'check_observed_measurement',
                                                                                                                                'check_outcomes',
                                                                                                                                'check_partial_measurement_write_failure',
                                                                                                                                'check_receive_cap',
                                                                                                                                'check_relative_refusals',
                                                                                                                                'check_report_projection',
                                                                                                                                'check_retired_selected_read',
                                                                                                                                'check_send_completion',
                                                                                                                                'check_shared_measurement_writer',
                                                                                                                                'check_target',
                                                                                                                                'check_typed_oversize_accounting',
                                                                                                                                'commit_neighbor',
                                                                                                                                'inject_io_case',
                                                                                                                                'observed_reference_ticks',
                                                                                                                                'product_ok',
                                                                                                                                'two_origin_rejections']},
                    'processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                       'helpers': ['assert_observed_difference',
                                                                                                                                   'check_actual_measurement_record',
                                                                                                                                   'check_commit_processing_boundaries',
                                                                                                                                   'check_completed_cleanup_error',
                                                                                                                                   'check_fact_getters',
                                                                                                                                   'check_failure_projection',
                                                                                                                                   'check_guard',
                                                                                                                                   'check_independent_staged_context_guards',
                                                                                                                                   'check_io_report',
                                                                                                                                   'check_literal_stage_rejections',
                                                                                                                                   'check_observed_measurement',
                                                                                                                                   'check_outcomes',
                                                                                                                                   'check_partial_measurement_write_failure',
                                                                                                                                   'check_receive_cap',
                                                                                                                                   'check_relative_refusals',
                                                                                                                                   'check_report_projection',
                                                                                                                                   'check_retired_selected_read',
                                                                                                                                   'check_send_completion',
                                                                                                                                   'check_shared_measurement_writer',
                                                                                                                                   'check_target',
                                                                                                                                   'check_typed_oversize_accounting',
                                                                                                                                   'commit_neighbor',
                                                                                                                                   'inject_io_case',
                                                                                                                                   'observed_reference_ticks',
                                                                                                                                   'product_ok',
                                                                                                                                   'two_origin_rejections']},
                    'processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                              'helpers': ['assert_observed_difference',
                                                                                                                          'check_actual_measurement_record',
                                                                                                                          'check_commit_processing_boundaries',
                                                                                                                          'check_completed_cleanup_error',
                                                                                                                          'check_fact_getters',
                                                                                                                          'check_failure_projection',
                                                                                                                          'check_guard',
                                                                                                                          'check_independent_staged_context_guards',
                                                                                                                          'check_io_report',
                                                                                                                          'check_literal_stage_rejections',
                                                                                                                          'check_observed_measurement',
                                                                                                                          'check_outcomes',
                                                                                                                          'check_partial_measurement_write_failure',
                                                                                                                          'check_receive_cap',
                                                                                                                          'check_relative_refusals',
                                                                                                                          'check_report_projection',
                                                                                                                          'check_retired_selected_read',
                                                                                                                          'check_send_completion',
                                                                                                                          'check_shared_measurement_writer',
                                                                                                                          'check_target',
                                                                                                                          'check_typed_oversize_accounting',
                                                                                                                          'commit_neighbor',
                                                                                                                          'inject_io_case',
                                                                                                                          'observed_reference_ticks',
                                                                                                                          'product_ok',
                                                                                                                          'two_origin_rejections']},
                    'processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                         'helpers': ['assert_observed_difference',
                                                                                                                     'check_actual_measurement_record',
                                                                                                                     'check_commit_processing_boundaries',
                                                                                                                     'check_completed_cleanup_error',
                                                                                                                     'check_fact_getters',
                                                                                                                     'check_failure_projection',
                                                                                                                     'check_guard',
                                                                                                                     'check_independent_staged_context_guards',
                                                                                                                     'check_io_report',
                                                                                                                     'check_literal_stage_rejections',
                                                                                                                     'check_observed_measurement',
                                                                                                                     'check_outcomes',
                                                                                                                     'check_partial_measurement_write_failure',
                                                                                                                     'check_receive_cap',
                                                                                                                     'check_relative_refusals',
                                                                                                                     'check_report_projection',
                                                                                                                     'check_retired_selected_read',
                                                                                                                     'check_send_completion',
                                                                                                                     'check_shared_measurement_writer',
                                                                                                                     'check_target',
                                                                                                                     'check_typed_oversize_accounting',
                                                                                                                     'commit_neighbor',
                                                                                                                     'inject_io_case',
                                                                                                                     'observed_reference_ticks',
                                                                                                                     'product_ok',
                                                                                                                     'two_origin_rejections']},
                    'processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                                'helpers': ['assert_observed_difference',
                                                                                                                            'check_actual_measurement_record',
                                                                                                                            'check_commit_processing_boundaries',
                                                                                                                            'check_completed_cleanup_error',
                                                                                                                            'check_fact_getters',
                                                                                                                            'check_failure_projection',
                                                                                                                            'check_guard',
                                                                                                                            'check_independent_staged_context_guards',
                                                                                                                            'check_io_report',
                                                                                                                            'check_literal_stage_rejections',
                                                                                                                            'check_observed_measurement',
                                                                                                                            'check_outcomes',
                                                                                                                            'check_partial_measurement_write_failure',
                                                                                                                            'check_receive_cap',
                                                                                                                            'check_relative_refusals',
                                                                                                                            'check_report_projection',
                                                                                                                            'check_retired_selected_read',
                                                                                                                            'check_send_completion',
                                                                                                                            'check_shared_measurement_writer',
                                                                                                                            'check_target',
                                                                                                                            'check_typed_oversize_accounting',
                                                                                                                            'commit_neighbor',
                                                                                                                            'inject_io_case',
                                                                                                                            'observed_reference_ticks',
                                                                                                                            'product_ok',
                                                                                                                            'two_origin_rejections']},
                    'processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                       'helpers': ['assert_observed_difference',
                                                                                                                   'check_actual_measurement_record',
                                                                                                                   'check_commit_processing_boundaries',
                                                                                                                   'check_completed_cleanup_error',
                                                                                                                   'check_fact_getters',
                                                                                                                   'check_failure_projection',
                                                                                                                   'check_guard',
                                                                                                                   'check_independent_staged_context_guards',
                                                                                                                   'check_io_report',
                                                                                                                   'check_literal_stage_rejections',
                                                                                                                   'check_observed_measurement',
                                                                                                                   'check_outcomes',
                                                                                                                   'check_partial_measurement_write_failure',
                                                                                                                   'check_receive_cap',
                                                                                                                   'check_relative_refusals',
                                                                                                                   'check_report_projection',
                                                                                                                   'check_retired_selected_read',
                                                                                                                   'check_send_completion',
                                                                                                                   'check_shared_measurement_writer',
                                                                                                                   'check_target',
                                                                                                                   'check_typed_oversize_accounting',
                                                                                                                   'commit_neighbor',
                                                                                                                   'inject_io_case',
                                                                                                                   'observed_reference_ticks',
                                                                                                                   'product_ok',
                                                                                                                   'two_origin_rejections']},
                    'processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                    'helpers': ['assert_observed_difference',
                                                                                                                'check_actual_measurement_record',
                                                                                                                'check_commit_processing_boundaries',
                                                                                                                'check_completed_cleanup_error',
                                                                                                                'check_fact_getters',
                                                                                                                'check_failure_projection',
                                                                                                                'check_guard',
                                                                                                                'check_independent_staged_context_guards',
                                                                                                                'check_io_report',
                                                                                                                'check_literal_stage_rejections',
                                                                                                                'check_observed_measurement',
                                                                                                                'check_outcomes',
                                                                                                                'check_partial_measurement_write_failure',
                                                                                                                'check_receive_cap',
                                                                                                                'check_relative_refusals',
                                                                                                                'check_report_projection',
                                                                                                                'check_retired_selected_read',
                                                                                                                'check_send_completion',
                                                                                                                'check_shared_measurement_writer',
                                                                                                                'check_target',
                                                                                                                'check_typed_oversize_accounting',
                                                                                                                'commit_neighbor',
                                                                                                                'inject_io_case',
                                                                                                                'observed_reference_ticks',
                                                                                                                'product_ok',
                                                                                                                'two_origin_rejections']},
                    'processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap': {'source': 'crates/nevrotick-engine/tests/private/processing_guards.rs',
                                                                                                     'helpers': ['assert_observed_difference',
                                                                                                                 'check_actual_measurement_record',
                                                                                                                 'check_commit_processing_boundaries',
                                                                                                                 'check_completed_cleanup_error',
                                                                                                                 'check_fact_getters',
                                                                                                                 'check_failure_projection',
                                                                                                                 'check_guard',
                                                                                                                 'check_independent_staged_context_guards',
                                                                                                                 'check_io_report',
                                                                                                                 'check_literal_stage_rejections',
                                                                                                                 'check_observed_measurement',
                                                                                                                 'check_outcomes',
                                                                                                                 'check_partial_measurement_write_failure',
                                                                                                                 'check_receive_cap',
                                                                                                                 'check_relative_refusals',
                                                                                                                 'check_report_projection',
                                                                                                                 'check_retired_selected_read',
                                                                                                                 'check_send_completion',
                                                                                                                 'check_shared_measurement_writer',
                                                                                                                 'check_target',
                                                                                                                 'check_typed_oversize_accounting',
                                                                                                                 'commit_neighbor',
                                                                                                                 'inject_io_case',
                                                                                                                 'observed_reference_ticks',
                                                                                                                 'product_ok',
                                                                                                                 'two_origin_rejections']},
                    'quality::tests::computed_policy_boundaries_preserve_strict_and_inclusive_comparisons': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                             'helpers': ['assumptions',
                                                                                                                         'exact',
                                                                                                                         'facts',
                                                                                                                         'facts_with_wall',
                                                                                                                         'product_ok']},
                    'quality::tests::declared_getters_and_constructor_precedence_preserve_original_bits': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                           'helpers': ['assumptions',
                                                                                                                       'exact',
                                                                                                                       'facts',
                                                                                                                       'facts_with_wall',
                                                                                                                       'product_ok']},
                    'quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                  'helpers': ['assumptions',
                                                                                                              'exact',
                                                                                                              'facts',
                                                                                                              'facts_with_wall',
                                                                                                              'product_ok']},
                    'quality::tests::evaluator_failure_precedence_preserves_original_observation': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                    'helpers': ['assumptions',
                                                                                                                'exact',
                                                                                                                'facts',
                                                                                                                'facts_with_wall',
                                                                                                                'product_ok']},
                    'quality::tests::finite_input_overflow_is_refused_at_the_first_computed_stage': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                     'helpers': ['assumptions',
                                                                                                                 'exact',
                                                                                                                 'facts',
                                                                                                                 'facts_with_wall',
                                                                                                                 'product_ok']},
                    'quality::tests::generated_garde_dispatch_covers_nested_and_computed_fields': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                   'helpers': ['assumptions',
                                                                                                               'exact',
                                                                                                               'facts',
                                                                                                               'facts_with_wall',
                                                                                                               'product_ok']},
                    'quality::tests::gradual_underflow_does_not_claim_exact_zero_uncertainty': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                'helpers': ['assumptions',
                                                                                                            'exact',
                                                                                                            'facts',
                                                                                                            'facts_with_wall',
                                                                                                            'product_ok']},
                    'quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                                 'helpers': ['assumptions',
                                                                                                                             'exact',
                                                                                                                             'facts',
                                                                                                                             'facts_with_wall',
                                                                                                                             'product_ok']},
                    'quality::tests::negative_local_wall_interval_grows_declared_frequency_dispersion': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                         'helpers': ['assumptions',
                                                                                                                     'exact',
                                                                                                                     'facts',
                                                                                                                     'facts_with_wall',
                                                                                                                     'product_ok']},
                    'quality::tests::negative_server_interval_processing_limit_uses_absolute_magnitude': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                          'helpers': ['assumptions',
                                                                                                                      'exact',
                                                                                                                      'facts',
                                                                                                                      'facts_with_wall',
                                                                                                                      'product_ok']},
                    'quality::tests::original_measurement_refusal_and_redacted_diagnostics_are_preserved': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                            'helpers': ['assumptions',
                                                                                                                        'exact',
                                                                                                                        'facts',
                                                                                                                        'facts_with_wall',
                                                                                                                        'product_ok']},
                    'quality::tests::precision_clamps_have_exact_boundary_and_neighbor_values': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                 'helpers': ['assumptions',
                                                                                                             'exact',
                                                                                                             'facts',
                                                                                                             'facts_with_wall',
                                                                                                             'product_ok']},
                    'quality::tests::raw_round_trip_is_converted_once_before_cancellation': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                             'helpers': ['assumptions',
                                                                                                         'exact',
                                                                                                         'facts',
                                                                                                         'facts_with_wall',
                                                                                                         'product_ok']},
                    'quality::tests::raw_scalar_and_interval_helpers_preserve_full_admitted_domains': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                       'helpers': ['assumptions',
                                                                                                                   'exact',
                                                                                                                   'facts',
                                                                                                                   'facts_with_wall',
                                                                                                                   'product_ok']},
                    'quality::tests::refusal_families_keep_bounded_diagnostics_and_typed_sources': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                    'helpers': ['assumptions',
                                                                                                                'exact',
                                                                                                                'facts',
                                                                                                                'facts_with_wall',
                                                                                                                'product_ok']},
                    'quality::tests::signed_server_interval_and_unsigned_root_words_are_retained': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                                    'helpers': ['assumptions',
                                                                                                                'exact',
                                                                                                                'facts',
                                                                                                                'facts_with_wall',
                                                                                                                'product_ok']},
                    'quality::tests::zero_raw_rtt_uses_combined_precision_floor': {'source': 'crates/nevrotick-engine/src/quality/tests.rs',
                                                                                   'helpers': ['assumptions',
                                                                                               'exact',
                                                                                               'facts',
                                                                                               'facts_with_wall',
                                                                                               'product_ok']},
                    'series::tests::actual_publication_rechecks_control_and_deadline_after_staging': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                      'helpers': ['assert_fixed_redaction',
                                                                                                                  'assert_no_extra_query',
                                                                                                                  'check_checkpoint_facts',
                                                                                                                  'checkpoint_queries',
                                                                                                                  'cleanup_boundary_neighbors',
                                                                                                                  'completion',
                                                                                                                  'final_checkpoint_case',
                                                                                                                  'issue_boundary_neighbors',
                                                                                                                  'modeled_read_boundary_neighbors',
                                                                                                                  'product_ok',
                                                                                                                  'queued_native_reply',
                                                                                                                  'receive_boundary_neighbors',
                                                                                                                  'reply_once',
                                                                                                                  'retired_writable_neighbor',
                                                                                                                  'selected_native_reply',
                                                                                                                  'send_cap_case',
                                                                                                                  'sending_slot',
                                                                                                                  'serve_checkpoint',
                                                                                                                  'setup_failure_case',
                                                                                                                  'staged_native_reply',
                                                                                                                  'terminal_slot',
                                                                                                                  'writable_retirement_neighbors']},
                    'series::tests::all_series_debug_outputs_are_exact_fixed_redactions': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                           'helpers': ['assert_fixed_redaction',
                                                                                                       'assert_no_extra_query',
                                                                                                       'check_checkpoint_facts',
                                                                                                       'checkpoint_queries',
                                                                                                       'cleanup_boundary_neighbors',
                                                                                                       'completion',
                                                                                                       'final_checkpoint_case',
                                                                                                       'issue_boundary_neighbors',
                                                                                                       'modeled_read_boundary_neighbors',
                                                                                                       'product_ok',
                                                                                                       'queued_native_reply',
                                                                                                       'receive_boundary_neighbors',
                                                                                                       'reply_once',
                                                                                                       'retired_writable_neighbor',
                                                                                                       'selected_native_reply',
                                                                                                       'send_cap_case',
                                                                                                       'sending_slot',
                                                                                                       'serve_checkpoint',
                                                                                                       'setup_failure_case',
                                                                                                       'staged_native_reply',
                                                                                                       'terminal_slot',
                                                                                                       'writable_retirement_neighbors']},
                    'series::tests::current_checkpoint_observes_new_controls_before_expired_deadline': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                        'helpers': ['assert_fixed_redaction',
                                                                                                                    'assert_no_extra_query',
                                                                                                                    'check_checkpoint_facts',
                                                                                                                    'checkpoint_queries',
                                                                                                                    'cleanup_boundary_neighbors',
                                                                                                                    'completion',
                                                                                                                    'final_checkpoint_case',
                                                                                                                    'issue_boundary_neighbors',
                                                                                                                    'modeled_read_boundary_neighbors',
                                                                                                                    'product_ok',
                                                                                                                    'queued_native_reply',
                                                                                                                    'receive_boundary_neighbors',
                                                                                                                    'reply_once',
                                                                                                                    'retired_writable_neighbor',
                                                                                                                    'selected_native_reply',
                                                                                                                    'send_cap_case',
                                                                                                                    'sending_slot',
                                                                                                                    'serve_checkpoint',
                                                                                                                    'setup_failure_case',
                                                                                                                    'staged_native_reply',
                                                                                                                    'terminal_slot',
                                                                                                                    'writable_retirement_neighbors']},
                    'series::tests::deadlines_and_controls_do_not_reset_the_series_origin': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                             'helpers': ['assert_fixed_redaction',
                                                                                                         'assert_no_extra_query',
                                                                                                         'check_checkpoint_facts',
                                                                                                         'checkpoint_queries',
                                                                                                         'cleanup_boundary_neighbors',
                                                                                                         'completion',
                                                                                                         'final_checkpoint_case',
                                                                                                         'issue_boundary_neighbors',
                                                                                                         'modeled_read_boundary_neighbors',
                                                                                                         'product_ok',
                                                                                                         'queued_native_reply',
                                                                                                         'receive_boundary_neighbors',
                                                                                                         'reply_once',
                                                                                                         'retired_writable_neighbor',
                                                                                                         'selected_native_reply',
                                                                                                         'send_cap_case',
                                                                                                         'sending_slot',
                                                                                                         'serve_checkpoint',
                                                                                                         'setup_failure_case',
                                                                                                         'staged_native_reply',
                                                                                                         'terminal_slot',
                                                                                                         'writable_retirement_neighbors']},
                    'series::tests::entropy_failure_and_entered_runtime_precede_all_acquisition_work': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                        'helpers': ['assert_fixed_redaction',
                                                                                                                    'assert_no_extra_query',
                                                                                                                    'check_checkpoint_facts',
                                                                                                                    'checkpoint_queries',
                                                                                                                    'cleanup_boundary_neighbors',
                                                                                                                    'completion',
                                                                                                                    'final_checkpoint_case',
                                                                                                                    'issue_boundary_neighbors',
                                                                                                                    'modeled_read_boundary_neighbors',
                                                                                                                    'product_ok',
                                                                                                                    'queued_native_reply',
                                                                                                                    'receive_boundary_neighbors',
                                                                                                                    'reply_once',
                                                                                                                    'retired_writable_neighbor',
                                                                                                                    'selected_native_reply',
                                                                                                                    'send_cap_case',
                                                                                                                    'sending_slot',
                                                                                                                    'serve_checkpoint',
                                                                                                                    'setup_failure_case',
                                                                                                                    'staged_native_reply',
                                                                                                                    'terminal_slot',
                                                                                                                    'writable_retirement_neighbors']},
                    'series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                                  'helpers': ['assert_fixed_redaction',
                                                                                                                              'assert_no_extra_query',
                                                                                                                              'check_checkpoint_facts',
                                                                                                                              'checkpoint_queries',
                                                                                                                              'cleanup_boundary_neighbors',
                                                                                                                              'completion',
                                                                                                                              'final_checkpoint_case',
                                                                                                                              'issue_boundary_neighbors',
                                                                                                                              'modeled_read_boundary_neighbors',
                                                                                                                              'product_ok',
                                                                                                                              'queued_native_reply',
                                                                                                                              'receive_boundary_neighbors',
                                                                                                                              'reply_once',
                                                                                                                              'retired_writable_neighbor',
                                                                                                                              'selected_native_reply',
                                                                                                                              'send_cap_case',
                                                                                                                              'sending_slot',
                                                                                                                              'serve_checkpoint',
                                                                                                                              'setup_failure_case',
                                                                                                                              'staged_native_reply',
                                                                                                                              'terminal_slot',
                                                                                                                              'writable_retirement_neighbors']},
                    'series::tests::genuine_terminal_report_admits_only_the_three_position_domain': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                     'helpers': ['assert_fixed_redaction',
                                                                                                                 'assert_no_extra_query',
                                                                                                                 'check_checkpoint_facts',
                                                                                                                 'checkpoint_queries',
                                                                                                                 'cleanup_boundary_neighbors',
                                                                                                                 'completion',
                                                                                                                 'final_checkpoint_case',
                                                                                                                 'issue_boundary_neighbors',
                                                                                                                 'modeled_read_boundary_neighbors',
                                                                                                                 'product_ok',
                                                                                                                 'queued_native_reply',
                                                                                                                 'receive_boundary_neighbors',
                                                                                                                 'reply_once',
                                                                                                                 'retired_writable_neighbor',
                                                                                                                 'selected_native_reply',
                                                                                                                 'send_cap_case',
                                                                                                                 'sending_slot',
                                                                                                                 'serve_checkpoint',
                                                                                                                 'setup_failure_case',
                                                                                                                 'staged_native_reply',
                                                                                                                 'terminal_slot',
                                                                                                                 'writable_retirement_neighbors']},
                    'series::tests::partial_report_preserves_primary_and_both_cleanup_causes': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                'helpers': ['assert_fixed_redaction',
                                                                                                            'assert_no_extra_query',
                                                                                                            'check_checkpoint_facts',
                                                                                                            'checkpoint_queries',
                                                                                                            'cleanup_boundary_neighbors',
                                                                                                            'completion',
                                                                                                            'final_checkpoint_case',
                                                                                                            'issue_boundary_neighbors',
                                                                                                            'modeled_read_boundary_neighbors',
                                                                                                            'product_ok',
                                                                                                            'queued_native_reply',
                                                                                                            'receive_boundary_neighbors',
                                                                                                            'reply_once',
                                                                                                            'retired_writable_neighbor',
                                                                                                            'selected_native_reply',
                                                                                                            'send_cap_case',
                                                                                                            'sending_slot',
                                                                                                            'serve_checkpoint',
                                                                                                            'setup_failure_case',
                                                                                                            'staged_native_reply',
                                                                                                            'terminal_slot',
                                                                                                            'writable_retirement_neighbors']},
                    'series::tests::post_commit_control_preserves_header_and_measurement_refusal': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                    'helpers': ['assert_fixed_redaction',
                                                                                                                'assert_no_extra_query',
                                                                                                                'check_checkpoint_facts',
                                                                                                                'checkpoint_queries',
                                                                                                                'cleanup_boundary_neighbors',
                                                                                                                'completion',
                                                                                                                'final_checkpoint_case',
                                                                                                                'issue_boundary_neighbors',
                                                                                                                'modeled_read_boundary_neighbors',
                                                                                                                'product_ok',
                                                                                                                'queued_native_reply',
                                                                                                                'receive_boundary_neighbors',
                                                                                                                'reply_once',
                                                                                                                'retired_writable_neighbor',
                                                                                                                'selected_native_reply',
                                                                                                                'send_cap_case',
                                                                                                                'sending_slot',
                                                                                                                'serve_checkpoint',
                                                                                                                'setup_failure_case',
                                                                                                                'staged_native_reply',
                                                                                                                'terminal_slot',
                                                                                                                'writable_retirement_neighbors']},
                    'series::tests::raw_config_and_tokens_have_mandatory_bounded_admission': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                              'helpers': ['assert_fixed_redaction',
                                                                                                          'assert_no_extra_query',
                                                                                                          'check_checkpoint_facts',
                                                                                                          'checkpoint_queries',
                                                                                                          'cleanup_boundary_neighbors',
                                                                                                          'completion',
                                                                                                          'final_checkpoint_case',
                                                                                                          'issue_boundary_neighbors',
                                                                                                          'modeled_read_boundary_neighbors',
                                                                                                          'product_ok',
                                                                                                          'queued_native_reply',
                                                                                                          'receive_boundary_neighbors',
                                                                                                          'reply_once',
                                                                                                          'retired_writable_neighbor',
                                                                                                          'selected_native_reply',
                                                                                                          'send_cap_case',
                                                                                                          'sending_slot',
                                                                                                          'serve_checkpoint',
                                                                                                          'setup_failure_case',
                                                                                                          'staged_native_reply',
                                                                                                          'terminal_slot',
                                                                                                          'writable_retirement_neighbors']},
                    'series::tests::receiver_rejects_duplicate_foreign_and_reused_identity_atomically': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                         'helpers': ['assert_fixed_redaction',
                                                                                                                     'assert_no_extra_query',
                                                                                                                     'check_checkpoint_facts',
                                                                                                                     'checkpoint_queries',
                                                                                                                     'cleanup_boundary_neighbors',
                                                                                                                     'completion',
                                                                                                                     'final_checkpoint_case',
                                                                                                                     'issue_boundary_neighbors',
                                                                                                                     'modeled_read_boundary_neighbors',
                                                                                                                     'product_ok',
                                                                                                                     'queued_native_reply',
                                                                                                                     'receive_boundary_neighbors',
                                                                                                                     'reply_once',
                                                                                                                     'retired_writable_neighbor',
                                                                                                                     'selected_native_reply',
                                                                                                                     'send_cap_case',
                                                                                                                     'sending_slot',
                                                                                                                     'serve_checkpoint',
                                                                                                                     'setup_failure_case',
                                                                                                                     'staged_native_reply',
                                                                                                                     'terminal_slot',
                                                                                                                     'writable_retirement_neighbors']},
                    'series::tests::selected_native_datagram_after_retirement_is_charged_without_publication': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                                'helpers': ['assert_fixed_redaction',
                                                                                                                            'assert_no_extra_query',
                                                                                                                            'check_checkpoint_facts',
                                                                                                                            'checkpoint_queries',
                                                                                                                            'cleanup_boundary_neighbors',
                                                                                                                            'completion',
                                                                                                                            'final_checkpoint_case',
                                                                                                                            'issue_boundary_neighbors',
                                                                                                                            'modeled_read_boundary_neighbors',
                                                                                                                            'product_ok',
                                                                                                                            'queued_native_reply',
                                                                                                                            'receive_boundary_neighbors',
                                                                                                                            'reply_once',
                                                                                                                            'retired_writable_neighbor',
                                                                                                                            'selected_native_reply',
                                                                                                                            'send_cap_case',
                                                                                                                            'sending_slot',
                                                                                                                            'serve_checkpoint',
                                                                                                                            'setup_failure_case',
                                                                                                                            'staged_native_reply',
                                                                                                                            'terminal_slot',
                                                                                                                            'writable_retirement_neighbors']},
                    'series::tests::setup_and_registration_failures_preserve_actual_prior_completion': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                        'helpers': ['assert_fixed_redaction',
                                                                                                                    'assert_no_extra_query',
                                                                                                                    'check_checkpoint_facts',
                                                                                                                    'checkpoint_queries',
                                                                                                                    'cleanup_boundary_neighbors',
                                                                                                                    'completion',
                                                                                                                    'final_checkpoint_case',
                                                                                                                    'issue_boundary_neighbors',
                                                                                                                    'modeled_read_boundary_neighbors',
                                                                                                                    'product_ok',
                                                                                                                    'queued_native_reply',
                                                                                                                    'receive_boundary_neighbors',
                                                                                                                    'reply_once',
                                                                                                                    'retired_writable_neighbor',
                                                                                                                    'selected_native_reply',
                                                                                                                    'send_cap_case',
                                                                                                                    'sending_slot',
                                                                                                                    'serve_checkpoint',
                                                                                                                    'setup_failure_case',
                                                                                                                    'staged_native_reply',
                                                                                                                    'terminal_slot',
                                                                                                                    'writable_retirement_neighbors']},
                    'series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                            'helpers': ['assert_fixed_redaction',
                                                                                                                        'assert_no_extra_query',
                                                                                                                        'check_checkpoint_facts',
                                                                                                                        'checkpoint_queries',
                                                                                                                        'cleanup_boundary_neighbors',
                                                                                                                        'completion',
                                                                                                                        'final_checkpoint_case',
                                                                                                                        'issue_boundary_neighbors',
                                                                                                                        'modeled_read_boundary_neighbors',
                                                                                                                        'product_ok',
                                                                                                                        'queued_native_reply',
                                                                                                                        'receive_boundary_neighbors',
                                                                                                                        'reply_once',
                                                                                                                        'retired_writable_neighbor',
                                                                                                                        'selected_native_reply',
                                                                                                                        'send_cap_case',
                                                                                                                        'sending_slot',
                                                                                                                        'serve_checkpoint',
                                                                                                                        'setup_failure_case',
                                                                                                                        'staged_native_reply',
                                                                                                                        'terminal_slot',
                                                                                                                        'writable_retirement_neighbors']},
                    'series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                         'helpers': ['assert_fixed_redaction',
                                                                                                                     'assert_no_extra_query',
                                                                                                                     'check_checkpoint_facts',
                                                                                                                     'checkpoint_queries',
                                                                                                                     'cleanup_boundary_neighbors',
                                                                                                                     'completion',
                                                                                                                     'final_checkpoint_case',
                                                                                                                     'issue_boundary_neighbors',
                                                                                                                     'modeled_read_boundary_neighbors',
                                                                                                                     'product_ok',
                                                                                                                     'queued_native_reply',
                                                                                                                     'receive_boundary_neighbors',
                                                                                                                     'reply_once',
                                                                                                                     'retired_writable_neighbor',
                                                                                                                     'selected_native_reply',
                                                                                                                     'send_cap_case',
                                                                                                                     'sending_slot',
                                                                                                                     'serve_checkpoint',
                                                                                                                     'setup_failure_case',
                                                                                                                     'staged_native_reply',
                                                                                                                     'terminal_slot',
                                                                                                                     'writable_retirement_neighbors']},
                    'series::tests::synchronous_counter_boundaries_do_not_panic_and_preserve_exact_effects': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                              'helpers': ['assert_fixed_redaction',
                                                                                                                          'assert_no_extra_query',
                                                                                                                          'check_checkpoint_facts',
                                                                                                                          'checkpoint_queries',
                                                                                                                          'cleanup_boundary_neighbors',
                                                                                                                          'completion',
                                                                                                                          'final_checkpoint_case',
                                                                                                                          'issue_boundary_neighbors',
                                                                                                                          'modeled_read_boundary_neighbors',
                                                                                                                          'product_ok',
                                                                                                                          'queued_native_reply',
                                                                                                                          'receive_boundary_neighbors',
                                                                                                                          'reply_once',
                                                                                                                          'retired_writable_neighbor',
                                                                                                                          'selected_native_reply',
                                                                                                                          'send_cap_case',
                                                                                                                          'sending_slot',
                                                                                                                          'serve_checkpoint',
                                                                                                                          'setup_failure_case',
                                                                                                                          'staged_native_reply',
                                                                                                                          'terminal_slot',
                                                                                                                          'writable_retirement_neighbors']},
                    'series::tests::transfer_control_and_irreversible_retirement_are_distinct': {'source': 'crates/nevrotick-engine/tests/private/series.rs',
                                                                                                 'helpers': ['assert_fixed_redaction',
                                                                                                             'assert_no_extra_query',
                                                                                                             'check_checkpoint_facts',
                                                                                                             'checkpoint_queries',
                                                                                                             'cleanup_boundary_neighbors',
                                                                                                             'completion',
                                                                                                             'final_checkpoint_case',
                                                                                                             'issue_boundary_neighbors',
                                                                                                             'modeled_read_boundary_neighbors',
                                                                                                             'product_ok',
                                                                                                             'queued_native_reply',
                                                                                                             'receive_boundary_neighbors',
                                                                                                             'reply_once',
                                                                                                             'retired_writable_neighbor',
                                                                                                             'selected_native_reply',
                                                                                                             'send_cap_case',
                                                                                                             'sending_slot',
                                                                                                             'serve_checkpoint',
                                                                                                             'setup_failure_case',
                                                                                                             'staged_native_reply',
                                                                                                             'terminal_slot',
                                                                                                             'writable_retirement_neighbors']},
                    'session::tests::completion_classification_covers_all_end_and_failure_combinations': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                          'helpers': ['config',
                                                                                                                      'idle_plan',
                                                                                                                      'product_ok']},
                    'session::tests::entered_runtime_refusals_preserve_prepare_rollback_and_retire_consumed_execution': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                                         'helpers': ['config',
                                                                                                                                     'idle_plan',
                                                                                                                                     'product_ok']},
                    'session::tests::equal_origin_foreign_owners_produce_distinct_genuine_report_memberships': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                                'helpers': ['config',
                                                                                                                            'idle_plan',
                                                                                                                            'product_ok']},
                    'session::tests::forgotten_plan_keeps_busy_until_idempotent_terminal_retirement': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                       'helpers': ['config',
                                                                                                                   'idle_plan',
                                                                                                                   'product_ok']},
                    'session::tests::private_busy_and_retired_states_reject_before_driver_preparation': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                         'helpers': ['config',
                                                                                                                     'idle_plan',
                                                                                                                     'product_ok']},
                    'session::tests::private_drop_contradictions_retire_without_entering_execution': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                      'helpers': ['config',
                                                                                                                  'idle_plan',
                                                                                                                  'product_ok']},
                    'session::tests::private_plan_state_chart_and_missing_inner_contradictions_retire': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                         'helpers': ['config',
                                                                                                                     'idle_plan',
                                                                                                                     'product_ok']},
                    'session::tests::reference_refusal_diagnostics_and_exact_typed_sources_are_fixed': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                        'helpers': ['config',
                                                                                                                    'idle_plan',
                                                                                                                    'product_ok']},
                    'session::tests::series_coherence_debug_contains_only_fixed_scalar_causes': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                 'helpers': ['config',
                                                                                                             'idle_plan',
                                                                                                             'product_ok']},
                    'session::tests::session_debug_and_driver_error_sources_are_fixed_and_typed': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                   'helpers': ['config',
                                                                                                               'idle_plan',
                                                                                                               'product_ok']},
                    'session::tests::synthetic_capture_coherence_preserves_nonzero_limit_and_checked_failure_precedence': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                                           'helpers': ['config',
                                                                                                                                       'idle_plan',
                                                                                                                                       'product_ok']},
                    'session::tests::synthetic_execution_unwind_retires_and_releases_prepared_resources': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                           'helpers': ['config',
                                                                                                                       'idle_plan',
                                                                                                                       'product_ok']},
                    'session::tests::synthetic_started_guard_drop_retires_without_reopening_capacity': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                        'helpers': ['config',
                                                                                                                    'idle_plan',
                                                                                                                    'product_ok']},
                    'session::tests::unused_plan_drop_releases_capacity_without_replacing_the_chart': {'source': 'crates/nevrotick-engine/tests/private/session.rs',
                                                                                                       'helpers': ['config',
                                                                                                                   'idle_plan',
                                                                                                                   'product_ok']},
                    'session_reference_refusals_preserve_original_causes_and_checked_positions': {'source': 'crates/nevrotick-engine/tests/session.rs',
                                                                                                  'helpers': ['actual_report',
                                                                                                              'checked_measurement',
                                                                                                              'checked_reference',
                                                                                                              'config',
                                                                                                              'fact',
                                                                                                              'idle_plan',
                                                                                                              'inspect',
                                                                                                              'inspect_reference',
                                                                                                              'product_ok',
                                                                                                              'run_three',
                                                                                                              'snapshots',
                                                                                                              'unchanged']},
                    'stale_execution_retires_owner_without_refreshing_retained_facts': {'source': 'crates/nevrotick-engine/tests/session.rs',
                                                                                        'helpers': ['actual_report',
                                                                                                    'checked_measurement',
                                                                                                    'checked_reference',
                                                                                                    'config',
                                                                                                    'fact',
                                                                                                    'idle_plan',
                                                                                                    'inspect',
                                                                                                    'inspect_reference',
                                                                                                    'product_ok',
                                                                                                    'run_three',
                                                                                                    'snapshots',
                                                                                                    'unchanged']},
                    'stop_preserves_first_completion_and_marks_third_not_started': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                    'helpers': ['check_exact_measurement',
                                                                                                'check_ratio',
                                                                                                'common_report_checks',
                                                                                                'facts',
                                                                                                'healthy_report_checks',
                                                                                                'no_extra_query',
                                                                                                'product_ok',
                                                                                                'profile',
                                                                                                'run_bound_case',
                                                                                                'run_case',
                                                                                                'serve',
                                                                                                'slot',
                                                                                                'terminal_case']},
                    'three_real_requests_retain_distinct_identity_and_one_exact_origin': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                          'helpers': ['check_exact_measurement',
                                                                                                      'check_ratio',
                                                                                                      'common_report_checks',
                                                                                                      'facts',
                                                                                                      'healthy_report_checks',
                                                                                                      'no_extra_query',
                                                                                                      'product_ok',
                                                                                                      'profile',
                                                                                                      'run_bound_case',
                                                                                                      'run_case',
                                                                                                      'serve',
                                                                                                      'slot',
                                                                                                      'terminal_case']},
                    'total_deadline_preserves_first_completion_and_prevents_third_send': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                          'helpers': ['check_exact_measurement',
                                                                                                      'check_ratio',
                                                                                                      'common_report_checks',
                                                                                                      'facts',
                                                                                                      'healthy_report_checks',
                                                                                                      'no_extra_query',
                                                                                                      'product_ok',
                                                                                                      'profile',
                                                                                                      'run_bound_case',
                                                                                                      'run_case',
                                                                                                      'serve',
                                                                                                      'slot',
                                                                                                      'terminal_case']},
                    'two_requested_series_share_chart_keep_distinct_runs_and_old_handles_isolated': {'source': 'crates/nevrotick-engine/tests/session.rs',
                                                                                                     'helpers': ['actual_report',
                                                                                                                 'checked_measurement',
                                                                                                                 'checked_reference',
                                                                                                                 'config',
                                                                                                                 'fact',
                                                                                                                 'idle_plan',
                                                                                                                 'inspect',
                                                                                                                 'inspect_reference',
                                                                                                                 'product_ok',
                                                                                                                 'run_three',
                                                                                                                 'snapshots',
                                                                                                                 'unchanged']},
                    'unavailable_measurements_still_consume_once_after_rejected_neighbors': {'source': 'crates/nevrotick-engine/tests/series.rs',
                                                                                             'helpers': ['check_exact_measurement',
                                                                                                         'check_ratio',
                                                                                                         'common_report_checks',
                                                                                                         'facts',
                                                                                                         'healthy_report_checks',
                                                                                                         'no_extra_query',
                                                                                                         'product_ok',
                                                                                                         'profile',
                                                                                                         'run_bound_case',
                                                                                                         'run_case',
                                                                                                         'serve',
                                                                                                         'slot',
                                                                                                         'terminal_case']}}}

SESSION_ORDINARY_STATUS = "passed-session-all"
SESSION_ORDINARY_LANES = (
    "boundary-control", "format", "compiler", "clippy", "engine-tests", "engine-doctests",
    "engine-optimized-tests", "engine-rustdoc", "windows-compiler", "windows-clippy",
)
SESSION_CONTROL_PANIC_TEST = "session::tests::synthetic_execution_unwind_retires_and_releases_prepared_resources"
SESSION_CONTROL_PANIC_MARKER = "deliberate session lifecycle unwind"
SESSION_REQUIRED_TESTS = frozenset({
    "session::tests::unused_plan_drop_releases_capacity_without_replacing_the_chart",
    "session::tests::forgotten_plan_keeps_busy_until_idempotent_terminal_retirement",
    "session::tests::entered_runtime_refusals_preserve_prepare_rollback_and_retire_consumed_execution",
    "session::tests::private_busy_and_retired_states_reject_before_driver_preparation",
    "session::tests::private_plan_state_chart_and_missing_inner_contradictions_retire",
    "session::tests::private_drop_contradictions_retire_without_entering_execution",
    "session::tests::equal_origin_foreign_owners_produce_distinct_genuine_report_memberships",
    "session::tests::synthetic_started_guard_drop_retires_without_reopening_capacity",
    "session::tests::synthetic_execution_unwind_retires_and_releases_prepared_resources",
    "session::tests::completion_classification_covers_all_end_and_failure_combinations",
    "session::tests::session_debug_and_driver_error_sources_are_fixed_and_typed",
    "session::tests::reference_refusal_diagnostics_and_exact_typed_sources_are_fixed",
    "two_requested_series_share_chart_keep_distinct_runs_and_old_handles_isolated",
    "stale_execution_retires_owner_without_refreshing_retained_facts",
    "clean_stop_and_real_deadline_each_allow_a_fresh_preparation",
    "session_reference_refusals_preserve_original_causes_and_checked_positions",
})
SESSION_PRIVATE_SEMANTIC_HELPERS = ("config", "idle_plan", "product_ok")
SESSION_PUBLIC_SEMANTIC_HELPERS = (
    "actual_report", "checked_measurement", "checked_reference", "config", "fact", "idle_plan",
    "inspect", "inspect_reference", "product_ok", "run_three", "snapshots", "unchanged",
)
SESSION_PROCESSING = "crates/nevrotick-engine/tests/private/processing_guards.rs"
SESSION_SOURCE_HELPERS = {
    SESSION_PRIVATE: SESSION_PRIVATE_SEMANTIC_HELPERS,
    SESSION_EXTERNAL: SESSION_PUBLIC_SEMANTIC_HELPERS,
    "crates/nevrotick-engine/tests/private/measurement.rs": (
        "assert_relative_phase", "nominal_relative_fixture", "product_ok"),
    "crates/nevrotick-engine/tests/private/series.rs": (
        "assert_fixed_redaction", "check_checkpoint_facts", "checkpoint_queries", "cleanup_boundary_neighbors",
        "completion", "final_checkpoint_case", "issue_boundary_neighbors", "product_ok",
        "assert_no_extra_query", "modeled_read_boundary_neighbors", "queued_native_reply", "reply_once",
        "selected_native_reply", "send_cap_case", "setup_failure_case", "staged_native_reply",
        "receive_boundary_neighbors", "retired_writable_neighbor", "sending_slot", "serve_checkpoint",
        "terminal_slot", "writable_retirement_neighbors"),
    SESSION_PROCESSING: (
        "assert_observed_difference", "check_actual_measurement_record", "check_completed_cleanup_error",
        "check_fact_getters", "check_failure_projection", "check_independent_staged_context_guards",
        "check_io_report", "check_literal_stage_rejections", "check_observed_measurement", "check_outcomes",
        "check_partial_measurement_write_failure", "check_relative_refusals", "check_report_projection",
        "check_send_completion", "check_shared_measurement_writer", "check_target",
        "observed_reference_ticks", "product_ok"),
    "crates/nevrotick-engine/src/quality/tests.rs": (
        "assumptions", "exact", "facts", "facts_with_wall", "product_ok"),
    "crates/nevrotick-engine/tests/series.rs": (
        "check_exact_measurement", "check_ratio", "common_report_checks", "facts", "healthy_report_checks",
        "no_extra_query", "product_ok", "profile", "run_bound_case", "run_case", "serve", "slot", "terminal_case"),
    "crates/nevrotick-engine/tests/quality.rs": (
        "assumptions", "check_near", "check_quality", "check_ratio", "check_report", "diagnostic",
        "facts", "product_ok", "run_series", "serve", "slot"),
    "crates/nevrotick-engine/tests/private/argument_guards.rs": ("product_ok",),
    "crates/nevrotick-engine/tests/runtime_boundary.rs": (
        "argument_rejection", "bounded_fields", "cli_argument_boundary_rejections", "cli_case",
        "experimental_argument_rejections", "fields", "integer", "measurement_arithmetic", "measurement_record",
        "measurement_transcript_controls", "non_text_arguments", "product_ok", "result_record", "signed_integer",
        "timeout_elapsed_controls", "transcript", "transcript_controls"),
}
SESSION_SOURCE_HELPERS = {name: tuple(sorted(helpers)) for name, helpers in SESSION_SOURCE_HELPERS.items()}
SESSION_SOURCE_HELPERS[SESSION_PROCESSING] = tuple(sorted((*SESSION_SOURCE_HELPERS[SESSION_PROCESSING],
    "check_guard", "inject_io_case", "commit_neighbor", "check_receive_cap", "two_origin_rejections",
    "check_retired_selected_read", "check_typed_oversize_accounting", "check_commit_processing_boundaries")))
SESSION_TEST_SOURCES = {name: (SESSION_PRIVATE if name.startswith("session::tests::") else SESSION_EXTERNAL)
                        for name in SESSION_REQUIRED_TESTS}
SESSION_TEST_SOURCES.update({"session::tests::" + name: SESSION_PRIVATE for name in (
    "series_coherence_debug_contains_only_fixed_scalar_causes",
    "synthetic_capture_coherence_preserves_nonzero_limit_and_checked_failure_precedence")})
for _prefix, _source in (
    ("measurement::tests::", "crates/nevrotick-engine/tests/private/measurement.rs"),
    ("processing_guard_tests::", SESSION_PROCESSING),
    ("argument_guard_tests::", "crates/nevrotick-engine/tests/private/argument_guards.rs"),
):
    SESSION_TEST_SOURCES.update({name: _source for name in REQUIRED_TESTS if name.startswith(_prefix)})
SESSION_TEST_SOURCES.update({"measurement::tests::" + name:
    "crates/nevrotick-engine/tests/private/measurement.rs" for name in (
        "coherence_helper_crossed_brackets_keep_touching_and_separated_neighbors",
        "series_coherence_empty_single_and_holes_keep_original_positions",
        "series_coherence_all_nine_positions_influence_the_maximum",
        "series_coherence_fixed_first_baseline_detects_accumulated_drift",
        "series_coherence_rejects_each_own_and_sequential_order_violation",
        "series_coherence_full_order_validation_precedes_earlier_arithmetic",
        "series_coherence_late_arithmetic_failure_wins_over_an_earlier_gap",
        "series_coherence_full_u64_residual_keeps_its_next_rejection")})
SESSION_TEST_SOURCES.update({name: ("crates/nevrotick-engine/src/quality/tests.rs"
    if name.startswith("quality::tests::") else "crates/nevrotick-engine/tests/quality.rs")
    for name in QUALITY_REQUIRED_TESTS})
SESSION_TEST_SOURCES.update({name: ("crates/nevrotick-engine/tests/private/series.rs"
    if name.startswith("series::tests::") else "crates/nevrotick-engine/tests/series.rs")
    for name in SERIES_REQUIRED_TESTS | SERIES_CHECKPOINT_REQUIRED_TESTS
    if "semantic_evidence_accepts_only_exact_named_admitted_sites" not in name})
SESSION_TEST_SOURCES.update({name: "crates/nevrotick-engine/tests/runtime_boundary.rs" for name in (
    "cli_worker_binary_preserves_literal_transport_contracts",
    "cli_worker_binary_rejects_arguments_and_observes_sink_failures")})
SESSION_TEST_SOURCES[SESSION_WITNESS_COUNTER_TEST] = "crates/nevrotick-engine/tests/private/series.rs"
SESSION_REQUIRED_TESTS = frozenset(SESSION_TEST_SOURCES)
SESSION_CHILD_CASES = frozenset({
    SESSION_WITNESS_COUNTER_TEST,
    "series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor",
    "series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor",
    "series::tests::setup_and_registration_failures_preserve_actual_prior_completion",
    "series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions",
    "three_real_requests_retain_distinct_identity_and_one_exact_origin",
    "ipv6_real_requests_retain_distinct_identity_and_one_exact_origin",
    "unavailable_measurements_still_consume_once_after_rejected_neighbors",
    "middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt",
    "stop_preserves_first_completion_and_marks_third_not_started",
    "invalidation_preserves_first_completion_and_marks_third_not_started",
    "total_deadline_preserves_first_completion_and_prevents_third_send",
    "processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals",
})
SESSION_EXCLUDED_FACTS = frozenset({"CLI failed or wrote unexpected stderr"})
SESSION_FORMATTED_ASSERTIONS = {
    (SESSION_PROCESSING, "check_independent_staged_context_guards",
     "an independent {field} mismatch consumed the pending request"):
        tuple("an independent " + field + " mismatch consumed the pending request"
              for field in ("identity", "peer", "local", "profile", "run")),
}


def session_test_inventory(policy: dict) -> tuple[list[str], list[str]]:
    """Retain the independently executed ordinary identities as the fixed oracle."""
    inventory = policy.get("test_inventory")
    if (type(SESSION_ADMISSION) is not dict
            or inventory != SESSION_ADMISSION.get("test_inventory")
            or type(inventory) is not dict or set(inventory) != {"ordinary", "doctests", "sha256"}):
        raise Incomplete("Session actual ordinary inventory is not frozen.")
    for field in ("ordinary", "doctests"):
        names = inventory[field]
        if (type(names) is not list or not 0 < len(names) <= 1024
                or any(type(name) is not str or not 0 < len(name) <= 4096 for name in names)
                or names != sorted(set(names))):
            raise Incomplete("Session native inventory is malformed or repeated.")
    ordinary, docs = inventory["ordinary"], inventory["doctests"]
    session_docs = [name for name in docs if name.startswith(SESSION_FILE + " - session::")]
    if (not any(name.endswith(" - compile fail") for name in session_docs)
            or not any(not name.endswith((" - compile fail", " - compile")) for name in session_docs)):
        raise Incomplete("Session inventory lacks actual legal and negative external doctests.")
    expected = {"ordinary": COMMON.canonical_sha256(ordinary),
                "doctests": COMMON.canonical_sha256(docs),
                "baseline": COMMON.canonical_sha256(sorted(ordinary + docs))}
    if inventory["sha256"] != expected or len(set(ordinary + docs)) != len(ordinary + docs):
        raise Incomplete("Session native inventory digest or identity differs.")
    selected = SESSION_ADMISSION.get("semantic_tests")
    if (type(selected) is not dict or set(selected) != SESSION_REQUIRED_TESTS
            or not set(selected) <= set(ordinary)
            or any(type(row) is not dict or row.get("source") != SESSION_TEST_SOURCES.get(name)
                   for name, row in selected.items())):
        raise Incomplete("Session requires its actual private and external semantic controls.")
    for name, row in selected.items():
        if (type(row) is not dict or set(row) != {"source", "helpers"}
                or row["source"] not in SESSION_SOURCE_HELPERS
                or type(row["helpers"]) is not list or row["helpers"] != sorted(set(row["helpers"]))
                or row["helpers"] != list(SESSION_SOURCE_HELPERS[row["source"]])
                or any(type(helper) is not str or re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", helper) is None
                       for helper in row["helpers"])):
            raise Incomplete("Session semantic owner/helper inventory is malformed.")
    return ordinary, docs


def session_inventory_bindings(policy: dict, *, required: bool = False) -> dict:
    """Discovery grants no campaign until one exact nonempty inventory is reviewed."""
    bindings = policy.get("mutation_inventories")
    if type(bindings) is not dict:
        raise Incomplete("Session mutation inventory must be an object.")
    if not bindings and not required:
        return {}
    if set(bindings) != set(SESSION_SUBJECTS):
        raise Incomplete("Session cohort needs all three complete reviewed inventories.")
    for item in bindings.values():
        if (type(item) is not dict or set(item) != {"count", "sha256"}
                or type(item["count"]) is not int or not 0 < item["count"] <= 512
                or type(item["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", item["sha256"]) is None):
            raise Incomplete("Session campaign needs its exact complete reviewed inventory.")
    return bindings


def validate_session_equivalence_scope(entries: object) -> None:
    """Permit no E or all five exact observed current-source misses, never kills."""
    if type(entries) is not list:
        raise Incomplete("Session equivalences require an exact list.")
    if not entries:
        return
    required = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    if (len(entries) != 5 or any(type(entry) is not dict or set(entry) != required for entry in entries)
            or any(type(entry["id"]) is not str for entry in entries)
            or {entry["id"] for entry in entries} != set(SESSION_EQUIVALENCE_DESCRIPTORS)):
        raise Incomplete("Session equivalences require the complete exact five-ID current cohort.")
    for entry in entries:
        filename, source, descriptor = SESSION_EQUIVALENCE_DESCRIPTORS[entry["id"]]
        mutant, proof = entry["mutant"], entry["proof"]
        if (entry["source_sha256"] != source or type(mutant) is not dict
                or mutant.get("file") != filename or COMMON.canonical_sha256(mutant) != descriptor
                or type(entry["candidate_sha256"]) is not str
                or re.fullmatch(r"[a-f0-9]{64}", entry["candidate_sha256"]) is None
                or type(proof) is not dict
                or set(proof) != {"path", "sha256", "section", "reviewer", "verdict"}
                or proof["path"] != SESSION_EQUIVALENCE_PROOF or proof["section"] != entry["id"]
                or proof["verdict"] != "independently-justified-equivalent"
                or type(proof["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", proof["sha256"]) is None
                or type(proof["reviewer"]) is not str or not 0 < len(proof["reviewer"].strip()) <= 128
                or any(type(entry[key]) is not str or not 0 < len(entry[key].strip()) <= maximum
                       for key, maximum in (("requirement", 256), ("rationale", 4096)))):
            raise Incomplete("Session equivalence leaves its exact source, native descriptor or reviewed dossier identity.")


def validate_session_policy(policy: dict) -> dict:
    """Reject all execution until actual candidate, receipt and controls are frozen."""
    if type(SESSION_ADMISSION) is not dict:
        raise Incomplete("Session ordinary evidence has not been executed and frozen.")
    extras = {"candidate_sha256", "source_file_count", "ordinary_evidence", "test_inventory",
              "inputs_sha256", "mutation_inventories", "mutation_witnesses", "scope"}
    if (type(policy) is not dict or set(policy) != set(SESSION_FIXED_POLICY) | extras
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in SESSION_FIXED_POLICY.items() if key != "mutation_equivalences")
            or any(policy.get(key) != SESSION_ADMISSION.get(key)
                   for key in ("candidate_sha256", "source_file_count", "ordinary_evidence"))
            or type(policy.get("candidate_sha256")) is not str
            or re.fullmatch(r"[a-f0-9]{64}", policy["candidate_sha256"]) is None
            or type(policy.get("source_file_count")) is not int or policy["source_file_count"] <= 0
            or type(policy.get("scope")) is not str or not 0 < len(policy["scope"]) <= 2048):
        raise Incomplete("Session policy changes its fixed scope, candidate, bounds or ordinary receipt.")
    pins = policy.get("inputs_sha256")
    if (type(pins) is not dict or set(pins) != set(SESSION_BOUND_INPUTS)
            or any(type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None
                   for digest in pins.values())):
        raise Incomplete("Session source/test/helper custody is incomplete.")
    if (type(policy["ordinary_evidence"]) is not dict
            or policy["ordinary_evidence"].get("status") != SESSION_ORDINARY_STATUS):
        raise Incomplete("Session ordinary receipt must establish the complete successful lane.")
    session_test_inventory(policy)
    session_inventory_bindings(policy)
    validate_session_equivalence_scope(policy["mutation_equivalences"])
    if any(entry["candidate_sha256"] != policy["candidate_sha256"] for entry in policy["mutation_equivalences"]):
        raise Incomplete("Session equivalence differs from its complete ordinary candidate.")
    if any(entry["proof"]["sha256"] != pins[SESSION_EQUIVALENCE_PROOF] for entry in policy["mutation_equivalences"]):
        raise Incomplete("Session equivalence proof differs from its exact protected input pin.")
    if policy["mutation_witnesses"] is not None:
        session_witness_policy(policy)
    return policy


def admit_session_ordinary(policy: dict) -> dict:
    """Read and hash the same bounded actual receipt, checking its source and recipes."""
    binding = policy["ordinary_evidence"]
    if (type(binding) is not dict or set(binding) != {"receipt", "sha256", "status", "recipes_sha256"}
            or type(binding["receipt"]) is not str or not Path(binding["receipt"]).is_absolute()
            or type(binding["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", binding["sha256"]) is None
            or binding["status"] != SESSION_ORDINARY_STATUS or type(binding["recipes_sha256"]) is not dict
            or not binding["recipes_sha256"]):
        raise Incomplete("Session ordinary receipt binding is malformed.")
    raw = COMMON.read_analysis_bytes(Path(binding["receipt"]), maximum=512 * 1024)
    if hashlib.sha256(raw).hexdigest() != binding["sha256"]:
        raise Incomplete("Session ordinary receipt bytes changed.")
    try:
        receipt = json.loads(raw, object_pairs_hook=COMMON.ANALYSIS.unique_object,
                             parse_float=finite_number, parse_constant=invalid_number)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Session ordinary receipt is not strict bounded JSON.") from error
    if (type(receipt) is not dict or receipt.get("status") != binding["status"]
            or receipt.get("bookends_unchanged") is not True
            or receipt.get("candidate_sha256") != policy["candidate_sha256"]
            or type(receipt.get("source_sha256")) is not dict
            or len(receipt["source_sha256"]) != policy["source_file_count"]
            or COMMON.canonical_sha256(receipt["source_sha256"]) != policy["candidate_sha256"]
            or receipt.get("recipes_sha256") != binding["recipes_sha256"]):
        raise Incomplete("Session ordinary candidate, recipe or custody evidence differs.")
    for name, digest in binding["recipes_sha256"].items():
        path = Path(name)
        if (not path.is_absolute() or path != path.resolve(strict=True)
                or type(digest) is not str or PRODUCT.sha256(path) != digest):
            raise Incomplete("Session ordinary executed recipe bytes changed.")
    lanes = receipt.get("lanes")
    if (type(lanes) is not list or any(type(row) is not dict for row in lanes)
            or [row.get("lane") for row in lanes] != list(SESSION_ORDINARY_LANES)
            or any(row.get("failure") is not None or type(row.get("exit_code")) is not int
                   or row["exit_code"] != 0 for row in lanes)):
        raise Incomplete("Session ordinary receipt lacks all ten successful required lanes.")
    ordinary, docs = session_test_inventory(policy)
    for lane, names in (("engine-tests", ordinary), ("engine-optimized-tests", ordinary), ("engine-doctests", docs)):
        rows = [row for row in receipt.get("lanes", []) if type(row) is dict and row.get("lane") == lane]
        if (len(rows) != 1 or type(rows[0].get("tests")) is not dict
                or rows[0]["tests"].get("executed") != len(names)
                or sorted(rows[0]["tests"].get("test_names", [])) != names):
            raise Incomplete("Session ordinary receipt lacks its actual complete native controls.")
    return {**binding, "candidate_sha256": policy["candidate_sha256"], "fresh_execution": False}


def read_session_policy(root: Path) -> dict:
    """Bind the new singleton without changing any historical engine role."""
    policy = validate_session_policy(read_json(root / SESSION_POLICY))
    _, identity = PRODUCT.admit_product(root)
    if (len(identity) != policy["source_file_count"]
            or COMMON.canonical_sha256(identity) != policy["candidate_sha256"]
            or any(PRODUCT.sha256(root / name) != digest for name, digest in policy["inputs_sha256"].items())):
        raise Incomplete("Session policy differs from complete current source/helper bytes.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SESSION_SUBJECTS.values():
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(row.get("package") == SESSION_SCOPE.package and filename in row.get("files", [])
                       for row in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Each session cohort file requires exactly one critical/security registration.")
    COMMON.admit_mutation_equivalences(root, SESSION_SCOPE, policy)
    admit_session_ordinary(policy)
    return policy


def validate_session_test_execution(text: str, policy: dict, *, doctests: bool) -> dict:
    """Require all actual selected native successes, not a minimum test count."""
    session_reject_harness_failure(text)
    if re.search(r"(?m)^Error: ", text):
        raise Incomplete("Session successful native log contains an error.")
    ordinary, docs = session_test_inventory(policy)
    expected = sorted(ordinary + docs) if doctests else ordinary
    records, summaries = [], []
    for line in text.splitlines():
        if line.startswith("test ") and not line.startswith("test result:"):
            record = re.fullmatch(r"test ([^\r\n]+?) \.\.\. ok", line)
            if record is None:
                raise Incomplete("Session native test did not complete successfully.")
            records.append(record[1])
        elif line.startswith("test result:"):
            summary = re.fullmatch(r"test result: ok\. (\d{1,6}) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*", line)
            if summary is None:
                raise Incomplete("Session native summary is incomplete or unsuccessful.")
            summaries.append(int(summary[1]))
        elif line in {"ok", "FAILED", "ignored"}:
            raise Incomplete("Session native result lacks its named identity.")
    blocks = list(re.finditer(
        r"^running (\d{1,6}) tests?\n(.*?)^test result: ok\. (\d{1,6}) passed; "
        r"0 failed; 0 ignored; 0 measured; 0 filtered out;[^\r\n]*$",
        text, re.MULTILINE | re.DOTALL))
    if (not records or not summaries or sorted(records) != expected
            or len(set(records)) != len(records) or sum(summaries) != len(expected)
            or len(blocks) != len(summaries) or len(re.findall(r"(?m)^running ", text)) != len(blocks)):
        raise Incomplete("Session native execution lacks its complete frozen inventory.")
    observed = 0
    for block in blocks:
        running, body, passed = block.groups()
        rows = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. ok$", body)
        if int(running) != int(passed) or len(rows) != int(running) or re.search(r"(?m)^running ", body):
            raise Incomplete("Session successful native block has inconsistent completion counts.")
        observed += len(rows)
    if observed != len(records):
        raise Incomplete("Session native record is outside a completed successful block.")
    return {"tests_executed": len(records), "ordinary_tests": len(ordinary),
            "doctests": len(docs) if doctests else 0, "test_names": records,
            "inventory_sha256": COMMON.canonical_sha256(sorted(records))}


def session_rust_functions(source: str) -> tuple[str, dict]:
    """Extend the existing bounded body map only for explicit async test helpers."""
    masked, functions = quality_rust_functions(source)
    for match in re.finditer(r"(?m)^async fn ([A-Za-z_][A-Za-z0-9_]*)\s*\(", masked):
        opening = masked.find("{", match.end())
        if opening < 0 or match[1] in functions:
            raise Incomplete("Session async semantic helper is malformed or duplicated.")
        depth, end = 1, opening + 1
        while end < len(masked) and depth:
            depth += (masked[end] == "{") - (masked[end] == "}")
            end += 1
        if depth:
            raise Incomplete("Session async semantic helper body is unfinished.")
        functions[match[1]] = {"start": opening + 1, "end": end - 1}
    return masked, functions


def session_semantic_assertion_sites(root: Path, policy: dict) -> dict:
    """Map only explicit assertions reachable through reviewed semantic helpers."""
    session_test_inventory(policy)
    result = {"candidate_sha256": policy["candidate_sha256"], "source_sha256": {}, "tests": {}}
    for filename in sorted({row["source"] for row in SESSION_ADMISSION["semantic_tests"].values()}):
        raw = COMMON.read_analysis_bytes(root / filename, maximum=512 * 1024)
        digest = hashlib.sha256(raw).hexdigest()
        if digest != policy["inputs_sha256"][filename]:
            raise Incomplete("Session semantic source differs from admitted bytes.")
        result["source_sha256"][filename] = digest
        source = raw.decode("utf-8")
        masked, functions = session_rust_functions(source)
        for test, spec in SESSION_ADMISSION["semantic_tests"].items():
            if spec["source"] != filename:
                continue
            pending, visited, sites = [test.rsplit("::", 1)[-1]], set(), []
            while pending:
                owner = pending.pop()
                if owner in visited:
                    continue
                if owner not in functions:
                    raise Incomplete("Session semantic control/helper has no admitted body: " + owner)
                visited.add(owner)
                start, end = functions[owner]["start"], functions[owner]["end"]
                body = masked[start:end]
                pending.extend(helper for helper in spec["helpers"] if helper != "product_ok"
                               and re.search(r"\b" + helper + r"\s*\(", body))
                for match in re.finditer(r"\bproduct_ok\s*\(", body):
                    if "product_ok" not in spec["helpers"] or "product_ok" not in functions:
                        raise Incomplete("Session product success witness has no reviewed helper.")
                    guard = functions["product_ok"]
                    guard_body = source[guard["start"]:guard["end"]]
                    witness = re.fullmatch(
                        r'\s*(assert!\s*\(\s*result[.]is_ok\(\)\s*,\s*"\{fact\}"\s*\)\s*;)\s*result\s*',
                        guard_body)
                    if witness is None:
                        raise Incomplete("Session product witness must assert success and return the unchanged result.")
                    arguments = session_call_arguments(masked, source, start + match.end() - 1, end)
                    fact = session_static_fact(source, arguments)
                    offset = guard["start"] + witness.start(1)
                    sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                                  "column": offset - source.rfind("\n", 0, offset), "owner": "product_ok",
                                  "kind": "panic_assertion", "marker": fact, "exact_message": True})
                for match in re.finditer(r"\b(assert|assert_eq|assert_ne)!\s*\(", body):
                    offset = start + match.start()
                    marker = {"assert": "assertion failed:", "assert_eq": "assertion `left == right` failed",
                              "assert_ne": "assertion `left != right` failed"}[match[1]]
                    exact_message = False
                    alternatives = None
                    if match[1] == "assert":
                        opening = start + match.end() - 1
                        nested, comma, close = 0, None, opening + 1
                        while close < end:
                            char = masked[close]
                            if char == ")" and not nested:
                                break
                            if char in "([{":
                                nested += 1
                            elif char in ")]}":
                                nested -= 1
                            elif char == "," and not nested and comma is None:
                                comma = close
                            close += 1
                        condition = masked[opening + 1:comma if comma is not None else close].strip()
                        if condition == "false":
                            continue  # A fixture's unreachable fallback earns no semantic credit.
                        if comma is not None and source[comma + 1:close].strip():
                            message = re.fullmatch(r'\s*"([^"\\]*)"\s*,?\s*', source[comma + 1:close])
                            if message is None:
                                raise Incomplete("Session custom assertion requires one static literal message: "
                                                 + filename + ":" + str(source.count("\n", 0, offset) + 1))
                            marker, exact_message = message[1], True
                            if "{" in marker or "}" in marker:
                                alternatives = SESSION_FORMATTED_ASSERTIONS.get((filename, owner, marker))
                                if alternatives is None:
                                    raise Incomplete("Session assertion formatting leaves its closed reviewed cases.")
                    for marker in alternatives or (marker,):
                        sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                                      "column": offset - source.rfind("\n", 0, offset), "owner": owner,
                                      "kind": "panic_assertion", "marker": marker, "exact_message": exact_message})
                if filename != "crates/nevrotick-engine/tests/private/measurement.rs":
                    for match in re.finditer(r"\b(check|checkpoint_fact)\s*\(", body):
                        offset, opening = start + match.start(), start + match.end() - 1
                        nested, close, argument = 0, opening + 1, opening + 1
                        arguments = []
                        while close < end:
                            char = masked[close]
                            if char == ")" and not nested:
                                if source[argument:close].strip():
                                    arguments.append((argument, close))
                                break
                            if char in "([{":
                                nested += 1
                            elif char in ")]}":
                                nested -= 1
                            elif char == "," and not nested:
                                arguments.append((argument, close))
                                argument = close + 1
                            close += 1
                        fact = session_static_fact(source, arguments)
                        if fact in SESSION_EXCLUDED_FACTS:
                            continue
                        sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                                      "column": offset - source.rfind("\n", 0, offset), "owner": owner,
                                      "kind": "typed_semantic_assertion",
                                      "fact": fact,
                                      "marker": 'Error: Custom { kind: Other, error: ' + json.dumps(fact) + ' }'})
                if test == SESSION_CONTROL_PANIC_TEST and owner == test.removeprefix("session::tests::"):
                    for match in re.finditer(r"\bpanic!\s*\(", body):
                        offset = start + match.start()
                        invocation = re.match(r'panic!\(\s*"' + re.escape(SESSION_CONTROL_PANIC_MARKER) + r'"\s*\)', source[offset:])
                        if invocation is None:
                            raise Incomplete("Session unwind control's exact static panic changed.")
                        sites.append({"source": filename, "line": source.count("\n", 0, offset) + 1,
                                      "column": offset - source.rfind("\n", 0, offset), "owner": owner,
                                      "kind": "expected_control_panic", "marker": SESSION_CONTROL_PANIC_MARKER})
            if not sites:
                raise Incomplete("Session semantic control has no explicit source-owned assertion: " + test)
            if test in SESSION_CHILD_CASES:
                bridge_file = SESSION_PROCESSING if test.startswith("processing_guard_tests::") else SERIES_PROCESS_SOURCE
                bridge_raw = COMMON.read_analysis_bytes(root / bridge_file, maximum=512 * 1024)
                bridge_digest = hashlib.sha256(bridge_raw).hexdigest()
                if bridge_digest != policy["inputs_sha256"].get(bridge_file):
                    raise Incomplete("Session child bridge differs from its frozen source.")
                result["source_sha256"][bridge_file] = bridge_digest
                bridge_source = bridge_raw.decode("utf-8")
                expression = (r'assert!\(\s*evidence[.]is_none\(\),\s*"N3 exact child semantic failure assertion"'
                              if bridge_file == SESSION_PROCESSING else
                              r'assert!\(\s*completion == Completion::Passed,\s*"N3 exact child semantic failure assertion"')
                bridges = list(re.finditer(expression, bridge_source))
                if len(bridges) != 1:
                    raise Incomplete("Session requires one exact completed child bridge.")
                offset = bridges[0].start()
                sites.append({"source": bridge_file, "line": bridge_source.count("\n", 0, offset) + 1,
                              "column": offset - bridge_source.rfind("\n", 0, offset), "owner": "completed_child",
                              "kind": "child_bridge", "marker": "N3 exact child semantic failure assertion"})
            unique = {json.dumps(site, sort_keys=True): site for site in sites}
            result["tests"][test] = sorted(unique.values(), key=lambda site: (site["source"], site["line"], site["column"], site["marker"]))
    return result


def session_call_arguments(masked: str, source: str, opening: int, end: int) -> list:
    """Split one admitted helper invocation without interpreting condition strings."""
    nested, position, argument, arguments = 0, opening + 1, opening + 1, []
    while position < end:
        char = masked[position]
        if char == ")" and not nested:
            if source[argument:position].strip():
                arguments.append((argument, position))
            return arguments
        if char in "([{":
            nested += 1
        elif char in ")]}":
            nested -= 1
        elif char == "," and not nested:
            arguments.append((argument, position))
            argument = position + 1
        position += 1
    raise Incomplete("Session semantic helper invocation is unfinished.")


def session_static_fact(source: str, arguments: list) -> str:
    """Accept condition/result plus one fixed redacted source-owned literal."""
    message = (re.fullmatch(r'\s*"([^"\\]*)"\s*', source[slice(*arguments[1])])
               if len(arguments) == 2 else None)
    if (message is None or not 0 < len(message[1]) <= 160 or not message[1].isascii()
            or any(ord(char) < 32 or ord(char) == 127 for char in message[1])):
        raise Incomplete("Session helper requires one exact static semantic fact.")
    return message[1]


def session_reject_harness_failure(text: str) -> None:
    """Keep native/compiler/fixture failure incomplete even beside valid results."""
    if re.search(r"N3_HARNESS_INCOMPLETE|Q1_HARNESS_INCOMPLETE|SESSION_HARNESS_INCOMPLETE|"
                 r"error\[E\d+\]|could not compile|timed out|no space left|cannot allocate memory|"
                 r"out of memory|failed to spawn|resource temporarily unavailable|signal:\s*\d|"
                 r"SIG(?:KILL|ABRT|SEGV|BUS|ILL)|process aborted|fatal runtime error",
                 text, re.IGNORECASE | re.MULTILINE):
        raise Incomplete("Session native log contains compiler, abort or harness failure.")


def session_complete_failures(text: str) -> list:
    """Require every entered native block and failed row/section/list to complete."""
    session_reject_harness_failure(text)
    return session_complete_failure_records(text)


def session_complete_failure_records(text: str) -> list:
    """Check structure only; this never supplies credit for an unsafe native cause."""
    if any(re.fullmatch(r"test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)", line) is None
           for line in text.splitlines() if line.startswith("test ") and not line.startswith("test result:")):
        raise Incomplete("Session caught fault contains a malformed or unfinished native test row.")
    sections = list(re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)",
                               text, re.MULTILINE | re.DOTALL))
    blocks = list(re.finditer(r"^running (\d{1,6}) tests?\n(.*?)^test result: (ok|FAILED)\. "
                             r"(\d{1,6}) passed; (\d{1,6}) failed; (\d{1,6}) ignored; "
                             r"(\d{1,6}) measured; (\d{1,6}) filtered out;[^\r\n]*$", text,
                             re.MULTILINE | re.DOTALL))
    records = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", text)
    failed = [name for name, status in records if status == "FAILED"]
    if (not blocks or not failed or len(failed) != len(set(failed))
            or len(re.findall(r"(?m)^running ", text)) != len(blocks)
            or len(re.findall(r"(?m)^test result:", text)) != len(blocks)
            or sorted(match[1] for match in sections) != sorted(failed)
            or any(sum(block.start() <= match.start() < block.end() for block in blocks) != 1 for match in sections)):
        raise Incomplete("Session caught fault lacks complete unique native failure records.")
    recorded = 0
    for block in blocks:
        running, body, status, passed, failures, ignored, measured, filtered = block.groups()
        counts = tuple(map(int, (running, passed, failures, ignored, measured, filtered)))
        rows = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", body)
        names = [name for name, result in rows if result == "FAILED"]
        section_names = re.findall(r"(?m)^---- ([^\r\n]+) stdout ----$", body)
        listing = body.rsplit("\nfailures:\n", 1)
        listed = listing[-1].strip("\n").splitlines()
        if (status != ("FAILED" if counts[2] else "ok") or any(counts[3:])
                or counts[0] != counts[1] + counts[2] or len(rows) != counts[0]
                or len({name for name, _ in rows}) != len(rows)
                or sum(result == "ok" for _, result in rows) != counts[1] or len(names) != counts[2]
                or sorted(section_names) != sorted(names) or re.search(r"(?m)^running ", body)
                or (counts[2] and (len(listing) != 2
                    or any(not line.startswith("    ") or not line[4:] for line in listed)
                    or sorted(line[4:] for line in listed) != sorted(names)))
                or (not counts[2] and len(listing) != 1)):
            raise Incomplete("Session caught fault has inconsistent completed native records.")
        recorded += len(rows)
    if recorded != len(records):
        raise Incomplete("Session test row is outside a completed native block.")
    return sections


def session_source_alias(filename: str) -> str:
    """Normalize only compiler spellings for the explicit path-included tests."""
    aliases = {"crates/nevrotick-engine/src/../tests/private/" + name + ".rs":
               "crates/nevrotick-engine/tests/private/" + name + ".rs"
               for name in ("session", "series", "processing_guards", "measurement", "argument_guards", "series_process")}
    aliases["crates/nevrotick-engine/src/bin/../../tests/private/argument_guards.rs"] = (
        "crates/nevrotick-engine/tests/private/argument_guards.rs")
    return aliases.get(filename, filename)


def session_child_causes(body: str, admitted: list) -> list:
    """Bind one completed-child relay to selected static facts or assertion sites."""
    prefix = "N3_SEMANTIC_EVIDENCE "
    evidence = [line.removeprefix(prefix) for line in body.splitlines() if line.startswith(prefix)]
    if body.count("N3_SEMANTIC_EVIDENCE") != len(evidence) or len(evidence) > 1:
        raise Incomplete("Session child relay is duplicated or malformed.")
    if not evidence:
        return []
    value = evidence[0]
    if value.startswith("independent fact: "):
        fact = value.removeprefix("independent fact: ")
        matches = [site for site in admitted if site["kind"] == "typed_semantic_assertion" and site.get("fact") == fact]
    else:
        location = re.fullmatch(r"test assertion/panic at ([^\r\n]+):([1-9][0-9]*):([1-9][0-9]*)", value)
        if location is None:
            raise Incomplete("Session child supplied no admitted independent assertion.")
        filename, line, column = location.groups()
        matches = [site for site in admitted if site["kind"] == "panic_assertion"
                   and (site["source"], site["line"], site["column"]) ==
                       (session_source_alias(filename), int(line), int(column))]
    if not matches:
        raise Incomplete("Session child cause is outside the named test's frozen semantic sites.")
    # A shared product_ok assertion can represent several reviewed valid calls.
    # Every candidate is a semantic witness; no raw Result error is admitted.
    return matches


def validate_session_caught_causes(directory: Path, outcomes: dict, sites: dict, *, subject: str = "session",
                                 witness=None) -> dict:
    """Credit only exact selected session assertions; any error remains incomplete."""
    if (type(SESSION_ADMISSION) is not dict or type(sites) is not dict
            or set(sites) != {"candidate_sha256", "source_sha256", "tests"}
            or type(sites["tests"]) is not dict
            or set(sites["tests"]) != set(SESSION_ADMISSION["semantic_tests"])):
        raise Incomplete("Session caught causes need the complete admitted semantic map.")
    if subject not in SESSION_SUBJECTS:
        raise Incomplete("Session caught causes require one exact selected cohort owner.")
    if witness is not None and (subject != "series" or type(witness) is not tuple or len(witness) != 2):
        raise Incomplete("Session singleton cause admission requires its closed Series witness.")
    rows = outcomes.get("outcomes")
    if type(rows) is not list or len(rows) > 513 or any(type(row) is not dict for row in rows):
        raise Incomplete("Session caught outcomes are malformed or oversized.")
    checked = {}
    for row in rows:
        if row.get("summary") != "CaughtMutant":
            continue
        scenario = row.get("scenario")
        mutant = scenario.get("Mutant") if type(scenario) is dict else None
        name = mutant.get("name") if type(mutant) is dict else None
        if (type(name) is not str or not 0 < len(name) <= 4096 or name in checked
                or mutant.get("file") != SESSION_SUBJECTS[subject] or mutant.get("package") != SESSION_SCOPE.package):
            raise Incomplete("Session caught identity is repeated or outside its selected owner.")
        if witness is not None and (name not in SESSION_WITNESS_CASES or witness[0] != SESSION_WITNESS_CASES[name][1]):
            raise Incomplete("Session singleton cause differs from its fixed descriptor-to-test mapping.")
        relative = COMMON.analysis_output_path(row.get("log_path", ""))
        if relative.is_absolute() or not relative.parts or ".." in relative.parts:
            raise Incomplete("Session caught log escapes its owned output.")
        raw = COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)
        text = raw.decode("utf-8")
        sections = (session_complete_failures(text) if witness is None else
                    session_singleton_execution(text, witness[0], witness[1], failed=True)["sections"])
        if (text.count("panicked at ") != sum(match[2].count("panicked at ") for match in sections)
                or len(re.findall(r"(?m)^Error: ", text))
                   != sum(len(re.findall(r"(?m)^Error: ", match[2])) for match in sections)):
            raise Incomplete("Session caught fault contains an unclassified error or panic.")
        causes, secondary = [], []
        for section in sections:
            test, body = section.groups()
            if test not in sites["tests"]:
                raise Incomplete("Session failed test is outside its admitted semantic owners.")
            locations = list(re.finditer(r"(?m)^thread '([^'\r\n]+)'(?: \(\d+\))? panicked at "
                                        r"(?:/work/source/)?([^\r\n]+):(\d+):(\d+):\n([^\r\n]+)", body))
            if body.count("panicked at ") != len(locations):
                raise Incomplete("Session caught fault lacks its exact semantic assertion.")
            relayed = session_child_causes(body, sites["tests"][test])
            section_causes, controls, bridges = [], set(), 0
            for location in locations:
                thread, filename, line, column, message = location.groups()
                filename = session_source_alias(filename)
                bridge = [site for site in sites["tests"][test] if site["kind"] == "child_bridge"
                          and (site["source"], site["line"], site["column"]) == (filename, int(line), int(column))
                          and message == site["marker"]]
                if thread == test and len(bridge) == 1 and relayed:
                    bridges += 1
                    continue
                expected = [site for site in sites["tests"][test] if site["kind"] == "expected_control_panic"
                            and test == SESSION_CONTROL_PANIC_TEST and site["marker"] == SESSION_CONTROL_PANIC_MARKER
                            and (site["source"], site["line"], site["column"]) == (filename, int(line), int(column))
                            and message == site["marker"]]
                if thread == test and len(expected) == 1:
                    identity = filename, int(line), int(column)
                    if identity in controls:
                        raise Incomplete("Session expected unwind control was repeated.")
                    controls.add(identity)
                    secondary.append({"test": test, **expected[0]})
                    continue
                matched = [site for site in sites["tests"][test] if site["kind"] == "panic_assertion"
                           and (site["source"], site["line"], site["column"]) == (filename, int(line), int(column))
                           and (message == site["marker"] if site.get("exact_message") else message.startswith(site["marker"]))]
                if thread != test or len(matched) != 1:
                    raise Incomplete("Session panic is not its named source-owned semantic assertion.")
                section_causes.append({"test": test, **matched[0]})
            if bridges != bool(relayed):
                raise Incomplete("Session child evidence lacks one exact completed parent bridge.")
            section_causes.extend({"test": test, **site, "completed_child_relay": True} for site in relayed)
            for line in body.splitlines():
                if not line.startswith("Error: "):
                    continue
                matched = [site for site in sites["tests"][test]
                           if site["kind"] == "typed_semantic_assertion" and line == site["marker"]]
                if len(matched) != 1:
                    raise Incomplete("Session failure contains an unrelated native, driver or fixture error.")
                section_causes.append({"test": test, **matched[0]})
            if not section_causes:
                raise Incomplete("Session expected control supplies no primary semantic cause.")
            causes.extend(section_causes)
        checked[name] = {"log_sha256": hashlib.sha256(raw).hexdigest(), "causes": causes,
                         "expected_controls_without_credit": secondary}
    return {"checked_caught_logs": checked, "compiler_or_harness_kill_credit": 0,
            "semantic_site_map_sha256": COMMON.canonical_sha256(sites),
            "independent_descriptor_review_required": True}


def session_witness_policy(policy: dict) -> dict:
    """Require the fixed six-case supplement bound to the new ordinary receipt."""
    expected = {**SESSION_WITNESS_LIMITS,
                "candidate_sha256": policy["candidate_sha256"],
                "ordinary_receipt_sha256": policy["ordinary_evidence"]["sha256"],
                "cases": {name: {"descriptor_sha256": item[0], "test": item[1]}
                          for name, item in SESSION_WITNESS_CASES.items()}}
    if (type(SESSION_WITNESS_ADMISSION) is not dict or type(policy.get("mutation_witnesses")) is not dict
            or COMMON.canonical_sha256(policy["mutation_witnesses"]) != COMMON.canonical_sha256(expected)
            or COMMON.canonical_sha256(SESSION_WITNESS_ADMISSION) != COMMON.canonical_sha256(expected)):
        raise Incomplete("Session supplemental witnesses lack their new closed ordinary binding.")
    return expected


def session_execution_blocks(text: str) -> list:
    """Read complete native blocks without guessing a selected or filtered roster."""
    pattern = (r"^running (\d{1,6}) tests?\n(.*?)^test result: (ok|FAILED)\. "
               r"(\d{1,6}) passed; (\d{1,6}) failed; (\d{1,6}) ignored; "
               r"(\d{1,6}) measured; (\d{1,6}) filtered out; finished in [0-9]+(?:\.[0-9]+)?s$")
    blocks = list(re.finditer(pattern, text, re.MULTILINE | re.DOTALL))
    if (not blocks or len(blocks) > 16
            or len(re.findall(r"(?m)^running ", text)) != len(blocks)
            or len(re.findall(r"(?m)^test result:", text)) != len(blocks)):
        raise Incomplete("Session witness has missing, truncated or extra native blocks.")
    return blocks


def session_baseline_blocks(text: str, policy: dict) -> list:
    """Derive per-binary rosters only after full current baseline admission."""
    validate_session_test_execution(text, policy, doctests=True)
    return [re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. ok$", block[2])
            for block in session_execution_blocks(text)]


def session_singleton_execution(text: str, selected: str, rosters: list, *, failed: bool) -> dict:
    """Admit the fixed filtered invocation's ordinary blocks; it runs no doctests."""
    allowed = {item[1] for item in SESSION_WITNESS_CASES.values()}
    expected = (SESSION_ADMISSION.get("test_inventory", {}) if type(SESSION_ADMISSION) is dict else {})
    frozen = expected.get("ordinary", []) + expected.get("doctests", [])
    if (type(selected) is not str or selected not in allowed or type(failed) is not bool
            or type(rosters) is not list or not 0 < len(rosters) <= 16
            or any(type(names) is not list or any(type(name) is not str for name in names) for names in rosters)
            or sorted(name for names in rosters for name in names) != sorted(frozen)
            or len({name for names in rosters for name in names}) != len(frozen)
            or rosters[0].count(selected) != 1
            or any(selected in names for names in rosters[1:])):
        raise Incomplete("Session witness selection differs from its complete current baseline roster.")
    ordinary, docs = set(expected.get("ordinary", [])), set(expected.get("doctests", []))
    ordinary_rosters = []
    for names in rosters:
        members = set(names)
        if members and members <= ordinary:
            ordinary_rosters.append(names)
        elif not members or not members <= docs:
            raise Incomplete("Session witness baseline mixes ordinary and doctest groups.")
    if not ordinary_rosters or ordinary_rosters[0] != rosters[0]:
        raise Incomplete("Session witness needs its selected ordinary library group first.")
    session_reject_harness_failure(text)
    if re.search(r"(?m)^Error: ", text):
        raise Incomplete("Session singleton witness contains a raw native, entropy or fixture error.")
    if any(re.fullmatch(r"test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)", line) is None
           for line in text.splitlines() if line.startswith("test ") and not line.startswith("test result:")):
        raise Incomplete("Session singleton witness contains a malformed native test row.")
    blocks = session_execution_blocks(text)
    if len(blocks) != (1 if failed else len(ordinary_rosters)):
        raise Incomplete("Session witness did not enter exactly the required native prefix.")
    observed = []
    for index, block in enumerate(blocks):
        running, body, status, passed, failures, ignored, measured, filtered = block.groups()
        actual = tuple(map(int, (running, passed, failures, ignored, measured, filtered)))
        active = index == 0
        expected_counts = (1, 0 if failed else 1, 1 if failed else 0, 0, 0, len(ordinary_rosters[0]) - 1) if active else (
            0, 0, 0, 0, 0, len(ordinary_rosters[index]))
        rows = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", body)
        wanted = [(selected, "FAILED" if failed else "ok")] if active else []
        if (actual != expected_counts or rows != wanted
                or status != ("FAILED" if failed else "ok")
                or re.search(r"(?m)^running ", body)):
            raise Incomplete("Session witness changed a selected result or expected filtered count.")
        observed.extend(rows)
    if re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (ok|FAILED|ignored)$", text) != observed:
        raise Incomplete("Session witness contains a result outside its completed native blocks.")
    sections = list(re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)",
                               text, re.MULTILINE | re.DOTALL))
    if failed:
        listing = blocks[0][2].rsplit("\nfailures:\n", 1)
        if (len(sections) != 1 or sections[0][1] != selected or len(listing) != 2
                or listing[1].strip("\n").splitlines() != ["    " + selected]
                or not blocks[0].start() <= sections[0].start() < blocks[0].end()):
            raise Incomplete("Session witness lacks its one complete named failure section.")
    elif sections or "panicked at " in text or re.search(r"(?m)^failures:", text):
        raise Incomplete("Session healthy witness contains an error, panic or failure section.")
    return {"selected_test": selected, "selected_executed": 1,
            "entered_blocks": len(blocks), "doctest_blocks_executed": 0,
            "expected_filtered_counts": [len(names) - (index == 0)
                                         for index, names in enumerate(ordinary_rosters[:len(blocks)])],
            "sections": sections}


def session_witness_phases(row: dict, selected: str, *, failed: bool) -> None:
    """Check actual pinned cargo-mutants phase argv and healthy/101 native statuses."""
    phases = row.get("phase_results")
    if (type(phases) is not list or any(type(phase) is not dict for phase in phases)
            or [phase.get("phase") for phase in phases] != ["Build", "Test"]):
        raise Incomplete("Session witness lacks its exact build and test phases.")
    for phase in phases:
        command = ["cargo", "test", *(["--no-run"] if phase["phase"] == "Build" else []),
                   "--verbose", "--package=nevrotick-engine@0.0.0", "--message-format=json", "--frozen"]
        if phase["phase"] == "Test":
            command.extend([selected, "--", "--exact", "--test-threads=1"])
        status = {"Failure": 101} if failed and phase["phase"] == "Test" else "Success"
        duration = phase.get("duration")
        if (phase.get("argv") != command or phase.get("process_status") != status
                or (type(status) is dict and (type(phase["process_status"]) is not dict
                    or type(phase["process_status"].get("Failure")) is not int))
                or type(duration) not in (int, float) or not math.isfinite(duration) or duration < 0):
            raise Incomplete("Session witness changed its command, duration or native phase outcome.")


def session_owned_mutation_bytes(directory: Path, relative: str) -> bytes:
    """Read only one bounded regular output path within this exact owned campaign."""
    name = COMMON.analysis_output_path(relative)
    if name.is_absolute() or not name.parts or ".." in name.parts:
        raise Incomplete("Session mutation output escapes its owned campaign.")
    path = directory / name
    if path.is_symlink() or not path.is_file() or path.resolve() != path:
        raise Incomplete("Session mutation output is missing or redirected.")
    return COMMON.read_analysis_bytes(path, maximum=16 * 1024 * 1024)


def session_witness_harness_sections(sections: list, name: str) -> None:
    """Accept only the two reviewed uncredited child-failure diagnostic shapes."""
    expected = ("series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions"
                if "refresh_slot" in name else
                "series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor"
                if "handle_series_read" in name else None)
    seen = []
    for section in sections:
        if "N3_HARNESS" not in section[2]:
            continue
        if expected is None or section[1] != expected:
            raise Incomplete("Rejected Session harness diagnostic is outside its reviewed failed section.")
        lines = section[2].strip("\n").splitlines()
        if "refresh_slot" in name:
            capture = (re.fullmatch(r"N3_HARNESS_CAUSE_CAPTURE case=" + re.escape(expected)
                                   + r" parent=([1-9][0-9]{0,9}) limit_bytes=16384", lines[0]) if lines else None)
            if capture is None or int(capture[1]) > 4_294_967_295 or lines[1:] != [
                    "N3_HARNESS_INCOMPLETE",
                    "N3_HARNESS_CAUSE N3 named child FAILED/101 lacked assertion or semantic evidence; bounded cause captures retained",
                    'Error: Custom { kind: Other, error: "N3 named child FAILED/101 lacked assertion or semantic evidence; bounded cause captures retained" }']:
                raise Incomplete("Rejected Session capture diagnostic changed its exact cause or count.")
        elif lines != [
                "N3_HARNESS_INCOMPLETE", "N3_HARNESS_CAUSE owned process cleanup incomplete",
                "N3_HARNESS_INCOMPLETE",
                "N3_HARNESS_CAUSE N3 owned child global reap deadline exceeded; N3 owned cleanup also failed: N3 owned child global reap deadline exceeded",
                'Error: Custom { kind: Other, error: "N3 owned child global reap deadline exceeded; N3 owned cleanup also failed: N3 owned child global reap deadline exceeded" }']:
            raise Incomplete("Rejected Session cleanup diagnostic changed its exact cause or count.")
        seen.append(section[1])
    if seen != ([] if expected is None else [expected]):
        raise Incomplete("Rejected Session main log lacks its one exact reviewed harness failure section.")


def session_witness_counter_sections(sections: list, sites: dict) -> None:
    """Recognize only the second observed 843 shape; neither section earns credit."""
    native = "series::tests::selected_native_datagram_after_retirement_is_charged_without_publication"
    counter = SESSION_WITNESS_COUNTER_TEST
    if [section[1] for section in sections] != [native, counter]:
        raise Incomplete("Rejected Session counter shape changed its two exact failed sections.")
    pid = r"(?: \([1-9][0-9]{0,9}\))?"
    note = r"(?:\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace)?"
    first = (r"thread '" + re.escape(native) + r"'" + pid + r" panicked at "
             + re.escape(SESSION_SUBJECTS["series"] + ":843:13:")
             + r"\nattempt to subtract with overflow" + note)
    relay = "N3_SEMANTIC_EVIDENCE test assertion/panic at crates/nevrotick-engine/tests/private/series.rs:561:5"
    second = (re.escape(relay) + r"\n\nthread '" + re.escape(counter) + r"'" + pid + r" panicked at "
              + re.escape("crates/nevrotick-engine/src/../tests/private/series_process.rs:430:9:")
              + r"\nN3 exact child semantic failure assertion" + note)
    if (re.fullmatch(first, sections[0][2].strip("\n")) is None
            or re.fullmatch(second, sections[1][2].strip("\n")) is None):
        raise Incomplete("Rejected Session counter shape changed its exact panic, relay or diagnostic payload.")
    admitted = sites.get("tests", {}).get(counter, [])
    primary = [site for site in admitted if site.get("kind") == "panic_assertion"
               and (site.get("source"), site.get("line"), site.get("column"), site.get("marker")) == (
                   "crates/nevrotick-engine/tests/private/series.rs", 561, 5,
                   "synchronous series counter operations must not panic") and site.get("exact_message") is True]
    bridge = [site for site in admitted if site.get("kind") == "child_bridge"
              and (site.get("source"), site.get("line"), site.get("column"), site.get("marker")) == (
                  SERIES_PROCESS_SOURCE, 430, 9, "N3 exact child semantic failure assertion")]
    if len(primary) != 1 or len(bridge) != 1 or session_child_causes(sections[1][2], admitted) != primary:
        raise Incomplete("Rejected Session counter shape lacks its exact source-bound completed-child cause.")


def session_witness_main_eligibility(text: str, descriptor: dict, reason: str,
                                     rosters: list, sites: dict) -> None:
    """Check a closed rejected-main case; malformed/foreign records never qualify."""
    entry = SESSION_WITNESS_CASES.get(descriptor.get("name"))
    if entry is None or COMMON.canonical_sha256(descriptor) != entry[0]:
        raise Incomplete("Rejected Session fault has no exact reviewed supplemental witness.")
    name = descriptor["name"]
    counter_shape = ("handle_series_read" in name
                     and reason == "Session panic is not its named source-owned semantic assertion.")
    expected_reason = ("Session failure contains an unrelated native, driver or fixture error."
                       if "token_issue" in name else
                       "Session panic is not its named source-owned semantic assertion."
                       if "publish_series_reply" in name else
                       "Session native log contains compiler, abort or harness failure.")
    if reason != expected_reason and not counter_shape:
        raise Incomplete("Rejected Session fault differs from its reviewed main rejection category.")
    # The closed main failures stay uncredited. Other compiler/abort/signal or
    # foreign harness failures are never converted into witness eligibility.
    session_reject_harness_failure(re.sub(r"(?m)^N3_HARNESS_INCOMPLETE\n", "", text))
    sections = session_complete_failure_records(text)
    if (text.count("panicked at ") != sum(section[2].count("panicked at ") for section in sections)
            or len(re.findall(r"(?m)^Error:", text)) != sum(len(re.findall(r"(?m)^Error:", section[2])) for section in sections)
            or text.count("N3_HARNESS") != sum(section[2].count("N3_HARNESS") for section in sections)
            or text.count("N3_SEMANTIC_EVIDENCE") != sum(section[2].count("N3_SEMANTIC_EVIDENCE") for section in sections)):
        raise Incomplete("Rejected Session main log contains a panic, error or harness diagnostic outside its failed sections.")
    if counter_shape:
        session_witness_counter_sections(sections, sites)
    else:
        session_witness_harness_sections(sections, name)
    blocks = session_execution_blocks(text)
    if len(blocks) > len(rosters):
        raise Incomplete("Rejected Session main log entered a foreign native block.")
    for index, block in enumerate(blocks):
        names = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. (?:ok|FAILED|ignored)$", block[2])
        if sorted(names) != sorted(rosters[index]):
            raise Incomplete("Rejected Session main log lost its complete native prefix roster.")
    if any(section[1] not in sites["tests"] for section in sections):
        raise Incomplete("Rejected Session main log failed a foreign test.")
    for section in sections:
        locations = list(re.finditer(r"(?m)^thread '([^'\r\n]+)'(?: \(\d+\))? panicked at "
                                    r"(?:/work/source/)?([^\r\n]+):(\d+):(\d+):\n([^\r\n]+)", section[2]))
        if section[2].count("panicked at ") != len(locations):
            raise Incomplete("Rejected Session main log contains a malformed panic.")
        for location in locations:
            thread, filename, line, column, message = location.groups()
            filename = session_source_alias(filename)
            owned = any((site.get("source"), site.get("line"), site.get("column")) ==
                        (filename, int(line), int(column))
                        and (message == site["marker"] if site.get("exact_message") or site["kind"] != "panic_assertion"
                             else message.startswith(site["marker"]))
                        for site in sites["tests"][section[1]]
                        if site["kind"] in {"panic_assertion", "child_bridge", "expected_control_panic"})
            counter = ("-= in " in name and filename == SESSION_SUBJECTS["series"]
                       and int(line) == descriptor["span"]["start"]["line"] and int(column) == 13
                       and message == "attempt to subtract with overflow")
            if thread != section[1] or not (owned or counter):
                raise Incomplete("Rejected Session main log contains an unreviewed panic.")
    allowed_errors = {"Error: DuplicateToken", "Error: AcquisitionError { .. }"} if "token_issue" in name else (
        {'Error: Custom { kind: Other, error: "N3 named child FAILED/101 lacked assertion or semantic evidence; bounded cause captures retained" }'}
        if "refresh_slot" in name else
        {'Error: Custom { kind: Other, error: "N3 owned child global reap deadline exceeded; N3 owned cleanup also failed: N3 owned child global reap deadline exceeded" }'}
        if "handle_series_read" in name else set())
    if any(line not in allowed_errors for line in text.splitlines() if line.startswith("Error:")):
        raise Incomplete("Rejected Session main log contains an unreviewed raw error.")


def session_execute_witness(session, policy: dict, descriptor: dict,
                            rosters: list, sites: dict, deadline: float, index: int) -> dict:
    """Run one sealed adopted-tool baseline/exact-mutant pair; no retry is possible."""
    profile = session_witness_policy(policy)
    name = descriptor["name"]
    if (name not in SESSION_WITNESS_CASES or COMMON.canonical_sha256(descriptor) != SESSION_WITNESS_CASES[name][0]
            or type(index) is not int or not 0 <= index < profile["maximum_cases"]):
        raise Incomplete("Session witness needs one exact listed descriptor and bounded case position.")
    selected = profile["cases"][name]["test"]
    directory = session.work / "witnesses" / f"series-{index}"
    if directory.exists():
        raise Incomplete("Session witness output already exists; retries and replacement are forbidden.")
    directory.parent.mkdir(mode=0o700, exist_ok=True)
    # cargo-mutants 27.1.0 src/main.rs and src/cargo.rs own --re and test args.
    pattern = "^" + re.sub(r"([\\.^$|?*+(){}\[\]])", r"\\\1", name) + "$"
    command = [str(session.tools / "cargo-mutants"), "mutants", "--package", "nevrotick-engine",
               "--file", SESSION_SUBJECTS["series"], "--test-workspace", "false", "--cap-lints", "false",
               "--jobs", "1", "--jobserver", "true", "--jobserver-tasks", "1",
               "--cargo-arg=--message-format=json", "--re", pattern, "--baseline", "run",
               "--build-timeout", "90", "--timeout", "60", "--output", f"/work/witnesses/series-{index}",
               "--", selected, "--", "--exact", "--test-threads=1"]
    label = f"mutation-witness:series-{index}"
    session.run(label, command, COMMON.remaining_timeout(deadline, profile["case_wall_seconds"]))
    lane = session.report["lanes"][-1]
    actual_command = lane.get("command")
    if (lane.get("lane") != label or type(actual_command) is not list
            or actual_command[-len(command):] != command
            or lane.get("failure") is not None or type(lane.get("exit_code")) is not int or lane["exit_code"] != 0):
        raise Incomplete("Session witness namespace did not terminate and reap successfully.")
    directory /= "mutants.out"
    session.protect(directory)
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    if type(outcomes) is not dict or type(outcomes.get("outcomes")) is not list or any(
            type(row) is not dict for row in outcomes["outcomes"]):
        raise Incomplete("Session witness outcomes are malformed.")
    rows = outcomes.get("outcomes", [])
    baseline = [row for row in rows if row.get("scenario") == "Baseline"]
    changed = [row for row in rows if type(row.get("scenario")) is dict]
    if (type(outcomes.get("end_time")) is not str or not outcomes["end_time"]
            or outcomes.get("cargo_mutants_version") != "27.1.0" or len(rows) != 2
            or len(baseline) != 1 or len(changed) != 1 or baseline[0].get("summary") != "Success"
            or changed[0].get("summary") != "CaughtMutant"
            or any(type(outcomes.get(key)) is not int or outcomes[key] != value for key, value in
                   {"total_mutants": 1, "caught": 1, "missed": 0, "unviable": 0, "timeout": 0, "success": 0}.items())
            or changed[0]["scenario"].get("Mutant") != {key: value for key, value in descriptor.items() if key != "diff"}):
        raise Incomplete("Session witness does not contain exactly its healthy baseline and listed caught fault.")
    session_witness_phases(baseline[0], selected, failed=False)
    session_witness_phases(changed[0], selected, failed=True)
    healthy = session_owned_mutation_bytes(directory, baseline[0].get("log_path", ""))
    failed = session_owned_mutation_bytes(directory, changed[0].get("log_path", ""))
    difference = session_owned_mutation_bytes(directory, changed[0].get("diff_path", ""))
    if difference != descriptor["diff"].encode():
        raise Incomplete("Session witness raw diff differs from the full generated descriptor.")
    healthy_result = session_singleton_execution(healthy.decode(), selected, rosters, failed=False)
    admitted = validate_session_caught_causes(directory, {"outcomes": changed}, sites,
                                             subject="series", witness=(selected, rosters))
    session.check()
    return {"descriptor_sha256": COMMON.canonical_sha256(descriptor), "selected_test": selected,
            "candidate_sha256": policy["candidate_sha256"],
            "scope": "One listed source-bound witness; no full-campaign qualification.",
            "qualification_passed": False, "mutation_gate_claim": False,
            "healthy_log_sha256": hashlib.sha256(healthy).hexdigest(),
            "mutated_log_sha256": hashlib.sha256(failed).hexdigest(),
            "diff_sha256": hashlib.sha256(difference).hexdigest(),
            "healthy_execution": {key: value for key, value in healthy_result.items() if key != "sections"},
            "source_cause_admission": admitted, "execution_attempts": {"baselines": 1, "mutants": 1},
            "healthy_test_exit_code": 0, "mutated_test_exit_code": 101,
            "source_helper_identity_sha256": COMMON.canonical_sha256(session.identity),
            "tools_sha256": dict(session.report["tools_sha256"]),
            "cargo_config_sha256": session.config_sha256,
            "outer_pid_namespace_terminal": True, "original_child_cleanup_claim": "none",
            "native_windows_qualification": "not performed"}


def session_run_witness(session, policy: dict, descriptor: dict, main_row: dict,
                        reason: str, rosters: list, sites: dict, deadline: float, index: int) -> dict:
    """Attach only the actual rejected-main context already admitted by the caller."""
    if (main_row.get("summary") != "CaughtMutant"
            or main_row.get("scenario") != {"Mutant": {key: value for key, value in descriptor.items() if key != "diff"}}
            or type(main_row.get("log_path")) is not str or not main_row["log_path"]
            or type(reason) is not str or not reason):
        raise Incomplete("Session witness adapter needs the actual complete rejected-main identity.")
    evidence = session_execute_witness(session, policy, descriptor, rosters, sites, deadline, index)
    return {**evidence, "main_rejection_preserved": reason, "main_log_path": main_row["log_path"]}


def validate_session_campaign(session, policy: dict, subject: str, native: dict,
                                report: dict, deadline: float) -> dict:
    """Require the complete main campaign before six closed independent supplements."""
    if subject not in SESSION_SUBJECTS:
        raise Incomplete("Session supplementary admission needs one exact cohort owner.")
    directory = session.work / "mutation" / subject / "mutants.out"
    session.check()
    if (type(native) is not dict or native.get("failure") is not None
            or type(native.get("exit_code")) is not int or native["exit_code"] not in (0, 2)):
        raise Incomplete("Session main namespace must terminate before supplemental admission.")
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    listed, _warning = COMMON.parse_mutation_listing(report["mutation_enumeration"][subject]["raw_output"],
                                                    expected_jobs=policy["mutation_jobs"])
    inventory = COMMON.mutation_inventory(SESSION_SCOPE, listed, policy["maximum_mutants"], SESSION_SUBJECTS[subject])
    # Integrity, native phase outcomes and ALL descriptor/diff bytes precede any
    # exception classification. A semantic supplement cannot repair corruption.
    COMMON.mutation_counts(SESSION_SCOPE, outcomes, inventory, SESSION_SUBJECTS[subject],
                           native_exit_code=native["exit_code"], defer_gate=True)
    for row in outcomes["outcomes"]:
        if type(row.get("scenario")) is dict:
            descriptor = inventory[row["scenario"]["Mutant"]["name"]]
            if session_owned_mutation_bytes(directory, row.get("diff_path", "")) != descriptor["diff"].encode():
                raise Incomplete("Session main raw diff differs from its full frozen descriptor.")
    baseline = next(row for row in outcomes["outcomes"] if row["scenario"] == "Baseline")
    baseline_raw = session_owned_mutation_bytes(directory, baseline.get("log_path", ""))
    baseline_result = validate_session_test_execution(baseline_raw.decode(), policy, doctests=True)
    rosters = session_baseline_blocks(baseline_raw.decode(), policy)
    sites = session_semantic_assertion_sites(session.source, policy)
    checked, rejected, supplements, attempts = {}, {}, {}, {}
    admission = {"checked_caught_logs": checked, "rejected_full_suite_logs": rejected,
                 "supplemental_witnesses": supplements, "compiler_or_harness_kill_credit": 0,
                 "supplemental_attempts": attempts, "full_campaign_generated": len(inventory),
                 "semantic_site_map_sha256": COMMON.canonical_sha256(sites),
                 "independent_descriptor_review_required": True}
    result = {**baseline_result, "log_sha256": hashlib.sha256(baseline_raw).hexdigest(),
              "caught_cause_admission": admission}
    report.setdefault("baseline_execution", {})[subject] = result
    witness_deadline = min(deadline, time.monotonic() + SESSION_WITNESS_LIMITS["total_wall_seconds"])
    for row in outcomes["outcomes"]:
        if row.get("summary") != "CaughtMutant":
            continue
        name = row["scenario"]["Mutant"]["name"]
        try:
            main = validate_session_caught_causes(directory, {"outcomes": [row]}, sites, subject=subject)
        except Incomplete as error:
            raw = session_owned_mutation_bytes(directory, row.get("log_path", ""))
            reason = str(error)
            rejected[name] = {"log_sha256": hashlib.sha256(raw).hexdigest(), "reason": reason,
                              "credit": 0, "original_failure_preserved": True}
            if subject != "series":
                raise
            descriptor = inventory[name]
            session_witness_main_eligibility(raw.decode(), descriptor, reason, rosters, sites)
            if len(supplements) >= SESSION_WITNESS_LIMITS["maximum_cases"]:
                raise Incomplete("Session supplemental witness case budget exceeded.")
            index = len(supplements)
            attempts[name] = {"status": "requested", "case_index": index,
                              "native_commands_observed": 0, "execution_phases": "not observed"}
            before = len(report.get("lanes", []))
            try:
                supplements[name] = session_run_witness(session, policy, descriptor, row, reason,
                                                        rosters, sites, witness_deadline, index)
            finally:
                completed = name in supplements
                attempts[name].update(status="complete" if completed else "incomplete",
                    native_commands_observed=sum(lane.get("lane") == f"mutation-witness:series-{index}"
                                                 for lane in report.get("lanes", [])[before:]),
                    execution_phases={"healthy_baselines": 1, "mutants": 1} if completed else "not qualified")
        else:
            checked.update(main["checked_caught_logs"])
    if (set(checked) & set(supplements) or len(checked) + len(supplements) != outcomes["caught"]):
        raise Incomplete("Session caught decisions are missing or duplicated across witnesses.")
    admission["unique_verified_caught"] = len(checked) + len(supplements)
    admission["supplemental_execution_attempts"] = 2 * len(supplements)
    return result


def validate_session_baseline(work: Path, policy: dict, subject: str = "session") -> dict:
    """Parse and hash one complete actual passing baseline before cause admission."""
    if subject not in SESSION_SUBJECTS:
        raise Incomplete("Session baseline requires one exact selected cohort owner.")
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    rows = outcomes.get("outcomes")
    if type(rows) is not list or any(type(row) is not dict for row in rows):
        raise Incomplete("Session baseline outcome list is malformed.")
    baselines = [row for row in rows if row.get("scenario") == "Baseline"]
    if len(baselines) != 1 or baselines[0].get("summary") != "Success":
        raise Incomplete("Session mutation needs exactly one successful actual baseline.")
    COMMON.mutation_phases(SESSION_SCOPE, baselines[0])
    relative = COMMON.analysis_output_path(baselines[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Session baseline log escapes its owned output.")
    raw = COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)
    return {**validate_session_test_execution(raw.decode("utf-8"), policy, doctests=True),
            "log_sha256": hashlib.sha256(raw).hexdigest(),
            "caught_cause_admission": validate_session_caught_causes(
                directory, outcomes, session_semantic_assertion_sites(work / "source", policy), subject=subject)}


def session_contract_examples() -> dict:
    """Prepare exact external ownership causes and one legal no-native neighbor."""
    setup = ("let peer = SocketAddr::from(([127,0,0,1],41001)); "
             "let config = SeriesConfig::new(peer, Duration::from_millis(100), Duration::from_secs(1))?; "
             "let _ = &config;")
    marker = "legal-session-contracts:checked-owner-no-native"
    examples = {
        "session_legal": {"statement": "let mut owner = AcquisitionSession::new(config); owner.retire(); owner.retire(); "
            "assert!(matches!(owner.prepare(), Err(AcquisitionError::Retired))); "
            "fn legal_view<'a>(report: &'a AcquisitionReport) -> Result<(&'a AcquisitionReport, &'a SeriesAttempt, &'a TransportFacts, &'a ExperimentalMeasurement, ExperimentalReferenceNs), AcquisitionReferenceRefusal> { "
            "let _ = report.coherence(); let view = report.reference_at(0)?; Ok((view.report(), view.attempt(), view.facts(), view.measurement(), view.midpoint())) } "
            "let _consumer = legal_view; "
            'println!("' + marker + '");', "code": None, "tokens": [], "success_marker": marker},
        "session_private_construction": {"statement": "fn forge(base: AcquisitionSession, config: SeriesConfig) -> AcquisitionSession { AcquisitionSession { config, ..base } }",
            "code": "E0451", "tokens": ["AcquisitionSession", "private"]},
        "session_config_mutation": {"statement": "fn mutate(mut owner: AcquisitionSession, config: SeriesConfig) { owner.config = config; }",
            "code": "E0616", "tokens": ["config", "private"]},
        "session_plan_retarget": {"statement": "fn retarget(mut plan: PreparedAcquisition<'_>, owner: &mut AcquisitionSession) { plan.session = owner; }",
            "code": "E0616", "tokens": ["session", "private"]},
        "session_report_substitution": {"statement": "fn replace(mut value: AcquisitionReport, series: SeriesReport) { value.series = series; }",
            "code": "E0616", "tokens": ["series", "private"]},
        "session_coherence_retarget": {"statement": "fn replace(mut value: AcquisitionReport, coherence: SeriesCoherence) { value.coherence = coherence; }",
            "code": "E0616", "tokens": ["coherence", "private"]},
        "session_plan_twice": {"statement": "fn twice(plan: PreparedAcquisition<'_>, schedule: &ControlSchedule) { let _ = plan.execute(schedule); let _ = plan.execute(schedule); }",
            "code": "E0382", "tokens": ["plan", "moved"]},
        "session_overlapping_prepare": {"statement": "fn overlap(owner: &mut AcquisitionSession) -> Result<(), AcquisitionError> { let first = owner.prepare()?; let second = owner.prepare()?; drop((first, second)); Ok(()) }",
            "code": "E0499", "tokens": ["owner", "mutable"]},
        "session_active_retire": {"statement": "fn overlap(owner: &mut AcquisitionSession) -> Result<(), AcquisitionError> { let first = owner.prepare()?; owner.retire(); drop(first); Ok(()) }",
            "code": "E0499", "tokens": ["owner", "mutable"]},
        "session_plan_owner_escape": {"statement": "fn escape(mut owner: AcquisitionSession) -> Result<PreparedAcquisition<'static>, AcquisitionError> { let (plan, control) = owner.prepare()?; drop(control); Ok(plan) }",
            "code": "E0515", "tokens": ["owner", "return"]},
        "session_reference_forge": {"statement": "fn forge<'a>(report: &'a AcquisitionReport, attempt: &'a SeriesAttempt, facts: &'a TransportFacts, measurement: &'a ExperimentalMeasurement, midpoint: ExperimentalReferenceNs) -> AcquisitionReference<'a> { AcquisitionReference { report, attempt, facts, measurement, midpoint } }",
            "code": "E0451", "tokens": ["AcquisitionReference", "private"]},
        "session_reference_owner_escape": {"statement": "fn escape<'a>(report: AcquisitionReport) -> Result<AcquisitionReference<'a>, AcquisitionReferenceRefusal> { report.reference_at(0) }",
            "code": "E0515", "tokens": ["report", "return"]},
        "session_reference_report_drop": {"statement": "fn drop_owner(report: AcquisitionReport) -> Result<(), AcquisitionReferenceRefusal> { let view = report.reference_at(0)?; drop(report); let _ = view.midpoint(); Ok(()) }",
            "code": "E0505", "tokens": ["report", "borrow"]},
        "session_reference_default": {"statement": "let _: AcquisitionReference<'static> = Default::default();",
            "code": "E0277", "tokens": ["AcquisitionReference", "Default"]},
        "session_reference_unchecked_from": {"statement": "fn unchecked<'a>(report: &'a AcquisitionReport, attempt: &'a SeriesAttempt, facts: &'a TransportFacts, measurement: &'a ExperimentalMeasurement, midpoint: ExperimentalReferenceNs) -> AcquisitionReference<'a> { (report, attempt, facts, measurement, midpoint).into() }",
            "code": "E0277", "tokens": ["AcquisitionReference", "From"]},
        "session_reference_private_module": {"statement": "use nevrotick_engine::relative;",
            "code": "E0603", "tokens": ["relative", "private"]},
        "session_reference_missing_export": {"statement": "use nevrotick_engine::reference_from_instants;",
            "code": "E0432", "tokens": ["reference_from_instants"]},
    }
    for field, typename, borrowed in (
            ("report", "AcquisitionReport", True), ("attempt", "SeriesAttempt", True),
            ("facts", "TransportFacts", True), ("measurement", "ExperimentalMeasurement", True),
            ("midpoint", "ExperimentalReferenceNs", False)):
        parameter = "&'a " + typename if borrowed else typename
        examples["session_reference_" + field + "_retarget"] = {
            "statement": "fn retarget<'a>(mut view: AcquisitionReference<'a>, other: " + parameter
                         + ") { view." + field + " = other; }",
            "code": "E0616", "tokens": [field, "private"]}
    for specimen in examples.values():
        specimen["source"] = "\n".join([
            "use nevrotick_engine::*; use std::{net::SocketAddr, time::Duration};",
            "fn main() -> Result<(), Box<dyn std::error::Error>> {", setup,
            specimen.pop("statement"), "Ok(())", "}"]) + "\n"
        specimen["line"] = 4
    return examples


def session_windows_variants(source: str, template: str) -> dict:
    """Copy the complete actual helper; remove only the named guard for one RED control."""
    marker = "// NEVROTICK_COHERENCE_RESIDUAL_SOURCE"
    masked, functions = quality_rust_functions(source)
    helper = functions.get("coherence_residual")
    declarations = list(re.finditer(r"(?m)^fn coherence_residual\s*\(", masked))
    if helper is None or len(declarations) != 1 or template.count(marker) != 1:
        raise Incomplete("Windows surrogate needs one complete actual helper and one source marker.")
    beginning = declarations[0].start()
    function = source[beginning:helper["end"] + 1]
    masked_function = masked[beginning:helper["end"] + 1]
    pattern = r"\|\|\s*later\s*[.]\s*mono\s*<\s*base\s*[.]\s*mono_after\b"
    guards = list(re.finditer(pattern, masked_function))
    if (len(guards) != 1
            or re.fullmatch(pattern, function[guards[0].start():guards[0].end()]) is None
            or re.match(r"\s*\{", masked_function[guards[0].end():]) is None):
        raise Incomplete("Windows surrogate cross-bracket guard changed its reviewed shape.")
    guard = guards[0]
    missing = function[:guard.start()] + function[guard.end():]
    return {"healthy": template.replace(marker, function),
            "missing_cross_guard": template.replace(marker, missing)}


def session_windows_control(session, policy: dict, report: dict, deadline: float) -> None:
    """Run one source-linked host surrogate pair, without product mutation credit."""
    raw = COMMON.read_analysis_bytes(session.source / SESSION_WINDOWS_CONTROL, maximum=128 * 1024)
    source = COMMON.read_analysis_bytes(session.source / SESSION_SUBJECTS["measurement"], maximum=512 * 1024)
    if (hashlib.sha256(raw).hexdigest() != policy["inputs_sha256"].get(SESSION_WINDOWS_CONTROL)
            or hashlib.sha256(source).hexdigest() != policy["inputs_sha256"].get(SESSION_SUBJECTS["measurement"])):
        raise Incomplete("Windows surrogate template or actual measurement source changed.")
    variants = session_windows_variants(source.decode("utf-8"), raw.decode("utf-8"))
    base = session.work / "coherence-control"
    base.mkdir(mode=0o700)
    inputs, binaries = base / "inputs", base / "bin"
    inputs.mkdir(mode=0o700); binaries.mkdir(mode=0o700)
    for name, text in variants.items():
        (inputs / (name + ".rs")).write_text(text)
    session.protect(inputs)
    for name in variants:
        session.run("session-windows-surrogate-compile:" + name,
            [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", "coherence_" + name,
             "/work/coherence-control/inputs/" + name + ".rs", "-o", "/work/coherence-control/bin/" + name],
            COMMON.remaining_timeout(deadline, 60), sealed=[inputs])
    session.protect(binaries)
    healthy = session.run("session-windows-surrogate-healthy", ["/work/coherence-control/bin/healthy"],
        COMMON.remaining_timeout(deadline, 30), sealed=[inputs, binaries])
    if healthy.splitlines() != ["measurement-coherence-windows-surrogate:healthy"]:
        raise Incomplete("Windows surrogate did not complete its exact healthy controls.")
    failed = session.run("session-windows-surrogate-removed-guard", ["/work/coherence-control/bin/missing_cross_guard"],
        COMMON.remaining_timeout(deadline, 30), sealed=[inputs, binaries], negative="WINDOWS_EPSILON_CROSS_GUARD_REQUIRED")
    native = report["lanes"][-1]
    session_reject_harness_failure(failed)
    text = variants["missing_cross_guard"]
    assertion = text.rfind("assert_eq!(", 0, text.index('"WINDOWS_EPSILON_CROSS_GUARD_REQUIRED"'))
    line, column = text.count("\n", 0, assertion) + 1, assertion - text.rfind("\n", 0, assertion)
    pattern = (r"(?m)^thread 'main'(?: \([0-9]+\))? panicked at "
               + re.escape(f"/work/coherence-control/inputs/missing_cross_guard.rs:{line}:{column}:\n")
               + r"assertion `left == right` failed: WINDOWS_EPSILON_CROSS_GUARD_REQUIRED\n"
                 r"\s*left: Ok\(0\)\n\s*right: Err\(CaptureOrder\)")
    if (native.get("failure") is not None or type(native.get("exit_code")) is not int
            or native["exit_code"] != 101 or failed.count("panicked at ") != 1
            or re.search(r"(?mi)^(?:Error:|error:)", failed)
            or re.search(pattern, failed) is None):
        raise Incomplete("Windows surrogate RED lacks its exact completed native 101 assertion.")
    report["windows_coherence_surrogate"] = {
        "template_sha256": hashlib.sha256(raw).hexdigest(), "measurement_sha256": hashlib.sha256(source).hexdigest(),
        "variants_sha256": {name: hashlib.sha256(value.encode()).hexdigest() for name, value in variants.items()},
        "binaries_sha256": PRODUCT.inventory(binaries), "healthy_exit_code": 0, "removed_guard_exit_code": 101,
        "scope": "Native host substituted-clock source control only; no native Windows or product mutation kill credit."}


def execute_session_profile(args, report: dict) -> None:
    """Use existing custody for exactly the current session/series/measurement cohort."""
    if args.tool_admission_profile != "full" or args.lane not in {"contracts", "enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Session requires its bounded full-profile lanes.")
    subjects = SESSION_SCOPE.select(args.subject)
    policy = read_session_policy(ROOT)
    if args.lane in {"mutation", "all"}:
        bindings = session_inventory_bindings(policy, required=True)
        if sum(bindings[name]["count"] for name in subjects) > policy["maximum_mutants"]:
            raise Incomplete("Selected complete session cohort exceeds 512 variants.")
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Session analysis requires five GiB free before admission.")
    selected = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        selected.lane = "mutation"
    report.update(subjects=subjects, candidate_sha256=policy["candidate_sha256"], qualification_passed=False,
                  profile="session", scope="Bounded host persistent acquisition session " + args.lane,
                  historical_family_bindings_reused=False, ordinary_evidence=admit_session_ordinary(policy),
                  generation_gap_controls="Separate lifecycle/function inventory and direct controls required.")
    session = COMMON.Session(ROOT, selected, report, SESSION_SCOPE, policy, SESSION_BOUND_INPUTS)
    try:
        if (COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]
                or len(session.product_identity) != policy["source_file_count"]
                or any(PRODUCT.sha256(session.source / name) != digest for name, digest in policy["inputs_sha256"].items())
                or read_json(session.source / SESSION_POLICY) != policy):
            raise Incomplete("Captured session candidate/policy/helper bytes differ.")
        if args.lane in {"contracts", "all"}:
            deadline = time.monotonic() + policy["contracts_wall_seconds"]
            contracts(session, policy, report, examples=session_contract_examples(),
                effect_scope="Checked session construction and terminal refusal execute without entropy/socket/runtime; view getter legal consumer and exact privacy/substitution/lifetime/default/From/module probes compile only.",
                deadline=deadline)
            session_windows_control(session, policy, report, deadline)
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(SESSION_SCOPE, subjects, policy, session.tools,
                session.run, report, deadline=time.monotonic() + policy["enumeration_wall_seconds"],
                maximum_per_file=512, maximum_total=512)
            report.update(listed_inventories=listing.inventories, listed_inventory_bindings=listed_summary(listing.inventories),
                          listed_total=listing.total, campaign_cap_exceeded=listing.campaign_budget_exceeded, campaign_admission=False)
        if args.lane in {"coverage", "all"}:
            # Keep the native per-file threshold on the reviewed three-owner
            # cohort; the inline quality test module is fixture source as well.
            ignored = "(^|/)tests/|/rustlib/src/rust/library/|/crates/(ntp-wire|time-domain)/|/nevrotick-engine/src/(lib[.]rs|quality[.]rs|quality/|relative[.]rs|bin/)"
            raw = session.run("coverage", [str(session.tools / "cargo-llvm-cov"), "llvm-cov", "--package", "nevrotick-engine",
                "--frozen", "--jobs", "6", "--lib", "--tests", "--json", "--output-path", "/work/coverage.json",
                "--failure-mode", "any", "--ignore-filename-regex", ignored, "--fail-under-file-lines", "90", "--", "--test-threads=1"],
                policy["coverage_wall_seconds"])
            report["coverage_execution"] = validate_session_test_execution(raw, policy, doctests=False)
            coverage = COMMON.read_analysis_json(session.work / "coverage.json")
            report["coverage"] = {name: COMMON.coverage_counts(SESSION_SCOPE, coverage, "/work/source/" + filename)
                                  for name, filename in subjects.items()}
        if args.lane in {"mutation", "all"}:
            def run(label, command, timeout, **options):
                command_deadline = time.monotonic() + timeout
                result = session.run(label, command, timeout, **options)
                if label.startswith("mutation-campaign:"):
                    subject = label.removeprefix("mutation-campaign:")
                    validate_session_campaign(session, policy, subject, result, report, command_deadline)
                return result
            COMMON.run_analysis_lanes(SESSION_SCOPE, subjects, policy, session.work, session.tools, run, report, "mutation")
    finally:
        session.finish()
        admit_session_ordinary(policy)
    if args.lane in {"contracts", "all"}:
        report["contracts_qualification_passed"] = True
    if args.lane != "enumerate":
        report["qualification_passed"] = True


# Exact two-owner laboratory continuation. No historical Engine policy is rebound.
ADDRESS_POLICY = "policy/address-round-analysis.json"
ADDRESS_TESTS = "scripts/tests/test_address_round_analysis.py"
ADDRESS_SUBJECTS = {
    "address_round": "crates/nevrotick-engine/src/address_round.rs",
    "address_progress": "crates/nevrotick-engine/src/address_round/progress.rs",
}
ADDRESS_SCOPE = COMMON.Scope("nevrotick-engine", tuple(ADDRESS_SUBJECTS.items()), ADDRESS_POLICY)
ADDRESS_TEST_SOURCES = {
    "series::address_round::tests::": "crates/nevrotick-engine/tests/private/address_round.rs",
    "series::address_round::progress::tests::": "crates/nevrotick-engine/tests/private/address_progress.rs",
}
ADDRESS_HELPERS = {
    "crates/nevrotick-engine/tests/private/address_round.rs": frozenset({
        "checked_config", "blackhole_then_reply", "check_query", "require_query",
        "observe_queries", "product_ok", "classify_writable_result", "owned_timer_case",
        "setup_handler_control_case", "owned_counter_boundary_case",
        "future_invalidation_boundary_case", "check_receive_boundary_history",
        "receive_budget_case", "check_readiness_io_history",
    }),
    "crates/nevrotick-engine/tests/private/address_progress.rs": frozenset({
        "config", "product_ok", "product_some",
    }),
}
ADDRESS_BOUND_INPUTS = tuple(dict.fromkeys((
    WRAPPER, ADDRESS_TESTS, *ADDRESS_SUBJECTS.values(), *ADDRESS_TEST_SOURCES.values(),
    "crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/series.rs",
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml", "rust-toolchain.toml",
    ".cargo/config.toml", ".cargo/mutants.toml", ".config/nextest.toml",
    "scripts/portable-analysis.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/analysis-controls.py",
    "scripts/verify.py", "scripts/check-environment.py", "scripts/check-strict.py",
    "scripts/prepare-product.py", "scripts/slint_dependency_profile.py", "scripts/slint_linux_sdk.py",
    "scripts/advisory_snapshot.py", "tools/crypto-aes-siv-witness/fuzz/input_custody.py",
    "policy/tool-manifest.json", "policy/analysis-tools.json", "policy/verification-manifest.toml",
)))
ADDRESS_FIXED_POLICY = {
    "schema_version": 1, "package": "nevrotick-engine",
    "critical_files": list(ADDRESS_SUBJECTS.values()), "tool_admission_profile": "full",
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 512,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240, "mutation_wall_seconds": 1200,
    "mutation_build_seconds": 90, "mutation_test_seconds": 60,
    "jobs": 1, "test_threads": 1, "coverage_test_threads": 1,
    "mutation_jobs": 1, "mutation_process_limit": 128, "mutation_message_format": "json",
    "minimum_free_bytes": 5 * 1024**3, "stop_free_bytes": 4 * 1024**3,
    "owned_output_maximum_allocated_bytes": 768 * 1024**2, "owned_output_maximum_entries": 32768,
    "mutation_equivalences": [],
}


def address_test_inventory(binding: dict) -> tuple[list[str], list[str]]:
    """Require the final ordinary Engine's complete 178/69 identity sets."""
    inventory = binding.get("test_inventory")
    if type(inventory) is not dict or set(inventory) != {"ordinary", "doctests", "sha256"}:
        raise Incomplete("Address-round ordinary identities are unbound.")
    for field, count in (("ordinary", 178), ("doctests", 69)):
        names = inventory[field]
        if (type(names) is not list or len(names) != count
                or any(type(name) is not str or not 0 < len(name) <= 4096 for name in names)
                or names != sorted(set(names))):
            raise Incomplete("Address-round requires its complete unique ordinary/doc inventory.")
    ordinary, docs = inventory["ordinary"], inventory["doctests"]
    hashes = {"ordinary": COMMON.canonical_sha256(ordinary), "doctests": COMMON.canonical_sha256(docs),
              "baseline": COMMON.canonical_sha256(sorted(ordinary + docs))}
    if inventory["sha256"] != hashes or len(set(ordinary + docs)) != 247:
        raise Incomplete("Address-round native identity digests differ.")
    if any(not any(name.startswith(prefix) for name in ordinary) for prefix in ADDRESS_TEST_SOURCES):
        raise Incomplete("Both actual address-round test owners are required.")
    selected = [name for name in docs if name.startswith(ADDRESS_SUBJECTS["address_round"] + " - series::address_round::")]
    legal = [name for name in selected if "::AddressRoundConfig::new " in name and not name.endswith((" - compile fail", " - compile"))]
    privacy = [name for name in selected if "::AddressRoundConfig " in name and name.endswith(" - compile fail")]
    consume = [name for name in selected if "::PreparedAddressRound " in name and name.endswith(" - compile fail")]
    if len(selected) != 5 or len(legal) != 1 or len(privacy) != 3 or len(consume) != 1:
        raise Incomplete("Address-round legal/privacy/consume-once docs are missing.")
    return ordinary, docs


def validate_address_policy(policy: dict, *, campaign: bool = False) -> dict:
    """Unbound ordinary evidence and empty campaign inventories always refuse."""
    if (type(policy) is not dict or set(policy) != set(ADDRESS_FIXED_POLICY) | {"binding", "mutation_inventories"}
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in ADDRESS_FIXED_POLICY.items())):
        raise Incomplete("Address-round scope, limits or thresholds differ.")
    binding = policy["binding"]
    if type(binding) is not dict or set(binding) != {"candidate_sha256", "source_file_count", "inputs_sha256", "ordinary_evidence", "test_inventory"}:
        raise Incomplete("Address-round final ordinary binding has not been admitted.")
    if (type(binding["source_file_count"]) is not int or binding["source_file_count"] <= 0
            or type(binding["candidate_sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", binding["candidate_sha256"]) is None):
        raise Incomplete("Address-round candidate identity is invalid.")
    pins = binding["inputs_sha256"]
    if (type(pins) is not dict or set(pins) != set(ADDRESS_BOUND_INPUTS)
            or any(type(value) is not str or re.fullmatch(r"[a-f0-9]{64}", value) is None for value in pins.values())):
        raise Incomplete("Address-round protected input closure is incomplete.")
    address_test_inventory(binding)
    inventories = policy["mutation_inventories"]
    if type(inventories) is not dict or (not inventories and campaign):
        raise Incomplete("Address-round campaign needs reviewed nonempty inventories.")
    if inventories:
        if set(inventories) != set(ADDRESS_SUBJECTS):
            raise Incomplete("Both exact address-round inventories are required.")
        for value in inventories.values():
            if (type(value) is not dict or set(value) != {"count", "sha256"}
                    or type(value["count"]) is not int or not 0 < value["count"] <= 512
                    or type(value["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", value["sha256"]) is None):
                raise Incomplete("Address-round inventory count/hash is invalid.")
        if sum(value["count"] for value in inventories.values()) > 512:
            raise Incomplete("Address-round complete inventory exceeds its finite campaign cap.")
    return binding


def address_execution(text: str, binding: dict, *, doctests: bool) -> dict:
    """Reconcile actual complete native successes against the frozen identities."""
    ordinary, docs = address_test_inventory(binding)
    records = read_successful_address_records(text)
    expected = sorted(ordinary + docs) if doctests else ordinary
    if sorted(records) != expected:
        raise Incomplete("Address-round execution lacks its complete actual baseline.")
    return {"tests_executed": len(records), "test_names": records,
            "inventory_sha256": COMMON.canonical_sha256(sorted(records))}


def read_successful_address_records(text: str) -> list[str]:
    """Require complete entered libtest blocks, exact declarations and terminal rows.

    Reuse the session parser's complete-block/count reconciliation shape for the
    success-only lane. Zero-test sub-binaries remain observable, never evidence
    for a missing nonempty cohort. An unfinished trailing block cannot disappear.
    """
    if type(text) is not str:
        raise Incomplete("Address-round native test capture must be decoded text.")
    if any(re.fullmatch(r"test ([^\r\n]+?) \.\.\. ok", line) is None
           for line in text.splitlines() if line.startswith("test ") and not line.startswith("test result:")):
        raise Incomplete("Address-round native test is failed, ignored or incomplete.")
    if any(line in {"ok", "FAILED", "ignored"} for line in text.splitlines()):
        raise Incomplete("Address-round native completion lacks its identity.")
    blocks = list(re.finditer(r"^running (\d{1,6}) tests?\n(.*?)^test result: ok\. "
                             r"(\d{1,6}) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;[^\r\n]*$",
                             text, re.MULTILINE | re.DOTALL))
    all_records = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. ok$", text)
    if (not blocks or not all_records
            or len(re.findall(r"(?m)^running ", text)) != len(blocks)
            or len(re.findall(r"(?m)^test result:", text)) != len(blocks)):
        raise Incomplete("Address-round native block declaration or summary is unmatched.")
    records = []
    for block in blocks:
        running, body, passed = block.groups()
        names = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. ok$", body)
        if (int(running) != int(passed) or len(names) != int(running)
                or re.search(r"(?m)^running |^test result:", body)):
            raise Incomplete("Address-round entered count differs from its complete native records.")
        records.extend(names)
    if records != all_records or len(records) != len(set(records)):
        raise Incomplete("Address-round native records are duplicated or outside an entered block.")
    return records


def address_storage_usage(directory: Path) -> dict:
    """Charge unique allocated blocks across the entire owned output tree.

    This deliberately includes source/tools/logs as well as the shared target,
    every cargo-mutants worker target and work/tmp (also the private /tmp).
    It is a bounded before/after sample, not a continuous filesystem quota.
    """
    if directory.is_symlink() or not directory.is_dir() or directory.resolve(strict=True) != directory:
        raise Incomplete("Address-round storage root is not its canonical owned directory.")
    pending, seen, allocated, entries = [directory], set(), 0, 0
    while pending:
        path = pending.pop()
        observed = path.lstat()
        if not stat.S_ISREG(observed.st_mode) and not stat.S_ISDIR(observed.st_mode):
            raise Incomplete("Address-round owned output contains a linked or special entry.")
        entries += 1
        if entries > ADDRESS_FIXED_POLICY["owned_output_maximum_entries"]:
            raise Incomplete("Address-round owned output exceeds its finite entry bound.")
        identity = observed.st_dev, observed.st_ino
        if identity not in seen:
            seen.add(identity)
            allocated += observed.st_blocks * 512
        if stat.S_ISDIR(observed.st_mode):
            for child in path.iterdir():
                pending.append(child)
                if len(pending) + entries > ADDRESS_FIXED_POLICY["owned_output_maximum_entries"]:
                    raise Incomplete("Address-round pending storage inventory exceeds its finite bound.")
    return {"allocated_bytes": allocated, "entries": entries}


def address_storage_checkpoint(session, policy: dict, report: dict, label: str, phase: str) -> None:
    """Sample the whole owned tree against this role's cap and free-space reserve."""
    usage = address_storage_usage(session.output)
    row = {"lane": label, "phase": phase, **usage, "available_bytes": shutil.disk_usage(session.output).free}
    report.setdefault("storage_samples", []).append(row)
    (session.output / "progress.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    if row["allocated_bytes"] >= policy["owned_output_maximum_allocated_bytes"]:
        raise Incomplete("Aggregate owned output reached its allocated cap "
                         f"({policy['owned_output_maximum_allocated_bytes']} bytes).")
    if row["available_bytes"] < policy["stop_free_bytes"]:
        raise Incomplete("Address-round four-GiB available reserve reached.")


def _admit_engine_ke_retirement_execution(policy: dict, execution: object) -> None:
    """Require exact ordinary execution evidence before retiring either cache."""
    binding = policy.get("binding")
    if type(binding) is not dict:
        raise Incomplete("Engine KE target retirement has no admitted binding.")
    candidate = binding.get("candidate_sha256")
    names = validate_engine_ke_inventory(binding.get("test_inventory"), candidate=candidate)
    validate_engine_ke_doctest_inventory(binding.get("doctest_inventory"), candidate=candidate)
    ordinary_count, ke_count = engine_ke_case_counts(candidate)
    records = execution.get("test_names") if type(execution) is dict else None
    if (type(records) is not list or len(records) != ordinary_count
            or any(type(name) is not str for name in records)
            or sorted(records) != names
            or type(execution.get("tests_executed")) is not int
            or execution["tests_executed"] != ordinary_count
            or type(execution.get("ke_tests_executed")) is not int
            or execution["ke_tests_executed"] != ke_count
            or execution.get("inventory_sha256") != COMMON.canonical_sha256(names)):
        raise Incomplete("Engine KE target retirement requires the exact ordinary execution roster and digest.")


def _admit_engine_ke_debug_retirement(policy: dict, report: dict) -> None:
    """Require complete debug execution and Clippy custody before removing their cache."""
    _admit_engine_ke_retirement_execution(policy, report.get("baseline_execution"))
    lanes, samples = report.get("lanes"), report.get("storage_samples")
    if (type(lanes) is not list or len(lanes) < 2
            or any(type(row) is not dict for row in lanes)
            or [row.get("lane") for row in lanes[-2:]] != ["ke-baseline", "ke-clippy"]
            or any(sum(row.get("lane") == label for row in lanes) != 1
                   for label in ("ke-baseline", "ke-clippy"))
            or any(type(row.get("exit_code")) is not int or row["exit_code"] != 0
                   or "failure" not in row or row["failure"] is not None for row in lanes[-2:])
            or type(samples) is not list or not samples
            or any(type(row) is not dict for row in samples)):
        raise Incomplete("Debug retirement requires unique ordered successful baseline and Clippy commands.")
    completed = [row for row in samples if row.get("phase") == "after"
                 and row.get("lane") in ("ke-baseline", "ke-clippy")]
    if ([row.get("lane") for row in completed] != ["ke-baseline", "ke-clippy"]
            or samples[-1] != completed[-1]
            or any(type(row.get("allocated_bytes")) is not int or row["allocated_bytes"] < 0
                   or row["allocated_bytes"] >= policy["owned_output_maximum_allocated_bytes"]
                   or type(row.get("available_bytes")) is not int
                   or row["available_bytes"] < policy["stop_free_bytes"] for row in completed)):
        raise Incomplete("Debug retirement requires passed baseline and final Clippy post-lane storage samples.")


def retire_engine_ke_debug_target(session, policy: dict, report: dict) -> None:
    """Retire completed GNU debug artifacts before release; retain cross-target metadata."""
    _retire_engine_ke_target(session, policy, report, "debug")


def retire_engine_ke_release_target(session, policy: dict, report: dict) -> None:
    """Retire only completed validated release artifacts before debug-only lanes."""
    _retire_engine_ke_target(session, policy, report, "release")


def _retire_engine_ke_target(session, policy: dict, report: dict, profile: str) -> None:
    """Admit and remove only one of two fixed completed caches, never an arbitrary path."""
    roles = {
        "debug": ("ke-clippy", "debug_target_retirement", "ke-debug-target-retirement", "Debug"),
        "release": ("ke-release", "release_target_retirement", "ke-release-target-retirement", "Release"),
    }
    if type(profile) is not str or profile not in roles:
        raise Incomplete("Engine KE target retirement accepts only its fixed debug or release profile.")
    completed_lane, record_key, sample_label, title = roles[profile]
    if profile == "debug":
        _admit_engine_ke_debug_retirement(policy, report)
    else:
        lanes, samples = report.get("lanes", []), report.get("storage_samples", [])
        if (not lanes or lanes[-1].get("lane") != "ke-release"
                or lanes[-1].get("exit_code") != 0 or lanes[-1].get("failure") is not None
                or sum(row.get("lane") == "ke-release" for row in lanes) != 1
                or not samples
                or samples[-1].get("lane") != "ke-release" or samples[-1].get("phase") != "after"
                or samples[-1]["allocated_bytes"] >= policy["owned_output_maximum_allocated_bytes"]
                or samples[-1]["available_bytes"] < policy["stop_free_bytes"]):
            raise Incomplete("Release retirement requires completed validation and a passed post-lane storage sample.")
        _admit_engine_ke_retirement_execution(policy, report.get("release_execution"))
    output, work = session.output, session.work
    target, cache = work / "target", work / "target" / profile
    if work != output / "work" or not getattr(shutil.rmtree, "avoids_symlink_attacks", False):
        raise Incomplete(f"{title} retirement requires its exact owned work tree and safe directory removal.")
    try:
        for path in (output, work, target, cache):
            observed = path.lstat()
            if (not stat.S_ISDIR(observed.st_mode) or observed.st_uid != os.geteuid()
                    or path.resolve(strict=True) != path or observed.st_mode & 0o022
                    or (path in (output, work) and observed.st_mode & 0o077)):
                raise Incomplete(f"{title} retirement parents must be canonical, private and owned directories.")
        address_storage_checkpoint(session, policy, report, sample_label, "before")
        row = {"path": "work/target/" + profile, "completed_lane": completed_lane,
               "before": address_storage_usage(cache), "removed": False}
        report[record_key] = row
        shutil.rmtree(cache)
        row["removed"] = True
        address_storage_checkpoint(session, policy, report, sample_label, "after")
    except OSError as error:
        raise Incomplete(f"Completed {profile} target retirement failed.") from error


def admit_address_ordinary(binding: dict) -> dict:
    """Require an actual final receipt and unchanged executed recipes, never placeholders."""
    evidence = binding["ordinary_evidence"]
    if (type(evidence) is not dict or set(evidence) != {"receipt", "sha256", "status", "recipes_sha256"}
            or evidence["status"] != "passed-address-round-all" or type(evidence["receipt"]) is not str
            or not Path(evidence["receipt"]).is_absolute() or type(evidence["sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", evidence["sha256"]) is None
            or type(evidence["recipes_sha256"]) is not dict or not evidence["recipes_sha256"]):
        raise Incomplete("Address-round ordinary receipt is unbound.")
    raw = COMMON.read_analysis_bytes(Path(evidence["receipt"]), maximum=2 * 1024**2)
    if hashlib.sha256(raw).hexdigest() != evidence["sha256"]:
        raise Incomplete("Address-round ordinary receipt changed.")
    receipt = json.loads(raw, object_pairs_hook=COMMON.ANALYSIS.unique_object,
                         parse_float=finite_number, parse_constant=invalid_number)
    if (type(receipt) is not dict or receipt.get("status") != evidence["status"]
            or receipt.get("bookends_unchanged") is not True
            or receipt.get("candidate_sha256") != binding["candidate_sha256"]
            or type(receipt.get("source_sha256")) is not dict
            or len(receipt["source_sha256"]) != binding["source_file_count"]
            or COMMON.canonical_sha256(receipt["source_sha256"]) != binding["candidate_sha256"]
            or receipt.get("recipes_sha256") != evidence["recipes_sha256"]):
        raise Incomplete("Address-round ordinary candidate/recipe custody differs.")
    for name, digest in evidence["recipes_sha256"].items():
        path = Path(name)
        if (not path.is_absolute() or path.resolve(strict=True) != path or path.is_symlink()
                or type(digest) is not str or PRODUCT.sha256(path) != digest):
            raise Incomplete("Address-round executed recipe bytes changed.")
    rows = receipt.get("lanes")
    if (type(rows) is not list or any(type(row) is not dict for row in rows)
            or [row.get("lane") for row in rows] != list(SESSION_ORDINARY_LANES)
            or any(row.get("failure") is not None or type(row.get("exit_code")) is not int or row["exit_code"] != 0 for row in rows)):
        raise Incomplete("Address-round ordinary receipt lacks all required successful lanes.")
    ordinary, docs = address_test_inventory(binding)
    for label, expected in (("engine-tests", ordinary), ("engine-optimized-tests", ordinary), ("engine-doctests", docs)):
        row = next(row for row in rows if row["lane"] == label)
        if (type(row.get("tests")) is not dict or row["tests"].get("executed") != len(expected)
                or sorted(row["tests"].get("test_names", [])) != expected):
            raise Incomplete("Address-round ordinary actual test records differ.")
    return evidence


def address_assertion_sites(root: Path, binding: dict) -> dict:
    """Traverse only closed semantic helpers in the two source-bound test owners."""
    ordinary, _docs = address_test_inventory(binding)
    sites = {}
    for prefix, filename in ADDRESS_TEST_SOURCES.items():
        raw = COMMON.read_analysis_bytes(root / filename, maximum=512 * 1024)
        if hashlib.sha256(raw).hexdigest() != binding["inputs_sha256"][filename]:
            raise Incomplete("Address-round semantic source bytes changed.")
        source = raw.decode("utf-8")
        masked, functions = quality_rust_functions(source)
        selected = [name for name in ordinary if name.startswith(prefix)]
        declared = re.findall(r"(?m)^#\[test\]\s*\n(?:#\[[^\n]*\]\s*\n)*fn ([A-Za-z_][A-Za-z0-9_]*)\s*\(", source)
        if sorted(prefix + name for name in declared) != selected:
            raise Incomplete("Address-round declared and actually executed test owners differ.")
        for test in selected:
            pending, visited, found, facts = [test.removeprefix(prefix)], set(), [], set()
            while pending:
                owner = pending.pop()
                if owner in visited:
                    continue
                if owner not in functions:
                    raise Incomplete("Address-round selected semantic helper is absent.")
                visited.add(owner)
                start, end = functions[owner]["start"], functions[owner]["end"]
                code = masked[start:end]
                for helper in ADDRESS_HELPERS[filename]:
                    for call in re.finditer(r"\b" + helper + r"\s*\(", code):
                        pending.append(helper)
                        if helper in {"product_ok", "product_some"}:
                            arguments = session_call_arguments(masked, source, start + call.end() - 1, end)
                            if len(arguments) == 2 and re.fullmatch(r'\s*"[^"\\]*"\s*', source[slice(*arguments[1])]):
                                facts.add(session_static_fact(source, arguments))
                for match in re.finditer(r"\b(assert|assert_eq|assert_ne)!\s*\(", code):
                    offset = start + match.start()
                    row = {"source": filename, "line": source.count("\n", 0, offset) + 1,
                           "column": offset - source.rfind("\n", 0, offset), "owner": owner}
                    marker = {"assert": "assertion failed:", "assert_eq": "assertion `left == right` failed",
                              "assert_ne": "assertion `left != right` failed"}[match[1]]
                    arguments = session_call_arguments(masked, source, start + match.end() - 1, end)
                    if match[1] == "assert" and len(arguments) > 1:
                        literal = source[slice(*arguments[1])].strip()
                        if literal == '"{fact}"' and owner in {"product_ok", "product_some"}:
                            found.append({**row, "fact_helper": True})
                            continue
                        if len(arguments) != 2 or re.fullmatch(r'"[^"\\{}]*"', literal) is None:
                            raise Incomplete("Address-round assertion message shape needs explicit source review.")
                        marker = json.loads(literal)
                        found.append({**row, "marker": marker, "exact": True})
                    else:
                        found.append({**row, "marker": marker, "exact": False})
            expanded = []
            for row in found:
                if row.get("fact_helper"):
                    expanded.extend({key: value for key, value in {**row, "marker": fact, "exact": True}.items() if key != "fact_helper"} for fact in sorted(facts))
                else:
                    expanded.append(row)
            if not expanded:
                raise Incomplete("Address-round test has no admitted semantic assertion.")
            sites[test] = expanded
    return sites


def address_caught_causes(directory: Path, outcomes: dict, sites: dict) -> dict:
    """Require semantic causes; retain one exact independently reviewed R47 deferral."""
    checked = {}
    aliases = {"crates/nevrotick-engine/src/../tests/private/address_round.rs": ADDRESS_TEST_SOURCES["series::address_round::tests::"],
               "crates/nevrotick-engine/src/address_round/../../tests/private/address_progress.rs": ADDRESS_TEST_SOURCES["series::address_round::progress::tests::"]}
    r47_name = "crates/nevrotick-engine/src/address_round.rs:394:8: delete ! in guarded_writable"
    r47_fixture = "series::address_round::tests::real_receive_budget_boundary_distinguishes_valid_sixty_fourth_from_all_rejected"
    r47_body = 'Error: Custom { kind: TimedOut, error: "receive-boundary fixture exceeded its actual original cutoff" }'
    r47_writable = "series::address_round::tests::final_writable_guard_rechecks_controls_after_actual_socket_readiness"
    r47_marker = "quiet writable neighbor never sent its actual query"
    for row in outcomes["outcomes"]:
        if row.get("summary") != "CaughtMutant":
            continue
        raw = session_owned_mutation_bytes(directory, row.get("log_path", ""))
        text = raw.decode("utf-8")
        name = row["scenario"]["Mutant"]["name"]
        # Run all raw compiler/resource/abort and complete-record checks first.
        sections = session_complete_failures(text)
        inconclusive = []
        r47_section = None
        marker_text = text
        for section in sections:
            if name == r47_name and section[1] == r47_fixture and section[2].strip() == r47_body:
                r47_section = section
                # Preserve every boundary/newline; mask only the exact admitted body.
                marker_text = text[:section.start(2)] + re.sub(r"[^\r\n]", " ", section[2]) + text[section.end(2):]
                inconclusive.append({"test": r47_fixture,
                                     "section_sha256": hashlib.sha256(section[0].encode("utf-8")).hexdigest(),
                                     "reason": "R47 receive cutoff is inconclusive; separate writable assertion supplies the cause",
                                     "kill_credit": 0})
        if re.search(r"(?m)^Error: |ADDRESS_ROUND_(?:PRIMARY|CLEANUP|ATTEMPT_IO|PEER|PEER_DROP|UNEXPECTED_ATTEMPT|WRITABLE_IO|SOCKET_RETAINED|RECEIVE_INCOMPLETE)\b", marker_text):
            raise Incomplete("Address-round caught log retains native/fixture error; no kill credit.")
        if text.count("panicked at ") != sum(section[2].count("panicked at ") for section in sections):
            raise Incomplete("Address-round log has an unclassified panic.")
        causes = []
        for section in sections:
            test, body = section.groups()
            if test not in sites:
                raise Incomplete("Address-round caught test is outside its semantic owners.")
            if section is r47_section:
                continue
            locations = list(re.finditer(r"(?m)^thread '([^'\r\n]+)'(?: \(\d+\))? panicked at (?:/work/source/)?([^\r\n]+):(\d+):(\d+):\n([^\r\n]+)", body))
            if not locations or body.count("panicked at ") != len(locations):
                raise Incomplete("Address-round caught assertion is missing or malformed.")
            for location in locations:
                thread, filename, line, column, message = location.groups()
                filename = aliases.get(filename, filename)
                matches = [site for site in sites[test] if (site["source"], site["line"], site["column"]) == (filename, int(line), int(column))
                           and (message == site["marker"] if site["exact"] else message.startswith(site["marker"]))]
                if thread != test or len(matches) != 1:
                    raise Incomplete("Address-round panic is not its exact source-owned semantic assertion.")
                causes.append({"test": test, **matches[0]})
        if inconclusive and not any(cause["test"] == r47_writable and cause["marker"] == r47_marker
                                    and cause["exact"] is True
                                    and cause["source"] == ADDRESS_TEST_SOURCES["series::address_round::tests::"]
                                    and cause["owner"] == r47_writable.removeprefix("series::address_round::tests::")
                                    for cause in causes):
            raise Incomplete("R47 inconclusive receive section lacks its exact separate writable assertion.")
        if name in checked:
            raise Incomplete("Address-round caught identity is duplicated.")
        checked[name] = {"log_sha256": hashlib.sha256(raw).hexdigest(), "causes": causes,
                         "inconclusive_sections": inconclusive}
    return {"checked_caught_logs": checked, "compiler_or_harness_kill_credit": 0}


def execute_address_profile(args, report: dict) -> None:
    """Select only the two new owners through unchanged shared custody and scoring."""
    if args.tool_admission_profile != "full" or args.lane not in {"enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Address-round admits only its full-profile metrics lanes.")
    policy = read_json(ROOT / ADDRESS_POLICY)
    binding = validate_address_policy(policy, campaign=args.lane in {"mutation", "all"})
    subjects = ADDRESS_SCOPE.select(args.subject)
    _, identity = PRODUCT.admit_product(ROOT)
    if (len(identity) != binding["source_file_count"] or COMMON.canonical_sha256(identity) != binding["candidate_sha256"]
            or any(PRODUCT.sha256(ROOT / name) != digest for name, digest in binding["inputs_sha256"].items())):
        raise Incomplete("Address-round final candidate/helper binding differs.")
    manifest = tomllib.loads((ROOT / "policy/verification-manifest.toml").read_text())
    for filename in ADDRESS_SUBJECTS.values():
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(row.get("package") == "nevrotick-engine" and filename in row.get("files", []) for row in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Both address-round owners need exact critical/security registration.")
    admit_address_ordinary(binding)
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Address-round metrics require five GiB before admission.")
    selected = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        selected.lane = "mutation"
    report.update(profile="address_round", subjects=subjects, candidate_sha256=binding["candidate_sha256"],
                  qualification_passed=False, ordinary_evidence=binding["ordinary_evidence"],
                  scope="Selected two-owner laboratory address-round metrics; no native or production qualification",
                  ordinary_rebinding="Manifest registration changes the product candidate: require a new ordinary receipt; never rewrite v5 or its metadata hashes.",
                  storage_scope="768 MiB aggregate over the entire owned output including all Cargo/worker targets; before/after samples, not a continuous quota.",
                  generation_gap_controls="Review every function/Garde guard against the actual inventory; direct alternatives receive no generated kill credit.")
    session = COMMON.Session(ROOT, selected, report, ADDRESS_SCOPE, policy, ADDRESS_BOUND_INPUTS)
    try:
        if (COMMON.canonical_sha256(session.product_identity) != binding["candidate_sha256"]
                or len(session.product_identity) != binding["source_file_count"]
                or read_json(session.source / ADDRESS_POLICY) != policy
                or any(PRODUCT.sha256(session.source / name) != digest
                       for name, digest in binding["inputs_sha256"].items())):
            raise Incomplete("Captured address-round candidate/policy/helper bytes differ.")
        def run(label, command, timeout, **options):
            address_storage_checkpoint(session, policy, report, label, "before")
            try:
                result = session.run(label, command, timeout, **options)
            finally:
                address_storage_checkpoint(session, policy, report, label, "after")
            if label.startswith("mutation-campaign:"):
                subject = label.removeprefix("mutation-campaign:")
                directory = session.work / "mutation" / subject / "mutants.out"
                outcomes = COMMON.read_analysis_json(directory / "outcomes.json")
                listing, _warning = COMMON.parse_mutation_listing(report["mutation_enumeration"][subject]["raw_output"], expected_jobs=1)
                inventory = COMMON.mutation_inventory(ADDRESS_SCOPE, listing, 512, subjects[subject])
                COMMON.mutation_counts(ADDRESS_SCOPE, outcomes, inventory, subjects[subject], native_exit_code=result["exit_code"], defer_gate=True)
                for row in outcomes["outcomes"]:
                    if type(row.get("scenario")) is dict:
                        descriptor = inventory[row["scenario"]["Mutant"]["name"]]
                        if session_owned_mutation_bytes(directory, row.get("diff_path", "")) != descriptor["diff"].encode():
                            raise Incomplete("Address-round mutation diff differs from its frozen descriptor.")
                baseline = next(row for row in outcomes["outcomes"] if row["scenario"] == "Baseline")
                baseline_raw = session_owned_mutation_bytes(directory, baseline.get("log_path", ""))
                checked = address_execution(baseline_raw.decode(), binding, doctests=True)
                checked["log_sha256"] = hashlib.sha256(baseline_raw).hexdigest()
                checked["caught_cause_admission"] = address_caught_causes(directory, outcomes, address_assertion_sites(session.source, binding))
                report.setdefault("baseline_execution", {})[subject] = checked
            return result
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(ADDRESS_SCOPE, subjects, policy, session.tools, run, report,
                deadline=time.monotonic() + policy["enumeration_wall_seconds"], maximum_per_file=512, maximum_total=512)
            report.update(listed_inventories=listing.inventories, listed_inventory_bindings=listed_summary(listing.inventories),
                          listed_total=listing.total, campaign_admission=False)
        if args.lane in {"coverage", "all"}:
            ignored = "(^|/)tests/|/rustlib/src/rust/library/|/crates/(ntp-wire|time-domain)/|/nevrotick-engine/src/(lib[.]rs|measurement[.]rs|relative[.]rs|series[.]rs|session[.]rs|quality[.]rs|quality/|bin/)"
            raw = run("coverage", [str(session.tools / "cargo-llvm-cov"), "llvm-cov", "--package", "nevrotick-engine", "--frozen", "--jobs", "1",
                "--lib", "--tests", "--json", "--output-path", "/work/coverage.json", "--failure-mode", "any",
                "--ignore-filename-regex", ignored, "--fail-under-file-lines", "90", "--", "--test-threads=1"], policy["coverage_wall_seconds"])
            report["coverage_execution"] = address_execution(raw, binding, doctests=False)
            coverage = COMMON.read_analysis_json(session.work / "coverage.json")
            report["coverage"] = {name: COMMON.coverage_counts(ADDRESS_SCOPE, coverage, "/work/source/" + filename) for name, filename in subjects.items()}
        if args.lane in {"mutation", "all"}:
            COMMON.run_analysis_lanes(ADDRESS_SCOPE, subjects, policy, session.work, session.tools, run, report, "mutation")
    finally:
        session.finish()
        admit_address_ordinary(binding)
    if args.lane != "enumerate":
        report["selected_lane_passed"] = True
        report["selected_qualification_passed"] = args.lane == "all"
        report["qualification_passed"] = args.lane == "all" and subjects == ADDRESS_SCOPE.select("all")



# One EP1 role; Windows-only and standalone-callsite owners remain in the full inventory.
ENGINE_KE_POLICY = COMMON.ENGINE_KE_POLICY
ENGINE_KE_HOST_SUBJECTS = dict(COMMON.ENGINE_KE_SUBJECTS)
ENGINE_KE_SCOPE = COMMON.Scope("nevrotick-engine", COMMON.ENGINE_KE_SUBJECTS, ENGINE_KE_POLICY)
ENGINE_KE_ORDINARY_COUNT = 576
ENGINE_KE_CASE_COUNT = 292
ENGINE_KE_DOCTEST_COUNT = 90
ENGINE_REVIEW_CANDIDATE = "eb0468583ee1c6a08e045296beec8d7fdf181c7a5154fbb36868cc4665422195"
ENGINE_REVIEW_INPUT_COUNT = 619
ENGINE_SCALE_CONFLICT_CANDIDATE = "9787ca8da2b8c768edc89e116568c71a9243bb7cfd4d1a85cf00001651989a89"
ENGINE_SCALE_CONFLICT_INPUT_COUNT = 620
ENGINE_SCALE_CONFLICT_TERMINAL = "b9866c64480d4c852bf9a3ff78b162d97826050fedeb673b130639075939314d"
ENGINE_SCALE_CONFLICT_MANIFEST = "688fc05e0740bc31d68b226c7e71394fe5aaed3ea6482438d250ec73168a98e0"
ENGINE_SCALE_CONFLICT_BOUNDARY = "a9383bfa0820f2f7e2453c1a4c4d3912468061329ff1d7c860405a833b1fb7b8"
ENGINE_SCALE_CONFLICT_EP1_CASE_DIGEST = "b97fb5d24414fffb4dd5b49b76bc8e6a5abd12c6fe1dd50c8aadfae7d8527b1b"
ENGINE_SCALE_CONFLICT_DEFAULT_CASE_DIGEST = "6fdab3b3e6a8bb20cd2e9801d425880cf9db791835c1468cbc458382bc52a10a"
ENGINE_SCALE_CONFLICT_PRIOR_DELTA = "64bd400d1d68511b78606b864244eb7c0d57b44e8fb84fa40815ca53e22945e5"
ENGINE_SCALE_CONFLICT_GRAPH_DELTA = "697611c380aa91e2b1c1adcaae6305c3da1b8e6b06042f927af63d470fecad5e"
ENGINE_SCALE_CONFLICT_WRAPPER = "217ea4bcdc4f9c5fbd5d8c13206ef92c8dd2271d396c4f4c782384ad2dc98064"
ENGINE_SCALE_CONFLICT_STOPPED = "0188b877d18537d9da98bb7ed53b6bb1aa0241df2bbe7fa8e602c266187b6bbb"
ENGINE_SCALE_CONFLICT_RETRY_WALL = 2270
ENGINE_SCALE_CONFLICT_FIRST_WALL = 129.291
ENGINE_SCALE_CONFLICT_NATIVE_FIXTURE = (
    "crates/nevrotick-engine/src/nts_ke_experiment/transport/managed_close_fixture.rs"
)
ENGINE_SCALE_CONFLICT_TEST = (
    "source_policy::tests::configuration::"
    "source_configuration_smeared_scale_refuses_only_experimental_measurement"
)
ENGINE_SCALE_CONFLICT_ROSTER = {
    "default-ordinary": (288, "48aa2d51baa22538640dfd383078b106ab833aef5971e4dea2a02796167fbb35"),
    "default-doctests": (90, "de4eee700118467c20f02a5af10034630aa66ab7aec6f9c27f71444a76d2d62c"),
    "ep1-ordinary": (585, "c55e41b045830ff7f9d119ea85f99a36b5d3a235737f9d4a1bd3a23ca9ee2fe8"),
    "ep1-doctests": (91, "b382fb2f46ca19b8cf406c05cdefe99fe226f53fba21839e5c32ec229b8529e3"),
}
ENGINE_PLACEMENT_CANDIDATE = "8f98bdae618d53c3b087809af6e904046db5a0ea3dbd0310d7634e829f22bae9"
ENGINE_PLACEMENT_TERMINAL = "ebbc33d919d96b7ace9466e7a17a135bbf92ae0af88a839d8f075572385b6107"
ENGINE_PLACEMENT_MANIFEST = "27f16874cde2e46740951280026b11ae431a55cf2066beb435fb28447b01862c"
ENGINE_PLACEMENT_BOUNDARY = "e2137a5632d20775ef0af5a2f6c75ceeb06b3352ef124ed94ebde24233bd16b7"
ENGINE_PLACEMENT_WRAPPER = "bb1df7fd1b9242513fdbbbf3322370a25cd3d3c0a273327b1d094293881c181d"
ENGINE_PLACEMENT_PRIOR_DELTA = "2a2ffd6e886084ed163cfb57f910ce309e541d432208c8ffcd76846548c4a11b"
ENGINE_PLACEMENT_FOCUS_REFERENCES = "c7d569fd8346c3c9f9ff712792b7a816c2ad4289ebb33a4c47e578f989c218ff"
ENGINE_PLACEMENT_INHERITED_RETRY = "a5fef0e0147e64ac0ecbb68b843e9f0984763c3273d408194b4e5d7bb43aa191"
ENGINE_PLACEMENT_FORMAT_REFERENCES = "ebef65633905a3b34df0bb057909035ccc0c8f697a399f55b8394631b972307d"
ENGINE_PLACEMENT_STOPPED = "d03d54e071e137602acb4888b1aa6af0378393f7fe0943b7627ae4c99c7b86ca"
ENGINE_PLACEMENT_UNFORMATTED = "a130c4f7aff936ef728bf9409a10153caa7d062a3e63d68adf2e619e9894e53c"
ENGINE_PLACEMENT_UNFORMATTED_MANIFEST = "07f45748fbe9d5173c0d25337b7dc45af2904d9419f1ee51e0c15031aedfe397"
ENGINE_PLACEMENT_FORMAT_FAILURE = "30f7b2520edf988fc6040e88baf28d0f76c689cf444bd0496c6744c544c46053"
ENGINE_PLACEMENT_FIRST_BOUNDARY = "98aa0aaa0ba9a129d6548d10f2a55ff3ddff7c29553a630520e078fcf5b01d89"
ENGINE_PLACEMENT_RETRY_WALL = 2383
ENGINE_PLACEMENT_FIRST_WALL_MS = 16105
ENGINE_PLACEMENT_TEST = (
    "nts_ke_experiment::managed_preview_tests::"
    "managed_protected_report_keeps_original_application_wall_elapsed_bounds"
)
ENGINE_PLACEMENT_ROSTER = {
    "default-ordinary": ENGINE_SCALE_CONFLICT_ROSTER["default-ordinary"],
    "default-doctests": (90, "cf9a6463a093cccdfd4a667a74d3269244246f04e17a6230a91a3f25c24890de"),
    "ep1-ordinary": (586, "33b079624f44119ae12e7928bc519a14cb29e5dab3b38b75e04b35d648146dc6"),
    "ep1-doctests": (91, "ef3b2b18d5d15c2ae8b99fd6f601ca006afd1e87d5824e4c48645ef8281e089f"),
}
ENGINE_PLACEMENT_EP1_CASE_DIGEST = "f8fe12a23ece323ccbfd1818e5056a69e5bb211bcb5b66a0f1639706d5221c23"
ENGINE_PROTECTED_DEBIT_CANDIDATE = "acc427ae2e19bf48011b4abdfa01ef9fdd973929a16aa068c8e29101c698333d"
ENGINE_PROTECTED_DEBIT_TERMINAL = "b1f628d08e4e044edbf010cc919ebc32be0358d8d44330187775526d31de472e"
ENGINE_PROTECTED_DEBIT_MANIFEST = "e29ea3c9d6fe505d977f5d5134b90a70f45ee3afa8f08e8799631a9f0a37b950"
ENGINE_PROTECTED_DEBIT_BOUNDARY = "b33ad33869a91649c5ef0f76f743b793b638a8f229211cb9a608bd932ee9bf4f"
ENGINE_PROTECTED_DEBIT_WRAPPER = "158368ee57cf5ae42ffbf36fea68bcfc658a86e2e682bd45a52cce6f936bf8f5"
ENGINE_PROTECTED_DEBIT_PRIOR_DELTA = "bd84c06e7aea91c190fcdde1dc636c36e5a8702c4d3f04422407446867409a49"
ENGINE_PROTECTED_DEBIT_GRAPH_DELTA = "29ab98a45ef08d2a68530e9ff4980bed955dcfb5e9bdd0c666bff329045b37d9"
ENGINE_PROTECTED_DEBIT_FOCUSED = "9ddbc82842ca9819e77c86a82a0205846575e68fa96f8e0b847d639b7e4da9d4"
ENGINE_PROTECTED_DEBIT_EP1_CASE_DIGEST = "e7c124237b7d965fba591a696fa55f4daaf522717ce24bebcc9c9a18114a7620"
ENGINE_PROTECTED_DEBIT_ROSTER = ENGINE_PLACEMENT_ROSTER | {
    "ep1-ordinary": (600, "caf3fe2393537ef970c3bf7043663273b06b2e31a44991adc06f77591b7482c0"),
}
ENGINE_PROTECTED_DEBIT_ADDITIONS = frozenset((
    "nts_ke_experiment::datagram::tests::managed_protected_pending_endpoint_deny_wakes_silent_receive",
    "nts_ke_experiment::datagram::tests::managed_protected_pending_expiry_observes_already_latched_stop",
    "nts_ke_experiment::datagram::tests::managed_protected_pending_ignores_deny_for_another_port",
    "nts_ke_experiment::datagram::tests::managed_protected_pending_rate_preserves_readiness_and_reply",
    "nts_ke_experiment::datagram::tests::managed_protected_pending_stop_and_revision_replacement_wake_receive",
    "nts_ke_experiment::datagram::tests::protected_consumed_known_debits_after_expiry",
    "nts_ke_experiment::datagram::tests::protected_consumed_known_debits_after_stop",
    "nts_ke_experiment::datagram::tests::protected_consumed_unknown_retires_after_expiry",
    "nts_ke_experiment::datagram::tests::protected_consumed_unknown_retires_after_stop",
    "nts_ke_experiment::datagram::tests::protected_known_live_reply_charges_rejected_and_matching_origins",
    "nts_ke_experiment::datagram::tests::protected_stop_before_receive_preserves_datagram_credit",
    "nts_ke_experiment::managed_preview_tests::managed_ke_preview_power_event_respects_sealing_and_join",
    "nts_ke_experiment::managed_preview_tests::managed_ke_preview_power_registration_refusal_releases_slot",
    "nts_ke_experiment::managed_preview_tests::managed_ke_preview_stop_after_sealing_refuses_final_commit",
))
ENGINE_FINAL_FRAME_CANDIDATE = 'a0df3abd9f964cea8bf28d184ac0e28daaad1b89c29696549a85adda4e45c9ee'
ENGINE_FINAL_FRAME_MANIFEST = 'fceb74ede2e4dd5af5fed45a0865d5e9e3b665ad428105599e717ad118e875ce'
ENGINE_FINAL_FRAME_TERMINAL = 'a366959a0df5a805a3d8dd2aadbe2674a72a525e907fbaf453e929e6b9cc78fc'
ENGINE_FINAL_FRAME_BOUNDARY = '8c8a6cfade33a0e119b1a16e843cb8d1297ddd82904a0fe22492cc6da5438b8b'
ENGINE_FINAL_FRAME_WRAPPER = 'c34d1e0e25a7116da40a8334925c2a0368557aad317db94493658e43a8870478'
ENGINE_FINAL_FRAME_FOCUSED = '3335efdb9098640a888223f3508eed71b4b6408a211943067fc784a2c839ed15'
ENGINE_FINAL_FRAME_FOCUSED_CANDIDATE = '0eb50c7f02c9f7623e88e2cde3ee3dd412308cb7c6a316227c9f9797c60693b6'
ENGINE_FINAL_FRAME_FIXTURE = 'd9bf79ce5512a557504c195a04a58b93f61e1dda7065503d67e887732761e87f'
ENGINE_FINAL_FRAME_FIXTURE_CANDIDATE = '0495021fa1525ca85aca79c78f370ceb60f13f4db73557c7c1ba8132199187c4'
ENGINE_FINAL_FRAME_WORKER_FIXTURE = '3b2a92a90c0c389ee3245cdf8af43f907c0b3efd7c811eba485552701b220b5f'
ENGINE_FINAL_FRAME_FIXTURE_CASES = ('nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::managed_loopback_successor_commits_exact_deny_after_normal_join', 'nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::managed_loopback_successor_refuses_same_values_replacement_before_commit')
ENGINE_FINAL_FRAME_INCOMPLETE = 'f6a79d66c59665cd792df5c58465e828bddd9124f5d391dd33193857e0eee865'
ENGINE_FINAL_FRAME_INCOMPLETE_CAPTURE = '2aa728bac7389dbb7036b4152932371d8ef0e26963c5dda8c0538bca150c96c0'
ENGINE_FINAL_FRAME_INCOMPLETE_PLACEMENT = 'aa36d0874c26db8b2712eb1e5c0ef472bc3fabcdbf78ae1cc39cc803f55e4291'
ENGINE_FINAL_FRAME_RETRY_WALL = 2250
ENGINE_FINAL_FRAME_WORKER_PATH = 'crates/nevrotick-engine/src/nts_ke_experiment/recovery/worker_tests.rs'
ENGINE_FINAL_FRAME_WORKER_PRE = '2e2f88f996683153f1be926bb42b820c0394525286692b507c8fa38c8b4441a4'
ENGINE_FINAL_FRAME_WORKER_POST = 'afc5836f5cb874cf6ff6e816f71f2b257538f8144aea88c6a210536f5424e53b'
ENGINE_FINAL_FRAME_EP1_CASE_DIGEST = '50bf4855d5a7601a88979520b62aba6f39d6f5e9bbdc55d6406c0592c19eb4a1'
ENGINE_FINAL_FRAME_TEST_PATH = 'crates/nevrotick-engine/src/nts_ke_experiment/datagram/tests.rs'
ENGINE_FINAL_FRAME_ADDITIONS = frozenset((
    'nts_ke_experiment::datagram::tests::protected_final_known_frame_retires_after_expiry',
    'nts_ke_experiment::datagram::tests::protected_final_known_frame_retires_after_stop',
    'nts_ke_experiment::datagram::tests::protected_final_known_frame_retires_after_stop_and_expiry',
))
ENGINE_FINAL_FRAME_DOCS = frozenset((
    'docs/plans/development-plan.md',
    'tasks/engine-transport-pilot.md',
    'tasks/windows11-integration.md',
))
ENGINE_FINAL_FRAME_ROSTER = ENGINE_PROTECTED_DEBIT_ROSTER | {
    "ep1-ordinary": (603, "9a15d0fb55d3045d1a6231d105c0a1da1f0841c9447929863f9e2ed57e175c26"),
}
ENGINE_STALE_DEBIT_CANDIDATE = '3609043e862889ed20e29e85345c06c019b6887049833af3fd1ca6b0ec82e382'
ENGINE_STALE_DEBIT_MANIFEST = '18d40af6c21180b605590e1e87cf4eb97cfb269162526aff7d7ea6b84aa93b4a'
ENGINE_STALE_DEBIT_TERMINAL = '9ea7c824f7b439aef49f81d73adbefe30f5c55b7625a0ab9d96d2c229332fa34'
ENGINE_STALE_DEBIT_BOUNDARY = 'df2ab64c4c7b73dc10a9ddad5b96a42a83bece0cc546de7e7fc948db91565431'
ENGINE_STALE_DEBIT_WRAPPER = 'b6a2a8c2269ca4703533f62a8f78dae311b3f84c4dbc09a184f85a91ee05ceb4'
ENGINE_STALE_DEBIT_PREPARATION = 'd786a321b2b98acb8535aeda63d28a478a56bec3fb4065aba4c8903c645dd3ae'
ENGINE_STALE_DEBIT_FULL = '8c8ee48f35cbec01534a1efbb10f639565e0e89a8b305af586512199047fc139'
ENGINE_STALE_DEBIT_READBACK = '39a4c867b03d1906839761da97194fa8e7becd363bf411d952f9f00709c1a3e4'
ENGINE_STALE_DEBIT_CASE = 'nts_ke_experiment::association::ledger::tests::receiver_stale_debit_keeps_current_pending'
ENGINE_STALE_DEBIT_TEST_PATH = 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/tests.rs'
ENGINE_STALE_DEBIT_TEST_PRE = 'fcee7434236a431abd90d11efcb7bdb61e367e457fa846edf849cbff5cfca5d7'
ENGINE_STALE_DEBIT_TEST_POST = '031e1f2a0896350a359c3ab52af4f48d75139e112787f612cda23de41b45ee7b'
ENGINE_STALE_DEBIT_EP1_CASE_DIGEST = '0e6f6d3e9d8b210799ee61270bff21fe40053efe69af799090d7eddabac03509'
ENGINE_STALE_DEBIT_FULL_COMMANDS = {'ep1-ordinary': '15fb7af1ad4678d1c3d428504b671e132361eeef54ba96e6e83964805a569118', 'ep1-optimized': '6c83ad7594673fb937511174ea5d07a9b0c529fca0ba2bf1f22f50d81b5c682d', 'ep1-doctests': '906f3b3a5cf44e0a4911772e655887aa7463852844d8a6e44c179c25a357d2e8'}
ENGINE_STALE_DEBIT_ROSTER = ENGINE_FINAL_FRAME_ROSTER | {"ep1-ordinary": (604, "aae2f8642303d4d8f39b4a8fce24a683b98289481deea570e9efe423a2ef9947")}
ENGINE_JOINED_RECEIVING_CANDIDATE = "1e6b438b9f86fabe50673b8571642d2bba323d62eb131d401280e3b2b2574435"
ENGINE_JOINED_RECEIVING_MANIFEST = "fb8151b66bb271ac9d1373f6cb280c03490e133e85773d9d728764e482b3bdc0"
ENGINE_JOINED_RECEIVING_TERMINAL = "5511669e84f4219cf0caa3feacbc0c0d458fce2f75c2e50819ed113b0285080f"
ENGINE_JOINED_RECEIVING_BOUNDARY = "d96ff06a587f6772448866f6b94e1b703a45a9b343395613e67faf3af65f6855"
ENGINE_JOINED_RECEIVING_WRAPPER = "f3ffb5927f4d76c8665e1bc5e26c9edc886432b5b72ccc4f67ddac4472d880b4"
ENGINE_JOINED_RECEIVING_GRAPH = "c543dbebe170e04aa3889d82646edeca791fab574923e8bd0862ba99dfc6b842"
ENGINE_JOINED_RECEIVING_FOCUSED = "e39a89ba30620e48ca43ed6dae02de51e3a53e3092f4e4a08df30e80f5e1b211"
ENGINE_JOINED_RECEIVING_RESOURCES = "b6b30c3c412f37a545b51702d7ca7577483f872071eab712f0ff48d49c864369"
ENGINE_JOINED_RECEIVING_CARGO_HOME = "d32fb62c39621c1a8e362cbd460897959f767f65b1bd41a47f7c3a5127984217"
ENGINE_JOINED_RECEIVING_ROSTER = {
    "default-ordinary": (318, "6783e4ebc08b35738253d0337799c4e610e254c4124f0513943eb6464b0544f7"),
    "ep1-ordinary": (684, "23eb26e971617c49da833284306e3040cf686fea22db11e5f5a90892e238168a"),
    "default-doctests": (90, "84e67f5ad5247fd75c90ab9f08b6c3dc58d5b3aff989c0f1371e295c4ebb9b83"),
    "ep1-doctests": (91, "154bcf865eb9cba5ae2f20db963e94b1eaee6d8fcb79c5bf9f48b352dc1d2ecc"),
}
ENGINE_JOINED_RECEIVING_CASE_DIGESTS = {
    "default": "93705d1beb0f3d849f6b481197f3df531ea5139d1011f7de0ff0a1b60d9a315d",
    "ep1": "d5ad64ac8f530ff83b7849a441a0b55db0fa20193c66d74d390c09aa0b74d966",
}
ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE = "d5729afdb817caeb07928ac960d6649427e756ec6135b19f9543cc2c45ff39e7"
ENGINE_JOINED_RECEIVING_BOUNDARY_MANIFEST = "b4ee2b0f5681f9e8f55a9a241f3df961155be941f6585ad945a35b60b73c66a1"
ENGINE_JOINED_RECEIVING_BOUNDARY_TERMINAL = "a3a9c8ef61bd5a642abad9010d36bc5deb439b63fa6222392cd3b7c500a5eb16"
ENGINE_JOINED_RECEIVING_BOUNDARY_CONTROL = "c7b2ad32b5b8e63da5ac8e9f51064f635739c19dfe56c567821ed87db208c008"
ENGINE_JOINED_RECEIVING_BOUNDARY_WRAPPER = "ac138acd9cae3f6ad655386dff285bfd3dad6a07902929f20791776ec4f2b3a8"
ENGINE_JOINED_RECEIVING_BOUNDARY_RESOURCES = "f7efbc90f1f41359e90cafd492f0a09468f86bfc5ee53a0f7e81b4033d5eae9d"
ENGINE_JOINED_RECEIVING_BOUNDARY_FOCUSED = "5de7d276484d61b299a5159179b5567fadf59c57d813c3ad6d4b524d439274ab"
ENGINE_JOINED_RECEIVING_PREDECESSOR_READBACK = "a6569ace03b3c10783444a3c28744220b08cc765871671b03f14080d2602c690"
ENGINE_JOINED_RECEIVING_FAMILY = (ENGINE_JOINED_RECEIVING_CANDIDATE, ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE)
ENGINE_JOINED_RECEIVING_BOUNDARY_DELTA = {
    "crates/nevrotick-engine/src/nts_ke_experiment/job.rs": (
        "a4cfdc52e2d7bc9f1dbeff7e407dd52c62b064d0d7847d3dc7b31d2a3554ec90",
        "48a49db6cb9a297f82bf335ae3033aa4ed781a2509f80e8990665e8f29235555",
    ),
    "crates/nevrotick-engine/src/nts_ke_experiment/recovery/worker_tests.rs": (
        "633fb069e9d4a7f806603522f4ea9fd8c99fe8b39bb9b85957fc58036ba123c6",
        "46e1c31b26824e9e153c0933abdec78e9beff5b7a5a4e54705b2a24f296bda51",
    ),
}
ENGINE_JOINED_RECEIVING_BOUNDARY_CASES = tuple(
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::" + name
    for name in ("managed_joined_receiving_checks_original_commit_boundary",
                 "managed_joined_receiving_without_context_preserves_originals")
)
ENGINE_JOINED_RECEIVING_BOUNDARY_ROSTER = ENGINE_JOINED_RECEIVING_ROSTER | {
    "ep1-ordinary": (686, "67e486fbbf7bb092e51f41fb673833625487915196587f30abf78858d28f0112"),
}
ENGINE_JOINED_RECEIVING_BOUNDARY_EP1_CASE_DIGEST = "024466c2ce2753984b13971d6a4916ca0d934b5cb1d2ac9b2de391927c406c71"
ENGINE_REVIEW_TERMINAL = "6459d484673913359fc69e58596c32ef113eca27363fa1ebc75ea14518ae31f7"
ENGINE_REVIEW_MANIFEST = "1cc89353a08cab0ca94d365ac65a5319709e4904a207122cc095e3aad19bfc35"
ENGINE_REVIEW_BOUNDARY = "a47858cedf91f8ae1d0fe3bb5440377cfa11d785b21afc34a4984aafa92b4994"
ENGINE_REVIEW_EP1_CASE_DIGEST = "48e0943c59713bdc7d972342ca9fc41f3268c34e8a5c90212010b82d4d0144c5"
ENGINE_REVIEW_PRIOR = "bfc89aa8c04f87d39449bded8e9171a6960a2fc3ad57ceb7d7372f1ad6a3589f"
ENGINE_REVIEW_PRIOR_DELTA = "d1dc50b9815343e8983df198708fa7b2e7db12dcf87b89dbf8b324fab5123310"
ENGINE_REVIEW_GRAPH_DELTA = "9a8213d18aa5d10916e6ccb86f43e005c88b0d1355d1213fc1e2cac9b96cbba0"
ENGINE_KE_SOURCE_SCALE_CANDIDATE = "5fc32aace7312d232a753325ccb124d104c74d7426c26d8a89ded0b8aa17f045"
ENGINE_KE_SOURCE_SCALE_TERMINAL_SHA256 = "87e79bc74d146527ee2e4c9d9ebd1d78f8de9ebac4aa9f71bfbb3f1d63474f6b"
ENGINE_KE_SOURCE_SCALE_DOCTEST_DIGEST = "f40e89f48370695701eea46bc046187451fd50adedada6a3517ee48240397e55"
ENGINE_KE_SOURCE_SCALE_CASE_DIGEST = "7924f239b872233b1993e4b4e317e5e9d2c1c9cad8ae73252c82e07db8029322"
ENGINE_KE_SOURCE_SCALE_GRAPH_PREDECESSOR = "dde78e2c87a143804407cf2a2f3e0092cb5d719954c5c5d6633d72547f6cc681"
ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256 = "4e3cf68485bcf74e88727e6620ec7b83c7b110023818a2ce6a46240fd99f8ef8"
ENGINE_KE_SOURCE_SCALE_GRAPH_DELTA = frozenset((
    "crates/nevrotick-engine/src/lib.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/managed_preview_tests.rs",
    "crates/nevrotick-engine/src/source_policy.rs",
    "crates/nevrotick-engine/tests/private/source_configuration.rs",
))
ENGINE_KE_DOCTEST_NAME = re.compile(
    r"[^\r\n]+[.]rs - [^\r\n]+ \(line [1-9][0-9]*\)(?: - compile(?: fail)?)?"
)
ENGINE_KE_SEND_AFTER_TEST = (
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "numerics_tests::facade_protected_numerics_send_after_wall_step_is_not_omitted"
)
ENGINE_KE_MANAGED_PROTECTED_TESTS = frozenset((
    "nts_ke_experiment::datagram::tests::managed_protected_attempts_keep_floors_after_failure_and_stop",
    "nts_ke_experiment::datagram::tests::managed_protected_rechecks_definition_after_readiness",
    "nts_ke_experiment::datagram::tests::managed_protected_reply_distinguishes_wire_peer_and_policy_epochs",
    "nts_ke_experiment::datagram::tests::managed_protected_send_uses_the_exact_shared_floor",
    "nts_ke_experiment::datagram::tests::managed_retired_first_request_allows_one_same_association_successor",
    "nts_ke_experiment::datagram::tests::managed_retired_authentication_refuses_uncommitted_and_invalidated_history",
    "nts_ke_experiment::managed_preview_tests::managed_protected_abnormal_join_preserves_sealed_configuration",
    "nts_ke_experiment::managed_preview_tests::managed_protected_refuses_preview_downgrade_before_charging",
    "nts_ke_experiment::managed_preview_tests::managed_protected_worker_preserves_committed_definition_through_join",
    "nts_ke_experiment::managed_preview_tests::managed_protected_worker_refuses_replacement_at_both_effects",
    "nts_ke_experiment::datagram::tests::managed_protected_deny_retires_only_the_exact_authenticated_successor",
    "nts_ke_experiment::datagram::tests::managed_protected_ordinary_follows_rejected_controls_without_renewal",
    "nts_ke_experiment::datagram::tests::managed_retired_deny_commits_only_its_source_and_exact_endpoint",
    "nts_ke_experiment::datagram::tests::managed_mixed_receive_admits_deny_on_final_frame",
    "nts_ke_experiment::datagram::tests::managed_mixed_receive_exhausts_on_invalid_final_frame",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::managed_deny_worker_retains_exact_history_without_numerical_projection",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::managed_deny_worker_post_record_stop_preserves_only_once_joined_history",
    "nts_ke_experiment::datagram::tests::managed_retired_rate_preserves_independent_intervals_and_exact_next_send",
    "nts_ke_experiment::datagram::tests::managed_protected_policy_wait_checks_out_cookie_only_when_due",
    "nts_ke_experiment::datagram::tests::managed_protected_policy_wait_refuses_before_cookie_checkout",
    "nts_ke_experiment::datagram::tests::managed_protected_policy_wait_last_credit_refuses_before_cookie_checkout",
    "nts_ke_experiment::datagram::reached::bound_endpoint_check_refuses_each_invalid_coordinate",
))
ENGINE_FINAL_REPLY_CUTOFF_TESTS = frozenset((
    "processing_guard_tests::pilot_final_guard_time_cutoff_rejects_selected_reply_once",
    "series::tests::reply_final_guard_time_cutoff_keeps_its_actual_receiving_checkpoint",
))
ENGINE_PLAIN_CAPTURE_TESTS = frozenset((
    "processing_guard_tests::unmanaged_send_cutoff_uses_both_clock_readings",
    "source_policy::tests::managed_send_cutoff_uses_both_clock_readings",
    "source_policy::tests::managed_inverted_pre_send_capture_refuses_before_reservation",
    "session::tests::mixed_series_capture_refusal_precedes_retained_healthy_facts_if_profiled",
))
ENGINE_RECEIVE_CAPTURE_TESTS = frozenset((
    "source_policy::tests::managed_plain_receive_cutoff_charges_once_before_policy_effect",
    "source_policy::tests::managed_plain_reversed_receive_before_cutoff_keeps_header_history",
    "nts_ke_experiment::datagram::tests::managed_retired_deny_refuses_receive_capture_at_cutoff",
))
ENGINE_RECEIVE_CHRONOLOGY_TESTS = frozenset((
    "source_policy::tests::managed_plain_reversed_receive_leaves_control_pending_until_ordered_reply",
    "nts_ke_experiment::datagram::tests::managed_retired_deny_refuses_reversed_receive_capture",
    "nts_ke_experiment::datagram::tests::managed_retired_deny_refuses_future_receive_tail",
))
ENGINE_KNOWN_RECEIVE_DEBIT_TEST = (
    "nts_ke_experiment::datagram::tests::known_protected_datagram_with_invalid_capture_charges_frame_and_bytes")
ENGINE_HEADER_AGE_TEST = (
    "source_policy::tests::managed_header_writes_receive_to_commit_age_only_for_ordered_capture")
ENGINE_ORDINARY_PROTECTED_RECEIVE_TEST = (
    "nts_ke_experiment::datagram::tests::ordinary_protected_reversed_receive_bracket_cannot_commit")
ENGINE_PROTECTED_RECEIVE_EFFECT_TESTS = frozenset((
    "nts_ke_experiment::datagram::tests::retired_completion_commit_respects_acceptance_and_expiry",
    "nts_ke_experiment::datagram::tests::recovery_tests::reversed_final_zero_cookie_capture_cannot_schedule_replacement",
    "nts_ke_experiment::datagram::tests::recovery_tests::final_zero_cookie_capture_at_cutoff_cannot_schedule_replacement",
))
ENGINE_KE_LEDGER_CHRONOLOGY_TESTS = frozenset((
    "nts_ke_experiment::association::ledger::tests::receive_span_checks_exact_capture_and_processing_boundaries",
    "nts_ke_experiment::datagram::tests::managed_retired_ordinal_two_cannot_open_first_successor",
))
ENGINE_KE_OBSERVED_FACADE_TESTS = frozenset((
    "nts_ke_experiment::managed_preview_tests::"
    "managed_protected_power_event_respects_reply_commit_and_join",
    "nts_ke_experiment::job::tests::"
    "synchronous_facade_collection_contention_retains_unconfirmed_actual_job",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "observed_facade_registration_failure_refuses_submission",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "observed_facade_early_event_cancels_the_original_worker",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "observed_facade_late_event_preserves_joined_protected_result",
))
ENGINE_KE_MANAGED_REPORT_TESTS = frozenset((
    "nts_ke_experiment::managed_preview_tests::"
    "managed_protected_report_preserves_declared_numerical_result",
    "nts_ke_experiment::managed_preview_tests::"
    "managed_preview_cannot_project_a_protected_report",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "numerics_tests::facade_protected_numerics_declared_era_zero_has_independent_math",
))
ENGINE_KE_TRANSPORT_CURRENTNESS_TESTS = frozenset((
    "nts_ke_experiment::managed_preview_tests::"
    "managed_ke_transport_refuses_replaced_definition_before_connect",
    "nts_ke_experiment::managed_preview_tests::"
    "managed_ke_transport_drops_owned_connect_on_revision_cutover",
    "nts_ke_experiment::managed_preview_tests::"
    "managed_ke_transport_rechecks_revision_after_each_adapter_return",
    "nts_ke_experiment::managed_preview_tests::"
    "managed_ke_transport_preserves_first_native_failure_after_replacement",
))
ENGINE_KE_LOOPBACK_AND_TRANSPORT_CUTOFF_TESTS = frozenset((
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "managed_loopback_successor_commits_exact_deny_after_normal_join",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
    "managed_loopback_successor_refuses_same_values_replacement_before_commit",
    "nts_ke_experiment::managed_preview_tests::"
    "managed_ke_transport_exchange_uses_original_later_cutoff",
    "nts_ke_experiment::managed_preview_tests::"
    "managed_ke_transport_refusal_keeps_initial_io_category",
))
ENGINE_KE_JOINED_RECEIVING_TEST_SOURCES = (
    "crates/nevrotick-engine/src/nts_ke_experiment/receiving/fixture.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/recovery/receiving_cases.rs",
)
ENGINE_KE_TEST_SOURCES = (
    "crates/nevrotick-engine/src/nts_ke_experiment/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/association/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/handoff/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/request_tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/retirement/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/response/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/response/fixtures.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/datagram/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/recovery/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/recovery/receiving_tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/recovery/worker_tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/recovery/numerics_tests.rs",
    *ENGINE_KE_JOINED_RECEIVING_TEST_SOURCES,
    "crates/nevrotick-engine/src/nts_ke_experiment/job_tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/managed_preview_tests.rs",
    "crates/nevrotick-engine/tests/private/source_configuration.rs",
    "crates/nevrotick-engine/tests/private/processing_guards.rs",
    "crates/nevrotick-engine/tests/private/series.rs",
    "crates/nevrotick-engine/tests/private/session.rs",
    "crates/nevrotick-engine/tests/private/source_policy.rs",
)
ENGINE_KE_BOUND_INPUTS = tuple(dict.fromkeys((
    WRAPPER, TESTS, *COMMON.ENGINE_KE_CRITICAL, *ENGINE_KE_TEST_SOURCES,
    "scripts/portable-analysis.py", "scripts/tests/test_portable_analysis.py",
    "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/analysis-controls.py",
    "scripts/prepare-product.py", "policy/analysis-tools.json", "policy/verification-manifest.toml",
    "policy/tool-manifest.json", "policy/ui-m0-dependencies.json", ".cargo/mutants.toml",
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml",
    "tools/crypto-cng-lifecycle/Cargo.toml", "tools/crypto-cng-lifecycle/Cargo.lock",
)))
ENGINE_KE_FIXTURE_SOURCE = "tools/crypto-aes-siv-witness/tests/ep2_response_fixtures.rs"
ENGINE_KE_FIXTURE_MANIFEST = "tools/crypto-aes-siv-witness/Cargo.toml"
ENGINE_KE_FIXTURE_LOCK = "tools/crypto-aes-siv-witness/Cargo.lock"
ENGINE_KE_FIXTURE_NAMES = frozenset((
    "HEALTHY ZERO_RECEIVE ZERO_TRANSMIT ZERO_TIMESTAMPS ZERO_COOKIES EMPTY_COOKIE TWO_COOKIES EIGHT_COOKIES NINE_COOKIES "
    "MISSING_UID SHORT_UID DUPLICATE_UID CLEAR_COOKIE CLEAR_PLACEHOLDER INNER_UID INNER_AUTH "
    "INNER_PLACEHOLDER UNKNOWN_FIELDS_AND_SUFFIX SUFFIX_DUPLICATE_UID_AUTH MALFORMED_INNER "
    "MAX_INNER_FIELDS TOO_MANY_INNER MAX_OUTER_FIELDS TOO_MANY_OUTER MAX_PACKET_BYTES "
    "TOO_MANY_PACKET_BYTES WRONG_UID WRONG_ORIGIN WRONG_VERSION WRONG_MODE UNSYNCHRONIZED "
    "STRATUM_ZERO STRATUM_SIXTEEN NONCE_ZERO NONCE_ONE NONCE_TWELVE").split())


def validate_engine_ke_fixture_output(text: str) -> tuple[str, dict]:
    """Admit one entered generator and only its complete bounded public literals."""
    name = "generate_ep2_response_fixtures"
    if type(text) is not str or len(text.encode()) > 256 * 1024:
        raise Incomplete("Public fixture capture exceeds its finite text bound.")
    if read_successful_address_records(text) != [name]:
        raise Incomplete("Public fixtures require exactly their one actual successful generator.")
    lines = text.splitlines(keepends=True)
    starts = [index for index, line in enumerate(lines) if line.rstrip("\r\n") == f"---- {name} stdout ----"]
    if len(starts) != 1:
        raise Incomplete("Public fixtures require one captured generator output block.")
    end = next((index for index in range(starts[0] + 1, len(lines)) if lines[index].strip() == "successes:"), None)
    if end is None or next((line.strip() for line in lines[end + 1:] if line.strip()), None) != name:
        raise Incomplete("Public fixture capture lacks its complete success footer.")
    source = "".join(lines[starts[0] + 1:end]).strip() + "\n"
    if len(source.encode()) > 128 * 1024:
        raise Incomplete("Public fixture source exceeds its finite text bound.")
    rows = source.splitlines()
    header = ["// SPDX-License-Identifier: GPL-2.0-only",
        "//! Public component fixtures generated through the existing maintained AES-SIV API.",
        "//! No TLS/exporter/native/entropy or production authentication provenance is claimed."]
    if rows[:3] != header:
        raise Incomplete("Public fixture source lacks its fixed public provenance header.")
    position, total, fixtures = 3, 0, {}
    while position < len(rows):
        if not rows[position].strip():
            position += 1
            continue
        document = re.fullmatch(r"/// Public independently generated valid-tag fixture: ([A-Z_]+)[.]", rows[position])
        position += 1
        declaration = re.fullmatch(r"pub\(in crate::nts_ke_experiment\) const ([A-Z_]+): &\[u8\] = &\[", rows[position]) if position < len(rows) else None
        if (document is None or declaration is None or document[1] != declaration[1]
                or declaration[1] not in ENGINE_KE_FIXTURE_NAMES or declaration[1] in fixtures):
            raise Incomplete("Public fixture declarations are unknown, duplicated or malformed.")
        position += 1
        payload = bytearray()
        while position < len(rows) and rows[position] != "];":
            line = rows[position].strip()
            position += 1
            if not line:
                continue
            tokens = line.split(",")
            if tokens[-1] or any(re.fullmatch(r"0x[0-9a-f]{2}", token.strip()) is None for token in tokens[:-1]):
                raise Incomplete("Public fixture bodies permit only comma-terminated literal bytes.")
            if len(payload) + len(tokens) - 1 > 4100:
                raise Incomplete("Public fixture packet exceeds its 4100-byte extent.")
            payload.extend(int(token.strip()[2:], 16) for token in tokens[:-1])
        if not payload or position == len(rows):
            raise Incomplete("Public fixture packet is empty or unterminated.")
        position += 1
        total += len(payload)
        if total > 128 * 1024:
            raise Incomplete("Public fixture aggregate payload exceeds its finite bound.")
        fixtures[declaration[1]] = {"bytes": len(payload), "sha256": hashlib.sha256(payload).hexdigest()}
    if set(fixtures) != ENGINE_KE_FIXTURE_NAMES:
        raise Incomplete("Public fixtures omit part of the fixed thirty-six-case inventory.")
    return source, {"generated": True, "tests_executed": 1, "payload_bytes": total, "fixtures": fixtures}


def admit_engine_ke_fixture_lock(root: Path, dependency: dict) -> dict:
    """Require the actual standalone lock to use only selected Root registry identities."""
    standalone = tomllib.loads(COMMON.read_analysis_text(root / ENGINE_KE_FIXTURE_LOCK))
    root_lock = tomllib.loads(COMMON.read_analysis_text(root / "Cargo.lock"))
    packages = standalone.get("package")
    if (standalone.get("version") != 4 or type(packages) is not list or not 0 < len(packages) <= 512
            or any(type(item) is not dict or type(item.get("name")) is not str
                   or type(item.get("version")) is not str for item in packages)):
        raise Incomplete("Public fixture standalone lock has an invalid bounded package inventory.")
    frozen = {(item["name"], item["version"]): item for item in root_lock["package"]}
    selected = {(item["name"], item["version"]): item for item in dependency["packages"]}
    identities, registry, locals_ = set(), [], set()
    for item in packages:
        identity = (item["name"], item["version"])
        if identity in identities or identity not in frozen:
            raise Incomplete("Public fixture standalone lock escapes or duplicates Root identities.")
        identities.add(identity)
        if "source" not in item:
            if "source" in frozen[identity] or "checksum" in item:
                raise Incomplete("Public fixture local identity differs from Root.")
            locals_.add(identity)
        elif (item["source"] != "registry+https://github.com/rust-lang/crates.io-index"
                or item["source"] != frozen[identity].get("source")
                or identity not in selected or type(item.get("checksum")) is not str
                or item["checksum"] != frozen[identity].get("checksum")
                or item["checksum"] != selected[identity]["checksum"]):
            raise Incomplete("Public fixture registry source/checksum lacks selected Root admission.")
        else:
            registry.append("@".join(identity))
    if locals_ != {("nevrotick-s115-aes-siv-fixture", "0.0.0"), ("ntp-wire", "0.0.0")}:
        raise Incomplete("Public fixture standalone lock acquired an unadmitted local owner.")
    return {"standalone_lock_sha256": PRODUCT.sha256(root / ENGINE_KE_FIXTURE_LOCK),
            "registry_packages": sorted(registry)}


def execute_engine_ke_fixtures(args, report: dict) -> None:
    """Generate public constants under fresh custody without stale Engine test credit."""
    if (getattr(args, "profile", "legacy") != "ke_association" or args.lane != "fixtures"
            or args.subject != "all" or args.tool_admission_profile != "stable-core"):
        raise Incomplete("Public fixtures require their exact Engine KE role, stable tools and all selection.")
    policy = read_json(ROOT / ENGINE_KE_POLICY)
    COMMON.require_engine_ke_role(ENGINE_KE_SCOPE, policy)
    if policy["campaigns_admitted"] or policy["mutation_inventories"]:
        raise Incomplete("Public fixture preparation requires closed mutation campaign admission.")
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Public fixtures require seven GiB before admission.")
    dependency, identity = PRODUCT.admit_product(ROOT)
    PRODUCT.SLINT.profile_rows(dependency, PRODUCT.SLINT.EP1_PROFILE)
    if any(name not in identity for name in (ENGINE_KE_FIXTURE_SOURCE, ENGINE_KE_FIXTURE_LOCK)):
        raise Incomplete("The actual public generator is outside the admitted product source inventory.")
    graph = admit_engine_ke_fixture_lock(ROOT, dependency)
    inputs = (*engine_ke_bound_inputs(COMMON.canonical_sha256(identity)),
              ENGINE_KE_FIXTURE_SOURCE, ENGINE_KE_FIXTURE_MANIFEST, ENGINE_KE_FIXTURE_LOCK)
    current = {name: PRODUCT.sha256(ROOT / name) for name in inputs}
    selected = argparse.Namespace(**vars(args))
    selected.lane = "baseline"
    report.update(profile="ke_association", scope="Public fixture generation only; no Engine test or metric credit",
        candidate_sha256=COMMON.canonical_sha256(identity), source_file_count=len(identity),
        fixture_generation={"generated": False}, fixture_graph=graph, qualification_passed=False,
        selected_host_metrics_passed=False, selected_host_qualification_passed=False,
        native_windows_qualification="not performed")
    session = COMMON.Session(ROOT, selected, report, ENGINE_KE_SCOPE, policy, inputs)
    try:
        if (session.product_identity != identity or read_json(session.source / ENGINE_KE_POLICY) != policy
                or any(PRODUCT.sha256(session.source / name) != value for name, value in current.items())):
            raise Incomplete("Captured public fixture source, helpers or policy differs before execution.")
        def run(label, argv):
            address_storage_checkpoint(session, policy, report, label, "before")
            try:
                return session.run(label, argv, 120)
            finally:
                address_storage_checkpoint(session, policy, report, label, "after")
        run("fixture-tree", ["cargo", "tree", "--package", "nevrotick-s115-aes-siv-fixture",
            "--manifest-path", "/work/source/" + ENGINE_KE_FIXTURE_MANIFEST,
            "--target", "x86_64-unknown-linux-gnu", "--frozen", "--edges", "normal,build,features"])
        raw = run("fixture-generator", ["cargo", "test", "--package", "nevrotick-s115-aes-siv-fixture",
            "--manifest-path", "/work/source/" + ENGINE_KE_FIXTURE_MANIFEST,
            "--test", "ep2_response_fixtures", "--frozen", "--jobs", "1", "--", "--exact",
            "generate_ep2_response_fixtures", "--show-output", "--test-threads=1"])
        source, facts = validate_engine_ke_fixture_output(raw)
        artifact = session.output / "ep2-response-fixtures.rs"
        with artifact.open("xb") as destination:
            destination.write(source.encode())
        facts.update(source_path=str(artifact), source_sha256=hashlib.sha256(source.encode()).hexdigest())
        if PRODUCT.sha256(artifact) != facts["source_sha256"]:
            raise Incomplete("Stored public fixtures differ from the actual complete emitted source.")
        report["fixture_generation"] = facts
        address_storage_checkpoint(session, policy, report, "fixture-source", "stored")
    finally:
        session.finish()
    if PRODUCT.sha256(artifact) != facts["source_sha256"]:
        raise Incomplete("Public fixture artifact changed during its final custody bookend.")


def engine_review_input_count(candidate: object) -> int | None:
    """Select each combined review candidate without changing the older binding."""
    if candidate in ENGINE_JOINED_RECEIVING_FAMILY:
        return 768
    if candidate in (ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE, ENGINE_PROTECTED_DEBIT_CANDIDATE, ENGINE_FINAL_FRAME_CANDIDATE, ENGINE_STALE_DEBIT_CANDIDATE):
        return ENGINE_SCALE_CONFLICT_INPUT_COUNT
    if candidate == ENGINE_REVIEW_CANDIDATE:
        return ENGINE_REVIEW_INPUT_COUNT
    return None


def engine_review_terminal_pin(candidate: object) -> str:
    """Choose the immutable combined terminal for the admitted review generation."""
    if candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE:
        return ENGINE_JOINED_RECEIVING_BOUNDARY_TERMINAL
    if candidate == ENGINE_JOINED_RECEIVING_CANDIDATE:
        return ENGINE_JOINED_RECEIVING_TERMINAL
    if candidate == ENGINE_STALE_DEBIT_CANDIDATE:
        return ENGINE_STALE_DEBIT_TERMINAL
    if candidate == ENGINE_FINAL_FRAME_CANDIDATE:
        return ENGINE_FINAL_FRAME_TERMINAL
    if candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE:
        return ENGINE_PROTECTED_DEBIT_TERMINAL
    if candidate == ENGINE_PLACEMENT_CANDIDATE:
        return ENGINE_PLACEMENT_TERMINAL
    if candidate == ENGINE_SCALE_CONFLICT_CANDIDATE:
        return ENGINE_SCALE_CONFLICT_TERMINAL
    return ENGINE_REVIEW_TERMINAL


def engine_ke_bound_inputs(candidate: object) -> tuple[str, ...]:
    """Bind the joined receiving owner and controls for current candidates.

    Retain the exact helper closure of the named predecessor evidence.
    """
    inputs: tuple[str, ...] = ENGINE_KE_BOUND_INPUTS
    if candidate in (ENGINE_KE_SOURCE_SCALE_CANDIDATE, ENGINE_REVIEW_CANDIDATE,
                     ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE,
                     ENGINE_PROTECTED_DEBIT_CANDIDATE, ENGINE_FINAL_FRAME_CANDIDATE,
                     ENGINE_STALE_DEBIT_CANDIDATE):
        later: tuple[str, ...] = (COMMON.ENGINE_KE_JOINED_RECEIVING_SOURCE, *ENGINE_KE_JOINED_RECEIVING_TEST_SOURCES)
        inputs = tuple(name for name in inputs if name not in later)
    if candidate in (ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE, ENGINE_PROTECTED_DEBIT_CANDIDATE, ENGINE_FINAL_FRAME_CANDIDATE, ENGINE_STALE_DEBIT_CANDIDATE):
        return (*inputs, ENGINE_SCALE_CONFLICT_NATIVE_FIXTURE)
    return inputs


def engine_ke_case_counts(candidate: object) -> tuple[int, int]:
    """Keep historical rosters distinct from the exact review candidate."""
    if candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE:
        return 686, 360
    if candidate == ENGINE_JOINED_RECEIVING_CANDIDATE:
        return 684, 358
    if candidate == ENGINE_STALE_DEBIT_CANDIDATE:
        return (604, 312)
    if candidate == ENGINE_FINAL_FRAME_CANDIDATE:
        return (603, 311)
    if candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE:
        return (600, 308)
    if candidate == ENGINE_PLACEMENT_CANDIDATE:
        return 586, 294
    if candidate == ENGINE_SCALE_CONFLICT_CANDIDATE:
        return 585, 293
    return (584, 293) if candidate == ENGINE_REVIEW_CANDIDATE else (
        ENGINE_KE_ORDINARY_COUNT, ENGINE_KE_CASE_COUNT)


def validate_engine_ke_inventory(names: object, *, candidate: object = None) -> list[str]:
    """Require the fixed roster, including forwarding, cutoff and capture regressions.

    Reuse this admission at policy, execution and completed-cache retirement;
    a historical roster or same-count substitution cannot omit a required case.
    This checks declared identity, never whether a test body actually executed.
    """
    ordinary_count, ke_count = engine_ke_case_counts(candidate)
    if (type(names) is not list or len(names) != ordinary_count
            or any(type(name) is not str or not 0 < len(name) <= 4096 for name in names)
            or names != sorted(set(names))
            or sum(name.startswith("nts_ke_experiment::") for name in names) != ke_count
            or ENGINE_KE_SEND_AFTER_TEST not in names
            or not ENGINE_KE_MANAGED_PROTECTED_TESTS.issubset(names)
            or not ENGINE_FINAL_REPLY_CUTOFF_TESTS.issubset(names)
            or not ENGINE_PLAIN_CAPTURE_TESTS.issubset(names)
            or not ENGINE_RECEIVE_CAPTURE_TESTS.issubset(names)
            or not ENGINE_RECEIVE_CHRONOLOGY_TESTS.issubset(names)
            or ENGINE_KNOWN_RECEIVE_DEBIT_TEST not in names
            or ENGINE_HEADER_AGE_TEST not in names
            or ENGINE_ORDINARY_PROTECTED_RECEIVE_TEST not in names
            or not ENGINE_PROTECTED_RECEIVE_EFFECT_TESTS.issubset(names)
            or not ENGINE_KE_LEDGER_CHRONOLOGY_TESTS.issubset(names)
            or not ENGINE_KE_OBSERVED_FACADE_TESTS.issubset(names)
            or not ENGINE_KE_MANAGED_REPORT_TESTS.issubset(names)
            or not ENGINE_KE_TRANSPORT_CURRENTNESS_TESTS.issubset(names)
            or not ENGINE_KE_LOOPBACK_AND_TRANSPORT_CUTOFF_TESTS.issubset(names)
            or (candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE
                and COMMON.canonical_sha256(names) != ENGINE_JOINED_RECEIVING_BOUNDARY_ROSTER["ep1-ordinary"][1])
            or (candidate == ENGINE_JOINED_RECEIVING_CANDIDATE
                and COMMON.canonical_sha256(names) != ENGINE_JOINED_RECEIVING_ROSTER["ep1-ordinary"][1])
            or (candidate == ENGINE_REVIEW_CANDIDATE
                and COMMON.canonical_sha256(names) != SOURCE_POLICY_ENGINE_REVIEW_ROSTER["ep1-ordinary"][1])
            or (candidate == ENGINE_SCALE_CONFLICT_CANDIDATE
                and (ENGINE_SCALE_CONFLICT_TEST not in names
                     or COMMON.canonical_sha256(names) != ENGINE_SCALE_CONFLICT_ROSTER["ep1-ordinary"][1]))
            or (candidate == ENGINE_STALE_DEBIT_CANDIDATE
                and (ENGINE_STALE_DEBIT_CASE not in names
                     or COMMON.canonical_sha256(names) != ENGINE_STALE_DEBIT_ROSTER["ep1-ordinary"][1]))
            or (candidate == ENGINE_FINAL_FRAME_CANDIDATE
                and (not (ENGINE_PROTECTED_DEBIT_ADDITIONS | ENGINE_FINAL_FRAME_ADDITIONS).issubset(names)
                     or COMMON.canonical_sha256(names) != ENGINE_FINAL_FRAME_ROSTER["ep1-ordinary"][1]))
            or (candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE
                and (not ENGINE_PROTECTED_DEBIT_ADDITIONS.issubset(names)
                     or COMMON.canonical_sha256(names) != ENGINE_PROTECTED_DEBIT_ROSTER["ep1-ordinary"][1]))
            or (candidate == ENGINE_PLACEMENT_CANDIDATE
                and (ENGINE_PLACEMENT_TEST not in names or ENGINE_SCALE_CONFLICT_TEST not in names
                     or COMMON.canonical_sha256(names) != ENGINE_PLACEMENT_ROSTER["ep1-ordinary"][1]))):
        raise Incomplete(
            f"Engine KE requires all {ordinary_count} actual Engine tests including its "
            f"{ke_count} KE cases and the required send, cutoff, chronology, receive debit, header age, "
            "ordinary protected receipt, final-effect, ledger chronology, observed facade, managed report and "
            "transport currentness, phase, and loopback successor regressions.")
    return names


def validate_engine_ke_doctest_inventory(names: object, *, candidate: object = None) -> list[str]:
    """Require the candidate's complete Rustdoc roster with original annotations."""
    exact_doctests = (candidate == ENGINE_KE_SOURCE_SCALE_CANDIDATE
                      or engine_review_input_count(candidate) is not None)
    count = 91 if exact_doctests else ENGINE_KE_DOCTEST_COUNT
    digest = ENGINE_KE_SOURCE_SCALE_DOCTEST_DIGEST
    if candidate in ENGINE_JOINED_RECEIVING_FAMILY:
        digest = ENGINE_JOINED_RECEIVING_ROSTER["ep1-doctests"][1]
    elif candidate == ENGINE_PLACEMENT_CANDIDATE:
        digest = ENGINE_PLACEMENT_ROSTER["ep1-doctests"][1]
    elif candidate == ENGINE_STALE_DEBIT_CANDIDATE:
        digest = ENGINE_STALE_DEBIT_ROSTER["ep1-doctests"][1]
    elif candidate == ENGINE_FINAL_FRAME_CANDIDATE:
        digest = ENGINE_FINAL_FRAME_ROSTER["ep1-doctests"][1]
    elif candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE:
        digest = ENGINE_PROTECTED_DEBIT_ROSTER["ep1-doctests"][1]
    elif candidate == ENGINE_SCALE_CONFLICT_CANDIDATE:
        digest = ENGINE_SCALE_CONFLICT_ROSTER["ep1-doctests"][1]
    if (type(names) is not list or len(names) != count
            or any(type(name) is not str or len(name) > 4096 or ENGINE_KE_DOCTEST_NAME.fullmatch(name) is None
                   for name in names)
            or names != sorted(set(names))
            or (exact_doctests and COMMON.canonical_sha256(names) != digest)):
        raise Incomplete("Engine KE requires its complete exact annotated Rustdoc inventory.")
    return names


def validate_engine_ke_policy(policy: dict, *, campaign: bool = False) -> dict:
    """Bind the exact current source/helper set, ordinary tests and Rustdoc names."""
    COMMON.require_engine_ke_role(ENGINE_KE_SCOPE, policy)
    binding = policy["binding"]
    fields = {"candidate_sha256", "source_file_count", "inputs_sha256", "test_inventory",
              "doctest_inventory", "source_context_sha256", "ordinary_evidence", "root_graph_evidence"}
    if type(binding) is not dict or set(binding) != fields:
        raise Incomplete("Engine KE source/evidence binding is incomplete.")
    for field in ("candidate_sha256", "source_context_sha256"):
        if type(binding[field]) is not str or re.fullmatch(r"[a-f0-9]{64}", binding[field]) is None:
            raise Incomplete("Engine KE candidate/context digest is malformed.")
    pins = binding["inputs_sha256"]
    candidate = binding["candidate_sha256"]
    if (type(binding["source_file_count"]) is not int or binding["source_file_count"] <= 0
            or type(pins) is not dict or set(pins) != set(engine_ke_bound_inputs(candidate))
            or any(type(value) is not str or re.fullmatch(r"[a-f0-9]{64}", value) is None for value in pins.values())):
        raise Incomplete("Engine KE source/helper inputs differ from the exact fixed set.")
    expected_count = engine_review_input_count(binding["candidate_sha256"])
    if expected_count is not None and binding["source_file_count"] != expected_count:
        raise Incomplete("Engine review KE requires the exact product input count.")
    validate_engine_ke_inventory(binding["test_inventory"], candidate=binding["candidate_sha256"])
    validate_engine_ke_doctest_inventory(binding["doctest_inventory"], candidate=binding["candidate_sha256"])
    for key in ("ordinary_evidence", "root_graph_evidence"):
        evidence = binding[key]
        if (type(evidence) is not dict or set(evidence) != {"path", "sha256"}
                or type(evidence["path"]) is not str or not Path(evidence["path"]).is_absolute()
                or type(evidence["sha256"]) is not str
                or re.fullmatch(r"[a-f0-9]{64}", evidence["sha256"]) is None):
            raise Incomplete("Engine KE prior evidence must have exact external custody.")
    if campaign and (not policy["campaigns_admitted"] or not policy["mutation_inventories"]):
        raise Incomplete("Engine KE mutation is held until the complete enumeration is reviewed and rebound.")
    return binding


def validate_engine_ke_execution(text: str, binding: dict, *, include_doctests: bool = False) -> dict:
    """Verify exact ordinary execution, including the complete Rustdoc roster for mutation."""
    candidate = binding.get("candidate_sha256")
    ordinary = validate_engine_ke_inventory(binding["test_inventory"], candidate=candidate)
    docs = (validate_engine_ke_doctest_inventory(binding.get("doctest_inventory"),
                                                 candidate=binding.get("candidate_sha256"))
            if include_doctests else [])
    records = read_successful_address_records(text)
    if sorted(records) != sorted([*ordinary, *docs]):
        raise Incomplete("Engine KE execution differs from the exact admitted ordinary and selected Rustdoc identities.")
    result = {"tests_executed": len(records), "ke_tests_executed": engine_ke_case_counts(candidate)[1],
              "test_names": records, "inventory_sha256": COMMON.canonical_sha256(sorted(records))}
    if include_doctests:
        result["additional_doctests"] = docs
    return result


def validate_engine_ke_doctests(text: str) -> dict:
    """Keep Rustdoc shape and compilation-only observations for all receipt readers."""
    records = read_successful_address_records(text)
    if any(ENGINE_KE_DOCTEST_NAME.fullmatch(name) is None for name in records):
        raise Incomplete("Engine KE doctest lane contains an unknown non-Rustdoc case.")
    return {"tests_executed": len(records), "test_names": records,
            "compilation_only": [name for name in records if name.endswith((" - compile", " - compile fail"))]}


def validate_engine_ke_current_doctests(text: str, binding: dict) -> dict:
    """Require actual Rustdoc observations for every exact live annotated identity."""
    executed = validate_engine_ke_doctests(text)
    names = validate_engine_ke_doctest_inventory(binding.get("doctest_inventory"),
                                                 candidate=binding.get("candidate_sha256"))
    if sorted(executed["test_names"]) != names:
        raise Incomplete("Engine KE Rustdoc execution differs from the exact admitted roster.")
    return executed


def engine_ke_development_ordinary_cases(text: str) -> list[dict[str, str]]:
    """Recover the development producer's package, actual binary and case coordinates."""
    cases = []
    binary = None
    for line in text.splitlines():
        runnable = re.fullmatch(r"\s+Running (.+?) \(/work/target/[^()]+\)", line)
        if runnable is not None:
            binary = runnable[1]
        elif (entered := re.fullmatch(r"test (.+?) \.\.\. ok", line)) is not None:
            if binary is None:
                raise Incomplete("Engine KE development case has no actual Cargo test binary.")
            cases.append({"package": "nevrotick-engine", "binary": binary, "name": entered[1]})
    return cases


def admit_engine_ke_evidence(binding: dict) -> dict:
    """Readmit the current development receipt and its required physical EP1 logs."""
    evidence = binding["ordinary_evidence"]
    candidate = binding["candidate_sha256"]
    source_scale = candidate == ENGINE_KE_SOURCE_SCALE_CANDIDATE
    review = engine_review_input_count(candidate) is not None
    terminal_pin = engine_review_terminal_pin(candidate)
    if review and evidence["sha256"] != terminal_pin:
        raise Incomplete("Engine review requires the exact combined terminal receipt.")
    if source_scale and evidence["sha256"] != ENGINE_KE_SOURCE_SCALE_TERMINAL_SHA256:
        raise Incomplete("Engine KE source-scale requires the exact combined terminal receipt.")
    path = Path(evidence["path"])
    receipt = read_json(path, maximum=8 * 1024**2, expected_sha256=evidence["sha256"],
                        owned_evidence=True)
    identity = receipt.get("inputs")
    lanes = receipt.get("lanes")
    if (receipt.get("status") != "passed" or receipt.get("mode") != "combined"
            or receipt.get("candidate_sha256") != binding["candidate_sha256"]
            or type(identity) is not dict or len(identity) != binding["source_file_count"]
            or COMMON.canonical_sha256(identity) != binding["candidate_sha256"]
            or type(lanes) is not list or any(type(row) is not dict for row in lanes)):
        raise Incomplete("Engine KE development reference differs from the actual admitted candidate or receipt.")
    result = {"ordinary_evidence": evidence["sha256"]}
    cargo = ["cargo", "test", "--package", "nevrotick-engine", "--frozen", "--jobs", "6",
             "--features", "ke-association-prototype"]
    required = [
        ("ep1-ordinary", ["--lib", "--tests"], validate_engine_ke_execution),
        ("ep1-doctests", ["--doc"], validate_engine_ke_current_doctests),
    ]
    if source_scale or review:
        required.append(("ep1-optimized", ["--release", "--lib", "--tests"], validate_engine_ke_execution))
    for lane, selectors, validate in required:
        rows = [row for row in lanes if row.get("lane") == lane]
        if len(rows) != 1:
            raise Incomplete("Engine KE development reference requires one physical row per selected EP1 lane.")
        row = rows[0]
        if (set(row) != {"lane", "sha256", "exit_code", "failure", "tests_executed", "cases"}
                or type(row["exit_code"]) is not int or row["exit_code"] != 0
                or row["failure"] is not None or type(row["tests_executed"]) is not int
                or type(row["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", row["sha256"]) is None):
            raise Incomplete("Engine KE development EP1 lane lacks complete successful execution identity.")
        raw = read_json(path.with_name(lane + ".json"), maximum=COMMON.ANALYSIS_JSON_MAX_BYTES,
                        expected_sha256=row["sha256"], owned_evidence=True)
        command = [*cargo, *selectors, "--", "--test-threads=1"]
        if (raw.get("command") != command or type(raw.get("exit_code")) is not int or raw["exit_code"] != 0
                or "failure" not in raw or raw["failure"] is not None or type(raw.get("output")) is not str):
            raise Incomplete("Engine KE development raw lane differs from its fixed successful EP1 command.")
        execution = validate(raw["output"], binding)
        cases = (execution["test_names"] if lane == "ep1-doctests"
                 else engine_ke_development_ordinary_cases(raw["output"]))
        if row["tests_executed"] != execution["tests_executed"] or row["cases"] != cases:
            raise Incomplete("Engine KE development EP1 row differs from the physical executed identities.")
        if (source_scale or review) and lane != "ep1-doctests":
            pairs = sorted((case["binary"], case["name"]) for case in cases)
            if candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE:
                digest = ENGINE_JOINED_RECEIVING_BOUNDARY_EP1_CASE_DIGEST
            elif candidate == ENGINE_JOINED_RECEIVING_CANDIDATE:
                digest = ENGINE_JOINED_RECEIVING_CASE_DIGESTS["ep1"]
            elif candidate == ENGINE_STALE_DEBIT_CANDIDATE:
                digest = ENGINE_STALE_DEBIT_EP1_CASE_DIGEST
            elif candidate == ENGINE_FINAL_FRAME_CANDIDATE:
                digest = ENGINE_FINAL_FRAME_EP1_CASE_DIGEST
            elif candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE:
                digest = ENGINE_PROTECTED_DEBIT_EP1_CASE_DIGEST
            elif candidate == ENGINE_PLACEMENT_CANDIDATE:
                digest = ENGINE_PLACEMENT_EP1_CASE_DIGEST
            elif candidate == ENGINE_SCALE_CONFLICT_CANDIDATE:
                digest = ENGINE_SCALE_CONFLICT_EP1_CASE_DIGEST
            else:
                digest = ENGINE_REVIEW_EP1_CASE_DIGEST if review else ENGINE_KE_SOURCE_SCALE_CASE_DIGEST
            if COMMON.canonical_sha256(pairs) != digest:
                raise Incomplete("Engine KE executed binary and case identities differ.")
        result[lane] = row["sha256"]
    graph = binding["root_graph_evidence"]
    graph_path = Path(graph["path"])
    if review:
        capture, reference, recipe, boundary = read_combined_development(candidate, identity, evidence)
        graph_key: str = ("root_graph_evidence" if candidate in ENGINE_JOINED_RECEIVING_FAMILY
                          else "predecessor_graph")
        if graph != capture[graph_key]:
            raise Incomplete(
                "Engine review KE binding names a different dependency graph."
                if candidate in ENGINE_JOINED_RECEIVING_FAMILY
                else "Engine review KE binding names a different predecessor graph.")
        if (candidate in ENGINE_JOINED_RECEIVING_FAMILY
                and binding["source_context_sha256"] != COMMON.canonical_sha256(capture["source"])):
            raise Incomplete("Joined receiving KE binding names a different frozen checkout.")
        result.update(admit_engine_combined_review_lineage(capture, reference, identity, path))
        ordinary, = (row["cases"] for row in lanes if row["lane"] == "default-ordinary")
        doctests, = (row["cases"] for row in lanes if row["lane"] == "default-doctests")
        default_binding = {"candidate_sha256": candidate,
                           "test_inventory": sorted(case["name"] for case in ordinary),
                           "doctest_inventory": sorted(doctests)}
        sandbox_prefix = boundary["sandbox_command"][:-len(boundary["command"])-1]
        result.update(admit_source_policy_capture_rows(path, lanes, recipe, default_binding, sandbox_prefix))
        result[str(path.parents[2] / "admission.json")] = reference["capture_sha256"]
        result["root_graph_evidence"] = graph["sha256"]
    elif source_scale:
        admit_engine_ke_source_scale_graph(graph_path, graph["sha256"], identity)
        if receipt.get("predecessor_graph") != graph:
            raise Incomplete("Engine KE source-scale terminal names a different predecessor graph.")
        result["root_graph_evidence"] = ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256
    else:
        raw = COMMON.read_analysis_bytes(graph_path, maximum=COMMON.ANALYSIS_JSON_MAX_BYTES)
        result["root_graph_evidence"] = hashlib.sha256(raw).hexdigest()
        if result["root_graph_evidence"] != graph["sha256"]:
            raise Incomplete("Engine KE root graph reference bytes changed.")
    return result


def admit_engine_ke_source_scale_graph(path: Path, pin: str, current: dict) -> None:
    """Keep the original graph receipt and check its exact four-Rust-file reuse boundary."""
    if pin != ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256:
        raise Incomplete("Engine KE source-scale requires the exact predecessor graph receipt.")
    graph = read_json(path, maximum=COMMON.ANALYSIS_JSON_MAX_BYTES, expected_sha256=pin, owned_evidence=True)
    previous = graph.get("product_input_sha256")
    lanes = graph.get("lanes")
    if (graph.get("status") != "passed" or graph.get("changed_tree_outputs") != []
            or graph.get("product_digest") != ENGINE_KE_SOURCE_SCALE_GRAPH_PREDECESSOR
            or type(previous) is not dict or previous.keys() != current.keys()
            or COMMON.canonical_sha256(previous) != ENGINE_KE_SOURCE_SCALE_GRAPH_PREDECESSOR
            or "Cargo.lock" not in current or graph.get("lockfile_sha256") != current["Cargo.lock"]
            or {name for name in current if current[name] != previous[name]} != ENGINE_KE_SOURCE_SCALE_GRAPH_DELTA
            or type(lanes) is not list or len(lanes) != 45
            or any(type(row) is not dict or type(row.get("exit_code")) is not int
                   or row["exit_code"] != 0 for row in lanes)):
        raise Incomplete("Engine KE source-scale predecessor graph or unchanged-input reuse boundary differs.")


def engine_ke_provider_contract_examples(original: bytes) -> list[dict]:
    """Append six real private-constructor callers; compile them without execution.

    The unchanged UTF-8 owner is bounded and newline-terminated. Each specimen
    changes only its appended caller; genuine admissions mint every claim.
    """
    if (type(original) is not bytes or not original or len(original) > 256 * 1024
            or not original.endswith(b"\n") or b"mod provider_claim_contract_probe" in original):
        raise Incomplete("Provider compiler contracts require the unchanged bounded private owner.")
    try:
        original_text = original.decode("utf-8")
    except UnicodeDecodeError as error:
        raise Incomplete("Provider compiler owner is not bounded UTF-8 source.") from error
    examples = []
    for target, constructor, cfg in (("gnu", "component", "not(windows)"), ("msvc", "new", "windows")):
        admission = ("let admission = super::ProcessAdmission::new();\n        let claim = admission.claim()?;"
                     if target == "gnu" else "let claim = super::super::PROCESS_ADMISSION.claim()?;")
        for case in ("once", "overlap", "after-drop"):
            second = f"        let second = ProviderScope::{constructor}(claim);"
            calls = f"        let first = ProviderScope::{constructor}(claim);\n"
            if case == "once":
                calls += "        drop(first);\n"
            elif case == "overlap":
                calls += second + "\n        drop((first, second));\n"
            else:
                calls += "        drop(first);\n" + second + "\n        drop(second);\n"
            probe = ("\n/// Actual private constructors; compile-only claim consumption control.\n"
                f"#[cfg(all(test, {cfg}))]\nmod provider_claim_contract_probe {{\n"
                "    use super::{Failure, ProviderScope};\n"
                "    /// Use genuine claims without executing the provider or its destructor.\n"
                "    /// # Errors\n    /// Preserve actual initial claim refusal.\n"
                "    #[test]\n    fn actual_owned_claim_contract_probe() -> Result<(), Failure> {\n"
                f"        {admission}\n{calls}        Ok(())\n    }}\n}}\n")
            diagnostic = None if case == "once" else {
                "code": "E0382", "tokens": ["claim", "moved"],
                "line": len(original_text.splitlines()) + probe.splitlines().index(second) + 1}
            examples.append({"target": target, "case": case, "constructor": "ProviderScope::" + constructor,
                "appended_source": probe, "source": original + probe.encode(), "diagnostic": diagnostic})
    return examples


ENGINE_KE_HISTORY_CAUSES = {
    "copy-completion": ("E0277", ("InvocationCompletion", "Copy")),
    "clone-completion": ("E0277", ("InvocationCompletion", "Clone")),
    "copy-history": ("E0277", ("ProtectedHistory", "Copy")),
    "clone-history": ("E0277", ("ProtectedHistory", "Clone")),
    "second-consume-completion": ("E0382", ("completion", "moved")),
    "raw-history": ("E0451", ("ProtectedHistory", "summary", "receipt", "population",
        "send_before", "send_after", "read_return", "local", "remote", "identity", "run_context", "private")),
    "raw-receiving": ("E0451", ("joined", "private")),
    "mutate-receiving": ("E0616", ("joined", "private")),
    "clone-receiving": ("E0277", ("Clone",)),
    "second-consume-receiving": ("E0382", ("receiving", "moved")),
    "read-received-history": ("E0616", ("history", "private")),
    "read-received-measurement": ("E0616", ("measurement", "private")),
    "mutate-received-profile": ("E0616", ("profile", "private")),
    "clone-received-numerics": ("E0277", ("Clone",)),
}

ENGINE_KE_HISTORY_CASES = (
    "legal", "copy-completion", "clone-completion", "copy-history", "clone-history",
    "second-consume-completion", "raw-history",
    "legal-receiving", "raw-receiving", "mutate-receiving", "clone-receiving",
    "second-consume-receiving",
    "legal-received", "read-received-history", "read-received-measurement",
    "mutate-received-profile", "clone-received-numerics",
)

ENGINE_KE_PROJECTION_CAUSES = {
    "projection-parent-preview": ("E0599", ("ReceivedCompletion", "Preview")),
    "projection-parent-inner-mut": ("E0616", ("inner", "private")),
    "projection-parent-literal": ("E0451", ("completion", "private")),
    "projection-parent-field": ("E0616", ("completion", "private")),
    "projection-parent-clone": ("E0277", ("CompletionProjection", "Clone")),
    "projection-parent-escape": ("E0515", ("owned", "return")),
    "projection-sibling-preview": ("E0599", ("ReceivedCompletion", "Preview")),
}
ENGINE_KE_PROJECTION_CASES = (
    "projection-parent-legal", "projection-parent-preview", "projection-parent-inner-mut",
    "projection-parent-literal", "projection-parent-field", "projection-parent-clone",
    "projection-parent-escape", "projection-sibling-legal", "projection-sibling-preview",
)


def engine_ke_history_contract_examples(original: bytes) -> list[dict]:
    """Append thirty-four existing consumers and nine closed production seal controls."""
    if (type(original) is not bytes or not original or len(original) > 256 * 1024
            or not original.endswith(b"\n") or b"protected_history_contract_probe" in original):
        raise Incomplete("History compiler contracts require the unchanged bounded private owner.")
    try:
        original_text = original.decode("utf-8")
    except UnicodeDecodeError as error:
        raise Incomplete("History compiler owner is not bounded UTF-8 source.") from error
    bodies = {
        "legal": ("", None),
        "copy-completion": ("    fn require_copy<T: Copy>() {}\n    require_copy::<InvocationCompletion>();\n",
            "    require_copy::<InvocationCompletion>();"),
        "clone-completion": ("    let duplicate = <InvocationCompletion as Clone>::clone(&completion);\n    drop(duplicate);\n",
            "    let duplicate = <InvocationCompletion as Clone>::clone(&completion);"),
        "copy-history": ("    fn require_copy<T: Copy>() {}\n    require_copy::<datagram::ProtectedHistory>();\n",
            "    require_copy::<datagram::ProtectedHistory>();"),
        "clone-history": ("    let duplicate = <datagram::ProtectedHistory as Clone>::clone(&history);\n    drop(duplicate);\n",
            "    let duplicate = <datagram::ProtectedHistory as Clone>::clone(&history);"),
        "second-consume-completion": ("    let first = completion;\n    let second = completion;\n    drop(second);\n",
            "    let second = completion;"),
    }
    # All ten private fields stay on one physical line: every E0451 primary span
    # must identify this same independent expected line in the unchanged matcher.
    raw = ("    let duplicate = datagram::ProtectedHistory { summary: *history.summary(), receipt: history.receipt(), "
        "population: history.population(), send_before, send_after, read_return, local, remote, "
        "identity: Arc::clone(history.identity()), run_context: Arc::clone(history.run_context()) };")
    bodies["raw-history"] = ("    let [send_before, send_after, read_return] = history.captures();\n"
        "    let (local, remote) = history.endpoints();\n" + raw + "\n    drop(duplicate);\n", raw)
    examples = []
    for target, cfg in (("gnu", "not(windows)"), ("msvc", "windows")):
        for case, (body, causal_line) in bodies.items():
            returned = "first" if case == "second-consume-completion" else "completion"
            probe = ("\n/// protected_history_contract_probe: compile-only actual owning consumers.\n"
                f"#[cfg(all(test, {cfg}))]\n"
                "const _: fn(InvocationCompletion, datagram::ProtectedHistory)\n"
                "    -> (InvocationCompletion, datagram::ProtectedHistory) = |completion, history| {\n"
                "    {\n        let _ = (completion.summary(), history.summary(), history.receipt(),\n"
                "            history.population(), history.captures(), history.endpoints(),\n"
                "            history.identity(), history.run_context());\n    }\n"
                + body + f"    ({returned}, history)\n}};\n")
            diagnostic = None
            if causal_line is not None:
                code, tokens = ENGINE_KE_HISTORY_CAUSES[case]
                diagnostic = {"code": code, "tokens": list(tokens),
                    "line": len(original_text.splitlines()) + probe.splitlines().index(causal_line) + 1}
            examples.append({"target": target, "case": case, "owner": "InvocationCompletion/ProtectedHistory",
                "appended_source": probe, "source": original + probe.encode(), "diagnostic": diagnostic})

        # Legal receiving reaches the actual collected take and both consuming APIs.
        # No raw JoinedOutcome flags or invented protected history mint its token.
        receiving_bodies = {
            "legal-receiving": (
                "const _: fn(&job::WorkerSlot, Option<crate::ExperimentalProfile>)\n"
                "    -> Result<datagram::ReceivedCompletion, RunFailed> = |slot, profile| {\n"
                "    slot.collect().map_err(RunFailed::Collection)?;\n"
                "    let receiving = slot.take_for_receiving().map_err(RunFailed::Collection)?;\n"
                "    datagram::receive_joined(receiving, profile)\n};\n"
                f"#[cfg(all(test, {cfg}))]\n"
                "const _: fn(job::ReceivingOutcome) -> Result<job::Operation, job::JoinedOutcome>\n"
                "    = |receiving| receiving.into_normal();\n", None),
            "raw-receiving": (
                "const _: fn(job::JoinedOutcome) -> job::ReceivingOutcome = |joined| {\n"
                "    let receiving = job::ReceivingOutcome { joined };\n"
                "    receiving\n};\n",
                "    let receiving = job::ReceivingOutcome { joined };"),
            "mutate-receiving": (
                "const _: fn(&mut job::ReceivingOutcome) = |receiving| {\n"
                "    receiving.joined.retired = true;\n};\n",
                "    receiving.joined.retired = true;"),
            "clone-receiving": (
                "const _: fn(job::ReceivingOutcome) -> job::ReceivingOutcome = |receiving| {\n"
                "    fn require_clone<T: Clone>(_: &T) {}\n"
                "    require_clone(&receiving);\n    receiving\n};\n",
                "    require_clone(&receiving);"),
            "second-consume-receiving": (
                "const _: fn(job::ReceivingOutcome) = |receiving| {\n"
                "    let first = receiving.into_normal();\n"
                "    let second = receiving.into_normal();\n    drop((first, second));\n};\n",
                "    let second = receiving.into_normal();"),
        }
        received_bodies = {
            "legal-received": ("    let _ = (received.history().summary(), received.profile(), received.measurement());\n", None),
            "read-received-history": ("    let _ = &received.history;\n", "    let _ = &received.history;"),
            "read-received-measurement": ("    let _ = &received.measurement;\n", "    let _ = &received.measurement;"),
            "mutate-received-profile": ("    received.profile = None;\n", "    received.profile = None;"),
            "clone-received-numerics": (
                "    fn require_clone<T: Clone>(_: &T) {}\n    require_clone(received);\n",
                "    require_clone(received);"),
        }
        for case, (body, causal_line) in receiving_bodies.items():
            probe = ("\n/// protected_history_contract_probe: actual collected receiving custody.\n"
                f"#[cfg(all(test, {cfg}))]\n" + body)
            diagnostic = None
            if causal_line is not None:
                code, tokens = ENGINE_KE_HISTORY_CAUSES[case]
                diagnostic = {"code": code, "tokens": list(tokens),
                    "line": len(original_text.splitlines()) + probe.splitlines().index(causal_line) + 1}
            examples.append({"target": target, "case": case, "owner": "job::ReceivingOutcome",
                "appended_source": probe, "source": original + probe.encode(), "diagnostic": diagnostic})
        for case, (body, causal_line) in received_bodies.items():
            prefix = ("\n/// protected_history_contract_probe: inferred actual received carrier.\n"
                f"#[cfg(all(test, {cfg}))]\n")
            probe = (prefix + "const _: fn(&datagram::ReceivedCompletion) = |completion| {\n"
                "    if let Some(received) = completion.protected() {\n" + body + "    }\n};\n")
            diagnostic = None
            if causal_line is not None:
                code, tokens = ENGINE_KE_HISTORY_CAUSES[case]
                diagnostic = {"code": code, "tokens": list(tokens),
                    "line": len(original_text.splitlines()) + probe.splitlines().index(causal_line) + 1}
            examples.append({"target": target, "case": case, "owner": "ReceivedCompletion borrowed protected numerics",
                "appended_source": probe, "source": original + probe.encode(), "diagnostic": diagnostic})
    examples.extend(_engine_ke_projection_contract_examples(original, original_text))
    return examples


def _engine_ke_projection_contract_examples(original: bytes, original_text: str) -> list[dict]:
    """Use the actual Windows production parent and its child, never an outer import failure."""
    legal = (
        "const _: fn(&job::WorkerSlot) -> Result<datagram::ReceivedCompletion, RunFailed> = |slot| {\n"
        "    slot.collect().map_err(RunFailed::Collection)?;\n"
        "    datagram::receive_joined(slot.take_for_receiving().map_err(RunFailed::Collection)?, None)\n};\n"
        "const _: for<'a> fn(&'a datagram::ReceivedCompletion) -> datagram::CompletionProjection<'a> = |completion| {\n"
        "    let projection = completion.projection();\n"
        "    let _ = (projection.summary(), projection.numerical(), projection.numerical_reason());\n"
        "    projection\n};\n")
    preview = (
        "const _: fn(Summary) -> datagram::ReceivedCompletion = |summary| {\n"
        "    datagram::ReceivedCompletion::Preview(summary)\n};\n",
        "    datagram::ReceivedCompletion::Preview(summary)")
    bodies = {
        "projection-parent-legal": (legal, None),
        "projection-parent-preview": preview,
        "projection-parent-inner-mut": (
            "const _: fn(&mut datagram::ReceivedCompletion) = |completion| {\n"
            "    let _inner = &mut completion.inner;\n};\n", "    let _inner = &mut completion.inner;"),
        "projection-parent-literal": (
            "const _: for<'a> fn(&'a datagram::ReceivedCompletion) -> datagram::CompletionProjection<'a> = |completion| {\n"
            "    datagram::CompletionProjection { completion }\n};\n", "    datagram::CompletionProjection { completion }"),
        "projection-parent-field": (
            "const _: fn(&datagram::CompletionProjection<'_>) = |projection| {\n"
            "    let _ = &projection.completion;\n};\n", "    let _ = &projection.completion;"),
        "projection-parent-clone": (
            "const _: fn(datagram::CompletionProjection<'_>) = |projection| {\n"
            "    fn require_clone<T: Clone>(_: &T) {}\n"
            "    require_clone(&projection);\n};\n", "    require_clone(&projection);"),
        "projection-parent-escape": (
            "const _: for<'a> fn(&'a datagram::ReceivedCompletion, datagram::ReceivedCompletion)\n"
            "    -> datagram::CompletionProjection<'a> = |_borrowed, owned| {\n"
            "    owned.projection()\n};\n", "    owned.projection()"),
        "projection-sibling-legal": (legal, None),
        "projection-sibling-preview": preview,
    }
    result = []
    for case in ENGINE_KE_PROJECTION_CASES:
        body, causal = bodies[case]
        sibling = case.startswith("projection-sibling-")
        if sibling:
            def qualify(text):
                return (text.replace("datagram::", "super::datagram::").replace("job::", "super::job::")
                    .replace("RunFailed", "super::RunFailed").replace("Summary", "super::Summary"))
            body = qualify(body)
            if causal is not None:
                causal = qualify(causal)
        probe = "\n/// protected_history_contract_probe: sealed joined production projection.\n#[cfg(windows)]\n"
        probe += ("mod joined_projection_contract_probe_sibling {\n" + body + "}\n" if sibling else body)
        diagnostic = None
        if causal is not None:
            code, tokens = ENGINE_KE_PROJECTION_CAUSES[case]
            diagnostic = {"code": code, "tokens": list(tokens),
                "line": len(original_text.splitlines()) + probe.splitlines().index(causal) + 1}
        result.append({"target": "msvc", "case": case,
            "owner": "ReceivedCompletion/CompletionProjection production " + ("sibling" if sibling else "parent"),
            "appended_source": probe, "source": original + probe.encode(), "diagnostic": diagnostic})
    return result


def validate_engine_ke_provider_cause(result: dict, specimen: dict, manifest: str) -> None:
    """Preserve the fixed provider-only E0382 cause and actual process source."""
    if (type(specimen) is not dict or set(specimen) != {"code", "tokens", "line"}
            or specimen["code"] != "E0382" or specimen["tokens"] != ["claim", "moved"]):
        raise Incomplete("Provider ownership probe requires its exact moved-claim diagnostic.")
    _validate_engine_ke_compiler_cause(result, specimen, manifest, ENGINE_KE_HOST_SUBJECTS["ke_process"])


def validate_engine_ke_history_cause(result: dict, specimen: dict, manifest: str) -> None:
    """Admit only the closed history type/trait/privacy causes in the actual parent owner."""
    if (type(specimen) is not dict or set(specimen) != {"code", "tokens", "line"}
            or not any(specimen["code"] == code and specimen["tokens"] == list(tokens)
                       for code, tokens in ENGINE_KE_HISTORY_CAUSES.values())):
        raise Incomplete("History ownership probe leaves its fixed diagnostic matrix.")
    _validate_engine_ke_compiler_cause(result, specimen, manifest, ENGINE_KE_HOST_SUBJECTS["ke_core"])


def validate_engine_ke_projection_cause(result: dict, specimen: dict, manifest: str) -> None:
    """Require a fixed production seal cause, including actual non-test library attribution."""
    if (type(specimen) is not dict or set(specimen) != {"code", "tokens", "line"}
            or not any(specimen["code"] == code and specimen["tokens"] == list(tokens)
                       for code, tokens in ENGINE_KE_PROJECTION_CAUSES.values())):
        raise Incomplete("Projection ownership probe leaves its fixed production diagnostic matrix.")
    _validate_engine_ke_compiler_cause(result, specimen, manifest, ENGINE_KE_HOST_SUBJECTS["ke_core"], production=True)


def _validate_engine_ke_compiler_cause(result: dict, specimen: dict, manifest: str, filename: str,
                                       *, production: bool = False) -> None:
    """Check actual Cargo attribution before the unchanged rustc causal matcher.

    Cargo's pinned JSON emits an exact terminal explanation note without rustc's
    plain abort summary. Validate and omit only that noncausal tail; never invent
    an abort record or discard another diagnostic. Raw command output is retained.
    """
    if (type(result) is not dict or type(result.get("exit_code")) is not int
            or result["exit_code"] != 101 or result.get("failure") is not None
            or "failure" not in result or type(result.get("output")) is not str
            or not 0 < len(result["output"].encode()) <= 4 * 1024 * 1024
            or type(specimen) is not dict or set(specimen) != {"code", "tokens", "line"}
            or type(specimen["code"]) is not str or re.fullmatch(r"E[0-9]{4}", specimen["code"]) is None
            or type(specimen["tokens"]) is not list or not specimen["tokens"]
            or any(type(token) is not str or not token for token in specimen["tokens"])
            or filename not in (ENGINE_KE_HOST_SUBJECTS["ke_process"], ENGINE_KE_HOST_SUBJECTS["ke_core"])
            or type(specimen["line"]) is not int or specimen["line"] < 1
            or type(manifest) is not str or not manifest.startswith("/work/")
            or ".." in Path(manifest).parts or not manifest.endswith("/crates/nevrotick-engine/Cargo.toml")):
        raise Incomplete("Provider ownership probe lacks its bounded ordinary Cargo compiler refusal.")
    records = result["output"].splitlines()
    if len(records) > 4096:
        raise Incomplete("Provider Cargo diagnostic records exceed their bound.")
    package = "path+file://" + str(Path(manifest).parent) + "#0.0.0"
    diagnostics, summaries = [], []
    explained = finished = False
    for line in records:
        if not line.strip():
            continue
        if not line.startswith("{"):
            summary = re.fullmatch((r"error: could not compile `nevrotick-engine` \(lib\) due to ([1-9][0-9]*) previous errors?"
                if production else r"error: could not compile `nevrotick-engine` \(lib(?: test)?\) due to ([1-9][0-9]*) previous errors?"), line)
            if summary:
                summaries.append(int(summary[1]))
            elif line != "warning: build failed, waiting for other jobs to finish...":
                raise Incomplete("Provider Cargo output contains unexpected non-JSON text.")
            continue
        try:
            item = COMMON.ANALYSIS.decode(line)
        except (Incomplete, RecursionError) as error:
            raise Incomplete("Provider Cargo output contains malformed or excessive JSON.") from error
        if type(item) is not dict or finished:
            raise Incomplete("Provider Cargo record is malformed or follows build completion.")
        reason = item.get("reason")
        if reason == "compiler-message":
            target = item.get("target")
            message = item.get("message")
            if (set(item) != {"reason", "package_id", "manifest_path", "target", "message"}
                    or item["package_id"] != package or item["manifest_path"] != manifest
                    or type(target) is not dict or target.get("name") != "nevrotick_engine"
                    or target.get("kind") != ["lib"] or target.get("crate_types") != ["lib"]
                    or target.get("src_path") != str(Path(manifest).parent / "src/lib.rs")
                    or type(message) is not dict or explained):
                raise Incomplete("Provider compiler diagnostic leaves the actual private Engine library.")
            if message.get("level") == "failure-note":
                expected = f"For more information about this error, try `rustc --explain {specimen['code']}`."
                if (not diagnostics or set(message) != {"$message_type", "message", "code", "level", "spans", "children", "rendered"}
                        or message["$message_type"] != "diagnostic" or message["message"] != expected
                        or message["code"] is not None or message["spans"] != [] or message["children"] != []
                        or message["rendered"] not in (None, expected + "\n")):
                    raise Incomplete("Provider Cargo explanation is not its exact terminal noncausal note.")
                explained = True
            else:
                diagnostics.append(message)
        elif reason == "compiler-artifact":
            if (set(item) != {"reason", "package_id", "manifest_path", "target", "profile", "features", "filenames", "executable", "fresh"}
                    or any(type(item[key]) is not str or not item[key] for key in ("package_id", "manifest_path"))
                    or type(item["target"]) is not dict or type(item["profile"]) is not dict
                    or type(item["fresh"]) is not bool
                    or (item["executable"] is not None and type(item["executable"]) is not str)
                    or any(type(item[key]) is not list or len(item[key]) > 4096
                           or any(type(value) is not str for value in item[key]) for key in ("features", "filenames"))):
                raise Incomplete("Provider Cargo artifact envelope is malformed.")
        elif reason == "build-script-executed":
            if (set(item) != {"reason", "package_id", "linked_libs", "linked_paths", "cfgs", "env", "out_dir"}
                    or any(type(item[key]) is not str or not item[key] for key in ("package_id", "out_dir"))
                    or any(type(item[key]) is not list or len(item[key]) > 4096
                           or any(type(value) is not str for value in item[key]) for key in ("linked_libs", "linked_paths", "cfgs"))
                    or type(item["env"]) is not list or len(item["env"]) > 4096
                    or any(type(pair) is not list or len(pair) != 2
                           or any(type(value) is not str for value in pair) for pair in item["env"])):
                raise Incomplete("Provider Cargo build-script envelope is malformed.")
        elif reason == "build-finished":
            if set(item) != {"reason", "success"} or item["success"] is not False:
                raise Incomplete("Provider Cargo completion contradicts its compiler refusal.")
            finished = True
        else:
            raise Incomplete("Provider Cargo output contains an unknown record.")
    errors = sum(item.get("level") == "error" and item.get("code") is not None for item in diagnostics)
    if errors != 1 or not explained or not finished or len(summaries) != 1 or summaries[0] != errors:
        raise Incomplete("Provider Cargo refusal lacks its exact terminal explanation/completion accounting.")
    normalized = {**result, "output": "\n".join(json.dumps(item, sort_keys=True) for item in diagnostics)}
    COMMON.compiler_cause(normalized, specimen, filename)


def engine_ke_provider_contracts(session, policy: dict, report: dict, run) -> None:
    """Retain exactly the six existing actual provider-claim metadata controls."""
    _engine_ke_owned_contracts(session, policy, report, run, "provider")


def engine_ke_history_contracts(session, policy: dict, report: dict, run) -> None:
    """Run the fixed history, collected-token and conditional-carrier metadata controls."""
    _engine_ke_owned_contracts(session, policy, report, run, "history")


def _engine_ke_owned_contracts(session, policy: dict, report: dict, run, owner: str) -> None:
    """Seal one private package copy per command with exactly one appended probe.

    The Session source and original checkout remain immutable. Expected island
    identities derive from the admitted base plus the single known byte delta;
    source restoration is mandatory even when compilation or custody fails.
    """
    if owner == "provider":
        filename, examples_for, validate = (ENGINE_KE_HOST_SUBJECTS["ke_process"],
            engine_ke_provider_contract_examples, validate_engine_ke_provider_cause)
        report_key, requirement, cases = "provider_claim_contracts", "KE-PROV-02", ("once", "overlap", "after-drop")
        scope = "Actual private component/Native constructors; two healthy and four moved-value metadata checks; no Windows execution."
        owner_field = "constructor"
    elif owner == "history":
        filename, examples_for, validate = (ENGINE_KE_HOST_SUBJECTS["ke_core"],
            engine_ke_history_contract_examples, validate_engine_ke_history_cause)
        report_key, requirement, cases = "protected_history_contracts", "KE-HIST-03/05/07", ENGINE_KE_HISTORY_CASES
        scope = "Actual history/receiving/carrier contracts: eight healthy and thirty-five ownership/privacy metadata refusals, including nine Windows production parent/sibling seal controls; no Windows execution."
        owner_field = "owner"
    else:
        raise Incomplete("Engine KE compiler controls require a fixed provider or history owner.")
    deadline = time.monotonic() + policy["baseline_wall_seconds"]
    private = session.work / (owner + "-contract-source")
    session.check()
    baseline = dict(session.source_inventory)
    original = COMMON.read_analysis_bytes(session.source / filename, maximum=256 * 1024)
    if hashlib.sha256(original).hexdigest() != baseline[filename]:
        raise Incomplete("Provider compiler owner differs from its immutable Session identity.")
    examples = examples_for(original)
    expected_cases = [(target, case) for target in ("gnu", "msvc") for case in cases]
    if owner == "history":
        expected_cases += [("msvc", case) for case in ENGINE_KE_PROJECTION_CASES]
    if [(row["target"], row["case"]) for row in examples] != expected_cases:
        raise Incomplete("Engine KE compiler controls differ from their complete fixed roster.")
    shutil.copytree(session.source, private)
    if PRODUCT.inventory(private) != baseline:
        raise Incomplete("Provider compiler package copy differs from the immutable Session source.")
    session.protected_islands[private] = dict(baseline)
    production = private / filename
    manifest = "/work/" + (private / "crates/nevrotick-engine/Cargo.toml").relative_to(session.work).as_posix()
    report[report_key] = {"requirement": requirement, "probes": [], "scope": scope}
    try:
        for example in examples:
            session.check()
            if PRODUCT.inventory(private) != baseline or session.protected_islands.get(private) != baseline:
                raise Incomplete("Provider compiler copy drifted before its next controlled delta.")
            data = example["source"]
            expected = {**baseline, filename: hashlib.sha256(data).hexdigest()}
            production.write_bytes(data)
            session.protected_islands[private] = dict(expected)
            try:
                projection = owner == "history" and example["case"] in ENGINE_KE_PROJECTION_CASES
                command = ["cargo", "check", "--package", "nevrotick-engine", "--features", "ke-association-prototype",
                    "--lib" if projection else "--all-targets", "--frozen", "--jobs", "6", "--message-format=json", "--quiet"]
                if example["target"] == "msvc":
                    command += ["--target", "x86_64-pc-windows-msvc"]
                label = "ke-" + owner + "-contract-" + example["target"] + "-" + example["case"]
                negative = example["diagnostic"] is not None
                raw = run(label, command, COMMON.remaining_timeout(deadline, policy["baseline_wall_seconds"]),
                    cwd=private, **({"negative": json.dumps(example["diagnostic"]["code"])} if negative else {}))
                if negative or owner == "history":
                    lanes = report.get("lanes")
                    if type(lanes) is not list or not lanes or type(lanes[-1]) is not dict:
                        raise Incomplete("Provider compiler refusal lacks its retained Session result.")
                    row = lanes[-1]
                    argv = row.get("command")
                    if (row.get("lane") != label or type(raw) is not str or row.get("output") != raw
                            or type(argv) is not list or len(argv) <= len(command)
                            or argv[-len(command) - 1] != "--" or argv[-len(command):] != command):
                        raise Incomplete("Provider diagnostic differs from its retained actual Session command.")
                    if (not negative and (type(row.get("exit_code")) is not int or row["exit_code"] != 0
                            or "failure" not in row or row["failure"] is not None)):
                        raise Incomplete("History legal consumer lacks its retained successful ordinary command.")
                if negative:
                    (validate_engine_ke_projection_cause if projection else validate)(row, example["diagnostic"], manifest)
                session.check()
                if PRODUCT.inventory(private) != expected or session.protected_islands.get(private) != expected:
                    raise Incomplete("Provider compiler copy changed outside its admitted single-source delta.")
                report[report_key]["probes"].append({
                    key: example[key] for key in ("target", "case", owner_field, "appended_source", "diagnostic")})
                report[report_key]["probes"][-1].update(
                    source_sha256=expected[filename], appended_sha256=hashlib.sha256(example["appended_source"].encode()).hexdigest())
                if projection:
                    report[report_key]["probes"][-1]["compile_scope"] = "Windows production library, no cfg(test)"
            finally:
                production.write_bytes(original)
                session.protected_islands[private] = dict(baseline)
                if PRODUCT.inventory(private) != baseline:
                    raise Incomplete("Provider compiler package was not restored exactly after its probe.")
    finally:
        production.write_bytes(original)
        session.protected_islands[private] = dict(baseline)
        session.check()


def execute_engine_ke_profile(args, report: dict) -> None:
    """Run only the admitted Root-lock feature, with explicit initial campaign hold."""
    if (getattr(args, "profile", None) != "ke_association" or args.tool_admission_profile != "stable-core"
            or args.lane not in {"contracts", "baseline", "enumerate", "coverage", "mutation", "all"}
            or (args.lane == "contracts" and args.subject not in ("ke_process", "ke_job"))):
        raise Incomplete("Engine KE requires the selected stable host analysis lane.")
    policy = read_json(ROOT / ENGINE_KE_POLICY)
    binding = validate_engine_ke_policy(policy, campaign=args.lane in {"mutation", "all"})
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Engine KE metrics require seven GiB before admission.")
    subjects = ENGINE_KE_SCOPE.select(args.subject)
    dependency, identity = PRODUCT.admit_product(ROOT)
    try:
        PRODUCT.SLINT.profile_rows(dependency, PRODUCT.SLINT.EP1_PROFILE)
    except PRODUCT.SLINT.Incomplete as error:
        raise Incomplete(str(error)) from error
    if (len(identity) != binding["source_file_count"]
            or COMMON.canonical_sha256(identity) != binding["candidate_sha256"]
            or any(PRODUCT.sha256(ROOT / name) != value for name, value in binding["inputs_sha256"].items())):
        raise Incomplete("Engine KE actual Root source/helper/graph candidate differs.")
    manifest = tomllib.loads((ROOT / "policy/verification-manifest.toml").read_text())
    for filename in COMMON.ENGINE_KE_CRITICAL:
        package = "nevrotick-crypto-cng-lifecycle" if filename.startswith("tools/") else "nevrotick-engine"
        owners: list[str | None] = [
            row.get("package") for row in manifest.get("security_modules", [])
            for member in row.get("files", []) if member == filename
        ]
        if (manifest.get("critical_files", []).count(filename) != 1
                or owners != [package]):
            raise Incomplete("Engine KE requires the complete current critical/security inventory.")
    evidence_before = admit_engine_ke_evidence(binding)
    selected = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        selected.lane = "mutation"
    report.update(profile="ke_association", subjects=subjects, qualification_passed=False,
                  full_critical_files=list(COMMON.ENGINE_KE_CRITICAL),
                  scope="Selected registered GNU KE owners; remaining file and Windows-arm evidence stays open",
                  source_context_sha256=binding["source_context_sha256"],
                  source_context_scope=(
                      "Exact complete frozen checkout used by the combined producer."
                      if binding["candidate_sha256"] in ENGINE_JOINED_RECEIVING_FAMILY else
                      "Historical Tool preparation reference; current Root identity is candidate_sha256/source_sha256"),
                  candidate_sha256=binding["candidate_sha256"],
                  ordinary_reference=binding["ordinary_evidence"],
                  ordinary_reference_scope=(
                      "Exact current combined terminal and graph; physical EP1 ordinary, optimized and Rustdoc rosters."
                      if binding["candidate_sha256"] in ENGINE_JOINED_RECEIVING_FAMILY else
                      "Exact combined terminal and predecessor graph; physical EP1 ordinary, optimized and Rustdoc rosters."
                      if binding["candidate_sha256"] in (
                          ENGINE_KE_SOURCE_SCALE_CANDIDATE, ENGINE_REVIEW_CANDIDATE,
                          ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE,
                          ENGINE_PROTECTED_DEBIT_CANDIDATE, ENGINE_FINAL_FRAME_CANDIDATE, ENGINE_STALE_DEBIT_CANDIDATE) else
                      "Exact current development receipt and physical EP1 ordinary/Rustdoc rosters; metrics require their own actual execution."),
                  pending_metric_owners=[name for name in COMMON.ENGINE_KE_CRITICAL if name not in ENGINE_KE_HOST_SUBJECTS.values()],
                  pending_windows="StrictProfile::capture, transport, provider shutdown and Windows-only control/error arms",
                  generation_gap_policy="Reconcile every constructor, generic helper, Garde expansion and atomic guard against actual descriptors; no manual-control kill credit.")
    if args.lane == "contracts":
        report.update(contracts_qualification_passed=False, provider_claim_contracts_passed=False,
                      protected_history_contracts_passed=False)
    session = COMMON.Session(ROOT, selected, report, ENGINE_KE_SCOPE, policy,
                             engine_ke_bound_inputs(binding["candidate_sha256"]))
    try:
        if (COMMON.canonical_sha256(session.product_identity) != binding["candidate_sha256"]
                or read_json(session.source / ENGINE_KE_POLICY) != policy
                or any(PRODUCT.sha256(session.source / name) != value
                       for name, value in binding["inputs_sha256"].items())):
            raise Incomplete("Captured Engine KE candidate/policy/helper differs before its first command.")
        def run(label, command, timeout, **options):
            address_storage_checkpoint(session, policy, report, label, "before")
            try:
                result = session.run(label, command, timeout, **options)
                if label.startswith("mutation-campaign:"):
                    subject = label.removeprefix("mutation-campaign:")
                    directory = session.work / "mutation" / subject / "mutants.out"
                    outcomes = COMMON.read_analysis_json(directory / "outcomes.json")
                    baseline = next(row for row in outcomes["outcomes"] if row["scenario"] == "Baseline")
                    relative = COMMON.analysis_output_path(baseline["log_path"])
                    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
                        raise Incomplete("Engine KE baseline log leaves its owned output.")
                    report.setdefault("baseline_execution", {})[subject] = validate_engine_ke_execution(
                        COMMON.read_analysis_text(directory / relative), binding, include_doctests=True)
                return result
            finally:
                address_storage_checkpoint(session, policy, report, label, "after")
        if args.lane == "contracts":
            report["contracts_qualification_passed"] = False
            if args.subject == "ke_process":
                report["scope"] = "KE-PROV-02 actual private GNU/MSVC provider constructors; metadata-only claim consumption"
                report["provider_claim_contracts_passed"] = False
                engine_ke_provider_contracts(session, policy, report, run)
            else:
                report["scope"] = "KE-HIST-03/07 actual private GNU/MSVC completion/history ownership; metadata-only privacy controls"
                report["protected_history_contracts_passed"] = False
                engine_ke_history_contracts(session, policy, report, run)
        if args.lane == "baseline":
            run("ke-format", ["cargo", "fmt", "--all", "--", "--check"], policy["baseline_wall_seconds"])
            # Admit held Windows source before repeating host execution/optimization.
            # Check emits metadata only; its actual artifact records remain available
            # for the separately held external-privacy route. No linking/native run.
            run("ke-windows-check", ["cargo", "check", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--all-targets", "--target", "x86_64-pc-windows-msvc",
                "--frozen", "--jobs", "6", "--message-format=json"], policy["baseline_wall_seconds"])
            raw = run("ke-baseline", ["cargo", "test", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--frozen", "--jobs", "6", "--lib", "--tests",
                "--", "--test-threads=1"], policy["baseline_wall_seconds"])
            report["baseline_execution"] = validate_engine_ke_execution(raw, binding)
            run("ke-clippy", ["cargo", "clippy", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--all-targets", "--frozen", "--jobs", "6",
                "--", "-D", "warnings"], policy["baseline_wall_seconds"])
            retire_engine_ke_debug_target(session, policy, report)
            raw = run("ke-release", ["cargo", "test", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--release", "--frozen", "--jobs", "6", "--lib", "--tests",
                "--", "--test-threads=1"], policy["baseline_wall_seconds"])
            report["release_execution"] = validate_engine_ke_execution(raw, binding)
            retire_engine_ke_release_target(session, policy, report)
            raw = run("ke-doctests", ["cargo", "test", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--frozen", "--jobs", "6", "--doc",
                "--", "--test-threads=1"], policy["baseline_wall_seconds"])
            report["doctest_execution"] = validate_engine_ke_current_doctests(raw, binding)
            run("ke-windows-clippy", ["cargo", "clippy", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--all-targets", "--target", "x86_64-pc-windows-msvc",
                "--frozen", "--jobs", "6", "--", "-D", "warnings"], policy["baseline_wall_seconds"])
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(ENGINE_KE_SCOPE, subjects, policy, session.tools,
                run, report, deadline=time.monotonic() + policy["enumeration_wall_seconds"],
                maximum_per_file=512, maximum_total=min(3072, 512 * len(subjects)))
            report.update(listed_inventories=listing.inventories,
                          listed_inventory_bindings=listed_summary(listing.inventories),
                          listed_total=listing.total, campaign_admission=False)
        if args.lane in {"coverage", "all"}:
            # Keep the native per-file threshold scoped to exactly the selected
            # production files; all other current source files remain in custody.
            ignored = [re.escape("/work/source/" + name) + "$" for name in session.product_identity
                       if name.endswith(".rs") and name not in subjects.values()]
            ignored.append("/rustlib/src/rust/library/")
            regex = "|".join(ignored)
            if len(regex.encode()) > 65_536:
                raise Incomplete("Engine KE exact coverage filename filter exceeds its bound.")
            raw = run("coverage", [str(session.tools / "cargo-llvm-cov"), "llvm-cov",
                "--package", "nevrotick-engine", "--features", "ke-association-prototype", "--frozen",
                "--jobs", "6", "--lib", "--tests", "--json", "--output-path", "/work/coverage.json",
                "--failure-mode", "any", "--ignore-filename-regex", regex,
                "--fail-under-file-lines", "90", "--", "--test-threads=1"], policy["coverage_wall_seconds"])
            report["coverage_execution"] = validate_engine_ke_execution(raw, binding)
            coverage = COMMON.read_analysis_json(session.work / "coverage.json")
            report["coverage"] = {name: COMMON.coverage_counts(ENGINE_KE_SCOPE, coverage,
                "/work/source/" + filename) for name, filename in subjects.items()}
        if args.lane in {"mutation", "all"}:
            COMMON.run_analysis_lanes(ENGINE_KE_SCOPE, subjects, policy, session.work, session.tools,
                                      run, report, "mutation")
    finally:
        session.finish()
        if admit_engine_ke_evidence(binding) != evidence_before:
            raise Incomplete("Engine KE reference evidence changed during the job.")
    report["selected_lane_passed"] = args.lane != "enumerate"
    report["selected_host_metrics_passed"] = args.lane == "all" and subjects == ENGINE_KE_HOST_SUBJECTS
    report["selected_host_qualification_passed"] = False
    if args.lane == "contracts":
        report["provider_claim_contracts_passed" if args.subject == "ke_process" else "protected_history_contracts_passed"] = True
    report["independent_mutation_classification"] = "Required for every caught, unviable or missed descriptor; pending separate review."
    # A selected GNU pass never closes the remaining owners, Windows or native custody.
    report["qualification_passed"] = False

SOURCE_POLICY_POLICY = COMMON.ENGINE_SOURCE_POLICY_POLICY
SOURCE_POLICY_SUBJECTS = dict(COMMON.ENGINE_SOURCE_POLICY_SUBJECTS)
SOURCE_POLICY_SCOPE = COMMON.Scope(
    "nevrotick-engine", COMMON.ENGINE_SOURCE_POLICY_SUBJECTS, SOURCE_POLICY_POLICY)
SOURCE_POLICY_ORDINARY_COUNT = 267
SOURCE_POLICY_DOCTEST_COUNT = 89
# Independent inspection of actual installed-configuration full-4 established
# these complete sorted identities. They identify a roster, not current execution.
SOURCE_POLICY_ORDINARY_DIGEST = '8ed080ae310ed733a87f58922edcf10880a1d0ec249b88673224e49efa9bc0f3'
SOURCE_POLICY_DOCTEST_DIGEST = '5f737f32c7949073c5738f61d95b62b847ba1eb4680a58c5ca2a02d8dd9f6483'
# The same independently inspected receipt contains the complete optional
# profile in both modes: its default roster plus these 234 KE identities.
SOURCE_POLICY_EP1_EXTRA_COUNT = 234
SOURCE_POLICY_EP1_DIGEST = 'f0de16de547951830fa06c1d49fa103ecce0d968ac20e68f0e538711d8f8a1a9'
SOURCE_POLICY_CAPTURE_CANDIDATE = "8019957465542f8586cd7d7635398dcb589e8813cf374c3615f4dea2b2ab5e80"
SOURCE_POLICY_CAPTURE_MANIFEST = "6cff4b71635c3243fa98fd962adf7075ef47b51b3fd1c48a51e3b6ae5fc208a2"
SOURCE_POLICY_CAPTURE_BOUNDARY = "f8e7451c343908a9c42abe6472317c2a40c56f3fc08b13585392e2c492d90ced"
SOURCE_POLICY_CAPTURE_ROSTER = {
    "default-ordinary": (276, "c06a3de85145150811788d80899f7581ef16e33aae63833dbf9d568b6e5ea505"),
    "default-doctests": (89, "57e7889097c2d6cae7592301d4ac009d9392d674aed73b6b2bb93eb69c466f2d"),
    "ep1-ordinary": (548, "3b0bce2e2af7f2a134d096e4191c645d5721866a39f539cb6bb88603ddee681f"),
    "ep1-doctests": (90, "27fda4fdb03c642627fa63e22d54d6c89ce0d4cd06a39af8d838821a356f576b"),
}
SOURCE_POLICY_RECEIVE_CANDIDATE = "27026b88695e648c1d5e0ea5ba34ed984a68a91b3ea0a34c2499d936b34c49d2"
SOURCE_POLICY_RECEIVE_MANIFEST = "26c4b874ec042cd29c3dffb3a75431c265d438c8be32ba240f121cf86d702c4b"
SOURCE_POLICY_RECEIVE_BOUNDARY = "2d631683e435fa46ad18c30f3e572bf3937385bb8ed5d8852fe4fcdbd92c0a32"
SOURCE_POLICY_RECEIVE_FOCUSED = "c3662299a084db3362065157df191f7e06f709aeecdf0548ecba895d93dcfc2c"
SOURCE_POLICY_RECEIVE_ROSTER = {
    "default-ordinary": (278, "a75cbee4454317189176d031906ca8b7aacfa331c73c0ebdcb39c2d0f227ea38"),
    "default-doctests": (89, "57e7889097c2d6cae7592301d4ac009d9392d674aed73b6b2bb93eb69c466f2d"),
    "ep1-ordinary": (551, "ff2942ea9cd28e65d99191407c65833fddd6a076d7707eb3f389539352f8ce8c"),
    "ep1-doctests": (90, "27fda4fdb03c642627fa63e22d54d6c89ce0d4cd06a39af8d838821a356f576b"),
}
SOURCE_POLICY_CHRONOLOGY_CANDIDATE = "703fb8c4c4fb402754bdad7adde7dfc651800eb57e9e9328f1ac5a5c5d5b3b09"
SOURCE_POLICY_CHRONOLOGY_MANIFEST = "95b241c00a64448c998159be536be99011c056388c188976a7ab2d71a8652748"
SOURCE_POLICY_CHRONOLOGY_FOCUSED = "85e597637c322ba091cca6e8f81770f18f7ead38c121f5c20dbd3ff72b75aaf1"
SOURCE_POLICY_CHRONOLOGY_PARTIAL = "0176c77dc50b9ccfae5c9545464fe3c70c47c79d43cb758be524795fce1a5130"
SOURCE_POLICY_CHRONOLOGY_BOUNDARY = "ac49edd2d1356df8d456c70d67a3349f41fa61edf40045443827a38ea423c01d"
SOURCE_POLICY_CHRONOLOGY_ROSTER = {
    "default-ordinary": (279, "fec56b9f68471d141456074a08d0f054a1ecbac0852fb11717558e5874f93151"),
    "default-doctests": (89, "57e7889097c2d6cae7592301d4ac009d9392d674aed73b6b2bb93eb69c466f2d"),
    "ep1-ordinary": (554, "1148532ddb9b532f65f328e93817ad43f24330691553ec6966beb636508f0128"),
    "ep1-doctests": (90, "27fda4fdb03c642627fa63e22d54d6c89ce0d4cd06a39af8d838821a356f576b"),
}
SOURCE_POLICY_DEBIT_CANDIDATE = "43abe8085314d61c793a7ec8fc2c19475a4bb4f15adffae2e10f658a65e752a5"
SOURCE_POLICY_DEBIT_MANIFEST = "c3f116dd7734d6b436b973c5ec38df1acda57c87237e722bc827a8cc9a25e0ec"
SOURCE_POLICY_DEBIT_BOUNDARY = "fc6420f30fabe729f4cf1b0c52d03d296a218803fc707bbca120327f9bb38008"
SOURCE_POLICY_DEBIT_FOCUSED = "e47376dccd5e24479cd1dacd1d5a368d7a282dc2a97e3230f7d9d67c5f8d9f76"
SOURCE_POLICY_DEBIT_FOCUSED_FAILURE = "692f2767c2f3bccc1832a7329268af7e8ff31f5bd705d5fe8b6cb42469dce4c4"
SOURCE_POLICY_DEBIT_FOCUSED_COUNT = 46
SOURCE_POLICY_DEBIT_ROSTER = {
    **SOURCE_POLICY_CHRONOLOGY_ROSTER,
    "ep1-ordinary": (555, "0b6ff42c270b6c308be24b1f6567a9eaa12ed743579ded375343ffa10a26964b"),
}
SOURCE_POLICY_HEADER_AGE_CANDIDATE = "f3c9a30558aa22eaa029d85a91adcc76c5d9b17df8e6aee454803cfc4d8a5709"
SOURCE_POLICY_HEADER_AGE_MANIFEST = "1c287d0910566cdfee3c251050a839e071015893e66d3931b520386f711a3b49"
SOURCE_POLICY_HEADER_AGE_BOUNDARY = "65bf0209d98f169a0dd78a792b07959d97bcc6ad27f7526045deb47107b8cd6c"
SOURCE_POLICY_HEADER_AGE_FOCUSED = "c9dd7e50f753b4f543b94c385127fed88654428b7e2f266249caaad83e89ee68"
SOURCE_POLICY_HEADER_AGE_FOCUSED_COUNT = 40
SOURCE_POLICY_HEADER_AGE_FOCUSED_FAILURE = None
SOURCE_POLICY_HEADER_AGE_ROSTER = {
    "default-ordinary": (280, "be73133484648bf1d02673a8c2eeb79a86f2acffb353deea8a8cf3b9ca27f1d4"),
    "default-doctests": (89, "dcc13ef92a10f8d4e15d4084dffe43b219eb7635c1a243e88320a18a8e926f06"),
    "ep1-ordinary": (556, "7fc2c14e28d3154b0e56f70b5e117d201ffadeedeee26b757a7f191ac9ac78cf"),
    "ep1-doctests": (90, "3181186a0c48e132f6ff5d1a834c0988ce6c36b34a95ee5972f179f2d635f7af"),
}
SOURCE_POLICY_PROTECTED_ORDER_CANDIDATE = "e8ef1377d9bcfa089eda65d0f5092f67f9e753d440a5092499593453f6a4c1fa"
SOURCE_POLICY_PROTECTED_ORDER_MANIFEST = "d9e8c0d6a6813a1b2f4c80b8224df27a0f790b82ae92210046b99a2602e6fa18"
SOURCE_POLICY_PROTECTED_ORDER_BOUNDARY = "e4d2034e187dc1fb03e140e84fe23266f81339df3cd300debf242d0636bb2f3c"
SOURCE_POLICY_PROTECTED_ORDER_FOCUSED = "cd929a5de7b1cc3be277920f4514820e7b4a2bd7b3d0c5ddcf8f427f8f1431d4"
SOURCE_POLICY_PROTECTED_ORDER_FOCUSED_COUNT = 54
SOURCE_POLICY_PROTECTED_ORDER_FOCUSED_FAILURE = "a773a8aa5fc4c9fdc63bdd0894dad05d3f36bc82539cc54c5c04ed3138b74b60"
SOURCE_POLICY_PROTECTED_ORDER_EARLIER_TREE = "fa58f28089d7e75ab21830617bfcebab01c8c586"
SOURCE_POLICY_PROTECTED_ORDER_ROSTER = {
    **SOURCE_POLICY_HEADER_AGE_ROSTER,
    "ep1-ordinary": (557, "bcd55a3e51d142761a2d6cee33aa6ae0fe768272f6bbadfd7ef8736c7d9766a7"),
}
SOURCE_POLICY_APPLICATION_PLACEMENT_CANDIDATE = "978691a8c54da5e9f9000dc00a13fc4c067dd7689fcf5c63e6145aaf98ca2461"
SOURCE_POLICY_APPLICATION_PLACEMENT_MANIFEST = "140d8cbfb1be00e790d0c6058f67b35326e34242019f83f67caca3398b79d5e6"
SOURCE_POLICY_APPLICATION_PLACEMENT_BOUNDARY = "085f4d313dd2df71559155f4f3c4262fc13d7eb9755ff2f6622e9cdb68ed172a"
SOURCE_POLICY_APPLICATION_PLACEMENT_FOCUSED = "fce5527574a3cdce921e0f87a35c8af3b4ee9d76e4f81aa406f752fc1a8da301"
SOURCE_POLICY_APPLICATION_PLACEMENT_FOCUSED_COUNT = 57
SOURCE_POLICY_APPLICATION_PLACEMENT_ROSTER = {
    **SOURCE_POLICY_PROTECTED_ORDER_ROSTER,
    "default-doctests": (89, "96a1e6a857c89bf32d753e1a9facb4cc70d504878e65059e3f9f3b816206d128"),
    "ep1-doctests": (90, "288546d3441c3e3ca074a8984a2d07fd5d5500259013e70429696162d3d46d86"),
}
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_CANDIDATE = "555a6a076d567a173e5f5c6f728115d5cfff4a0b05da2959ebd77b137dc76ec0"
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_MANIFEST = "11122157011c0e7e026c34e726f00a1679278bce8e709be02dfe4897d4e3fff0"
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_BOUNDARY = "2234b175fb39c0c31012057487209015b709cbd4e36b0915ed6ebf9d65457d11"
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_FOCUSED = "bcd20cab305b1dd60094d1694ae660420e754a4065d7ae95466a4463e8956203"
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_FOCUSED_COUNT = 105
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_PREVIOUS_FULL_FAILURE = 'f0e1386f57be0e92cd5a315ab45b044cefbb0a2c80168f8403a7b35c30fec1a7'
SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_ROSTER = {
    **SOURCE_POLICY_APPLICATION_PLACEMENT_ROSTER,
    "ep1-ordinary": (560, "c401b5b1f770c4a7b187fedcf9326e20ce485a09cbac1f49e97bafccdfbd3a3b"),
}
SOURCE_POLICY_SOURCE_SCALE_CANDIDATE = "5fc32aace7312d232a753325ccb124d104c74d7426c26d8a89ded0b8aa17f045"
SOURCE_POLICY_SOURCE_SCALE_MANIFEST = "cf025d23e8a01905b1ecb2ae3eac3263c72933ba85308dd57252c4ae73bb190d"
SOURCE_POLICY_SOURCE_SCALE_BOUNDARY = "2055347cbcc606094bf089a72090a5ff06c0932b4fb96bfb7cf308a36de8e3e7"
SOURCE_POLICY_SOURCE_SCALE_CARGO_HOME = "c9f613cefbacfe54d325a6823c1721eafaf6013236aca3b002411880fcbb9d00"
SOURCE_POLICY_SOURCE_SCALE_OVERLAY = frozenset((
    "crates/nevrotick-engine/src/lib.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/managed_preview_tests.rs",
    "crates/nevrotick-engine/src/source_policy.rs",
    "crates/nevrotick-engine/tests/private/source_configuration.rs",
))
SOURCE_POLICY_SOURCE_SCALE_ROSTER = {
    "default-ordinary": SOURCE_POLICY_HEADER_AGE_ROSTER["default-ordinary"],
    "default-doctests": (90, "ebff9da9a5f427dd6086bb8733b1c646f6812654e6eef3163c1a682830cf39a6"),
    "ep1-ordinary": (576, "61bdfced5c6df7bc5ee77f7b6e492a9ec701f80afa500c237acd825d1f0934c5"),
    "ep1-doctests": (91, "f40e89f48370695701eea46bc046187451fd50adedada6a3517ee48240397e55"),
}
SOURCE_POLICY_ENGINE_REVIEW_ROSTER = {
    **SOURCE_POLICY_SOURCE_SCALE_ROSTER,
    "default-ordinary": (287, "84d08df5d0a20953516189d599a4f347aa3ee87b721742cfb5db3a3ccfb7eeee"),
    "ep1-ordinary": (584, "576e1343e3d222ff0db4b7c2721ced9c7c6c5b453dfacc619065f0c9aa548268"),
}
SOURCE_POLICY_SNAPSHOT_TESTS = frozenset(
    "source_policy::tests::policy_snapshot_" + name for name in (
        "canonical_aliases_and_unknown_reads_do_not_allocate",
        "capped_rate_and_wait_are_historical",
        "distinguishes_source_endpoint_and_selective_reenable",
        "does_not_publish_changes_or_wake_a_future_floor",
        "keeps_unavailable_records_as_data",
        "preserves_independent_rate_intervals",
        "preserves_poison_and_the_healthy_owner",
        "public_sessions_observe_real_restriction_and_unmanaged_absence",
        "status_is_bounded_payload_free_and_preserves_sink_errors",
    ))
SOURCE_POLICY_PROVENANCE_TESTS = frozenset((
    "source_policy::tests::managed_source_provenance_follows_actual_owner_across_sessions_and_endpoints",
    "series::tests::managed_source_provenance_receiving_rejects_absent_and_foreign_owners",
    "series::address_round::tests::managed_source_provenance_address_transfer_binds_the_original_candidate",
))
SOURCE_POLICY_CONFIGURATION_TESTS = frozenset((
    'source_policy::tests::configuration::source_configuration_address_set_bounds_and_canonical_membership',
    'source_policy::tests::configuration::source_configuration_install_and_expected_snapshot_replacement_are_atomic',
    'source_policy::tests::configuration::source_configuration_parallel_public_sessions_and_reordered_round_share_revision',
    'source_policy::tests::configuration::source_configuration_outside_peers_cannot_publish_endpoint_capacity',
    'source_policy::tests::configuration::source_configuration_first_install_and_equal_replace_retire_old_prepared_plans',
    'source_policy::tests::configuration::source_configuration_replacement_preserves_kod_records_floors_and_targets',
    'source_policy::tests::configuration::source_configuration_revision_exhaustion_is_sticky_and_source_local',
    'series::tests::source_configuration_final_reply_replacement_preserves_report',
    'series::tests::source_configuration_final_send_replacement_prevents_send',
    'series::tests::source_configuration_postcommit_replacement_preserves_history',
))
# One actual full development receipt, independently inspected before reader adoption.
# These pins admit historical execution bytes, never a reporter-selected replacement.
SOURCE_POLICY_MANAGED_DEVELOPMENT = {
    "candidate_sha256": "23c91549b893b1319ef8dd3337977ae18b23fe54766c045ff8733fdc4f4e9d1e",
    "recipe_sha256": "90a72882e4f3ef8f1eca513dc14d0aa6af58c759da18876ad513221b32c81b28",
    "budget_sha256": "a10983778a8b8a4f82112f58cc06866448a037ab72055271e74c5cdf90d1d88b",
    "source_budget_sha256": "15db3989c53eee4b62cc408eafadb58f12c5f80a6c8f9d316b2bd83f04cfb89b",
    "captured_inputs_sha256": "df68d9563eae9a6ade4ac479da9d21dcc5ad60cd5a0402d154513e905584e8ff",
    "monitor_sha256": "195230d35949351788eba7b027fd66d73e033a580c3693f195d5f7f14c793e23"
}
SOURCE_POLICY_CONFIGURATION_LINEAGE = {
    "original_development_budget_sha256": "be5a791bf90643e493a6a7df5aa2f905169eb790c2444625c5bb63255fed575d",
    "original_recipe_sha256": "55478c7df9e4c32e5a969b4775c39171c3cfeba3bb89611e0d51dfc714ab8445",
    "original_monitor_sha256": "af339c40cb1e9b93913c3a2f724c7fa29a5b677995fcba27a1525ff1652ed03f",
    "replan_review_sha256": "7e578d5bd4022d0aeb2f9e0d924f564b6604518aac44d62b59a51dc1333ca814",
    "closeout_sha256": "f35ca498d803206f2fae37e077b4681bc1fc5689cb5b6d4ed271082a9d855e77"
}
# These controls have their own later finite clock. They did not produce full-4
# and cannot replace or restart its original development allowance.
SOURCE_POLICY_MANAGED_CONTROL_BUDGET = {
    "sha256": "3a561e24c98dff8e1105dd46dddacaf11e426f4bcbe3cc7d19ffab086e1ce24d",
    "start_unix": 1794939988.4403768,
}
# Actual healthy and deliberate stale-observer controls; the latter is no product pass.
SOURCE_POLICY_MANAGED_MONITOR_CONTROLS = {
    "healthy/control.json": "43ee2568409398f2a6224f0a93e4ba76821f4207b60a980bc2fcc85eb66cebfd",
    "healthy/monitor-result.json": "78925aeaff58297cab02516e7179f345d2d84fae69f1cf879fa6433c1ab20b90",
    "healthy/resources.jsonl": "a553c43da422b150eaf4f756e2832dd5ae8aaa0f6dd45768b89286f90bdb0d99",
    "gap/control.json": "08919608b6195e41ba5bd964711467237a6c1c0bba00de48c6eb63530830af1b",
    "gap/monitor-result.json": "3ba802dee07a2e8ebfad8e1d4fd48186b3d53fd10a815d18ef1061f9892c40e4",
    "gap/resources.jsonl": "5069a2eb2afc5538f35b4d2b6304b779c8ad80a48e5c0d949051c9b9a178356c"
}

SOURCE_POLICY_BOUND_INPUTS = tuple(dict.fromkeys((
    WRAPPER, TESTS, *SOURCE_POLICY_SUBJECTS.values(),
    "crates/nevrotick-engine/tests/private/source_policy.rs",
    "crates/nevrotick-engine/tests/private/source_configuration.rs",
    "crates/nevrotick-engine/tests/private/session.rs",
    "crates/nevrotick-engine/tests/private/series.rs",
    "crates/nevrotick-engine/tests/private/series_process.rs",
    "crates/nevrotick-engine/tests/private/address_round.rs",
    "crates/nevrotick-engine/tests/private/address_progress.rs",
    "crates/nevrotick-engine/tests/private/processing_guards.rs",
    "crates/nevrotick-engine/tests/private/argument_guards.rs",
    "crates/nevrotick-engine/tests/runtime_boundary.rs",
    "crates/nevrotick-engine/tests/prepare_release.rs",
    "crates/nevrotick-engine/tests/prepare_entered.rs",
    "crates/nevrotick-engine/tests/execute_entered.rs",
    "crates/nevrotick-engine/tests/series.rs",
    "crates/nevrotick-engine/tests/session.rs",
    "crates/nevrotick-engine/tests/quality.rs",
    "tasks/engine-source-policy-equivalences.md",
    "scripts/portable-analysis.py", "scripts/tests/test_portable_analysis.py",
    "scripts/analysis-runtime.py", "scripts/check-analysis.py", "scripts/analysis-controls.py",
    "scripts/prepare-product.py", "scripts/verify.py", "scripts/check-environment.py",
    "scripts/check-strict.py", "scripts/slint_dependency_profile.py",
    "scripts/slint_linux_sdk.py", "scripts/advisory_snapshot.py",
    "policy/analysis-tools.json", "policy/verification-manifest.toml",
    "policy/tool-manifest.json", "policy/ui-m0-dependencies.json",
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml", "rust-toolchain.toml",
    ".cargo/config.toml", ".cargo/mutants.toml", ".config/nextest.toml",
)))


def source_policy_combined_roster(candidate: object) -> tuple[dict, int] | None:
    """Select the roster and private-case count for the exact combined product."""
    if candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE:
        return ENGINE_JOINED_RECEIVING_BOUNDARY_ROSTER, 77
    if candidate == ENGINE_JOINED_RECEIVING_CANDIDATE:
        return ENGINE_JOINED_RECEIVING_ROSTER, 77
    if candidate == ENGINE_STALE_DEBIT_CANDIDATE:
        return (ENGINE_STALE_DEBIT_ROSTER, 75)
    if candidate == ENGINE_FINAL_FRAME_CANDIDATE:
        return (ENGINE_FINAL_FRAME_ROSTER, 75)
    if candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE:
        return (ENGINE_PROTECTED_DEBIT_ROSTER, 75)
    if candidate == ENGINE_PLACEMENT_CANDIDATE:
        return ENGINE_PLACEMENT_ROSTER, 75
    if candidate == ENGINE_SCALE_CONFLICT_CANDIDATE:
        return ENGINE_SCALE_CONFLICT_ROSTER, 75
    if candidate == ENGINE_REVIEW_CANDIDATE:
        return SOURCE_POLICY_ENGINE_REVIEW_ROSTER, 74
    if candidate == SOURCE_POLICY_SOURCE_SCALE_CANDIDATE:
        return SOURCE_POLICY_SOURCE_SCALE_ROSTER, 74
    if candidate == SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_CANDIDATE:
        return SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_ROSTER, 74
    if candidate == SOURCE_POLICY_APPLICATION_PLACEMENT_CANDIDATE:
        return SOURCE_POLICY_APPLICATION_PLACEMENT_ROSTER, 74
    if candidate == SOURCE_POLICY_PROTECTED_ORDER_CANDIDATE:
        return SOURCE_POLICY_PROTECTED_ORDER_ROSTER, 74
    if candidate == SOURCE_POLICY_HEADER_AGE_CANDIDATE:
        return SOURCE_POLICY_HEADER_AGE_ROSTER, 74
    if candidate == SOURCE_POLICY_DEBIT_CANDIDATE:
        return SOURCE_POLICY_DEBIT_ROSTER, 73
    if candidate == SOURCE_POLICY_CHRONOLOGY_CANDIDATE:
        return SOURCE_POLICY_CHRONOLOGY_ROSTER, 73
    if candidate == SOURCE_POLICY_RECEIVE_CANDIDATE:
        return SOURCE_POLICY_RECEIVE_ROSTER, 72
    if candidate == SOURCE_POLICY_CAPTURE_CANDIDATE:
        return SOURCE_POLICY_CAPTURE_ROSTER, 70
    return None


def source_policy_test_inventory(binding: dict) -> tuple[list[str], list[str]]:
    """Require complete independent default identities, not a same-count substitute."""
    ordinary, docs = binding.get("test_inventory"), binding.get("doctest_inventory")
    combined = source_policy_combined_roster(binding.get("candidate_sha256"))
    default_ordinary = (combined[0]["default-ordinary"] if combined else
                        (SOURCE_POLICY_ORDINARY_COUNT, SOURCE_POLICY_ORDINARY_DIGEST))
    default_doctests = (combined[0]["default-doctests"] if combined else
                        (SOURCE_POLICY_DOCTEST_COUNT, SOURCE_POLICY_DOCTEST_DIGEST))
    for names, (count, digest) in ((ordinary, default_ordinary), (docs, default_doctests)):
        if (type(names) is not list or len(names) != count
                or any(type(name) is not str or not 0 < len(name) <= 4096 for name in names)
                or names != sorted(set(names)) or COMMON.canonical_sha256(names) != digest):
            raise Incomplete(f"Source-policy qualification requires its complete "
                             f"{default_ordinary[0]}/{default_doctests[0]} default roster.")
    selected = [name for name in ordinary if name.startswith("source_policy::tests::")]
    kod_docs = [name for name in docs if name.startswith(
        "crates/nevrotick-engine/src/source_policy.rs - ")]
    compiler_count, negative_count = (
        (19, 14) if binding.get("candidate_sha256") in (
            SOURCE_POLICY_SOURCE_SCALE_CANDIDATE, ENGINE_REVIEW_CANDIDATE,
            ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE, ENGINE_PROTECTED_DEBIT_CANDIDATE,
            ENGINE_FINAL_FRAME_CANDIDATE, ENGINE_STALE_DEBIT_CANDIDATE, *ENGINE_JOINED_RECEIVING_FAMILY)
        else (18, 13))
    if (len(selected) != (combined[1] if combined else 68)
            or "source_policy::tests::public_policy_diagnostics_are_literal_bounded_and_redacted" not in ordinary
            or "source_policy::tests::public_managed_setup_exhausts_64_stale_hints_without_a_send_or_refund" not in ordinary
            or any(name.startswith("nts_ke_experiment::") for name in ordinary)
            or not SOURCE_POLICY_SNAPSHOT_TESTS <= set(selected)
            or not SOURCE_POLICY_PROVENANCE_TESTS <= set(ordinary)
            or not SOURCE_POLICY_CONFIGURATION_TESTS <= set(ordinary)
            or (binding.get("candidate_sha256") in (ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE, ENGINE_PROTECTED_DEBIT_CANDIDATE, ENGINE_FINAL_FRAME_CANDIDATE, ENGINE_STALE_DEBIT_CANDIDATE)
                and ENGINE_SCALE_CONFLICT_TEST not in ordinary)
            or len(kod_docs) != compiler_count
            or sum(name.endswith(" - compile fail") for name in kod_docs) != negative_count):
        raise Incomplete("Source-policy roster must retain all controls, public diagnostics, the setup budget, "
                         f"snapshots, source provenance, configuration cases and {compiler_count} compiler examples.")
    return ordinary, docs


def validate_source_policy_policy(policy: dict, *, campaign: bool = False) -> dict:
    """Admit only the closed five-owner role and independently bound current inputs."""
    COMMON.require_engine_source_policy_role(SOURCE_POLICY_SCOPE, policy)
    binding = policy["binding"]
    fields = {"candidate_sha256", "source_file_count", "inputs_sha256", "test_inventory",
              "doctest_inventory", "development_evidence"}
    if type(binding) is not dict or set(binding) != fields:
        raise Incomplete("Source-policy current candidate binding has not been admitted.")
    if (type(binding["source_file_count"]) is not int or binding["source_file_count"] <= 0
            or type(binding["candidate_sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", binding["candidate_sha256"]) is None):
        raise Incomplete("Source-policy candidate identity is malformed.")
    pins = binding["inputs_sha256"]
    if (type(pins) is not dict or set(pins) != set(SOURCE_POLICY_BOUND_INPUTS)
            or any(type(value) is not str or re.fullmatch(r"[a-f0-9]{64}", value) is None
                   for value in pins.values())):
        raise Incomplete("Source-policy protected source/helper closure is incomplete.")
    expected_count = engine_review_input_count(binding["candidate_sha256"])
    if expected_count is not None and binding["source_file_count"] != expected_count:
        raise Incomplete("Engine review source-policy requires the exact product input count.")
    source_policy_test_inventory(binding)
    evidence = binding["development_evidence"]
    if (type(evidence) is not dict or set(evidence) != {"path", "sha256"}
            or type(evidence["path"]) is not str or not Path(evidence["path"]).is_absolute()
            or type(evidence["sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", evidence["sha256"]) is None):
        raise Incomplete("Source-policy development reference needs exact external custody.")
    if campaign and (not policy["campaigns_admitted"] or not policy["mutation_inventories"]):
        raise Incomplete("Source-policy campaigns remain closed pending all five reviewed inventories.")
    return binding


def read_source_policy_policy(root: Path, *, campaign: bool = False) -> tuple[dict, dict, dict]:
    """Refuse stale bytes or incomplete security registration before any Session/effect."""
    policy = read_json(root / SOURCE_POLICY_POLICY)
    binding = validate_source_policy_policy(policy, campaign=campaign)
    _, identity = PRODUCT.admit_product(root)
    if (len(identity) != binding["source_file_count"]
            or COMMON.canonical_sha256(identity) != binding["candidate_sha256"]
            or any(PRODUCT.sha256(root / name) != digest
                   for name, digest in binding["inputs_sha256"].items())):
        raise Incomplete("Source-policy actual candidate or protected helper bytes differ.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    critical, modules = manifest.get("critical_files"), manifest.get("security_modules")
    if (type(critical) is not list or any(type(name) is not str for name in critical)
            or type(modules) is not list
            or any(type(row) is not dict or type(row.get("package")) is not str
                   or type(row.get("files")) is not list
                   or any(type(name) is not str for name in row["files"]) for row in modules)):
        raise Incomplete("Source-policy critical/security inventory has malformed records.")
    for filename in SOURCE_POLICY_SUBJECTS.values():
        owners = [row["package"] for row in modules for name in row["files"] if name == filename]
        if critical.count(filename) != 1 or owners != ["nevrotick-engine"]:
            raise Incomplete("Every source-policy owner needs one exact critical/security registration.")
    return policy, binding, identity


def source_policy_execution(text: str, binding: dict, *, doctests: bool = False) -> dict:
    """Reconcile actual complete libtest blocks against independently frozen identities."""
    ordinary, docs = source_policy_test_inventory(binding)
    names = read_successful_address_records(text)
    if sorted(names) != sorted(ordinary + docs if doctests else ordinary):
        raise Incomplete("Source-policy execution omitted or substituted an admitted case.")
    return {"tests_executed": len(names), "test_names": names,
            "inventory_sha256": COMMON.canonical_sha256(sorted(names))}


def source_policy_ep1_execution(text: str, binding: dict) -> dict:
    """Require the complete independently bound optional-profile execution."""
    ordinary, _docs = source_policy_test_inventory(binding)
    names = read_successful_address_records(text)
    default_names = set(ordinary)
    digest = COMMON.canonical_sha256(sorted(names))
    combined = source_policy_combined_roster(binding.get("candidate_sha256"))
    if combined:
        count, expected_digest = combined[0]["ep1-ordinary"]
    else:
        count = len(ordinary) + SOURCE_POLICY_EP1_EXTRA_COUNT
        expected_digest = SOURCE_POLICY_EP1_DIGEST
        extras = [name for name in names if name not in default_names]
        if (len(extras) != SOURCE_POLICY_EP1_EXTRA_COUNT
                or any(not name.startswith("nts_ke_experiment::") for name in extras)):
            raise Incomplete("Source-policy EP1 execution omitted or substituted an admitted case.")
    if len(names) != count or not default_names <= set(names) or digest != expected_digest:
        raise Incomplete("Source-policy EP1 execution omitted or substituted an admitted case.")
    return {"tests_executed": len(names), "test_names": names,
            "inventory_sha256": digest}


def source_policy_development_doctests(text: str, binding: dict, lane: str) -> dict:
    """Validate the complete annotated Rustdoc identities for the selected graph."""
    facts = validate_engine_ke_doctests(text)
    names = sorted(facts["test_names"])
    combined = source_policy_combined_roster(binding.get("candidate_sha256"))
    if lane == "ep1-doctests" and combined:
        count, digest = combined[0][lane]
        if len(names) != count or COMMON.canonical_sha256(names) != digest:
            raise Incomplete("Source-policy EP1 development compiler examples differ.")
    elif lane not in {"default-doctests", "ep1-doctests"} or names != binding["doctest_inventory"]:
        raise Incomplete("Source-policy development compiler examples differ.")
    return facts


def admit_source_policy_resource_samples(raw: bytes, result: dict, *, gap_control: bool = False) -> None:
    """Reconcile the retained sampler's finite timeline and unchanged resource floors."""
    try:
        samples = [json.loads(line, object_pairs_hook=COMMON.ANALYSIS.unique_object,
                             parse_float=finite_number, parse_constant=invalid_number)
                   for line in raw.decode("utf-8").splitlines()]
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Source-policy resource samples are malformed.") from error
    if not samples or len(samples) != result["samples"]:
        raise Incomplete("Source-policy resource sample count differs.")
    previous = None
    for sample in samples:
        if (type(sample) is not dict
                or any(type(sample.get(key)) not in {int, float}
                       or not 0 <= sample[key] <= sys.float_info.max or not math.isfinite(sample[key])
                       for key in ("monotonic_seconds", "gap_seconds"))
                or (not gap_control and sample["gap_seconds"] > 2)
                or (previous is not None and (sample["monotonic_seconds"] < previous
                    or sample["monotonic_seconds"] - previous != sample["gap_seconds"]))
                or any(type(sample.get(key)) is not int or sample[key] < minimum
                       for key, minimum in (("available_bytes", 1024**3), ("free_bytes", 4 * 1024**3),
                                            ("memory_bytes", 0), ("tasks", 1)))
                or sample["memory_bytes"] > 4 * 1024**3 or sample["tasks"] > 512):
            raise Incomplete("Source-policy resource samples violate their clock or resource bounds.")
        previous = sample["monotonic_seconds"]
    maximum = max(sample["gap_seconds"] for sample in samples)
    terminal_gap = result["finished_monotonic"] - previous
    if (result["maximum_gap_seconds"] != maximum or not 0 <= terminal_gap <= 2
            or (gap_control and maximum <= 2)):
        raise Incomplete("Source-policy resource gap or terminal freshness differs.")


def admit_source_policy_monitor_control(directory: Path, role: str, recipe: str, inputs: dict, raw: dict) -> None:
    """Keep the deliberately failed freshness control distinct from healthy runtime proof."""
    control_path = directory / "control.json"
    terminal_path = directory / "monitor-result.json"
    samples_path = directory / "resources.jsonl"
    control = read_json(control_path, expected_sha256=inputs[control_path])
    terminal = read_json(terminal_path, expected_sha256=inputs[terminal_path])
    monitor = control.get("monitor")
    gap = role == "gap"
    if (control.get("expected_control_observed") is not True or type(control.get("notifications")) is not list
            or type(monitor) is not dict or monitor.get("joined") is not True
            or monitor.get("ok") is not (not gap) or type(monitor.get("exit_code")) is not int
            or monitor["exit_code"] != (2 if gap else 0)
            or monitor.get("errors") != (["Monitor failed or stopped unexpectedly."] if gap else [])
            or type(monitor.get("result")) is not dict
            or COMMON.canonical_sha256(monitor["result"]) != COMMON.canonical_sha256(terminal)
            or monitor.get("result_sha256") != inputs[terminal_path]
            or monitor.get("resource_samples_sha256") != inputs[samples_path]
            or terminal.get("status") != ("failed" if gap else "stopped")
            or terminal.get("role") != role or terminal.get("recipe_sha256") != recipe
            or terminal.get("recipe_final_sha256") != recipe
            or any(type(terminal.get(key)) is not int or terminal[key] <= 0
                   for key in ("samples", "parent_pid", "parent_start", "observer_pid"))
            or terminal["parent_pid"] == terminal["observer_pid"]
            or any(type(terminal.get(key)) not in {int, float}
                   or not 0 <= terminal[key] <= sys.float_info.max or not math.isfinite(terminal[key])
                   for key in ("maximum_gap_seconds", "finished_monotonic"))
            or terminal["finished_monotonic"] <= 0):
        raise Incomplete("Source-policy prior monitor control lacks its exact observed disposition.")
    notifications = control["notifications"]
    if ((gap and (len(notifications) != 1
                  or type(notifications[0]) not in {int, float}
                  or not 0 < notifications[0] <= sys.float_info.max or not math.isfinite(notifications[0])
                  or terminal["maximum_gap_seconds"] <= 2
                  or terminal.get("error") != "Resource observer freshness exceeded two seconds."
                  or terminal.get("parent_notification") != "pidfd-SIGTERM"))
            or (not gap and (notifications != [] or terminal["maximum_gap_seconds"] > 2
                             or "error" in terminal or "parent_notification" in terminal))):
        raise Incomplete("Source-policy monitor freshness control or owned notification differs.")
    admit_source_policy_resource_samples(raw[samples_path], terminal, gap_control=gap)


# Detached draft: the full2 receipt pin names actual completed development only.
SOURCE_POLICY_RECEIVER_DEVELOPMENT = {
    "candidate_sha256": "0910c474424d5b8c37410d757d0cfe147f3ce60b3b2b00395263c24acf588762",
    "incomplete_candidate_sha256": "20cd2fae9a8745e870d332002137feea936cb184557ff5ba453632c3b38a3f21",
    "test_path": "crates/nevrotick-engine/tests/private/address_round.rs",
    "test_sha256": "c6ebb3b16cc874a3a0b2b0c780df063a681ee7dc95bd0d0f13d3425166d44991",
    "prior_path": "/home/david_osipov/.local/share/nevrotick/continuation-20261116-tools-v1/source-config-revisions-20261117-v1/full-4/verification.json",
    "prior_sha256": "c0b612def73fb20eeb78b99710b98201f38d51ff60a27f4c2c2861fdeb16157c",
    "recipe_sha256": "6435d63975170b7ec75ecfcad195db07d816770697f55a683a23b46f5915b6e7",
    "monitor_sha256": "df8dc68e68c34ba8a3b67a0fa3e67586cdcc13cc0dfa4d29505ba781c4e44276",
    "source_application_sha256": "dbae27c8d59437d5215c76e41c3e1220844f7ff18f869295f9ad650c19673089",
    "lint_repair_sha256": "ef592c90504e4c5ff51cce7d12492f5077a895b44b236c788584fe30644cf23e",
    "incomplete_full1_sha256": "6c7e84e81ff3969751b64b28de765733e83b1dc1898358802163e989c96a6d4f",
    "initial_recipe_sha256": "55b9a83b6cb56f4846a44342b852e0baaeb197024c8972d68be0c5dd27410a2c",
    "initial_budget_sha256": "24d1d607bb6f05fc5e0b4f2c68358286bafb7ec5a32de42a8f38ba87f9dc5ab5",
    "initial_parent_sha256": "8bb5b3b77e04a3c9f2d8d576618de78252bcfc01d487760aaa93e478f8df935f",
    "current_witness_sha256": "511a05a664f63d0e94c39d06702858a115fd20d9be8b93758e9a11ebf83aef2c",
    "preimage_cleanup_sha256": "3a325e9b1342d7b8dc51240db7c1128dd9bfc2650003fb26f7f9016bc3fa027f",
    "readmission_plan_sha256": "76cda92c724d120b49eea79a79e58a79e9d0f709a182c62e6112de1b3c51cc91",
    "reference_sha256": "917e021da12d6e3e223e5f90934cdbe91baa68d6d79dd79e47cbec78a5063fae",
    "budget_sha256": "7691e12f35b1079d05bd928bde8e1653c17bebd0f0461484aa6ae650e349a15c",
    "continuation_parent_budget_sha256": "e1ca3656280edf24d904905b12a5e50f6d5b31ada62463e0152a6c819cf584d2",
    "captured_inputs_sha256": "d642c9f052606ecc760ad16755b176d361adb8b275ac6f78fcf5fb3fedb21c8d",
    "protected_inputs_digest": "0631e9ae361e6d4949ff8415060f75937768faa253c473b052f3fe8f4f8648b7",
    "captured_file_count": 786,
    "monitor_control_inputs": {
        "healthy/control.json": "16a5f4941e7a497960a7e3b1299a4db9192d796efe69a90e25235dea2e560898",
        "healthy/monitor-result.json": "c819dc3a0d4286ff47425326ee64c9ac00ea35f60528011aa998b0e39ec49796",
        "healthy/resources.jsonl": "9dfb028648f06efb7cf825c57a6c75fa195bceb18b0c2ea7126afbb1abf1c53e",
        "gap/control.json": "503ddd44d66a7dceae3338c90c4e2275955c3d399c6920e7b264e0f781d0e0f4",
        "gap/monitor-result.json": "4ce893d7659cb317b1bee31d501af33afda139a0e1cf91a12f647755bc5db9dc",
        "gap/resources.jsonl": "ff457f097d40e240ade44e949db79fa52db8dc4b4172b184a696b28a1f866e6a",
    },
}


def admit_source_policy_receiver_observer(reference: dict, path: Path, binding: dict,
                                         identity: dict) -> dict:
    """Admit only the fixed existing-test repair; keep the historical profile intact."""
    pins = SOURCE_POLICY_RECEIVER_DEVELOPMENT
    hashes = ("prior_sha256", "recipe_sha256", "monitor_sha256", "source_application_sha256",
              "readmission_plan_sha256", "reference_sha256", "budget_sha256",
              "continuation_parent_budget_sha256", "captured_inputs_sha256", "protected_inputs_digest",
              "lint_repair_sha256", "incomplete_full1_sha256", "initial_recipe_sha256", "initial_budget_sha256",
              "initial_parent_sha256", "current_witness_sha256", "preimage_cleanup_sha256")
    controls = pins["monitor_control_inputs"]
    control_names = {role + "/" + leaf for role in ("healthy", "gap")
                     for leaf in ("control.json", "monitor-result.json", "resources.jsonl")}
    if (any(type(pins.get(key)) is not str or re.fullmatch(r"[a-f0-9]{64}", pins[key]) is None for key in hashes)
            or type(pins["captured_file_count"]) is not int or not 606 <= pins["captured_file_count"] <= 4096
            or type(controls) is not dict or set(controls) != control_names
            or any(type(value) is not str or re.fullmatch(r"[a-f0-9]{64}", value) is None for value in controls.values())):
        raise Incomplete("Receiver-guard development pins remain unbound.")
    if (path.name != "verification.json" or path.parent.name != "full-2"
            or path.parent.parent.name != "receiver-guard-development-v1"
            or PRODUCT.sha256(path) != pins["reference_sha256"]):
        raise Incomplete("Receiver-guard development leaves its exact fixed producer.")
    base = path.parent.parent
    inputs = {path: pins["reference_sha256"]}

    def physical(filename, digest, maximum=4 * 1024**2):
        raw = COMMON.read_analysis_bytes(filename, maximum=maximum)
        if type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None or hashlib.sha256(raw).hexdigest() != digest:
            raise Incomplete("Receiver-guard physical evidence changed.")
        inputs[filename] = digest
        return raw

    def document(filename, digest):
        physical(filename, digest)
        return read_json(filename, maximum=4 * 1024**2, expected_sha256=digest)

    prior_path = Path(pins["prior_path"])
    prior = document(prior_path, pins["prior_sha256"])
    old_binding = {**binding, "candidate_sha256": SOURCE_POLICY_MANAGED_DEVELOPMENT["candidate_sha256"],
                   "source_file_count": 606,
                   "development_evidence": {"path": str(prior_path), "sha256": pins["prior_sha256"]}}
    # This recursive call selects the unchanged historical23c profile, including
    # actual old lanes, physical capture, original failures/clocks and controls.
    inputs.update({Path(name): digest for name, digest in
                   admit_source_policy_development(old_binding, prior["source_sha256"]).items()})
    expected = dict(prior["source_sha256"])
    expected[pins["test_path"]] = pins["test_sha256"]
    if (len(expected) != 606 or identity != expected or reference.get("source_sha256") != expected
            or COMMON.canonical_sha256(expected) != pins["candidate_sha256"]
            or binding["candidate_sha256"] != pins["candidate_sha256"]):
        raise Incomplete("Receiver-guard development requires exactly its single test delta.")
    fields = {"HEAD", "actual_named_results", "application_start_unix", "attempt", "budget_sha256", "candidate_sha256",
        "captured_inputs", "cargo_home_config_sha256", "cleanup_deadline_unix", "compiler", "continuation_parent_budget_sha256",
        "cumulative_charged_case_ceiling", "custody_unchanged", "durable_monitor", "effective_work_deadline_unix", "elapsed_seconds",
        "generated_mutation_credit", "independent_monitor", "index_sha256", "lanes", "maximum_new_Rust_result_instances",
        "monitor_control_inputs", "monitor_sha256", "ordered_witness", "phase", "prior_charged_case_ceiling",
        "prior_development_reference", "protected_inputs_digest", "protected_inputs_sha256", "public_campaigns_admitted",
        "qualification_passed", "readmission_plan_sha256", "recipe_sha256", "scope", "source_application_sha256", "source_sha256",
        "status", "terminal_resource_failures", "tools", "vendor_identity", "work_completed_unix",
        "prior_incomplete_development", "lint_repair_source_sha256", "this_attempt_case_ceiling",
        "current_ordered_witness", "ordered_witness_reuse_scope"}
    if (set(reference) != fields or reference["status"] != "workload-passed" or reference["phase"] != "full"
            or type(reference["attempt"]) is not int or reference["attempt"] != 2
            or reference["custody_unchanged"] is not True or reference["terminal_resource_failures"] != []
            or any(reference[key] is not False for key in ("generated_mutation_credit", "qualification_passed", "public_campaigns_admitted"))
            or any(type(reference[key]) is not int or reference[key] != value for key, value in (
                ("actual_named_results", 1714), ("maximum_new_Rust_result_instances", 3428), ("this_attempt_case_ceiling", 1714)))
            or any(reference[key] != pins[key] for key in ("recipe_sha256", "budget_sha256", "monitor_sha256",
                   "source_application_sha256", "readmission_plan_sha256", "continuation_parent_budget_sha256", "protected_inputs_digest"))
            or reference["prior_development_reference"] != {"path": str(prior_path), "sha256": pins["prior_sha256"]}
            or any(reference[key] != prior[key] for key in ("tools", "compiler", "vendor_identity", "cargo_home_config_sha256"))):
        raise Incomplete("Receiver-guard development format, producer or limits differ.")
    budget = document(base / "development-budget-retry.json", pins["budget_sha256"])
    parent = document(base / "continuation-parent-budget-retry.json", pins["continuation_parent_budget_sha256"])
    first_budget = document(base / "development-budget.json", pins["initial_budget_sha256"])
    first_parent = document(base / "continuation-parent-budget.json", pins["initial_parent_sha256"])
    application = document(base / "source-application.json", pins["source_application_sha256"])
    repair = document(base / "lint-repair-source.json", pins["lint_repair_sha256"])
    first_path = base / "full-1/verification.json"
    first = document(first_path, pins["incomplete_full1_sha256"])
    physical(base / "development.py", pins["initial_recipe_sha256"])
    physical(base / "development-retry-canonical.py", pins["recipe_sha256"])
    physical(base / "monitor.py", pins["monitor_sha256"])
    physical(base / "readmission-plan.json", pins["readmission_plan_sha256"])
    if (budget.get("admitted") is not True or budget.get("mutations_admitted") is not False
            or budget.get("public_campaigns_admitted") is not False
            or any(type(budget.get(key)) is not int or budget[key] != value for key, value in (
                ("schema_version", 1), ("budget_seconds", 1800), ("work_seconds", 1500), ("reserve_seconds", 300),
                ("maximum_full_attempts", 2), ("maximum_focused_attempts", 0), ("maximum_new_Rust_result_instances", 3428), ("source_file_count", 606)))
            or budget.get("retry_authorized") is not True
            or budget.get("prior_incomplete_development_sha256") != pins["incomplete_full1_sha256"]
            or budget.get("lint_repair_source_sha256") != pins["lint_repair_sha256"]
            or budget.get("new_test_names") != [] or budget.get("new_doctest_roles") != []
            or budget.get("source_files") != {pins["test_path"]: pins["test_sha256"]}
            or any(budget.get(key) != reference[key] for key in ("recipe_sha256", "monitor_sha256", "readmission_plan_sha256",
                "candidate_sha256", "protected_inputs_sha256", "protected_inputs_digest", "captured_inputs", "monitor_control_inputs",
                "prior_charged_case_ceiling", "cumulative_charged_case_ceiling", "ordered_witness", "current_ordered_witness", "continuation_parent_budget_sha256"))
            or budget.get("captured_inputs_sha256") != pins["captured_inputs_sha256"]
            or budget.get("baseline") != reference["HEAD"] or budget.get("index_sha256") != reference["index_sha256"]):
        raise Incomplete("Receiver-guard child allowance differs from its exact one-attempt contract.")
    if (parent.get("admitted") is not True or parent.get("separate_subsequent_parent") is not True
            or parent.get("all_prior_costs_and_failures_retained") is not True
            or type(parent.get("budget_seconds")) is not int or parent["budget_seconds"] != 3000
            or type(parent.get("cleanup_reserve_seconds")) is not int or parent["cleanup_reserve_seconds"] != 300
            or type(parent.get("maximum_new_Rust_result_instances")) is not int or parent["maximum_new_Rust_result_instances"] != 3700
            or parent.get("source_application_sha256") != pins["source_application_sha256"]
            or parent.get("prior_parent_budget_sha256") != pins["initial_parent_sha256"]
            or any(type(parent.get(key)) is not int or parent[key] != value for key, value in (
                ("additional_retry_case_ceiling", 1714), ("additional_current_witness_case_ceiling", 5)))
            or type(first_parent.get("maximum_new_Rust_result_instances")) is not int
            or first_parent["maximum_new_Rust_result_instances"] + 1714 + 5 != parent["maximum_new_Rust_result_instances"]):
        raise Incomplete("Receiver-guard requires its separately admitted retained-cost parent.")
    origin = application["source_application_start_unix"]
    times = [origin, application["source_application_end_unix"], parent.get("start_unix"),
             *(budget.get(key) for key in ("start_unix", "preparation_started_unix", "application_start_unix", "shared_parent_start_unix",
               "shared_parent_work_deadline_unix", "shared_parent_total_deadline_unix")),
             *(reference[key] for key in ("application_start_unix", "effective_work_deadline_unix", "cleanup_deadline_unix", "work_completed_unix", "elapsed_seconds"))]
    if (any(type(value) not in {int, float} or not 0 < value <= sys.float_info.max or not math.isfinite(value) for value in times)
            or any(budget[key] != origin for key in ("start_unix", "preparation_started_unix", "application_start_unix", "shared_parent_start_unix"))
            or parent["start_unix"] != origin or reference["application_start_unix"] != origin
            or application["source_application_end_unix"] < origin
            or application["candidate_sha256"] != pins["incomplete_candidate_sha256"]
            or application["head"] != reference["HEAD"] or application["index_sha256"] != reference["index_sha256"]
            or budget["shared_parent_work_deadline_unix"] != origin + 2700 or budget["shared_parent_total_deadline_unix"] != origin + 3000
            or reference["effective_work_deadline_unix"] != origin + 1500 or reference["cleanup_deadline_unix"] != origin + 1800
            or not application["source_application_end_unix"] <= reference["work_completed_unix"] < origin + 1500
            or not reference["elapsed_seconds"] < 1800):
        raise Incomplete("Receiver-guard cannot reset, precede or exceed its actual application clock.")
    if (first.get("status") != "incomplete" or first.get("candidate_sha256") != pins["incomplete_candidate_sha256"]
            or type(first.get("attempt")) is not int or first["attempt"] != 1
            or first.get("recipe_sha256") != pins["initial_recipe_sha256"] or first.get("budget_sha256") != pins["initial_budget_sha256"]
            or first.get("continuation_parent_budget_sha256") != pins["initial_parent_sha256"]
            or first.get("application_start_unix") != origin or first_budget.get("start_unix") != origin or first_parent.get("start_unix") != origin
            or first.get("terminal_resource_failures") != [] or first.get("independent_monitor", {}).get("joined") is not True
            or reference["prior_incomplete_development"] != {"path": str(first_path), "sha256": pins["incomplete_full1_sha256"]}
            or reference["lint_repair_source_sha256"] != pins["lint_repair_sha256"]
            or repair.get("prior_development_sha256") != pins["incomplete_full1_sha256"]
            or repair.get("application_start_unix") != origin or repair.get("prior_candidate_sha256") != pins["incomplete_candidate_sha256"]
            or repair.get("candidate_sha256") != pins["candidate_sha256"] or repair.get("source_sha256") != pins["test_sha256"]
            or repair.get("previous_test_sha256") != application["source_sha256"] or repair.get("path") != pins["test_path"]
            or type(repair.get("repair_observed_unix")) not in {int, float} or not math.isfinite(repair["repair_observed_unix"])
            or not application["source_application_end_unix"] <= repair["repair_observed_unix"] <= reference["work_completed_unix"]):
        raise Incomplete("Receiver-guard must retain its genuine first failure and same-clock lint repair.")
    first_source = dict(prior["source_sha256"])
    first_source[pins["test_path"]] = application["source_sha256"]
    if first.get("source_sha256") != first_source or COMMON.canonical_sha256(first_source) != pins["incomplete_candidate_sha256"]:
        raise Incomplete("Receiver-guard first failure source identity differs.")
    first_rows = first.get("lanes")
    if (type(first_rows) is not list or any(type(row) is not dict or type(row.get("exit_code")) is not int for row in first_rows)
            or [(row.get("lane"), row.get("exit_code"), row.get("failure")) for row in first_rows] != [
            ("boundary-control", 0, None), ("format", 0, None), ("gnu-check", 0, None), ("gnu-clippy", 101, None)]):
        raise Incomplete("Receiver-guard first failure status was rewritten.")
    for row in first_rows:
        filename = Path(row["path"])
        if filename.parent != first_path.parent:
            raise Incomplete("Receiver-guard first failure lane escapes its directory.")
        raw_lane = document(filename, row["sha256"])
        if type(raw_lane.get("exit_code")) is not int or raw_lane["exit_code"] != row["exit_code"] or raw_lane.get("failure") is not None:
            raise Incomplete("Receiver-guard first failure raw status differs.")
    cleanup_path = first_path.parent / "duplicate-cleanup.json"
    cleanup = document(cleanup_path, pins["preimage_cleanup_sha256"])
    canonical_preimage = base.parent / "receiver-guard-sensitivity-v1/run-attempt-2/candidate"
    if (cleanup.get("snapshot_duplicate_removed") != str(first_path.parent / "source")
            or cleanup.get("canonical_snapshot") != str(canonical_preimage)
            or cleanup.get("retained_receipts", {}).get(str(first_path)) != pins["incomplete_full1_sha256"]):
        raise Incomplete("Receiver-guard first-source reconstruction bridge differs.")
    original_test = physical(canonical_preimage / pins["test_path"], application["source_sha256"])
    before_parameter = b"fn managed_setup_binding_mismatch(case: SetupBindingMismatch)"
    before_call = b"managed_setup_binding_mismatch(mismatch)?;"
    if original_test.count(before_parameter) != 1 or original_test.count(before_call) != 1:
        raise Incomplete("Receiver-guard lint repair preimage differs.")
    repaired_test = original_test.replace(before_parameter, b"fn managed_setup_binding_mismatch(case: &SetupBindingMismatch)").replace(
        before_call, b"managed_setup_binding_mismatch(&mismatch)?;")
    if hashlib.sha256(repaired_test).hexdigest() != pins["test_sha256"]:
        raise Incomplete("Receiver-guard retry contains more than its two borrowed-argument tokens.")
    preceding = budget.get("preceding_operation_inputs")
    parent_inputs = parent.get("prior_cost_and_failure_inputs")
    if (type(preceding) is not dict or not preceding or type(parent_inputs) is not dict
            or preceding != {**parent_inputs, str(base / "continuation-parent-budget.json"): pins["initial_parent_sha256"],
                             str(base / "development.py"): pins["initial_recipe_sha256"],
                             str(cleanup_path): pins["preimage_cleanup_sha256"]}):
        raise Incomplete("Receiver-guard preceding costs/failures are not preserved.")
    for name, digest in preceding.items():
        declared = Path(name)
        if not declared.is_absolute() or ".." in declared.parts:
            raise Incomplete("Receiver-guard preceding material leaves its canonical scope.")
        physical(declared, digest)
    witness_ref = reference["current_ordered_witness"]
    if type(witness_ref) is not dict or set(witness_ref) != {"path", "sha256"} or preceding.get(witness_ref["path"]) != witness_ref["sha256"]:
        raise Incomplete("Receiver-guard preceding ordered witness is missing.")
    witness = document(Path(witness_ref["path"]), witness_ref["sha256"])
    ledger_path = base.parent / "mutation-cohorts/cohort-ledger.json"
    ledger = document(ledger_path, preceding.get(str(ledger_path)))
    old_witness_ref = reference["ordered_witness"]
    if type(old_witness_ref) is not dict or old_witness_ref != first.get("ordered_witness") or preceding.get(old_witness_ref["path"]) != old_witness_ref["sha256"]:
        raise Incomplete("Receiver-guard original ordered witness lineage differs.")
    old_witness = document(Path(old_witness_ref["path"]), old_witness_ref["sha256"])
    if (ledger.get("active") is not None or witness_ref["sha256"] != pins["current_witness_sha256"]
            or witness.get("status") != "focused-generated-sensitivity-observed-independent-review-pending"
            or witness.get("detached_candidate_sha256") != pins["candidate_sha256"]
            or old_witness.get("status") != "focused-generated-sensitivity-observed-independent-review-pending"
            or old_witness.get("detached_candidate_sha256") != pins["incomplete_candidate_sha256"]
            or witness.get("original_custody_unchanged") is not True or witness.get("terminal_resource_failures") != []
            or any(item.get("qualification_passed") is not False or item.get("generated_mutation_credit") is not False
                   or item.get("independent_monitor", {}).get("ok") is not True or item.get("independent_monitor", {}).get("joined") is not True
                   or type(item.get("healthy_exact_executions")) is not int or item["healthy_exact_executions"] != 1
                   or type(item.get("caught_exact_executions")) is not int or item["caught_exact_executions"] != 4 for item in (old_witness, witness))
            or type(witness.get("maximum_test_executions")) is not int or witness["maximum_test_executions"] != 5
            or type(ledger.get("cumulative_charged_case_ceiling")) is not int
            or type(reference["prior_charged_case_ceiling"]) is not int
            or any(type(item.get("maximum_test_executions")) is not int or item["maximum_test_executions"] != 5 for item in (old_witness, witness))
            or type(first_budget.get("prior_charged_case_ceiling")) is not int
            or first_budget["prior_charged_case_ceiling"] != ledger["cumulative_charged_case_ceiling"] + 5
            or type(first_budget.get("cumulative_charged_case_ceiling")) is not int
            or first_budget["cumulative_charged_case_ceiling"] != first_budget["prior_charged_case_ceiling"] + 1714
            or reference["prior_charged_case_ceiling"] != first_budget["cumulative_charged_case_ceiling"] + 5
            or type(reference["cumulative_charged_case_ceiling"]) is not int
            or reference["cumulative_charged_case_ceiling"] != reference["prior_charged_case_ceiling"] + 1714
            or reference["cumulative_charged_case_ceiling"] > first_budget["prior_charged_case_ceiling"] + parent["maximum_new_Rust_result_instances"]):
        raise Incomplete("Receiver-guard original history or cumulative charges changed.")
    captured, protected = reference["captured_inputs"], reference["protected_inputs_sha256"]
    if (type(captured) is not dict or len(captured) != pins["captured_file_count"]
            or any(type(name) is not str or not 0 < len(name) <= 512 or Path(name).is_absolute() or ".." in Path(name).parts
                   or Path(name).as_posix() != name or any(char in name for char in ("\\", "\0", "\n", "\r"))
                   or type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None for name, digest in captured.items())
            or COMMON.canonical_sha256(captured) != pins["captured_inputs_sha256"]
            or any(captured.get(name) != digest for name, digest in expected.items())
            or type(protected) is not dict or set(protected) != set(SOURCE_POLICY_BOUND_INPUTS)
            or COMMON.canonical_sha256(protected) != pins["protected_inputs_digest"]
            or any(captured.get(name) != digest for name, digest in protected.items())
            or reference["monitor_control_inputs"] != controls):
        raise Incomplete("Receiver-guard frozen source/protected closure differs.")
    for name, digest in captured.items():
        physical(path.parent / "source" / name, digest, COMMON.ANALYSIS_JSON_MAX_BYTES)
    monitor = reference["independent_monitor"]
    result = monitor.get("result") if type(monitor) is dict else None
    if (type(monitor) is not dict or set(monitor) != {"ok", "errors", "result", "exit_code", "joined", "result_sha256", "resource_samples_sha256"}
            or monitor["ok"] is not True or monitor["joined"] is not True or monitor["errors"] != []
            or type(monitor["exit_code"]) is not int or monitor["exit_code"] != 0 or type(result) is not dict
            or set(result) != {"status", "parent_pid", "parent_start", "observer_pid", "role", "recipe_sha256", "samples", "maximum_gap_seconds", "finished_monotonic", "recipe_final_sha256"}
            or result["status"] != "stopped" or result["role"] != "development"
            or result["recipe_sha256"] != pins["monitor_sha256"] or result["recipe_final_sha256"] != pins["monitor_sha256"]
            or any(type(result[key]) is not int or result[key] <= 0 for key in ("samples", "parent_pid", "parent_start", "observer_pid"))
            or result["parent_pid"] == result["observer_pid"]
            or any(type(result[key]) not in {int, float} or not 0 <= result[key] <= sys.float_info.max or not math.isfinite(result[key])
                   for key in ("maximum_gap_seconds", "finished_monotonic"))
            or result["maximum_gap_seconds"] > 2 or result["finished_monotonic"] <= 0):
        raise Incomplete("Receiver-guard observer lacks joined actual custody.")
    directory = path.parent / "resource-monitor"
    if reference["durable_monitor"] != str(directory):
        raise Incomplete("Receiver-guard observer leaves its fixed durable directory.")
    terminal = document(directory / "monitor-result.json", monitor["result_sha256"])
    samples_raw = physical(directory / "resources.jsonl", monitor["resource_samples_sha256"])
    if terminal != result:
        raise Incomplete("Receiver-guard physical observer differs.")
    admit_source_policy_resource_samples(samples_raw, result)
    samples = [json.loads(line) for line in samples_raw.decode().splitlines()]
    previous_elapsed = None
    for sample in samples:
        elapsed = sample.get("elapsed_seconds")
        if (type(elapsed) not in {int, float} or not math.isfinite(elapsed) or not 0 <= elapsed < 1500
                or (previous_elapsed is not None and elapsed < previous_elapsed)):
            raise Incomplete("Receiver-guard samples escaped the original application work clock.")
        previous_elapsed = elapsed
    final = samples[-1]
    terminal_unix = origin + final["elapsed_seconds"] + result["finished_monotonic"] - final["monotonic_seconds"]
    if not reference["work_completed_unix"] <= terminal_unix <= reference["cleanup_deadline_unix"]:
        raise Incomplete("Receiver-guard actual join escaped its original cleanup clock.")
    raw_controls = {}
    for name, digest in controls.items():
        filename = base / "monitor-controls" / name
        raw_controls[filename] = physical(filename, digest)
    for role in ("healthy", "gap"):
        admit_source_policy_monitor_control(base / "monitor-controls" / role, role, pins["monitor_sha256"], inputs, raw_controls)
        control_result = read_json(base / "monitor-controls" / role / "monitor-result.json", expected_sha256=controls[role + "/monitor-result.json"])
        control_samples = [json.loads(line) for line in raw_controls[base / "monitor-controls" / role / "resources.jsonl"].decode().splitlines()]
        previous_elapsed = None
        for sample in control_samples:
            elapsed = sample.get("elapsed_seconds")
            if (type(elapsed) not in {int, float} or not math.isfinite(elapsed) or not 0 <= elapsed < 1500
                    or (previous_elapsed is not None and elapsed < previous_elapsed)):
                raise Incomplete("Receiver-guard controls reset or exceed the actual application clock.")
            previous_elapsed = elapsed
        if control_result["finished_monotonic"] > samples[0]["monotonic_seconds"]:
            raise Incomplete("Receiver-guard controls did not precede the actual workload observer.")
    return {str(filename): digest for filename, digest in inputs.items()}


def admit_source_policy_managed_observer(reference: dict, path: Path) -> dict:
    """Bind the fixed managed-observation producer, capture and actual monitor controls."""
    pins = SOURCE_POLICY_MANAGED_DEVELOPMENT
    fields = {"status", "phase", "attempt", "lanes", "scope", "generated_mutation_credit",
              "budget_sha256", "recipe_sha256", "source_budget_sha256", "HEAD", "index_sha256", "source_sha256",
              "captured_inputs", "candidate_sha256", "tools", "compiler", "vendor_identity",
              "cargo_home_config_sha256", "custody_unchanged", "durable_monitor",
              "terminal_resource_failures", "independent_monitor", "elapsed_seconds"}
    if (set(reference) != fields or reference["status"] != "workload-passed"
            or reference["phase"] != "full" or type(reference["attempt"]) is not int
            or reference["attempt"] != 4 or reference["custody_unchanged"] is not True
            or reference["generated_mutation_credit"] is not False
            or reference["terminal_resource_failures"] != []
            or type(reference["elapsed_seconds"]) not in {int, float}
            or not 0 < reference["elapsed_seconds"] < 4200
            or reference["candidate_sha256"] != pins["candidate_sha256"]
            or reference["recipe_sha256"] != pins["recipe_sha256"]
            or reference["budget_sha256"] != pins["budget_sha256"]
            or reference["source_budget_sha256"] != pins["source_budget_sha256"]):
        raise Incomplete("Source-policy managed development format or fixed producer differs.")
    source, captured = reference["source_sha256"], reference["captured_inputs"]
    for inventory, count in ((source, 606), (captured, 786)):
        if (type(inventory) is not dict or len(inventory) != count
                or any(type(name) is not str or not 0 < len(name) <= 512
                       or Path(name).is_absolute() or ".." in Path(name).parts
                       or Path(name).as_posix() != name
                       or any(char in name for char in ("\\", "\0", "\n", "\r"))
                       or type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None
                       for name, digest in inventory.items())):
            raise Incomplete("Source-policy managed capture has malformed source identities.")
    if (COMMON.canonical_sha256(source) != pins["candidate_sha256"]
            or COMMON.canonical_sha256(captured) != pins["captured_inputs_sha256"]
            or any(captured.get(name) != digest for name, digest in source.items())):
        raise Incomplete("Source-policy managed capture differs from its complete reviewed closure.")
    base = path.parent.parent
    producer = base / "extension-v1"
    budget_path = producer / "development-budget.json"
    lineage = SOURCE_POLICY_CONFIGURATION_LINEAGE
    inputs = {
        budget_path: pins["budget_sha256"], producer / "development.py": pins["recipe_sha256"],
        producer / "monitor.py": pins["monitor_sha256"],
        base / "source-development-budget.json": pins["source_budget_sha256"],
        base / "development-budget.json": lineage["original_development_budget_sha256"],
        base / "development.py": lineage["original_recipe_sha256"],
        base / "monitor.py": lineage["original_monitor_sha256"],
        producer / "resource-replan-review.json": lineage["replan_review_sha256"],
        base / "source-development-closeout-coherent.json": lineage["closeout_sha256"],
    }
    budget = read_json(budget_path, maximum=262144, expected_sha256=pins["budget_sha256"])
    if (budget.get("admitted") is not True
            or any(type(budget.get(key)) is not int or budget[key] != expected for key, expected in (
                ("budget_seconds", 4500), ("work_seconds", 4200), ("reserve_seconds", 300),
                ("maximum_attempts_per_phase", 4), ("maximum_focused_attempts", 3),
                ("maximum_full_attempts", 4), ("additional_full_attempts", 1),
                ("extension_seconds", 900), ("maximum_new_Rust_result_instances", 6886)))
            or budget.get("mutations_admitted") is not False
            or budget.get("recipe_sha256") != pins["recipe_sha256"]
            or budget.get("source_budget_sha256") != pins["source_budget_sha256"]
            or budget.get("prior_development_budget_sha256") != lineage["original_development_budget_sha256"]
            or budget.get("prior_recipe_sha256") != lineage["original_recipe_sha256"]
            or budget.get("prior_monitor_sha256") != lineage["original_monitor_sha256"]
            or budget.get("coordinator_review_sha256") != lineage["replan_review_sha256"]
            or budget.get("candidate_sha256") != pins["candidate_sha256"]
            or budget.get("original_budget_disposition") != "incomplete; no original failure or clock reset"
            or type(budget.get("new_test_names")) is not list
            or len(budget["new_test_names"]) != len(SOURCE_POLICY_CONFIGURATION_TESTS)
            or any(type(name) is not str for name in budget["new_test_names"])
            or set(budget["new_test_names"]) != SOURCE_POLICY_CONFIGURATION_TESTS
            or any(type(budget.get(key)) not in {int, float}
                   or not 0 < budget[key] <= sys.float_info.max or not math.isfinite(budget[key])
                   for key in ("start_unix", "preparation_started_unix"))
            or budget["start_unix"] != budget["preparation_started_unix"]
            or type(budget.get("source_files")) is not dict or not budget["source_files"]
            or any(type(name) is not str or name not in source or source[name] != digest
                   for name, digest in budget["source_files"].items())
            or reference["HEAD"] != budget.get("baseline")
            or reference["index_sha256"] != budget.get("index_sha256")):
        raise Incomplete("Source-policy managed development allowance or original custody differs.")
    source_budget = read_json(base / "source-development-budget.json", maximum=262144,
                              expected_sha256=pins["source_budget_sha256"])
    original = read_json(base / "development-budget.json", maximum=262144,
                         expected_sha256=lineage["original_development_budget_sha256"])
    if (source_budget.get("admitted") is not True
            or source_budget.get("baseline") != reference["HEAD"]
            or type(source_budget.get("preparation_start_unix")) not in {int, float}
            or source_budget["preparation_start_unix"] != budget["start_unix"]
            or any(type(source_budget.get(key)) is not int or source_budget[key] != expected
                   for key, expected in (("budget_seconds", 3600), ("work_seconds", 3300),
                       ("cleanup_seconds", 300), ("max_focused_attempts", 3),
                       ("max_full_development_attempts", 3)))
            or source_budget.get("no_mutations_admitted") is not True
            or source_budget.get("public_campaigns_admitted") is not False
            or source_budget.get("all_prior_source_qualification_clocks_and_charges_retained") is not True
            or source_budget.get("dependency_or_toolchain_changes_allowed") is not False
            or original.get("admitted") is not True
            or original.get("mutations_admitted") is not False
            or any(type(original.get(key)) is not int or original[key] != expected
                   for key, expected in (("budget_seconds", 3600), ("reserve_seconds", 300),
                       ("maximum_attempts_per_phase", 3), ("maximum_new_Rust_result_instances", 5172)))
            or any(type(original.get(key)) not in {int, float} or original[key] != budget["start_unix"]
                   for key in ("start_unix", "preparation_started_unix"))
            or original.get("source_budget_sha256") != pins["source_budget_sha256"]
            or original.get("recipe_sha256") != lineage["original_recipe_sha256"]
            or original.get("baseline") != reference["HEAD"]
            or original.get("index_sha256") != reference["index_sha256"]):
        raise Incomplete("Source-policy original source allowance or retained extension lineage differs.")
    review = read_json(producer / "resource-replan-review.json", maximum=262144,
                       expected_sha256=lineage["replan_review_sha256"])
    reviewed = review.get("reviewed_extension")
    if (review.get("verdict") != "READY" or review.get("completion") != "completed"
            or type(reviewed) is not dict
            or any(type(reviewed.get(key)) is not int or reviewed[key] != expected
                   for key, expected in (("original_total_seconds", 3600), ("original_work_seconds", 3300),
                       ("extension_seconds", 900), ("cumulative_total_seconds", 4500),
                       ("cumulative_work_seconds", 4200), ("cleanup_seconds", 300),
                       ("maximum_focused_attempts", 3), ("maximum_full_attempts", 4),
                       ("original_result_upper_charge", 5172), ("additional_result_upper_charge", 1714),
                       ("cumulative_result_upper_charge", 6886), ("product_inputs", 606),
                       ("complete_lanes", 13), ("default_cases_per_mode", 267),
                       ("ep1_cases_per_mode", 501), ("doctests_per_graph", 89)))
            or type(reviewed.get("original_start_unix")) not in {int, float}
            or reviewed["original_start_unix"] != budget["start_unix"]
            or reviewed.get("candidate_sha256") != pins["candidate_sha256"]):
        raise Incomplete("Source-policy extension requires its independent original-clock review.")
    closeout = read_json(base / "source-development-closeout-coherent.json", maximum=262144,
                         expected_sha256=lineage["closeout_sha256"])
    closeout_fields = {"status", "baseline", "product", "product_inputs", "captured_inputs",
        "source_origin_unix", "closed_unix", "cumulative_elapsed_seconds", "original3600_disposition",
        "extension4500_disposition", "verification_sha256", "implementation_review_sha256",
        "reviewed_source_unchanged", "executed_named_results", "prior_focused_results",
        "actual_result_total", "cumulative_result_upper_charge", "metric_admission",
        "public_campaigns", "native_qualification"}
    if (type(closeout) is not dict or set(closeout) != closeout_fields
            or closeout["status"] != "source-development-passed-qualification-not-admitted"
            or closeout["baseline"] != reference["HEAD"] or closeout["product"] != pins["candidate_sha256"]
            or closeout["verification_sha256"] != PRODUCT.sha256(path)
            or closeout["original3600_disposition"] != "incomplete; original first failures retained"
            or closeout["extension4500_disposition"] != "all13 development lanes passed within4200work; observer joined"
            or type(closeout["implementation_review_sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", closeout["implementation_review_sha256"]) is None
            or closeout["reviewed_source_unchanged"] is not True
            or any(closeout[key] is not False for key in ("metric_admission", "public_campaigns", "native_qualification"))
            or any(type(closeout[key]) is not int or closeout[key] != expected for key, expected in (
                ("product_inputs", 606), ("captured_inputs", 786), ("executed_named_results", 1714),
                ("prior_focused_results", 10), ("actual_result_total", 1724), ("cumulative_result_upper_charge", 6886)))
            or any(type(closeout[key]) not in {int, float} or not 0 < closeout[key] <= sys.float_info.max
                   or not math.isfinite(closeout[key])
                   for key in ("source_origin_unix", "closed_unix", "cumulative_elapsed_seconds"))
            or closeout["source_origin_unix"] != budget["start_unix"]
            or closeout["closed_unix"] - closeout["source_origin_unix"] != closeout["cumulative_elapsed_seconds"]
            or not 0 < closeout["cumulative_elapsed_seconds"] < 4200):
        raise Incomplete("Source-policy development closeout lacks actual source/clock/terminal custody.")
    control_base = base / "qualification-admission"
    control_budget_path = control_base / "development-budget.json"
    control_pin = SOURCE_POLICY_MANAGED_CONTROL_BUDGET
    if (type(control_pin) is not dict or set(control_pin) != {"sha256", "start_unix"}
            or type(control_pin["sha256"]) is not str
            or re.fullmatch(r"[a-f0-9]{64}", control_pin["sha256"]) is None
            or type(control_pin["start_unix"]) not in {int, float}
            or not 0 < control_pin["start_unix"] <= sys.float_info.max
            or not math.isfinite(control_pin["start_unix"])):
        raise Incomplete("Source-policy managed monitor control allowance has not been independently pinned.")
    control_budget = read_json(control_budget_path, maximum=262144, expected_sha256=control_pin["sha256"])
    resource_profile = {
        "memory_max": 6 * 1024**3, "swap_max": 512 * 1024**2, "tasks_max": 512,
        "startup_free_disk": 7 * 1024**3, "startup_available_ram": 15 * 1024**3 // 2,
        "running_free_disk": 4 * 1024**3, "running_available_ram": 1024**3,
        "observation_max_gap_seconds": 2,
    }
    if (type(control_budget) is not dict
            or set(control_budget) != {"admitted", "start_unix", "preparation_started_unix",
                                       "budget_seconds", "reserve_seconds", "monitor_sha256", "resource_profile"}
            or control_budget["admitted"] is not True
            or any(type(control_budget[key]) is not int or control_budget[key] != expected
                   for key, expected in (("budget_seconds", 4500), ("reserve_seconds", 300)))
            or any(type(control_budget[key]) not in {int, float}
                   or not 0 < control_budget[key] <= sys.float_info.max
                   or not math.isfinite(control_budget[key])
                   or control_budget[key] != control_pin["start_unix"]
                   for key in ("start_unix", "preparation_started_unix"))
            or control_budget["start_unix"] < closeout["closed_unix"]
            or control_budget["monitor_sha256"] != pins["monitor_sha256"]
            or type(control_budget["resource_profile"]) is not dict
            or set(control_budget["resource_profile"]) != set(resource_profile)
            or any(type(control_budget["resource_profile"][key]) is not int
                   or control_budget["resource_profile"][key] != value
                   for key, value in resource_profile.items())):
        raise Incomplete("Source-policy managed monitor controls cannot replace or restart the development allowance.")
    inputs.update({control_budget_path: control_pin["sha256"],
                   control_base / "monitor.py": pins["monitor_sha256"]})
    monitor = reference["independent_monitor"]
    monitor_fields = {"ok", "errors", "result", "exit_code", "joined", "result_sha256",
                      "resource_samples_sha256"}
    result_fields = {"status", "parent_pid", "parent_start", "observer_pid", "role", "recipe_sha256",
                     "samples", "maximum_gap_seconds", "finished_monotonic", "recipe_final_sha256"}
    if (type(monitor) is not dict or set(monitor) != monitor_fields
            or monitor["ok"] is not True or monitor["joined"] is not True or monitor["errors"] != []
            or type(monitor["exit_code"]) is not int or monitor["exit_code"] != 0
            or type(monitor["result"]) is not dict or set(monitor["result"]) != result_fields):
        raise Incomplete("Source-policy managed monitor lacks complete successful custody.")
    result = monitor["result"]
    if (result["status"] != "stopped" or result["role"] != "development"
            or result["recipe_sha256"] != pins["monitor_sha256"]
            or result["recipe_final_sha256"] != pins["monitor_sha256"]
            or any(type(result[key]) is not int or result[key] <= 0
                   for key in ("samples", "parent_pid", "parent_start", "observer_pid"))
            or result["parent_pid"] == result["observer_pid"]
            or any(type(result[key]) not in {int, float}
                   or not 0 <= result[key] <= sys.float_info.max or not math.isfinite(result[key])
                   for key in ("maximum_gap_seconds", "finished_monotonic"))
            or result["maximum_gap_seconds"] > 2 or result["finished_monotonic"] <= 0):
        raise Incomplete("Source-policy managed monitor identity or freshness differs.")
    directory = path.parent / "resource-monitor"
    if reference["durable_monitor"] != str(directory):
        raise Incomplete("Source-policy managed monitor leaves its fixed durable directory.")
    inputs.update({directory / "monitor-result.json": monitor["result_sha256"],
                   directory / "resources.jsonl": monitor["resource_samples_sha256"]})
    controls = SOURCE_POLICY_MANAGED_MONITOR_CONTROLS
    expected = {role + "/" + leaf for role in ("healthy", "gap")
                for leaf in ("control.json", "monitor-result.json", "resources.jsonl")}
    if type(controls) is not dict or set(controls) != expected:
        raise Incomplete("Source-policy managed monitor controls have not been independently pinned.")
    inputs.update({control_base / "monitor-controls" / name: digest for name, digest in controls.items()})
    raw = {}
    for filename, digest in inputs.items():
        if type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None:
            raise Incomplete("Source-policy managed physical evidence has malformed identities.")
        raw[filename] = COMMON.read_analysis_bytes(filename, maximum=4 * 1024**2)
        if hashlib.sha256(raw[filename]).hexdigest() != digest:
            raise Incomplete("Source-policy managed physical evidence is missing or changed.")
    terminal = read_json(directory / "monitor-result.json", expected_sha256=monitor["result_sha256"])
    if COMMON.canonical_sha256(terminal) != COMMON.canonical_sha256(result):
        raise Incomplete("Source-policy managed physical monitor terminal differs.")
    admit_source_policy_resource_samples(raw[directory / "resources.jsonl"], result)
    # The later controls need actual joined source closure, not expiry of its
    # allocated allowance. Resource elapsed coordinates retain the original
    # source clock; observer monotonic coordinates locate its final join gap.
    samples = [json.loads(line) for line in raw[directory / "resources.jsonl"].decode("utf-8").splitlines()]
    previous_elapsed = None
    for sample in samples:
        elapsed = sample.get("elapsed_seconds")
        if (type(elapsed) not in {int, float} or not 0 <= elapsed < 4200
                or not math.isfinite(elapsed)
                or (previous_elapsed is not None and elapsed < previous_elapsed)):
            raise Incomplete("Source-policy development samples exceed the original extended work clock.")
        previous_elapsed = elapsed
    final_sample = samples[-1]
    terminal_unix = (budget["start_unix"] + final_sample["elapsed_seconds"]
                     + (result["finished_monotonic"] - final_sample["monotonic_seconds"]))
    if closeout["closed_unix"] < terminal_unix:
        raise Incomplete("Source-policy development closeout precedes its actual joined monitor terminal.")
    for role in ("healthy", "gap"):
        admit_source_policy_monitor_control(control_base / "monitor-controls" / role, role,
                                            pins["monitor_sha256"], inputs, raw)
    for name, digest in captured.items():
        filename = path.parent / "source" / name
        if hashlib.sha256(COMMON.read_analysis_bytes(
                filename, maximum=COMMON.ANALYSIS_JSON_MAX_BYTES)).hexdigest() != digest:
            raise Incomplete("Source-policy managed frozen source is missing or changed.")
        inputs[filename] = digest
    return {str(filename): digest for filename, digest in inputs.items()}


def admit_source_policy_observer(reference: dict, path: Path) -> dict:
    """Admit one exclusive legacy or durable independent-monitor custody format."""
    if {"phase", "attempt", "captured_inputs", "durable_monitor"} & set(reference):
        return admit_source_policy_managed_observer(reference, path)
    legacy = {"resource_observer_joined", "terminal_resource_failures"}
    current = {"source_revision", "monitor_sha256", "monitor_control_inputs"}
    if not current & set(reference):
        # Historical receipts already carried independent_monitor diagnostics;
        # their accepted guarantee is the separately recorded flat custody pair.
        if (reference.get("resource_observer_joined") is not True
                or reference.get("terminal_resource_failures") != []):
            raise Incomplete("Source-policy development evidence lacks passing terminal custody.")
        return {}
    monitor = reference.get("independent_monitor")
    fields = {"ok", "errors", "result", "exit_code", "joined", "result_sha256", "resource_samples_sha256"}
    result_fields = {"status", "parent_pid", "parent_start", "observer_pid", "role", "recipe_sha256",
                     "samples", "maximum_gap_seconds", "finished_monotonic", "recipe_final_sha256"}
    if (not current <= set(reference) or legacy & set(reference)
            or type(monitor) is not dict or set(monitor) != fields
            or monitor["ok"] is not True or monitor["joined"] is not True or monitor["errors"] != []
            or type(monitor["exit_code"]) is not int or monitor["exit_code"] != 0
            or type(reference.get("source_revision")) is not int or reference["source_revision"] <= 0
            or type(monitor["result"]) is not dict or set(monitor["result"]) != result_fields):
        raise Incomplete("Source-policy independent monitor lacks exclusive passing terminal custody.")
    result = monitor["result"]
    hashes = (reference.get("monitor_sha256"), monitor["result_sha256"], monitor["resource_samples_sha256"],
              result["recipe_sha256"], result["recipe_final_sha256"])
    if (any(type(value) is not str or re.fullmatch(r"[a-f0-9]{64}", value) is None for value in hashes)
            or hashes[0] != hashes[3] or hashes[0] != hashes[4] or result["status"] != "stopped"
            or result["role"] != f"attempt-{reference['source_revision']}"
            or any(type(result[key]) is not int or result[key] <= 0
                   for key in ("samples", "parent_pid", "parent_start", "observer_pid"))
            or result["parent_pid"] == result["observer_pid"]
            or any(type(result[key]) not in {int, float} or not 0 <= result[key] <= sys.float_info.max
                   or not math.isfinite(result[key]) for key in ("maximum_gap_seconds", "finished_monotonic"))
            or result["maximum_gap_seconds"] > 2 or result["finished_monotonic"] <= 0):
        raise Incomplete("Source-policy independent monitor identity or clock bounds differ.")
    inputs = {path.parent.parent / "monitor.py": hashes[0],
              path.parent / "observer/monitor-result.json": hashes[1],
              path.parent / "observer/resources.jsonl": hashes[2]}
    controls = reference.get("monitor_control_inputs")
    expected = {(role, leaf) for role in ("healthy", "gap")
                for leaf in ("control.json", "monitor-result.json", "resources.jsonl")}
    if type(controls) is not dict or len(controls) != len(expected):
        raise Incomplete("Source-policy monitor controls are incomplete.")
    roots, received = set(), set()
    for name, digest in controls.items():
        if type(name) is not str or type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None:
            raise Incomplete("Source-policy monitor control identity is malformed.")
        declared = Path(name)
        if (not declared.is_absolute() or declared.as_posix() != name or ".." in declared.parts
                or len(declared.parts) != 6 or declared.parts[:3] != ("/", "dev", "shm")
                or re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]{0,127}", declared.parts[3]) is None
                or (declared.parts[-2], declared.name) not in expected):
            raise Incomplete("Source-policy monitor control leaves its fixed declared RAM layout.")
        roots.add(declared.parent.parent)
        received.add((declared.parent.name, declared.name))
        inputs[path.parent / "monitor-controls" / declared.parent.name / declared.name] = digest
    if len(roots) != 1 or received != expected:
        raise Incomplete("Source-policy monitor control roles or parent differ.")
    raw = {}
    for filename, digest in inputs.items():
        raw[filename] = COMMON.read_analysis_bytes(filename, maximum=4 * 1024**2)
        if hashlib.sha256(raw[filename]).hexdigest() != digest:
            raise Incomplete("Source-policy physical monitor evidence is missing or changed.")
    terminal = read_json(path.parent / "observer/monitor-result.json", expected_sha256=hashes[1])
    if COMMON.canonical_sha256(terminal) != COMMON.canonical_sha256(result):
        raise Incomplete("Source-policy physical monitor terminal result differs.")
    admit_source_policy_resource_samples(raw[path.parent / "observer/resources.jsonl"], result)
    for role in ("healthy", "gap"):
        admit_source_policy_monitor_control(path.parent / "monitor-controls" / role, role, hashes[0], inputs, raw)
    return {str(filename): digest for filename, digest in inputs.items()}


def source_policy_combined_evidence(candidate: str) -> tuple[str, str]:
    """Select the pinned manifest and raw boundary for this combined candidate."""
    if candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE:
        return ENGINE_JOINED_RECEIVING_BOUNDARY_MANIFEST, ENGINE_JOINED_RECEIVING_BOUNDARY_CONTROL
    if candidate == ENGINE_JOINED_RECEIVING_CANDIDATE:
        return ENGINE_JOINED_RECEIVING_MANIFEST, ENGINE_JOINED_RECEIVING_BOUNDARY
    if candidate == ENGINE_STALE_DEBIT_CANDIDATE:
        return (ENGINE_STALE_DEBIT_MANIFEST, ENGINE_STALE_DEBIT_BOUNDARY)
    if candidate == ENGINE_FINAL_FRAME_CANDIDATE:
        return (ENGINE_FINAL_FRAME_MANIFEST, ENGINE_FINAL_FRAME_BOUNDARY)
    if candidate == ENGINE_PROTECTED_DEBIT_CANDIDATE:
        return (ENGINE_PROTECTED_DEBIT_MANIFEST, ENGINE_PROTECTED_DEBIT_BOUNDARY)
    if candidate == ENGINE_PLACEMENT_CANDIDATE:
        return ENGINE_PLACEMENT_MANIFEST, ENGINE_PLACEMENT_BOUNDARY
    if candidate == ENGINE_SCALE_CONFLICT_CANDIDATE:
        return ENGINE_SCALE_CONFLICT_MANIFEST, ENGINE_SCALE_CONFLICT_BOUNDARY
    if candidate == ENGINE_REVIEW_CANDIDATE:
        return ENGINE_REVIEW_MANIFEST, ENGINE_REVIEW_BOUNDARY
    if candidate == SOURCE_POLICY_SOURCE_SCALE_CANDIDATE:
        return SOURCE_POLICY_SOURCE_SCALE_MANIFEST, SOURCE_POLICY_SOURCE_SCALE_BOUNDARY
    if candidate == SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_CANDIDATE:
        return SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_MANIFEST, SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_BOUNDARY
    if candidate == SOURCE_POLICY_APPLICATION_PLACEMENT_CANDIDATE:
        return SOURCE_POLICY_APPLICATION_PLACEMENT_MANIFEST, SOURCE_POLICY_APPLICATION_PLACEMENT_BOUNDARY
    if candidate == SOURCE_POLICY_PROTECTED_ORDER_CANDIDATE:
        return SOURCE_POLICY_PROTECTED_ORDER_MANIFEST, SOURCE_POLICY_PROTECTED_ORDER_BOUNDARY
    if candidate == SOURCE_POLICY_HEADER_AGE_CANDIDATE:
        return SOURCE_POLICY_HEADER_AGE_MANIFEST, SOURCE_POLICY_HEADER_AGE_BOUNDARY
    if candidate == SOURCE_POLICY_DEBIT_CANDIDATE:
        return SOURCE_POLICY_DEBIT_MANIFEST, SOURCE_POLICY_DEBIT_BOUNDARY
    if candidate == SOURCE_POLICY_CHRONOLOGY_CANDIDATE:
        return SOURCE_POLICY_CHRONOLOGY_MANIFEST, SOURCE_POLICY_CHRONOLOGY_BOUNDARY
    if candidate == SOURCE_POLICY_RECEIVE_CANDIDATE:
        return SOURCE_POLICY_RECEIVE_MANIFEST, SOURCE_POLICY_RECEIVE_BOUNDARY
    return SOURCE_POLICY_CAPTURE_MANIFEST, SOURCE_POLICY_CAPTURE_BOUNDARY


def validate_source_policy_capture_lane(row: dict, selected: dict, raw: dict, binding: dict,
                                        sandbox_prefix: list[str]) -> None:
    """Match the observed invocation and complete test blocks to its captured lane."""
    lane = selected["label"]
    command = raw.get("command")
    output = raw.get("output")
    if (type(row.get("exit_code")) is not int or row["exit_code"] != 0
            or "failure" not in row or row["failure"] is not None
            or type(raw.get("exit_code")) is not int or raw["exit_code"] != 0
            or "failure" not in raw or raw["failure"] is not None
            or type(output) is not str or type(command) is not list
            or type(raw.get("sandbox_command")) is not list):
        raise Incomplete("Source-policy capture development raw lane is failed or incomplete.")
    if not command or raw["sandbox_command"] != [*sandbox_prefix, "--", *command]:
        raise Incomplete("Source-policy capture development effective sandbox command differs.")
    if lane == "boundary-control":
        _, boundary = source_policy_combined_evidence(binding["candidate_sha256"])
        if (row["sha256"] != boundary
                or len(command) != 3 or command[:2] != selected["command"][:2]
                or output.splitlines().count("ANALYSIS_BOUNDARY_OK") != 1):
            raise Incomplete("Source-policy capture development lacks its actual namespace boundary control.")
    elif command != selected["command"]:
        raise Incomplete("Source-policy capture development raw lane changes its captured command.")
    if lane in {"default-ordinary", "default-optimized", "ep1-ordinary", "ep1-optimized"}:
        validate = source_policy_execution if lane.startswith("default-") else source_policy_ep1_execution
        facts = validate(output, binding)
        cases = engine_ke_development_ordinary_cases(output)
        pairs = sorted((case["binary"], case["name"]) for case in cases)
        candidate = binding["candidate_sha256"]
        expected_digests = {
            ENGINE_SCALE_CONFLICT_CANDIDATE: ENGINE_SCALE_CONFLICT_EP1_CASE_DIGEST,
            ENGINE_PLACEMENT_CANDIDATE: ENGINE_PLACEMENT_EP1_CASE_DIGEST,
            ENGINE_PROTECTED_DEBIT_CANDIDATE: ENGINE_PROTECTED_DEBIT_EP1_CASE_DIGEST,
            ENGINE_FINAL_FRAME_CANDIDATE: ENGINE_FINAL_FRAME_EP1_CASE_DIGEST,
            ENGINE_STALE_DEBIT_CANDIDATE: ENGINE_STALE_DEBIT_EP1_CASE_DIGEST,
        }
        expected_digest: str | None = None
        if candidate in ENGINE_JOINED_RECEIVING_FAMILY:
            profile: str = lane.split("-", 1)[0]
            expected_digest = (ENGINE_JOINED_RECEIVING_BOUNDARY_EP1_CASE_DIGEST
                               if profile == "ep1" and candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE
                               else ENGINE_JOINED_RECEIVING_CASE_DIGESTS[profile])
        elif candidate in expected_digests:
            expected_digest = (expected_digests[candidate] if lane.startswith("ep1-")
                               else ENGINE_SCALE_CONFLICT_DEFAULT_CASE_DIGEST)
        if expected_digest is not None and COMMON.canonical_sha256(pairs) != expected_digest:
            raise Incomplete("Engine combined ordinary execution differs by binary.")
        if (Counter(pairs) != Counter(map(tuple, selected["expected"]))
                or cases != row.get("cases") or facts["tests_executed"] != row.get("tests_executed")):
            raise Incomplete("Source-policy capture development ordinary identities differ by binary.")
    elif lane in {"default-doctests", "ep1-doctests"}:
        facts = source_policy_development_doctests(output, binding, lane)
        if (sorted(facts["test_names"]) != sorted(selected["expected"])
                or facts["test_names"] != row.get("cases") or facts["tests_executed"] != row.get("tests_executed")):
            raise Incomplete("Source-policy capture development Rustdoc identities differ.")
    elif selected["kind"] == "core":
        blocks = session_execution_blocks(output)
        if len(blocks) != 1:
            raise Incomplete("Source-policy capture development needs one selected Core test block.")
        running, block, status, passed, failed, ignored, measured, _filtered = blocks[0].groups()
        names = re.findall(r"(?m)^test ([^\r\n]+?) \.\.\. ok$", block)
        test_rows = [line for line in output.splitlines()
                     if line.startswith("test ") and not line.startswith("test result:")]
        cases = engine_ke_development_ordinary_cases(output)
        if (status != "ok" or any(map(int, (failed, ignored, measured)))
                or int(running) != len(names) or int(passed) != len(names)
                or test_rows != [f"test {name} ... ok" for name in names]
                or [case["name"] for case in cases] != names
                or Counter((case["binary"], case["name"]) for case in cases)
                != Counter(map(tuple, selected["expected"]))
                or names != row.get("cases") or len(names) != row.get("tests_executed")):
            raise Incomplete("Source-policy capture development Core identities differ by binary.")


def admit_source_policy_receive_focus(capture_path: Path, capture: dict, reference: dict) -> dict:
    """Reconstruct the focused path registry from pinned receipts and read every referenced lane."""
    focus_path = capture_path.with_name("focused-index.json")
    focus = read_json(focus_path, maximum=1024**2, expected_sha256=SOURCE_POLICY_RECEIVE_FOCUSED,
                      owned_evidence=True)
    invocation = reference.get("invocation_id")
    if (reference.get("focused_evidence_index_sha256") != SOURCE_POLICY_RECEIVE_FOCUSED
            or type(invocation) is not str or re.fullmatch(r"[a-f0-9]{32}", invocation) is None
            or type(focus) is not dict or set(focus) != {"plain", "protected"}
            or any(type(refs) is not list or len(refs) != 2 for refs in focus.values())):
        raise Incomplete("Source-policy receive development lacks its exact focused lineage.")
    admitted = {str(focus_path): SOURCE_POLICY_RECEIVE_FOCUSED}
    for prior in (*capture["prior_plain_failed_development"], capture["plain_red_focused"]):
        path = Path(prior["path"])
        read_json(path, maximum=4 * 1024**2, expected_sha256=prior["sha256"], owned_evidence=True)
        admitted[str(path)] = prior["sha256"]
    expected = (
        (focus["plain"][0], "incomplete", "green", capture["expected_plain_candidate_sha256"]),
        (focus["plain"][1], "passed", "green", capture["expected_plain_candidate_sha256"]),
        (focus["protected"][0], "passed", "red", capture["expected_protected_red_candidate_sha256"]),
        (focus["protected"][1], "passed", "green", capture["expected_protected_candidate_sha256"]),
    )
    for record, status, mode, candidate in expected:
        path = Path(record["path"])
        stamp = record["sha256"]
        report = read_json(path, maximum=4 * 1024**2, expected_sha256=stamp, owned_evidence=True)
        if (report.get("status") != status or report.get("mode") != mode
                or report.get("candidate_sha256") != candidate
                or report.get("baseline_report_sha256") != capture["combined_development_reference"]["sha256"]
                or (status == "passed" and report.get("temporary_work_removed") is not True)):
            raise Incomplete("Source-policy receive focused terminal identity differs.")
        admitted[str(path)] = stamp
        for row in report["lanes"]:
            if type(row.get("lane")) is not str or re.fullmatch(r"[a-z0-9-]+", row["lane"]) is None:
                raise Incomplete("Source-policy receive focused lane leaves its receipt directory.")
            lane = path.with_name(row["lane"] + ".json")
            raw = read_json(lane, maximum=4 * 1024**2, expected_sha256=row["sha256"], owned_evidence=True)
            if (type(row.get("exit_code")) is not int or type(raw.get("exit_code")) is not int
                    or raw["exit_code"] != row["exit_code"] or raw.get("failure") != row.get("failure")):
                raise Incomplete("Source-policy receive focused lane summary differs from its raw result.")
            admitted[str(lane)] = row["sha256"]
    if reference.get("focused_evidence") != admitted:
        raise Incomplete("Source-policy receive focused path registry differs.")
    return admitted


def admit_source_policy_exact_focus(reference: dict, expected_count: int, expected_digest: str) -> dict:
    """Rehash every file in the exact focused evidence registry."""
    focused = reference.get("focused_evidence")
    invocation = reference.get("invocation_id")
    if (type(focused) is not dict or len(focused) != expected_count
            or type(invocation) is not str or re.fullmatch(r"[a-f0-9]{32}", invocation) is None
            or reference.get("focused_evidence_index_sha256") != expected_digest
            or COMMON.canonical_sha256(focused) != expected_digest):
        raise Incomplete("Source-policy exact focused evidence or invocation differs.")
    for name, digest in focused.items():
        raw = COMMON.read_analysis_bytes(Path(name), maximum=8 * 1024**2)
        if hashlib.sha256(raw).hexdigest() != digest:
            raise Incomplete("Source-policy exact focused evidence bytes differ.")
    return focused


def admit_source_policy_chronology_focus(reference: dict) -> dict:
    """Verify the pinned focused chronology history."""
    return admit_source_policy_exact_focus(reference, 74, SOURCE_POLICY_CHRONOLOGY_FOCUSED)


def admit_source_policy_capture_development(binding: dict, identity: dict, path: Path) -> dict:
    """Admit the exact combined capture producer and physical lane receipts."""
    evidence_sha = binding["development_evidence"]["sha256"]
    candidate = binding["candidate_sha256"]
    receive = candidate == SOURCE_POLICY_RECEIVE_CANDIDATE
    chronology = candidate == SOURCE_POLICY_CHRONOLOGY_CANDIDATE
    debit = candidate == SOURCE_POLICY_DEBIT_CANDIDATE
    header_age = candidate == SOURCE_POLICY_HEADER_AGE_CANDIDATE
    protected_order = candidate == SOURCE_POLICY_PROTECTED_ORDER_CANDIDATE
    application_placement = candidate == SOURCE_POLICY_APPLICATION_PLACEMENT_CANDIDATE
    receive_effect_guards = candidate == SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_CANDIDATE
    full_capture = header_age or protected_order or application_placement or receive_effect_guards
    report_directory = "full-reports" if full_capture else "reports"
    if len(path.parents) < 3 or path.name != "verification.json" or path.parents[1].name != report_directory:
        raise Incomplete("Source-policy capture development must bind a combined terminal receipt.")
    capture_filename = "candidate-retry-v1.json" if chronology else "candidate.json"
    if full_capture:
        capture_filename = "full-candidate.json"
    capture_path = path.parents[2] / capture_filename
    manifest, boundary_sha = source_policy_combined_evidence(candidate)
    capture = read_json(capture_path, maximum=1024**2, expected_sha256=manifest,
                        owned_evidence=True)
    reference = read_json(path, maximum=8 * 1024**2, expected_sha256=evidence_sha, owned_evidence=True)
    elapsed = reference.get("elapsed_seconds")
    peak = reference.get("peak_memory_bytes")
    memory_events = reference.get("memory_events", {})
    if (capture["candidate_sha256"] != candidate
            or reference.get("status") != "passed" or reference.get("mode") != "combined"
            or reference.get("candidate_sha256") != candidate
            or reference.get("capture_sha256") != manifest
            or reference.get("inputs") != identity or capture["inputs"] != identity
            or reference.get("protected") != capture["protected"]
            or reference.get("main_admission") != {
                "posture": "verified-staged-capture-base", "head": capture["head"],
                "index_sha256": capture["index_sha256"], "tree": capture["staged_tree"]}
            or reference.get("compiler") != capture["compiler"]
            or reference.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(reference.get("vendor_identity")) != capture["vendor_identity_sha256"]
            or reference.get("cargo_home_config_sha256") != capture["cargo_home_config_sha256"]
            or reference.get("reference_sha256") != capture["combined_development_reference"]["sha256"]
            or reference.get("prior_failed_development_sha256", "") != (
                SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_PREVIOUS_FULL_FAILURE if receive_effect_guards else
                None if application_placement else
                SOURCE_POLICY_PROTECTED_ORDER_FOCUSED_FAILURE if protected_order else
                SOURCE_POLICY_HEADER_AGE_FOCUSED_FAILURE if header_age else
                SOURCE_POLICY_DEBIT_FOCUSED_FAILURE if debit else
                SOURCE_POLICY_CHRONOLOGY_PARTIAL if chronology else capture["previous_focused_failure"]["sha256"])
            or reference.get("repair_patch_sha256") != capture["patches"][-1]["sha256"]
            or reference.get("native_windows_execution") != "not performed"
            or reference.get("metrics") != "not performed"
            or reference.get("resource_observer_joined") is not True
            or reference.get("owned_work_allocated") is not True or reference.get("owned_work_retired") is not True
            or reference.get("tasks_events") != "max 0"
            or type(memory_events) is not dict
            or any(memory_events.get(key) != "0" for key in ("max", "oom", "oom_kill", "oom_group_kill"))
            or type(elapsed) not in (int, float) or not 0 < elapsed <= 2400
            or type(peak) is not int or not 0 < peak <= 6 * 1024**3
            or "active_lane" in reference or "error" in reference):
        raise Incomplete("Source-policy capture development lacks exact passing terminal custody.")

    if chronology and (
            reference.get("first_full_failure") != capture["previous_full_failure"]
            or reference.get("focus_borrow_only") != capture["focus_borrow_only"]
            or reference.get("focused_observation_candidate_sha256") != capture["original_candidate_sha256"]
            or capture["focus_borrow_only"]["focused_rerun_claimed"] is not False
            or not 0 < elapsed <= capture["wall_seconds"] == 2377):
        raise Incomplete("Source-policy chronology borrow-only retry custody differs.")
    if debit and reference.get("focused_reference_tree") != capture["focused_reference_tree"]:
        raise Incomplete("Source-policy protected receive debit focused stage differs.")
    if protected_order and (
            reference.get("focused_earlier_tree") != SOURCE_POLICY_PROTECTED_ORDER_EARLIER_TREE
            or reference.get("focused_red_preimage_bridge") != capture["focused_red_preimage_bridge"]):
        raise Incomplete("Source-policy ordinary protected receive focused predecessor differs.")

    if application_placement and (reference.get("focused_earlier_tree") != capture["staged_tree"]
            or reference.get("focused_red_preimage_bridge") != capture["focused_red_preimage_bridge"]
            or reference.get("rustdoc_line_changes") != capture["rustdoc_line_changes"]):
        raise Incomplete("Source-policy application placement source anchors differ.")

    if receive_effect_guards and (
            reference.get("focused_earlier_tree") != capture["staged_tree"]
            or reference.get("focused_references") != capture["focused"]
            or reference.get("focused_red_preimage_bridge") != capture["focused_red_preimage_bridge"]
            or reference.get("rustdoc_line_changes") != capture["rustdoc_line_changes"]):
        raise Incomplete("Source-policy receive effect guards focused lineage differs.")

    lanes = reference.get("lanes")
    recipe = capture["lanes"]
    if (type(lanes) is not list or len(lanes) != 19 or len(recipe) != 19
            or any(type(row) is not dict for row in lanes)
            or [row.get("lane") for row in lanes] != [row["label"] for row in recipe]
            or reference.get("cache_retirement") != [row["label"] for row in recipe if row["retire"]]):
        raise Incomplete("Source-policy capture development requires all 19 physical lanes.")
    boundary = read_json(path.with_name("boundary-control.json"), maximum=4 * 1024**2,
                         expected_sha256=boundary_sha, owned_evidence=True)
    sandbox_prefix = boundary["sandbox_command"][:-len(boundary["command"])-1]
    admitted = {str(path): evidence_sha, str(capture_path): manifest}
    if chronology:
        admitted.update(admit_source_policy_chronology_focus(reference))
    elif debit:
        admitted.update(admit_source_policy_exact_focus(
            reference, SOURCE_POLICY_DEBIT_FOCUSED_COUNT, SOURCE_POLICY_DEBIT_FOCUSED))
    elif header_age:
        admitted.update(admit_source_policy_exact_focus(
            reference, SOURCE_POLICY_HEADER_AGE_FOCUSED_COUNT, SOURCE_POLICY_HEADER_AGE_FOCUSED))
    elif receive_effect_guards:
        admitted.update(admit_source_policy_exact_focus(
            reference, SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_FOCUSED_COUNT,
            SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_FOCUSED))
    elif application_placement:
        admitted.update(admit_source_policy_exact_focus(
            reference, SOURCE_POLICY_APPLICATION_PLACEMENT_FOCUSED_COUNT, SOURCE_POLICY_APPLICATION_PLACEMENT_FOCUSED))
    elif protected_order:
        admitted.update(admit_source_policy_exact_focus(
            reference, SOURCE_POLICY_PROTECTED_ORDER_FOCUSED_COUNT, SOURCE_POLICY_PROTECTED_ORDER_FOCUSED))
    elif receive:
        admitted.update(admit_source_policy_receive_focus(capture_path, capture, reference))
    return {**admitted, **admit_source_policy_capture_rows(path, lanes, recipe, binding, sandbox_prefix)}


def admit_source_policy_capture_rows(path: Path, lanes: list, recipe: list, binding: dict,
                                     sandbox_prefix: list[str]) -> dict:
    """Read and validate each physical lane against the admitted ordered recipe."""
    admitted = {}
    for row, selected in zip(lanes, recipe, strict=True):
        lane = selected["label"]
        digest = row.get("sha256")
        if type(digest) is not str or re.fullmatch(r"[a-f0-9]{64}", digest) is None:
            raise Incomplete("Source-policy capture development lane lacks its physical receipt hash.")
        raw_path = path.with_name(lane + ".json")
        raw = read_json(raw_path, maximum=4 * 1024**2, expected_sha256=digest, owned_evidence=True)
        validate_source_policy_capture_lane(row, selected, raw, binding, sandbox_prefix)
        admitted[str(raw_path)] = digest
    return admitted


def admit_source_policy_scale_lineage(capture: dict, reference: dict, identity: dict) -> dict:
    """Verify the actual scale overlay, failed focused run, and retained graph lineage."""
    admitted, lineage = {}, {}
    for key in ("combined_development_reference", "focused_incomplete_reference", "predecessor_graph"):
        pinned = capture[key]
        prior_path = Path(pinned["path"])
        lineage[key] = read_json(prior_path, maximum=8 * 1024**2, expected_sha256=pinned["sha256"],
                                 owned_evidence=True)
        admitted[str(prior_path)] = pinned["sha256"]
    focus_ref = capture["focused_incomplete_reference"]
    focus = lineage["focused_incomplete_reference"]
    focus_patch = Path(focus_ref["path"]).with_name("source.patch")
    patch_digest = hashlib.sha256(COMMON.read_analysis_bytes(focus_patch, maximum=1024**2)).hexdigest()
    prior = lineage["combined_development_reference"]
    previous = prior["inputs"]
    graph = lineage["predecessor_graph"]
    if (reference.get("prior_failed_development_sha256") != focus_ref["sha256"]
            or reference.get("repair_patch_sha256") != patch_digest
            or reference.get("overlay_patch_sha256") != patch_digest or focus["patch_sha256"] != patch_digest
            or focus["status"] != "incomplete" or focus["owned_scratch_retired"] is not True
            or focus["candidate_sha256"] != capture["candidate_sha256"] or focus["inputs"] != identity
            or reference.get("reference_sha256") != capture["combined_development_reference"]["sha256"]
            or reference.get("predecessor_graph") != capture["predecessor_graph"]
            or prior["status"] != "passed" or graph["status"] != "passed"
            or graph["product_input_sha256"] != previous or graph["product_digest"] != prior["candidate_sha256"]
            or identity.keys() != previous.keys()
            or {name for name, digest in identity.items() if digest != previous[name]} != SOURCE_POLICY_SOURCE_SCALE_OVERLAY):
        raise Incomplete("Source-scale development focused overlay or predecessor lineage differs.")
    admitted[str(focus_patch)] = patch_digest
    return admitted


def admit_engine_review_lineage(capture: dict, terminal: dict, identity: dict) -> dict:
    """Keep the older workload, focused producer and graph under their original identities."""
    admitted, references = {}, {}
    for label in ("combined_development_reference", "focused_reference", "predecessor_graph"):
        binding = capture[label]
        path = Path(binding["path"])
        references[label] = read_json(path, maximum=8 * 1024**2, expected_sha256=binding["sha256"],
                                      owned_evidence=True)
        admitted[str(path)] = binding["sha256"]
    prior = references["combined_development_reference"]
    focus = references["focused_reference"]
    graph = references["predecessor_graph"]
    predecessor = prior.get("inputs")
    if (prior.get("status") != "passed" or prior.get("mode") != "combined"
            or prior.get("candidate_sha256") != ENGINE_REVIEW_PRIOR or type(predecessor) is not dict
            or COMMON.canonical_sha256(predecessor) != ENGINE_REVIEW_PRIOR
            or not predecessor.keys() <= identity.keys()
            or COMMON.canonical_sha256(sorted(name for name, digest in identity.items()
                                              if predecessor.get(name) != digest)) != ENGINE_REVIEW_PRIOR_DELTA
            or terminal.get("reference_sha256") != capture["combined_development_reference"]["sha256"]
            or terminal.get("focused_development_sha256") != capture["focused_reference"]["sha256"]
            or focus.get("status") != "passed-review-gnu-msvc-development"
            or focus.get("owned_scratch_retired") is not True or focus.get("inputs") != identity
            or focus.get("candidate_sha256") != ENGINE_REVIEW_CANDIDATE
            or focus.get("tool_inventories") != capture["tool_inventories"]
            or prior.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(prior.get("vendor_identity")) != capture["vendor_identity_sha256"]
            or focus.get("vendor_sha256") != capture["vendor_identity_sha256"]):
        raise Incomplete("Engine review baseline and focused source/tool lineage differs.")

    admitted.update(admit_engine_review_graph(capture, terminal, identity, graph, ENGINE_REVIEW_GRAPH_DELTA))
    return admitted


def admit_engine_review_graph(capture: dict, terminal: dict, identity: dict, graph: dict,
                              delta_digest: str) -> dict:
    """Read the original 45 dependency lanes and four projections for the exact reuse delta."""
    admitted = {}
    graph_ref = capture["predecessor_graph"]
    graph_inputs = graph.get("product_input_sha256")
    lanes, projections = graph.get("lanes"), graph.get("selected_graph_test_only_delta")
    if (terminal.get("predecessor_graph") != graph_ref
            or graph_ref["sha256"] != ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256
            or type(graph_inputs) is not dict or not graph_inputs.keys() <= identity.keys()
            or graph.get("product_digest") != ENGINE_KE_SOURCE_SCALE_GRAPH_PREDECESSOR
            or COMMON.canonical_sha256(graph_inputs) != ENGINE_KE_SOURCE_SCALE_GRAPH_PREDECESSOR
            or graph.get("status") != "passed" or graph.get("changed_tree_outputs") != []
            or graph.get("lockfile_sha256") != identity.get("Cargo.lock")
            or COMMON.canonical_sha256(sorted(name for name, digest in identity.items()
                                              if graph_inputs.get(name) != digest)) != delta_digest
            or type(lanes) is not list or len(lanes) != 45
            or type(graph.get("rows")) is not list or len(graph["rows"]) != 4
            or type(projections) is not list or len(projections) != 4):
        raise Incomplete("Engine review predecessor graph or unchanged-input reuse boundary differs.")
    directory = Path(graph_ref["path"]).parent
    graph_files = {}
    for lane in lanes:
        if type(lane.get("exit_code")) is not int or lane["exit_code"] != 0:
            raise Incomplete("Engine review predecessor graph lane did not pass.")
        name = lane["profile"] + "--" + lane["label"] + ".json"
        graph_files[name] = lane["sha256"]
    graph_files.update((row["file"], row["sha256"]) for row in projections)
    if len(graph_files) != 49 or any(re.fullmatch(r"[a-z0-9_-]+\.json", name) is None for name in graph_files):
        raise Incomplete("Engine review predecessor graph physical roster differs.")
    for name, digest in graph_files.items():
        path = directory / name
        read_json(path, maximum=4 * 1024**2, expected_sha256=digest, owned_evidence=True)
        admitted[str(path)] = digest
    return admitted


def validate_engine_placement_retry(capture: dict, terminal: dict) -> dict:
    """Keep the exact formatter retry within the original combined allowance."""
    retry = capture.get("placement_format_retry")
    first_ms = ENGINE_PLACEMENT_FIRST_WALL_MS
    retry_seconds = ENGINE_PLACEMENT_RETRY_WALL
    cumulative_ms = first_ms + 1000 * retry_seconds
    references = retry.get("references") if type(retry) is dict else None
    if (type(references) is not dict or len(references) != 6
            or COMMON.canonical_sha256(references) != ENGINE_PLACEMENT_FORMAT_REFERENCES
            or retry != terminal.get("placement_format_retry")
            or retry != {"references": references, "first_outer_seconds": first_ms / 1000,
                         "retry_wall_seconds": retry_seconds, "original_wall_seconds": 2400,
                         "cumulative_wall_seconds_ceiling": cumulative_ms / 1000}
            or cumulative_ms > 2400 * 1000 or capture.get("wall_seconds") != retry_seconds
            or terminal.get("prior_failed_development_sha256") != ENGINE_PLACEMENT_STOPPED):
        raise Incomplete("Engine placement formatter retry custody or cumulative allowance differs.")
    return references


def admit_engine_placement_lineage(capture: dict, terminal: dict, identity: dict, path: Path) -> dict:
    """Read the actual format stop, successful retry and original graph dependency closure."""
    retry = validate_engine_placement_retry(capture, terminal)
    focused, inherited = capture.get("references"), capture.get("prior_combined_retry")
    if (type(focused) is not dict or len(focused) != 18 or terminal.get("focused_references") != focused
            or COMMON.canonical_sha256(focused) != ENGINE_PLACEMENT_FOCUS_REFERENCES
            or type(inherited) is not dict or len(inherited) != 4 or terminal.get("prior_combined_retry") != inherited
            or COMMON.canonical_sha256(inherited) != ENGINE_PLACEMENT_INHERITED_RETRY
            or any(focused.get(name) != digest for name, digest in retry.items())):
        raise Incomplete("Engine placement retained producer and retry reference set differs.")
    wrapper = path.parents[2] / "run-combined.py"
    if (capture.get("wrapper_sha256") != ENGINE_PLACEMENT_WRAPPER
            or hashlib.sha256(COMMON.read_analysis_bytes(wrapper, maximum=1024**2)).hexdigest()
                != ENGINE_PLACEMENT_WRAPPER):
        raise Incomplete("Engine placement wrapper bytes differ.")

    admitted = {**focused, **inherited}
    records: dict[str, dict] = {}
    for name, digest in admitted.items():
        member = Path(name)
        if not member.is_absolute():
            raise Incomplete("Engine placement lineage requires absolute evidence paths.")
        if member.suffix == ".json" and member.name != "launch-command.json":
            record = read_json(member, maximum=8 * 1024**2, expected_sha256=digest, owned_evidence=True)
            records[digest] = record
        elif (member.suffix not in {".py", ".patch"} and member.name != "launch-command.json"
              or hashlib.sha256(COMMON.read_analysis_bytes(member, maximum=1024**2)).hexdigest() != digest):
            raise Incomplete("Engine placement retained recipe or patch bytes differ.")
    admitted[str(wrapper)] = ENGINE_PLACEMENT_WRAPPER

    prior = records[ENGINE_SCALE_CONFLICT_TERMINAL]
    previous = prior.get("inputs")
    if (terminal.get("reference_sha256") != ENGINE_SCALE_CONFLICT_TERMINAL
            or prior.get("status") != "passed" or prior.get("mode") != "combined"
            or prior.get("candidate_sha256") != ENGINE_SCALE_CONFLICT_CANDIDATE
            or type(previous) is not dict or previous.keys() != identity.keys()
            or COMMON.canonical_sha256(previous) != ENGINE_SCALE_CONFLICT_CANDIDATE
            or COMMON.canonical_sha256(sorted(name for name in identity if identity[name] != previous[name]))
                != ENGINE_PLACEMENT_PRIOR_DELTA
            or prior.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(prior.get("vendor_identity")) != capture["vendor_identity_sha256"]):
        raise Incomplete("Engine placement exact predecessor or source delta differs.")

    stopped = records[ENGINE_PLACEMENT_STOPPED]
    first_capture = records[ENGINE_PLACEMENT_UNFORMATTED_MANIFEST]
    first_inputs = first_capture.get("inputs")
    boundary = records[ENGINE_PLACEMENT_FIRST_BOUNDARY]
    format_failure = records[ENGINE_PLACEMENT_FORMAT_FAILURE]
    first_lanes = [
        {"lane": "boundary-control", "sha256": ENGINE_PLACEMENT_FIRST_BOUNDARY, "exit_code": 0, "failure": None},
        {"lane": "format", "sha256": ENGINE_PLACEMENT_FORMAT_FAILURE, "exit_code": 1, "failure": None},
    ]
    if (stopped.get("status") != "incomplete" or stopped.get("mode") != "combined"
            or stopped.get("active_lane") != "format" or stopped.get("cache_retirement") != []
            or stopped.get("candidate_sha256") != ENGINE_PLACEMENT_UNFORMATTED
            or stopped.get("capture_sha256") != ENGINE_PLACEMENT_UNFORMATTED_MANIFEST
            or stopped.get("inputs") != first_inputs or type(first_inputs) is not dict
            or first_capture.get("candidate_sha256") != ENGINE_PLACEMENT_UNFORMATTED
            or first_capture.get("wall_seconds") != 2400 or first_inputs.keys() != identity.keys()
            or COMMON.canonical_sha256(first_inputs) != ENGINE_PLACEMENT_UNFORMATTED
            or {name for name in identity if identity[name] != first_inputs[name]} != {ENGINE_SCALE_CONFLICT_NATIVE_FIXTURE}
            or stopped.get("staged_index_sha256") != capture["index_sha256"]
            or stopped.get("staged_tree") != capture["staged_tree"]
            or stopped.get("owned_work_retired") is not True or stopped.get("resource_observer_joined") is not True
            or stopped.get("public_campaigns_admitted") is not False or first_capture.get("campaigns_admitted") is not False
            or stopped.get("lanes") != first_lanes
            or boundary.get("exit_code") != 0 or boundary.get("failure") is not None
            or boundary.get("output", "").splitlines().count("ANALYSIS_BOUNDARY_OK") != 1
            or format_failure.get("exit_code") != 1 or format_failure.get("failure") is not None
            or format_failure.get("command") != ["cargo", "fmt", "--all", "--", "--check"]
            or re.findall(r"^Diff in /work/source/([^:]+):([0-9]+):$", format_failure.get("output", ""), re.M)
                != [(ENGINE_SCALE_CONFLICT_NATIVE_FIXTURE, line) for line in ("13", "238", "306")]):
        raise Incomplete("Engine placement first attempt must retain its exact incomplete formatting outcome.")

    graph_ref = capture["predecessor_graph"]
    if focused.get(graph_ref["path"]) != graph_ref["sha256"]:
        raise Incomplete("Engine placement predecessor graph reference differs.")
    graph = records[graph_ref["sha256"]]
    graph_dependencies = graph["manifest_sha256"] | graph["checkers"]
    if any(capture["source"].get(name) != digest for name, digest in graph_dependencies.items()):
        raise Incomplete("Engine placement predecessor graph inputs changed.")
    admitted.update(admit_engine_review_graph(capture, terminal, identity, graph, ENGINE_SCALE_CONFLICT_GRAPH_DELTA))
    return admitted


def admit_engine_protected_debit_lineage(capture: dict, terminal: dict, identity: dict,
                                          path: Path) -> dict:
    """Reconcile the new receipt with the closed placement graph and exact debit evidence."""
    references = (
        ("combined_development_reference", ENGINE_PLACEMENT_TERMINAL),
        ("focused_reference", ENGINE_PROTECTED_DEBIT_FOCUSED),
        ("predecessor_graph", ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256),
    )
    admitted: dict[str, str] = {}
    records: dict[str, dict] = {}
    for label, expected in references:
        reference = capture.get(label)
        if (type(reference) is not dict or set(reference) != {"path", "sha256"}
                or reference["sha256"] != expected or type(reference["path"]) is not str
                or not Path(reference["path"]).is_absolute()):
            raise Incomplete("Engine protected-debit predecessor reference differs.")
        member = Path(reference["path"])
        records[label] = read_json(member, maximum=8 * 1024**2,
                                   expected_sha256=expected, owned_evidence=True)
        admitted[str(member)] = expected

    prior = records["combined_development_reference"]
    focus = records["focused_reference"]
    graph = records["predecessor_graph"]
    previous = prior.get("inputs")
    if (prior.get("status") != "passed" or prior.get("mode") != "combined"
            or prior.get("candidate_sha256") != ENGINE_PLACEMENT_CANDIDATE
            or type(previous) is not dict or previous.keys() != identity.keys()
            or COMMON.canonical_sha256(previous) != ENGINE_PLACEMENT_CANDIDATE
            or prior.get("predecessor_graph") != capture["predecessor_graph"]
            or terminal.get("reference_sha256") != ENGINE_PLACEMENT_TERMINAL
            or prior.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(prior.get("vendor_identity")) != capture["vendor_identity_sha256"]):
        raise Incomplete("Engine protected-debit exact placement predecessor differs.")
    overlay = {name: digest for name, digest in identity.items() if previous[name] != digest}
    overlay_sha = COMMON.canonical_sha256(overlay)
    if (COMMON.canonical_sha256(sorted(overlay)) != ENGINE_PROTECTED_DEBIT_PRIOR_DELTA
            or capture.get("overlay_sha256") != overlay_sha or terminal.get("overlay_sha256") != overlay_sha
            or terminal.get("repair_patch_sha256") != overlay_sha
            or focus.get("status") != "passed-integrated-receive-debit"
            or focus.get("owned_work_retired") is not True or focus.get("inputs") != identity
            or focus.get("candidate_sha256") != ENGINE_PROTECTED_DEBIT_CANDIDATE
            or focus.get("stage_tree") != capture["staged_tree"]
            or focus.get("index_sha256") != capture["index_sha256"]
            or terminal.get("focused_development_sha256") != ENGINE_PROTECTED_DEBIT_FOCUSED):
        raise Incomplete("Engine protected-debit exact source delta or focused execution differs.")
    wrapper = path.parents[2] / "run-combined.py"
    if (capture.get("wrapper_sha256") != ENGINE_PROTECTED_DEBIT_WRAPPER
            or hashlib.sha256(COMMON.read_analysis_bytes(wrapper, maximum=1024**2)).hexdigest()
                != ENGINE_PROTECTED_DEBIT_WRAPPER):
        raise Incomplete("Engine protected-debit wrapper bytes differ.")
    admitted[str(wrapper)] = ENGINE_PROTECTED_DEBIT_WRAPPER
    if (capture.get("graph_delta_sha256") != ENGINE_PROTECTED_DEBIT_GRAPH_DELTA
            or any(capture["source"].get(name) != digest for name, digest in
                   (graph["manifest_sha256"] | graph["checkers"]).items())):
        raise Incomplete("Engine protected-debit predecessor graph inputs changed.")
    admitted.update(admit_engine_review_graph(capture, terminal, identity, graph,
                                              ENGINE_PROTECTED_DEBIT_GRAPH_DELTA))
    return admitted


def admit_engine_final_frame_lineage(capture: dict, terminal: dict, identity: dict,
                                     path: Path) -> dict:
    """Reconcile both test-only changes with the admitted receive-debit run."""
    evidence_root = path.parents[3]
    expected = {
        "combined_development_reference": (
            evidence_root / "combined-01/reports/run-01/verification.json", ENGINE_PROTECTED_DEBIT_TERMINAL),
        "focused_reference": (
            evidence_root / "final-known-frame-01/run-01/verification.json", ENGINE_FINAL_FRAME_FOCUSED),
        "bounded_fixture_reference": (
            evidence_root / "fixture-window-current-01/run-03/verification.json", ENGINE_FINAL_FRAME_FIXTURE),
        "incomplete_combined_reference": (
            evidence_root / "combined-03-durable/reports/run-01/verification.json", ENGINE_FINAL_FRAME_INCOMPLETE),
        "incomplete_capture_reference": (
            evidence_root / "combined-03-durable/admission.json", ENGINE_FINAL_FRAME_INCOMPLETE_CAPTURE),
        "incomplete_placement_reference": (
            evidence_root / "durable-storage-preflight-01/full-placement-01.json", ENGINE_FINAL_FRAME_INCOMPLETE_PLACEMENT),
        "predecessor_graph": (None, ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256),
    }
    admitted: dict[str, str] = {}
    records: dict[str, dict] = {}
    for label, (expected_path, pin) in expected.items():
        reference = capture.get(label)
        if (type(reference) is not dict or set(reference) != {"path", "sha256"}
                or type(reference["path"]) is not str or reference["sha256"] != pin):
            raise Incomplete("Engine final-frame predecessor reference differs.")
        member = Path(reference["path"])
        if not member.is_absolute() or (expected_path is not None and member != expected_path):
            raise Incomplete("Engine final-frame predecessor path differs.")
        records[label] = read_json(member, maximum=8 * 1024**2, expected_sha256=pin, owned_evidence=True)
        admitted[str(member)] = pin

    prior = records["combined_development_reference"]
    focus = records["focused_reference"]
    fixture = records["bounded_fixture_reference"]
    stopped = records["incomplete_combined_reference"]
    stopped_capture = records["incomplete_capture_reference"]
    placement = records["incomplete_placement_reference"]
    graph = records["predecessor_graph"]
    old_capture_path = evidence_root / "combined-01/admission.json"
    old_capture = read_json(old_capture_path, maximum=1024**2,
                            expected_sha256=ENGINE_PROTECTED_DEBIT_MANIFEST, owned_evidence=True)
    admitted[str(old_capture_path)] = ENGINE_PROTECTED_DEBIT_MANIFEST
    previous = prior.get("inputs")
    if (prior.get("status") != "passed" or prior.get("mode") != "combined"
            or prior.get("candidate_sha256") != ENGINE_PROTECTED_DEBIT_CANDIDATE
            or prior.get("capture_sha256") != ENGINE_PROTECTED_DEBIT_MANIFEST
            or type(previous) is not dict or previous.keys() != identity.keys()
            or COMMON.canonical_sha256(previous) != ENGINE_PROTECTED_DEBIT_CANDIDATE
            or old_capture["inputs"] != previous
            or prior.get("owned_work_retired") is not True or prior.get("resource_observer_joined") is not True
            or prior.get("public_campaigns_admitted") is not False
            or prior.get("predecessor_graph") != capture["predecessor_graph"]
            or terminal.get("reference_sha256") != ENGINE_PROTECTED_DEBIT_TERMINAL
            or terminal.get("prior_failed_development_sha256") != ENGINE_FINAL_FRAME_INCOMPLETE
            or prior.get("staged_tree") != capture["staged_tree"]
            or prior.get("staged_index_sha256") != capture["index_sha256"]
            or old_capture["head"] != capture["head"]
            or prior.get("compiler") != capture["compiler"]
            or prior.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(prior.get("vendor_identity")) != capture["vendor_identity_sha256"]):
        raise Incomplete("Engine final-frame exact receive-debit predecessor differs.")

    overlay = {name: digest for name, digest in identity.items() if previous[name] != digest}
    overlay_sha = COMMON.canonical_sha256(overlay)
    if (set(overlay) != {ENGINE_FINAL_FRAME_TEST_PATH, ENGINE_FINAL_FRAME_WORKER_PATH}
            or previous[ENGINE_FINAL_FRAME_WORKER_PATH] != ENGINE_FINAL_FRAME_WORKER_PRE
            or overlay[ENGINE_FINAL_FRAME_WORKER_PATH] != ENGINE_FINAL_FRAME_WORKER_POST
            or capture["source"] != old_capture["source"] | overlay
            or capture["protected"] != old_capture["protected"] | overlay
            or any(record.get("overlay_sha256") != overlay_sha for record in (capture, terminal))
            or terminal.get("repair_patch_sha256") != overlay_sha
            or set(capture["live_doc_sha256"]) != ENGINE_FINAL_FRAME_DOCS
            or capture["frozen_doc_sha256"] != {name: capture["source"][name] for name in ENGINE_FINAL_FRAME_DOCS}
            or focus.get("status") != "passed-final-known-frame"
            or focus.get("base_product") != ENGINE_PROTECTED_DEBIT_CANDIDATE
            or focus.get("candidate_sha256") != ENGINE_FINAL_FRAME_FOCUSED_CANDIDATE
            or focus.get("owned_work_retired") is not True
            or focus.get("inputs") != previous | {ENGINE_FINAL_FRAME_TEST_PATH: overlay[ENGINE_FINAL_FRAME_TEST_PATH]}
            or fixture.get("status") != "passed-fixture-qualification-controls"
            or fixture.get("prior_product") != ENGINE_FINAL_FRAME_FOCUSED_CANDIDATE
            or fixture.get("candidate_sha256") != ENGINE_FINAL_FRAME_FIXTURE_CANDIDATE
            or fixture.get("inputs") != identity | {ENGINE_FINAL_FRAME_WORKER_PATH: ENGINE_FINAL_FRAME_WORKER_FIXTURE}
            or fixture.get("owned_work_retired") is not True
            or type(fixture.get("generated_credit")) is not int or fixture["generated_credit"] != 0
            or fixture.get("patched_fixture_sha256") != ENGINE_FINAL_FRAME_WORKER_FIXTURE
            or terminal.get("bounded_fixture_reference") != capture["bounded_fixture_reference"]
            or focus.get("generated_mutation_credit") != 0 or focus.get("native_windows") is not False
            or terminal.get("focused_development_sha256") != ENGINE_FINAL_FRAME_FOCUSED):
        raise Incomplete("Engine final-frame exact test-source delta or focused evidence differs.")

    if (stopped.get("status") != "incomplete" or stopped.get("active_lane") != "msvc-clippy"
            or stopped.get("candidate_sha256") != ENGINE_FINAL_FRAME_FIXTURE_CANDIDATE
            or stopped.get("inputs") != fixture["inputs"] or stopped_capture.get("inputs") != fixture["inputs"]
            or stopped.get("capture_sha256") != ENGINE_FINAL_FRAME_INCOMPLETE_CAPTURE
            or stopped_capture.get("wall_seconds") != 2400 or not stopped.get("resource_observer_joined")
            or stopped.get("owned_work_retired") is not True
            or placement.get("exported_sha256", {}).get("reports/run-01/verification.json") != ENGINE_FINAL_FRAME_INCOMPLETE
            or any(placement.get(key) is not True for key in ("outer_work_retired", "mount_retired", "image_retired"))
            or placement["elapsed"]["launch_seconds"] >= 69 or placement["elapsed"]["prepare_seconds"] >= 15
            or capture.get("previous_full_charged_seconds") != 84
            or terminal.get("previous_full_charged_seconds") != 84
            or 84 + 50 + ENGINE_FINAL_FRAME_RETRY_WALL + 10 > 2400
            or any(terminal.get(key) != capture.get(key) for key in (
                "incomplete_combined_reference", "incomplete_capture_reference", "incomplete_placement_reference"))):
        raise Incomplete("Engine final-frame retry lost its failed lint result or cumulative allowance.")
    lint = stopped.get("lanes", [])
    if [(row.get("lane"), row.get("exit_code")) for row in lint] != [
        ("boundary-control", 0), ("format", 0), ("msvc-check", 0), ("msvc-clippy", 101)
    ]:
        raise Incomplete("Engine final-frame failed lint roster differs.")
    lint_path = expected["incomplete_combined_reference"][0].with_name("msvc-clippy.json")
    lint_raw = read_json(lint_path, maximum=4 * 1024**2, expected_sha256=lint[-1]["sha256"], owned_evidence=True)
    if (lint_raw.get("exit_code") != 101 or lint_raw.get("failure", True) is not None
            or "too many lines (102/100)" not in lint_raw.get("output", "")
            or "run_managed_loopback_control" not in lint_raw["output"]):
        raise Incomplete("Engine final-frame original lint diagnostic differs.")
    admitted[str(lint_path)] = lint[-1]["sha256"]

    fixture_path = expected["bounded_fixture_reference"][0]
    fixture_exits = {"boundary-control": 0, "original-healthy": 0, "format": 0,
                     "bounded-healthy": 0, "pending-phase": 101, "adapter-match": 101,
                     "bounded-restored": 0}
    fixture_lanes = fixture.get("lanes", [])
    if [row.get("lane") for row in fixture_lanes] != list(fixture_exits):
        raise Incomplete("Engine bounded-loopback raw lane roster differs.")
    for row in fixture_lanes:
        label = row["lane"]
        expected_exit = fixture_exits[label]
        path_relative = fixture_path.with_name(label + ".json")
        raw = read_json(path_relative, maximum=4 * 1024**2, expected_sha256=row["sha256"], owned_evidence=True)
        output = raw.get("output")
        if (type(raw.get("exit_code")) is not int or raw["exit_code"] != expected_exit
                or row.get("exit") != expected_exit or raw.get("failure", True) is not None
                or row.get("failure", True) is not None or type(output) is not str):
            raise Incomplete("Engine bounded-loopback raw lane exit differs.")
        if label == "boundary-control" and "PRODUCT_BOUNDARY_OK" not in output:
            raise Incomplete("Engine bounded-loopback boundary marker is missing.")
        if label in {"original-healthy", "bounded-healthy", "pending-phase", "adapter-match", "bounded-restored"}:
            observed = re.findall(r"(?m)^test ([\w:]+) \.\.\. (ok|FAILED)$", output)
            wanted = [(name, "ok") for name in ENGINE_FINAL_FRAME_FIXTURE_CASES]
            if expected_exit == 101:
                wanted = [(ENGINE_FINAL_FRAME_FIXTURE_CASES[0], "FAILED")]
                if not re.search(r"(?m)^Error: Deadline$", output) or row.get("test_elapsed_seconds") != 4.0:
                    raise Incomplete("Engine bounded-loopback original deadline witness differs.")
            if sorted(observed) != sorted(wanted):
                raise Incomplete("Engine bounded-loopback raw cases differ.")
        admitted[str(path_relative)] = row["sha256"]

    focus_path = expected["focused_reference"][0]
    focus_lanes = focus.get("lanes")
    exits = {"boundary-control": 0, "format": 0, "healthy": 0, "retirement-omission": 101,
             "restored": 0, "clippy-gnu": 0, "clippy-msvc": 0}
    if type(focus_lanes) is not dict or focus_lanes.keys() != exits.keys():
        raise Incomplete("Engine final-frame focused evidence roster differs.")
    for label, exit_code in exits.items():
        row = focus_lanes[label]
        raw_path = focus_path.with_name(label + ".json")
        raw = read_json(raw_path, maximum=4 * 1024**2, expected_sha256=row["sha256"], owned_evidence=True)
        if (type(raw.get("exit_code")) is not int or raw["exit_code"] != exit_code
                or type(row.get("exit_code")) is not int or row["exit_code"] != exit_code
                or raw.get("failure", True) is not None or row.get("failure", True) is not None
                or type(raw.get("output")) is not str):
            raise Incomplete("Engine final-frame focused raw lane differs.")
        if label in {"healthy", "retirement-omission", "restored"}:
            observed = re.findall(r"(?m)^test ([\w:]+) \.\.\. (ok|FAILED)$", raw["output"])
            wanted = "FAILED" if label == "retirement-omission" else "ok"
            cases = row.get("cases")
            if (type(cases) is not dict or len(cases) != (11 if label == "healthy" else 4)
                    or len(observed) != len(cases) or dict(observed) != cases
                    or any(cases.get(name) != wanted for name in ENGINE_FINAL_FRAME_ADDITIONS)
                    or any(result != "ok" for name, result in cases.items()
                           if name not in ENGINE_FINAL_FRAME_ADDITIONS)):
                raise Incomplete("Engine final-frame focused test cases differ.")
        admitted[str(raw_path)] = row["sha256"]

    wrapper = path.parents[2] / "run-combined.py"
    if (capture.get("wrapper_sha256") != ENGINE_FINAL_FRAME_WRAPPER
            or hashlib.sha256(COMMON.read_analysis_bytes(wrapper, maximum=1024**2)).hexdigest()
                != ENGINE_FINAL_FRAME_WRAPPER):
        raise Incomplete("Engine final-frame wrapper differs.")
    admitted[str(wrapper)] = ENGINE_FINAL_FRAME_WRAPPER

    if (capture.get("graph_delta_sha256") != ENGINE_PROTECTED_DEBIT_GRAPH_DELTA
            or any(capture["source"].get(name) != digest for name, digest in
                   (graph["manifest_sha256"] | graph["checkers"]).items())):
        raise Incomplete("Engine final-frame predecessor graph inputs differ.")
    admitted.update(admit_engine_review_graph(capture, terminal, identity, graph,
                                              ENGINE_PROTECTED_DEBIT_GRAPH_DELTA))
    return admitted


def admit_engine_stale_debit_lineage(capture: dict, terminal: dict, identity: dict,
                                     path: Path) -> dict:
    """Bind one added regression to its actual predecessor and complete raw lanes."""
    evidence_root = path.parents[3]
    prior_path = evidence_root / "combined-04-durable/reports/run-01/verification.json"
    prior_capture_path = prior_path.parents[2] / "admission.json"
    old_capture = read_json(prior_capture_path, maximum=1024**2,
                            expected_sha256=ENGINE_FINAL_FRAME_MANIFEST, owned_evidence=True)
    previous = old_capture.get("inputs")
    if (type(previous) is not dict or len(previous) != 620
            or previous.keys() != identity.keys()
            or COMMON.canonical_sha256(previous) != ENGINE_FINAL_FRAME_CANDIDATE
            or {name for name in identity if previous[name] != identity[name]} != {ENGINE_STALE_DEBIT_TEST_PATH}
            or previous[ENGINE_STALE_DEBIT_TEST_PATH] != ENGINE_STALE_DEBIT_TEST_PRE
            or identity[ENGINE_STALE_DEBIT_TEST_PATH] != ENGINE_STALE_DEBIT_TEST_POST):
        raise Incomplete("Engine stale-debit exact one-test product delta differs.")
    old_binding = {"path": str(prior_path), "sha256": ENGINE_FINAL_FRAME_TERMINAL}
    prior_capture, prior, recipe, boundary = read_combined_development(
        ENGINE_FINAL_FRAME_CANDIDATE, previous, old_binding)
    old_rows = {row["lane"]: row for row in prior["lanes"]}
    old_default = {"candidate_sha256": ENGINE_FINAL_FRAME_CANDIDATE,
                   "test_inventory": sorted(case["name"] for case in old_rows["default-ordinary"]["cases"]),
                   "doctest_inventory": sorted(old_rows["default-doctests"]["cases"])}
    admitted = {str(prior_path): ENGINE_FINAL_FRAME_TERMINAL,
                str(prior_capture_path): ENGINE_FINAL_FRAME_MANIFEST}
    admitted.update(admit_engine_final_frame_lineage(prior_capture, prior, previous, prior_path))
    prefix = boundary["sandbox_command"][:-len(boundary["command"])-1]
    admitted.update(admit_source_policy_capture_rows(prior_path, prior["lanes"], recipe, old_default, prefix))
    overlay = {ENGINE_STALE_DEBIT_TEST_PATH: ENGINE_STALE_DEBIT_TEST_POST}
    references = {
        "combined_development_reference": (prior_path, ENGINE_FINAL_FRAME_TERMINAL),
        "focused_reference": (evidence_root / "ledger-stale-receive-check-01/run-full-01/verification.json", ENGINE_STALE_DEBIT_FULL),
        "independent_affected_reference": (evidence_root / "ledger-stale-receive-check-01/independent-full-readback-01.json", ENGINE_STALE_DEBIT_READBACK),
        "preparation_reference": (path.parents[2] / "preparation.json", ENGINE_STALE_DEBIT_PREPARATION),
    }
    records = {}
    for label, (member, digest) in references.items():
        if capture.get(label) != {"path": str(member), "sha256": digest}:
            raise Incomplete("Engine stale-debit immutable evidence reference differs.")
        records[label] = read_json(member, maximum=8 * 1024**2,
                                   expected_sha256=digest, owned_evidence=True)
        admitted[str(member)] = digest
    if (capture["source"] != old_capture["source"] | overlay
            or capture["protected"] != old_capture["protected"] | overlay
            or len(capture["source"]) != 2146 or len(capture["protected"]) != 696
            or any(capture.get(key) != old_capture.get(key) for key in (
                "head", "staged_tree", "index_sha256", "compiler", "tool_inventories",
                "vendor_identity_sha256", "predecessor_graph", "frozen_doc_sha256"))
            or set(capture["live_doc_sha256"]) != ENGINE_FINAL_FRAME_DOCS
            or capture["overlay_sha256"] != COMMON.canonical_sha256(overlay)
            or terminal.get("repair_patch_sha256") != COMMON.canonical_sha256(overlay)
            or terminal.get("reference_sha256") != ENGINE_FINAL_FRAME_TERMINAL
            or terminal.get("focused_development_sha256") != ENGINE_STALE_DEBIT_FULL):
        raise Incomplete("Engine stale-debit source, graph or predecessor custody differs.")
    graph_ref = capture["predecessor_graph"]
    graph_path = Path(graph_ref["path"])
    graph = read_json(graph_path, maximum=8 * 1024**2,
                      expected_sha256=ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256, owned_evidence=True)
    graph_inputs = graph.get("product_input_sha256")
    if type(graph_inputs) is not dict or not graph_inputs.keys() <= identity.keys():
        raise Incomplete("Engine stale-debit predecessor graph input map differs.")
    graph_delta = COMMON.canonical_sha256(sorted(
        name for name, digest in identity.items() if graph_inputs.get(name) != digest))
    if (capture.get("graph_delta_sha256") != graph_delta
            or any(capture["source"].get(name) != digest for name, digest in
                   (graph["manifest_sha256"] | graph["checkers"]).items())):
        raise Incomplete("Engine stale-debit graph delta or immutable graph inputs differ.")
    admitted[str(graph_path)] = ENGINE_KE_SOURCE_SCALE_GRAPH_SHA256
    admitted.update(admit_engine_review_graph(capture, terminal, identity, graph, graph_delta))
    full = records["focused_reference"]
    readback = records["independent_affected_reference"]
    prep = records["preparation_reference"]
    accounting = {"total_seconds": 2400, "prior_combined_seconds": 623,
                  "focused_seconds": 123, "affected_full_seconds": 267,
                  "charged_seconds": 1013, "preflight_seconds": 50,
                  "service_seconds": 1200, "stop_seconds": 10, "bounded_total_seconds": 2273}
    if (capture.get("accounting") != accounting or prep.get("accounting") != accounting
            or capture.get("previous_full_charged_seconds") != 1013
            or terminal.get("previous_full_charged_seconds") != 1013
            or capture.get("wall_seconds") != 1200
            or full.get("status") != "passed-stale-debit-affected-ep1"
            or full.get("base_product") != ENGINE_FINAL_FRAME_CANDIDATE
            or full.get("candidate_sha256") != ENGINE_STALE_DEBIT_CANDIDATE or full.get("inputs") != identity
            or full.get("previous_full_sha256") != ENGINE_FINAL_FRAME_TERMINAL
            or full.get("owned_work_retired") is not True
            or type(full.get("generated_mutation_credit")) is not int or full["generated_mutation_credit"] != 0
            or full.get("native_windows") is not False
            or readback.get("status") != "passed-independent-full-raw-readback"
            or readback.get("full_verification_sha256") != ENGINE_STALE_DEBIT_FULL
            or readback.get("candidate_sha256") != ENGINE_STALE_DEBIT_CANDIDATE
            or readback.get("input_count") != 620
            or prep.get("status") != "prepared-not-admitted"
            or prep.get("candidate_sha256") != ENGINE_STALE_DEBIT_CANDIDATE):
        raise Incomplete("Engine stale-debit affected evidence or cumulative allowance differs.")
    full_path = references["focused_reference"][0]
    binding = {"candidate_sha256": ENGINE_STALE_DEBIT_CANDIDATE,
               "test_inventory": sorted(case["name"] for case in full["lanes"]["ep1-ordinary"]["cases"]),
               "doctest_inventory": sorted(full["lanes"]["ep1-doctests"]["cases"])}
    for current, old in zip(capture["lanes"], old_capture["lanes"], strict=True):
        expected = list(old)
        if old[0] in {"ep1-ordinary", "ep1-optimized"}:
            pairs = set(map(tuple, old[3]))
            added = ("unittests src/lib.rs", ENGINE_STALE_DEBIT_CASE)
            if len(pairs) != 603 or added in pairs:
                raise Incomplete("Engine stale-debit predecessor roster differs.")
            expected[3] = [list(pair) for pair in sorted(pairs | {added})]
        if current != expected:
            raise Incomplete("Engine stale-debit captured command or exact roster differs.")
        if old[0] not in {"ep1-ordinary", "ep1-optimized", "ep1-doctests"}:
            continue
        label = old[0]
        row = full["lanes"][label]
        member = full_path.with_name(label + ".json")
        raw = read_json(member, maximum=4 * 1024**2, expected_sha256=row["sha256"], owned_evidence=True)
        if (row.get("exit_code") != 0 or raw.get("exit_code") != 0
                or row.get("failure", True) is not None or raw.get("failure", True) is not None
                or type(raw.get("command")) is not list
                or COMMON.canonical_sha256(raw["command"]) != ENGINE_STALE_DEBIT_FULL_COMMANDS[label]):
            raise Incomplete("Engine stale-debit affected raw command or outcome differs.")
        validate = validate_engine_ke_current_doctests if label == "ep1-doctests" else validate_engine_ke_execution
        facts = validate(raw["output"], binding)
        cases = facts["test_names"] if label == "ep1-doctests" else engine_ke_development_ordinary_cases(raw["output"])
        if cases != row.get("cases") or facts["tests_executed"] != row.get("tests_executed"):
            raise Incomplete("Engine stale-debit affected raw identities differ.")
        if label != "ep1-doctests" and COMMON.canonical_sha256(sorted((case["binary"], case["name"]) for case in cases)) != ENGINE_STALE_DEBIT_EP1_CASE_DIGEST:
            raise Incomplete("Engine stale-debit affected raw binary roster differs.")
        admitted[str(member)] = row["sha256"]
    wrapper = path.parents[2] / "run-combined.py"
    if (capture.get("wrapper_sha256") != ENGINE_STALE_DEBIT_WRAPPER
            or hashlib.sha256(COMMON.read_analysis_bytes(wrapper, maximum=1024**2)).hexdigest() != ENGINE_STALE_DEBIT_WRAPPER):
        raise Incomplete("Engine stale-debit configured wrapper differs.")
    admitted[str(wrapper)] = ENGINE_STALE_DEBIT_WRAPPER
    return admitted


def validate_joined_receiving_boundary_delta(identity: dict[str, str], predecessor: dict[str, str]) -> None:
    """Refuse graph reuse for anything beyond the reviewed two-file test delta."""
    changes: dict[str, tuple[str, str]] = {
        name: (predecessor[name], digest) for name, digest in identity.items()
        if name in predecessor and predecessor[name] != digest
    }
    if (identity.keys() != predecessor.keys() or changes != ENGINE_JOINED_RECEIVING_BOUNDARY_DELTA):
        raise Incomplete("Joined receiving test-only source delta differs.")


def admit_joined_receiving_boundary_material(capture: dict, terminal: dict, identity: dict[str, str],
                                            predecessor: dict[str, str], path: Path) -> dict[str, str]:
    """Read the exact predecessor recipe and the focused evidence retained with this run."""
    validate_joined_receiving_boundary_delta(identity, predecessor)
    lineage: dict = capture["test_only_lineage"]
    if (terminal.get("test_only_lineage") != lineage
            or lineage["candidate"] != ENGINE_JOINED_RECEIVING_CANDIDATE
            or lineage["test_only_delta"] != {
                name: list(pins) for name, pins in ENGINE_JOINED_RECEIVING_BOUNDARY_DELTA.items()
            }
            or lineage["new_ep1_cases"] != list(ENGINE_JOINED_RECEIVING_BOUNDARY_CASES)
            or lineage["focused_relative_path"] != "lineage/focus/report.json"
            or lineage["focused_sha256"] != ENGINE_JOINED_RECEIVING_BOUNDARY_FOCUSED):
        raise Incomplete("Joined receiving current evidence has a different test-only lineage.")
    admitted: dict[str, str] = {}
    previous_evidence: dict[str, str] = lineage["predecessor_terminal"]
    old_capture, _, _, _ = read_combined_development(
        ENGINE_JOINED_RECEIVING_CANDIDATE, predecessor, previous_evidence)
    old_capture_path: Path = Path(previous_evidence["path"]).parents[2] / "admission.json"
    if (lineage["predecessor_capture"] != {"path": str(old_capture_path), "sha256": ENGINE_JOINED_RECEIVING_MANIFEST}
            or lineage["predecessor_readback"]["sha256"] != ENGINE_JOINED_RECEIVING_PREDECESSOR_READBACK
            or old_capture["root_graph_evidence"] != capture["root_graph_evidence"]):
        raise Incomplete("Joined receiving predecessor capture or graph differs.")
    old_readback: Path = Path(lineage["predecessor_readback"]["path"])
    read_json(old_readback, maximum=64 * 1024, expected_sha256=ENGINE_JOINED_RECEIVING_PREDECESSOR_READBACK,
              owned_evidence=True)
    admitted.update({previous_evidence["path"]: ENGINE_JOINED_RECEIVING_TERMINAL,
                     str(old_capture_path): ENGINE_JOINED_RECEIVING_MANIFEST,
                     str(old_readback): ENGINE_JOINED_RECEIVING_PREDECESSOR_READBACK})
    for row, old in zip(capture["lanes"], old_capture["lanes"], strict=True):
        expected: list[object] = list(old)
        if old[0] in {"ep1-ordinary", "ep1-optimized"}:
            expected[3] = sorted(old[3] + [
                ["unittests src/lib.rs", name] for name in ENGINE_JOINED_RECEIVING_BOUNDARY_CASES
            ])
        if row != expected:
            raise Incomplete("Joined receiving successor selects a different physical lane recipe.")
    material: dict[str, str] = lineage["material"]
    home: Path = path.parents[2]
    focus_path: Path = home / lineage["focused_relative_path"]
    focused: dict = read_json(focus_path, maximum=4 * 1024**2,
                             expected_sha256=ENGINE_JOINED_RECEIVING_BOUNDARY_FOCUSED, owned_evidence=True)
    if (focused.get("status") != "passed" or focused.get("source_inputs") != identity
            or focused.get("source_sha256") != ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE
            or not all(name in focused["requested_cases"] for name in ENGINE_JOINED_RECEIVING_BOUNDARY_CASES)
            or len(focused["attempts"]) != 17):
        raise Incomplete("Joined receiving focused observations differ from the current candidate.")
    expected_material: dict[str, str] = {
        "focus/report.json": ENGINE_JOINED_RECEIVING_BOUNDARY_FOCUSED,
        **{f"preimages/{name}": pins[0] for name, pins in ENGINE_JOINED_RECEIVING_BOUNDARY_DELTA.items()},
    }
    for row in focused["attempts"]:
        label: str = row["lane"]
        if (not re.fullmatch(r"boundary|(?:discover|execute)-0[1-8]", label)
                or type(row.get("exit_code")) is not int or row["exit_code"] != 0
                or row.get("failure", True) is not None):
            raise Incomplete("Joined receiving focused raw lane is missing or failed.")
        expected_material[f"focus/{label}.log"] = row["log_sha256"]
    if len(expected_material) != 20 or material != expected_material:
        raise Incomplete("Joined receiving focused evidence member set differs.")
    for name, pin in material.items():
        member: Path = home / "lineage" / name
        data: bytes = COMMON.read_analysis_bytes(member, maximum=4 * 1024**2)
        if hashlib.sha256(data).hexdigest() != pin:
            raise Incomplete("Joined receiving focused or preimage bytes differ.")
        admitted[str(member)] = pin
    return admitted


def admit_engine_joined_receiving_lineage(capture: dict, terminal: dict, identity: dict,
                                          path: Path) -> dict[str, str]:
    """Read the current raw dependency graph, roster reference and combined producer bytes."""
    admitted: dict[str, str] = {}
    graph_reference: dict[str, str] = capture["root_graph_evidence"]
    focused_reference: dict[str, str] = capture["current_raw_reference"]
    boundary: bool = terminal.get("candidate_sha256") == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE
    wrapper_pin: str = (ENGINE_JOINED_RECEIVING_BOUNDARY_WRAPPER if boundary else ENGINE_JOINED_RECEIVING_WRAPPER)
    resource_pin: str = (ENGINE_JOINED_RECEIVING_BOUNDARY_RESOURCES if boundary else ENGINE_JOINED_RECEIVING_RESOURCES)
    reference_pin: str = (ENGINE_JOINED_RECEIVING_TERMINAL if boundary else ENGINE_JOINED_RECEIVING_FOCUSED)
    if (terminal.get("root_graph_evidence") != graph_reference
            or terminal.get("current_raw_reference") != focused_reference
            or graph_reference["sha256"] != ENGINE_JOINED_RECEIVING_GRAPH
            or focused_reference["sha256"] != ENGINE_JOINED_RECEIVING_FOCUSED
            or terminal.get("reference_sha256") != reference_pin
            or capture.get("wrapper_sha256") != wrapper_pin
            or capture.get("cargo_home_config_sha256") != ENGINE_JOINED_RECEIVING_CARGO_HOME):
        raise Incomplete("Joined receiving development references differ from their admitted identities.")
    graph_path: Path = Path(graph_reference["path"])
    focused_path: Path = Path(focused_reference["path"])
    graph: dict = read_json(graph_path, maximum=8 * 1024**2, expected_sha256=ENGINE_JOINED_RECEIVING_GRAPH,
                            owned_evidence=True)
    focused: dict = read_json(focused_path, maximum=8 * 1024**2, expected_sha256=ENGINE_JOINED_RECEIVING_FOCUSED,
                              owned_evidence=True)
    graph_identity: dict[str, str] = graph["product_input_sha256"] if boundary else identity
    if (graph.get("status") != "passed" or graph.get("product_input_sha256") != graph_identity
            or COMMON.canonical_sha256(graph_identity) != ENGINE_JOINED_RECEIVING_CANDIDATE
            or graph.get("product_digest") != ENGINE_JOINED_RECEIVING_CANDIDATE
            or graph.get("lockfile_sha256") != identity.get("Cargo.lock")
            or any(identity.get(name) != digest for name, digest in
                   (graph["manifest_sha256"] | graph["checkers"]).items())
            or focused.get("status") != "passed" or focused.get("inputs") != graph_identity
            or focused.get("candidate_sha256") != ENGINE_JOINED_RECEIVING_CANDIDATE
            or focused.get("root_graph_evidence") != graph_reference
            or focused.get("vendor", {}).get("inventory_sha256") != capture["vendor_identity_sha256"]):
        raise Incomplete("Joined receiving dependency graph or raw roster source differs.")
    admitted.update({str(graph_path): ENGINE_JOINED_RECEIVING_GRAPH, str(focused_path): ENGINE_JOINED_RECEIVING_FOCUSED})
    if boundary:
        admitted.update(admit_joined_receiving_boundary_material(capture, terminal, identity, graph_identity, path))
    graph_pins: dict[str, str] = {
        f"{row['profile']}--{row['label']}.json": row["sha256"] for row in graph["lanes"]
    }
    graph_pins.update({row["file"]: row["sha256"] for row in graph["projections"]})
    if (len(graph["lanes"]) != 45 or len(graph["rows"]) != 4 or len(graph["projections"]) != 4
            or len(graph_pins) != 49 or graph_pins != capture["graph_members"]
            or any(row.get("exit_code") != 0 or type(row["exit_code"]) is not int for row in graph["lanes"])
            or any(re.fullmatch(r"[a-z0-9_-]+\.json", name) is None for name in graph_pins)):
        raise Incomplete("Joined receiving dependency graph physical roster differs.")
    for name, digest in graph_pins.items():
        member: Path = graph_path.with_name(name)
        read_json(member, maximum=16 * 1024**2, expected_sha256=digest, owned_evidence=True)
        admitted[str(member)] = digest
    for row in focused["lanes"]:
        raw: Path = focused_path.with_name(row["lane"] + ".json")
        read_json(raw, maximum=4 * 1024**2, expected_sha256=row["sha256"], owned_evidence=True)
        admitted[str(raw)] = row["sha256"]
    for file, expected in ((path.parents[2] / "run-combined.py", wrapper_pin),
                           (path.with_name("resources.jsonl"), resource_pin)):
        content: bytes = COMMON.read_analysis_bytes(file, maximum=4 * 1024**2)
        if hashlib.sha256(content).hexdigest() != expected:
            raise Incomplete("Joined receiving producer or resource observation bytes differ.")
        admitted[str(file)] = expected
    return admitted


def admit_engine_combined_review_lineage(capture: dict, terminal: dict, identity: dict,
                                         path: Path) -> dict:
    """Keep each exact combined review on its own original construction and retry evidence."""
    if terminal.get("candidate_sha256") in ENGINE_JOINED_RECEIVING_FAMILY:
        return admit_engine_joined_receiving_lineage(capture, terminal, identity, path)
    if terminal.get("candidate_sha256") == ENGINE_STALE_DEBIT_CANDIDATE:
        return admit_engine_stale_debit_lineage(capture, terminal, identity, path)
    if terminal.get("candidate_sha256") == ENGINE_FINAL_FRAME_CANDIDATE:
        return admit_engine_final_frame_lineage(capture, terminal, identity, path)
    if terminal.get("candidate_sha256") == ENGINE_PROTECTED_DEBIT_CANDIDATE:
        return admit_engine_protected_debit_lineage(capture, terminal, identity, path)
    if terminal.get("candidate_sha256") == ENGINE_PLACEMENT_CANDIDATE:
        return admit_engine_placement_lineage(capture, terminal, identity, path)
    if terminal.get("candidate_sha256") != ENGINE_SCALE_CONFLICT_CANDIDATE:
        return admit_engine_review_lineage(capture, terminal, identity)

    admitted, references = {}, {}
    for label in ("combined_development_reference", "focused_reference", "predecessor_graph"):
        binding = capture[label]
        reference_path = Path(binding["path"])
        references[label] = read_json(reference_path, maximum=8 * 1024**2,
                                      expected_sha256=binding["sha256"], owned_evidence=True)
        admitted[str(reference_path)] = binding["sha256"]
    prior_ref = capture["combined_development_reference"]
    focus_ref = capture["focused_reference"]
    prior, focus, graph = (references[label] for label in (
        "combined_development_reference", "focused_reference", "predecessor_graph"))
    previous = prior.get("inputs")
    if (prior_ref["sha256"] != ENGINE_REVIEW_TERMINAL
            or terminal.get("reference_sha256") != prior_ref["sha256"]
            or prior.get("status") != "passed" or prior.get("mode") != "combined"
            or prior.get("candidate_sha256") != ENGINE_REVIEW_CANDIDATE or type(previous) is not dict
            or len(previous) != ENGINE_REVIEW_INPUT_COUNT or COMMON.canonical_sha256(previous) != ENGINE_REVIEW_CANDIDATE
            or not previous.keys() <= identity.keys()
            or COMMON.canonical_sha256(sorted(name for name, digest in identity.items()
                                              if previous.get(name) != digest)) != ENGINE_SCALE_CONFLICT_PRIOR_DELTA
            or focus.get("status") != "passed-focused-portable-causal-pair"
            or focus.get("owned_work_retired") is not True
            or focus.get("snapshots", {}).get("green") != {
                "candidate": ENGINE_SCALE_CONFLICT_CANDIDATE, "inputs": identity}
            or terminal.get("focused_development_sha256") != focus_ref["sha256"]
            or prior.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(prior.get("vendor_identity")) != capture["vendor_identity_sha256"]):
        raise Incomplete("Engine scale-conflict predecessor and focused source identity differs.")

    wrapper = path.parents[2] / "run-combined.py"
    expected_cumulative = ENGINE_SCALE_CONFLICT_FIRST_WALL + ENGINE_SCALE_CONFLICT_RETRY_WALL
    if (capture.get("wrapper_sha256") != ENGINE_SCALE_CONFLICT_WRAPPER
            or hashlib.sha256(COMMON.read_analysis_bytes(wrapper, maximum=1024**2)).hexdigest()
                != ENGINE_SCALE_CONFLICT_WRAPPER
            or capture.get("first_attempt_outer_seconds") != ENGINE_SCALE_CONFLICT_FIRST_WALL
            or terminal.get("cumulative_wall_seconds_ceiling") != expected_cumulative
            or expected_cumulative > 2400
            or terminal.get("prior_failed_development_sha256") != ENGINE_SCALE_CONFLICT_STOPPED):
        raise Incomplete("Engine scale-conflict retry custody or cumulative allowance differs.")
    admitted[str(wrapper)] = ENGINE_SCALE_CONFLICT_WRAPPER
    stopped = None
    for label, length in (("focused_references", 6), ("previous_combined_attempt", 4)):
        pins = capture.get(label)
        if type(pins) is not dict or len(pins) != length or terminal.get(label) != pins:
            raise Incomplete("Engine scale-conflict focused or stopped-run reference set differs.")
        for name, digest in pins.items():
            reference_path = Path(name)
            if not reference_path.is_absolute():
                raise Incomplete("Engine scale-conflict lineage requires an absolute evidence path.")
            if reference_path.suffix == ".json":
                record = read_json(reference_path, maximum=8 * 1024**2, expected_sha256=digest,
                                   owned_evidence=True)
                if digest == ENGINE_SCALE_CONFLICT_STOPPED:
                    stopped = record
            elif (reference_path.suffix != ".py"
                  or hashlib.sha256(COMMON.read_analysis_bytes(reference_path, maximum=1024**2)).hexdigest()
                      != digest):
                raise Incomplete("Engine scale-conflict retained wrapper bytes differ.")
            admitted[name] = digest
    if (type(stopped) is not dict or stopped.get("status") != "incomplete"
            or stopped.get("candidate_sha256") != ENGINE_SCALE_CONFLICT_CANDIDATE
            or stopped.get("inputs") != identity
            or stopped.get("active_lane") != "ep1-doctests"
            or stopped.get("error") != "RuntimeError('Rustdoc identity changes: ep1-doctests')"
            or stopped.get("owned_work_retired") is not True):
        raise Incomplete("Engine scale-conflict first attempt must retain the exact incomplete outcome.")
    graph_dependencies = graph["manifest_sha256"] | graph["checkers"]
    if any(capture["source"].get(name) != digest for name, digest in graph_dependencies.items()):
        raise Incomplete("Engine scale-conflict predecessor graph inputs changed.")
    admitted.update(admit_engine_review_graph(capture, terminal, identity, graph, ENGINE_SCALE_CONFLICT_GRAPH_DELTA))
    return admitted


def read_combined_development(candidate: str, identity: dict, evidence: dict) -> tuple[dict, dict, list, dict]:
    """Read the owned frozen capture and its complete combined terminal observation."""
    path = Path(evidence["path"])
    review_count = engine_review_input_count(candidate)
    terminal_pin = engine_review_terminal_pin(candidate)
    if review_count is not None and evidence["sha256"] != terminal_pin:
        raise Incomplete("Engine review requires the exact combined terminal receipt.")
    if (len(path.parents) < 3 or path.name != "verification.json" or path.parent.name != "run-01"
            or path.parents[1].name != "reports"):
        raise Incomplete("Engine combined development requires the fixed receipt layout.")
    manifest, boundary_sha = source_policy_combined_evidence(candidate)
    capture_path = path.parents[2] / "admission.json"
    capture = read_json(capture_path, maximum=1024**2, expected_sha256=manifest, owned_evidence=True)
    evidence_sha = evidence["sha256"]
    reference = read_json(path, maximum=8 * 1024**2, expected_sha256=evidence_sha, owned_evidence=True)
    expected_main = {
        "posture": "verified-staged-base-and-exact-overlay", "head": capture["head"],
        "tree": capture["staged_tree"], "index_sha256": capture["index_sha256"],
    }
    elapsed, peak = reference.get("elapsed_seconds"), reference.get("peak_memory_bytes")
    memory_events, invocation = reference.get("memory_events"), reference.get("invocation_id")
    expected_wall = (ENGINE_SCALE_CONFLICT_RETRY_WALL if candidate == ENGINE_SCALE_CONFLICT_CANDIDATE
                     else 2400)
    if candidate == ENGINE_FINAL_FRAME_CANDIDATE:
        expected_wall = ENGINE_FINAL_FRAME_RETRY_WALL
    if candidate in (ENGINE_STALE_DEBIT_CANDIDATE, ENGINE_JOINED_RECEIVING_CANDIDATE):
        expected_wall = 1200
    if candidate == ENGINE_JOINED_RECEIVING_BOUNDARY_CANDIDATE:
        expected_wall = 600
    cargo_home_pin: str = (ENGINE_JOINED_RECEIVING_CARGO_HOME if candidate in ENGINE_JOINED_RECEIVING_FAMILY
                           else SOURCE_POLICY_SOURCE_SCALE_CARGO_HOME)
    documentation_keys = {"live_doc_sha256", "frozen_doc_sha256"}
    if candidate == ENGINE_PLACEMENT_CANDIDATE:
        expected_wall = ENGINE_PLACEMENT_RETRY_WALL
        document_custody = not documentation_keys & (capture.keys() | reference.keys())
    else:
        document_custody = all(key in capture and reference.get(key) == capture[key]
                               for key in documentation_keys)
    if (reference.get("status") != "passed" or reference.get("mode") != "combined"
            or reference.get("capture_sha256") != manifest
            or reference.get("candidate_sha256") != capture["candidate_sha256"]
            or capture["candidate_sha256"] != candidate
            or reference.get("inputs") != identity or capture["inputs"] != identity
            or (review_count is not None
                and (len(identity) != review_count or COMMON.canonical_sha256(identity) != candidate
                     or capture.get("input_manifest_count") != len(identity)))
            or reference.get("protected") != capture["protected"] or reference.get("main_admission") != expected_main
            or reference.get("staged_tree") != capture["staged_tree"]
            or reference.get("staged_index_sha256") != capture["index_sha256"]
            or reference.get("compiler") != capture["compiler"]
            or reference.get("tool_inventories") != capture["tool_inventories"]
            or COMMON.canonical_sha256(reference.get("vendor_identity")) != capture["vendor_identity_sha256"]
            or reference.get("cargo_home_config_sha256") != cargo_home_pin
            or not document_custody
            or reference.get("native_windows_execution") != "not performed"
            or reference.get("metrics") != "not performed"
            or reference.get("public_campaigns_admitted") is not False or capture["campaigns_admitted"] is not False
            or any(reference.get(key) is not True for key in (
                "resource_observer_joined", "owned_work_allocated", "owned_work_retired"))
            or reference.get("tasks_events") != "max 0" or type(memory_events) is not dict
            or any(memory_events.get(key) != "0" for key in ("max", "oom", "oom_kill", "oom_group_kill"))
            or type(invocation) is not str or re.fullmatch(r"[a-f0-9]{32}", invocation) is None
            or type(elapsed) not in (int, float) or not 0 < elapsed <= capture["wall_seconds"] == expected_wall
            or type(peak) is not int or not 0 < peak <= 6 * 1024**3
            or {"active_lane", "error", "final_custody_error"} & reference.keys()):
        raise Incomplete("Engine combined development lacks exact passing terminal custody.")

    recipe = [dict(label=label, command=command, kind=kind, expected=expected, retire=retire)
              for label, command, kind, expected, retire in capture["lanes"]]
    lanes = reference.get("lanes")
    if (type(lanes) is not list or len(lanes) != 19 or len(recipe) != 19
            or any(type(row) is not dict for row in lanes)
            or [row.get("lane") for row in lanes] != [row["label"] for row in recipe]
            or reference.get("cache_retirement") != [row["label"] for row in recipe if row["retire"]]):
        raise Incomplete("Engine combined development requires all 19 physical lanes.")
    boundary = read_json(path.with_name("boundary-control.json"), maximum=4 * 1024**2,
                         expected_sha256=boundary_sha, owned_evidence=True)
    return capture, reference, recipe, boundary


def admit_source_policy_combined_development(binding: dict, identity: dict, path: Path) -> dict:
    """Read all physical lanes and the named producer's own historical lineage."""
    candidate = binding["candidate_sha256"]
    evidence = binding["development_evidence"]
    capture, reference, recipe, boundary = read_combined_development(candidate, identity, evidence)
    sandbox_prefix = boundary["sandbox_command"][:-len(boundary["command"])-1]
    lineage = (admit_engine_combined_review_lineage(capture, reference, identity, path)
               if engine_review_input_count(candidate) is not None
               else admit_source_policy_scale_lineage(capture, reference, identity))
    return {str(path): evidence["sha256"], str(path.parents[2] / "admission.json"): reference["capture_sha256"],
            **lineage,
            **admit_source_policy_capture_rows(path, reference["lanes"], recipe, binding, sandbox_prefix)}


def admit_source_policy_development(binding: dict, identity: dict) -> dict:
    """Retain prior-source development evidence without transferring a candidate pass.

    Current helper/manifest changes have their own admission. Every actual Rust,
    Cargo and compiler configuration byte must still match the development
    source; new runtime inputs require new development evidence.
    """
    evidence = binding["development_evidence"]
    path = Path(evidence["path"])
    actual_candidate = COMMON.canonical_sha256(identity)
    if (source_policy_combined_roster(actual_candidate) is not None
            or source_policy_combined_roster(binding["candidate_sha256"]) is not None):
        if binding["candidate_sha256"] != actual_candidate:
            raise Incomplete("Source-policy capture development candidate binding differs.")
        if (actual_candidate == SOURCE_POLICY_SOURCE_SCALE_CANDIDATE
                or engine_review_input_count(actual_candidate) is not None):
            return admit_source_policy_combined_development(binding, identity, path)
        return admit_source_policy_capture_development(binding, identity, path)
    reference = read_json(path, maximum=4 * 1024**2, expected_sha256=evidence["sha256"])
    if (PRODUCT.sha256(path) != evidence["sha256"] or reference.get("status") != "workload-passed"
            or reference.get("custody_unchanged") is not True):
        raise Incomplete("Source-policy development evidence lacks passing terminal custody.")
    managed_candidate = SOURCE_POLICY_MANAGED_DEVELOPMENT["candidate_sha256"]
    receiver = SOURCE_POLICY_RECEIVER_DEVELOPMENT
    receiver_candidates = {receiver["candidate_sha256"], receiver["incomplete_candidate_sha256"]}
    if actual_candidate in receiver_candidates or binding["candidate_sha256"] in receiver_candidates:
        if binding["candidate_sha256"] != actual_candidate:
            raise Incomplete("Source-policy receiver development candidate binding differs.")
        if actual_candidate == receiver["incomplete_candidate_sha256"]:
            raise Incomplete("Receiver-guard initial development remains incomplete; retain full1 failure.")
        observer_inputs = admit_source_policy_receiver_observer(reference, path, binding, identity)
    elif actual_candidate == managed_candidate or binding["candidate_sha256"] == managed_candidate:
        if binding["candidate_sha256"] != actual_candidate:
            raise Incomplete("Source-policy current development candidate binding differs.")
        # Current source selects its required producer contract. Receipt fields
        # cannot downgrade that contract by omitting the managed markers.
        observer_inputs = admit_source_policy_managed_observer(reference, path)
    else:
        observer_inputs = admit_source_policy_observer(reference, path)
    previous = reference.get("source_sha256")
    if type(previous) is not dict:
        raise Incomplete("Source-policy development source identity is missing.")
    runtime_names = {name for name in identity if name.endswith((".rs", "Cargo.toml", "Cargo.lock", "clippy.toml"))
                     or name in {"rust-toolchain.toml", ".cargo/config.toml", ".cargo/mutants.toml", ".config/nextest.toml"}}
    if any(previous.get(name) != identity[name] for name in runtime_names):
        raise Incomplete("Source-policy runtime/graph/configuration differs from its development reference.")
    lanes = reference.get("lanes")
    required = {"boundary-control", "format", "gnu-check", "gnu-clippy", "msvc-clippy", "msvc-check", "gnu-rustdoc",
                "default-ordinary", "default-optimized", "default-doctests",
                "ep1-ordinary", "ep1-optimized", "ep1-doctests"}
    if (type(lanes) is not list or len(lanes) != len(required)
            or any(type(row) is not dict for row in lanes)
            or {row.get("lane") for row in lanes} != required):
        raise Incomplete("Source-policy development evidence lacks its complete thirteen lanes.")
    admitted = {str(path): evidence["sha256"]}
    for row in lanes:
        raw_path = Path(row.get("path", ""))
        if not raw_path.is_absolute() or raw_path.parent != path.parent:
            raise Incomplete("Source-policy development lane leaves its fixed receipt directory.")
        raw = read_json(raw_path, maximum=4 * 1024**2, expected_sha256=row.get("sha256"))
        digest = PRODUCT.sha256(raw_path)
        if (digest != row.get("sha256") or type(row.get("exit_code")) is not int
                or row["exit_code"] != 0 or row.get("failure") is not None
                or type(raw.get("exit_code")) is not int or raw["exit_code"] != 0
                or raw.get("failure") is not None or type(raw.get("output")) is not str):
            raise Incomplete("Source-policy development raw lane is failed or changed.")
        admitted[str(raw_path)] = digest
        if row["lane"] in {"default-ordinary", "default-optimized"}:
            source_policy_execution(raw["output"], binding)
        elif row["lane"] in {"ep1-ordinary", "ep1-optimized"}:
            source_policy_ep1_execution(raw["output"], binding)
        elif row["lane"] in {"default-doctests", "ep1-doctests"}:
            source_policy_development_doctests(raw["output"], binding, row["lane"])
    return {**admitted, **observer_inputs}


def execute_source_policy_profile(args, report: dict) -> None:
    """Use the existing isolated metric machinery for exactly five default-graph owners."""
    report.update(profile="source_policy", qualification_passed=False, selected_lane_passed=False)
    if (args.tool_admission_profile != "stable-core"
            or args.lane not in {"baseline", "enumerate", "coverage", "mutation", "all"}):
        raise Incomplete("Source-policy admits only fixed stable-core baseline/enumeration/metric lanes.")
    policy, binding, identity = read_source_policy_policy(
        ROOT, campaign=args.lane in {"mutation", "all"})
    subjects = SOURCE_POLICY_SCOPE.select(args.subject)
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Source-policy metrics require seven GiB before admission.")
    evidence_before = admit_source_policy_development(binding, identity)
    selected = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        selected.lane = "mutation"
    report.update(subjects=subjects, candidate_sha256=binding["candidate_sha256"],
                  full_critical_files=list(SOURCE_POLICY_SUBJECTS.values()),
                  scope="Selected managed laboratory KoD owners; no NTS, native or product qualification",
                  dependency_profile="default", development_reference=binding["development_evidence"],
                  development_reference_scope=(
                      "Exact current 19-lane development; helpers and metric results retain separate admission."
                      if binding["candidate_sha256"] in (
                          *ENGINE_JOINED_RECEIVING_FAMILY, SOURCE_POLICY_SOURCE_SCALE_CANDIDATE,
                          ENGINE_REVIEW_CANDIDATE,
                          ENGINE_SCALE_CONFLICT_CANDIDATE, ENGINE_PLACEMENT_CANDIDATE,
                          ENGINE_PROTECTED_DEBIT_CANDIDATE, ENGINE_FINAL_FRAME_CANDIDATE, ENGINE_STALE_DEBIT_CANDIDATE) else
                      "Prior-source development only; current helpers/manifest and metric results retain their own admission."),
                  pending_qualification="Generation gaps, independent outcomes, EP1/native/production and other critical owners remain separate.")
    session = COMMON.Session(ROOT, selected, report, SOURCE_POLICY_SCOPE, policy, SOURCE_POLICY_BOUND_INPUTS)
    try:
        if (session.product_identity != identity
                or read_json(session.source / SOURCE_POLICY_POLICY) != policy
                or any(PRODUCT.sha256(session.source / name) != digest
                       for name, digest in binding["inputs_sha256"].items())):
            raise Incomplete("Captured source-policy source/role/helper identity differs before execution.")

        def run(label, command, timeout, **options):
            address_storage_checkpoint(session, policy, report, label, "before")
            try:
                result = session.run(label, command, timeout, **options)
                if label.startswith("mutation-campaign:"):
                    subject = label.removeprefix("mutation-campaign:")
                    directory = session.work / "mutation" / subject / "mutants.out"
                    outcomes = COMMON.read_analysis_json(directory / "outcomes.json")
                    baseline = next(row for row in outcomes["outcomes"] if row["scenario"] == "Baseline")
                    relative = COMMON.analysis_output_path(baseline["log_path"])
                    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
                        raise Incomplete("Source-policy baseline log leaves its owned output.")
                    report.setdefault("baseline_execution", {})[subject] = source_policy_execution(
                        COMMON.read_analysis_text(directory / relative), binding, doctests=True)
                return result
            finally:
                address_storage_checkpoint(session, policy, report, label, "after")

        if args.lane == "baseline":
            deadline = time.monotonic() + policy["baseline_wall_seconds"]
            prefix = ["cargo", "test", "--package", "nevrotick-engine", "--frozen", "--jobs", "6"]
            raw = run("source-policy-baseline", [*prefix, "--lib", "--tests", "--", "--test-threads=1"],
                      COMMON.remaining_timeout(deadline, policy["baseline_wall_seconds"]))
            report["baseline_execution"] = source_policy_execution(raw, binding)
            raw = run("source-policy-doctests", [*prefix, "--doc", "--", "--test-threads=1"],
                      COMMON.remaining_timeout(deadline, policy["baseline_wall_seconds"]))
            docs = validate_engine_ke_doctests(raw)
            if sorted(docs["test_names"]) != binding["doctest_inventory"]:
                raise Incomplete("Source-policy baseline omitted or substituted a compiler example.")
            report["doctest_execution"] = docs
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(
                SOURCE_POLICY_SCOPE, subjects, policy, session.tools, run, report,
                deadline=time.monotonic() + policy["enumeration_wall_seconds"],
                maximum_per_file=512, maximum_total=min(2560, 512 * len(subjects)))
            report.update(listed_inventories=listing.inventories,
                          listed_inventory_bindings=listed_summary(listing.inventories),
                          listed_total=listing.total, campaign_admission=False)
        if args.lane in {"coverage", "all"}:
            ignored = [re.escape("/work/source/" + name) + "$" for name in session.product_identity
                       if name.endswith(".rs") and name not in subjects.values()]
            ignored.append("/rustlib/src/rust/library/")
            regex = "|".join(ignored)
            if len(regex.encode()) > 65_536:
                raise Incomplete("Source-policy exact coverage filename filter exceeds its bound.")
            raw = run("coverage", [str(session.tools / "cargo-llvm-cov"), "llvm-cov",
                "--package", "nevrotick-engine", "--frozen", "--jobs", "6", "--lib", "--tests",
                "--json", "--output-path", "/work/coverage.json", "--failure-mode", "any",
                "--ignore-filename-regex", regex, "--fail-under-file-lines", "90", "--", "--test-threads=1"],
                policy["coverage_wall_seconds"])
            report["coverage_execution"] = source_policy_execution(raw, binding)
            coverage = COMMON.read_analysis_json(session.work / "coverage.json")
            report["coverage"] = {name: COMMON.coverage_counts(
                SOURCE_POLICY_SCOPE, coverage, "/work/source/" + filename)
                for name, filename in subjects.items()}
        if args.lane in {"mutation", "all"}:
            COMMON.run_analysis_lanes(SOURCE_POLICY_SCOPE, subjects, policy, session.work,
                                      session.tools, run, report, "mutation")
    finally:
        session.finish()
        if admit_source_policy_development(binding, identity) != evidence_before:
            raise Incomplete("Source-policy development reference changed during execution.")
    report["selected_lane_passed"] = args.lane != "enumerate"
    # Raw gates do not establish independent outcome/generation review. The
    # coordinator owns that separate conclusion, including a per-owner union.
    report["qualification_passed"] = False
    report["independent_mutation_classification"] = "Every outcome and ungenerated guard requires separate review; no manual kill credit."


def execute(args, report: dict) -> None:
    """Execute only the fixed selected lane, with full shared custody bookends."""
    profile = getattr(args, "profile", "legacy")
    if profile == "roughtime":
        if args.lane == "enumerate":
            execute_roughtime_inventory_profile(args, report)
        else:
            execute_roughtime_line_profile(args, report)
        return
    if profile == "source_policy":
        execute_source_policy_profile(args, report)
        return
    if args.lane in {"baseline", "fixtures"} and profile != "ke_association":
        raise Incomplete("The baseline and fixture lanes belong solely to the Engine KE role.")
    if profile == "ke_association":
        if args.lane == "fixtures":
            execute_engine_ke_fixtures(args, report)
            return
        execute_engine_ke_profile(args, report)
        return
    if profile not in {"legacy", "series", "quality", "session", "address_round"}:
        raise Incomplete("Unknown engine analysis profile.")
    if profile == "address_round":
        execute_address_profile(args, report)
        return
    if profile == "session":
        execute_session_profile(args, report)
        return
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


ROUGHTIME_LINE_SCOPE: COMMON.Scope = COMMON.Scope(
    "nevrotick-engine", COMMON.ENGINE_ROUGHTIME_LINE_SUBJECTS, COMMON.ENGINE_ROUGHTIME_LINE_POLICY)
ROUGHTIME_INVENTORY_SCOPE: COMMON.Scope = COMMON.Scope(
    "nevrotick-engine", COMMON.ENGINE_ROUGHTIME_LINE_SUBJECTS, COMMON.ENGINE_ROUGHTIME_INVENTORY_POLICY)
ROUGHTIME_LINE_FACTS: dict[str, object] = {
    "schema_version": 1,
    "admission_status": COMMON.ENGINE_ROUGHTIME_LINE_ROLE,
    "package": "nevrotick-engine",
    "feature": "roughtime-verification",
    "target": "x86_64-unknown-linux-gnu",
    "tool_admission_profile": "stable-core",
    "line_minimum_percent": 90,
    "coverage_wall_seconds": 240,
    "jobs": 2,
    "test_threads": 1,
    "critical_files": list(ROUGHTIME_LINE_SCOPE.select("all").values()),
}
ROUGHTIME_RESULT_FILES: dict[str, tuple[str, int]] = {
    "unittests src/lib.rs": ("nevrotick_engine", 283),
    "tests/roughtime_verification.rs": ("roughtime_verification", 0),
}


def read_roughtime_line_policy(root: Path) -> tuple[dict[str, object], dict[str, list[str]]]:
    """Admit the frozen host feature, owners and non-overlapping per-binary rosters."""
    policy: dict[str, object] = read_json(root / ROUGHTIME_LINE_SCOPE.policy)
    if (set(policy) != set(ROUGHTIME_LINE_FACTS) | {"candidate_sha256", "test_inventory"}
            or any(type(policy[key]) is not type(value) or policy[key] != value
                   for key, value in ROUGHTIME_LINE_FACTS.items())):
        raise Incomplete("Engine Roughtime coverage policy differs from its fixed role.")
    fingerprint: object = policy["candidate_sha256"]
    if type(fingerprint) is not str or re.fullmatch(r"[0-9a-f]{64}", fingerprint) is None:
        raise Incomplete("Engine Roughtime requires an exact candidate digest.")
    inventory: object = policy["test_inventory"]
    if type(inventory) is not dict or set(inventory) != {"src/lib.rs", "tests/roughtime_verification.rs"}:
        raise Incomplete("Engine Roughtime requires both exact test binaries.")
    roster: dict[str, list[str]] = {}
    for binary, names in inventory.items():
        expected_count: int = 23 if binary == "src/lib.rs" else 60
        if (type(names) is not list or len(names) != expected_count
                or any(type(name) is not str or re.fullmatch(r"\w+(?:::\w+)*", name, re.ASCII) is None
                       or len(name) > 256 for name in names)
                or names != sorted(set(names))
                or any(name.startswith("roughtime::") != (binary == "src/lib.rs") for name in names)):
            raise Incomplete("Engine Roughtime test roster is malformed or assigned to the wrong binary.")
        roster[binary] = names
    manifest: dict[str, object] = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    minimum: object = manifest.get("security_module_minimum_executable_line_percent")
    if type(minimum) is not int or minimum != ROUGHTIME_LINE_FACTS["line_minimum_percent"]:
        raise Incomplete("Engine Roughtime coverage requires the registered security line threshold.")
    files: list[str] = [filename for row in manifest["security_modules"]
                        if row["package"] == "nevrotick-engine" and row["name"].startswith("roughtime-v1-")
                        for filename in row["files"]]
    if (sorted(files) != sorted(ROUGHTIME_LINE_SCOPE.select("all").values())
            or any(manifest["critical_files"].count(filename) != 1 for filename in files)):
        raise Incomplete("Engine Roughtime coverage omits or duplicates a registered security owner.")
    return policy, roster


def roughtime_line_execution(text: str, roster: dict[str, list[str]]) -> dict[str, list[str]]:
    """Require actual successful output for each selected case in its original binary."""
    blocks: list[re.Match[str]] = session_execution_blocks(text)
    headers: list[re.Match[str]] = list(re.finditer(
        r"(?m)^[ \t]*Running (unittests src/lib\.rs|tests/roughtime_verification\.rs) "
        r"\(/work/target/llvm-cov-target/x86_64-unknown-linux-gnu/debug/build/nevrotick-engine/"
        r"([0-9a-f]{16})/out/(nevrotick_engine|roughtime_verification)-\2\)$", text))
    if (len(headers) != len(blocks) or len(headers) != len(roster)
            or {header[1] for header in headers} != set(ROUGHTIME_RESULT_FILES)):
        raise Incomplete("Engine Roughtime coverage lost a selected test binary.")
    passes: dict[str, list[str]] = {}
    previous_end: int = 0
    for header, block in zip(headers, blocks, strict=True):
        binary: str = header[1].removeprefix("unittests ")
        stem, filtered = ROUGHTIME_RESULT_FILES[header[1]]
        records: list[str] = re.findall(r"(?m)^test (\S+) \.\.\. ok$", block[2])
        expected: list[str] = roster[binary]
        if (header[3] != stem or not previous_end <= header.start() < header.end() <= block.start()
                or block[3] != "ok" or int(block[1]) != len(expected) or int(block[4]) != len(expected)
                or any(int(block[index]) for index in (5, 6, 7))
                or int(block[8]) != filtered
                or len(re.findall(r"(?m)^test ", block[2])) != len(records)
                or sorted(records) != expected):
            raise Incomplete("Engine Roughtime coverage omitted, substituted or failed a selected case.")
        previous_end = block.end()
        passes[binary] = records
    if len(re.findall(r"(?m)^test (?!result:)", text)) != sum(map(len, passes.values())):
        raise Incomplete("Engine Roughtime coverage has test records outside its selected binaries.")
    return passes


def execute_roughtime_line_profile(args: argparse.Namespace, report: dict[str, object]) -> None:
    """Measure the ten GNU owners with fixed source, graph, tool and result custody."""
    policy, roster = read_roughtime_line_policy(ROOT)
    COMMON.admission_profile(args, ROUGHTIME_LINE_SCOPE, policy)
    subjects: dict[str, str] = ROUGHTIME_LINE_SCOPE.select("all")
    report.update(profile="roughtime", scope="Engine Roughtime per-file GNU line coverage only",
                  subjects=subjects, line_gate_passed=False, qualification_passed=False,
                  mutation_qualification="not performed", selected_test_inventory=roster)
    inputs: tuple[str, ...] = (WRAPPER, "scripts/tests/test_engine_roughtime_line_analysis.py")
    session: COMMON.Session = COMMON.Session(ROOT, args, report, ROUGHTIME_LINE_SCOPE, policy, inputs)
    try:
        copied_policy, copied_roster = read_roughtime_line_policy(session.source)
        if (copied_policy != policy or copied_roster != roster
                or COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]):
            raise Incomplete("Engine Roughtime coverage requires its exact declared product candidate.")
        try:
            PRODUCT.SLINT.profile_rows(session.dependency_policy, PRODUCT.SLINT.RGT_PROFILE)
        except PRODUCT.SLINT.Incomplete as error:
            raise Incomplete(str(error)) from error
        ignored: list[str] = [re.escape("/work/source/" + name) + "$" for name in session.product_identity
                              if name.endswith(".rs") and name not in subjects.values()]
        ignored.append("/rustlib/src/rust/library/")
        regex: str = "|".join(ignored)
        if len(regex.encode()) > 65_536:
            raise Incomplete("Engine Roughtime coverage filename filter exceeds its bound.")
        coverage_path: Path = session.work / "coverage.json"
        report["raw_coverage"] = str(coverage_path)
        command: list[str] = [str(session.tools / "cargo-llvm-cov"), "llvm-cov", "--package", "nevrotick-engine",
                              "--features=roughtime-verification", "--target=x86_64-unknown-linux-gnu", "--frozen",
                              "--jobs", "2", "--lib", "--test", "roughtime_verification", "--json",
                              "--output-path", "/work/coverage.json", "--failure-mode", "any",
                              "--ignore-filename-regex", regex, "--fail-under-file-lines", "90", "--",
                              "--test-threads=1", "--exact", *(name for names in roster.values() for name in names)]
        tool_refusal: Exception | None = None
        output: str
        try:
            output = session.run("roughtime-line-coverage", command, 240)
        except PRODUCT.ENV.Incomplete as error:
            # The pinned coverage tool writes its complete report before the
            # file threshold refuses. Keep those observations without a pass.
            lanes: object = report.get("lanes")
            if type(lanes) is not list or not lanes:
                raise
            result: dict[str, object] = lanes[-1]
            if (result["lane"] != "roughtime-line-coverage" or result["failure"] is not None
                    or result["exit_code"] != 1 or not coverage_path.is_file()):
                raise
            raw_output: object = result["output"]
            if type(raw_output) is not str:
                raise
            output = raw_output
            tool_refusal = error
        report["coverage_execution"] = roughtime_line_execution(output, roster)
        coverage: dict[str, object] = COMMON.read_analysis_json(coverage_path)
        observations: dict[str, COMMON.CoverageObservation] = {
            name: COMMON.coverage_observation(ROUGHTIME_LINE_SCOPE, coverage, "/work/source/" + filename)
            for name, filename in subjects.items()}
        report["coverage"] = observations
        low: list[str] = [name for name, observation in observations.items() if observation["line_percent"] < 90]
        if low:
            raise Incomplete("Engine Roughtime executable-line threshold was not met: " + ", ".join(low))
        if tool_refusal is not None:
            raise tool_refusal
    finally:
        session.finish()
        report["vendor_sha256"] = COMMON.canonical_sha256(session.vendor_identity)
        report["vendor_file_count"] = len(session.vendor_identity)
    report["line_gate_passed"] = True


def read_roughtime_inventory_policy(root: Path) -> dict[str, object]:
    """Bind listing to the same admitted product, registered files and line policy."""
    policy: dict[str, object] = read_json(root / ROUGHTIME_INVENTORY_SCOPE.policy)
    COMMON.require_roughtime_inventory_role(ROUGHTIME_INVENTORY_SCOPE, policy)
    line_policy, _ = read_roughtime_line_policy(root)
    manifest: dict[str, object] = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    minimum: object = manifest.get("critical_file_minimum_mutation_percent")
    if type(minimum) is not int or minimum != policy["mutation_minimum_viable_killed_percent"]:
        raise Incomplete("Engine Roughtime inventory requires the registered critical-file mutation threshold.")
    if (policy["candidate_sha256"] != line_policy["candidate_sha256"]
            or policy["line_policy_sha256"] != PRODUCT.sha256(root / ROUGHTIME_LINE_SCOPE.policy)):
        raise Incomplete("Engine Roughtime inventory and line coverage identify different candidates or policies.")
    return policy


def execute_roughtime_inventory_profile(args: argparse.Namespace, report: dict[str, object]) -> None:
    """Generate ten GNU source inventories without compiling or testing mutants."""
    deadline: float = time.monotonic() + 120
    policy: dict[str, object] = read_roughtime_inventory_policy(ROOT)
    COMMON.admission_profile(args, ROUGHTIME_INVENTORY_SCOPE, policy)
    subjects: dict[str, str] = ROUGHTIME_INVENTORY_SCOPE.select("all")
    report.update(profile="roughtime", scope="Engine Roughtime GNU mutation inventory only",
                  subjects=subjects, campaign_admission=False, qualification_passed=False,
                  mutation_qualification="not performed")
    inputs: tuple[str, ...] = (WRAPPER, ROUGHTIME_LINE_SCOPE.policy,
                               "scripts/tests/test_engine_roughtime_inventory.py")
    session: COMMON.Session = COMMON.Session(ROOT, args, report, ROUGHTIME_INVENTORY_SCOPE, policy, inputs,
                                             listing_deadline=deadline)
    try:
        if (read_roughtime_inventory_policy(session.source) != policy
                or COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]):
            raise Incomplete("Engine Roughtime inventory requires its exact declared product candidate.")
        try:
            PRODUCT.SLINT.profile_rows(session.dependency_policy, PRODUCT.SLINT.RGT_PROFILE)
        except PRODUCT.SLINT.Incomplete as error:
            raise Incomplete(str(error)) from error
        enumeration: COMMON.MutationEnumeration = COMMON.enumerate_mutation_inventories(
            ROUGHTIME_INVENTORY_SCOPE, subjects, policy, session.tools, session.run, report,
            deadline=deadline, maximum_per_file=512, maximum_total=2048)
        report.update(listed_inventories=enumeration.inventories,
                      listed_inventory_bindings=listed_summary(enumeration.inventories))
    finally:
        session.finish()
        report["vendor_sha256"] = COMMON.canonical_sha256(session.vendor_identity)
        report["vendor_file_count"] = len(session.vendor_identity)
    if time.monotonic() >= deadline:
        raise Incomplete("Engine Roughtime mutation inventory exhausted its shared deadline.")


def main(argv=None) -> int:
    """Keep listing completion distinct from selected host coverage/mutation gates."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--profile", choices=("legacy", "series", "quality", "session", "address_round", "ke_association", "source_policy", "roughtime"), default="legacy")
    parser.add_argument("--lane", choices=("contracts", "baseline", "fixtures", "enumerate", "coverage", "mutation", "all"), required=True)
    parser.add_argument("--subject", choices=(*SUBJECTS, "series", "quality", "session", *ADDRESS_SUBJECTS, *ENGINE_KE_HOST_SUBJECTS, "source_policy", "all"), default="all")
    parser.add_argument("--tool-admission-profile", choices=("full", "stable-core"), default="full")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    if (args.profile == "roughtime" and (args.lane not in {"coverage", "enumerate"} or args.subject != "all"
                                        or args.tool_admission_profile != "stable-core")):
        parser.error("Engine Roughtime selects only stable-core coverage or inventory of all registered owners")
    if args.profile == "source_policy" and args.subject not in {*SOURCE_POLICY_SUBJECTS, "all"}:
        parser.error("Source-policy selects only its exact five named owners or all")
    if args.profile != "source_policy" and args.subject == "source_policy":
        parser.error("--subject source_policy requires --profile source_policy")
    if args.profile == "ke_association" and args.subject not in {*ENGINE_KE_HOST_SUBJECTS, "all"}:
        parser.error("Engine KE selects only its exact named host owners or all")
    if args.profile != "ke_association" and args.subject in ENGINE_KE_HOST_SUBJECTS:
        parser.error("Engine KE subjects require --profile ke_association")
    if args.lane == "baseline" and args.profile not in {"ke_association", "source_policy"}:
        parser.error("--lane baseline requires --profile ke_association or source_policy")
    if args.lane == "fixtures" and args.profile != "ke_association":
        parser.error("--lane fixtures requires --profile ke_association")
    if args.lane == "fixtures" and args.subject != "all":
        parser.error("--lane fixtures requires --subject all")
    if args.profile == "address_round" and args.subject not in {*ADDRESS_SUBJECTS, "all"}:
        parser.error("--profile address_round selects only address_round, address_progress or all")
    if args.profile not in {"address_round", "source_policy"} and args.subject in ADDRESS_SUBJECTS:
        parser.error("Address-round subjects require --profile address_round")
    if args.profile == "legacy" and args.subject == "series":
        parser.error("--subject series requires --profile series")
    if args.profile == "legacy" and args.subject == "quality":
        parser.error("--subject quality requires --profile quality")
    if args.profile == "series" and args.subject not in {*SERIES_SUBJECTS, "all"}:
        parser.error("--profile series selects only library, series or all")
    if args.profile == "quality" and args.subject not in {"quality", "all"}:
        parser.error("--profile quality selects only quality or all")
    if args.profile == "session" and args.subject not in {*SESSION_SUBJECTS, "all"}:
        parser.error("--profile session selects only session, series, measurement or all")
    if args.profile not in {"session", "source_policy"} and args.subject == "session":
        parser.error("--subject session requires --profile session")
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
