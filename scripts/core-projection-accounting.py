"""Strict bounded source-recipe observations for fixture anchor projection.

Pure transport/accounting only: this module launches no tool and performs no I/O.
Reported public attempts, conservative nested source work and native assertion
outcomes are distinct. An absent record is None/pending, never observed zero.
"""
from __future__ import annotations

import re


class Incomplete(ValueError):
    """Projection markers or their frozen source recipes are incomplete."""


MARKER = "CORE_PROJECTION_PROFILE"
FIELDS = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections")
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 1024
U64_MAX = (1 << 64) - 1
# Actual authored public-call recipes; execution/admission remains root-owned.
SOURCE_RECIPES = (
    ("terms_admission", (0, 0, 0, 0, 0, 0, 0, 36, 48, 0)),
    ("linear_identity", (1, 4, 4, 1, 8, 16, 4, 280, 1, 6)),
    ("shape_controls", (2, 6, 6, 1, 8, 24, 6, 290, 1, 2)),
    ("origin_durations", (2, 6, 6, 1, 10, 24, 6, 290, 1, 2)),
    ("integer_envelope", (1, 3, 3, 1, 7, 12, 3, 170, 1, 3)),
    ("error_precedence", (3, 4, 4, 4, 9, 16, 4, 205, 2, 12)),
    ("evicted_anchor", (1, 65, 65, 1, 68, 260, 65, 1489, 1, 2)),
    ("numeric_failures", (1, 3, 3, 1, 7, 12, 3, 170, 5, 6)),
    ("signed_zero_consumption", (1, 3, 3, 1, 5, 12, 3, 520, 8, 16)),
    ("projection_private_predicate", (0,) * 10),
    ("projection_private_candidate", (0,) * 10),
    ("projection_private_arithmetic", (0,) * 10),
    ("projection_private_errors", (0,) * 10),
)
SOURCE_TESTS = (
    "terms_admit_signed_zeros_and_reject_each_raw_invalid_field_in_order",
    "linear_anchors_agree_and_repeated_projection_preserves_original_identity_and_age",
    "nonlinear_anchor_is_not_fitted_intercept_and_negative_rate_keeps_its_sign",
    "large_monotonic_origins_retain_one_nanosecond_without_absolute_float_conversion",
    "duration_conversion_accepts_two_to_53_and_rejects_only_its_next_integer",
    "relationship_errors_precede_numeric_work_and_preserve_every_history_view",
    "eviction_cannot_retarget_a_stale_index_to_a_different_observation",
    "separately_checked_growth_and_sums_reject_hidden_subnormal_and_overflow",
    "signed_zero_terms_reach_projection_without_clamping_or_losing_supplied_bits",
    "regression::projection::projection_guards::raw_predicate_and_garde_share_finite_normal_nonnegative_or_signed_zero_domain",
    "regression::projection::projection_guards::single_bad_raw_candidates_cover_all_three_rules_and_declared_first_error_order",
    "regression::projection::projection_guards::raw_arithmetic_steps_reject_unsupported_values_before_hidden_intermediates",
    "regression::projection::projection_guards::closed_errors_preserve_bounded_formatting_and_actual_typed_sources",
)
# Direct private-test source work, including the nested shared predicate calls.
# These constants assert no observation of private algorithm work.
SOURCE_PRIVATE_WORK = {
    "term_predicate_calls_upper": 148, "term_rule_calls_upper": 84,
    "raw_candidate_constructions_upper": 24, "candidate_validation_calls_upper": 24,
    "first_error_calls_upper": 24, "projection_arithmetic_calls_upper": 56,
    "shared_arithmetic_calls_upper": 56, "supported_result_calls_upper": 31,
    "expected_arithmetic_error_calls_upper": 36, "step_operation_calls_upper": 61,
    "error_wrapper_constructions_upper": 47, "error_format_calls_upper": 94,
    "error_source_calls_upper": 47, "typed_error_downcast_attempts_upper": 32,
}
SOURCE_RECIPE_STATUS = "admitted-reviewed-source-recipes"
_DECIMAL = r"(0|[1-9][0-9]{0,19})"


def _recipes():
    """Require the actual finite named source inventory before parsing/charging."""
    if (type(SOURCE_RECIPES) is not tuple or not 0 < len(SOURCE_RECIPES) <= 32
            or type(SOURCE_TESTS) is not tuple or len(SOURCE_TESTS) != len(SOURCE_RECIPES)):
        raise Incomplete("Projection named source recipes/test inventory are pending.")
    recipes = {}
    for case, values in SOURCE_RECIPES:
        if (type(case) is not str or re.fullmatch(r"[a-z_]{1,64}", case) is None or case in recipes
                or type(values) is not tuple or len(values) != len(FIELDS)
                or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)):
            raise Incomplete("Projection source recipe is unknown, duplicated or unbounded.")
        recipes[case] = values
    if len(set(SOURCE_TESTS)) != len(SOURCE_TESTS):
        raise Incomplete("Projection selected test-name inventory contains a duplicate.")
    return recipes


