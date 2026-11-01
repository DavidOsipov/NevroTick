"""Pure observations and bounded source work for the fixture combination suite.

No I/O or execution occurs here. Exact public attempt rows, canonical module
work uppers and native assertion outcomes remain separate. The coordinator owns
the existing history/projection/principal producer chain and charges it once
using producer_calls; this helper does not import the verification monolith.
"""
from __future__ import annotations

import hashlib
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


class CaughtBridgeZeroPrefix(NamedTuple):
    """Byte-bound structural evidence; the caller separately validates Native Caught."""

    output_sha256: str


def caught_bridge_zero_prefix(output: str) -> CaughtBridgeZeroPrefix | None:
    """Recognize the bridge's own pre-append failure, never a healthy observation.

    Only the native-Caught receiving path may request this token. It binds the
    complete Test text and exact named bridge failure. Other callers keep the
    strict no-zero default; no count, ceiling or raw byte is changed.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS:
        raise Incomplete("Caught bridge prefix requires bounded Test text.")
    test = "combination::admission::receiving_tests::bridge_receiving_and_mask_boundaries"
    zero = ("CORE_LIFECYCLE_BRIDGE_PROFILE case=bridge_receiving_and_mask_boundaries"
            " appends=0 admissions=0 masks=0 garde=0")
    markers = sum(1 for _match in re.finditer(
        r"(?m)^(?:test " + re.escape(test) + r" \.\.\. )?" + re.escape(zero) + r"\r?$", output))
    header = "---- " + test + " stdout ----"
    headers = sum(1 for _match in re.finditer(r"(?m)^" + re.escape(header) + r"\r?$", output))
    if markers != 1 or headers != 1:
        return None
    failures = sum(1 for _match in re.finditer(
        r"(?m)^" + re.escape(header)
        + r"\r?\nError: Divisor \{ issue: (?:OutOfRange|NonFinite) \}\r?\n\r?\n(?=---- |failures:)",
        output))
    footers = list(re.finditer(
        r"(?m)^failures:\r?\n(?:\r?\n)?((?:    [A-Za-z_][A-Za-z0-9_:]*\r?\n)+)"
        r"\r?\ntest result: FAILED\. ([0-9]{1,3}) passed; ([0-9]{1,3}) failed; 0 ignored;",
        output))
    footer_groups = [match.groups("") for match in footers]
    if (failures != 1 or not footer_groups
            or any(int(failed) != len(names.splitlines()) for names, _passed, failed in footer_groups)
            or sum(name.strip() == test for names, _passed, _failed in footer_groups
                   for name in names.splitlines()) != 1):
        return None
    return CaughtBridgeZeroPrefix(hashlib.sha256(output.encode("utf-8")).hexdigest())


class CombinationAccounting(Protocol):
    """Bind the actual checked implementation; a dynamic cast is insufficient."""

    def combination_counts(self, output: str, *, require_complete: bool = False,
                           bridge_zero_prefix: CaughtBridgeZeroPrefix | None = None) -> CombinationObservation: ...
    def caught_bridge_zero_prefix(self, output: str) -> CaughtBridgeZeroPrefix | None: ...
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


def combination_counts(output: str, *, require_complete: bool = False,
                       bridge_zero_prefix: CaughtBridgeZeroPrefix | None = None) -> CombinationObservation:
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
    if require_complete and bridge_zero_prefix is not None:
        raise Incomplete("Caught bridge context cannot supply complete package observations.")
    recorded = _registered_rows(output, bridge_zero_prefix=bridge_zero_prefix)[MARKER]
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


# These three transports belong to the lifecycle suite, not the kernel's
# independently maintained 22-position transport. No expected numerical value or
# qualification decision is generated from these mechanical field definitions.
LifecycleCounterField: TypeAlias = Literal[
    "histories", "appends", "observations", "contexts", "monotonic", "coordinates",
    "exchanges", "getters", "terms", "projections", "enclosures", "batches", "minima",
    "principal_parameters", "strata", "combination_parameters", "seeds",
    "reachability_updates", "reachability_records", "advances", "uncertainties", "equalities",
]


class LifecycleCalls(TypedDict):
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
    principal_parameters: int
    strata: int
    combination_parameters: int
    seeds: int
    reachability_updates: int
    reachability_records: int
    advances: int
    uncertainties: int
    equalities: int


class LifecycleBridgeCalls(TypedDict):
    """The four actually observed private-fixture call subsets only."""

    appends: int
    admissions: int
    masks: int
    garde: int


class LifecycleCounterCalls(TypedDict):
    """Actual standalone counter seam requests, not their internal validator work."""

    attempts: int
    source_constructions: int
    source_predicates: int
    garde: int
    fixture_setups: int
    outcome_predicates: int
    estimate_predicates: int
    outcome_garde: int


class _LifecycleObservationCommon(TypedDict):
    lifecycle_counter_records: int
    lifecycle_profiles: dict[str, LifecycleCalls]
    lifecycle_bridge_profiles: dict[str, LifecycleBridgeCalls]
    lifecycle_counter_profiles: dict[str, LifecycleCounterCalls]
    lifecycle_missing_cases: list[str]
    lifecycle_partial_cases: list[str]
    lifecycle_observation_scope: str


class LifecycleCompleteObservation(_LifecycleObservationCommon):
    lifecycle_observation_status: Literal["complete-source-recipe"]
    lifecycle_observation_pending: Literal[False]
    lifecycle_counter_observation_complete: Literal[True]
    lifecycle_recorded_call_subtotals: LifecycleCalls
    lifecycle_recorded_bridge_subtotals: LifecycleBridgeCalls
    lifecycle_recorded_counter_subtotals: LifecycleCounterCalls
    lifecycle_observed_calls: LifecycleCalls
    lifecycle_observed_bridge_calls: LifecycleBridgeCalls
    lifecycle_observed_counter_calls: LifecycleCounterCalls


class LifecyclePartialObservation(_LifecycleObservationCommon):
    lifecycle_observation_status: Literal["partial-source-recipe"]
    lifecycle_observation_pending: Literal[True]
    lifecycle_counter_observation_complete: Literal[False]
    lifecycle_recorded_call_subtotals: LifecycleCalls
    lifecycle_recorded_bridge_subtotals: LifecycleBridgeCalls
    lifecycle_recorded_counter_subtotals: LifecycleCounterCalls
    lifecycle_observed_calls: None
    lifecycle_observed_bridge_calls: None
    lifecycle_observed_counter_calls: None


class LifecycleMissingObservation(_LifecycleObservationCommon):
    lifecycle_observation_status: Literal["missing"]
    lifecycle_observation_pending: Literal[True]
    lifecycle_counter_observation_complete: Literal[False]
    lifecycle_recorded_call_subtotals: None
    lifecycle_recorded_bridge_subtotals: None
    lifecycle_recorded_counter_subtotals: None
    lifecycle_observed_calls: None
    lifecycle_observed_bridge_calls: None
    lifecycle_observed_counter_calls: None


LifecycleObservation: TypeAlias = (
    LifecycleCompleteObservation | LifecyclePartialObservation | LifecycleMissingObservation
)


class LifecycleReservation(TypedDict):
    lifecycle_reserved_calls: LifecycleCalls
    lifecycle_reserved_bridge_calls: LifecycleBridgeCalls
    lifecycle_reserved_counter_calls: LifecycleCounterCalls
    lifecycle_reserved_module_work_uppers: dict[str, int]
    lifecycle_reservation_scope: str


class ObservedLifecycleAttempts(NamedTuple):
    calls: LifecycleCalls
    bridge_calls: LifecycleBridgeCalls
    counter_calls: LifecycleCounterCalls


class ReservedLifecycleAttempts(NamedTuple):
    calls: LifecycleCalls
    bridge_calls: LifecycleBridgeCalls
    counter_calls: LifecycleCounterCalls


class LifecycleAccounting(Protocol):
    """Static consumer checks the actual implementation across module loading."""

    def lifecycle_counts(self, output: str, *, require_complete: bool = False,
                         bridge_zero_prefix: CaughtBridgeZeroPrefix | None = None) -> LifecycleObservation: ...
    def caught_bridge_zero_prefix(self, output: str) -> CaughtBridgeZeroPrefix | None: ...
    def lifecycle_case_recipes(self) -> dict[str, LifecycleCalls]: ...
    def lifecycle_case_ceilings(self) -> dict[str, LifecycleCalls]: ...
    def lifecycle_producer_calls(self, calls: LifecycleCalls) -> ProducerCalls: ...
    def lifecycle_bridge_producer_reservation(self) -> ProducerCalls: ...
    def lifecycle_guard_fixture_calls(self) -> LifecycleCalls: ...
    def lifecycle_guard_fixture_producer_reservation(self) -> ProducerCalls: ...
    def lifecycle_source_work_uppers(
        self, calls: LifecycleCalls, bridge_calls: LifecycleBridgeCalls,
        counter_calls: LifecycleCounterCalls,
    ) -> dict[str, int]: ...
    def lifecycle_full_package_reservation(self) -> LifecycleReservation: ...


LIFECYCLE_MARKER = "CORE_COMBINATION_LIFECYCLE_PROFILE"
LIFECYCLE_BRIDGE_MARKER = "CORE_LIFECYCLE_BRIDGE_PROFILE"
LIFECYCLE_COUNTER_MARKER = "CORE_LIFECYCLE_COUNTER_PROFILE"
LIFECYCLE_FIELDS: tuple[LifecycleCounterField, ...] = (
    "histories", "appends", "observations", "contexts", "monotonic", "coordinates",
    "exchanges", "getters", "terms", "projections", "enclosures", "batches", "minima",
    "principal_parameters", "strata", "combination_parameters", "seeds",
    "reachability_updates", "reachability_records", "advances", "uncertainties", "equalities",
)
LIFECYCLE_BRIDGE_FIELDS = ("appends", "admissions", "masks", "garde")
LIFECYCLE_COUNTER_FIELDS = (
    "attempts", "source_constructions", "source_predicates", "garde", "fixture_setups",
    "outcome_predicates", "estimate_predicates", "outcome_garde",
)

# Exact successful source-call recipes, distinct from the pre-effect ceilings
# below. Phases cost two getters/append, Built costs one getter/projection,
# observe_round costs eight getters, source_view five, pending three. The mature
# loop's authored counts and the unchanged eight-case source were independently
# reconciled before adding the three atomic-retry recipes. The new retry helpers
# cost 14 initial +46 correction/replay getters, plus phases/projections.
LIFECYCLE_SOURCE_RECIPES: tuple[tuple[str, tuple[int, ...]], ...] = (
    ("lifecycle_strict_boundaries_and_original_principal", (21,63,63,21,73,126,63,373,21,21,21,10,10,10,21,10,10,0,0,10,21,0)),
    ("lifecycle_startup_replay_and_supporter_only_pauses", (2,9,9,2,14,18,9,129,10,10,10,5,1,1,10,1,1,0,0,7,10,0)),
    ("lifecycle_maturity_countdown_relapse_and_event_order", (2,53,53,2,102,106,53,876,98,98,98,49,3,3,98,3,3,23,22,98,98,63)),
    ("lifecycle_rejection_rolls_back_and_same_event_recovers", (2,7,7,2,11,14,7,57,8,8,8,4,2,2,8,2,2,4,4,8,8,7)),
    ("lifecycle_filtered_one_keeps_multiwitness_arithmetic", (2,6,6,2,9,12,6,58,5,5,5,3,3,3,5,3,3,0,0,6,9,4)),
    ("lifecycle_outside_witness_loss_waiting_and_reacquisition", (3,14,14,3,20,28,14,166,18,18,18,6,1,1,18,1,1,0,0,8,10,0)),
    ("lifecycle_direct_replacement_clears_all_then_rechecks_supporters", (3,11,11,3,14,22,11,125,9,9,9,3,1,1,9,1,1,16,16,3,9,0)),
    ("lifecycle_raw_parameter_and_ordinal_admission_neighbors", (0,0,0,0,0,0,0,24,0,0,0,0,0,0,0,21,0,6,0,0,0,6)),
    ("lifecycle_distance_limit_overflow_rolls_back_and_recovers", (2,7,7,2,10,14,7,80,6,6,6,3,2,2,6,2,2,0,0,7,6,4)),
    ("lifecycle_skew_sum_overflow_rolls_back_and_recovers", (2,7,7,2,9,14,7,78,4,4,4,2,2,2,4,2,2,0,0,7,6,4)),
    ("lifecycle_max_clock_error_sum_overflow_rolls_back_and_recovers", (2,7,7,2,9,14,7,78,4,4,4,2,2,2,4,2,2,0,0,7,6,4)),
    ("lifecycle_capacity_neighbors_preserve_pending_credit", (16,48,48,16,49,96,48,264,16,16,16,1,1,1,16,1,4,0,0,6,16,4)),
)
LIFECYCLE_SOURCE_TESTS = tuple(case for case, _ in LIFECYCLE_SOURCE_RECIPES)

# Limits order: histories,appends,projections,batches,seeds,advances,
# reachability constructors,reachability records,uncertainties,equalities.
# These literals mirror the independently authored bounded fixture contracts,
# not an observed stream. The raw case has separate 21/6 constructor caps.
LIFECYCLE_SOURCE_LIMITS: tuple[tuple[str, tuple[int, ...]], ...] = (
    ("lifecycle_strict_boundaries_and_original_principal", (21,63,21,10,10,10,0,0,21,0)),
    ("lifecycle_startup_replay_and_supporter_only_pauses", (2,9,10,5,1,7,0,0,10,0)),
    ("lifecycle_maturity_countdown_relapse_and_event_order", (2,53,98,49,3,98,23,22,98,64)),
    ("lifecycle_rejection_rolls_back_and_same_event_recovers", (2,7,8,4,2,8,4,4,8,16)),
    ("lifecycle_filtered_one_keeps_multiwitness_arithmetic", (2,6,5,3,3,6,0,0,9,8)),
    ("lifecycle_outside_witness_loss_waiting_and_reacquisition", (3,14,18,6,1,8,0,0,10,0)),
    ("lifecycle_direct_replacement_clears_all_then_rechecks_supporters", (3,11,9,3,1,3,16,16,9,0)),
    ("lifecycle_raw_parameter_and_ordinal_admission_neighbors", (0,0,0,0,0,0,6,0,0,6)),
    ("lifecycle_distance_limit_overflow_rolls_back_and_recovers", (2,7,6,3,2,7,0,0,6,4)),
    ("lifecycle_skew_sum_overflow_rolls_back_and_recovers", (2,7,4,2,2,7,0,0,6,4)),
    ("lifecycle_max_clock_error_sum_overflow_rolls_back_and_recovers", (2,7,4,2,2,7,0,0,6,4)),
    ("lifecycle_capacity_neighbors_preserve_pending_credit", (16,48,16,1,4,6,0,0,16,4)),
)
LIFECYCLE_BRIDGE_SOURCE_RECIPES = (("bridge_receiving_and_mask_boundaries", (6,5,5,4)),)
LIFECYCLE_BRIDGE_SOURCE_TESTS = (
    "combination::admission::receiving_tests::bridge_receiving_and_mask_boundaries",
)
LIFECYCLE_COUNTER_SOURCE_RECIPES = (("lifecycle_counter_success_and_exhaustion", (4,24,24,24,1,34,18,34)),)
LIFECYCLE_COUNTER_SOURCE_TESTS = (
    "combination::lifecycle::counter_tests::lifecycle_counter_success_and_exhaustion",
)


def _named_recipes(
    rows: tuple[tuple[str, tuple[int, ...]], ...], tests: tuple[str, ...], width: int,
) -> dict[str, tuple[int, ...]]:
    """Admit closed, nonempty bounded metadata before receiving any native row."""
    if (type(rows) is not tuple or not 0 < len(rows) <= 32
            or type(tests) is not tuple or len(tests) != len(rows)
            or any(type(name) is not str or not 0 < len(name) <= 256 for name in tests)
            or len(set(tests)) != len(tests)):
        raise Incomplete("Lifecycle source/test recipes are missing or malformed.")
    result: dict[str, tuple[int, ...]] = {}
    for row in rows:
        if type(row) is not tuple or len(row) != 2:
            raise Incomplete("Lifecycle recipe is not a case/vector pair.")
        case, values = row
        if (type(case) is not str or re.fullmatch(r"[a-z_]{1,96}", case) is None
                or case in result or type(values) is not tuple or len(values) != width
                or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)
                or not any(values)):
            raise Incomplete("Lifecycle recipe has unknown, duplicate or unbounded fields.")
        result[case] = values
    return result


def _lifecycle_vector(values: tuple[int, ...]) -> LifecycleCalls:
    """Give every admitted position one precise field, with no dynamic-key cast."""
    if (type(values) is not tuple or len(values) != 22
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)):
        raise Incomplete("Lifecycle calls require exactly twenty-two u64 counts.")
    return {"histories": values[0], "appends": values[1], "observations": values[2],
            "contexts": values[3], "monotonic": values[4], "coordinates": values[5],
            "exchanges": values[6], "getters": values[7], "terms": values[8],
            "projections": values[9], "enclosures": values[10], "batches": values[11],
            "minima": values[12], "principal_parameters": values[13], "strata": values[14],
            "combination_parameters": values[15], "seeds": values[16],
            "reachability_updates": values[17], "reachability_records": values[18],
            "advances": values[19], "uncertainties": values[20], "equalities": values[21]}


def _bridge_vector(values: tuple[int, ...]) -> LifecycleBridgeCalls:
    if (type(values) is not tuple or len(values) != 4
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)):
        raise Incomplete("Private bridge calls require four u64 counts.")
    return {"appends": values[0], "admissions": values[1], "masks": values[2], "garde": values[3]}


def _counter_vector(values: tuple[int, ...]) -> LifecycleCounterCalls:
    if (type(values) is not tuple or len(values) != 8
            or any(type(value) is not int or not 0 <= value <= U64_MAX for value in values)):
        raise Incomplete("Private counter calls require exactly eight u64 counts.")
    return {"attempts": values[0], "source_constructions": values[1],
            "source_predicates": values[2], "garde": values[3],
            "fixture_setups": values[4], "outcome_predicates": values[5],
            "estimate_predicates": values[6], "outcome_garde": values[7]}


def lifecycle_case_recipes() -> dict[str, LifecycleCalls]:
    """Exact successful source rows establish transport completion only."""
    return {case: _lifecycle_vector(values) for case, values in _named_recipes(
        LIFECYCLE_SOURCE_RECIPES, LIFECYCLE_SOURCE_TESTS, 22).items()}


def lifecycle_case_ceilings() -> dict[str, LifecycleCalls]:
    """Reserve literal Limits, independently of successful or failed observation."""
    recipes = lifecycle_case_recipes()
    limits = _named_recipes(LIFECYCLE_SOURCE_LIMITS, LIFECYCLE_SOURCE_TESTS, 10)
    if set(limits) != set(recipes):
        raise Incomplete("Lifecycle limits and exact case inventory disagree.")
    result: dict[str, LifecycleCalls] = {}
    for case, values in limits.items():
        h, a, p, b, s, v, updates, records, uncertainty, equality = values
        getters = 64 * (a + p + v + records + s + 1)
        parameters = 21 if case == "lifecycle_raw_parameter_and_ordinal_admission_neighbors" else s
        row = _lifecycle_vector((h,a,a,h,a+b,2*a,a,getters,p,p,p,b,s,s,p,parameters,
                                 s,updates,records,v,uncertainty,equality))
        if any(recipes[case][field] > row[field] for field in LIFECYCLE_FIELDS):
            raise Incomplete("A complete lifecycle recipe exceeds its prior source ceiling.")
        result[case] = row
    return result


class _ProfileFamily(NamedTuple):
    marker: str
    fields: tuple[str, ...]
    recipes: dict[str, tuple[int, ...]]
    ceilings: dict[str, tuple[int, ...]]
    tests: tuple[str, ...]


def _profile_families() -> tuple[_ProfileFamily, ...]:
    """One rule owner dispatches the closed kernel/lifecycle/private namespace."""
    kernel = _recipes()
    lifecycle = _named_recipes(LIFECYCLE_SOURCE_RECIPES, LIFECYCLE_SOURCE_TESTS, 22)
    ceilings = {case: tuple(row[field] for field in LIFECYCLE_FIELDS)
                for case, row in lifecycle_case_ceilings().items()}
    bridge = _named_recipes(LIFECYCLE_BRIDGE_SOURCE_RECIPES, LIFECYCLE_BRIDGE_SOURCE_TESTS, 4)
    counter = _named_recipes(LIFECYCLE_COUNTER_SOURCE_RECIPES, LIFECYCLE_COUNTER_SOURCE_TESTS, 8)
    return (
        _ProfileFamily(MARKER, FIELDS, kernel, kernel, SOURCE_TESTS),
        _ProfileFamily(LIFECYCLE_MARKER, LIFECYCLE_FIELDS, lifecycle, ceilings, LIFECYCLE_SOURCE_TESTS),
        _ProfileFamily(LIFECYCLE_BRIDGE_MARKER, LIFECYCLE_BRIDGE_FIELDS, bridge, bridge, LIFECYCLE_BRIDGE_SOURCE_TESTS),
        _ProfileFamily(LIFECYCLE_COUNTER_MARKER, LIFECYCLE_COUNTER_FIELDS, counter, counter, LIFECYCLE_COUNTER_SOURCE_TESTS),
    )


def _registered_rows(output: str, *, bridge_zero_prefix: CaughtBridgeZeroPrefix | None = None,
                     ) -> dict[str, dict[str, tuple[int, ...]]]:
    """Validate every registered row once, including families another caller ignores.

    Unknown or malformed family rows cannot disappear during dispatch. Other
    unrelated core telemetry remains outside this helper's namespace. Each
    retained count is observed transport, never a source reservation or success.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS:
        raise Incomplete("Profile dispatch requires bounded text.")
    if bridge_zero_prefix is not None and (
            type(bridge_zero_prefix) is not CaughtBridgeZeroPrefix
            or caught_bridge_zero_prefix(output) != bridge_zero_prefix):
        raise Incomplete("Caught bridge prefix context is stale or lacks the exact native failure.")
    families = _profile_families()
    patterns: dict[str, re.Pattern[str]] = {}
    for family in families:
        progress = r"(?:test (?:" + "|".join(re.escape(name) for name in family.tests) + r") \.\.\. )?"
        patterns[family.marker] = re.compile(progress + re.escape(family.marker) + r" case=([a-z_]+)"
                                          + "".join(" " + field + "=" + _DECIMAL for field in family.fields))
    result: dict[str, dict[str, tuple[int, ...]]] = {family.marker: {} for family in families}
    lines = output.split("\n")
    for line_index, raw in enumerate(lines):
        if "CORE_COMBINATION_" not in raw and "CORE_LIFECYCLE_" not in raw:
            continue
        if line_index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Profile marker lacks a terminated bounded native line.")
        line = raw[:-1] if raw.endswith("\r") else raw
        found = False
        for family in families:
            match = patterns[family.marker].fullmatch(line)
            if match is None:
                continue
            captures = match.groups("")
            case = captures[0]
            values = tuple(int(value) for value in captures[1:])
            if (case not in family.recipes or case in result[family.marker]
                    or any(value > U64_MAX for value in values)
                    or any(value > maximum for value, maximum in zip(
                        values, family.ceilings.get(case, ()), strict=True))
                    or (not any(values) and not (
                        bridge_zero_prefix is not None
                        and family.marker == LIFECYCLE_BRIDGE_MARKER
                        and case == "bridge_receiving_and_mask_boundaries"
                        and values == (0, 0, 0, 0)))):
                raise Incomplete("Profile case is unknown, duplicated or exceeds its source ceiling.")
            expected_test = dict(zip(family.recipes, family.tests, strict=True))[case]
            if line.startswith("test ") and not line.startswith("test " + expected_test + " ... "):
                raise Incomplete("Profile case and native test identity disagree.")
            result[family.marker][case] = values
            found = True
            break
        if not found:
            raise Incomplete("Unknown or malformed kernel/lifecycle family marker.")
    return result


