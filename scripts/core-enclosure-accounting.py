"""Pure strict accounting for the fixture projection-to-Q33 enclosure owner.

Reported public attempts, canonical private-work uppers and native outcomes are
separate. No I/O or execution occurs here. Missing observations remain None.
"""
from __future__ import annotations

import re


class Incomplete(ValueError):
    """Enclosure observations or source reservations are incomplete."""


MARKER = "CORE_ENCLOSURE_PROFILE"
FIELDS = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections", "enclosures")
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 1024
U64_MAX = (1 << 64) - 1
# Filled only from the actual source authors; these are no executed observations.
SOURCE_RECIPES = (
    ("linear_latest", (1, 3, 3, 1, 5, 12, 3, 208, 1, 1, 1)),
    ("linear_older", (1, 3, 3, 1, 5, 12, 3, 208, 1, 1, 1)),
    ("tiny_radius", (1, 3, 3, 1, 5, 12, 3, 208, 1, 1, 1)),
    ("fractional_signs", (2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2)),
    ("asymmetric_endpoints", (2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2)),
    ("subtick_overlap", (2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2)),
    ("signed_zero", (2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2)),
    ("large_radius", (1, 3, 3, 1, 7, 12, 3, 256, 2, 2, 2)),
    ("identity_labels", (3, 9, 9, 3, 15, 36, 9, 624, 3, 3, 3)),
    ("enclosure_private_rules", (0,) * 11),
    ("enclosure_private_admission", (0,) * 11),
    ("enclosure_private_facts", (0,) * 11),
    ("enclosure_private_ranges", (0,) * 11),
    ("enclosure_private_fractions", (0,) * 11),
    ("enclosure_private_errors", (0,) * 11),
)
SOURCE_TESTS = (
    "latest_linear_anchor_has_exact_q33_endpoints",
    "older_linear_anchor_retains_its_larger_distance_and_original_age",
    "tiny_radius_is_enclosed_before_binary64_endpoint_rounding_can_erase_it",
    "public_half_tick_centers_round_outward_for_both_signs",
    "public_fraction_cancellation_has_independent_endpoint_exactness",
    "disjoint_subtick_fixture_operands_can_have_overlapping_integer_enclosures",
    "signed_zero_terms_and_computed_radius_survive_original_projection_retention",
    "valid_large_radius_projection_is_rejected_without_changing_history_or_input",
    "actual_source_labels_and_complete_context_survive_repeated_reads_and_copy",
    "projection_enclosure::enclosure_guards::enclosure_private_rules",
    "projection_enclosure::enclosure_guards::enclosure_private_admission",
    "projection_enclosure::enclosure_guards::enclosure_private_facts",
    "projection_enclosure::enclosure_guards::enclosure_private_ranges",
    "projection_enclosure::enclosure_guards::enclosure_private_fractions",
    "projection_enclosure::enclosure_guards::enclosure_private_errors",
)
SOURCE_PRIVATE_CASE_WORK = {
    "rules": {"source_helper_entries_upper": 86, "raw_garde_validate_calls_upper": 0},
    "admission": {"source_helper_entries_upper": 194, "raw_garde_validate_calls_upper": 36},
    "facts": {"source_helper_entries_upper": 669, "raw_garde_validate_calls_upper": 23},
    "ranges": {"source_helper_entries_upper": 478, "raw_garde_validate_calls_upper": 17},
    "fractions": {"source_helper_entries_upper": 138, "raw_garde_validate_calls_upper": 0},
    "errors": {"source_helper_entries_upper": 17, "raw_garde_validate_calls_upper": 0},
}
# Includes nested enclosure-owned functions plus one test entry/profile helper
# per scope. Callbacks/predicates are overlapping subsets, never extra observed
# totals. Twenty direct checked primitives per helper entry deliberately exceeds
# every canonical helper (the largest direct helper uses at most four); it also
# covers the complete twenty-primitive raw scalar path. This is source reservation.
SOURCE_PRIVATE_WORK = {
    "source_helper_entries_upper": 1582,
    "raw_garde_validate_calls_upper": 76,
    "raw_garde_rule_callbacks_upper": 180,
    "raw_predicate_calls_upper": 208,
    "checked_scalar_primitive_calls_upper": 31640,
}
SOURCE_RECIPE_STATUS = "authored-source-recipes-pending-root-candidate-binding"
LITERAL_DOCTESTS_ADDED = 0
# Both endpoints are fixed-size paths: <=28 enclosure-owned helper entries,
# <=20 checked integer primitives, and one validation with two raw callbacks.
# The bridge owns five immutable getter/source-chain calls and at most one
# interval construction, whose existing owner invokes two validates/one rule.
ENCLOSURE_WORK_PER_CALL = {
    "enclosure_bridge_entry_calls_upper": 1,
    "enclosure_wrapper_debug_format_calls_upper": 1,
    "enclosure_projection_getter_chain_calls_upper": 5,
    "enclosure_raw_helper_entries_upper": 28,
    "enclosure_raw_garde_validate_calls_upper": 1,
    "enclosure_raw_garde_rule_callbacks_upper": 2,
    "enclosure_checked_scalar_primitive_calls_upper": 20,
    "enclosure_interval_constructor_attempts_upper": 1,
    "enclosure_interval_garde_validate_calls_upper": 2,
    "enclosure_interval_rule_calls_upper": 1,
}
_DECIMAL = r"(0|[1-9][0-9]{0,19})"


