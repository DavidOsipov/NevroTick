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


class Incomplete(ValueError):
    """A closed coordinator plan or its receipt union is incomplete."""


MAX_COHORT_VARIANTS = 512
MAX_LISTING_VARIANTS = 3072
MAX_OWNERS = 2
MAX_INPUTS = 256
MAX_BUDGET_FIELDS = 265
PLAN_KEYS = frozenset({
    "schema_version", "status", "candidate_sha256", "product_inputs_sha256",
    "tool_inputs_sha256", "tool_payloads_sha256", "recipe_sha256", "inventories",
    "cohorts", "remaining_checks", "baseline_tests", "cumulative_reservation",
})
COHORT_KEYS = frozenset({"id", "subject", "output_dir"})
CHECKS = ("coverage", "contracts")
_SHA256 = re.compile(r"[0-9a-f]{64}")
_NAME = re.compile(r"[a-z][a-z0-9_]{0,63}")


def canonical_sha256(value):
    """Hash an already validated JSON value using the shared canonical form."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def _digest(value):
    """Require an actual SHA-256; pending labels never admit execution."""
    if type(value) is not str or _SHA256.fullmatch(value) is None:
        raise Incomplete("Cohort binding is pending or is not an exact SHA-256.")
    return value


def _identity(value):
    """Validate bounded nonempty path/name bindings without coercion."""
    if type(value) is not dict or not 0 < len(value) <= MAX_INPUTS:
        raise Incomplete("Cohort identity inventory has an invalid size or shape.")
    for name, digest in value.items():
        if (type(name) is not str or not 0 < len(name) <= 256
                or PurePosixPath(name).is_absolute() or ".." in PurePosixPath(name).parts):
            raise Incomplete("Cohort identity contains an invalid relative path/name.")
        _digest(digest)
    return value


def _budget(value):
    """Require one exact finite nonnegative integer budget vector."""
    if (type(value) is not dict or not 0 < len(value) <= MAX_BUDGET_FIELDS
            or any(type(key) is not str or not key or len(key) > 160
                   or type(amount) is not int or not 0 <= amount <= (1 << 63) - 1
                   for key, amount in value.items())):
        raise Incomplete("Cohort source-work budget has an invalid field or bound.")
    return value


def _same(actual, expected):
    """Preserve type distinctions, including bool versus int."""
    return type(actual) is type(expected) and canonical_sha256(actual) == canonical_sha256(expected)


def plan_reservation(plan, package_budget, check_budgets):
    """Sum every variant, one baseline per file and the remaining checks.

    This is a source-derived upper reservation before effects. It is never
    reduced for an eventual unviable variant, failed assertion or partial log.
    Native time/process limits remain the wrapper's separate obligations.
    """
    package_budget = _budget(package_budget)
    if (type(check_budgets) is not dict or set(check_budgets) != set(CHECKS)
            or any(set(_budget(row)) != set(package_budget) for row in check_budgets.values())):
        raise Incomplete("Remaining-check reservations differ from the package budget schema.")
    packages = sum(row["count"] + 1 for row in plan["inventories"].values())
    totals = {key: packages * amount for key, amount in package_budget.items()}
    for name in CHECKS:
        for key, amount in check_budgets[name].items():
            totals[key] += plan["remaining_checks"][name] * amount
    return {"mutation_variants": packages - len(plan["inventories"]),
            "mutation_baselines": len(plan["inventories"]),
            "mutation_package_invocations": packages, "source_work": totals}


def admit_plan(plan, *, subjects, product_inputs, tool_inputs, recipe_sha256,
               inventories, baseline_tests, package_budget, check_budgets, global_limits):
    """Admit a complete disjoint focused partition against current frozen facts.

    Each cohort selects one existing named owner through the wrapper's ordinary
    --subject switch. A file exceeding 512 cannot be descriptor-partitioned by
    this schema. Listing discovery has its separate 3072 aggregate ceiling.
    """
    if type(plan) is not dict or set(plan) != PLAN_KEYS:
        raise Incomplete("Coordinator cohort plan has unknown or missing fields.")
    if (type(plan["schema_version"]) is not int or plan["schema_version"] != 1
            or plan["status"] != "admitted-exact-source-and-cohorts"):
        raise Incomplete("Coordinator cohort plan awaits final-source/inventory binding.")
    if (type(subjects) is not dict or len(subjects) != MAX_OWNERS
            or len(set(subjects.values())) != MAX_OWNERS):
        raise Incomplete("Checkpoint scope must contain exactly two distinct changed critical owners.")
    if (not _same(_identity(plan["product_inputs_sha256"]), _identity(product_inputs))
            or plan["candidate_sha256"] != canonical_sha256(product_inputs)):
        raise Incomplete("Coordinator cohort plan names a different complete product candidate.")
    _digest(plan["candidate_sha256"])
    if not _same(_identity(plan["tool_inputs_sha256"]), _identity(tool_inputs)):
        raise Incomplete("Coordinator cohort plan names different tool sources.")
    _identity(plan["tool_payloads_sha256"])
    if _digest(plan["recipe_sha256"]) != _digest(recipe_sha256):
        raise Incomplete("Coordinator cohort plan names different source-call recipes.")
    if (type(plan["baseline_tests"]) is not int or plan["baseline_tests"] <= 0
            or type(baseline_tests) is not int or plan["baseline_tests"] != baseline_tests):
        raise Incomplete("Coordinator baseline test count differs from final source discovery.")
    if (type(inventories) is not dict or set(inventories) != set(subjects)
            or not _same(plan["inventories"], inventories)):
        raise Incomplete("Coordinator inventory union is missing, unknown or changed.")
    for row in inventories.values():
        if (type(row) is not dict or set(row) != {"count", "sha256"}
                or type(row["count"]) is not int or not 0 < row["count"] <= MAX_COHORT_VARIANTS):
            raise Incomplete("One critical-file inventory is empty or exceeds the focused 512 cap.")
        _digest(row["sha256"])
    if sum(row["count"] for row in inventories.values()) > MAX_LISTING_VARIANTS:
        raise Incomplete("Checkpoint inventory exceeds the separate listing ceiling.")
    cohorts = plan["cohorts"]
    if type(cohorts) is not list or len(cohorts) != MAX_OWNERS:
        raise Incomplete("Checkpoint partition must contain each of two owners once.")
    ids, owners, outputs = set(), set(), set()
    for row in cohorts:
        if type(row) is not dict or set(row) != COHORT_KEYS:
            raise Incomplete("Cohort partition has unknown or missing fields.")
        name, owner, output = row["id"], row["subject"], row["output_dir"]
        if (type(name) is not str or _NAME.fullmatch(name) is None or name in ids
                or type(owner) is not str or owner not in subjects or owner in owners):
            raise Incomplete("Cohort partition contains an unknown, duplicate or overlapping owner/id.")
        if (type(output) is not str or len(output) > 1024 or not PurePosixPath(output).is_absolute()
                or str(PurePosixPath(output)) != output or ".." in PurePosixPath(output).parts
                or output in outputs
                or any(PurePosixPath(output).is_relative_to(PurePosixPath(previous))
                       or PurePosixPath(previous).is_relative_to(PurePosixPath(output)) for previous in outputs)):
            raise Incomplete("Cohorts require distinct exact absolute new output paths.")
        ids.add(name); owners.add(owner); outputs.add(output)
    if owners != set(subjects):
        raise Incomplete("Coordinator partition omits a critical owner.")
    if (type(plan["remaining_checks"]) is not dict or set(plan["remaining_checks"]) != set(CHECKS)
            or any(type(count) is not int or count != 1 for count in plan["remaining_checks"].values())):
        raise Incomplete("Checkpoint must reserve one coverage and contracts invocation.")
    reservation = plan_reservation(plan, package_budget, check_budgets)
    if not _same(plan["cumulative_reservation"], reservation):
        raise Incomplete("Coordinator cumulative reservation omits variant, baseline or remaining-check work.")
    limits = _budget(global_limits)
    if (set(limits) != set(reservation["source_work"])
            or any(amount > limits[key] for key, amount in reservation["source_work"].items())):
        raise Incomplete("Complete cohort union exceeds the existing cumulative source-work allowances.")
    return {"plan_sha256": canonical_sha256(plan), "reservation": reservation,
            "qualification": "pre-effect reservation only; no campaign or quality result"}


def cohort_binding(plan, cohort_id, *, subject, output_dir):
    """Select exactly one admitted named subject/path, never an arbitrary subset."""
    matches = [row for row in plan["cohorts"] if row["id"] == cohort_id]
    if (len(matches) != 1 or matches[0]["subject"] != subject
            or matches[0]["output_dir"] != output_dir):
        raise Incomplete("Invocation does not match its unique planned focused subject/output.")
    return {"plan_sha256": canonical_sha256(plan), "cohort_id": cohort_id,
            "candidate_sha256": plan["candidate_sha256"], "recipe_sha256": plan["recipe_sha256"]}


def reconcile_receipts(plan, receipts, *, subjects):
    """Check a complete actual successful receipt union without averaging gates.

    The caller must first admit_plan against current source/tool/recipe facts.
    Passing receipts retain one successful full-package baseline per owner and
    exact descriptor identities. Missing/partial receipts cannot become zero.
    """
    if type(receipts) is not list or len(receipts) != len(plan["cohorts"]):
        raise Incomplete("Receipt union is missing or duplicates a focused cohort.")
    seen_ids, seen_receipts, seen_owners = set(), set(), set()
    plan_hash = canonical_sha256(plan)
    for receipt in receipts:
        if type(receipt) is not dict:
            raise Incomplete("Focused native receipt must be an object.")
        digest = canonical_sha256(receipt)
        binding = receipt.get("core_cohort_binding", {})
        cohort_id = binding.get("cohort_id")
        matching = [row for row in plan["cohorts"] if row["id"] == cohort_id]
        if len(matching) != 1 or cohort_id in seen_ids or digest in seen_receipts:
            raise Incomplete("Unknown or duplicate cohort/receipt in union.")
        cohort = matching[0]; subject = cohort["subject"]
        expected_binding = {"plan_sha256": plan_hash, "cohort_id": cohort_id,
                            "candidate_sha256": plan["candidate_sha256"], "recipe_sha256": plan["recipe_sha256"]}
        sources = receipt.get("source_sha256", {})
        expected_sources = {**plan["product_inputs_sha256"], **plan["tool_inputs_sha256"]}
        if (receipt.get("status") != "passed" or receipt.get("selected_lane") != "mutation"
                or receipt.get("owned_output") != cohort["output_dir"]
                or not _same(binding, expected_binding)
                or not _same(receipt.get("subjects"), {subject: subjects[subject]})
                or type(sources) is not dict
                or any(sources.get(name) != value for name, value in expected_sources.items())
                or not _same(receipt.get("tools_sha256"), plan["tool_payloads_sha256"])
                or not _same(receipt.get("mutation_reviewed_inventories"), {subject: plan["inventories"][subject]})):
            raise Incomplete("Focused receipt differs in outcome, source, tools, recipe, scope or inventory.")
        mutation = receipt.get("mutation", {})
        row = mutation.get(subject, {}) if type(mutation) is dict else {}
        if (type(mutation) is not dict or set(mutation) != {subject} or type(row) is not dict
                or row.get("gate_passed") is not True
                or type(row.get("baseline_tests_executed")) is not int
                or row.get("baseline_tests_executed") != plan["baseline_tests"]
                or type(receipt.get("mutation_baselines_required")) is not int
                or type(receipt.get("mutation_baselines_observed")) is not int
                or receipt.get("mutation_baselines_required") != 1
                or receipt.get("mutation_baselines_observed") != 1
                or receipt.get("mutation_missing_selected_files") != []
                or receipt.get("mutation_unprocessed_selected_files") != []
                or receipt.get("core_observation_pending_invocations") != []
                or type(receipt.get("mutation_generated_variants_selected_total")) is not int
                or receipt.get("mutation_generated_variants_selected_total") != plan["inventories"][subject]["count"]):
            raise Incomplete("Focused receipt lacks a successful complete baseline/inventory/per-file gate.")
        seen_ids.add(cohort_id); seen_receipts.add(digest); seen_owners.add(subject)
    if seen_owners != set(subjects):
        raise Incomplete("Focused receipt union omits a critical owner.")
    return {"status": "passed-exact-mutation-union", "plan_sha256": plan_hash,
            "critical_owners": sorted(seen_owners), "focused_receipts": len(receipts),
            "generated_variants": sum(row["count"] for row in plan["inventories"].values()),
            "baselines": len(receipts), "reservation": plan["cumulative_reservation"],
            "critical_manifest_complete_selection": False,
            "scope": "Two changed critical owners' mutation receipt union; coverage/contracts and native Windows remain separate."}
