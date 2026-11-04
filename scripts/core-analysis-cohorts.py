"""Pure admission and reconciliation for an exact fixture-core cohort plan.

No I/O or execution is performed here. The wrapper owns current file custody,
new output directories, native outcomes and the actual per-file 90/95 checks.
An admitted reservation is neither consumed work nor a passing receipt.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import PurePosixPath
import re
from typing import Literal, Protocol, TypedDict, TypeGuard


class Inventory(TypedDict):
    count: int
    sha256: str


class RemainingChecks(TypedDict):
    coverage: int
    contracts: int


class CheckBudgets(TypedDict):
    coverage: dict[str, int]
    contracts: dict[str, int]


class ReservationInputs(TypedDict):
    inventories: dict[str, Inventory]
    remaining_checks: RemainingChecks


class SourceReservation(TypedDict):
    mutation_variants: int
    mutation_baselines: int
    mutation_package_invocations: int
    source_work: dict[str, int]


class Cohort(TypedDict):
    id: str
    subject: str
    output_dir: str


class CohortPlan(ReservationInputs):
    schema_version: Literal[11]
    status: Literal["admitted-exact-source-and-cohorts"]
    candidate_sha256: str
    product_inputs_sha256: dict[str, str]
    tool_inputs_sha256: dict[str, str]
    tool_payloads_sha256: dict[str, str]
    recipe_sha256: str
    cohorts: list[Cohort]
    baseline_tests: int
    cumulative_reservation: SourceReservation


class PlanAdmission(TypedDict):
    plan_sha256: str
    reservation: SourceReservation
    qualification: str


class CohortBinding(TypedDict):
    plan_sha256: str
    cohort_id: str
    candidate_sha256: str
    recipe_sha256: str


class ReceiptUnion(TypedDict):
    status: Literal["passed-exact-mutation-union"]
    plan_sha256: str
    critical_owners: list[str]
    focused_receipts: int
    generated_variants: int
    baselines: int
    reservation: SourceReservation
    critical_manifest_complete_selection: Literal[False]
    scope: str


def is_exact_dict(value: object) -> TypeGuard[dict[object, object]]:
    """Narrow unknown dictionary entries to object, never dynamic Any."""
    return type(value) is dict


def is_exact_list(value: object) -> TypeGuard[list[object]]:
    """Preserve the exact built-in list admission boundary."""
    return type(value) is list


class CohortAccounting(Protocol):
    """JSON admission accepts unknown objects and returns precise checked records."""

    def canonical_sha256(self, value: object) -> str: ...
    def plan_reservation(self, plan: object, package_budget: object, check_budgets: object) -> SourceReservation: ...
    def admit_plan(self, plan: object, *, subjects: object, product_inputs: object,
                   tool_inputs: object, recipe_sha256: object, inventories: object,
                   baseline_tests: object, package_budget: object,
                   check_budgets: object, global_limits: object) -> PlanAdmission: ...
    def cohort_binding(self, plan: object, cohort_id: str, *, subject: str,
                       output_dir: str) -> CohortBinding: ...
    def reconcile_receipts(self, plan: object, receipts: object, *, subjects: object) -> ReceiptUnion: ...


class Incomplete(ValueError):
    """A closed coordinator plan or its receipt union is incomplete."""


MAX_COHORT_VARIANTS = 512
MAX_LISTING_VARIANTS = 512
MAX_S158_LISTING_VARIANTS = 512 * 9
MAX_OWNERS = 9
MAX_INPUTS = 256
# Preserve all 1508 earlier fields and add 315 disjoint S155 chart-batch fields.
# This reviewed width binds the batch recipe and separate legal-consumer delta;
# reservations are not observed operation counters or arbitrary-mutant quotas.
# S157 preserves the prior1823 fields and adds391 reviewed source-work fields.
# Per-plan exact namespaces remain independently checked against the bound recipe.
MAX_BUDGET_FIELDS = 2214
# S158 preserves2214 fields and adds468 disjoint reviewed chart_combination fields.
# Each profile still binds its exact supplied package/check/reservation namespace.
MAX_S158_BUDGET_FIELDS = 2682
# S1.4a's2700 fields remain valid for this same registered owner. S1.4c
# retains S1.4b's2740 and adds25 receiver fields. This is only the current
# resource ceiling: admission still binds every supplied recipe key and value.
# Other registered owner profiles keep their independent historical ceilings.
MAX_REFERENCE_INPUT_BUDGET_FIELDS = 2765
# S1.4b keeps2700 earlier keys and appends40 disjoint composition work fields.
MAX_CHART_COMPOSITION_BUDGET_FIELDS = 2740
MAX_CHART_COMPOSITION_LISTING_VARIANTS = 512 * 3
# Four changed settled-rate owners retain the predecessor widths and add55
# rate-delivery fields. Discovery allows512 per file; campaigns stay focused.
MAX_SETTLED_RATE_BUDGET_FIELDS = 2820
MAX_SETTLED_RATE_LISTING_VARIANTS = 512 * 4
S157_OWNERS = {
    "principal": "crates/time-core/src/principal.rs",
    "chart_principal": "crates/time-core/src/principal/chart.rs",
    "chart_batch": "crates/time-core/src/chart_batch.rs",
}
S158_OWNERS = {
    "combination": "crates/time-core/src/combination.rs",
    "combination_admission": "crates/time-core/src/combination/admission.rs",
    "combination_lifecycle": "crates/time-core/src/combination/lifecycle.rs",
    "chart": "crates/time-core/src/regression/chart.rs",
    "chart_roundoff": "crates/time-core/src/regression/chart/roundoff.rs",
    "chart_combination_roundoff": "crates/time-core/src/combination/chart/roundoff.rs",
    "chart_combination_receiving": "crates/time-core/src/combination/chart/receiving.rs",
    "chart_combination_reduction": "crates/time-core/src/combination/chart/reduction.rs",
    "chart_combination_lifecycle": "crates/time-core/src/combination/lifecycle/chart.rs",
}
REFERENCE_INPUT_OWNERS = {
    "reference_input": "crates/time-core/src/reference_input.rs",
}
CHART_COMPOSITION_OWNERS = {
    "chart": "crates/time-core/src/regression/chart.rs",
    "chart_roundoff": "crates/time-core/src/regression/chart/roundoff.rs",
    "chart_batch": "crates/time-core/src/chart_batch.rs",
}
SETTLED_RATE_OWNERS = {
    "reference_input": "crates/time-core/src/reference_input.rs",
    "rate_delivery": "crates/time-core/src/reference_input/rate_delivery.rs",
    "chart": "crates/time-core/src/regression/chart.rs",
    "chart_roundoff": "crates/time-core/src/regression/chart/roundoff.rs",
}

PLAN_KEYS = frozenset({
    "schema_version", "status", "candidate_sha256", "product_inputs_sha256",
    "tool_inputs_sha256", "tool_payloads_sha256", "recipe_sha256", "inventories",
    "cohorts", "remaining_checks", "baseline_tests", "cumulative_reservation",
})
COHORT_KEYS = frozenset({"id", "subject", "output_dir"})
CHECKS: tuple[Literal["coverage", "contracts"], ...] = ("coverage", "contracts")
_SHA256 = re.compile(r"[0-9a-f]{64}")
_NAME = re.compile(r"[a-z][a-z0-9_]{0,63}")


def canonical_sha256(value: object) -> str:
    """Hash an already validated JSON value using the shared canonical form."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def _digest(value: object) -> str:
    """Require an actual SHA-256; pending labels never admit execution."""
    if type(value) is not str or _SHA256.fullmatch(value) is None:
        raise Incomplete("Cohort binding is pending or is not an exact SHA-256.")
    return value