def _recipes():
    """Require a finite exact source/test-name inventory before interpreting rows."""
    if (type(SOURCE_RECIPES) is not tuple or not 0 < len(SOURCE_RECIPES) <= 32
            or type(SOURCE_TESTS) is not tuple or len(SOURCE_TESTS) != len(SOURCE_RECIPES)
            or len(set(SOURCE_TESTS)) != len(SOURCE_TESTS)
            or any(type(name) is not str or not 0 < len(name) <= 256 for name in SOURCE_TESTS)):
        raise Incomplete("Enclosure named source recipes/test inventory are pending.")
    result = {}
    for case, values in SOURCE_RECIPES:
        if (type(case) is not str or re.fullmatch(r"[a-z_]{1,64}", case) is None
                or case in result or type(values) is not tuple or len(values) != len(FIELDS)
                or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)):
            raise Incomplete("Enclosure source recipe is unknown, duplicated or unbounded.")
        result[case] = values
    return result


def case_recipes():
    """Return named source ceilings, never native measurements."""
    return {case: dict(zip(FIELDS, counts, strict=True)) for case, counts in _recipes().items()}


def enclosure_counts(output, *, require_complete=False):
    """Parse complete native lines, retaining missing and failed-prefix attempts.

    Exact source recipe completion is neither a native outcome nor an assertion
    pass. Only the separately named private recipes may report all-zero public
    attempts, and they still reserve their actual private work independently.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Enclosure observations require bounded text and Boolean completeness mode.")
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
            raise Incomplete("Enclosure marker lacks its complete bounded native line.")
        match = pattern.fullmatch(raw[:-1] if raw.endswith("\r") else raw)
        if match is None:
            raise Incomplete("Enclosure marker is malformed or outside its exact native prefix.")
        case, *values = match.groups()
        values = tuple(int(value) for value in values)
        if (case not in recipes or case in recorded or any(value > U64_MAX for value in values)
                or any(value > maximum for value, maximum in zip(values, recipes.get(case, ()), strict=True))):
            raise Incomplete("Enclosure case is unknown, duplicated or exceeds its source recipe.")
        if raw.startswith("test ") and not raw.startswith(
                "test " + SOURCE_TESTS[list(recipes).index(case)] + " ... "):
            raise Incomplete("Enclosure libtest prefix names a different source case.")
        if not any(values) and any(recipes[case]):
            raise Incomplete("Public enclosure case reports no prepaid source attempt.")
        recorded[case] = values
    missing = [case for case in recipes if case not in recorded]
    partial = [case for case, values in recorded.items() if values != recipes[case]]
    complete = not missing and not partial
    subtotals = ({field: sum(values[index] for values in recorded.values())
                  for index, field in enumerate(FIELDS)} if recorded else None)
    if require_complete and not complete:
        raise Incomplete("Enclosure observations omit cases or differ from complete source recipes.")
    return {"enclosure_counter_records": len(recorded),
            "enclosure_profiles": {case: dict(zip(FIELDS, values, strict=True)) for case, values in recorded.items()},
            "enclosure_missing_cases": missing, "enclosure_partial_cases": partial,
            "enclosure_observation_pending": not complete,
            "enclosure_counter_observation_complete": complete,
            "enclosure_observation_status": "complete-source-recipe" if complete else "partial-source-recipe" if recorded else "missing",
            "enclosure_recorded_call_subtotals": subtotals,
            "enclosure_observed_calls": dict(subtotals) if complete else None,
            "enclosure_observation_scope": "Reported public attempts only; private source uppers and native outcomes remain separate."}


def _work_vector(value):
    """Reject pending, unknown-shaped or unbounded canonical work reservations."""
    if (type(value) is not dict or not 0 < len(value) <= 128
            or any(type(name) is not str or not 0 < len(name) <= 160
                   or type(amount) is not int or not 0 <= amount <= U64_MAX
                   for name, amount in value.items())):
        raise Incomplete("Enclosure private/source work remains pending or unbounded.")
    return value


def source_work_uppers(calls):
    """Reserve this owner's producer and bridge work without borrowing telemetry.

    Genuine history construction is charged to this family independently of the
    old projection suite. Bridge work has no history, normalization or fit call.
    These are conservative canonical source bounds, never arbitrary-mutant work,
    CPU, RSS, physical confidence or measured private-operation counts.
    """
    if (type(calls) is not dict or set(calls) != set(FIELDS)
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in calls.values())):
        raise Incomplete("Enclosure source work requires every exact public-attempt field.")
    appends, projections, terms = calls["appends"], calls["projections"], calls["terms"]
    work = {"charged_fit_work_upper": appends, "point_admissions_upper": 64 * appends,
            "history_normalization_entries_upper": appends, "history_statistics_entries_upper": appends,
            "history_statistics_inner_fit_upper": appends, "history_duration_operations_upper": 64 * appends,
            "history_generated_point_admissions_upper": 64 * appends,
            "history_fit_input_constructions_upper": appends, "history_staging_Copy_assignments_upper": 127 * appends,
            "projection_duration_operations_upper": 2 * projections,
            "projection_internal_product_getter_requests_upper": 12 * projections,
            "projection_raw_candidate_constructions_upper": terms, "projection_candidate_validation_calls_upper": terms,
            "projection_first_error_calls_upper": terms, "projection_term_garde_rule_callbacks_upper": 3 * terms,
            "projection_term_predicate_calls_upper": 6 * terms,
            "projection_checked_integer_helper_calls_upper": 2 * projections,
            "projection_normalization_primitive_calls_upper": 6 * projections,
            "projection_arithmetic_primitive_calls_upper": 5 * projections}
    work.update({name: calls["enclosures"] * amount for name, amount in _work_vector(ENCLOSURE_WORK_PER_CALL).items()})
    return work


def full_package_reservation():
    """Reserve every source case and private guard before any package effects."""
    recipes = _recipes()
    calls = {field: sum(values[index] for values in recipes.values()) for index, field in enumerate(FIELDS)}
    return {"enclosure_reserved_calls": calls,
            "enclosure_reserved_source_work_uppers": source_work_uppers(calls),
            "enclosure_reserved_private_source_work": dict(_work_vector(SOURCE_PRIVATE_WORK)),
            "enclosure_reservation_scope": "Complete canonical source recipe before effects; no native result or private measurement."}
