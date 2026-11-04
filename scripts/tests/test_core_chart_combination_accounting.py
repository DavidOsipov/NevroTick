"""Independent S158/S1.4a fixture census and closed source-reservation controls."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import unittest


SCRIPTS = Path(__file__).resolve().parents[1]


def load(name):
    """Load pure helpers without importing the coordinator or running native work."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), SCRIPTS / (name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


MODULE = load("core-chart-combination-accounting")
ADAPTIVE = load("core-adaptive-accounting")
PROJECTION = load("core-projection-accounting")
ENCLOSURE = load("core-enclosure-accounting")
SELECTION = load("core-selection-accounting")
CHART = load("core-chart-accounting")
COMBINATION = load("core-combination-accounting")
CONTRACTS = load("core-chart-combination-contracts")


def owners():
    """Reference exact injected wrappers and units for the coordinator integration."""
    def lifecycle(calls):
        full = dict.fromkeys(COMBINATION.LIFECYCLE_FIELDS, 0)
        full.update(calls)
        return COMBINATION.lifecycle_source_work_uppers(
            full, dict.fromkeys(COMBINATION.LIFECYCLE_BRIDGE_FIELDS, 0),
            dict.fromkeys(COMBINATION.LIFECYCLE_COUNTER_FIELDS, 0))
    return MODULE.WorkOwners(
        adaptive_work=lambda c: ADAPTIVE.source_work(c, include_raw_minimum=False),
        projection_work=PROJECTION.source_work_uppers,
        adapter_work=lambda c: PROJECTION.projection_adapter_work(c["fixed"], c["adaptive"]),
        enclosure_work=ENCLOSURE.source_work_uppers,
        selection_work=lambda c: SELECTION.source_work_uppers(c["evaluations"], c["batches"]),
        chart_owner_work=CHART._owner_work,
        tracking_work=lambda c: CHART._tracking_work(c["tracked"]),
        chart_roundoff_work=CHART._roundoff_work,
        chart_batch_work=lambda c: CHART._batch_work(c["batches"], c["selections"], c["minima"], raw_header=False),
        legacy_batch_work=lambda c: CHART._principal_legacy_batch_work(c["batches"]),
        descriptor_work=lambda c: CHART._principal_descriptor_work(c["advances"]),
        principal_receiver_work=CHART._principal_receiver_work,
        lifecycle_work=lifecycle)


def prior_budget(*, legal=False):
    """Independent old-width fixture retains named aggregate and adapter scopes."""
    seeds, advances, completions, combinations = (2, 2, 0, 1) if legal else (100, 304, 7, 60)
    result = {
        "charged_fit_work_upper": 499 if legal else 48807,
        "point_admissions_upper": 17256 if legal else 1758792,
        "standard_errors.geometry.report_copies_upper": 499 if legal else 48807,
        "standard_errors.geometry.logical_bytes_upper": 32 * (499 if legal else 48807),
        "chart_principal.prior_adapter.seeds_source_upper": seeds,
        "chart_principal.prior_adapter.advances_source_upper": advances,
        "chart_principal.prior_adapter.direct_completions_source_upper": completions,
        "combination.call.combinations": combinations,
        "lifecycle.work.bridge_admission_entry_attempts_upper": 2 if legal else 99,
        "lifecycle.work.lifecycle_output_predicate_calls_upper": 4 if legal else 300,
        "lifecycle.work.lifecycle_internal_product_getter_requests_upper": 16384 if legal else 1000000,
        "lifecycle.work.bridge_reduction_entry_attempts_upper": 2 if legal else 120,
        "chart.roundoff_private.bound_requests_upper": 0 if legal else 41,
        "chart.roundoff_work.bound_requests_upper": 7 if legal else 100,
        "chart_batch.roundoff_work.bound_requests_upper": 5 if legal else 200,
        "chart_principal.roundoff_work.bound_requests_upper": 3 if legal else 300,
    }
    result.update({"old.owner." + str(i): i for i in range(2214 - len(result))})
    return result


