"""Finite pure controls for coordinator partition and receipt-union admission."""
import copy
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "core_analysis_cohorts", Path(__file__).resolve().parents[1] / "core-analysis-cohorts.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

S157_SUBJECTS = {
    'principal': 'crates/time-core/src/principal.rs',
    'chart_principal': 'crates/time-core/src/principal/chart.rs',
    'chart_batch': 'crates/time-core/src/chart_batch.rs',
}
S158_SUBJECTS = {
    'combination': 'crates/time-core/src/combination.rs',
    'combination_admission': 'crates/time-core/src/combination/admission.rs',
    'combination_lifecycle': 'crates/time-core/src/combination/lifecycle.rs',
    'chart': 'crates/time-core/src/regression/chart.rs',
    'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
    'chart_combination_roundoff': 'crates/time-core/src/combination/chart/roundoff.rs',
    'chart_combination_receiving': 'crates/time-core/src/combination/chart/receiving.rs',
    'chart_combination_reduction': 'crates/time-core/src/combination/chart/reduction.rs',
    'chart_combination_lifecycle': 'crates/time-core/src/combination/lifecycle/chart.rs',
}


class CohortPlanTests(unittest.TestCase):
    """Synthetic identities test admission mechanics, never a product candidate."""

    def test_three_owner_schema_eleven_and_literal_reservation(self):
        plan, args = self.fixture()
        subjects = dict(S157_SUBJECTS)
        inventories = {name: {"count": 100 + index, "sha256": str(index) * 64}
                       for index, name in enumerate(subjects)}
        plan['inventories'] = args['inventories'] = inventories
        args['subjects'] = subjects
        plan['cohorts'] = [{"id": name, "subject": name, "output_dir": "/tmp/" + name}
                           for name in subjects]
        # 303 variants + three baselines, one coverage and one contracts:
        # fits=306*2+2+3, points=306*7+7+4.
        plan['cumulative_reservation'] = {
            'mutation_variants': 303, 'mutation_baselines': 3,
            'mutation_package_invocations': 306,
            'source_work': {'fits': 617, 'points': 2153},
        }
        args['global_limits'] = {'fits': 617, 'points': 2153}
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted['reservation'], plan['cumulative_reservation'])
        union = MODULE.reconcile_receipts(plan, self.receipts(plan, subjects), subjects=subjects)
        self.assertEqual((union['focused_receipts'], union['baselines'], union['generated_variants']),
                         (3, 3, 303))

    def test_historical_s155_single_owner_schema_eleven_rejects_as_current(self):
        plan, args = self.fixture()
        plan['schema_version'] = 11
        args['subjects'] = {'principal': 'src/principal.rs'}
        inventories = {'principal': {'count': 300, 'sha256': '0' * 64}}
        plan['inventories'] = inventories
        args['inventories'] = inventories
        plan['cohorts'] = [{'id': 'principal', 'subject': 'principal', 'output_dir': '/tmp/principal'}]
        # 300 variants + one baseline, followed by one coverage and one contracts
        # reservation: fits=301*2+2+3, points=301*7+7+4.
        plan['cumulative_reservation'] = {
            'mutation_variants': 300, 'mutation_baselines': 1,
            'mutation_package_invocations': 301,
            'source_work': {'fits': 607, 'points': 2118},
        }
        args['global_limits'] = {'fits': 607, 'points': 2118}
        with self.assertRaisesRegex(MODULE.Incomplete, 'exactly three changed critical owners'):
            MODULE.admit_plan(plan, **args)

    def fixture(self, subjects=None):
        subjects = dict(S157_SUBJECTS if subjects is None else subjects)
        product = {"Cargo.toml": "a" * 64,
                   **{path: str(index) * 64 for index, path in enumerate(subjects.values())}}
        tools = {"scripts/wrapper.py": "b" * 64}
        inventories = {name: {"count": 100 + index, "sha256": str(index) * 64}
                       for index, name in enumerate(subjects)}
        package = {"fits": 2, "points": 7}
        checks = {"coverage": dict(package), "contracts": {"fits": 3, "points": 4}}
        plan = {"schema_version": 11, "status": "admitted-exact-source-and-cohorts",
                "candidate_sha256": MODULE.canonical_sha256(product),
                "product_inputs_sha256": product, "tool_inputs_sha256": tools,
                "tool_payloads_sha256": {"cargo-mutants": "e" * 64},
                "recipe_sha256": "c" * 64, "inventories": inventories,
                "cohorts": [{"id": name, "subject": name, "output_dir": "/tmp/" + name}
                            for name in subjects], "remaining_checks": {"coverage": 1, "contracts": 1},
                "baseline_tests": 13, "cumulative_reservation": None}
        plan["cumulative_reservation"] = MODULE.plan_reservation(plan, package, checks)
        arguments = {"subjects": subjects, "product_inputs": product, "tool_inputs": tools,
                     "recipe_sha256": "c" * 64, "inventories": inventories, "baseline_tests": 13,
                     "package_budget": package, "check_budgets": checks,
                     "global_limits": dict(plan["cumulative_reservation"]["source_work"])}
        return plan, arguments

    def test_registered_nine_owner_schema_eleven_admits_complete_union(self):
        plan, args = self.fixture(S158_SUBJECTS)
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted['reservation'], {
            'mutation_variants': 936, 'mutation_baselines': 9,
            'mutation_package_invocations': 945,
            'source_work': {'fits': 1895, 'points': 6626}})
        union = MODULE.reconcile_receipts(plan, self.receipts(plan, args['subjects']),
                                          subjects=args['subjects'])
        self.assertEqual((union['focused_receipts'], union['baselines'], union['generated_variants']),
                         (9, 9, 936))
        self.assertFalse(union['critical_manifest_complete_selection'])

    def test_nine_owner_aggregate_4608_and_per_cohort_512_are_independent(self):
        plan, args = self.fixture(S158_SUBJECTS)
        for row in plan['inventories'].values():
            row['count'] = 512
        plan['cumulative_reservation'] = MODULE.plan_reservation(
            plan, args['package_budget'], args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted['reservation']['mutation_variants'], 4608)
        self.assertEqual(admitted['reservation']['mutation_baselines'], 9)
        self.assertEqual(admitted['reservation']['mutation_package_invocations'], 4617)
        plan['inventories']['combination']['count'] = 513
        with self.assertRaisesRegex(MODULE.Incomplete, 'focused 512 cap'):
            MODULE.admit_plan(plan, **args)

    def test_registered_profiles_reject_rebound_arbitrary_or_unbound_source_owners(self):
        for profile in (S157_SUBJECTS, S158_SUBJECTS):
            for change in ('unknown_name', 'wrong_path', 'duplicate_path', 'missing_product'):
                plan, args = self.fixture(profile)
                names = list(args['subjects'])
                if change == 'unknown_name':
                    args['subjects']['unknown'] = args['subjects'].pop(names[0])
                elif change == 'wrong_path':
                    args['subjects'][names[0]] = 'src/unregistered.rs'
                elif change == 'duplicate_path':
                    args['subjects'][names[0]] = args['subjects'][names[1]]
                else:
                    del args['product_inputs'][args['subjects'][names[0]]]
                    plan['candidate_sha256'] = MODULE.canonical_sha256(args['product_inputs'])
                with self.subTest(profile=len(profile), change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_plan(plan, **args)

    def test_nine_owner_missing_duplicate_malformed_or_overbudget_plan_rejects(self):
        for change in ('missing', 'duplicate', 'unknown', 'output', 'empty_inventory',
                       'malformed_inventory', 'baseline', 'reservation', 'limit'):
            plan, args = self.fixture(S158_SUBJECTS)
            if change == 'missing': plan['cohorts'].pop()
            if change == 'duplicate': plan['cohorts'][1] = copy.deepcopy(plan['cohorts'][0])
            if change == 'unknown': plan['cohorts'][0]['subject'] = 'unknown'
            if change == 'output': plan['cohorts'][1]['output_dir'] = plan['cohorts'][0]['output_dir']
            if change == 'empty_inventory': plan['inventories']['combination']['count'] = 0
            if change == 'malformed_inventory': plan['inventories']['combination'] = []
            if change == 'baseline': plan['baseline_tests'] = args['baseline_tests'] = 0
            if change == 'reservation': plan['cumulative_reservation']['mutation_baselines'] = 8
            if change == 'limit': args['global_limits']['fits'] -= 1
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_nine_owner_union_requires_every_independent_nonempty_baseline_and_gate(self):
        plan, args = self.fixture(S158_SUBJECTS)
        for change in ('missing', 'duplicate', 'zero_baseline', 'missing_baseline', 'failed_gate',
                       'pending', 'partial_inventory', 'source', 'tool', 'output'):
            receipts = copy.deepcopy(self.receipts(plan, args['subjects']))
            row = receipts[0]
            if change == 'missing': receipts.pop()
            if change == 'duplicate': receipts[1] = copy.deepcopy(row)
            if change == 'zero_baseline': row['mutation']['combination']['baseline_tests_executed'] = 0
            if change == 'missing_baseline': row['mutation_baselines_observed'] = 0
            if change == 'failed_gate': row['mutation']['combination']['gate_passed'] = False
            if change == 'pending': row['core_observation_pending_invocations'] = ['combination:baseline']
            if change == 'partial_inventory': row['mutation_generated_variants_selected_total'] -= 1
            if change == 'source': row['source_sha256']['Cargo.toml'] = 'd' * 64
            if change == 'tool': row['tools_sha256']['cargo-mutants'] = 'd' * 64
            if change == 'output': row['owned_output'] = '/tmp/reused'
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args['subjects'])

    def test_nine_owner_reviewed_2682_field_cap_preserves_three_owner_2214_cap(self):
        plan, args = self.fixture(S158_SUBJECTS)
        package = {f'metric{index}': 1 for index in range(2682)}
        args['package_budget'] = package
        args['check_budgets'] = {'coverage': dict(package), 'contracts': dict(package)}
        plan['cumulative_reservation'] = MODULE.plan_reservation(
            plan, package, args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(len(admitted['reservation']['source_work']), 2682)
        self.assertEqual(set(admitted['reservation']['source_work'].values()), {947})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.plan_reservation(plan, {**package, 'metric2682': 1}, args['check_budgets'])
        old, old_args = self.fixture()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.plan_reservation(old, {f'metric{index}': 1 for index in range(2215)},
                                    old_args['check_budgets'])

    def test_binding_and_union_cannot_bypass_registered_partition_validation(self):
        for change in ('missing', 'duplicate', 'output', 'unsupported_inventory'):
            plan, args = self.fixture(S158_SUBJECTS)
            receipts = self.receipts(plan, args['subjects'])
            if change == 'missing': plan['cohorts'].pop()
            if change == 'duplicate': plan['cohorts'][1] = copy.deepcopy(plan['cohorts'][0])
            if change == 'output': plan['cohorts'][1]['output_dir'] = plan['cohorts'][0]['output_dir']
            if change == 'unsupported_inventory':
                plan['inventories']['unknown'] = plan['inventories'].pop('combination')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.cohort_binding(plan, 'combination', subject='combination',
                                      output_dir='/tmp/combination')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args['subjects'])

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

    def test_three_owner_checkpoint_has_three_baselines_and_no_manifest_claim(self):
        plan, args = self.fixture()
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted["reservation"]["mutation_variants"], 303)
        self.assertEqual(admitted["reservation"]["mutation_baselines"], 3)
        self.assertEqual(admitted["reservation"]["source_work"], {"fits": 617, "points": 2153})
        union = MODULE.reconcile_receipts(plan, self.receipts(plan, args["subjects"]), subjects=args["subjects"])
        self.assertEqual(union["focused_receipts"], 3)
        self.assertEqual(union["generated_variants"], 303)
        self.assertFalse(union["critical_manifest_complete_selection"])
        self.assertEqual(union['scope'],
                         'Three retained-chart critical owner mutation receipts; '
                         'coverage/contracts and native Windows remain separate.')

    def test_scope_requires_exactly_three_distinct_owners(self):
        for change in (0, 1, 2, 4, "duplicate_file"):
            plan, args = self.fixture()
            if change == "duplicate_file":
                args['subjects']['chart_principal'] = args['subjects']['principal']
            else:
                args['subjects'] = {f'owner{index}': f'src/owner{index}.rs'
                                    for index in range(change)}
            with self.subTest(change=change), self.assertRaisesRegex(
                    MODULE.Incomplete, "exactly three changed critical owners"):
                MODULE.admit_plan(plan, **args)

    def test_partition_requires_exactly_three_owners(self):
        for count in (0, 1, 2, 4):
            plan, args = self.fixture()
            plan['cohorts'] = [
                {'id': f'owner{index}', 'subject': f'owner{index}', 'output_dir': f'/tmp/owner{index}'}
                for index in range(count)]
            with self.subTest(count=count), self.assertRaisesRegex(
                    MODULE.Incomplete, 'three retained-chart owners'):
                MODULE.admit_plan(plan, **args)

    def test_current_schema_eleven_accepts_and_predecessor_ten_rejects(self):
        plan, args = self.fixture()
        plan["schema_version"] = 11
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted["reservation"]["mutation_baselines"], 3)
        plan["schema_version"] = 10
        with self.assertRaisesRegex(MODULE.Incomplete, "final-source/inventory binding"):
            MODULE.admit_plan(plan, **args)

    def test_three_cohort_union_reaches_independent_listing_limit(self):
        plan, args = self.fixture()
        for row, count in zip(plan['inventories'].values(), (510, 1, 1), strict=True):
            row['count'] = count
        plan['cumulative_reservation'] = MODULE.plan_reservation(
            plan, args['package_budget'], args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted['reservation']['mutation_variants'], 512)
        self.assertEqual(admitted['reservation']['mutation_baselines'], 3)
        self.assertEqual(admitted['reservation']['mutation_package_invocations'], 515)
        self.assertEqual(admitted['reservation']['source_work'], {'fits': 1035, 'points': 3616})
        plan['inventories']['principal']['count'] = 511
        with self.assertRaisesRegex(MODULE.Incomplete, 'listing ceiling'):
            MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets'])
        with self.assertRaisesRegex(MODULE.Incomplete, 'listing ceiling'):
            MODULE.admit_plan(plan, **args)

    def test_reviewed_2214_field_width_accepts_exact_limit_and_rejects_next(self):
        exact = {f"metric{index}": 1 for index in range(2214)}
        self.assertEqual(MODULE._budget(exact), exact)
        with self.assertRaises(MODULE.Incomplete):
            MODULE._budget({**exact, 'metric2214': 1})

    def test_checkpoint_totals_include_exactly_one_spent_contract_and_one_retry(self):
        expected = {"coverage": 1, "contracts": 2}
        self.assertEqual(MODULE.historical_s148_remaining_checks(expected), expected)
        for value in ({"coverage": 1, "contracts": 1}, {"coverage": 1, "contracts": 3},
                      {"coverage": 2, "contracts": 2}, {"coverage": True, "contracts": 2},
                      {"coverage": 1, "contracts": 2.0}, {**expected, "mutation": 1}):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s148_remaining_checks(value)

    def test_current_checkpoint_requires_one_fresh_coverage_and_contracts_without_retry(self):
        expected = {"coverage": 1, "contracts": 1}
        self.assertEqual(MODULE._remaining_checks(expected), expected)
        for value in ({"coverage": 1, "contracts": 2}, {"coverage": 1, "contracts": 0},
                      {"coverage": 2, "contracts": 1}, {"coverage": True, "contracts": 1},
                      {"coverage": 1, "contracts": 1.0}, {**expected, "mutation": 1}):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE._remaining_checks(value)

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
                plan["inventories"]["principal"]["count"] = True
            elif change == "baseline":
                receipts[0]["mutation"]["principal"]["baseline_tests_executed"] = True
            elif change == "required":
                receipts[0]["mutation_baselines_required"] = True
            elif change == "observed":
                receipts[0]["mutation_baselines_observed"] = True
            else:
                receipts[0]["mutation_generated_variants_selected_total"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args["subjects"])

    def test_unknown_missing_and_pending_plan_fields_reject(self):
        for change in ("unknown", "missing", "pending", "schema_bool", "historical_plan", "historical_admission", "historical_s148", "historical_s149", "historical_s150", "historical_s151", "historical_s154b", "historical_s154c"):
            plan, args = self.fixture()
            if change == "unknown": plan["extra"] = 0
            if change == "missing": del plan["inventories"]
            if change == "pending": plan["status"] = "PENDING"
            if change == "schema_bool": plan["schema_version"] = True
            if change == "historical_plan": plan["schema_version"] = 1
            if change == "historical_admission": plan["schema_version"] = 4
            if change == "historical_s148": plan["schema_version"] = 5
            if change == "historical_s149": plan["schema_version"] = 6
            if change == "historical_s150": plan["schema_version"] = 7
            if change == "historical_s151": plan["schema_version"] = 8
            if change == "historical_s154b": plan["schema_version"] = 9
            if change == "historical_s154c": plan["schema_version"] = 10
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
            if change == "relative": plan["cohorts"][0]["output_dir"] = "tmp/principal"
            if change == "traversal": plan["cohorts"][0]["output_dir"] = "/tmp/../principal"
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
            if change == "inventory": plan["inventories"]["principal"]["sha256"] = "d" * 64
            if change == "empty": plan["inventories"]["principal"]["count"] = 0
            if change == "oversized":
                plan["inventories"]["principal"]["count"] = 513
                args["inventories"] = plan["inventories"]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_inventory_namespace_rejects_missing_or_extra_owners(self):
        for change in ('missing', 'extra'):
            plan, args = self.fixture()
            if change == 'missing':
                del plan['inventories']['chart_batch']
            else:
                plan['inventories']['owner3'] = {'count': 1, 'sha256': '3' * 64}
            with self.subTest(change=change), self.assertRaisesRegex(
                    MODULE.Incomplete, 'exactly three changed critical owners'):
                MODULE.admit_plan(plan, **args)

    def test_source_work_namespaces_cannot_omit_a_budget_field(self):
        for change in ('package', 'coverage', 'contracts', 'limits', 'reservation'):
            plan, args = self.fixture()
            if change == 'package':
                del args['package_budget']['points']
            elif change in ('coverage', 'contracts'):
                del args['check_budgets'][change]['points']
            elif change == 'limits':
                del args['global_limits']['points']
            else:
                del plan['cumulative_reservation']['source_work']['points']
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
        for subject, output in (("all", "/tmp/principal"), ("chart_principal", "/tmp/principal"),
                                ("principal", "/tmp/retry")):
            with self.subTest(subject=subject, output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.cohort_binding(plan, "principal", subject=subject, output_dir=output)

    def test_union_rejects_missing_duplicate_unknown_stale_and_incomplete_receipts(self):
        for change in ("missing", "duplicate", "extra", "unknown", "candidate", "source", "tool", "recipe", "inventory",
                       "baseline", "failed_gate", "pending", "partial_inventory", "wrong_path", "wrong_lane"):
            plan, args = self.fixture(); receipts = copy.deepcopy(self.receipts(plan, args["subjects"]))
            row = receipts[0]
            if change == "missing": receipts.pop()
            if change == "duplicate": receipts[1] = copy.deepcopy(row)
            if change == "extra": receipts.append(copy.deepcopy(row))
            if change == "unknown": row["core_cohort_binding"]["cohort_id"] = "unknown"
            if change == "candidate": row["core_cohort_binding"]["candidate_sha256"] = "d" * 64
            if change == "source": row["source_sha256"]["Cargo.toml"] = "d" * 64
            if change == "tool": row["tools_sha256"]["cargo-mutants"] = "d" * 64
            if change == "recipe": row["core_cohort_binding"]["recipe_sha256"] = "d" * 64
            if change == "inventory": row["mutation_reviewed_inventories"]["principal"]["sha256"] = "d" * 64
            if change == "baseline": row["mutation"]["principal"]["baseline_tests_executed"] = 12
            if change == "failed_gate": row["mutation"]["principal"]["gate_passed"] = False
            if change == "pending": row["core_observation_pending_invocations"] = ["principal:baseline"]
            if change == "partial_inventory": row["mutation_generated_variants_selected_total"] -= 1
            if change == "wrong_path": row["owned_output"] = "/tmp/retry"
            if change == "wrong_lane": row["selected_lane"] = "all"
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args["subjects"])


