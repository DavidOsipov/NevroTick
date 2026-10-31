"""Finite pure controls for coordinator partition and receipt-union admission."""
import copy
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "core_analysis_cohorts", Path(__file__).resolve().parents[1] / "core-analysis-cohorts.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class CohortPlanTests(unittest.TestCase):
    """Synthetic identities test admission mechanics, never a product candidate."""

    def fixture(self):
        subjects = {f"owner{index}": f"src/owner{index}.rs" for index in range(4)}
        product = {"Cargo.toml": "a" * 64}
        tools = {"scripts/wrapper.py": "b" * 64}
        inventories = {name: {"count": 300 + index, "sha256": str(index) * 64}
                       for index, name in enumerate(subjects)}
        package = {"fits": 2, "points": 7}
        checks = {"coverage": dict(package), "contracts": {"fits": 3, "points": 4}}
        plan = {"schema_version": 1, "status": "admitted-exact-source-and-cohorts",
                "candidate_sha256": MODULE.canonical_sha256(product),
                "product_inputs_sha256": product, "tool_inputs_sha256": tools,
                "tool_payloads_sha256": {"cargo-mutants": "e" * 64},
                "recipe_sha256": "c" * 64, "inventories": inventories,
                "cohorts": [{"id": name, "subject": name, "output_dir": "/tmp/" + name}
                            for name in subjects], "remaining_checks": {name: 1 for name in MODULE.CHECKS},
                "baseline_tests": 13, "cumulative_reservation": None}
        plan["cumulative_reservation"] = MODULE.plan_reservation(plan, package, checks)
        arguments = {"subjects": subjects, "product_inputs": product, "tool_inputs": tools,
                     "recipe_sha256": "c" * 64, "inventories": inventories, "baseline_tests": 13,
                     "package_budget": package, "check_budgets": checks,
                     "global_limits": dict(plan["cumulative_reservation"]["source_work"])}
        return plan, arguments

    def receipts(self, plan, subjects):
        result = []
        for cohort in plan["cohorts"]:
            subject = cohort["subject"]
            result.append({"status": "passed", "selected_lane": "mutation",
                           "owned_output": cohort["output_dir"],
                           "core_cohort_binding": MODULE.cohort_binding(
                               plan, cohort["id"], subject=subject, output_dir=cohort["output_dir"]),
                           "subjects": {subject: subjects[subject]},
                           "source_sha256": {**plan["product_inputs_sha256"], **plan["tool_inputs_sha256"]},
                           "tools_sha256": plan["tool_payloads_sha256"],
                           "mutation_reviewed_inventories": {subject: plan["inventories"][subject]},
                           "mutation": {subject: {"gate_passed": True, "baseline_tests_executed": 13}},
                           "mutation_baselines_required": 1, "mutation_baselines_observed": 1,
                           "mutation_missing_selected_files": [], "mutation_unprocessed_selected_files": [],
                           "core_observation_pending_invocations": [],
                           "mutation_generated_variants_selected_total": plan["inventories"][subject]["count"]})
        return result

    def test_disjoint_four_owner_union_can_exceed_one_cohort_cap(self):
        plan, args = self.fixture()
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted["reservation"]["mutation_variants"], 1206)
        self.assertEqual(admitted["reservation"]["mutation_baselines"], 4)
        self.assertEqual(admitted["reservation"]["source_work"], {"fits": 2425, "points": 8481})
        union = MODULE.reconcile_receipts(plan, self.receipts(plan, args["subjects"]), subjects=args["subjects"])
        self.assertEqual(union["focused_receipts"], 4)
        self.assertEqual(union["generated_variants"], 1206)
        self.assertFalse(union["critical_manifest_complete_selection"])

    def test_scope_requires_exactly_four_distinct_owners(self):
        for change in ("one", "three", "five", "duplicate_file"):
            plan, args = self.fixture()
            if change == "one":
                args['subjects'] = {'owner0': args['subjects']['owner0']}
            elif change == "three":
                args['subjects'].pop('owner3')
            elif change == "five":
                args['subjects']['owner4'] = 'src/owner4.rs'
            else:
                args['subjects']['owner1'] = args['subjects']['owner0']
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_reviewed_366_field_width_accepts_exact_limit_and_rejects_next(self):
        exact = {f"metric{index}": 1 for index in range(366)}
        self.assertEqual(MODULE._budget(exact), exact)
        with self.assertRaises(MODULE.Incomplete):
            MODULE._budget({**exact, 'metric366': 1})

    def test_typed_budget_projection_rejects_boolean_and_unbounded_counts(self):
        self.assertEqual(MODULE._budget({"fits": 0, "points": (1 << 63) - 1}),
                         {"fits": 0, "points": (1 << 63) - 1})
        for budget in ({"fits": True}, {"fits": -1}, {"fits": 1 << 63},
                       {"fits": 1.0}, {1: 1}, []):
            with self.subTest(budget=budget), self.assertRaises(MODULE.Incomplete):
                MODULE._budget(budget)

    def test_malformed_nested_receipts_fail_closed_after_typed_projection(self):
        for field in ("core_cohort_binding", "source_sha256", "mutation"):
            plan, args = self.fixture()
            receipts = self.receipts(plan, args["subjects"])
            receipts[0][field] = []
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args["subjects"])

    def test_boolean_inventory_and_baseline_counts_remain_invalid(self):
        for change in ("inventory", "baseline", "required", "observed", "generated"):
            plan, args = self.fixture()
            receipts = self.receipts(plan, args["subjects"])
            if change == "inventory":
                plan["inventories"]["owner0"]["count"] = True
            elif change == "baseline":
                receipts[0]["mutation"]["owner0"]["baseline_tests_executed"] = True
            elif change == "required":
                receipts[0]["mutation_baselines_required"] = True
            elif change == "observed":
                receipts[0]["mutation_baselines_observed"] = True
            else:
                receipts[0]["mutation_generated_variants_selected_total"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args["subjects"])

    def test_unknown_missing_and_pending_plan_fields_reject(self):
        for change in ("unknown", "missing", "pending", "schema_bool"):
            plan, args = self.fixture()
            if change == "unknown": plan["extra"] = 0
            if change == "missing": del plan["inventories"]
            if change == "pending": plan["status"] = "PENDING"
            if change == "schema_bool": plan["schema_version"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_partition_rejects_unknown_duplicate_missing_overlap_and_output_reuse(self):
        for change in ("unknown", "duplicate_id", "missing", "overlap", "same_output", "relative", "traversal"):
            plan, args = self.fixture()
            if change == "unknown": plan["cohorts"][0]["subject"] = "unknown"
            if change == "duplicate_id": plan["cohorts"][1]["id"] = plan["cohorts"][0]["id"]
            if change == "missing": plan["cohorts"].pop()
            if change == "overlap": plan["cohorts"][1]["subject"] = plan["cohorts"][0]["subject"]
            if change == "same_output": plan["cohorts"][1]["output_dir"] = plan["cohorts"][0]["output_dir"]
            if change == "relative": plan["cohorts"][0]["output_dir"] = "tmp/owner0"
            if change == "traversal": plan["cohorts"][0]["output_dir"] = "/tmp/../owner0"
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_current_candidate_tools_recipe_baseline_and_inventory_must_match(self):
        for change in ("candidate", "product", "tools", "recipe", "baseline", "inventory", "empty", "oversized"):
            plan, args = self.fixture(); plan = copy.deepcopy(plan)
            if change == "candidate": plan["candidate_sha256"] = "d" * 64
            if change == "product": plan["product_inputs_sha256"]["Cargo.toml"] = "d" * 64
            if change == "tools": plan["tool_inputs_sha256"]["scripts/wrapper.py"] = "d" * 64
            if change == "recipe": plan["recipe_sha256"] = "d" * 64
            if change == "baseline": plan["baseline_tests"] += 1
            if change == "inventory": plan["inventories"]["owner0"]["sha256"] = "d" * 64
            if change == "empty": plan["inventories"]["owner0"]["count"] = 0
            if change == "oversized":
                plan["inventories"]["owner0"]["count"] = 513
                args["inventories"] = plan["inventories"]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_exact_cumulative_reservation_cannot_reset_across_cohorts(self):
        for change in ("variant", "baseline", "coverage", "contracts", "fuzz", "limit"):
            plan, args = self.fixture(); plan = copy.deepcopy(plan)
            if change in ("coverage", "contracts", "fuzz"): plan["remaining_checks"][change] = 0
            if change == "variant": plan["cumulative_reservation"]["mutation_variants"] -= 1
            if change == "baseline": plan["cumulative_reservation"]["mutation_baselines"] -= 1
            if change == "limit": args["global_limits"]["points"] -= 1
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_focused_binding_cannot_select_another_owner_output_or_all(self):
        plan, _ = self.fixture()
        for subject, output in (("all", "/tmp/owner0"), ("owner1", "/tmp/owner0"),
                                ("owner0", "/tmp/retry")):
            with self.subTest(subject=subject, output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.cohort_binding(plan, "owner0", subject=subject, output_dir=output)

    def test_union_rejects_missing_duplicate_unknown_stale_and_incomplete_receipts(self):
        for change in ("missing", "duplicate", "unknown", "candidate", "source", "tool", "recipe", "inventory",
                       "baseline", "failed_gate", "pending", "partial_inventory", "wrong_path", "wrong_lane"):
            plan, args = self.fixture(); receipts = copy.deepcopy(self.receipts(plan, args["subjects"]))
            row = receipts[0]
            if change == "missing": receipts.pop()
            if change == "duplicate": receipts[1] = copy.deepcopy(row)
            if change == "unknown": row["core_cohort_binding"]["cohort_id"] = "unknown"
            if change == "candidate": row["core_cohort_binding"]["candidate_sha256"] = "d" * 64
            if change == "source": row["source_sha256"]["Cargo.toml"] = "d" * 64
            if change == "tool": row["tools_sha256"]["cargo-mutants"] = "d" * 64
            if change == "recipe": row["core_cohort_binding"]["recipe_sha256"] = "d" * 64
            if change == "inventory": row["mutation_reviewed_inventories"]["owner0"]["sha256"] = "d" * 64
            if change == "baseline": row["mutation"]["owner0"]["baseline_tests_executed"] = 12
            if change == "failed_gate": row["mutation"]["owner0"]["gate_passed"] = False
            if change == "pending": row["core_observation_pending_invocations"] = ["owner0:baseline"]
            if change == "partial_inventory": row["mutation_generated_variants_selected_total"] -= 1
            if change == "wrong_path": row["owned_output"] = "/tmp/retry"
            if change == "wrong_lane": row["selected_lane"] = "all"
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args["subjects"])


if __name__ == "__main__":
    unittest.main()