def _sum_vectors(rows: dict[str, tuple[int, ...]], width: int) -> tuple[int, ...]:
    """Sum observed or reserved vectors without silently crossing u64 range."""
    values = tuple(sum(row[index] for row in rows.values()) for index in range(width))
    if any(value > U64_MAX for value in values):
        raise Incomplete("Profile subtotal exceeds u64 arithmetic.")
    return values


def lifecycle_counts(output: str, *, require_complete: bool = False,
                     bridge_zero_prefix: CaughtBridgeZeroPrefix | None = None) -> LifecycleObservation:
    """Keep twelve exact trajectory rows and both private subsets distinct.

    Completion means every authored transport recipe was reported. It says
    nothing about assertion outcomes. A prefix within a larger authored ceiling
    remains partial, including a count greater than the healthy exact recipe.
    """
    if type(require_complete) is not bool:
        raise Incomplete("Lifecycle completion mode must be Boolean.")
    if require_complete and bridge_zero_prefix is not None:
        raise Incomplete("Caught bridge context cannot supply complete lifecycle observations.")
    rows = _registered_rows(output, bridge_zero_prefix=bridge_zero_prefix)
    selected = tuple(family for family in _profile_families() if family.marker != MARKER)
    missing = [case for family in selected for case in family.recipes if case not in rows[family.marker]]
    partial = [case for family in selected for case, values in rows[family.marker].items()
               if values != family.recipes[case]]
    records = sum(len(rows[family.marker]) for family in selected)
    complete = not missing and not partial
    if require_complete and not complete:
        raise Incomplete("Complete lifecycle profiles are missing or partial.")
    common: _LifecycleObservationCommon = {
        "lifecycle_counter_records": records,
        "lifecycle_profiles": {case: _lifecycle_vector(values)
                               for case, values in rows[LIFECYCLE_MARKER].items()},
        "lifecycle_bridge_profiles": {case: _bridge_vector(values)
                                      for case, values in rows[LIFECYCLE_BRIDGE_MARKER].items()},
        "lifecycle_counter_profiles": {case: _counter_vector(values)
                                       for case, values in rows[LIFECYCLE_COUNTER_MARKER].items()},
        "lifecycle_missing_cases": missing, "lifecycle_partial_cases": partial,
        "lifecycle_observation_scope": "Reported attempts only; completeness proves no assertion outcome or reserved internal work.",
    }
    if records == 0:
        return LifecycleMissingObservation(**common, lifecycle_observation_status="missing",
            lifecycle_observation_pending=True, lifecycle_counter_observation_complete=False,
            lifecycle_recorded_call_subtotals=None, lifecycle_recorded_bridge_subtotals=None,
            lifecycle_recorded_counter_subtotals=None, lifecycle_observed_calls=None,
            lifecycle_observed_bridge_calls=None, lifecycle_observed_counter_calls=None)
    calls = _lifecycle_vector(_sum_vectors(rows[LIFECYCLE_MARKER], 22))
    bridge = _bridge_vector(_sum_vectors(rows[LIFECYCLE_BRIDGE_MARKER], 4))
    counter = _counter_vector(_sum_vectors(rows[LIFECYCLE_COUNTER_MARKER], 8))
    if complete:
        return LifecycleCompleteObservation(**common, lifecycle_observation_status="complete-source-recipe",
            lifecycle_observation_pending=False, lifecycle_counter_observation_complete=True,
            lifecycle_recorded_call_subtotals=calls, lifecycle_recorded_bridge_subtotals=bridge,
            lifecycle_recorded_counter_subtotals=counter, lifecycle_observed_calls=calls.copy(),
            lifecycle_observed_bridge_calls=bridge.copy(), lifecycle_observed_counter_calls=counter.copy())
    return LifecyclePartialObservation(**common, lifecycle_observation_status="partial-source-recipe",
        lifecycle_observation_pending=True, lifecycle_counter_observation_complete=False,
        lifecycle_recorded_call_subtotals=calls, lifecycle_recorded_bridge_subtotals=bridge,
        lifecycle_recorded_counter_subtotals=counter, lifecycle_observed_calls=None,
        lifecycle_observed_bridge_calls=None, lifecycle_observed_counter_calls=None)


