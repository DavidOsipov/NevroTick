"""Independent immediate consumer contract for the two admitted actual modules.

These Protocols are maintained separately from the implementation Protocols.
The assignments bind the actual normalized implementation modules, without a
dynamic import, cast, stub or runtime attribute-presence approximation.
"""
from typing import Protocol, assert_never

import core_analysis_cohorts as cohorts
import core_combination_accounting as combination


class CombinationInterface(Protocol):
    def combination_counts(self, output: str, *, require_complete: bool = False) -> combination.CombinationObservation: ...
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


accounting: CombinationInterface = combination
cohort_accounting: CohortInterface = cohorts


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
