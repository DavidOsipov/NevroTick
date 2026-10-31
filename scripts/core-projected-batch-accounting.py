"""Pure projected-batch attempt/guard observations and source reservations.

No I/O or execution occurs here. Reported attempts and passive observations are
separate from canonical source-work bounds and actual native outcomes. The
wrapper separately reuses the existing producer and selection work owners.
"""
from __future__ import annotations

import re


class Incomplete(ValueError):
    """Projected-batch source recipes, observations or work are incomplete."""


MARKER = "CORE_PROJECTED_BATCH_PROFILE"
GUARD_MARKER = "CORE_PROJECTED_BATCH_GUARD_PROFILE"
FIELDS = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections", "enclosures",
          "batches", "minima", "selections")
GUARD_FIELDS = ("garde_callbacks", "entry_read_attempts")
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 1024
U64_MAX = (1 << 64) - 1

# Frozen authoring inventory; Root must still admit current source/counts,
# actual native discovery, all cumulative limits and the complete cohort plan.
SOURCE_RECIPES = (
    ("single_compatible", (1,3,3,1,6,12,3,24,1,1,1,1,1,1)),
    ("linear_selection", (3,9,9,3,16,36,9,525,3,3,3,2,2,3)),
    ("capacity_neighbors", (17,51,51,17,86,204,51,2234,17,17,17,5,1,3)),
    ("context_admission", (8,24,24,8,42,96,24,680,8,8,8,12,1,1)),
    ("duplicate_precedence", (3,9,9,3,16,36,9,305,3,3,3,3,1,1)),
    ("permutation_retention", (3,9,9,3,16,36,9,618,3,3,3,3,1,6)),
    ("subtick_overlap", (2,6,6,2,11,24,6,231,2,2,2,1,1,1)),
    ("projected_batch_private_admission", (1,3,3,1,5,12,3,6,1,1,1,3,0,0)),
    ("projected_batch_private_priority", (3,9,9,3,14,36,9,18,3,3,3,0,0,0)),
    ("projected_batch_private_delegation", (2,6,6,2,9,24,6,14,2,2,2,2,0,0)),
    ("projected_batch_private_errors", (0,) * 14),
)
SOURCE_TESTS = (
    "genuine_compatible_projection_reaches_real_batch_selection",
    "linear_sources_preserve_original_ages_and_configured_minimum",
    "capacity_neighbors_keep_every_entry_and_the_original_minimum",
    "equal_numeric_intervals_still_require_exact_ordered_context_admission",
    "context_failure_precedes_duplicates_and_real_duplicate_cause_is_preserved",
    "permutations_and_repeated_selection_retain_the_original_complete_borrow",
    "disjoint_subtick_inputs_keep_rounding_overlap_without_physical_claims",
    "projected_batch::projected_batch_guards::projected_batch_private_admission",
    "projected_batch::projected_batch_guards::projected_batch_private_priority",
    "projected_batch::projected_batch_guards::projected_batch_private_delegation",
    "projected_batch::projected_batch_guards::projected_batch_private_errors",
)
SOURCE_PRIVATE_CASES = (
    "projected_batch_private_admission", "projected_batch_private_priority",
    "projected_batch_private_delegation", "projected_batch_private_errors",
)
GUARD_RECIPES = (
    ("projected_batch_private_admission", (3, 5)),
    ("projected_batch_private_priority", (9, 20)),
    ("projected_batch_private_delegation", (2, 12)),
    ("projected_batch_private_errors", (0, 0)),
)
GUARD_TESTS = SOURCE_TESTS[-4:]
# At n<=16: new/admit/early-cap/rule/two predicates/two caps/build = 9
# authored entries plus 3*n adapter entries. Compatibility getters use at most
# 2*n*5 requests and staging n more. Sixteen initial scratch copies and n-1
# occupied overwrites give31. These are canonical bounds, not observations or
# arbitrary-mutant/CPU/RSS guarantees. Garde callbacks are overlapping subsets.
SOURCE_WORK_PER_BATCH = {
    "constructor_authored_helper_entries_upper": 57,
    "raw_garde_validate_calls_upper": 1,
    "raw_garde_rule_callbacks_upper": 1,
    "entry_read_attempts_upper": 48,
    "internal_product_getter_requests_upper": 176,
    "scratch_Copy_assignments_upper": 31,
    "delegated_batch_attempts_upper": 1,
}
# A distinct counted entries-view getter precedes each explicit wrapper Debug
# format. The current public error helper returns a fixed error on unexpected
# success without formatting the batch. Getter requests therefore conservatively
# cover explicit formats in retained prefixes; the nominal twelve source formats
# are not a measured or universal per-batch cap.
SOURCE_WORK_PER_GETTER = {"wrapper_debug_format_calls_upper": 1}
# Direct raw/helper/error calls only: public factory bodies and genuine producer
# calls are already charged by the fourteen-field attempts. Authored entries
# include nested rule/predicate/adapters, with derived validate separate. These
# overlapping subsets are never added as extra authored entries or observations.
SOURCE_PRIVATE_WORK = {
    "direct_raw_authored_helper_entries_upper": 91,
    "direct_raw_garde_validate_calls_upper": 10,
    "direct_raw_garde_rule_callbacks_upper": 10,
    "direct_raw_entry_read_attempts_upper": 22,
    "direct_raw_internal_product_getter_requests_upper": 100,
}
SOURCE_RECIPE_STATUS = "source-authored-projected-batch-recipes-awaiting-root-review"
LITERAL_DOCTESTS_ADDED = 0
_DECIMAL = r"(0|[1-9][0-9]{0,19})"