def _exact_vector(calls: object, fields: tuple[str, ...]) -> tuple[int, ...]:
    """Runtime JSON-like values still need exact keys, integers and ranges."""
    if not is_exact_dict(calls) or set(calls) != set(fields):
        raise Incomplete("Lifecycle calls require the exact closed field set.")
    result: list[int] = []
    for field in fields:
        value = calls[field]
        if type(value) is not int or not 0 <= value <= U64_MAX:
            raise Incomplete("Lifecycle calls require exact u64 integers, never Boolean counts.")
        result.append(value)
    return tuple(result)


def complete_lifecycle_attempts(observation: LifecycleCompleteObservation) -> ObservedLifecycleAttempts:
    """Expose complete counts in a nominal observed unit, with runtime guards."""
    if (observation.get("lifecycle_observation_status") != "complete-source-recipe"
            or observation.get("lifecycle_observation_pending") is not False
            or observation.get("lifecycle_counter_observation_complete") is not True):
        raise Incomplete("Complete lifecycle attempts require complete transport.")
    return ObservedLifecycleAttempts(
        _lifecycle_vector(_exact_vector(observation["lifecycle_observed_calls"], LIFECYCLE_FIELDS)),
        _bridge_vector(_exact_vector(observation["lifecycle_observed_bridge_calls"], LIFECYCLE_BRIDGE_FIELDS)),
        _counter_vector(_exact_vector(observation["lifecycle_observed_counter_calls"], LIFECYCLE_COUNTER_FIELDS)))