def _object(value: object) -> dict[str, object]:
    """Project an exact object without trusting unchecked entry types."""
    if not is_exact_dict(value):
        raise Incomplete("Cohort record must be an exact object.")
    result: dict[str, object] = {}
    for key, item in value.items():
        if type(key) is not str:
            raise Incomplete("Cohort record keys must be strings.")
        result[key] = item
    return result


def _identity(value: object) -> dict[str, str]:
    """Validate bounded nonempty path/name bindings without coercion."""
    record = _object(value)
    if not 0 < len(record) <= MAX_INPUTS:
        raise Incomplete("Cohort identity inventory has an invalid size or shape.")
    result: dict[str, str] = {}
    for name, digest in record.items():
        if (not 0 < len(name) <= 256 or PurePosixPath(name).is_absolute()
                or ".." in PurePosixPath(name).parts):
            raise Incomplete("Cohort identity contains an invalid relative path/name.")
        result[name] = _digest(digest)
    return result


def _budget(value: object, *, maximum: int = MAX_BUDGET_FIELDS) -> dict[str, int]:
    """Require one exact finite nonnegative integer budget vector."""
    record = _object(value)
    if not 0 < len(record) <= maximum:
        raise Incomplete("Cohort source-work budget has an invalid field or bound.")
    result: dict[str, int] = {}
    for key, amount in record.items():
        if (not key or len(key) > 160 or type(amount) is not int
                or not 0 <= amount <= (1 << 63) - 1):
            raise Incomplete("Cohort source-work budget has an invalid field or bound.")
        result[key] = amount
    return result