def _recipes(rows, tests, fields, *, public):
    """Admit one closed source/test inventory; pending data cannot become zero."""
    if (type(public) is not bool or type(rows) is not tuple or not 0 < len(rows) <= 32
            or type(tests) is not tuple or len(tests) != len(rows)
            or any(type(name) is not str or not 0 < len(name) <= 256 for name in tests)
            or len(set(tests)) != len(tests)
            or type(SOURCE_PRIVATE_CASES) is not tuple
            or any(type(name) is not str for name in SOURCE_PRIVATE_CASES)
            or len(set(SOURCE_PRIVATE_CASES)) != len(SOURCE_PRIVATE_CASES)):
        raise Incomplete("Projected-batch source/test recipes remain pending or malformed.")
    result = {}
    for row in rows:
        if type(row) is not tuple or len(row) != 2:
            raise Incomplete("Projected-batch source row is not an exact case/vector pair.")
        case, values = row
        if (type(case) is not str or re.fullmatch(r"[a-z_]{1,64}", case) is None
                or case in result or type(values) is not tuple or len(values) != len(fields)
                or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)
                or public and not any(values) and case not in SOURCE_PRIVATE_CASES):
            raise Incomplete("Projected-batch source recipe is unowned, duplicated or unbounded.")
        result[case] = values
    return result


def case_recipes():
    """Return actual source-authored attempt ceilings, never native observations."""
    recipes = _recipes(SOURCE_RECIPES, SOURCE_TESTS, FIELDS, public=True)
    if not set(SOURCE_PRIVATE_CASES) <= set(recipes):
        raise Incomplete("Projected-batch private scope is absent from the source inventory.")
    return {case: dict(zip(FIELDS, values, strict=True)) for case, values in recipes.items()}


def guard_recipes():
    """Return closed passive-observer recipes for genuine private scopes."""
    case_recipes()
    recipes = _recipes(GUARD_RECIPES, GUARD_TESTS, GUARD_FIELDS, public=False)
    if set(recipes) != set(SOURCE_PRIVATE_CASES):
        raise Incomplete("Projected-batch guard inventory must cover every private source scope exactly.")
    tests = dict(zip((case for case, _ in SOURCE_RECIPES), SOURCE_TESTS, strict=True))
    if any(tests[case] != test for case, test in zip(recipes, GUARD_TESTS, strict=True)):
        raise Incomplete("Projected-batch guard and public markers must share their actual private test owner.")
    return {case: dict(zip(GUARD_FIELDS, values, strict=True)) for case, values in recipes.items()}