def lifecycle_observation_attempts(observation: LifecycleObservation) -> ObservedLifecycleAttempts | None:
    if observation["lifecycle_observation_status"] == "complete-source-recipe":
        return complete_lifecycle_attempts(observation)
    if observation["lifecycle_observation_status"] == "partial-source-recipe":
        return None
    if observation["lifecycle_observation_status"] == "missing":
        return None
    assert_never(observation)


def reserved_lifecycle_attempts(reservation: LifecycleReservation) -> ReservedLifecycleAttempts:
    """Never let a reservation masquerade as actually reported attempts."""
    return ReservedLifecycleAttempts(
        _lifecycle_vector(_exact_vector(reservation["lifecycle_reserved_calls"], LIFECYCLE_FIELDS)),
        _bridge_vector(_exact_vector(reservation["lifecycle_reserved_bridge_calls"], LIFECYCLE_BRIDGE_FIELDS)),
        _counter_vector(_exact_vector(reservation["lifecycle_reserved_counter_calls"], LIFECYCLE_COUNTER_FIELDS)))


def lifecycle_producer_calls(calls: LifecycleCalls) -> ProducerCalls:
    """Charge the real history/projection/principal chain exactly once.

    Each lifecycle advance clones its complete inner principal owner once before
    calling the real principal advance. The latter's internal staging belongs to
    that existing principal work owner. This additional outer clone is explicitly
    projected into state_clones; no implicit fit is introduced by a projection.
    """
    calls = _lifecycle_vector(_exact_vector(calls, LIFECYCLE_FIELDS))
    return {"histories": calls["histories"], "appends": calls["appends"],
            "observations": calls["observations"], "contexts": calls["contexts"],
            "monotonic": calls["monotonic"], "coordinates": calls["coordinates"],
            "exchanges": calls["exchanges"], "getters": calls["getters"], "terms": calls["terms"],
            "projections": calls["projections"], "enclosures": calls["enclosures"],
            "batches": calls["batches"], "minima": calls["minima"],
            "parameters": calls["principal_parameters"], "strata": calls["strata"],
            "seeds": calls["seeds"], "advances": calls["advances"],
            "state_clones": calls["advances"], "value_equalities": calls["equalities"],
            "state_formats": 0}


