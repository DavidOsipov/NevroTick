"""Independent combination transport/source controls; no Rust/native execution."""
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location(
    "core_combination_accounting", Path(__file__).resolve().parents[1] / "core-combination-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

FIELDS = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections", "enclosures",
          "batches", "minima", "parameters", "strata", "seeds", "advances",
          "state_clones", "value_equalities", "state_formats", "uncertainties", "combinations")
ROWS = (
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
TOTAL = (43,129,129,43,145,516,129,977,43,43,43,14,14,14,43,14,15,0,0,5,132,60)


def row(case=ROWS[0][0], values=ROWS[0][1]):
    """Author a literal transport row without importing the production schema."""
    return "CORE_COMBINATION_PROFILE case=" + case + "".join(
        " " + name + "=" + str(value) for name, value in zip(FIELDS, values, strict=True)) + "\n"


class CombinationAccountingTests(unittest.TestCase):
    """Challenge the receiver independently of native assertion outcomes."""

    def test_all_twelve_literal_rows_have_exact_independent_totals(self):
        plain = "".join(row(case, values) for case, values in ROWS)
        prefixed = "".join("test combination::combination_cases::" + case + " ... " + row(case, values)
                           for case, values in ROWS)
        reversed_rows = "".join(row(case, values) for case, values in reversed(ROWS))
        for text in (plain, plain.replace("\n", "\r\n"), prefixed, reversed_rows):
            observed = MODULE.combination_counts(text, require_complete=True)
            self.assertEqual(observed["combination_counter_records"], 12)
            self.assertEqual(observed["combination_observed_calls"], dict(zip(FIELDS, TOTAL, strict=True)))
            self.assertEqual(observed["combination_missing_cases"], [])
            self.assertEqual(observed["combination_partial_cases"], [])
            self.assertFalse(observed["combination_observation_pending"])
            self.assertNotIn("native_outcome", observed)

    def test_missing_partial_and_complete_attempts_remain_distinct(self):
        missing = MODULE.combination_counts("CORE_PROFILE_COUNTS fits=1 points=3\n")
        self.assertEqual(missing["combination_counter_records"], 0)
        self.assertIsNone(missing["combination_recorded_call_subtotals"])
        self.assertIsNone(missing["combination_observed_calls"])
        partial = MODULE.combination_counts(row(values=(1,) + (0,) * 21))
        self.assertEqual(partial["combination_recorded_call_subtotals"]["histories"], 1)
        self.assertEqual(partial["combination_partial_cases"], [ROWS[0][0]])
        self.assertEqual(len(partial["combination_missing_cases"]), 11)
        self.assertIsNone(partial["combination_observed_calls"])
        for text in ("", row(), row(values=(1,) + (0,) * 21)):
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_counts(text, require_complete=True)

    def test_typed_projection_keeps_reserved_and_observed_attempts_distinct(self):
        complete = MODULE.combination_counts("".join(row(case, values) for case, values in ROWS))
        observed = MODULE.observation_attempts(complete)
        reserved = MODULE.reserved_attempts(MODULE.full_package_reservation())
        self.assertIsInstance(observed, MODULE.ObservedAttempts)
        self.assertIsInstance(reserved, MODULE.ReservedAttempts)
        self.assertIsNot(type(observed), type(reserved))
        self.assertEqual(observed.calls["combinations"], 60)
        self.assertEqual(reserved.calls["getters"], 977)
        observed.calls["getters"] = 0
        self.assertEqual(complete["combination_observed_calls"]["getters"], 977)
        self.assertIsNone(MODULE.observation_attempts(MODULE.combination_counts("")))
        self.assertIsNone(MODULE.observation_attempts(MODULE.combination_counts(row())))
        for incomplete in (MODULE.combination_counts(""), MODULE.combination_counts(row())):
            with self.subTest(incomplete=incomplete["combination_observation_status"]), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.complete_attempts(incomplete)

    def test_named_vector_projection_preserves_exact_integer_admission(self):
        self.assertEqual(MODULE._counts_from_vector(TOTAL), dict(zip(FIELDS, TOTAL, strict=True)))
        distinct = tuple(range(22))
        self.assertEqual(MODULE._counts_from_vector(distinct), dict(zip(FIELDS, distinct, strict=True)))
        for values in (TOTAL[:-1], (True,) + TOTAL[1:], (-1,) + TOTAL[1:],
                       (1 << 64,) + TOTAL[1:]):
            with self.subTest(values=values), self.assertRaises(MODULE.Incomplete):
                MODULE._counts_from_vector(values)

    def test_unknown_duplicate_truncated_reordered_and_noncanonical_rows_reject(self):
        good = row()
        invalid = (good + good, good.rstrip("\n"), good.replace(ROWS[0][0], "unowned"),
                   good.replace("histories=2 appends=6", "appends=6 histories=2"),
                   good.replace("histories=2", "histories=02"), good.replace("histories=2", "histories=-1"),
                   good.replace("histories=2", "histories=true"), good.replace("histories=2", "histories=2.0"),
                   good.replace("histories=2", "histories=+2"), good.replace("histories=2", "histories=18446744073709551616"),
                   good.replace(" uncertainties=2", ""), good.rstrip("\n") + " extra=0\n",
                   "replayed: " + good, good.replace("CORE_COMBINATION_PROFILE", "CORE_COMBINATION_UNKNOWN"),
                   '"' + good.replace("\n", r"\n") + '"\n', row(values=(0,) * 22),
                   " " * 2048 + good)
        for text in invalid:
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_counts(text)

    def test_exact_source_test_and_case_pair_is_required(self):
        good = "test combination::combination_cases::" + ROWS[0][0] + " ... " + row()
        self.assertEqual(MODULE.combination_counts(good)["combination_counter_records"], 1)
        for text in (good.replace("combination::combination_cases::", "unowned::"),
                     # First-row values fit the capacity-case ceiling, so this
                     # challenges prefix/case ownership rather than a bound.
                     "test combination::combination_cases::" + ROWS[0][0] + " ... " + row(ROWS[8][0]),
                     good.replace(" ... ", " ...  ")):
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_counts(text)

    def test_every_field_ceiling_rejects_its_next_integer(self):
        for index, field in enumerate(FIELDS):
            values = list(ROWS[0][1])
            values[index] += 1
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_counts(row(values=values))

    def test_bounded_text_and_boolean_completeness_modes_are_mandatory(self):
        for text in (None, [], 0):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_counts(text)
        for mode in (None, 0, 1, "true"):
            with self.subTest(mode=mode), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_counts(row(), require_complete=mode)
        with patch.object(MODULE, "MAX_OUTPUT_CHARACTERS", 3), self.assertRaises(MODULE.Incomplete):
            MODULE.combination_counts(row())

    def test_complete_source_reservation_cannot_shrink_after_failed_prefixes(self):
        before = MODULE.full_package_reservation()
        MODULE.combination_counts(row(values=(1,) + (0,) * 21))
        self.assertEqual(MODULE.full_package_reservation(), before)
        self.assertEqual(before["combination_reserved_calls"], dict(zip(FIELDS, TOTAL, strict=True)))
        self.assertEqual(before["combination_reserved_module_work_uppers"]["kernel_checked_binary_calls_upper"], 17580)
        self.assertEqual(before["combination_reserved_module_work_uppers"]["kernel_checked_square_root_calls_upper"], 180)

    def test_producer_projection_preserves_twenty_fields_without_double_charging(self):
        calls = dict(zip(FIELDS, TOTAL, strict=True))
        producer = MODULE.producer_calls(calls)
        self.assertEqual(producer, dict(zip(FIELDS[:20], TOTAL[:20], strict=True)))
        self.assertEqual(producer["appends"], 129)
        self.assertEqual(producer["getters"], 977)
        self.assertEqual(producer["state_formats"], 5)
        self.assertNotIn("uncertainties", producer)
        own = MODULE.source_work_uppers(calls)
        self.assertNotIn("charged_fit_work_upper", own)
        self.assertNotIn("point_admissions_upper", own)
        self.assertEqual(own["uncertainty_constructor_attempts_upper"], 132)
        self.assertEqual(own["uncertainty_garde_validation_calls_upper"], 132)
        self.assertEqual(own["uncertainty_garde_rule_callbacks_upper"], 396)
        self.assertEqual(own["uncertainty_predicate_calls_upper"], 792)
        self.assertEqual(own["kernel_final_uncertainty_predicate_calls_upper"], 180)
        self.assertEqual(own["kernel_positive_denominator_guards_upper"], 2880)
        self.assertEqual(own["kernel_record_lookup_visits_upper"], 15360)

    def test_call_and_work_values_reject_missing_unknown_boolean_negative_and_overflow(self):
        calls = dict(zip(FIELDS, TOTAL, strict=True))
        for bad in ({}, {**calls, "extra": 0}, {**calls, "combinations": True},
                    {**calls, "uncertainties": -1}, {**calls, "combinations": 1 << 64}):
            for function in (MODULE.source_work_uppers, MODULE.producer_calls):
                with self.subTest(bad=bad, function=function.__name__), self.assertRaises(MODULE.Incomplete):
                    function(bad)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.source_work_uppers({**calls, "combinations": (1 << 64) - 1})
        for value in (None, {}, {"work": True}, {"work": -1}, {"work": 1 << 64}):
            with patch.object(MODULE, "SOURCE_WORK_PER_UNCERTAINTY", value), self.assertRaises(MODULE.Incomplete):
                MODULE.source_work_uppers(calls)
        with patch.object(MODULE, "SOURCE_WORK_PER_UNCERTAINTY", {"shared": 1}), \
                patch.object(MODULE, "SOURCE_WORK_PER_COMBINATION", {"shared": 1}), \
                self.assertRaises(MODULE.Incomplete):
            MODULE.source_work_uppers(calls)

    def test_pending_or_malformed_source_inventory_fails_closed(self):
        for recipes in (None, (), (("bad",),), (("bad-name", ROWS[0][1]),),
                        ((ROWS[0][0], (0,) * 21),),
                        ((ROWS[0][0], (True,) + (0,) * 21),),
                        ((ROWS[0][0], (1 << 64,) + (0,) * 21),), (ROWS[0], ROWS[0])):
            names = tuple("controlled::case_" + str(index)
                          for index in range(len(recipes) if type(recipes) is tuple else 1))
            with patch.object(MODULE, "SOURCE_RECIPES", recipes), \
                    patch.object(MODULE, "SOURCE_TESTS", names), self.subTest(recipes=recipes), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.full_package_reservation()
        with patch.object(MODULE, "SOURCE_TESTS", ()), self.assertRaises(MODULE.Incomplete):
            MODULE.combination_counts(row())


LIFECYCLE_FIELDS = (
    "histories", "appends", "observations", "contexts", "monotonic", "coordinates",
    "exchanges", "getters", "terms", "projections", "enclosures", "batches", "minima",
    "principal_parameters", "strata", "combination_parameters", "seeds",
    "reachability_updates", "reachability_records", "advances", "uncertainties", "equalities",
)
LIFECYCLE_FIRST = "lifecycle_strict_boundaries_and_original_principal"
LIFECYCLE_ROWS = (
    (LIFECYCLE_FIRST, (21,63,63,21,73,126,63,373,21,21,21,10,10,10,21,10,10,0,0,10,21,0)),
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
BRIDGE_CASE = "bridge_receiving_and_mask_boundaries"
BRIDGE_TEST = "combination::admission::receiving_tests::" + BRIDGE_CASE
COUNTER_CASE = "lifecycle_counter_success_and_exhaustion"
COUNTER_TEST = "combination::lifecycle::counter_tests::" + COUNTER_CASE
BRIDGE_ROW = "CORE_LIFECYCLE_BRIDGE_PROFILE case=" + BRIDGE_CASE + " appends=6 admissions=5 masks=5 garde=4\n"
COUNTER_FIELDS = ("attempts", "source_constructions", "source_predicates", "garde", "fixture_setups",
                  "outcome_predicates", "estimate_predicates", "outcome_garde")
COUNTER_ROW = ("CORE_LIFECYCLE_COUNTER_PROFILE case=" + COUNTER_CASE
                + " attempts=4 source_constructions=24 source_predicates=24 garde=24"
               + " fixture_setups=1 outcome_predicates=34 estimate_predicates=18 outcome_garde=34\n")


def lifecycle_row(case=LIFECYCLE_FIRST, values=(1,) + (0,) * 21):
    """Independently specify transport names and a valid observed failed prefix."""
    return "CORE_COMBINATION_LIFECYCLE_PROFILE case=" + case + "".join(
        " " + name + "=" + str(value)
        for name, value in zip(LIFECYCLE_FIELDS, values, strict=True)) + "\n"


def complete_lifecycle_text():
    """The complete literal transport does not import implementation recipes."""
    return "".join(lifecycle_row(case, values) for case, values in LIFECYCLE_ROWS) + BRIDGE_ROW + COUNTER_ROW


def zero_bridge_caught_text(issue="OutOfRange", *, complete=False):
    """Independent inert bridge failure; no retained log or Native claim."""
    zero = ("CORE_LIFECYCLE_BRIDGE_PROFILE case=" + BRIDGE_CASE
            + " appends=0 admissions=0 masks=0 garde=0\n")
    rows = complete_lifecycle_text().replace(BRIDGE_ROW, zero) if complete else zero
    return (rows + "FAILED\n\nfailures:\n\n---- " + BRIDGE_TEST + " stdout ----\n"
            + "Error: Divisor { issue: " + issue + " }\n\nfailures:\n\n    "
            + BRIDGE_TEST + "\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored;\n")


class LifecycleDispatchTests(unittest.TestCase):
    """The kernel receiver must coexist with exact registered lifecycle rows."""

    def test_complete_kernel_stream_can_contain_valid_lifecycle_failed_prefix(self):
        text = "".join(row(case, values) for case, values in ROWS) + lifecycle_row()
        observed = MODULE.combination_counts(text, require_complete=True)
        self.assertEqual(observed["combination_observation_status"], "complete-source-recipe")
        self.assertEqual(observed["combination_observed_calls"]["combinations"], 60)

    def test_caught_zero_bridge_stays_partial_with_all_families_present(self):
        healthy = MODULE.lifecycle_counts(complete_lifecycle_text(), require_complete=True)
        self.assertEqual(healthy["lifecycle_observation_status"], "complete-source-recipe")
        for issue in ("OutOfRange", "NonFinite"):
            for newline in ("\n", "\r\n"):
                output = zero_bridge_caught_text(issue, complete=True).replace("\n", newline)
                context = MODULE.caught_bridge_zero_prefix(output)
                self.assertIsNotNone(context)
                observed = MODULE.lifecycle_counts(output, bridge_zero_prefix=context)
                self.assertEqual(observed["lifecycle_missing_cases"], [])
                self.assertEqual(observed["lifecycle_partial_cases"], [BRIDGE_CASE])
                self.assertEqual(observed["lifecycle_observation_status"], "partial-source-recipe")
                self.assertEqual(observed["lifecycle_bridge_profiles"],
                                 {BRIDGE_CASE: {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}})
                self.assertIsNone(observed["lifecycle_observed_bridge_calls"])
                self.assertIsNone(MODULE.lifecycle_observation_attempts(observed))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.complete_lifecycle_attempts(observed)

    def test_zero_bridge_requires_exact_current_context_and_never_complete_mode(self):
        output = zero_bridge_caught_text()
        context = MODULE.caught_bridge_zero_prefix(output)
        for function in (MODULE.lifecycle_counts, MODULE.combination_counts):
            with self.assertRaises(MODULE.Incomplete):
                function(output)
            with self.assertRaises(MODULE.Incomplete):
                function(output, require_complete=True, bridge_zero_prefix=context)
            for bad_context in (MODULE.CaughtBridgeZeroPrefix("0" * 64), True):
                with self.subTest(bad_context=bad_context), self.assertRaises(MODULE.Incomplete):
                    function(output, bridge_zero_prefix=bad_context)
            with self.assertRaises(MODULE.Incomplete):
                function(output + "changed-byte\n", bridge_zero_prefix=context)
        wrong = (output.replace("issue: OutOfRange", "issue: Unknown"),
                 output.replace("---- " + BRIDGE_TEST, "---- another_test"),
                 output.replace("    " + BRIDGE_TEST + "\n", "    another_test\n"))
        for value in wrong:
            with self.subTest(value=value[:90]):
                self.assertIsNone(MODULE.caught_bridge_zero_prefix(value))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.lifecycle_counts(value)

    def test_caught_bridge_context_keeps_other_rows_and_limits_closed(self):
        output = zero_bridge_caught_text()
        variants = (output + BRIDGE_ROW,
                    output + BRIDGE_ROW.replace(BRIDGE_CASE, "unknown"),
                    output + row(values=(3,) + ROWS[0][1][1:]),
                    output + row(values=(0,) * 22),
                    output + lifecycle_row(values=(0,) * 22),
                    output.replace(" masks=0", " unknown=0"),
                    output.replace(" garde=0\n", " garde=18446744073709551616\n"))
        for value in variants:
            context = MODULE.caught_bridge_zero_prefix(value)
            for function in (MODULE.lifecycle_counts, MODULE.combination_counts):
                with self.subTest(value=value[-120:], function=function.__name__), self.assertRaises(MODULE.Incomplete):
                    function(value, bridge_zero_prefix=context)

    def test_lifecycle_receiver_keeps_a_registered_failed_prefix_partial(self):
        observed = MODULE.lifecycle_counts(lifecycle_row())
        self.assertEqual(observed["lifecycle_counter_records"], 1)
        self.assertEqual(observed["lifecycle_recorded_call_subtotals"]["histories"], 1)
        self.assertIsNone(observed["lifecycle_observed_calls"])
        self.assertEqual(observed["lifecycle_observation_status"], "partial-source-recipe")

    def test_all_fourteen_literal_rows_and_mixed_kernel_stream_complete(self):
        plain = complete_lifecycle_text()
        prefixed = "".join("test " + case + " ... " + lifecycle_row(case, values)
                           for case, values in LIFECYCLE_ROWS)
        prefixed += "test " + BRIDGE_TEST + " ... " + BRIDGE_ROW
        prefixed += "test " + COUNTER_TEST + " ... " + COUNTER_ROW
        kernel = "".join(row(case, values) for case, values in ROWS)
        for text in (plain, plain.replace("\n", "\r\n"), prefixed, kernel + plain, plain + kernel):
            with self.subTest(prefix=text[:70]):
                observed = MODULE.lifecycle_counts(text, require_complete=True)
                self.assertEqual(observed["lifecycle_counter_records"], 14)
                self.assertEqual(observed["lifecycle_missing_cases"], [])
                self.assertEqual(observed["lifecycle_partial_cases"], [])
                self.assertEqual(observed["lifecycle_observed_calls"]["histories"], 57)
                self.assertEqual(observed["lifecycle_observed_calls"]["appends"], 232)
                self.assertEqual(observed["lifecycle_observed_calls"]["advances"], 167)
                self.assertEqual(observed["lifecycle_observed_bridge_calls"],
                                 {"appends": 6, "admissions": 5, "masks": 5, "garde": 4})
                self.assertEqual(observed["lifecycle_observed_counter_calls"],
                                 {"attempts": 4, "source_constructions": 24,
                                  "source_predicates": 24, "garde": 24, "fixture_setups": 1,
                                  "outcome_predicates": 34, "estimate_predicates": 18, "outcome_garde": 34})
                self.assertNotIn("native_outcome", observed)

    def test_missing_private_or_public_families_are_not_complete(self):
        missing = MODULE.lifecycle_counts("unrelated core output\n")
        self.assertEqual(missing["lifecycle_observation_status"], "missing")
        self.assertIsNone(missing["lifecycle_recorded_call_subtotals"])
        self.assertEqual(len(missing["lifecycle_missing_cases"]), 14)
        for text in (BRIDGE_ROW, COUNTER_ROW, BRIDGE_ROW + COUNTER_ROW,
                     complete_lifecycle_text().replace(BRIDGE_ROW, ""),
                     complete_lifecycle_text().replace(COUNTER_ROW, "")):
            with self.subTest(text=text[:80]):
                partial = MODULE.lifecycle_counts(text)
                self.assertEqual(partial["lifecycle_observation_status"], "partial-source-recipe")
                self.assertIsNone(partial["lifecycle_observed_calls"])
                self.assertIsNone(MODULE.lifecycle_observation_attempts(partial))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.lifecycle_counts(text, require_complete=True)

    def test_failed_prefix_and_ceilings_do_not_masquerade_as_exact_complete_recipe(self):
        raw = list(LIFECYCLE_ROWS[0][1])
        raw[7] += 1  # below the independent 6720 getter ceiling, above healthy373
        prefix = lifecycle_row(values=tuple(raw))
        partial = MODULE.lifecycle_counts(prefix)
        self.assertEqual(partial["lifecycle_partial_cases"], [LIFECYCLE_FIRST])
        self.assertEqual(partial["lifecycle_recorded_call_subtotals"]["getters"], 374)
        self.assertIsNone(partial["lifecycle_observed_calls"])
        reserved = MODULE.lifecycle_full_package_reservation()
        self.assertGreater(reserved["lifecycle_reserved_calls"]["getters"], 366)
        ceilings = (21,63,63,21,73,126,63,6720,21,21,21,10,10,10,21,10,10,0,0,10,21,0)
        for index, field in enumerate(LIFECYCLE_FIELDS):
            above = list(ceilings)
            above[index] += 1
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_counts(lifecycle_row(values=tuple(above)))

    def test_invalid_registered_and_unknown_rows_cannot_be_ignored_by_either_receiver(self):
        good = lifecycle_row()
        invalid = (good + good, good.rstrip("\n"), good.replace(LIFECYCLE_FIRST, "unowned"),
                   good.replace("histories=1 appends=0", "appends=0 histories=1"),
                   good.replace("histories=1", "histories=01"), good.replace("histories=1", "histories=-1"),
                   good.replace("histories=1", "histories=true"), good.replace("histories=1", "histories=1.0"),
                   good.replace("histories=1", "histories=+1"), good.replace("histories=1", "histories=18446744073709551616"),
                   good.replace(" uncertainties=0", ""), good.rstrip("\n") + " extra=0\n",
                   "replayed: " + good, good.replace("CORE_COMBINATION_LIFECYCLE_PROFILE", "CORE_COMBINATION_UNKNOWN"),
                   good.replace("CORE_COMBINATION_LIFECYCLE_PROFILE", "CORE_LIFECYCLE_UNKNOWN"),
                   "test unowned ... " + good, '"' + good.replace("\n", r"\n") + '"\n',
                   lifecycle_row(values=(0,) * 22), " " * 2048 + good,
                   BRIDGE_ROW.replace("masks=5", "masks=true"), BRIDGE_ROW + BRIDGE_ROW,
                   BRIDGE_ROW.replace("admissions=5", "admissions=6"),
                   "test " + COUNTER_TEST + " ... " + BRIDGE_ROW,
                   COUNTER_ROW.replace("attempts=4", "attempts=5"), COUNTER_ROW + COUNTER_ROW,
                   COUNTER_ROW.replace("attempts=4 source_constructions=24 source_predicates=24 garde=24",
                                       "attempts=0 source_constructions=0 source_predicates=0 garde=0")
                              .replace("fixture_setups=1 outcome_predicates=34 estimate_predicates=18 outcome_garde=34",
                                       "fixture_setups=0 outcome_predicates=0 estimate_predicates=0 outcome_garde=0"),
                   COUNTER_ROW.rstrip("\n"))
        kernel = "".join(row(case, values) for case, values in ROWS)
        for text in invalid:
            for function in (MODULE.lifecycle_counts, MODULE.combination_counts):
                with self.subTest(text=text[:90], function=function.__name__), self.assertRaises(MODULE.Incomplete):
                    function(kernel + text)

    def test_exact_runtime_vectors_preserve_nominal_observed_reserved_units(self):
        observation = MODULE.lifecycle_counts(complete_lifecycle_text(), require_complete=True)
        observed = MODULE.complete_lifecycle_attempts(observation)
        reservation = MODULE.lifecycle_full_package_reservation()
        reserved = MODULE.reserved_lifecycle_attempts(reservation)
        self.assertIs(type(observed), MODULE.ObservedLifecycleAttempts)
        self.assertIs(type(reserved), MODULE.ReservedLifecycleAttempts)
        self.assertIsNot(type(observed), type(reserved))
        self.assertEqual(observed.calls["appends"], reserved.calls["appends"])
        self.assertLess(observed.calls["getters"], reserved.calls["getters"])
        observed.calls["appends"] = 0
        self.assertEqual(observation["lifecycle_observed_calls"]["appends"], 232)
        for record in (MODULE.lifecycle_counts(""), MODULE.lifecycle_counts(lifecycle_row())):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.complete_lifecycle_attempts(record)
        raw = dict(zip(LIFECYCLE_FIELDS, LIFECYCLE_ROWS[0][1], strict=True))
        zero_bridge = {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}
        for bad in ({}, {**raw, "extra": 0}, {**raw, "advances": True},
                    {**raw, "equalities": -1}, {**raw, "seeds": 1 << 64}):
            for function in (MODULE.lifecycle_producer_calls,
                             lambda value: MODULE.lifecycle_source_work_uppers(value, zero_bridge, dict.fromkeys(COUNTER_FIELDS, 0))):
                with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                    function(bad)
        for bad in ({}, {**zero_bridge, "garde": True}, {**zero_bridge, "extra": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_source_work_uppers(raw, bad, dict.fromkeys(COUNTER_FIELDS, 0))
        zero_counter = dict.fromkeys(COUNTER_FIELDS, 0)
        for field in COUNTER_FIELDS:
            missing = {key: value for key, value in zero_counter.items() if key != field}
            invalid = (missing, {**zero_counter, "extra": 0},
                       *({**zero_counter, field: value}
                         for value in (True, -1, 1.0, 1 << 64)))
            for bad in invalid:
                with self.subTest(field=field, bad=bad), self.assertRaises(MODULE.Incomplete):
                    MODULE.lifecycle_source_work_uppers(raw, zero_bridge, bad)

    def test_counter_source_matrix_prefix_is_partial_and_full_recipe_is_complete(self):
        prefix = (COUNTER_ROW.replace("source_constructions=24 source_predicates=24 garde=24",
                                      "source_constructions=12 source_predicates=12 garde=12")
                  .replace("fixture_setups=1 outcome_predicates=34 estimate_predicates=18 outcome_garde=34",
                           "fixture_setups=0 outcome_predicates=0 estimate_predicates=0 outcome_garde=0"))
        text = complete_lifecycle_text().replace(COUNTER_ROW, prefix)
        partial = MODULE.lifecycle_counts(text)
        self.assertEqual(partial["lifecycle_partial_cases"], [COUNTER_CASE])
        self.assertEqual(partial["lifecycle_recorded_counter_subtotals"],
                         {"attempts": 4, "source_constructions": 12,
                          "source_predicates": 12, "garde": 12, "fixture_setups": 0,
                          "outcome_predicates": 0, "estimate_predicates": 0, "outcome_garde": 0})
        self.assertIsNone(partial["lifecycle_observed_counter_calls"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.lifecycle_counts(text, require_complete=True)
        reserved = MODULE.lifecycle_full_package_reservation()
        self.assertEqual(reserved["lifecycle_reserved_counter_calls"],
                         {"attempts": 4, "source_constructions": 24,
                          "source_predicates": 24, "garde": 24, "fixture_setups": 1,
                                  "outcome_predicates": 34, "estimate_predicates": 18, "outcome_garde": 34})
        self.assertEqual(MODULE.lifecycle_counts(complete_lifecycle_text(), require_complete=True)
                         ["lifecycle_observed_counter_calls"], reserved["lifecycle_reserved_counter_calls"])

    def test_counter_source_matrix_transport_is_closed_and_individually_bounded(self):
        kernel = "".join(row(case, values) for case, values in ROWS)
        invalid = ["CORE_LIFECYCLE_COUNTER_PROFILE case=" + COUNTER_CASE + " attempts=4\n",
                   "CORE_LIFECYCLE_COUNTER_PROFILE case=" + COUNTER_CASE
                   + " attempts=4 source_constructions=24 source_predicates=24 garde=24\n",
                   COUNTER_ROW.replace(" source_constructions=24 source_predicates=24 garde=24", ""),
                   COUNTER_ROW.replace("attempts=4 source_constructions=24",
                                       "source_constructions=24 attempts=4"),
                   COUNTER_ROW.replace(" garde=24", " garde=24 unknown=0"),
                   COUNTER_ROW + COUNTER_ROW, COUNTER_ROW.rstrip("\n")]
        for field, value in zip(COUNTER_FIELDS, (4, 24, 24, 24, 1, 34, 18, 34), strict=True):
            token = " " + field + "=" + str(value)
            invalid.extend((COUNTER_ROW.replace(token, ""),
                            COUNTER_ROW.replace(token, token + token),
                            *(COUNTER_ROW.replace(token, " " + field + "=" + bad)
                              for bad in (str(value + 1), "-1", "true", "1.0", "+1",
                                          "18446744073709551616"))))
        for text in invalid:
            for function in (MODULE.lifecycle_counts, MODULE.combination_counts):
                with self.subTest(row=text[:120], receiver=function.__name__), \
                        self.assertRaises(MODULE.Incomplete):
                    function(kernel + text)

    def test_private_source_counter_work_counts_direct_and_nested_requests_once(self):
        zero = dict.fromkeys(LIFECYCLE_FIELDS, 0)
        bridge = {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}
        counter = dict.fromkeys(COUNTER_FIELDS, 0)
        cases = (
            ({**counter, "source_constructions": 3}, (3, 3, 6, 0, 24)),
            ({**counter, "source_predicates": 5}, (0, 0, 5, 0, 0)),
            ({**counter, "garde": 7}, (7, 0, 7, 0, 0)),
            ({**counter, "attempts": 4}, (0, 0, 0, 4, 0)),
            ({**counter, "attempts": 4, "source_constructions": 24,
              "source_predicates": 24, "garde": 24}, (48, 24, 96, 4, 192)),
        )
        fields = ("lifecycle_garde_validate_calls_upper",
                  "lifecycle_source_counter_constructions_upper",
                  "lifecycle_source_counter_predicate_calls_upper",
                  "lifecycle_checked_next_calls_upper",
                  "lifecycle_internal_product_getter_requests_upper")
        for calls, expected in cases:
            with self.subTest(calls=calls):
                work = MODULE.lifecycle_source_work_uppers(zero, bridge, calls)
                self.assertEqual(tuple(work[field] for field in fields), expected)
                self.assertTrue(all(value == 0 for field, value in work.items() if field not in fields))
                self.assertNotIn("charged_fit_work_upper", work)
                self.assertNotIn("point_admissions_upper", work)

    def test_raw_parameter_comparisons_have_independent_observed_and_reserved_counts(self):
        case = "lifecycle_raw_parameter_and_ordinal_admission_neighbors"
        observed = MODULE.lifecycle_counts(complete_lifecycle_text(), require_complete=True)
        calls = observed["lifecycle_profiles"][case]
        self.assertEqual((calls["combination_parameters"], calls["getters"], calls["equalities"]),
                         (21, 24, 6))
        ceiling = MODULE.lifecycle_case_ceilings()[case]
        self.assertEqual((ceiling["combination_parameters"], ceiling["equalities"]), (21, 6))
        self.assertGreaterEqual(ceiling["getters"], 24)

    def test_guard_fixture_has_a_separate_fixed_source_reservation(self):
        expected = (4,12,12,4,24,4,4,4096,6,6,6,6,11,11,16,11,11,0,0,12,16,0)
        calls = MODULE.lifecycle_guard_fixture_calls()
        self.assertEqual(tuple(calls[field] for field in LIFECYCLE_FIELDS), expected)
        producer = MODULE.lifecycle_guard_fixture_producer_reservation()
        self.assertEqual(producer, {
            "histories": 4, "appends": 12, "observations": 12, "contexts": 4,
            "monotonic": 24, "coordinates": 4, "exchanges": 4, "getters": 4096,
            "terms": 6, "projections": 6, "enclosures": 6, "batches": 6,
            "minima": 11, "parameters": 11, "strata": 16, "seeds": 11,
            "advances": 12, "state_clones": 15, "value_equalities": 0, "state_formats": 0,
        })
        calls["appends"] = 0
        producer["state_clones"] = 0
        self.assertEqual(MODULE.lifecycle_guard_fixture_calls()["appends"], 12)
        self.assertEqual(MODULE.lifecycle_guard_fixture_producer_reservation()["state_clones"], 15)

    def test_direct_outcome_and_estimate_work_does_not_repeat_fixture_producers(self):
        zero = dict.fromkeys(LIFECYCLE_FIELDS, 0)
        bridge = {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}
        counter = dict.fromkeys(COUNTER_FIELDS, 0)
        raw = {**counter, "outcome_predicates": 34, "outcome_garde": 34, "estimate_predicates": 18}
        work = MODULE.lifecycle_source_work_uppers(zero, bridge, raw)
        self.assertEqual(work["lifecycle_output_predicate_calls_upper"], 68)
        self.assertEqual(work["lifecycle_garde_validate_calls_upper"], 34)
        self.assertEqual(work["lifecycle_internal_product_getter_requests_upper"], 704512)
        self.assertEqual(work["lifecycle_witness_membership_label_probes_upper"], 17408)
        self.assertEqual(work["lifecycle_output_contributor_label_probes_upper"], 17408)
        self.assertEqual(work["lifecycle_checked_next_calls_upper"], 0)
        self.assertEqual(work["bridge_reduction_entry_attempts_upper"], 0)
        self.assertEqual(work["lifecycle_source_counter_constructions_upper"], 0)
        full = {"attempts": 4, "source_constructions": 24, "source_predicates": 24, "garde": 24,
                "fixture_setups": 1, "outcome_predicates": 34, "estimate_predicates": 18, "outcome_garde": 34}
        complete = MODULE.lifecycle_source_work_uppers(zero, bridge, full)
        self.assertEqual(complete["lifecycle_output_predicate_calls_upper"], 92)
        self.assertEqual(complete["lifecycle_garde_validate_calls_upper"], 857)
        self.assertEqual(complete["lifecycle_source_counter_constructions_upper"], 776)
        self.assertEqual(complete["lifecycle_source_counter_predicate_calls_upper"], 1600)
        self.assertEqual(complete["lifecycle_checked_next_calls_upper"], 16)
        self.assertEqual(complete["lifecycle_staged_source_copy_cells_upper"], 240)
        self.assertEqual(complete["lifecycle_internal_product_getter_requests_upper"], 803184)
        self.assertEqual(complete["bridge_reduction_entry_attempts_upper"], 12)
        self.assertNotIn("charged_fit_work_upper", complete)
        self.assertNotIn("point_admissions_upper", complete)

    def test_capacity_source_recipe_preserves_exact_calls_and_larger_ceilings(self):
        case = "lifecycle_capacity_neighbors_preserve_pending_credit"
        exact = MODULE.lifecycle_case_recipes()[case]
        ceiling = MODULE.lifecycle_case_ceilings()[case]
        self.assertEqual((exact["seeds"], exact["principal_parameters"], exact["combination_parameters"],
                          exact["minima"], exact["getters"], exact["advances"]), (4, 1, 1, 1, 264, 6))
        self.assertEqual((ceiling["seeds"], ceiling["principal_parameters"], ceiling["combination_parameters"],
                          ceiling["minima"], ceiling["getters"], ceiling["advances"]), (4, 4, 4, 4, 4800, 6))

    def test_source_reservation_charges_inherited_work_once_and_module_units_separately(self):
        reserved = MODULE.lifecycle_full_package_reservation()
        calls = reserved["lifecycle_reserved_calls"]
        self.assertEqual(calls["appends"], 232)
        self.assertEqual(calls["getters"], 43712)
        self.assertEqual(calls["advances"], 167)
        producer = MODULE.lifecycle_producer_calls(calls)
        self.assertEqual(producer["appends"], 232)
        self.assertEqual(producer["state_clones"], 167)
        self.assertEqual(producer["parameters"], 31)
        self.assertNotIn("combination_parameters", producer)
        private = MODULE.lifecycle_bridge_producer_reservation()
        self.assertEqual(private["appends"], 6)
        self.assertEqual(private["histories"], 2)
        self.assertEqual(private["projections"], 2)
        own = reserved["lifecycle_reserved_module_work_uppers"]
        self.assertNotIn("charged_fit_work_upper", own)
        self.assertNotIn("point_admissions_upper", own)
        self.assertEqual(own["lifecycle_compatibility_checked_binary_calls_upper"], 11098)
        self.assertEqual(own["lifecycle_checked_next_calls_upper"], 267)
        self.assertEqual(own["lifecycle_garde_validate_calls_upper"], 9721)
        self.assertEqual(own["lifecycle_source_counter_constructions_upper"], 9330)
        self.assertEqual(own["lifecycle_source_counter_predicate_calls_upper"], 18708)
        self.assertEqual(own["bridge_complete_receiving_passes_upper"], 372)
        self.assertEqual(own["uncertainty_constructor_attempts_upper"], 218)
        zero = dict.fromkeys(LIFECYCLE_FIELDS, 0)
        zero_work = MODULE.lifecycle_source_work_uppers(
            zero, {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}, dict.fromkeys(COUNTER_FIELDS, 0))
        self.assertTrue(all(value == 0 for value in zero_work.values()))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.lifecycle_source_work_uppers({**zero, "advances": (1 << 64) - 1},
                {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}, dict.fromkeys(COUNTER_FIELDS, 0))

    def test_pending_or_malformed_new_source_metadata_fails_closed(self):
        for recipes in (None, (), (("bad",),), (("bad-name", (1,) * 22),),
                        (("good", (True,) + (0,) * 21),),
                        ((LIFECYCLE_FIRST, LIFECYCLE_ROWS[0][1]),) * 2):
            with patch.object(MODULE, "LIFECYCLE_SOURCE_RECIPES", recipes), self.subTest(recipes=recipes), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_full_package_reservation()
        for values in ((4,), (4, 24, 24, 24), (4, 24, 24, 24, True, 34, 18, 34),
                       (4, 24, 24, 24, 1, 34, 18, 1 << 64), (0,) * 8):
            recipes = ((COUNTER_CASE, values),)
            with patch.object(MODULE, "LIFECYCLE_COUNTER_SOURCE_RECIPES", recipes), \
                    self.subTest(counter_values=values), self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_full_package_reservation()
        with patch.object(MODULE, "LIFECYCLE_SOURCE_TESTS", ()), self.assertRaises(MODULE.Incomplete):
            MODULE.lifecycle_counts(complete_lifecycle_text())
        with patch.object(MODULE, "LIFECYCLE_SOURCE_LIMITS", ()), self.assertRaises(MODULE.Incomplete):
            MODULE.combination_counts(row())


if __name__ == "__main__":
    unittest.main()