def _counts(output, *, guard, require_complete):
    """Parse complete bounded lines and retain well-formed failed-test prefixes."""
    if (type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS
            or type(guard) is not bool or type(require_complete) is not bool):
        raise Incomplete("Projected-batch observations require bounded text and Boolean modes.")
    marker, fields, tests, named = (
        (GUARD_MARKER, GUARD_FIELDS, GUARD_TESTS, guard_recipes()) if guard else
        (MARKER, FIELDS, SOURCE_TESTS, case_recipes()))
    recipes = {case: tuple(values[field] for field in fields) for case, values in named.items()}
    progress = r"(?:test (?:" + "|".join(re.escape(name) for name in tests) + r") \.\.\. )?"
    pattern = re.compile(progress + marker + r" case=([a-z_]+)"
                         + "".join(" " + name + "=" + _DECIMAL for name in fields))
    recorded = {}
    lines = output.split("\n")
    for index, raw in enumerate(lines):
        # The longer guard marker does not contain the public marker; each
        # family is received independently without borrowing the other's rows.
        if marker not in raw:
            continue
        if index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Projected-batch marker lacks its complete bounded native line.")
        match = pattern.fullmatch(raw[:-1] if raw.endswith("\r") else raw)
        if match is None:
            raise Incomplete("Projected-batch marker is malformed or has an unowned progress prefix.")
        case, *values = match.groups()
        values = tuple(int(value) for value in values)
        if (case not in recipes or case in recorded
                or any(value > U64_MAX for value in values)
                or any(value > maximum for value, maximum in zip(values, recipes.get(case, ()), strict=True))):
            raise Incomplete("Projected-batch row is unowned, duplicated or exceeds its source recipe.")
        if raw.startswith("test ") and not raw.startswith(
                "test " + tests[list(recipes).index(case)] + " ... "):
            raise Incomplete("Projected-batch progress name and case differ.")
        if not guard and not any(values) and any(recipes[case]):
            raise Incomplete("Projected-batch public source scope reports no prepaid attempt.")
        recorded[case] = values
    missing = [case for case in recipes if case not in recorded]
    partial = [case for case, values in recorded.items() if values != recipes[case]]
    complete = not missing and not partial
    subtotals = ({field: sum(values[index] for values in recorded.values())
                  for index, field in enumerate(fields)} if recorded else None)
    if subtotals is not None and any(value > U64_MAX for value in subtotals.values()):
        raise Incomplete("Projected-batch observed subtotal exceeds exact u64 arithmetic.")
    if require_complete and not complete:
        raise Incomplete("Projected-batch observations omit scopes or differ from full source recipes.")
    prefix = "projected_batch_guard" if guard else "projected_batch"
    return {prefix + "_counter_records": len(recorded),
            prefix + "_profiles": {case: dict(zip(fields, values, strict=True)) for case, values in recorded.items()},
            prefix + "_missing_cases": missing, prefix + "_partial_cases": partial,
            prefix + "_observation_pending": not complete,
            prefix + "_counter_observation_complete": complete,
            prefix + "_observation_status": "complete-source-recipe" if complete else "partial-source-recipe" if recorded else "missing",
            prefix + "_recorded_call_subtotals": subtotals,
            prefix + "_observed_calls": dict(subtotals) if complete else None,
            prefix + "_observation_scope": (
                "Actual private callback/adapter observations only; no native outcome or canonical-work measurement."
                if guard else "Reported explicit public attempts only; private work and native outcomes remain separate.")}


def projected_batch_counts(output, *, require_complete=False):
    """Receive this disjoint suite's fourteen public-attempt fields."""
    return _counts(output, guard=False, require_complete=require_complete)


def guard_counts(output, *, require_complete=False):
    """Receive passive callback/read attempts; absent data remains None/pending."""
    return _counts(output, guard=True, require_complete=require_complete)


def _work_vector(value):
    """Reject pending, malformed or non-u64 canonical source-work vectors."""
    if (type(value) is not dict or not 0 < len(value) <= 128
            or any(type(name) is not str or not 0 < len(name) <= 160
                   or type(amount) is not int or not 0 <= amount <= U64_MAX
                   for name, amount in value.items())):
        raise Incomplete("Projected-batch source work remains pending or unbounded.")
    return value


def source_work_uppers(calls):
    """Reserve only the new module's canonical work from its public attempts.

    The wrapper separately reuses the existing enclosure producer and selection
    calculators for this same disjoint suite, exactly once. Direct private raw
    helper work has a separate reservation; it must not recount public factory
    bodies or genuine producer work already represented by these attempts.
    """
    if (type(calls) is not dict or set(calls) != set(FIELDS)
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in calls.values())):
        raise Incomplete("Projected-batch work requires every exact public-attempt field.")
    coefficients = _work_vector(SOURCE_WORK_PER_BATCH)
    getter_coefficients = _work_vector(SOURCE_WORK_PER_GETTER)
    if "delegated_batch_attempts_upper" not in coefficients:
        raise Incomplete("Projected-batch delegated-batch source bound is absent.")
    if coefficients["delegated_batch_attempts_upper"] != 1:
        raise Incomplete("A projected-batch attempt may delegate at most one batch construction.")
    if set(coefficients) & set(getter_coefficients):
        raise Incomplete("Projected-batch constructor/getter work ownership overlaps.")
    work = {name: calls["batches"] * amount for name, amount in coefficients.items()}
    work.update({name: calls["getters"] * amount for name, amount in getter_coefficients.items()})
    return dict(_work_vector(work))


def full_package_reservation():
    """Reserve complete source attempts and disjoint direct-private work."""
    recipes = case_recipes()
    calls = {field: sum(row[field] for row in recipes.values()) for field in FIELDS}
    if any(value > U64_MAX for value in calls.values()):
        raise Incomplete("Projected-batch source attempt total exceeds u64.")
    return {"projected_batch_reserved_calls": calls,
            "projected_batch_reserved_module_work_uppers": source_work_uppers(calls),
            "projected_batch_reserved_private_source_work": dict(_work_vector(SOURCE_PRIVATE_WORK)),
            "projected_batch_reservation_scope": "Complete canonical source reservation before effects; no observed native outcome."}