def _integer(value: object, *, minimum: int = 0) -> int:
    """Keep Boolean values outside all integer accounting units."""
    if type(value) is not int or not minimum <= value <= (1 << 63) - 1:
        raise Incomplete("Cohort count is not an exact finite integer.")
    return value


def _inventories(value: object) -> dict[str, Inventory]:
    """Admit exact bounded per-owner count/digest pairs."""
    result: dict[str, Inventory] = {}
    rows = _object(value)
    if not 0 < len(rows) <= MAX_OWNERS:
        raise Incomplete("Critical-file inventory has an invalid owner count.")
    for owner, raw in rows.items():
        row = _object(raw)
        if set(row) != {"count", "sha256"}:
            raise Incomplete("One critical-file inventory has unknown or missing fields.")
        count = _integer(row["count"], minimum=1)
        if count > MAX_COHORT_VARIANTS:
            raise Incomplete("One critical-file inventory is empty or exceeds the focused 512 cap.")
        result[owner] = {"count": count, "sha256": _digest(row["sha256"])}
    return result


def _registered(names: set[str]) -> dict[str, str]:
    """Select an exact reviewed owner namespace, never arbitrary singleton owners."""
    for owners in (S157_OWNERS, S158_OWNERS, REFERENCE_INPUT_OWNERS,
                   CHART_COMPOSITION_OWNERS, SETTLED_RATE_OWNERS):
        if names == set(owners):
            return owners
    raise Incomplete("Checkpoint must contain exactly three changed critical owners, nine registered S158 owners, one registered reference-input owner, the exact chart-composition trio or the exact settled-rate quartet.")


def _field_limit(inventories: dict[str, Inventory]) -> int:
    """Check a closed namespace/aggregate, then select its independent field cap."""
    owners = _registered(set(inventories))
    ceiling = (MAX_SETTLED_RATE_LISTING_VARIANTS if owners == SETTLED_RATE_OWNERS else
               MAX_CHART_COMPOSITION_LISTING_VARIANTS if owners == CHART_COMPOSITION_OWNERS else
               MAX_S158_LISTING_VARIANTS if len(owners) == 9 else MAX_LISTING_VARIANTS)
    if sum(row["count"] for row in inventories.values()) > ceiling:
        raise Incomplete("Checkpoint inventory exceeds the separate listing ceiling.")
    if owners == SETTLED_RATE_OWNERS:
        return MAX_SETTLED_RATE_BUDGET_FIELDS
    if owners == CHART_COMPOSITION_OWNERS:
        return MAX_CHART_COMPOSITION_BUDGET_FIELDS
    if len(owners) == 1:
        return MAX_REFERENCE_INPUT_BUDGET_FIELDS
    return MAX_BUDGET_FIELDS if len(owners) == 3 else MAX_S158_BUDGET_FIELDS


