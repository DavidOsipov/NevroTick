"""Pure observations and bounded source work for the fixture combination suite.

No I/O or execution occurs here. Exact public attempt rows, canonical module
work uppers and native assertion outcomes remain separate. The coordinator owns
the existing history/projection/principal producer chain and charges it once
using producer_calls; this helper does not import the verification monolith.
"""
from __future__ import annotations

import re
from typing import Literal, NamedTuple, Protocol, TypeAlias, TypedDict, TypeGuard, assert_never

CounterField: TypeAlias = Literal[
    "histories", "appends", "observations", "contexts", "monotonic",
    "coordinates", "exchanges", "getters", "terms", "projections", "enclosures",
    "batches", "minima", "parameters", "strata", "seeds", "advances",
    "state_clones", "value_equalities", "state_formats", "uncertainties", "combinations",
]


class ProducerCalls(TypedDict):
    histories: int
    appends: int
    observations: int
    contexts: int
    monotonic: int
    coordinates: int
    exchanges: int
    getters: int
    terms: int
    projections: int
    enclosures: int
    batches: int
    minima: int
    parameters: int
    strata: int
    seeds: int
    advances: int
    state_clones: int
    value_equalities: int
    state_formats: int


class PublicCalls(ProducerCalls):
    uncertainties: int
    combinations: int


class _ObservationCommon(TypedDict):
    combination_counter_records: int
    combination_profiles: dict[str, PublicCalls]
    combination_missing_cases: list[str]
    combination_partial_cases: list[str]
    combination_observation_scope: str


class CompleteObservation(_ObservationCommon):
    combination_observation_status: Literal["complete-source-recipe"]
    combination_observation_pending: Literal[False]
    combination_counter_observation_complete: Literal[True]
    combination_recorded_call_subtotals: PublicCalls
    combination_observed_calls: PublicCalls


class PartialObservation(_ObservationCommon):
    combination_observation_status: Literal["partial-source-recipe"]
    combination_observation_pending: Literal[True]
    combination_counter_observation_complete: Literal[False]
    combination_recorded_call_subtotals: PublicCalls
    combination_observed_calls: None


class MissingObservation(_ObservationCommon):
    combination_observation_status: Literal["missing"]
    combination_observation_pending: Literal[True]
    combination_counter_observation_complete: Literal[False]
    combination_recorded_call_subtotals: None
    combination_observed_calls: None


CombinationObservation: TypeAlias = CompleteObservation | PartialObservation | MissingObservation


class CombinationReservation(TypedDict):
    combination_reserved_calls: PublicCalls
    combination_reserved_module_work_uppers: dict[str, int]
    combination_reservation_scope: str


class ObservedAttempts(NamedTuple):
    """Reported complete public attempts; never reserved work or native success."""

    calls: PublicCalls


class ReservedAttempts(NamedTuple):
    """Pre-effect source reservation, independent of reported attempts."""

    calls: PublicCalls


def complete_attempts(observation: CompleteObservation) -> ObservedAttempts:
    """Keep complete observation access separate from the reservation interface."""
    if (observation.get("combination_observation_status") != "complete-source-recipe"
            or observation.get("combination_counter_observation_complete") is not True
            or observation.get("combination_observation_pending") is not False):
        raise Incomplete("Complete attempts require a complete observation, never a partial prefix.")
    return ObservedAttempts(_calls(observation["combination_observed_calls"]))


def reserved_attempts(reservation: CombinationReservation) -> ReservedAttempts:
    """Project a source reservation into its distinct nominal unit."""
    return ReservedAttempts(_calls(reservation["combination_reserved_calls"]))


def observation_attempts(observation: CombinationObservation) -> ObservedAttempts | None:
    """An exhaustive immediate consumer never treats partial transport as complete."""
    if observation["combination_observation_status"] == "complete-source-recipe":
        return complete_attempts(observation)
    if observation["combination_observation_status"] == "partial-source-recipe":
        return None
    if observation["combination_observation_status"] == "missing":
        return None
    assert_never(observation)


def is_exact_dict(value: object) -> TypeGuard[dict[object, object]]:
    """Narrow unknown dictionary entries to object, never dynamic Any."""
    return type(value) is dict


class CombinationAccounting(Protocol):
    """Bind the actual checked implementation; a dynamic cast is insufficient."""

    def combination_counts(self, output: str, *, require_complete: bool = False) -> CombinationObservation: ...
    def case_recipes(self) -> dict[str, PublicCalls]: ...
    def producer_calls(self, calls: PublicCalls) -> ProducerCalls: ...
    def source_work_uppers(self, calls: PublicCalls) -> dict[str, int]: ...
    def full_package_reservation(self) -> CombinationReservation: ...


class Incomplete(ValueError):
    """Combination observations or their finite source reservation are incomplete."""