class ReservationTests(unittest.TestCase):
    def test_real_delegates_fund_complete_new_source(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        self.assertEqual(len(vector), 486)
        self.assertEqual(MODULE.numeric_work(vector), {
            "charged_fit_work_upper": 13020, "point_admissions_upper": 449610})
        self.assertEqual(vector["chart_combination.fixture.appends"], 210)
        self.assertEqual(vector["chart_combination.observer_work.ordinary_outcomes_upper"], 28)

    def test_independent_complete_source_census(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        expected = {"histories": 69, "observations": 210, "appends": 210, "charts": 32,
                    "prepares": 83, "terms": 83, "anchor_projections": 4, "tracking_projections": 79,
                    "chart_enclosures": 83, "raw_enclosures": 73, "chart_batches": 29,
                    "raw_batches": 23, "minima": 24, "principal_parameters": 23, "strata": 83,
                    "principal_seeds": 7, "chart_principal_advances": 10, "combination_seeds": 16,
                    "combination_advances": 31, "state_equalities": 12, "state_clones": 0}
        self.assertEqual({key: vector["chart_combination.fixture." + key] for key in expected}, expected)
        # Forty-one real principal advances plus one direct bound-case selection.
        self.assertEqual(vector["chart_combination.selection_work.selection_endpoint_collection_writes_upper"], 32 * 42)
        bound = MODULE.case_recipes()["bound_only_failure_rolls_back_and_allows_same_observation_retry"]
        self.assertEqual(bound["direct_selections"], 1)
        self.assertEqual(bound["fixture"]["minima"], 3)
        self.assertEqual(bound["fixture"]["combination_advances"], 4)

    def test_all_28_names_match_actual_authored_test_functions(self):
        paths = (
            ("crates/time-core/tests/private/chart_combination_roundoff.rs", "combination::chart::roundoff::tests::"),
            ("crates/time-core/src/combination/chart/receiving_tests.rs", "combination::admission::chart_receiving::tests::"),
            ("crates/time-core/tests/chart_combination_lifecycle.rs", ""),
            ("crates/time-core/tests/chart_combination_edges.rs", ""),
            ("crates/time-core/tests/private/chart_combination_lifecycle_guards.rs",
             "combination::lifecycle::chart::guard_tests::"),
            ("crates/time-core/tests/private/reference_input_readiness.rs", "reference_input_readiness::"))
        actual = []
        for path, prefix in paths:
            source = (SCRIPTS.parent / path).read_text()
            actual.extend(prefix + name for name in re.findall(r"#\[test\]\s*fn ([A-Za-z0-9_]+)\(", source))
        self.assertEqual(len(actual), 33)
        self.assertEqual(tuple(name for name in actual if name not in MODULE.ACHIEVED_PHASE_ORDINARY_TESTS),
                         MODULE.CURRENT_ORDINARY_TESTS)
        self.assertEqual(tuple(name for name in actual if name in MODULE.ACHIEVED_PHASE_ORDINARY_TESTS),
                         MODULE.ACHIEVED_PHASE_ORDINARY_TESTS)
        self.assertEqual(len(set(MODULE.CURRENT_ORDINARY_TESTS)), 28)

    def test_all_18_historical_source_pins_remain_explicit(self):
        actual = dict(MODULE.SOURCE_SHA256)
        self.assertEqual(len(actual), 18)
        self.assertEqual(MODULE.admit_sources(actual), actual)
        current = {path: hashlib.sha256((SCRIPTS.parent / path).read_bytes()).hexdigest()
                   for path in actual}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_sources(current)
        for key in actual:
            altered = dict(actual)
            altered[key] = "0" * 64
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_sources(altered)
        for altered in ({}, {**actual, "unreviewed.rs": "0" * 64}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_sources(altered)

    def test_doctest_and_coverage_keep_all_other_482_fields(self):
        whole = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        coverage = MODULE.full_package_reservation(owners(), prior_budget=prior_budget(), include_doctests=False)
        changed = {key for key in whole if whole[key] != coverage[key]}
        self.assertEqual(changed, {"chart_combination.doctest.compilation_cases", "chart_combination.doctest.compile_fail_cases",
                                  "chart_combination.reference_input_work.doctest_compilation_cases",
                                  "chart_combination.reference_input_work.doctest_compile_fail_cases"})
        self.assertEqual(MODULE.numeric_work(whole), MODULE.numeric_work(coverage))
        self.assertEqual(whole["chart_combination.doctest.executed_examples"], 0)
        source = (SCRIPTS.parent / "crates/time-core/src/combination/lifecycle/chart.rs").read_text()
        lines = tuple(index for index, line in enumerate(source.splitlines(), 1) if "```compile_fail" in line)
        self.assertEqual(lines, (163,))
        self.assertEqual(MODULE.DOCTEST_LINES, lines)
        for flag in (0, 1, None, "true"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.full_package_reservation(owners(), prior_budget=prior_budget(), include_doctests=flag)

    def test_extension_preserves_every_old_nonaggregate(self):
        old = prior_budget()
        vector = MODULE.full_package_reservation(owners(), prior_budget=old)
        extended = MODULE.preserve_and_extend(old, vector)
        self.assertEqual(len(old), 2214)
        self.assertEqual(len(extended), 2700)
        aggregate = {"charged_fit_work_upper", "point_admissions_upper",
                     "standard_errors.geometry.report_copies_upper", "standard_errors.geometry.logical_bytes_upper"}
        self.assertEqual({key: value for key, value in old.items() if key not in aggregate},
                         {key: extended[key] for key in old if key not in aggregate})
        self.assertEqual(extended["charged_fit_work_upper"], 61827)
        self.assertEqual(extended["point_admissions_upper"], 2208402)
        self.assertEqual(extended["standard_errors.geometry.report_copies_upper"], 61827)
        self.assertEqual(extended["standard_errors.geometry.logical_bytes_upper"], 61827 * 32)
        self.assertEqual(old, prior_budget())

    def test_prior_shared_refactor_and_bridge_work_is_separately_funded(self):
        old = prior_budget()
        vector = MODULE.full_package_reservation(owners(), prior_budget=old)
        prefix = "chart_combination.prior_adapter_work."
        self.assertEqual(vector[prefix + "legacy_principal_seeds_upper"], 100)
        self.assertEqual(vector[prefix + "legacy_principal_advances_upper"], 304)
        self.assertEqual(vector[prefix + "legacy_direct_completions_upper"], 7)
        # 100+304+7 old operations,99 admission entries and60 direct kernels.
        self.assertEqual(vector[prefix + "original_cache_slot_initializations_upper"], 16 * 570)
        self.assertEqual(vector[prefix + "original_cache_materialization_getters_upper"], 64 * 570)
        self.assertEqual(vector[prefix + "supporter_values_extractions_upper"], 16 * (570 + 120))
        self.assertEqual(vector[prefix + "transaction_adapter_entries_upper"], 3 * 311 + 300 + 3 * 31)
        self.assertEqual(vector[prefix + "decision_outcome_adapter_getters_upper"], 128 * 411 + 1000000 + 128 * 31)
        self.assertEqual(vector[prefix + "bound_bridge_calls_upper"], 641)
        # Previously named ceilings remain present without any favorable credit.
        extended = MODULE.preserve_and_extend(old, vector)
        for key in (*MODULE.PRIOR_ADAPTER_KEYS, *MODULE.PRIOR_BOUND_KEYS, *MODULE.PRIOR_LIFECYCLE_KEYS):
            self.assertEqual(extended[key], old[key])

    def test_every_positive_delegated_named_ceiling_rejects_a_discount(self):
        recipe = MODULE._merge_recipes(list(MODULE.case_recipes().values()))
        original = owners()
        actual = MODULE._delegated(original, recipe)
        checked = 0
        for group, fields in actual.items():
            callback = getattr(original, group)
            for field, amount in fields.items():
                if not amount:
                    continue
                def discounted(calls, callback=callback, field=field):
                    result = callback(calls)
                    if result[field]:
                        result[field] -= 1
                    return result
                altered = original._replace(**{group: discounted})
                with self.subTest(group=group, field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.full_package_reservation(altered, prior_budget=prior_budget())
                checked += 1
        self.assertGreater(checked, 150)

    def test_delegate_shapes_types_and_overflow_are_closed(self):
        original = owners()
        for change in (lambda d: d.pop(next(iter(d))), lambda d: d.update(extra=1),
                       lambda d: d.update(charged_fit_work_upper=True),
                       lambda d: d.update(charged_fit_work_upper=-1),
                       lambda d: d.update(charged_fit_work_upper=1 << 63)):
            def hostile(calls, change=change):
                result = ADAPTIVE.source_work(calls, include_raw_minimum=False)
                change(result)
                return result
            with self.assertRaises(MODULE.Incomplete):
                MODULE.full_package_reservation(original._replace(adaptive_work=hostile), prior_budget=prior_budget())

    def test_every_current_budget_field_is_closed_exact_integer(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        self.assertEqual(set(vector), set(MODULE.FIELDS))
        self.assertEqual(len(set(MODULE.FIELDS)), 486)
        for key in vector:
            for invalid in (True, -1, 1 << 63):
                altered = dict(vector)
                altered[key] = invalid
                with self.subTest(key=key, invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.numeric_work(altered)
        for changed in ({}, {**vector, "chart_combination.extra": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.numeric_work(changed)

    def test_prior_missing_duplicate_namespace_discount_and_overflow_reject(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        for key in (*MODULE.PRIOR_ADAPTER_KEYS, *MODULE.PRIOR_BOUND_KEYS, *MODULE.PRIOR_LIFECYCLE_KEYS):
            missing = prior_budget()
            missing.pop(key)
            missing["old.replacement"] = 0
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.preserve_and_extend(missing, vector)
        for key in MODULE.PRIOR_ADAPTER_KEYS[:2]:
            old = prior_budget()
            old[key] = 0
            with self.assertRaises(MODULE.Incomplete):
                MODULE.full_package_reservation(owners(), prior_budget=old)
        for key in ("chart_combination.extra", "charged_fit_work_upper"):
            old = prior_budget()
            if key.startswith("chart_combination"):
                old.pop("old.owner.0")
            old[key] = MODULE.MAX_UNITS
            with self.assertRaises(MODULE.Incomplete):
                MODULE.preserve_and_extend(old, vector)

    def test_capacity_16_kernel_and_full_failure_prefix_are_preserved(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        #32 reduction attempt envelopes plus4record,3finish and2singleton calls.
        for key, ceiling in MODULE.POINT_CEILINGS.items():
            self.assertEqual(vector["chart_combination.point_work." + key], ceiling * 41)
        self.assertEqual(vector["chart_combination.reduction_work.driver_attempts_upper"], 32)
        self.assertGreaterEqual(vector["chart_combination.reduction_work.direct_record_attempts_upper"], 16 * 32)
        self.assertEqual(vector["chart_combination.output_work.postcondition_passes_upper"], 264)
        self.assertEqual(vector["chart_combination.companion_work.bound_successor_requests_upper"],
                         vector["chart_combination.companion_work.bound_primitive_requests_upper"] +
                         vector["chart_combination.companion_work.lower_margin_requests_upper"])

    def test_direct_raw_garde_callbacks_add_receiving_predicate_passes(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        #15 direct receiver admissions plus31 lifecycle attempts each invoke
        #typed+Garde validation;10 direct predicates and4 direct raw Garde calls
        #are disjoint authored requests in the wrong-relationship matrix.
        prefix = "chart_combination.receiving_work."
        self.assertEqual(vector[prefix + "admission_requests_upper"], 46)
        self.assertEqual(vector[prefix + "garde_validations_upper"], 50)
        self.assertEqual(vector[prefix + "receiving_predicate_passes_upper"], 106)
        self.assertEqual(vector[prefix + "context_comparisons_upper"], 106 * 48)
        self.assertEqual(vector[prefix + "actual_wrapper_comparisons_upper"], 106 * 256)

    def test_same_label_copied_principal_adds_receiving_without_producers(self):
        name = "combination::admission::chart_receiving::tests::chart_tracking_receiver_rejects_wrong_relationships"
        recipe = MODULE.case_recipes()[name]
        self.assertEqual(recipe["receiving"], (1, 6, 10, 4))
        self.assertEqual(recipe["fixture"]["getters_upper"], 2060)
        self.assertEqual(recipe["fixture"]["histories"], 6)
        self.assertEqual(recipe["fixture"]["appends"], 18)
        self.assertEqual(recipe["fixture"]["tracking_projections"], 6)
        self.assertEqual(recipe["fixture"]["chart_principal_advances"], 1)

        #Independent old-recipe reconstruction identifies every changed upper;
        #the copied principal reuses an existing genuine slot and attachment.
        before = MODULE.case_recipes()
        before[name]["receiving"] = (1, 6, 9, 3)
        before[name]["fixture"]["getters_upper"] -= 12
        before[name]["fixture"]["labels_upper"] -= 12
        old = MODULE._reservation(owners(), MODULE._merge_recipes(list(before.values())), prior_budget(),
                                  ordinary=28, docs=True, external_legal=False)
        current = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        actual_delta = {key.removeprefix("chart_combination."): current[key] - old[key]
                        for key in current if current[key] != old[key]}
        self.assertEqual(actual_delta, {
            "fixture.getters_upper": 12, "fixture.labels_upper": 12,
            "receiving_work.garde_validations_upper": 1,
            "receiving_work.receiving_predicate_passes_upper": 2,
            "receiving_work.context_comparisons_upper": 96,
            "receiving_work.actual_wrapper_comparisons_upper": 512,
            "receiving_work.duplicate_label_comparisons_upper": 240,
            "receiving_work.witness_label_comparisons_upper": 512,
            "receiving_work.attachment_pointer_comparisons_upper": 32,
            "receiving_work.padding_slot_visits_upper": 32,
            "observer_work.getter_requests_upper": 12})
        self.assertEqual(MODULE.numeric_work(current), MODULE.numeric_work(old))

    def test_new_fixed_arrays_fund_defaults_and_occupied_label_writes(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        #Holder16defaults and16occupied record fills have no separate field.
        self.assertEqual(vector["chart_combination.receiving_work.holder_requests_upper"], 41)
        self.assertEqual(vector["chart_combination.receiving_work.holder_slot_initializations_upper"], 41 * 32)
        #Each new receiving admission defaults two16-slot arrays and fills up
        #to16 witness-label positions;16 original materializations are separate.
        self.assertEqual(vector["chart_combination.receiving_work.cache_slot_initializations_upper"], 46 * 48)
        self.assertEqual(vector["chart_combination.receiving_work.cache_record_materializations_upper"], 46 * 16)
        #Each reduction defaults16labels and writes up to16 contributors.
        self.assertEqual(vector["chart_combination.reduction_work.canonical_label_writes_upper"], 32 * 32)

    def test_direct_lifecycle_guards_are_funded_without_fabricated_advances(self):
        recipe = MODULE._recipe(MODULE._fixture())
        recipe["guards"] = {
            "quantity_predicates": 2, "estimate_predicates": 3, "principal_matches": 5,
            "contributor_predicates": 7, "contributor_garde": 11,
            "outcome_predicates": 13, "outcome_garde": 17,
        }
        vector = MODULE._reservation(owners(), recipe, prior_budget(), ordinary=0,
                                     docs=False, external_legal=False)
        prefix = "chart_combination."
        # Forty complete-output or proper subpredicate envelopes include all
        # nested quantity/estimate visits; eighteen direct mask passes are separate.
        self.assertEqual(vector[prefix + "output_work.postcondition_passes_upper"], 40)
        self.assertEqual(vector[prefix + "output_work.garde_validations_upper"], 17)
        self.assertEqual(vector[prefix + "output_work.completion_attempts_upper"], 0)
        self.assertEqual(vector[prefix + "reduction_work.mask_garde_validations_upper"], 11)
        self.assertEqual(vector[prefix + "reduction_work.mask_predicate_passes_upper"], 18)
        self.assertEqual(vector[prefix + "reduction_work.mask_slot_visits_upper"], 18 * 16)
        # Only whole-output calls enter the shared parent predicate. Every
        # direct/subpredicate envelope retains its full nested getter ceiling.
        self.assertEqual(vector[prefix + "lifecycle_work.lifecycle_output_predicate_calls_upper"], 30)
        self.assertEqual(vector[prefix + "lifecycle_work.lifecycle_garde_validate_calls_upper"], 17)
        self.assertEqual(vector[prefix + "lifecycle_work.lifecycle_witness_membership_label_probes_upper"], 30 * 256)
        self.assertEqual(vector[prefix + "lifecycle_work.lifecycle_output_contributor_label_probes_upper"], 30 * 256)
        self.assertEqual(vector[prefix + "lifecycle_work.lifecycle_internal_product_getter_requests_upper"], 58 * 8192)
        for key in ("fixture.combination_advances", "fixture.chart_reductions",
                    "reduction_work.driver_attempts_upper", "point_work.kernel_checked_binary_calls_upper",
                    "companion_work.exact_constructions_upper"):
            self.assertEqual(vector[prefix + key], 0)
        self.assertEqual(MODULE.numeric_work(vector), {"charged_fit_work_upper": 0, "point_admissions_upper": 0})
        extended = MODULE.preserve_and_extend(prior_budget(), vector)
        self.assertEqual(len(extended), 2700)
        self.assertEqual({key: extended[key] for key in prior_budget()}, prior_budget())

    def test_raw_guard_case_census_and_reachable_direct_combine_prefix(self):
        pair_name = "combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_preserve_complete_relationships"
        count_name = "combination::lifecycle::chart::guard_tests::chart_raw_count_seventeen_cannot_reuse_sixteen_slots"
        pair, count = (MODULE.case_recipes()[name] for name in (pair_name, count_name))
        self.assertEqual(pair["guards"], {
            "quantity_predicates": 49, "estimate_predicates": 49, "principal_matches": 2,
            "contributor_predicates": 3, "contributor_garde": 3,
            "outcome_predicates": 49, "outcome_garde": 49})
        self.assertEqual({key: pair["fixture"][key] for key in (
            "histories", "appends", "tracking_projections", "raw_enclosures", "chart_batches",
            "combination_advances", "chart_reductions", "labels_upper", "getters_upper")}, {
                "histories": 2, "appends": 6, "tracking_projections": 2, "raw_enclosures": 2,
                "chart_batches": 2, "combination_advances": 2, "chart_reductions": 3,
                "labels_upper": 14, "getters_upper": 512})
        self.assertEqual(pair["receiving"], (1, 1, 0, 0))
        vector = MODULE._reservation(owners(), pair, prior_budget(), ordinary=1,
                                     docs=False, external_legal=False)
        self.assertEqual(vector["chart_combination.fixture.combination_advances"], 2)
        self.assertEqual(vector["chart_combination.reduction_work.driver_attempts_upper"], 3)
        # The direct selection mismatch returns before reducing in the baseline.
        # Its known OR-to-AND fault reaches the full two-record reduction. Every
        # attempt therefore retains the complete capacity16 driver envelope.
        self.assertEqual(vector["chart_combination.reduction_work.direct_record_attempts_upper"], 3 * 16)
        self.assertEqual(vector["chart_combination.reduction_work.finish_attempts_upper"], 3)
        for key, maximum in MODULE.POINT_CEILINGS.items():
            self.assertEqual(vector["chart_combination.point_work." + key], 3 * maximum)
        self.assertGreaterEqual(vector["chart_combination.companion_work.garde_validations_upper"], 3)
        self.assertEqual(count["guards"], {
            "quantity_predicates": 2, "estimate_predicates": 2, "principal_matches": 0,
            "contributor_predicates": 0, "contributor_garde": 0,
            "outcome_predicates": 0, "outcome_garde": 0})
        self.assertEqual(count["fixture"]["histories"], 16)
        self.assertEqual(count["fixture"]["appends"], 48)
        self.assertEqual(count["fixture"]["chart_principal_advances"], 1)
        self.assertEqual(count["fixture"]["combination_advances"], 0)
        self.assertEqual(count["fixture"]["chart_reductions"], 0)
        vector = MODULE._reservation(owners(), count, prior_budget(), ordinary=1,
                                     docs=False, external_legal=False)
        for key in ("output_work.completion_attempts_upper", "output_work.garde_validations_upper",
                    "reduction_work.driver_attempts_upper", "point_work.kernel_checked_binary_calls_upper",
                    "companion_work.exact_constructions_upper", "lifecycle_work.lifecycle_output_predicate_calls_upper"):
            self.assertEqual(vector["chart_combination." + key], 0)
        self.assertEqual(MODULE.numeric_work(vector), {
            "charged_fit_work_upper": 2976, "point_admissions_upper": 102768})

    def test_direct_guard_recipe_requires_every_exact_axis(self):
        recipe = MODULE._recipe(MODULE._fixture())
        recipe["guards"] = dict.fromkeys((
            "quantity_predicates", "estimate_predicates", "principal_matches",
            "contributor_predicates", "contributor_garde", "outcome_predicates", "outcome_garde"), 0)
        # The missing guard record must not silently mean zero for a supplied
        # current mechanism recipe; internal _recipe owns explicit zero defaults.
        missing = dict(recipe)
        del missing["guards"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mechanism_reservation(owners(), missing, prior_budget=prior_budget(), ordinary_outcomes=0)
        healthy = MODULE.mechanism_reservation(owners(), recipe, prior_budget=prior_budget(), ordinary_outcomes=0)
        self.assertEqual(healthy["chart_combination.output_work.postcondition_passes_upper"], 0)
        for change in (lambda d: d.pop("quantity_predicates"), lambda d: d.update(unknown=1),
                       lambda d: d.update(outcome_garde=True), lambda d: d.update(outcome_garde=-1),
                       lambda d: d.update(outcome_garde=1 << 63), lambda d: d.update(outcome_garde=1 << 62)):
            bad = {**recipe, "guards": dict(recipe["guards"])}
            change(bad["guards"])
            with self.subTest(guards=bad["guards"]), self.assertRaises(MODULE.Incomplete):
                MODULE.mechanism_reservation(owners(), bad, prior_budget=prior_budget(), ordinary_outcomes=0)

    def test_external_legal_consumes_actual_strict_owner_and_separate_old_delta(self):
        contract = MODULE.ContractOwner(CONTRACTS.FIELDS, CONTRACTS.legal_expected, CONTRACTS.legal_contract_source)
        vector = MODULE.external_legal_reservation(owners(), contract, prior_budget=prior_budget(legal=True))
        self.assertEqual(MODULE.numeric_work(vector), {"charged_fit_work_upper": 186, "point_admissions_upper": 6423})
        self.assertEqual(sum(CONTRACTS.legal_expected().values()), 86)
        self.assertEqual(vector["chart_combination.fixture.histories"], 1)
        self.assertEqual(vector["chart_combination.fixture.coordinates"], 1)
        self.assertEqual(vector["chart_combination.fixture.combination_seeds"], 2)
        self.assertEqual(vector["chart_combination.fixture.combination_advances"], 3)
        self.assertEqual(vector["chart_combination.fixture.state_equalities"], 2)
        self.assertEqual(vector["chart_combination.fixture.state_clones"], 0)
        self.assertEqual(vector["chart_combination.prior_adapter_work.legacy_principal_advances_upper"], 2)
        self.assertEqual(vector["chart_combination.prior_adapter_work.transaction_adapter_entries_upper"], 19)
        self.assertEqual(vector["chart_combination.prior_adapter_work.decision_outcome_adapter_getters_upper"], 17280)
        self.assertEqual(vector["chart_combination.doctest.compilation_cases"], 0)
        for key in CONTRACTS.FIELDS:
            def changed(key=key):
                result = CONTRACTS.legal_expected()
                result[key] += 1
                return result
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.external_legal_reservation(owners(), contract._replace(expected=changed), prior_budget=prior_budget(legal=True))
        for altered in (contract._replace(fields=contract.fields[:-1]),
                        contract._replace(source=lambda: CONTRACTS.legal_contract_source() + "\n")):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.external_legal_reservation(owners(), altered, prior_budget=prior_budget(legal=True))

    def test_explicit_mechanism_projection_requires_independent_admission(self):
        recipe = MODULE._merge_recipes(list(MODULE.case_recipes().values()))
        projected = MODULE.mechanism_reservation(owners(), recipe, prior_budget=prior_budget(), ordinary_outcomes=28)
        coverage = MODULE.full_package_reservation(owners(), prior_budget=prior_budget(), include_doctests=False)
        self.assertEqual(projected, coverage)
        self.assertEqual(MODULE.STATUS, "prepared-source-recipe-not-native-admission")
        for altered in ({}, {**recipe, "unreviewed": 0}, {**recipe, "direct_selections": True},
                        {**recipe, "companion": (0,)}, {**recipe, "receiving": [0, 0, 0, 0]}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mechanism_reservation(owners(), altered, prior_budget=prior_budget(), ordinary_outcomes=28)

    def test_huge_recipe_rejects_without_unbounded_partition_loop(self):
        recipe = MODULE._merge_recipes(list(MODULE.case_recipes().values()))
        recipe["fixture"]["histories"] = MODULE.MAX_UNITS
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mechanism_reservation(owners(), recipe, prior_budget=prior_budget(), ordinary_outcomes=24)

    def test_recipes_and_results_do_not_share_mutable_state(self):
        rows = MODULE.case_recipes()
        first = next(iter(rows))
        rows[first]["fixture"]["appends"] = 999
        rows[first]["guards"]["quantity_predicates"] = 999
        self.assertEqual(MODULE.case_recipes()[first]["fixture"]["appends"], 0)
        self.assertEqual(MODULE.case_recipes()[first]["guards"]["quantity_predicates"], 0)
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        vector["chart_combination.fixture.appends"] = 0
        self.assertEqual(MODULE.full_package_reservation(owners(), prior_budget=prior_budget())["chart_combination.fixture.appends"], 210)


class OutcomeTests(unittest.TestCase):
    def setUp(self):
        self.complete = "".join("test " + name + " ... ok\n" for name in MODULE.CURRENT_ORDINARY_TESTS)

    def test_complete_anchored_new_outcomes_only(self):
        received = MODULE.source_test_outcomes(self.complete + "test historical_case ... ok\n", require_complete=True)
        self.assertTrue(received["complete"])
        self.assertEqual(received["status"], "complete")
        self.assertEqual(received["expected"], 28)
        self.assertEqual(len(received["outcomes"]), 28)

    def test_failed_ignored_and_missing_never_complete_or_refund(self):
        budget = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        for changed in ("", self.complete.replace(" ... ok", " ... FAILED", 1),
                        self.complete.replace(" ... ok", " ... ignored", 1),
                        "\n".join(self.complete.splitlines()[1:])):
            with self.subTest(output=changed[:40]):
                result = MODULE.source_test_outcomes(changed)
                self.assertFalse(result["complete"])
                self.assertEqual(result["status"], "incomplete")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.source_test_outcomes(changed, require_complete=True)
                self.assertEqual(MODULE.full_package_reservation(owners(), prior_budget=prior_budget()), budget)

    def test_duplicate_unknown_private_and_malformed_current_rows_reject(self):
        first = self.complete.splitlines()[0]
        for changed in (self.complete + first + "\n", first[:-1], first + " extra",
                        "test combination::chart::roundoff::tests::unreviewed ... ok\n",
                        "test combination::lifecycle::chart::guard_tests::unreviewed ... ok\n"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_test_outcomes(changed)
        for value in (None, True, "x" * (16 * 1024 * 1024 + 1)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_test_outcomes(value)


class ReferenceInputReservationTests(unittest.TestCase):
    """Independent S1.4a source census, mutation prefixes and legacy custody."""

    def test_new_fixture_and_direct_reference_census(self):
        names = (
            "reference_input_readiness::genuine_ready_input_is_readonly_and_replay_has_no_new_credit",
            "reference_input_readiness::unavailable_and_waiting_outcomes_reject_without_reviving_readiness",
            "reference_input_readiness::foreign_chart_instances_reject_beside_the_bound_original",
            "reference_input_readiness::feedback_pairs_and_repeats_never_restore_an_invalidated_input",
        )
        self.assertEqual(MODULE.REFERENCE_ORDINARY_TESTS, names)
        rows = MODULE.case_recipes()
        # Constructors/readiness/admissions/feedback/healthy tokens/estimate and
        # round reads/Display/Error Debug/State+Token Debug, in frozen field order.
        expected = ((1, 7, 6, 0, 5, 6, 6, 6, 1, 2),
                    (1, 4, 5, 1, 1, 1, 1, 0, 0, 0),
                    (9, 40, 32, 8, 16, 16, 16, 0, 0, 0),
                    (9, 63, 27, 45, 5, 5, 5, 0, 0, 0))
        for name, values in zip(names, expected, strict=True):
            self.assertEqual(rows[name]["reference_input"], dict(zip(MODULE.REFERENCE_FIELDS, values, strict=True)))
        merged = MODULE._merge_recipes([rows[name] for name in names])
        fixture = merged["fixture"]
        for key, value in {"histories": 8, "appends": 25, "observations": 25,
                           "tracking_projections": 10, "chart_enclosures": 10,
                           "charts": 12, "frames_upper": 9, "domains_upper": 6,
                           "raw_enclosures": 8, "raw_batches": 4, "chart_batches": 6,
                           "combination_seeds": 4, "combination_advances": 8,
                           "chart_reductions": 8, "strata": 8}.items():
            with self.subTest(key=key):
                self.assertEqual(fixture[key], value)

    def test_old_2682_field_contract_and_recipes_are_preserved(self):
        self.assertEqual((MODULE.BUDGET_FIELDS, MODULE.FULL_BUDGET_FIELDS), (486, 2700))
        digest = lambda value: hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        self.assertEqual(digest(MODULE.FIELDS[:468]), "f06eedd0983c80d8ad11512fdbfbd547200ea429ffb30f26366d1b77c23cddfe")
        self.assertEqual(len(MODULE.FIELDS[468:]), 18)
        self.assertTrue(all(name.startswith("chart_combination.reference_input_work.") for name in MODULE.FIELDS[468:]))
        old = MODULE.s158_case_recipes()
        self.assertEqual(digest(tuple(old)), "4b004505ace60b331a7236f304e59b9ed90504e75c08df0127eb924221bd2f40")
        vector = MODULE._reservation(owners(), MODULE._merge_recipes(list(old.values())),
                                     prior_budget(), ordinary=24, docs=True, external_legal=False)
        self.assertEqual(digest({key: vector[key] for key in MODULE.FIELDS[:468]}),
                         "391b5a85c2b557cf5b7efed247919a5d1fc50cdc777e82c56c37b70927223568")
        for row in old.values():
            self.assertEqual(row["reference_input"], dict.fromkeys(MODULE.REFERENCE_FIELDS, 0))

    def test_complete_current_numeric_delta_is_once_and_docs_are_uninvoked(self):
        vector = MODULE.full_package_reservation(owners(), prior_budget=prior_budget())
        self.assertEqual(MODULE.numeric_work(vector), {
            "charged_fit_work_upper": 13020, "point_admissions_upper": 449610})
        extended = MODULE.preserve_and_extend(prior_budget(), vector)
        self.assertEqual(extended["charged_fit_work_upper"], 61827)
        self.assertEqual(extended["point_admissions_upper"], 2208402)
        self.assertEqual(extended["standard_errors.geometry.report_copies_upper"], 61827)
        self.assertEqual(extended["standard_errors.geometry.logical_bytes_upper"], 61827 * 32)
        prefix = "chart_combination.reference_input_work."
        self.assertEqual(vector[prefix + "relationship_passes_upper"], 140)
        self.assertEqual(vector[prefix + "garde_validations_upper"], 70)
        self.assertEqual(vector[prefix + "garde_callbacks_upper"], 70)
        self.assertEqual(vector[prefix + "upstream_getter_requests_upper"], 840)
        self.assertEqual(vector[prefix + "doctest_compilation_cases"], 3)
        self.assertEqual(vector[prefix + "doctest_compile_fail_cases"], 2)
        self.assertEqual(vector[prefix + "doctest_executed_examples"], 0)
        without = MODULE.full_package_reservation(owners(), prior_budget=prior_budget(), include_doctests=False)
        self.assertEqual(MODULE.numeric_work(vector), MODULE.numeric_work(without))
        self.assertEqual({key for key in vector if vector[key] != without[key]}, {
            "chart_combination.doctest.compilation_cases", "chart_combination.doctest.compile_fail_cases",
            prefix + "doctest_compilation_cases", prefix + "doctest_compile_fail_cases"})

    def test_every_admission_reserves_possible_unexpected_token(self):
        rows = MODULE.case_recipes()
        recipe = MODULE._merge_recipes([rows[name] for name in MODULE.REFERENCE_ORDINARY_TESTS])
        self.assertEqual(recipe["reference_input"]["token_constructions"], 27)
        vector = MODULE.mechanism_reservation(owners(), recipe, prior_budget=prior_budget(), ordinary_outcomes=4)
        # A weakened guard can create a token before rejected() detects failure.
        # Reserving only the healthy27 successes discounts that actual prefix.
        self.assertEqual(vector["chart_combination.reference_input_work.token_constructions"], 70)
        self.assertEqual(vector["chart_combination.prior_adapter_work.legacy_principal_seeds_upper"], 100)
        self.assertEqual(vector["chart_combination.prior_adapter_work.legacy_principal_advances_upper"], 304)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mechanism_reservation(owners(), recipe, prior_budget=dict.fromkeys(prior_budget(), 0), ordinary_outcomes=4)

    def test_reference_recipe_requires_complete_exact_axes(self):
        recipe = MODULE._merge_recipes(list(MODULE.case_recipes().values()))
        absent = dict(recipe)
        del absent["reference_input"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mechanism_reservation(owners(), absent, prior_budget=prior_budget(), ordinary_outcomes=28)
        for field in MODULE.REFERENCE_FIELDS:
            for invalid in (True, -1, 1 << 63):
                bad = {**recipe, "reference_input": {**recipe["reference_input"], field: invalid}}
                with self.subTest(field=field, invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.mechanism_reservation(owners(), bad, prior_budget=prior_budget(), ordinary_outcomes=28)
        for changed in ({}, {**recipe["reference_input"], "foreign": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mechanism_reservation(owners(), {**recipe, "reference_input": changed},
                                             prior_budget=prior_budget(), ordinary_outcomes=28)

    def test_reference_outcomes_cannot_hide_behind_all_24_historical_rows(self):
        old = "".join("test " + name + " ... ok\n" for name in MODULE.s158_case_recipes())
        received = MODULE.source_test_outcomes(old)
        self.assertFalse(received["complete"])
        self.assertEqual(received["missing"], list(MODULE.REFERENCE_ORDINARY_TESTS))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.source_test_outcomes(old, require_complete=True)
        complete = old + "".join("test " + name + " ... ok\n" for name in MODULE.REFERENCE_ORDINARY_TESTS)
        self.assertTrue(MODULE.source_test_outcomes(complete, require_complete=True)["complete"])
        for changed in (complete + "test reference_input_readiness::foreign ... ok\n",
                        complete + "test " + MODULE.REFERENCE_ORDINARY_TESTS[0] + " ... ok\n"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_test_outcomes(changed)

    def test_reference_external_profile_preserves_actual_legacy_producer_calls(self):
        old_contract = MODULE.ContractOwner(CONTRACTS.FIELDS, CONTRACTS.legal_expected, CONTRACTS.legal_contract_source)
        new_contract = MODULE.ContractOwner(CONTRACTS.REFERENCE_FIELDS,
            CONTRACTS.reference_legal_expected, CONTRACTS.reference_legal_contract_source)
        old = MODULE.external_legal_reservation(owners(), old_contract, prior_budget=prior_budget(legal=True))
        new = MODULE.external_legal_reservation(owners(), new_contract,
            prior_budget=prior_budget(legal=True), reference_input=True)
        self.assertEqual(sum(CONTRACTS.reference_legal_expected().values()), 102)
        self.assertEqual(MODULE.numeric_work(new), {"charged_fit_work_upper": 186, "point_admissions_upper": 6423})
        self.assertEqual(MODULE.numeric_work(new), MODULE.numeric_work(old))
        prefix = "chart_combination.reference_input_work."
        expected = dict(zip(MODULE.REFERENCE_FIELDS, (1, 2, 4, 4, 4, 1, 2, 0, 0, 0), strict=True))
        expected.update(relationship_passes_upper=8, garde_validations_upper=4, garde_callbacks_upper=4,
                        chart_identity_checks_upper=8, upstream_getter_requests_upper=48,
                        doctest_compilation_cases=0, doctest_compile_fail_cases=0, doctest_executed_examples=0)
        self.assertEqual({key.removeprefix(prefix): value for key, value in new.items() if key.startswith(prefix)}, expected)
        self.assertTrue(all(value == 0 for key, value in old.items() if key.startswith(prefix)))
        # Two explicit estimate reads and two chart getter visits are the only
        # changes to the old source family; constructors and advances are reused.
        changed = {key: new[key] - old[key] for key in MODULE.FIELDS[:468] if new[key] != old[key]}
        self.assertEqual(changed, {"chart_combination.fixture.getters_upper": 4,
                                  "chart_combination.observer_work.getter_requests_upper": 4})
        for contract, selected in ((old_contract, True), (new_contract, False)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.external_legal_reservation(owners(), contract,
                    prior_budget=prior_budget(legal=True), reference_input=selected)
        for selected in (0, 1, None, "true"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.external_legal_reservation(owners(), new_contract,
                    prior_budget=prior_budget(legal=True), reference_input=selected)

    def test_reference_external_metadata_and_source_are_exactly_bound(self):
        fields = CONTRACTS.REFERENCE_FIELDS
        actual = CONTRACTS.reference_legal_expected()
        source = CONTRACTS.reference_legal_contract_source()
        self.assertEqual(hashlib.sha256(source.encode()).hexdigest(),
                         "c7ea8535d63efe3003ba6c83c63f38bb853643f6a7fb988674ea12460d49b1df")
        for field in fields:
            for invalid in (True, -1, actual[field] + 1):
                expected = lambda field=field, invalid=invalid: {**actual, field: invalid}
                contract = MODULE.ContractOwner(fields, expected, lambda: source)
                with self.subTest(field=field, invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.external_legal_reservation(owners(), contract,
                        prior_budget=prior_budget(legal=True), reference_input=True)
        for changed in (source + "\n", CONTRACTS.legal_contract_source()):
            contract = MODULE.ContractOwner(fields, lambda: actual, lambda changed=changed: changed)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.external_legal_reservation(owners(), contract,
                    prior_budget=prior_budget(legal=True), reference_input=True)

    def test_reference_doctest_positions_are_actual_zero_runtime_functions(self):
        # These source positions belong to the explicitly retained S1.4a pins.
        # Current source positions are independently bound by the continuation.
        self.assertEqual(MODULE.REFERENCE_DOCTEST_LINES, (229, 248, 266))
        self.assertEqual(MODULE.REFERENCE_COMPILE_FAIL_LINES, (229, 248))


class CompositionFixtureDeltaTests(unittest.TestCase):
    """New source fixtures add work without rerunning historical floor recipes."""

    def test_asymmetric_consumer_delta_reuses_history_and_preserves_old_recipes(self):
        before = MODULE.case_recipes()
        recipe = MODULE.asymmetric_settled_transform_fixture_recipe()
        fixture = recipe['fixture']
        self.assertEqual({key: fixture[key] for key in (
            'histories', 'observations', 'appends', 'charts', 'frames_upper', 'domains_upper',
            'prepares', 'known_prepares_upper', 'terms', 'anchor_projections',
            'tracking_projections', 'chart_enclosures', 'raw_enclosures')}, {
            'histories': 0, 'observations': 0, 'appends': 0, 'charts': 1,
            'frames_upper': 0, 'domains_upper': 0, 'prepares': 2, 'known_prepares_upper': 2,
            'terms': 2, 'anchor_projections': 2, 'tracking_projections': 0,
            'chart_enclosures': 0, 'raw_enclosures': 0})
        delta = MODULE.fixture_delta_reservation(owners(), recipe, ordinary_outcomes=0)
        self.assertEqual(MODULE.numeric_work(delta), {'charged_fit_work_upper': 0, 'point_admissions_upper': 0})
        self.assertEqual(delta['chart_combination.chart_owner_work.history_garde_validations_upper'], 2)
        self.assertEqual(delta['chart_combination.chart_owner_work.history_relationship_passes_upper'], 4)
        self.assertEqual(delta['chart_combination.projection_call.projections'], 2)
        self.assertEqual(delta['chart_combination.chart_roundoff_work.bound_requests_upper'], 66)
        self.assertEqual(delta['chart_combination.chart_owner_work.affine_garde_validations_upper'], 1)
        self.assertTrue(all(value == 0 for key, value in delta.items() if key.startswith(
            ('chart_combination.tracking_work.', 'chart_combination.enclosure_call.',
             'chart_combination.enclosure_work.'))))
        self.assertEqual(MODULE.case_recipes(), before)
        self.assertEqual(MODULE.SOURCE_SHA256[
            'crates/time-core/tests/private/chart_combination_lifecycle_guards.rs'],
            '42bd34d892968439bd5852857e2c56ae980a42f9ad4594a4fb7a688e99ff3e45')
        recipe['fixture']['histories'] = 99
        self.assertEqual(MODULE.asymmetric_settled_transform_fixture_recipe()['fixture']['histories'], 0)

    def test_four_composition_recipes_are_separate_from_frozen_readiness(self):
        self.assertEqual(len(MODULE.case_recipes()), 28)
        rows = MODULE.composition_case_recipes()
        self.assertEqual(tuple(rows), (
            "chart_composition::composition_preserves_dyadic_values_and_rounding_provenance",
            "chart_composition::composition_retains_both_operand_errors_through_signed_elapsed",
            "chart_composition::composition_rejects_relationships_and_ordered_arithmetic_failures",
            "chart_composition_lifecycle::coefficient_error_identity_precedes_batch_principal_and_combination_updates"))
        recipe = MODULE.composition_package_fixture_recipe()
        expected = {"histories": 4, "appends": 12, "charts": 54, "domains_upper": 7,
                    "tracking_projections": 17, "chart_enclosures": 15, "raw_enclosures": 8,
                    "raw_batches": 4, "chart_batches": 4, "combination_seeds": 4,
                    "chart_principal_advances": 6, "combination_advances": 6}
        self.assertEqual({key: recipe["fixture"][key] for key in expected}, expected)
        self.assertFalse(set(rows) & set(MODULE.case_recipes()))

    def test_fixture_delta_has_no_historical_floor_or_numerical_duplication(self):
        recipe = MODULE.composition_package_fixture_recipe()
        delta = MODULE.fixture_delta_reservation(owners(), recipe, ordinary_outcomes=4)
        self.assertEqual(len(delta), 486)
        self.assertEqual(MODULE.numeric_work(delta), {"charged_fit_work_upper": 744, "point_admissions_upper": 25692})
        prefix = "chart_combination.prior_adapter_work."
        self.assertEqual(delta[prefix + "legacy_principal_seeds_upper"], 0)
        self.assertEqual(delta[prefix + "legacy_principal_advances_upper"], 0)
        self.assertEqual(delta[prefix + "transaction_adapter_entries_upper"], 18)
        self.assertEqual(delta[prefix + "decision_outcome_adapter_getters_upper"], 768)
        self.assertEqual(delta["chart_combination.reduction_work.driver_attempts_upper"], 6)
        self.assertEqual(delta["chart_combination.descriptor_work.relationship_passes_upper"], 24)
        # Existing whole-package admission still rejects favorable zero floors.
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mechanism_reservation(owners(), recipe, prior_budget=dict.fromkeys(prior_budget(), 0), ordinary_outcomes=4)
        self.assertEqual(MODULE.numeric_work(MODULE.full_package_reservation(owners(), prior_budget=prior_budget())),
                         {"charged_fit_work_upper": 13020, "point_admissions_upper": 449610})

    def test_descriptor_only_external_delta_has_no_fit_or_projection(self):
        recipe = MODULE.composition_external_fixture_recipe()
        delta = MODULE.fixture_delta_reservation(owners(), recipe, ordinary_outcomes=1)
        self.assertEqual(MODULE.numeric_work(delta), {"charged_fit_work_upper": 0, "point_admissions_upper": 0})
        self.assertEqual(delta["chart_combination.fixture.charts"], 5)
        self.assertEqual(delta["chart_combination.fixture.frames_upper"], 5)
        self.assertEqual(delta["chart_combination.fixture.domains_upper"], 1)
        self.assertEqual(delta["chart_combination.fixture.tracking_projections"], 0)
        self.assertEqual(delta["chart_combination.prior_adapter_work.transaction_adapter_entries_upper"], 0)

    def test_composition_sources_require_the_actual_new_profile(self):
        actual = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest()
                  for name in MODULE.COMPOSITION_SOURCE_SHA256}
        self.assertEqual(len(actual), 23)
        self.assertEqual(MODULE.admit_composition_sources(dict(MODULE.COMPOSITION_SOURCE_SHA256)),
                         MODULE.COMPOSITION_SOURCE_SHA256)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_composition_sources(actual)
        for bad in ({}, {**actual, "foreign": "0" * 64}, MODULE.SOURCE_SHA256):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_composition_sources(bad)
        for field in actual:
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_composition_sources({**actual, field: "0" * 64})


class AchievedPhaseFixtureTests(unittest.TestCase):
    """Current source consumers and failed prefixes beside frozen earlier recipes."""

    def test_five_actual_cases_and_receiver_attempts_are_independent_of_admission_success(self):
        source = (SCRIPTS.parent / "crates/time-core/tests/private/reference_input_readiness.rs").read_text()
        names = tuple("reference_input_readiness::" + name for name in
                      re.findall(r"#\[test\]\s*fn (achieved_phase_[A-Za-z0-9_]+)\(\)", source))
        self.assertEqual(names, MODULE.ACHIEVED_PHASE_ORDINARY_TESTS)
        self.assertEqual(MODULE.ACHIEVED_PHASE_RECEIVER_ATTEMPTS, (5, 26, 4, 2, 1))
        self.assertEqual(MODULE.ACHIEVED_PHASE_ADMITTED, (5, 13, 1, 1, 1))
        self.assertEqual(sum(MODULE.ACHIEVED_PHASE_RECEIVER_ATTEMPTS), 38)
        self.assertEqual(sum(MODULE.ACHIEVED_PHASE_ADMITTED), 21)
        rows = MODULE.achieved_phase_case_recipes()
        expected = (
            (1, 3, 12, 12, 1, 5, 0, 0),
            (0, 0, 54, 61, 5, 0, 0, 0),
            (0, 0, 9, 10, 1, 0, 0, 0),
            (0, 0, 6, 6, 1, 0, 0, 0),
            (2, 6, 3, 4, 1, 0, 4, 2),
        )
        fields = ("histories", "appends", "charts", "frames_upper", "domains_upper",
                  "anchor_projections", "tracking_projections", "combination_advances")
        for name, values in zip(names, expected, strict=True):
            self.assertEqual(tuple(rows[name]["fixture"][field] for field in fields), values)
        self.assertEqual(len(MODULE.case_recipes()), 28)
        self.assertEqual(len(MODULE.composition_case_recipes()), 4)

    def test_three_genuine_histories_are_the_only_new_numerical_producers(self):
        recipe = MODULE.achieved_phase_package_fixture_recipe()
        f = recipe["fixture"]
        expected = {"histories": 3, "appends": 9, "observations": 9, "coordinates": 18,
                    "exchanges": 9, "charts": 84, "frames_upper": 93, "domains_upper": 9,
                    "prepares": 9, "terms": 9, "anchor_projections": 5, "tracking_projections": 4,
                    "chart_enclosures": 9, "raw_enclosures": 2, "raw_batches": 1,
                    "chart_batches": 2, "combination_seeds": 1, "combination_advances": 2,
                    "state_equalities": 1, "state_clones": 1}
        self.assertEqual({field: f[field] for field in expected}, expected)
        delta = MODULE.fixture_delta_reservation(owners(), recipe, ordinary_outcomes=5)
        self.assertEqual(len(delta), 486)
        self.assertEqual(MODULE.numeric_work(delta), {"charged_fit_work_upper": 558,
                                                     "point_admissions_upper": 19269})
        # Chart-only rejection/monotonicity/overflow cases never produce fits,
        # despite funding their complete possible constructor/compose suffixes.
        for name in MODULE.ACHIEVED_PHASE_ORDINARY_TESTS[1:4]:
            vector = MODULE.fixture_delta_reservation(owners(), MODULE.achieved_phase_case_recipes()[name],
                                                      ordinary_outcomes=1)
            self.assertEqual(MODULE.numeric_work(vector), {"charged_fit_work_upper": 0,
                                                           "point_admissions_upper": 0})
        self.assertEqual(delta["chart_combination.reference_input_work.admissions"], 3)
        self.assertEqual(delta["chart_combination.reference_input_work.token_constructions"], 3)
        self.assertEqual(delta["chart_combination.reference_input_work.feedback"], 6)

    def test_historical_source_pins_and_six_uninvoked_documentation_cases(self):
        actual = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest()
                  for name in MODULE.ACHIEVED_PHASE_SOURCE_SHA256}
        self.assertEqual(len(actual), 23)
        frozen = dict(MODULE.ACHIEVED_PHASE_SOURCE_SHA256)
        self.assertEqual(MODULE.admit_achieved_phase_sources(frozen), frozen)
        # Rate delivery changed these source bytes later. The retained S1.4c
        # profile must continue rejecting them rather than retarget old pins.
        self.assertNotEqual(actual, frozen)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_achieved_phase_sources(actual)
        for bad in ({}, MODULE.COMPOSITION_SOURCE_SHA256, {**actual, "foreign": "0" * 64}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_achieved_phase_sources(bad)
        for field in actual:
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_achieved_phase_sources({**actual, field: "0" * 64})
        self.assertEqual(MODULE.ACHIEVED_PHASE_DOCTEST_LINES, (106, 436, 455, 473, 495, 517))
        self.assertEqual(MODULE.ACHIEVED_PHASE_COMPILE_FAIL_LINES, (106, 436, 455, 473))

    def test_named_outcomes_are_complete_lines_and_cannot_hide_missing_cases(self):
        names = MODULE.ACHIEVED_PHASE_ORDINARY_TESTS
        text = "".join("test " + name + " ... ok\n" for name in names)
        summary = MODULE.achieved_phase_test_observation(text, require_complete=True)
        self.assertTrue(summary["complete"])
        self.assertEqual(summary["expected"], 5)
        self.assertNotIn("observed_operation_counts", summary)
        self.assertTrue(MODULE.achieved_phase_test_observation(text.replace("\n", "\r\n"))["complete"])
        for bad in ("", text.replace(" ... ok", " ... FAILED", 1),
                    text.replace(" ... ok", " ... ignored", 1), "\n".join(text.splitlines()[1:]) + "\n"):
            self.assertFalse(MODULE.achieved_phase_test_observation(bad)["complete"])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_test_observation(bad, require_complete=True)
        for bad in (text[:-1], text + text.splitlines()[0] + "\n",
                    text.replace(names[0], "reference_input_readiness::achieved_phase_foreign"),
                    text.replace(" ... ok", " ... ok trailing", 1),
                    text.replace(" ... ok\n", " ... ok\rok\n", 1)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_test_observation(bad)
        older = "".join("test " + name + " ... ok\n" for name in MODULE.CURRENT_ORDINARY_TESTS)
        self.assertTrue(MODULE.source_test_outcomes(older + text, require_complete=True)["complete"])
        self.assertFalse(MODULE.achieved_phase_test_observation(older)["complete"])
        for invalid in (None, True, 1, "x" * (MODULE.MAX_OUTPUT_CHARACTERS + 1)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_test_observation(invalid)


if __name__ == "__main__":
    unittest.main()