def projection_counts(output, *, require_complete=False):
    """Parse exact terminated marker lines and preserve complete/partial/missing.

    Partial means present nonnegative public-attempt prefixes within the exact
    named case ceilings. All-zero records are accepted only for the source-owned
    private tests that make no public calls. Completeness proves no native result.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Projection observations require bounded text and Boolean completeness mode.")
    recipes = _recipes()
    progress = r"(?:test (?:" + "|".join(re.escape(name) for name in SOURCE_TESTS) + r") \.\.\. )?"
    pattern = re.compile(progress + MARKER + r" case=([a-z_]+)"
                         + "".join(" " + field + "=" + _DECIMAL for field in FIELDS))
    recorded = {}
    lines = output.split("\n")
    for index, raw in enumerate(lines):
        if MARKER not in raw:
            continue
        if index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Projection marker lacks its complete bounded native line.")
        match = pattern.fullmatch(raw[:-1] if raw.endswith("\r") else raw)
        if match is None:
            raise Incomplete("Projection marker is malformed or outside its exact native prefix.")
        case, *values = match.groups()
        values = tuple(int(value) for value in values)
        if (case not in recipes or case in recorded or any(value > U64_MAX for value in values)
                or any(value > maximum for value, maximum in zip(values, recipes.get(case, ()), strict=True))):
            raise Incomplete("Projection case is unknown, duplicated or exceeds its source recipe.")
        if raw.startswith('test ') and not raw.startswith(
                'test ' + SOURCE_TESTS[list(recipes).index(case)] + ' ... '):
            raise Incomplete("Projection libtest prefix names a different source case.")
        if not any(values) and any(recipes[case]):
            raise Incomplete("Public projection case reports no prepaid source attempt.")
        recorded[case] = values
    missing = [case for case in recipes if case not in recorded]
    partial = [case for case, values in recorded.items() if values != recipes[case]]
    complete = not missing and not partial
    subtotals = ({field: sum(values[index] for values in recorded.values())
                  for index, field in enumerate(FIELDS)} if recorded else None)
    if require_complete and not complete:
        raise Incomplete("Projection package observations omit cases or differ from complete source recipes.")
    return {"projection_counter_records": len(recorded),
            "projection_profiles": {case: dict(zip(FIELDS, values, strict=True)) for case, values in recorded.items()},
            "projection_missing_cases": missing, "projection_partial_cases": partial,
            "projection_observation_pending": bool(missing),
            "projection_counter_observation_complete": complete,
            "projection_observation_status": "complete-source-recipe" if complete else "partial-source-recipe" if recorded else "missing",
            "projection_recorded_call_subtotals": subtotals,
            "projection_observed_calls": dict(subtotals) if not missing else None,
            "projection_observation_scope": "Reported public attempts only; no inferred private work, native outcome or fresh observation."}


def source_work_uppers(calls):
    """Project finite canonical nested work, separately from reported attempts.

    An append conservatively reserves the existing 64-row history bound. The
    projection borrows its existing report and performs no fit or normalization.
    These are source bounds, not measurements or arbitrary-mutant work limits.
    """
    if (type(calls) is not dict or set(calls) != set(FIELDS)
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in calls.values())):
        raise Incomplete("Projection source work requires every exact nonnegative public call field.")
    appends, projections, terms = calls["appends"], calls["projections"], calls["terms"]
    return {"charged_fit_work_upper": appends, "point_admissions_upper": 64 * appends,
            "history_normalization_entries_upper": appends, "history_statistics_entries_upper": appends,
            "history_statistics_inner_fit_upper": appends, "history_duration_operations_upper": 64 * appends,
            "history_generated_point_admissions_upper": 64 * appends,
            "history_fit_input_constructions_upper": appends,
            "history_staging_Copy_assignments_upper": 127 * appends,
            "projection_duration_operations_upper": 2 * projections,
            "projection_internal_product_getter_requests_upper": 12 * projections,
            "projection_raw_candidate_constructions_upper": terms,
            "projection_candidate_validation_calls_upper": terms,
            "projection_first_error_calls_upper": terms,
            "projection_term_garde_rule_callbacks_upper": 3 * terms,
            "projection_term_predicate_calls_upper": 6 * terms,
            "projection_checked_integer_helper_calls_upper": 2 * projections,
            "projection_normalization_primitive_calls_upper": 6 * projections,
            "projection_arithmetic_primitive_calls_upper": 5 * projections}


def full_package_reservation():
    """Reserve every actual source case before any complete package invocation."""
    recipes = _recipes()
    calls = {field: sum(values[index] for values in recipes.values()) for index, field in enumerate(FIELDS)}
    return {"projection_reserved_calls": calls,
            "projection_reserved_source_work_uppers": source_work_uppers(calls),
            "projection_reserved_private_source_work": dict(SOURCE_PRIVATE_WORK),
            "projection_reservation_scope": "Complete source recipe before effects; native results and partial observations remain separate."}
