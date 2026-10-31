"""Pure observations and bounded source work for the fixture combination suite.

No I/O or execution occurs here. Exact public attempt rows, canonical module
work uppers and native assertion outcomes remain separate. The coordinator owns
the existing history/projection/principal producer chain and charges it once
using producer_calls; this helper does not import the verification monolith.
"""
from __future__ import annotations

import re


class Incomplete(ValueError):
    """Combination observations or their finite source reservation are incomplete."""


MARKER = "CORE_COMBINATION_PROFILE"
FIELDS = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections", "enclosures",
          "batches", "minima", "parameters", "strata", "seeds", "advances",
          "state_clones", "value_equalities", "state_formats", "uncertainties", "combinations")
PRODUCER_FIELDS = FIELDS[:20]
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 2048
U64_MAX = (1 << 64) - 1

# Literal ceilings from the twelve held public-input case entries. These are
# authored attempt recipes, never inferred execution observations or outcomes.
SOURCE_RECIPES = (
    ("combination_distinct_weights_and_principal_disagreement", (2,6,6,2,7,24,6,44,2,2,2,1,1,1,2,1,1,0,0,0,2,1)),
    ("combination_unavailable_and_checked_uncertainty", (1,3,3,1,4,12,3,21,1,1,1,1,1,1,1,1,1,0,0,0,1,1)),
    ("combination_singleton_signed_zero_and_retained", (1,3,3,1,4,12,3,52,1,1,1,1,1,1,1,1,2,0,0,0,1,2)),
    ("combination_raw_uncertainty_boundaries", (1,3,3,1,4,12,3,326,1,1,1,1,1,1,1,1,1,0,0,0,40,15)),
    ("combination_three_source_permutation_and_zero_phase_sd", (3,9,9,3,10,36,9,73,3,3,3,1,1,1,3,1,1,0,0,0,3,2)),
    ("combination_metadata_association_and_context", (6,18,18,6,22,72,18,68,6,6,6,1,1,1,6,1,1,0,0,0,7,10)),
    ("combination_zero_denominators_and_phase_neighbor", (4,12,12,4,14,48,12,55,4,4,4,2,2,2,4,2,2,0,0,0,8,6)),
    ("combination_finite_square_failures_and_neighbors", (2,6,6,2,7,24,6,31,2,2,2,1,1,1,2,1,1,0,0,0,13,12)),
    ("combination_capacity_and_weight_sum_overflow", (16,48,48,16,49,192,48,148,16,16,16,1,1,1,16,1,1,0,0,0,48,4)),
    ("combination_variance_mean_underflow_and_neighbor", (2,6,6,2,7,24,6,36,2,2,2,1,1,1,2,1,1,0,0,0,3,2)),
    ("combination_weighted_rate_subnormal_and_neighbor", (2,6,6,2,7,24,6,36,2,2,2,1,1,1,2,1,1,0,0,0,3,2)),
    ("combination_canonical_reduction_rounding_order", (3,9,9,3,10,36,9,61,3,3,3,1,1,1,3,1,1,0,0,0,3,2)),
)
SOURCE_TESTS = tuple("combination::combination_cases::" + case for case, _ in SOURCE_RECIPES)
SOURCE_RECIPE_STATUS = "source-authored-combination-recipes-awaiting-root-review"
LITERAL_DOCTESTS_ADDED = 0

# For n<=16, successful multisource work is18*n+5 binary operations and3 roots.
# Absent/singleton paths use fewer. Each uncertainty constructor has1 validate,
# 3 callbacks and <=3 first-error predicate calls. Final publication adds3
# uncertainty predicates. These source-owned bounds cover canonical selected
# units only, not every private helper, arbitrary mutants, CPU/RSS or observed
# invocation counts. Callback/predicate subsets are overlapping, not additive.
SOURCE_WORK_PER_UNCERTAINTY = {
    "uncertainty_constructor_attempts_upper": 1,
    "uncertainty_garde_validation_calls_upper": 1,
    "uncertainty_garde_rule_callbacks_upper": 3,
    "uncertainty_predicate_calls_upper": 6,
}
SOURCE_WORK_PER_COMBINATION = {
    "kernel_checked_binary_calls_upper": 293,
    "kernel_checked_square_root_calls_upper": 3,
    "kernel_positive_denominator_guards_upper": 48,
    "kernel_supporter_reduction_visits_upper": 16,
    "kernel_record_lookup_visits_upper": 256,
    "kernel_metadata_pointer_comparisons_upper": 256,
    # Admission: <=120 duplicate comparisons +256 witness-membership checks
    # +16 principal-membership checks +1 principal-label equality. The record
    # lookup's separate <=256 label comparisons have their own field above.
    "kernel_metadata_label_comparisons_upper": 393,
    # Source chains through original entries/projections/contexts are bounded
    # by16-by16 association and lookup loops, plus fixed publication getters.
    "kernel_internal_product_getter_requests_upper": 4096,
    "kernel_final_uncertainty_predicate_calls_upper": 3,
}
_DECIMAL = r"(0|[1-9][0-9]{0,19})"


def _recipes():
    """Admit the bounded exact case/test inventory before parsing or charging."""
    if (type(SOURCE_RECIPES) is not tuple or not 0 < len(SOURCE_RECIPES) <= 32
            or type(SOURCE_TESTS) is not tuple or len(SOURCE_TESTS) != len(SOURCE_RECIPES)
            or any(type(name) is not str or not 0 < len(name) <= 256 for name in SOURCE_TESTS)
            or len(set(SOURCE_TESTS)) != len(SOURCE_TESTS)):
        raise Incomplete("Combination source/test recipes are pending or malformed.")
    recipes = {}
    for row in SOURCE_RECIPES:
        if type(row) is not tuple or len(row) != 2:
            raise Incomplete("Combination source row is not an exact case/vector pair.")
        case, values = row
        if (type(case) is not str or re.fullmatch(r"[a-z_]{1,96}", case) is None
                or case in recipes or type(values) is not tuple or len(values) != len(FIELDS)
                or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)
                or not any(values)):
            raise Incomplete("Combination source recipe is unknown, duplicated or unbounded.")
        recipes[case] = values
    return recipes