def historical_s148_remaining_checks(value: object) -> RemainingChecks:
    """Retain checkpoint totals: one coverage and two contract attempts.

    The historical field name includes one spent, incomplete contract attempt
    and exactly one fresh retry; it never authorizes two future attempts.
    """
    row = _object(value)
    expected = {"coverage": 1, "contracts": 2}
    if set(row) != set(CHECKS) or any(type(count) is not int or count != expected[name]
                                    for name, count in row.items()):
        raise Incomplete("Checkpoint must reserve one coverage and two total contract attempts.")
    return {"coverage": _integer(row["coverage"]), "contracts": _integer(row["contracts"])}


def _remaining_checks(value: object) -> RemainingChecks:
    """Reserve one fresh coverage and contracts lane, with no retry credit."""
    row = _object(value)
    if set(row) != set(CHECKS) or any(type(count) is not int or count != 1 for count in row.values()):
        raise Incomplete("Checkpoint must reserve exactly one coverage and one contracts invocation.")
    return {"coverage": _integer(row["coverage"]), "contracts": _integer(row["contracts"])}


def _reservation(value: object, *, maximum: int = MAX_BUDGET_FIELDS) -> SourceReservation:
    """Project the complete source-work reservation, never observed consumption."""
    row = _object(value)
    if set(row) != {"mutation_variants", "mutation_baselines", "mutation_package_invocations", "source_work"}:
        raise Incomplete("Coordinator cumulative reservation has unknown or missing fields.")
    return {"mutation_variants": _integer(row["mutation_variants"]),
            "mutation_baselines": _integer(row["mutation_baselines"]),
            "mutation_package_invocations": _integer(row["mutation_package_invocations"]),
            "source_work": _budget(row["source_work"], maximum=maximum)}


def _same(actual: object, expected: object) -> bool:
    """Preserve type distinctions, including bool versus int."""
    return type(actual) is type(expected) and canonical_sha256(actual) == canonical_sha256(expected)


def _reservation_inputs(plan: object) -> ReservationInputs:
    """Read only the two fields needed by the pre-admission recipe calculator."""
    row = _object(plan)
    if not {"inventories", "remaining_checks"} <= set(row):
        raise Incomplete("Coordinator reservation inputs are missing.")
    return {"inventories": _inventories(row["inventories"]),
            "remaining_checks": _remaining_checks(row["remaining_checks"])}


def plan_reservation(plan: object, package_budget: object, check_budgets: object) -> SourceReservation:
    """Sum every variant, one baseline per file and total checkpoint checks.

    This is a source-derived upper reservation before effects. It is never
    reduced for an eventual unviable variant, failed assertion or partial log.
    No profile transfers predecessor results or grants a retry allowance.
    Native time/process limits remain the wrapper's separate obligations.
    """
    admitted = _reservation_inputs(plan)
    maximum = _field_limit(admitted["inventories"])
    package = _budget(package_budget, maximum=maximum)
    raw_checks = _object(check_budgets)
    if set(raw_checks) != set(CHECKS):
        raise Incomplete("Remaining-check reservations differ from the package budget schema.")
    checks: CheckBudgets = {"coverage": _budget(raw_checks["coverage"], maximum=maximum),
                           "contracts": _budget(raw_checks["contracts"], maximum=maximum)}
    if any(set(row) != set(package) for row in (checks["coverage"], checks["contracts"])):
        raise Incomplete("Remaining-check reservations differ from the package budget schema.")
    packages = sum(row["count"] + 1 for row in admitted["inventories"].values())
    totals = {key: packages * amount for key, amount in package.items()}
    for name in CHECKS:
        for key, amount in checks[name].items():
            totals[key] += admitted["remaining_checks"][name] * amount
    return {"mutation_variants": packages - len(admitted["inventories"]),
            "mutation_baselines": len(admitted["inventories"]),
            "mutation_package_invocations": packages, "source_work": totals}


def _cohorts(value: object, owners: dict[str, str]) -> list[Cohort]:
    """Project exact named subject/path rows; partition ownership is checked later."""
    if not is_exact_list(value) or len(value) != len(owners):
        raise Incomplete("Checkpoint partition must contain the three retained-chart owners, nine registered S158 owners, one reference-input owner or four settled-rate owners once.")
    result: list[Cohort] = []
    for raw in value:
        row = _object(raw)
        if set(row) != COHORT_KEYS:
            raise Incomplete("Cohort partition has unknown or missing fields.")
        name, owner, output = row["id"], row["subject"], row["output_dir"]
        if type(name) is not str or type(owner) is not str or type(output) is not str:
            raise Incomplete("Cohort partition requires string identity and path fields.")
        result.append({"id": name, "subject": owner, "output_dir": output})
    return result