def lifecycle_bridge_producer_reservation() -> ProducerCalls:
    """Source-only private-fixture setup, never inferred from its four counters.

    Two three-row histories: one UTC zero/exchange per history, three row clocks,
    two projection clocks, then one batch clock. Two projection/stratum pairs,
    one real principal seed/advance. Sixty-four getter requests conservatively
    cover the fixed source's direct accessors; there are no state clones/formats.
    The separate six observed append attempts must not charge a second fit chain.
    """
    return {"histories": 2, "appends": 6, "observations": 6, "contexts": 2,
            "monotonic": 11, "coordinates": 2, "exchanges": 2, "getters": 64,
            "terms": 2, "projections": 2, "enclosures": 2, "batches": 1,
            "minima": 1, "parameters": 1, "strata": 2, "seeds": 1, "advances": 1,
            "state_clones": 0, "value_equalities": 0, "state_formats": 0}


def lifecycle_guard_fixture_calls() -> LifecycleCalls:
    """Return a fixed source reservation, never an observed lifecycle call row.

    One private raw-output setup enters four genuine three-row histories, six
    projections/batches and eleven public lifecycle owners, with twelve advances.
    Its independent 4,096 getter ceiling covers the fixture's direct accessor
    work. Direct raw predicates and three additional Inner clones are separate.
    Twelve fits/768 points belong to the delegated producer, not this module.
    """
    return _lifecycle_vector((4,12,12,4,24,4,4,4096,6,6,6,6,11,11,16,11,
                              11,0,0,12,16,0))