MARKER = "CORE_COMBINATION_PROFILE"
FIELDS: tuple[CounterField, ...] = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections", "enclosures",
          "batches", "minima", "parameters", "strata", "seeds", "advances",
          "state_clones", "value_equalities", "state_formats", "uncertainties", "combinations")
PRODUCER_FIELDS = FIELDS[:20]
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 2048
U64_MAX = (1 << 64) - 1

# Literal public-attempt ceilings from the twelve held case entries. These are
# authored attempt recipes, never inferred execution observations or outcomes.
# Direct private receiver attempts are separately source-reserved by the coordinator.
SOURCE_RECIPES: tuple[tuple[str, tuple[int, ...]], ...] = (
    ("combination_distinct_weights_and_principal_disagreement", (2,6,6,2,7,24,6,45,2,2,2,1,1,1,2,1,1,0,0,2,2,1)),
    ("combination_unavailable_and_checked_uncertainty", (1,3,3,1,4,12,3,22,1,1,1,1,1,1,1,1,1,0,0,3,1,2)),
    ("combination_singleton_signed_zero_and_retained", (1,3,3,1,4,12,3,67,1,1,1,1,1,1,1,1,2,0,0,0,1,2)),
    ("combination_raw_uncertainty_boundaries", (1,3,3,1,4,12,3,326,1,1,1,1,1,1,1,1,1,0,0,0,40,15)),
    ("combination_three_source_permutation_and_zero_phase_sd", (3,9,9,3,10,36,9,73,3,3,3,1,1,1,3,1,1,0,0,0,3,2)),
    ("combination_metadata_association_and_context", (6,18,18,6,22,72,18,76,6,6,6,2,2,2,6,2,2,0,0,0,7,10)),
    ("combination_zero_denominators_and_phase_neighbor", (4,12,12,4,14,48,12,55,4,4,4,2,2,2,4,2,2,0,0,0,8,6)),
    ("combination_finite_square_failures_and_neighbors", (2,6,6,2,7,24,6,31,2,2,2,1,1,1,2,1,1,0,0,0,13,12)),
    ("combination_capacity_and_weight_sum_overflow", (16,48,48,16,49,192,48,148,16,16,16,1,1,1,16,1,1,0,0,0,48,4)),
    ("combination_variance_mean_underflow_and_neighbor", (2,6,6,2,7,24,6,36,2,2,2,1,1,1,2,1,1,0,0,0,3,2)),
    ("combination_weighted_rate_subnormal_and_neighbor", (2,6,6,2,7,24,6,37,2,2,2,1,1,1,2,1,1,0,0,0,3,2)),
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
# The historical state_formats slot precharges each product Debug/Display
# dispatch. All five current observations select fixed-literal redaction arms,
# so the inherited one-format-call upper remains conservative. Product
# std::error::Error::source dispatches are separately precharged as getters.
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


def _recipes() -> dict[str, tuple[int, ...]]:
    """Admit the bounded exact case/test inventory before parsing or charging."""
    if (type(SOURCE_RECIPES) is not tuple or not 0 < len(SOURCE_RECIPES) <= 32
            or type(SOURCE_TESTS) is not tuple or len(SOURCE_TESTS) != len(SOURCE_RECIPES)
            or any(type(name) is not str or not 0 < len(name) <= 256 for name in SOURCE_TESTS)
            or len(set(SOURCE_TESTS)) != len(SOURCE_TESTS)):
        raise Incomplete("Combination source/test recipes are pending or malformed.")
    recipes: dict[str, tuple[int, ...]] = {}
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


def _counts_from_vector(values: tuple[int, ...]) -> PublicCalls:
    """Name the exact admitted 22-position vector without dynamic typed keys."""
    if (type(values) is not tuple or len(values) != len(FIELDS)
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)):
        raise Incomplete("Combination public vector has a different cardinality.")
    return {"histories": values[0], "appends": values[1], "observations": values[2],
            "contexts": values[3], "monotonic": values[4], "coordinates": values[5],
            "exchanges": values[6], "getters": values[7], "terms": values[8],
            "projections": values[9], "enclosures": values[10], "batches": values[11],
            "minima": values[12], "parameters": values[13], "strata": values[14],
            "seeds": values[15], "advances": values[16], "state_clones": values[17],
            "value_equalities": values[18], "state_formats": values[19],
            "uncertainties": values[20], "combinations": values[21]}


def case_recipes() -> dict[str, PublicCalls]:
    """Return immutable-source attempt recipes as fresh mappings, not observations."""
    return {case: _counts_from_vector(values) for case, values in _recipes().items()}