def _plan(value: object) -> CohortPlan:
    """Project the complete plan only after strict shape and field validation."""
    row = _object(value)
    if set(row) != PLAN_KEYS:
        raise Incomplete("Coordinator cohort plan has unknown or missing fields.")
    if (type(row["schema_version"]) is not int or row["schema_version"] != 11
            or row["status"] != "admitted-exact-source-and-cohorts"):
        raise Incomplete("Coordinator cohort plan awaits final-source/inventory binding.")
    inventories = _inventories(row["inventories"])
    owners = _registered(set(inventories))
    cohorts = _cohorts(row["cohorts"], owners)
    _partition(cohorts, owners)
    product = _identity(row["product_inputs_sha256"])
    if not set(owners.values()) <= set(product):
        raise Incomplete("Registered owner sources are missing from the complete product binding.")
    return {"schema_version": 11, "status": "admitted-exact-source-and-cohorts",
            "candidate_sha256": _digest(row["candidate_sha256"]),
            "product_inputs_sha256": product,
            "tool_inputs_sha256": _identity(row["tool_inputs_sha256"]),
            "tool_payloads_sha256": _identity(row["tool_payloads_sha256"]),
            "recipe_sha256": _digest(row["recipe_sha256"]),
            "inventories": inventories, "cohorts": cohorts,
            "remaining_checks": _remaining_checks(row["remaining_checks"]),
            "baseline_tests": _integer(row["baseline_tests"], minimum=1),
            "cumulative_reservation": _reservation(row["cumulative_reservation"],
                                                    maximum=_field_limit(inventories))}


def _subjects(value: object) -> dict[str, str]:
    """Require one closed reviewed name/path registration without rebinding."""
    raw = _object(value)
    result: dict[str, str] = {}
    for name, path in raw.items():
        if type(path) is not str:
            raise Incomplete("Checkpoint source owner paths must be strings.")
        result[name] = path
    if result != _registered(set(result)):
        raise Incomplete("Checkpoint scope must contain exactly three changed critical owners, nine registered S158 owners, one reference-input owner, the chart-composition trio or settled-rate quartet at their exact paths.")
    return result


def _partition(cohorts: list[Cohort], owners_by_name: dict[str, str]) -> None:
    """Check a complete disjoint owner/output partition on every plan entrypoint."""
    ids: set[str] = set()
    owners: set[str] = set()
    outputs: set[str] = set()
    for row in cohorts:
        name, owner, output = row["id"], row["subject"], row["output_dir"]
        if (_NAME.fullmatch(name) is None or name in ids
                or owner not in owners_by_name or owner in owners):
            raise Incomplete("Cohort partition contains an unknown, duplicate or overlapping owner/id.")
        if (len(output) > 1024 or not PurePosixPath(output).is_absolute()
                or str(PurePosixPath(output)) != output or ".." in PurePosixPath(output).parts
                or output in outputs
                or any(PurePosixPath(output).is_relative_to(PurePosixPath(previous))
                       or PurePosixPath(previous).is_relative_to(PurePosixPath(output)) for previous in outputs)):
            raise Incomplete("Cohorts require distinct exact absolute new output paths.")
        ids.add(name)
        owners.add(owner)
        outputs.add(output)
    if owners != set(owners_by_name):
        raise Incomplete("Coordinator partition omits a critical owner.")