def case_recipes():
    """Return immutable-source attempt recipes as fresh mappings, not observations."""
    return {case: dict(zip(FIELDS, values, strict=True)) for case, values in _recipes().items()}


def combination_counts(output, *, require_complete=False):
    """Parse exact terminated rows and retain absent/failed-prefix distinctions.

    A healthy complete stream contains every exact authored row. Partial rows
    remain prepaid attempts and never lower the source reservation. Unknown
    combination-family markers and incorrect libtest case/name pairs fail closed.
    Completion here is transport/recipe completion, never an assertion outcome.
    """
    if (type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS
            or type(require_complete) is not bool):
        raise Incomplete("Combination observations require bounded text and a Boolean mode.")
    recipes = _recipes()
    tests = dict(zip(recipes, SOURCE_TESTS, strict=True))
    progress = r"(?:test (?:" + "|".join(re.escape(name) for name in SOURCE_TESTS) + r") \.\.\. )?"
    pattern = re.compile(progress + MARKER + r" case=([a-z_]+)"
                         + "".join(" " + field + "=" + _DECIMAL for field in FIELDS))
    recorded = {}
    lines = output.split("\n")
    for index, raw in enumerate(lines):
        if "CORE_COMBINATION_" not in raw:
            continue
        if index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Combination marker lacks its complete bounded native line.")
        match = pattern.fullmatch(raw[:-1] if raw.endswith("\r") else raw)
        if match is None:
            raise Incomplete("Combination marker is malformed or has an unowned native prefix.")
        case, *values = match.groups()
        values = tuple(int(value) for value in values)
        if (case not in recipes or case in recorded
                or any(value > U64_MAX for value in values)
                or any(value > maximum for value, maximum in zip(values, recipes.get(case, ()), strict=True))):
            raise Incomplete("Combination case is unknown, duplicated or exceeds its source recipe.")
        if raw.startswith("test ") and not raw.startswith("test " + tests[case] + " ... "):
            raise Incomplete("Combination libtest prefix names a different source case.")
        if not any(values):
            raise Incomplete("Combination public case reports no prepaid source attempt.")
        recorded[case] = values
    missing = [case for case in recipes if case not in recorded]
    partial = [case for case, values in recorded.items() if values != recipes[case]]
    complete = not missing and not partial
    subtotals = ({field: sum(values[index] for values in recorded.values())
                  for index, field in enumerate(FIELDS)} if recorded else None)
    if subtotals is not None and any(value > U64_MAX for value in subtotals.values()):
        raise Incomplete("Combination observation subtotal exceeds exact u64 arithmetic.")
    if require_complete and not complete:
        raise Incomplete("Combination complete package profiles are missing or partial.")
    return {"combination_counter_records": len(recorded),
            "combination_profiles": {case: dict(zip(FIELDS, values, strict=True))
                                     for case, values in recorded.items()},
            "combination_missing_cases": missing, "combination_partial_cases": partial,
            "combination_observation_pending": not complete,
            "combination_counter_observation_complete": complete,
            "combination_observation_status": "complete-source-recipe" if complete else "partial-source-recipe" if recorded else "missing",
            "combination_recorded_call_subtotals": subtotals,
            "combination_observed_calls": dict(subtotals) if complete else None,
            "combination_observation_scope": "Reported public attempts only; source uppers and native outcomes remain separate."}


def _calls(calls):
    """Require the complete exact-u64 public vector, including explicit zero fields."""
    if (type(calls) is not dict or set(calls) != set(FIELDS)
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in calls.values())):
        raise Incomplete("Combination work requires every exact public-attempt field.")
    return calls


def producer_calls(calls):
    """Project the original twenty producer fields for the coordinator's one charge."""
    calls = _calls(calls)
    return {field: calls[field] for field in PRODUCER_FIELDS}


def _work_vector(work):
    """Reject unknown-shaped, overlapping or unbounded source-work coefficients."""
    if (type(work) is not dict or not 0 < len(work) <= 64
            or any(type(key) is not str or not 0 < len(key) <= 160
                   or type(value) is not int or not 0 <= value <= U64_MAX
                   for key, value in work.items())):
        raise Incomplete("Combination canonical module work is missing or unbounded.")
    return work


def source_work_uppers(calls):
    """Reserve only combination-owned work; the producer chain is charged elsewhere."""
    calls = _calls(calls)
    uncertainty = _work_vector(SOURCE_WORK_PER_UNCERTAINTY)
    kernel = _work_vector(SOURCE_WORK_PER_COMBINATION)
    if set(uncertainty) & set(kernel):
        raise Incomplete("Combination uncertainty/kernel work ownership overlaps.")
    work = {key: calls["uncertainties"] * value for key, value in uncertainty.items()}
    work.update({key: calls["combinations"] * value for key, value in kernel.items()})
    return dict(_work_vector(work))


def full_package_reservation():
    """Reserve the complete held source suite before any native package invocation."""
    recipes = case_recipes()
    calls = {field: sum(row[field] for row in recipes.values()) for field in FIELDS}
    _calls(calls)
    return {"combination_reserved_calls": calls,
            "combination_reserved_module_work_uppers": source_work_uppers(calls),
            "combination_reservation_scope": "Complete source reservation before effects; no native result or private measurement."}