def combination_counts(output: str, *, require_complete: bool = False) -> CombinationObservation:
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
    recorded: dict[str, tuple[int, ...]] = {}
    lines = output.split("\n")
    for index, raw in enumerate(lines):
        if "CORE_COMBINATION_" not in raw:
            continue
        if index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Combination marker lacks its complete bounded native line.")
        match = pattern.fullmatch(raw[:-1] if raw.endswith("\r") else raw)
        if match is None:
            raise Incomplete("Combination marker is malformed or has an unowned native prefix.")
        case = match.group(1)
        values = tuple(int(match.group(index + 2)) for index in range(len(FIELDS)))
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
    subtotals = (_counts_from_vector(tuple(sum(values[index] for values in recorded.values())
                                         for index in range(len(FIELDS)))) if recorded else None)
    if subtotals is not None and any(subtotals[field] > U64_MAX for field in FIELDS):
        raise Incomplete("Combination observation subtotal exceeds exact u64 arithmetic.")
    if require_complete and not complete:
        raise Incomplete("Combination complete package profiles are missing or partial.")
    common: _ObservationCommon = {
        "combination_counter_records": len(recorded),
        "combination_profiles": {case: _counts_from_vector(values) for case, values in recorded.items()},
        "combination_missing_cases": missing, "combination_partial_cases": partial,
        "combination_observation_scope": "Reported public attempts only; source uppers and native outcomes remain separate."}
    if complete and subtotals is not None:
        return CompleteObservation(**common, combination_observation_status="complete-source-recipe",
                                   combination_observation_pending=False,
                                   combination_counter_observation_complete=True,
                                   combination_recorded_call_subtotals=subtotals,
                                   combination_observed_calls=subtotals.copy())
    if subtotals is not None:
        return PartialObservation(**common, combination_observation_status="partial-source-recipe",
                                  combination_observation_pending=True,
                                  combination_counter_observation_complete=False,
                                  combination_recorded_call_subtotals=subtotals,
                                  combination_observed_calls=None)
    return MissingObservation(**common, combination_observation_status="missing",
                              combination_observation_pending=True,
                              combination_counter_observation_complete=False,
                              combination_recorded_call_subtotals=None,
                              combination_observed_calls=None)


def _calls(calls: object) -> PublicCalls:
    """Require the complete exact-u64 public vector, including explicit zero fields."""
    if not is_exact_dict(calls) or set(calls) != set(FIELDS):
        raise Incomplete("Combination work requires every exact public-attempt field.")
    values: list[int] = []
    for field in FIELDS:
        value = calls[field]
        if type(value) is not int or not 0 <= value <= U64_MAX:
            raise Incomplete("Combination work requires every exact public-attempt field.")
        values.append(value)
    return _counts_from_vector(tuple(values))


def producer_calls(calls: PublicCalls) -> ProducerCalls:
    """Project the original twenty producer fields for the coordinator's one charge."""
    calls = _calls(calls)
    return {"histories": calls["histories"], "appends": calls["appends"],
            "observations": calls["observations"], "contexts": calls["contexts"],
            "monotonic": calls["monotonic"], "coordinates": calls["coordinates"],
            "exchanges": calls["exchanges"], "getters": calls["getters"], "terms": calls["terms"],
            "projections": calls["projections"], "enclosures": calls["enclosures"],
            "batches": calls["batches"], "minima": calls["minima"], "parameters": calls["parameters"],
            "strata": calls["strata"], "seeds": calls["seeds"], "advances": calls["advances"],
            "state_clones": calls["state_clones"], "value_equalities": calls["value_equalities"],
            "state_formats": calls["state_formats"]}


def _work_vector(work: object) -> dict[str, int]:
    """Reject unknown-shaped, overlapping or unbounded source-work coefficients."""
    if not is_exact_dict(work) or not 0 < len(work) <= 64:
        raise Incomplete("Combination canonical module work is missing or unbounded.")
    result: dict[str, int] = {}
    for key, value in work.items():
        if (type(key) is not str or not 0 < len(key) <= 160
                or type(value) is not int or not 0 <= value <= U64_MAX):
            raise Incomplete("Combination canonical module work is missing or unbounded.")
        result[key] = value
    return result


def source_work_uppers(calls: PublicCalls) -> dict[str, int]:
    """Reserve only combination-owned work; the producer chain is charged elsewhere."""
    calls = _calls(calls)
    uncertainty = _work_vector(SOURCE_WORK_PER_UNCERTAINTY)
    kernel = _work_vector(SOURCE_WORK_PER_COMBINATION)
    if set(uncertainty) & set(kernel):
        raise Incomplete("Combination uncertainty/kernel work ownership overlaps.")
    work = {key: calls["uncertainties"] * value for key, value in uncertainty.items()}
    work.update({key: calls["combinations"] * value for key, value in kernel.items()})
    return dict(_work_vector(work))


def full_package_reservation() -> CombinationReservation:
    """Reserve the complete held source suite before any native package invocation."""
    recipes = case_recipes()
    calls = _calls({field: sum(row[field] for row in recipes.values()) for field in FIELDS})
    return {"combination_reserved_calls": calls,
            "combination_reserved_module_work_uppers": source_work_uppers(calls),
            "combination_reservation_scope": "Complete source reservation before effects; no native result or private measurement."}