def lifecycle_guard_fixture_producer_reservation() -> ProducerCalls:
    """Project one genuine setup once, retaining all fifteen principal clones.

    Twelve outer lifecycle advances each clone the principal. Three explicitly
    uncommitted raw Inner candidates add three more copies of that same owner.
    The corresponding fixed source-array copies belong to the module-work bound.
    No raw probe count is interpreted as an additional numerical producer call.
    """
    calls = lifecycle_producer_calls(lifecycle_guard_fixture_calls())
    calls["state_clones"] += 3
    return calls



def lifecycle_source_work_uppers(
    calls: LifecycleCalls, bridge_calls: LifecycleBridgeCalls, counter_calls: LifecycleCounterCalls,
) -> dict[str, int]:
    """Bound disjoint lifecycle/bridge units, without claiming internal observation.

    At most16 sources: seed constructs16 counters; an advance has at most three
    source-construction passes (48), two output-predicate passes and62 checked
    compatibility binaries. Event admission runs twice and constructs one source.
    Full receiving admission runs twice per bridge entry; mask admission scans16
    positions twice. Direct raw Garde controls are reserved separately and may
    enter either receiving or mask predicates, so each unit gets its own upper.
    Parent reduction uses18*n+5<=293 binaries, three roots and48 denominator guards.
    Source getter bounds cover the nested16-by16 scans and fixed provenance chains.
    The private raw-counter matrix separately requests constructors, predicates
    and Garde validation. Each constructor performs one validation/callback and
    two source predicates; direct Garde performs one callback/predicate. Direct
    predicates add one only. Thus 24/24/24 requests cost at most 48 validations,
    48 source callbacks and 96 predicates; callbacks overlap these same owners.
    One precharged raw-output fixture contributes its fixed lifecycle setup once.
    Its direct outcome/Garde calls contribute two separate predicate requests;
    direct estimate calls retain their own counter unit and getter-work bound.
    Three raw Inner clones add 48 source cells per fixture. Standalone source
    constructors retain an eight-getter upper even before fixture setup starts.
    Callback/predicate/validation counts describe overlapping subsets, not CPU work.
    """
    calls = _lifecycle_vector(_exact_vector(calls, LIFECYCLE_FIELDS))
    bridge_calls = _bridge_vector(_exact_vector(bridge_calls, LIFECYCLE_BRIDGE_FIELDS))
    counter_calls = _counter_vector(_exact_vector(counter_calls, LIFECYCLE_COUNTER_FIELDS))
    fixtures = counter_calls["fixture_setups"]
    fixture_calls = lifecycle_guard_fixture_calls()
    calls = _lifecycle_vector(tuple(calls[field] + fixtures * fixture_calls[field]
                                    for field in LIFECYCLE_FIELDS))
    advances, seeds, records = calls["advances"], calls["seeds"], calls["reachability_records"]
    parameters, updates = calls["combination_parameters"], calls["reachability_updates"]
    admissions = advances + bridge_calls["admissions"]
    reductions = advances + bridge_calls["masks"]
    direct_garde = bridge_calls["garde"]
    source_constructions = counter_calls["source_constructions"]
    source_predicates = counter_calls["source_predicates"]
    source_garde = counter_calls["garde"]
    outcome_garde = counter_calls["outcome_garde"]
    raw_outcomes = counter_calls["outcome_predicates"] + outcome_garde
    raw_estimates = counter_calls["estimate_predicates"]
    # The fixed private receiving fixture constructs exactly three uncertainties.
    # This is a source upper on entry, not an observed missing counter field.
    private_uncertainty = 3 if any(bridge_calls.values()) else 0
    uncertainty = calls["uncertainties"] + private_uncertainty
    work = {
        "lifecycle_parameter_constructor_attempts_upper": parameters,
        "lifecycle_parameter_garde_callbacks_upper": 2 * parameters,
        "lifecycle_parameter_predicate_calls_upper": 4 * parameters,
        "lifecycle_raw_ordinal_constructor_attempts_upper": updates,
        "lifecycle_garde_validate_calls_upper": (parameters + updates + 16*seeds
                                                + 49*advances + 2*records
                                                + source_constructions + source_garde + outcome_garde),
        "lifecycle_source_counter_constructions_upper": 16*seeds + 48*advances + records + source_constructions,
        "lifecycle_source_counter_predicate_calls_upper": (32*seeds + 96*advances + 2*records
                                                         + 2*source_constructions
                                                         + source_predicates + source_garde),
        "lifecycle_event_admission_predicate_calls_upper": 2*records,
        "lifecycle_checked_next_calls_upper": advances + 2*records + counter_calls["attempts"],
        "lifecycle_compatibility_checked_binary_calls_upper": 62*advances,
        "lifecycle_compatibility_source_visits_upper": 16*advances,
        "lifecycle_source_index_label_probes_upper": 256*advances + 32*records,
        "lifecycle_witness_membership_label_probes_upper": 768*advances + 256*raw_outcomes,
        "lifecycle_output_contributor_label_probes_upper": 512*advances + 256*raw_outcomes,
        "lifecycle_output_predicate_calls_upper": 2*advances + raw_outcomes,
        "lifecycle_source_slot_writes_upper": 16*seeds + 48*advances + records,
        "lifecycle_staged_source_copy_cells_upper": 16*advances + 48*fixtures,
        "lifecycle_state_equality_source_cells_upper": 16*calls["equalities"],
        "lifecycle_state_equality_fixed_fields_upper": 4*calls["equalities"],
        "lifecycle_internal_product_getter_requests_upper": (8192*advances + 64*records + 16*seeds
                                                             + 8*source_constructions
                                                             + 8192*(raw_outcomes + raw_estimates)),
        "bridge_admission_entry_attempts_upper": admissions,
        "bridge_complete_receiving_passes_upper": 2*admissions + direct_garde,
        "bridge_receiving_pointer_comparisons_upper": 256*(2*admissions + direct_garde),
        "bridge_receiving_label_comparisons_upper": 393*(2*admissions + direct_garde),
        "bridge_mask_admission_slot_visits_upper": 16*(2*reductions + direct_garde),
        "bridge_direct_garde_validate_calls_upper": direct_garde,
        "bridge_reduction_entry_attempts_upper": reductions,
        "bridge_reduction_checked_binary_calls_upper": 293*reductions,
        "bridge_reduction_checked_root_calls_upper": 3*reductions,
        "bridge_reduction_positive_denominator_guards_upper": 48*reductions,
        "bridge_reduction_supporter_visits_upper": 16*reductions,
        "bridge_record_lookup_visits_upper": 768*advances + 256*bridge_calls["masks"] + (16 if private_uncertainty else 0),
        "bridge_internal_product_getter_requests_upper": 8192*admissions + 4096*reductions + 4096*direct_garde,
        "uncertainty_constructor_attempts_upper": uncertainty,
        "uncertainty_garde_validation_calls_upper": uncertainty,
        "uncertainty_garde_rule_callbacks_upper": 3*uncertainty,
        "uncertainty_predicate_calls_upper": 6*uncertainty,
        "kernel_final_uncertainty_predicate_calls_upper": 3*reductions,
    }
    return _work_vector(work)


def lifecycle_full_package_reservation() -> LifecycleReservation:
    """Prepay all declared ceilings; no observed prefix can lower this reservation."""
    ceilings = lifecycle_case_ceilings()
    calls = _lifecycle_vector(tuple(sum(row[field] for row in ceilings.values())
                                   for field in LIFECYCLE_FIELDS))
    bridge = _bridge_vector(_sum_vectors(_named_recipes(
        LIFECYCLE_BRIDGE_SOURCE_RECIPES, LIFECYCLE_BRIDGE_SOURCE_TESTS, 4), 4))
    counter = _counter_vector(_sum_vectors(_named_recipes(
        LIFECYCLE_COUNTER_SOURCE_RECIPES, LIFECYCLE_COUNTER_SOURCE_TESTS, 8), 8))
    return {"lifecycle_reserved_calls": calls, "lifecycle_reserved_bridge_calls": bridge,
            "lifecycle_reserved_counter_calls": counter,
            "lifecycle_reserved_module_work_uppers": lifecycle_source_work_uppers(calls, bridge, counter),
            "lifecycle_reservation_scope": "Complete source ceilings before effects, including unobserved private fixture work; no execution or assertion claim."}
