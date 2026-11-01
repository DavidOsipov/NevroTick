"""Independent immediate consumer contract for the two admitted actual modules.

These Protocols are maintained separately from the implementation Protocols.
The assignments bind the actual normalized implementation modules, without a
dynamic import, cast, stub or runtime attribute-presence approximation.
"""
from typing import Protocol, assert_never

import core_analysis_cohorts as cohorts
import core_combination_accounting as combination


class CombinationInterface(Protocol):
    def combination_counts(self, output: str, *, require_complete: bool = False,
                           bridge_zero_prefix: combination.CaughtBridgeZeroPrefix | None = None) -> combination.CombinationObservation: ...
    def caught_bridge_zero_prefix(self, output: str) -> combination.CaughtBridgeZeroPrefix | None: ...
    def case_recipes(self) -> dict[str, combination.PublicCalls]: ...
    def producer_calls(self, calls: combination.PublicCalls) -> combination.ProducerCalls: ...
    def source_work_uppers(self, calls: combination.PublicCalls) -> dict[str, int]: ...
    def full_package_reservation(self) -> combination.CombinationReservation: ...


class CohortInterface(Protocol):
    def canonical_sha256(self, value: object) -> str: ...
    def plan_reservation(self, plan: object, package_budget: object, check_budgets: object) -> cohorts.SourceReservation: ...
    def admit_plan(self, plan: object, *, subjects: object, product_inputs: object,
                   tool_inputs: object, recipe_sha256: object, inventories: object,
                   baseline_tests: object, package_budget: object,
                   check_budgets: object, global_limits: object) -> cohorts.PlanAdmission: ...
    def cohort_binding(self, plan: object, cohort_id: str, *, subject: str, output_dir: str) -> cohorts.CohortBinding: ...
    def reconcile_receipts(self, plan: object, receipts: object, *, subjects: object) -> cohorts.ReceiptUnion: ...


class LifecycleInterface(Protocol):
    def lifecycle_counts(self, output: str, *, require_complete: bool = False,
                         bridge_zero_prefix: combination.CaughtBridgeZeroPrefix | None = None) -> combination.LifecycleObservation: ...
    def caught_bridge_zero_prefix(self, output: str) -> combination.CaughtBridgeZeroPrefix | None: ...
    def lifecycle_case_recipes(self) -> dict[str, combination.LifecycleCalls]: ...
    def lifecycle_case_ceilings(self) -> dict[str, combination.LifecycleCalls]: ...
    def lifecycle_producer_calls(self, calls: combination.LifecycleCalls) -> combination.ProducerCalls: ...
    def lifecycle_bridge_producer_reservation(self) -> combination.ProducerCalls: ...
    def lifecycle_guard_fixture_calls(self) -> combination.LifecycleCalls: ...
    def lifecycle_guard_fixture_producer_reservation(self) -> combination.ProducerCalls: ...
    def lifecycle_source_work_uppers(self, calls: combination.LifecycleCalls,
                                   bridge_calls: combination.LifecycleBridgeCalls,
                                   counter_calls: combination.LifecycleCounterCalls) -> dict[str, int]: ...
    def lifecycle_full_package_reservation(self) -> combination.LifecycleReservation: ...


accounting: CombinationInterface = combination
cohort_accounting: CohortInterface = cohorts
lifecycle_accounting: LifecycleInterface = combination


def observed(output: str) -> combination.ObservedAttempts | None:
    """An exhaustive consumer requires a complete record before observed access."""
    record = accounting.combination_counts(output)
    if record["combination_observation_status"] == "complete-source-recipe":
        return combination.complete_attempts(record)
    if record["combination_observation_status"] == "partial-source-recipe":
        return None
    if record["combination_observation_status"] == "missing":
        return None
    assert_never(record)


def reserved() -> combination.ReservedAttempts:
    """Pre-effect reservation carries its own nominal record."""
    return combination.reserved_attempts(accounting.full_package_reservation())


def cohort_reservation(plan: object, budget: object, checks: object) -> cohorts.SourceReservation:
    return cohort_accounting.plan_reservation(plan, budget, checks)


def lifecycle_observed(output: str) -> combination.ObservedLifecycleAttempts | None:
    """Require all lifecycle families before projecting complete observed attempts."""
    record = lifecycle_accounting.lifecycle_counts(output)
    if record["lifecycle_observation_status"] == "complete-source-recipe":
        return combination.complete_lifecycle_attempts(record)
    if record["lifecycle_observation_status"] == "partial-source-recipe":
        return None
    if record["lifecycle_observation_status"] == "missing":
        return None
    assert_never(record)


def lifecycle_reserved() -> combination.ReservedLifecycleAttempts:
    """Keep pre-effect lifecycle reservations distinct from observed attempts."""
    return combination.reserved_lifecycle_attempts(
        lifecycle_accounting.lifecycle_full_package_reservation())


def lifecycle_counter_units(value: combination.ObservedLifecycleAttempts) -> tuple[int, int, int, int, int, int, int, int]:
    """A complete nominal observation exposes every precise private-counter field."""
    calls = value.counter_calls
    return (calls["attempts"], calls["source_constructions"],
            calls["source_predicates"], calls["garde"], calls["fixture_setups"],
            calls["outcome_predicates"], calls["estimate_predicates"], calls["outcome_garde"])


def combination_caught_prefix(output: str) -> combination.CombinationObservation | None:
    """Type the optional context seam; Native Caught authority belongs to Core."""
    context = accounting.caught_bridge_zero_prefix(output)
    if context is None:
        return None
    return accounting.combination_counts(output, bridge_zero_prefix=context)


def lifecycle_caught_prefix(output: str) -> combination.LifecyclePartialObservation | None:
    """A valid structural context never supplies a complete observed attempt unit."""
    context = lifecycle_accounting.caught_bridge_zero_prefix(output)
    if context is None:
        return None
    record = lifecycle_accounting.lifecycle_counts(output, bridge_zero_prefix=context)
    if record["lifecycle_observation_status"] == "partial-source-recipe":
        return record
    if record["lifecycle_observation_status"] == "complete-source-recipe":
        return None
    if record["lifecycle_observation_status"] == "missing":
        return None
    assert_never(record)