class ReferenceInputCohortTests(unittest.TestCase):
    """One exact new owner cannot inherit another profile's scope or allowance."""

    SUBJECTS = {'reference_input': 'crates/time-core/src/reference_input.rs'}

    def fixture(self):
        plan, args = CohortPlanTests.fixture(self, self.SUBJECTS)
        plan['baseline_tests'] = args['baseline_tests'] = 313
        return plan, args

    def receipts(self, plan):
        receipts = CohortPlanTests.receipts(self, plan, self.SUBJECTS)
        receipts[0]['mutation']['reference_input']['baseline_tests_executed'] = 313
        return receipts

    def test_reference_input_schema_eleven_and_literal_single_baseline_union(self):
        plan, args = self.fixture()
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(plan['schema_version'], 11)
        self.assertEqual(admitted['reservation'], {
            'mutation_variants': 100, 'mutation_baselines': 1,
            'mutation_package_invocations': 101,
            'source_work': {'fits': 207, 'points': 718}})
        binding = MODULE.cohort_binding(plan, 'reference_input', subject='reference_input',
                                        output_dir='/tmp/reference_input')
        self.assertEqual(binding['candidate_sha256'], plan['candidate_sha256'])
        union = MODULE.reconcile_receipts(plan, self.receipts(plan), subjects=self.SUBJECTS)
        self.assertEqual((union['focused_receipts'], union['baselines'], union['generated_variants']),
                         (1, 1, 100))
        self.assertFalse(union['critical_manifest_complete_selection'])
        self.assertEqual(union['scope'], 'One reference-input critical owner mutation receipt; '
                         'coverage/contracts and native Windows remain separate.')

    def test_singleton_512_inventory_cap_and_one_baseline_are_independent(self):
        plan, args = self.fixture()
        plan['inventories']['reference_input']['count'] = 1
        self.assertEqual(MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets']), {
            'mutation_variants': 1, 'mutation_baselines': 1,
            'mutation_package_invocations': 2,
            'source_work': {'fits': 9, 'points': 25}})
        plan['inventories']['reference_input']['count'] = 512
        plan['cumulative_reservation'] = MODULE.plan_reservation(
            plan, args['package_budget'], args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        self.assertEqual(MODULE.admit_plan(plan, **args)['reservation'], {
            'mutation_variants': 512, 'mutation_baselines': 1,
            'mutation_package_invocations': 513,
            'source_work': {'fits': 1031, 'points': 3602}})
        for count in (0, 513, True, 512.0):
            with self.subTest(count=count), self.assertRaises(MODULE.Incomplete):
                plan['inventories']['reference_input']['count'] = count
                MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets'])

    def test_singleton_2700_fields_do_not_widen_historical_profile_caps(self):
        plan, args = self.fixture()
        package = {f'metric{index}': 1 for index in range(2700)}
        args['package_budget'] = package
        args['check_budgets'] = {'coverage': dict(package), 'contracts': dict(package)}
        plan['cumulative_reservation'] = MODULE.plan_reservation(plan, package, args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(len(admitted['reservation']['source_work']), 2700)
        self.assertEqual(set(admitted['reservation']['source_work'].values()), {103})
        # The current receiver extends this same owner, while other owners'
        # historical ceilings remain separate. Its exact namespace still comes
        # from the independently supplied package/check recipe.
        oversized = {f'metric{index}': 1 for index in range(2766)}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.plan_reservation(plan, oversized,
                                    {'coverage': dict(oversized), 'contracts': dict(oversized)})
        for owners, width in ((S157_SUBJECTS, 2214), (S158_SUBJECTS, 2682)):
            old, old_args = CohortPlanTests.fixture(self, owners)
            with self.subTest(owners=len(owners)), self.assertRaises(MODULE.Incomplete):
                MODULE.plan_reservation(old, {f'metric{index}': 1 for index in range(width + 1)},
                                        old_args['check_budgets'])

    def test_current_receiver_2765_fields_still_bind_exact_recipe_and_limits(self):
        plan, args = self.fixture()
        plan['baseline_tests'] = args['baseline_tests'] = 325
        package = {f'metric{index}': 1 for index in range(2765)}
        args['package_budget'] = package
        args['check_budgets'] = {'coverage': dict(package), 'contracts': dict(package)}
        plan['cumulative_reservation'] = MODULE.plan_reservation(plan, package, args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        self.assertEqual(len(MODULE.admit_plan(plan, **args)['reservation']['source_work']), 2765)
        self.assertEqual(set(plan['cumulative_reservation']['source_work'].values()), {103})
        for change in ('missing', 'renamed', 'undercharged', 'limit', 'boolean'):
            bad, options = copy.deepcopy(plan), copy.deepcopy(args)
            work = bad['cumulative_reservation']['source_work']
            if change == 'missing': del work['metric0']
            elif change == 'renamed': work['unreviewed'] = work.pop('metric0')
            elif change == 'undercharged': work['metric0'] -= 1
            elif change == 'limit': options['global_limits']['metric0'] -= 1
            else: work['metric0'] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(bad, **options)

    def test_singleton_registration_partition_and_all_entrypoints_remain_closed(self):
        for change in ('missing_inventory', 'extra_inventory', 'other_singleton', 'missing_partition',
                       'duplicate_partition', 'wrong_subject', 'wrong_id', 'wrong_path', 'missing_source'):
            plan, args = self.fixture()
            receipts = self.receipts(plan)
            if change == 'missing_inventory': plan['inventories'].clear()
            if change == 'extra_inventory': plan['inventories']['other'] = {'count': 1, 'sha256': 'd' * 64}
            if change == 'other_singleton':
                plan['inventories']['principal'] = plan['inventories'].pop('reference_input')
            if change == 'missing_partition': plan['cohorts'].clear()
            if change == 'duplicate_partition': plan['cohorts'].append(copy.deepcopy(plan['cohorts'][0]))
            if change == 'wrong_subject': plan['cohorts'][0]['subject'] = 'other'
            if change == 'wrong_id': plan['cohorts'][0]['id'] = '../other'
            if change == 'wrong_path': args['subjects'] = {'reference_input': 'src/other.rs'}
            if change == 'missing_source':
                del plan['product_inputs_sha256'][self.SUBJECTS['reference_input']]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)
            if change not in ('wrong_path', 'wrong_id'):
                with self.subTest(binding=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.cohort_binding(plan, 'reference_input', subject='reference_input',
                                          output_dir='/tmp/reference_input')
            with self.subTest(union=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=args['subjects'])

    def test_singleton_candidate_baseline_budget_and_remaining_checks_cannot_drift(self):
        for change in ('candidate', 'tool', 'recipe', 'baseline', 'baseline_bool', 'extra_baseline',
                       'reservation', 'limit', 'checks', 'budget_namespace'):
            plan, args = self.fixture()
            if change == 'candidate': plan['candidate_sha256'] = 'd' * 64
            if change == 'tool':
                plan['tool_inputs_sha256'] = {**plan['tool_inputs_sha256'], 'scripts/wrapper.py': 'd' * 64}
            if change == 'recipe': args['recipe_sha256'] = 'd' * 64
            if change == 'baseline': plan['baseline_tests'] = 312
            if change == 'baseline_bool': plan['baseline_tests'] = True
            if change == 'extra_baseline': plan['cumulative_reservation']['mutation_baselines'] = 2
            if change == 'reservation': plan['cumulative_reservation']['source_work']['fits'] -= 1
            if change == 'limit': args['global_limits']['fits'] -= 1
            if change == 'checks': plan['remaining_checks']['coverage'] = 2
            if change == 'budget_namespace': del args['check_budgets']['coverage']['points']
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_singleton_union_rejects_incomplete_stale_or_failed_receipts(self):
        for change in ('missing', 'duplicate', 'failed', 'wrong_lane', 'baseline', 'baseline_bool',
                       'missing_baseline', 'extra_baseline', 'gate', 'pending', 'inventory', 'source',
                       'tool', 'binding', 'output'):
            plan, _args = self.fixture()
            receipts = self.receipts(plan)
            row = receipts[0]
            if change == 'missing': receipts.clear()
            if change == 'duplicate': receipts.append(copy.deepcopy(row))
            if change == 'failed': row['status'] = 'incomplete'
            if change == 'wrong_lane': row['selected_lane'] = 'all'
            if change == 'baseline': row['mutation']['reference_input']['baseline_tests_executed'] = 312
            if change == 'baseline_bool': row['mutation']['reference_input']['baseline_tests_executed'] = True
            if change == 'missing_baseline': row['mutation_baselines_observed'] = 0
            if change == 'extra_baseline': row['mutation_baselines_observed'] = 2
            if change == 'gate': row['mutation']['reference_input']['gate_passed'] = False
            if change == 'pending': row['core_observation_pending_invocations'] = ['reference_input:baseline']
            if change == 'inventory': row['mutation_generated_variants_selected_total'] -= 1
            if change == 'source': row['source_sha256']['Cargo.toml'] = 'd' * 64
            if change == 'tool': row['tools_sha256']['cargo-mutants'] = 'd' * 64
            if change == 'binding': row['core_cohort_binding']['candidate_sha256'] = 'd' * 64
            if change == 'output': row['owned_output'] = '/tmp/stale'
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=self.SUBJECTS)


class ChartCompositionCohortTests(unittest.TestCase):
    """The changed chart trio has its own closed scope and independent baselines."""

    SUBJECTS = {'chart': 'crates/time-core/src/regression/chart.rs',
                'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
                'chart_batch': 'crates/time-core/src/chart_batch.rs'}

    def fixture(self):
        plan, args = CohortPlanTests.fixture(self, self.SUBJECTS)
        plan['baseline_tests'] = args['baseline_tests'] = 317
        return plan, args

    def receipts(self, plan):
        rows = CohortPlanTests.receipts(self, plan, self.SUBJECTS)
        for row in rows:
            for outcome in row['mutation'].values():
                outcome['baseline_tests_executed'] = 317
        return rows

    def test_exact_chart_trio_has_three_independent_baselines_and_complete_union(self):
        plan, args = self.fixture()
        admitted = MODULE.admit_plan(plan, **args)
        self.assertEqual(admitted['reservation'], {
            'mutation_variants': 303, 'mutation_baselines': 3,
            'mutation_package_invocations': 306, 'source_work': {'fits': 617, 'points': 2153}})
        union = MODULE.reconcile_receipts(plan, self.receipts(plan), subjects=self.SUBJECTS)
        self.assertEqual((union['focused_receipts'], union['generated_variants'], union['baselines']), (3, 303, 3))
        self.assertEqual(union['critical_owners'], sorted(self.SUBJECTS))
        self.assertFalse(union['critical_manifest_complete_selection'])
        self.assertEqual(union['scope'], 'Three chart-composition critical owner mutation receipts; '
                         'coverage/contracts and native Windows remain separate.')

    def test_chart_trio_1536_total_does_not_widen_old_three_owner_512_cap(self):
        plan, args = self.fixture()
        for row in plan['inventories'].values():
            row['count'] = 512
        plan['cumulative_reservation'] = MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        self.assertEqual(MODULE.admit_plan(plan, **args)['reservation'], {
            'mutation_variants': 1536, 'mutation_baselines': 3,
            'mutation_package_invocations': 1539, 'source_work': {'fits': 3083, 'points': 10784}})
        for value in (0, 513, True, 512.0):
            with self.subTest(count=value), self.assertRaises(MODULE.Incomplete):
                plan['inventories']['chart']['count'] = value
                MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets'])
        old, old_args = CohortPlanTests.fixture(self, S157_SUBJECTS)
        for row in old['inventories'].values():
            row['count'] = 200
        with self.assertRaises(MODULE.Incomplete):
            MODULE.plan_reservation(old, old_args['package_budget'], old_args['check_budgets'])

    def test_chart_trio_registration_partition_and_paths_are_exact(self):
        for change in ('missing_inventory', 'extra_inventory', 'foreign_owner', 'missing_cohort',
                       'duplicate_cohort', 'wrong_subject', 'wrong_path', 'missing_source'):
            plan, args = self.fixture()
            if change == 'missing_inventory': del plan['inventories']['chart']
            if change == 'extra_inventory': plan['inventories']['foreign'] = {'count': 1, 'sha256': 'd' * 64}
            if change == 'foreign_owner': plan['inventories']['principal'] = plan['inventories'].pop('chart')
            if change == 'missing_cohort': plan['cohorts'].pop()
            if change == 'duplicate_cohort': plan['cohorts'].append(copy.deepcopy(plan['cohorts'][0]))
            if change == 'wrong_subject': plan['cohorts'][0]['subject'] = 'principal'
            if change == 'wrong_path': args['subjects'] = {**self.SUBJECTS, 'chart': 'src/other.rs'}
            if change == 'missing_source': del plan['product_inputs_sha256'][self.SUBJECTS['chart']]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_composition_2740_field_cap_preserves_every_historical_cap(self):
        plan, args = self.fixture()
        package = {f'metric{index}': 1 for index in range(2740)}
        args['package_budget'] = package
        args['check_budgets'] = {'coverage': dict(package), 'contracts': dict(package)}
        plan['cumulative_reservation'] = MODULE.plan_reservation(plan, package, args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        self.assertEqual(len(MODULE.admit_plan(plan, **args)['reservation']['source_work']), 2740)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.plan_reservation(plan, {**package, 'metric2740': 1}, args['check_budgets'])
        for owners, width in ((S157_SUBJECTS, 2214), (S158_SUBJECTS, 2682),
                              (ReferenceInputCohortTests.SUBJECTS, 2765)):
            old, old_args = CohortPlanTests.fixture(self, owners)
            with self.subTest(owners=len(owners)), self.assertRaises(MODULE.Incomplete):
                MODULE.plan_reservation(old, {f'metric{index}': 1 for index in range(width + 1)},
                                        old_args['check_budgets'])

    def test_chart_trio_bindings_baselines_and_complete_budget_cannot_drift(self):
        for change in ('candidate', 'tool', 'recipe', 'baseline', 'baseline_bool', 'extra_baseline',
                       'reservation', 'limit', 'checks', 'budget_namespace'):
            plan, args = self.fixture()
            if change == 'candidate': plan['candidate_sha256'] = 'd' * 64
            if change == 'tool': args['tool_inputs'] = {'scripts/wrapper.py': 'd' * 64}
            if change == 'recipe': args['recipe_sha256'] = 'd' * 64
            if change == 'baseline': plan['baseline_tests'] = 313
            if change == 'baseline_bool': plan['baseline_tests'] = True
            if change == 'extra_baseline': plan['cumulative_reservation']['mutation_baselines'] = 4
            if change == 'reservation': plan['cumulative_reservation']['source_work']['fits'] -= 1
            if change == 'limit': args['global_limits']['fits'] -= 1
            if change == 'checks': plan['remaining_checks']['coverage'] = 2
            if change == 'budget_namespace': del args['check_budgets']['coverage']['points']
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_chart_trio_union_rejects_partial_duplicate_or_previous_checkpoint_receipts(self):
        for change in ('missing', 'duplicate', 'failed', 'baseline', 'gate', 'pending', 'inventory',
                       'source', 'tool', 'binding', 'output'):
            plan, _args = self.fixture()
            receipts = self.receipts(plan)
            row = receipts[0]
            subject = plan['cohorts'][0]['subject']
            if change == 'missing': receipts.pop()
            if change == 'duplicate': receipts[1] = copy.deepcopy(row)
            if change == 'failed': row['status'] = 'incomplete'
            if change == 'baseline': row['mutation'][subject]['baseline_tests_executed'] = 313
            if change == 'gate': row['mutation'][subject]['gate_passed'] = False
            if change == 'pending': row['core_observation_pending_invocations'] = ['chart:baseline']
            if change == 'inventory': row['mutation_generated_variants_selected_total'] -= 1
            if change == 'source': row['source_sha256']['Cargo.toml'] = 'd' * 64
            if change == 'tool': row['tools_sha256']['cargo-mutants'] = 'd' * 64
            if change == 'binding': row['core_cohort_binding']['plan_sha256'] = 'd' * 64
            if change == 'output': row['owned_output'] = '/tmp/previous-checkpoint'
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, receipts, subjects=self.SUBJECTS)


class SettledRateCohortTests(unittest.TestCase):
    """The exact changed four-owner set has four independent baselines."""

    SUBJECTS = {
        'reference_input': 'crates/time-core/src/reference_input.rs',
        'rate_delivery': 'crates/time-core/src/reference_input/rate_delivery.rs',
        'chart': 'crates/time-core/src/regression/chart.rs',
        'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
    }

    def fixture(self):
        return CohortPlanTests.fixture(self, self.SUBJECTS)

    def test_exact_four_owners_reserve_each_baseline_and_reconcile_union(self):
        plan, args = self.fixture()
        self.assertEqual(MODULE.admit_plan(plan, **args)['reservation'], {
            'mutation_variants': 406, 'mutation_baselines': 4,
            'mutation_package_invocations': 410, 'source_work': {'fits': 825, 'points': 2881}})
        rows = CohortPlanTests.receipts(self, plan, self.SUBJECTS)
        union = MODULE.reconcile_receipts(plan, rows, subjects=self.SUBJECTS)
        self.assertEqual((union['focused_receipts'], union['baselines'], union['generated_variants']),
                         (4, 4, 406))
        self.assertEqual(union['critical_owners'], sorted(self.SUBJECTS))
        self.assertFalse(union['critical_manifest_complete_selection'])
        self.assertEqual(union['scope'], 'Four settled-rate critical owner mutation receipts; '
                         'coverage/contracts and native Windows remain separate.')

    def test_four_owner_listing_2048_keeps_each_cohort_at_512(self):
        plan, args = self.fixture()
        for row in plan['inventories'].values():
            row['count'] = 512
        plan['cumulative_reservation'] = MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        self.assertEqual(MODULE.admit_plan(plan, **args)['reservation']['mutation_variants'], 2048)
        for count in (0, 513, True, 512.0):
            with self.subTest(count=count), self.assertRaises(MODULE.Incomplete):
                plan['inventories']['rate_delivery']['count'] = count
                MODULE.plan_reservation(plan, args['package_budget'], args['check_budgets'])

    def test_new_2820_field_width_does_not_widen_predecessor_profiles(self):
        plan, args = self.fixture()
        package = {f'metric{index}': 1 for index in range(2820)}
        args['package_budget'] = package
        args['check_budgets'] = {'coverage': dict(package), 'contracts': dict(package)}
        plan['cumulative_reservation'] = MODULE.plan_reservation(plan, package, args['check_budgets'])
        args['global_limits'] = dict(plan['cumulative_reservation']['source_work'])
        self.assertEqual(len(MODULE.admit_plan(plan, **args)['reservation']['source_work']), 2820)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.plan_reservation(plan, {**package, 'extra': 1}, args['check_budgets'])
        for owners, width in ((S157_SUBJECTS, 2214), (S158_SUBJECTS, 2682),
                              (ReferenceInputCohortTests.SUBJECTS, 2765),
                              (ChartCompositionCohortTests.SUBJECTS, 2740)):
            old, old_args = CohortPlanTests.fixture(self, owners)
            with self.subTest(owners=tuple(owners)), self.assertRaises(MODULE.Incomplete):
                MODULE.plan_reservation(old, package, {'coverage': dict(package), 'contracts': dict(package)})

    def test_four_owner_registration_and_cumulative_bindings_are_exact(self):
        for change in ('missing_owner', 'foreign_owner', 'wrong_path', 'missing_source',
                       'missing_cohort', 'duplicate_cohort', 'baseline', 'reservation', 'limit'):
            plan, args = self.fixture()
            if change == 'missing_owner': del plan['inventories']['rate_delivery']
            if change == 'foreign_owner': plan['inventories']['principal'] = plan['inventories'].pop('rate_delivery')
            if change == 'wrong_path': args['subjects'] = {**self.SUBJECTS, 'rate_delivery': 'src/other.rs'}
            if change == 'missing_source': del plan['product_inputs_sha256'][self.SUBJECTS['rate_delivery']]
            if change == 'missing_cohort': plan['cohorts'].pop()
            if change == 'duplicate_cohort': plan['cohorts'][1] = copy.deepcopy(plan['cohorts'][0])
            if change == 'baseline': plan['cumulative_reservation']['mutation_baselines'] = 3
            if change == 'reservation': plan['cumulative_reservation']['source_work']['fits'] -= 1
            if change == 'limit': args['global_limits']['fits'] -= 1
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_plan(plan, **args)

    def test_four_owner_union_rejects_partial_or_replayed_receipts(self):
        for change in ('missing', 'duplicate', 'baseline', 'gate', 'pending'):
            plan, args = self.fixture()
            rows = CohortPlanTests.receipts(self, plan, self.SUBJECTS)
            if change == 'missing': rows.pop()
            if change == 'duplicate': rows[-1] = copy.deepcopy(rows[0])
            if change == 'baseline': rows[0]['mutation']['reference_input']['baseline_tests_executed'] = 12
            if change == 'gate': rows[0]['mutation']['reference_input']['gate_passed'] = False
            if change == 'pending': rows[0]['core_observation_pending_invocations'] = ['reference_input:baseline']
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.reconcile_receipts(plan, rows, subjects=args['subjects'])


if __name__ == "__main__":
    unittest.main()