def admit_plan(plan: object, *, subjects: object, product_inputs: object, tool_inputs: object,
               recipe_sha256: object, inventories: object, baseline_tests: object,
               package_budget: object, check_budgets: object, global_limits: object) -> PlanAdmission:
    """Admit a complete disjoint focused partition against current frozen facts.

    Each cohort selects one existing named owner through the wrapper's ordinary
    --subject switch. A file exceeding 512 cannot be descriptor-partitioned by
    this schema. Historical profiles retain512/4608/512 aggregate ceilings; the
    composition trio has1536 and settled-rate quartet2048. No profile widens an
    individual cohort beyond512.
    """
    admitted = _plan(plan)
    owners_by_name = _subjects(subjects)
    current_product = _identity(product_inputs)
    if not set(owners_by_name.values()) <= set(current_product):
        raise Incomplete("Registered owner sources are missing from the complete product binding.")
    if (not _same(admitted["product_inputs_sha256"], current_product)
            or admitted["candidate_sha256"] != canonical_sha256(current_product)):
        raise Incomplete("Coordinator cohort plan names a different complete product candidate.")
    if not _same(admitted["tool_inputs_sha256"], _identity(tool_inputs)):
        raise Incomplete("Coordinator cohort plan names different tool sources.")
    if admitted["recipe_sha256"] != _digest(recipe_sha256):
        raise Incomplete("Coordinator cohort plan names different source-call recipes.")
    if admitted["baseline_tests"] != _integer(baseline_tests, minimum=1):
        raise Incomplete("Coordinator baseline test count differs from final source discovery.")
    current_inventories = _inventories(inventories)
    if (set(current_inventories) != set(owners_by_name)
            or not _same(admitted["inventories"], current_inventories)):
        raise Incomplete("Coordinator inventory union is missing, unknown or changed.")
    ceiling = (MAX_SETTLED_RATE_LISTING_VARIANTS if owners_by_name == SETTLED_RATE_OWNERS else
               MAX_CHART_COMPOSITION_LISTING_VARIANTS if owners_by_name == CHART_COMPOSITION_OWNERS else
               MAX_S158_LISTING_VARIANTS if len(owners_by_name) == 9 else MAX_LISTING_VARIANTS)
    if sum(row["count"] for row in current_inventories.values()) > ceiling:
        raise Incomplete("Checkpoint inventory exceeds the separate listing ceiling.")
    if set(admitted["inventories"]) != set(owners_by_name):
        raise Incomplete("Coordinator partition omits a critical owner.")
    reservation = plan_reservation(admitted, package_budget, check_budgets)
    if not _same(admitted["cumulative_reservation"], reservation):
        raise Incomplete("Coordinator cumulative reservation omits variant, baseline or remaining-check work.")
    limits = _budget(global_limits, maximum=_field_limit(admitted["inventories"]))
    if (set(limits) != set(reservation["source_work"])
            or any(amount > limits[key] for key, amount in reservation["source_work"].items())):
        raise Incomplete("Complete cohort union exceeds the existing cumulative source-work allowances.")
    return {"plan_sha256": canonical_sha256(plan), "reservation": reservation,
            "qualification": "pre-effect reservation only; no campaign or quality result"}


def cohort_binding(plan: object, cohort_id: str, *, subject: str, output_dir: str) -> CohortBinding:
    """Select exactly one admitted named subject/path, never an arbitrary subset."""
    admitted = _plan(plan)
    matches = [row for row in admitted["cohorts"] if row["id"] == cohort_id]
    if (len(matches) != 1 or matches[0]["subject"] != subject
            or matches[0]["output_dir"] != output_dir):
        raise Incomplete("Invocation does not match its unique planned focused subject/output.")
    return {"plan_sha256": canonical_sha256(plan), "cohort_id": cohort_id,
            "candidate_sha256": admitted["candidate_sha256"], "recipe_sha256": admitted["recipe_sha256"]}


