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
    """Keep one aggregate 768-MiB sample and the four-GiB free reserve per command."""
    usage = address_storage_usage(session.output)
    row = {"lane": label, "phase": phase, **usage, "available_bytes": shutil.disk_usage(session.output).free}
    report.setdefault("storage_samples", []).append(row)
    (session.output / "progress.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    if row["allocated_bytes"] >= policy["owned_output_maximum_allocated_bytes"]:
        raise Incomplete("Address-round aggregate owned output reached its 768-MiB allocated cap.")
    if row["available_bytes"] < policy["stop_free_bytes"]:
        raise Incomplete("Address-round four-GiB available reserve reached.")


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
ENGINE_KE_TEST_SOURCES = (
    "crates/nevrotick-engine/src/nts_ke_experiment/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/association/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/handoff/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/request_tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/response/tests.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/response/fixtures.rs",
)
ENGINE_KE_BOUND_INPUTS = tuple(dict.fromkeys((
    WRAPPER, TESTS, *COMMON.ENGINE_KE_CRITICAL, *ENGINE_KE_TEST_SOURCES,
    "scripts/portable-analysis.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/prepare-product.py", "policy/analysis-tools.json", "policy/verification-manifest.toml",
    "policy/tool-manifest.json", "policy/ui-m0-dependencies.json", ".cargo/mutants.toml",
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml",
    "tools/crypto-cng-lifecycle/Cargo.toml", "tools/crypto-cng-lifecycle/Cargo.lock",
)))
ENGINE_KE_FIXTURE_SOURCE = "tools/crypto-aes-siv-witness/tests/ep2_response_fixtures.rs"
ENGINE_KE_FIXTURE_MANIFEST = "tools/crypto-aes-siv-witness/Cargo.toml"
ENGINE_KE_FIXTURE_LOCK = "tools/crypto-aes-siv-witness/Cargo.lock"
ENGINE_KE_FIXTURE_NAMES = frozenset((
    "HEALTHY ZERO_COOKIES EMPTY_COOKIE TWO_COOKIES EIGHT_COOKIES NINE_COOKIES "
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
        raise Incomplete("Public fixtures omit part of the fixed thirty-three-case inventory.")
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
    dependency, identity = PRODUCT.admit_product(ROOT)
    PRODUCT.SLINT.profile_rows(dependency, PRODUCT.SLINT.EP1_PROFILE)
    if any(name not in identity for name in (ENGINE_KE_FIXTURE_SOURCE, ENGINE_KE_FIXTURE_LOCK)):
        raise Incomplete("The actual public generator is outside the admitted product source inventory.")
    graph = admit_engine_ke_fixture_lock(ROOT, dependency)
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Public fixtures require five GiB before admission.")
    inputs = (*ENGINE_KE_BOUND_INPUTS, ENGINE_KE_FIXTURE_SOURCE, ENGINE_KE_FIXTURE_MANIFEST, ENGINE_KE_FIXTURE_LOCK)
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


def validate_engine_ke_policy(policy: dict, *, campaign: bool = False) -> dict:
    """Bind the exact current source/helper set and all actual ordinary test identities."""
    COMMON.require_engine_ke_role(ENGINE_KE_SCOPE, policy)
    binding = policy["binding"]
    fields = {"candidate_sha256", "source_file_count", "inputs_sha256", "test_inventory",
              "source_context_sha256", "ordinary_evidence", "root_graph_evidence"}
    if type(binding) is not dict or set(binding) != fields:
        raise Incomplete("Engine KE source/evidence binding is incomplete.")
    for field in ("candidate_sha256", "source_context_sha256"):
        if type(binding[field]) is not str or re.fullmatch(r"[a-f0-9]{64}", binding[field]) is None:
            raise Incomplete("Engine KE candidate/context digest is malformed.")
    pins = binding["inputs_sha256"]
    if (type(binding["source_file_count"]) is not int or binding["source_file_count"] <= 0
            or type(pins) is not dict or set(pins) != set(ENGINE_KE_BOUND_INPUTS)
            or any(type(value) is not str or re.fullmatch(r"[a-f0-9]{64}", value) is None for value in pins.values())):
        raise Incomplete("Engine KE source/helper inputs differ from the exact fixed set.")
    names = binding["test_inventory"]
    if (type(names) is not list or len(names) != 261
            or any(type(name) is not str or not 0 < len(name) <= 4096 for name in names)
            or names != sorted(set(names))
            or sum(name.startswith("nts_ke_experiment::") for name in names) != 83):
        raise Incomplete("Engine KE requires all 261 actual Engine tests including its 83 KE cases.")
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
    """Use real entered libtest blocks, not metadata-only names or borrowed counts."""
    records = read_successful_address_records(text)
    ordinary = binding["test_inventory"]
    extra = set(records) - set(ordinary)
    if (not set(ordinary) <= set(records)
            or (extra and not include_doctests)
            or any(re.fullmatch(r"[^\r\n]+[.]rs - [^\r\n]+ \(line [1-9][0-9]*\)(?: - compile(?: fail)?)?", name) is None
                   for name in extra)):
        raise Incomplete("Engine KE did not execute every current actual library/integration case.")
    result = {"tests_executed": len(records), "ke_tests_executed": 83,
              "test_names": records, "inventory_sha256": COMMON.canonical_sha256(sorted(records))}
    if include_doctests:
        result["additional_doctests"] = sorted(extra)
    return result


def validate_engine_ke_doctests(text: str) -> dict:
    """Keep complete Rustdoc observations separate from ordinary tests and native work."""
    records = read_successful_address_records(text)
    if any(re.fullmatch(r"[^\r\n]+[.]rs - [^\r\n]+ \(line [1-9][0-9]*\)(?: - compile(?: fail)?)?", name) is None
           for name in records):
        raise Incomplete("Engine KE doctest lane contains an unknown non-Rustdoc case.")
    return {"tests_executed": len(records), "test_names": records,
            "compilation_only": [name for name in records if name.endswith((" - compile", " - compile fail"))]}



def admit_engine_ke_evidence(binding: dict) -> dict:
    """Bound and recheck the same exact reference bytes before and after jobs."""
    result = {}
    for key in ("ordinary_evidence", "root_graph_evidence"):
        evidence = binding[key]
        raw = COMMON.read_analysis_bytes(Path(evidence["path"]), maximum=COMMON.ANALYSIS_JSON_MAX_BYTES)
        result[key] = hashlib.sha256(raw).hexdigest()
        if result[key] != evidence["sha256"]:
            raise Incomplete("Engine KE prior evidence bytes changed.")
    return result

def execute_engine_ke_profile(args, report: dict) -> None:
    """Run only the admitted Root-lock feature, with explicit initial campaign hold."""
    if args.tool_admission_profile != "stable-core" or args.lane not in {"baseline", "enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Engine KE requires the selected stable host analysis lane.")
    policy = read_json(ROOT / ENGINE_KE_POLICY)
    binding = validate_engine_ke_policy(policy, campaign=args.lane in {"mutation", "all"})
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
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(row.get("package") == package and filename in row.get("files", [])
                       for row in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Engine KE requires the complete twelve-owner critical/security inventory.")
    evidence_before = admit_engine_ke_evidence(binding)
    if shutil.disk_usage(args.output_dir.parent).free < policy["minimum_free_bytes"]:
        raise Incomplete("Engine KE metrics require five GiB before admission.")
    selected = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        selected.lane = "mutation"
    report.update(profile="ke_association", subjects=subjects, qualification_passed=False,
                  full_critical_files=list(COMMON.ENGINE_KE_CRITICAL),
                  scope="Eight reached GNU KE owners; four remaining owners and Windows arms stay open",
                  source_context_sha256=binding["source_context_sha256"],
                  source_context_scope="Historical Tool preparation reference; current Root identity is candidate_sha256/source_sha256",
                  candidate_sha256=binding["candidate_sha256"],
                  ordinary_reference=binding["ordinary_evidence"],
                  ordinary_reference_scope="Historical predecessor under a standalone Tool lock; aggregate receipt incomplete. Later source/test deltas require separate reconciliation; no Root graph or metric credit transferred.",
                  pending_metric_owners=[name for name in COMMON.ENGINE_KE_CRITICAL if name not in ENGINE_KE_HOST_SUBJECTS.values()],
                  pending_windows="StrictProfile::capture, transport, provider shutdown and Windows-only control/error arms",
                  generation_gap_policy="Reconcile every constructor, generic helper, Garde expansion and atomic guard against actual descriptors; no manual-control kill credit.")
    session = COMMON.Session(ROOT, selected, report, ENGINE_KE_SCOPE, policy, ENGINE_KE_BOUND_INPUTS)
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
            raw = run("ke-release", ["cargo", "test", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--release", "--frozen", "--jobs", "6", "--lib", "--tests",
                "--", "--test-threads=1"], policy["baseline_wall_seconds"])
            report["release_execution"] = validate_engine_ke_execution(raw, binding)
            raw = run("ke-doctests", ["cargo", "test", "--package", "nevrotick-engine",
                "--features", "ke-association-prototype", "--frozen", "--jobs", "6", "--doc",
                "--", "--test-threads=1"], policy["baseline_wall_seconds"])
            report["doctest_execution"] = validate_engine_ke_doctests(raw)
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
    report["independent_mutation_classification"] = "Required for every caught, unviable or missed descriptor; pending separate review."
    # An eight-owner host pass never closes all twelve files, Windows or native custody.
    report["qualification_passed"] = False

def execute(args, report: dict) -> None:
    """Execute only the fixed selected lane, with full shared custody bookends."""
    profile = getattr(args, "profile", "legacy")
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


def main(argv=None) -> int:
    """Keep listing completion distinct from selected host coverage/mutation gates."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--profile", choices=("legacy", "series", "quality", "session", "address_round", "ke_association"), default="legacy")
    parser.add_argument("--lane", choices=("contracts", "baseline", "fixtures", "enumerate", "coverage", "mutation", "all"), required=True)
    parser.add_argument("--subject", choices=(*SUBJECTS, "series", "quality", "session", *ADDRESS_SUBJECTS, *ENGINE_KE_HOST_SUBJECTS, "all"), default="all")
    parser.add_argument("--tool-admission-profile", choices=("full", "stable-core"), default="full")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    if args.profile == "ke_association" and args.subject not in {*ENGINE_KE_HOST_SUBJECTS, "all"}:
        parser.error("Engine KE selects only its eight exact host owners or all")
    if args.profile != "ke_association" and args.subject in ENGINE_KE_HOST_SUBJECTS:
        parser.error("Engine KE subjects require --profile ke_association")
    if args.lane in {"baseline", "fixtures"} and args.profile != "ke_association":
        parser.error("--lane baseline/fixtures requires --profile ke_association")
    if args.lane == "fixtures" and args.subject != "all":
        parser.error("--lane fixtures requires --subject all")
    if args.profile == "address_round" and args.subject not in {*ADDRESS_SUBJECTS, "all"}:
        parser.error("--profile address_round selects only address_round, address_progress or all")
    if args.profile != "address_round" and args.subject in ADDRESS_SUBJECTS:
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
    if args.profile != "session" and args.subject == "session":
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