def reconcile_receipts(plan: object, receipts: object, *, subjects: object) -> ReceiptUnion:
    """Check a complete actual successful receipt union without averaging gates.

    The caller must first admit_plan against current source/tool/recipe facts.
    Passing receipts retain one successful full-package baseline per owner and
    exact descriptor identities. Missing/partial receipts cannot become zero.
    """
    admitted = _plan(plan)
    owners_by_name = _subjects(subjects)
    if set(admitted["inventories"]) != set(owners_by_name):
        raise Incomplete("Receipt union names a different registered owner profile.")
    if not is_exact_list(receipts) or len(receipts) != len(admitted["cohorts"]):
        raise Incomplete("Receipt union is missing or duplicates a focused cohort.")
    seen_ids: set[str] = set()
    seen_receipts: set[str] = set()
    seen_owners: set[str] = set()
    plan_hash = canonical_sha256(plan)
    for raw in receipts:
        receipt = _object(raw)
        digest = canonical_sha256(receipt)
        binding = _object(receipt.get("core_cohort_binding", {}))
        cohort_id = binding.get("cohort_id")
        matching = [row for row in admitted["cohorts"] if row["id"] == cohort_id]
        if (type(cohort_id) is not str or len(matching) != 1
                or cohort_id in seen_ids or digest in seen_receipts):
            raise Incomplete("Unknown or duplicate cohort/receipt in union.")
        cohort = matching[0]
        subject = cohort["subject"]
        if subject not in owners_by_name:
            raise Incomplete("Focused receipt names an unknown source owner.")
        expected_binding: CohortBinding = {
            "plan_sha256": plan_hash, "cohort_id": cohort_id,
            "candidate_sha256": admitted["candidate_sha256"], "recipe_sha256": admitted["recipe_sha256"]}
        sources = _object(receipt.get("source_sha256", {}))
        expected_sources = {**admitted["product_inputs_sha256"], **admitted["tool_inputs_sha256"]}
        if (receipt.get("status") != "passed" or receipt.get("selected_lane") != "mutation"
                or receipt.get("owned_output") != cohort["output_dir"]
                or not _same(binding, expected_binding)
                or not _same(receipt.get("subjects"), {subject: owners_by_name[subject]})
                or any(sources.get(name) != value for name, value in expected_sources.items())
                or not _same(receipt.get("tools_sha256"), admitted["tool_payloads_sha256"])
                or not _same(receipt.get("mutation_reviewed_inventories"), {subject: admitted["inventories"][subject]})):
            raise Incomplete("Focused receipt differs in outcome, source, tools, recipe, scope or inventory.")
        mutation = _object(receipt.get("mutation", {}))
        row = _object(mutation.get(subject, {}))
        baseline = row.get("baseline_tests_executed")
        required = receipt.get("mutation_baselines_required")
        observed = receipt.get("mutation_baselines_observed")
        generated = receipt.get("mutation_generated_variants_selected_total")
        if (set(mutation) != {subject} or row.get("gate_passed") is not True
                or type(baseline) is not int or baseline != admitted["baseline_tests"]
                or type(required) is not int or required != 1
                or type(observed) is not int or observed != 1
                or receipt.get("mutation_missing_selected_files") != []
                or receipt.get("mutation_unprocessed_selected_files") != []
                or receipt.get("core_observation_pending_invocations") != []
                or type(generated) is not int or generated != admitted["inventories"][subject]["count"]):
            raise Incomplete("Focused receipt lacks a successful complete baseline/inventory/per-file gate.")
        seen_ids.add(cohort_id)
        seen_receipts.add(digest)
        seen_owners.add(subject)
    if seen_owners != set(owners_by_name):
        raise Incomplete("Focused receipt union omits a critical owner.")
    return {"status": "passed-exact-mutation-union", "plan_sha256": plan_hash,
            "critical_owners": sorted(seen_owners), "focused_receipts": len(receipts),
            "generated_variants": sum(row["count"] for row in admitted["inventories"].values()),
            "baselines": len(receipts), "reservation": admitted["cumulative_reservation"],
            "critical_manifest_complete_selection": False,
            "scope": ("Four settled-rate critical owner mutation receipts; coverage/contracts and native Windows remain separate."
                      if owners_by_name == SETTLED_RATE_OWNERS else
                      "Three chart-composition critical owner mutation receipts; coverage/contracts and native Windows remain separate."
                      if owners_by_name == CHART_COMPOSITION_OWNERS else
                      "Three retained-chart critical owner mutation receipts; coverage/contracts and native Windows remain separate."
                      if len(owners_by_name) == 3 else
                      "One reference-input critical owner mutation receipt; coverage/contracts and native Windows remain separate."
                      if len(owners_by_name) == 1 else
                      "Nine chart-combination critical owner mutation receipts; coverage/contracts and native Windows remain separate.")}
