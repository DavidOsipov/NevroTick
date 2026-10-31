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
    ("lifecycle_raw_parameter_and_ordinal_admission_neighbors", (0,0,0,0,0,0,0,22,0,0,0,0,0,0,0,20,0,6,0,0,0,0)),
    ("lifecycle_distance_limit_overflow_rolls_back_and_recovers", (2,7,7,2,10,14,7,80,6,6,6,3,2,2,6,2,2,0,0,7,6,4)),
    ("lifecycle_skew_sum_overflow_rolls_back_and_recovers", (2,7,7,2,9,14,7,78,4,4,4,2,2,2,4,2,2,0,0,7,6,4)),
    ("lifecycle_max_clock_error_sum_overflow_rolls_back_and_recovers", (2,7,7,2,9,14,7,78,4,4,4,2,2,2,4,2,2,0,0,7,6,4)),
)
BRIDGE_CASE = "bridge_receiving_and_mask_boundaries"
BRIDGE_TEST = "combination::admission::receiving_tests::" + BRIDGE_CASE
COUNTER_CASE = "lifecycle_counter_success_and_exhaustion"
COUNTER_TEST = "combination::lifecycle::counter_tests::" + COUNTER_CASE
BRIDGE_ROW = "CORE_LIFECYCLE_BRIDGE_PROFILE case=" + BRIDGE_CASE + " appends=6 admissions=5 masks=5 garde=4\n"
COUNTER_ROW = "CORE_LIFECYCLE_COUNTER_PROFILE case=" + COUNTER_CASE + " attempts=4\n"


def lifecycle_row(case=LIFECYCLE_FIRST, values=(1,) + (0,) * 21):
    """Independently specify transport names and a valid observed failed prefix."""
    return "CORE_COMBINATION_LIFECYCLE_PROFILE case=" + case + "".join(
        " " + name + "=" + str(value)
        for name, value in zip(LIFECYCLE_FIELDS, values, strict=True)) + "\n"


def complete_lifecycle_text():
    """The complete literal transport does not import implementation recipes."""
    return "".join(lifecycle_row(case, values) for case, values in LIFECYCLE_ROWS) + BRIDGE_ROW + COUNTER_ROW


class LifecycleDispatchTests(unittest.TestCase):
    """The kernel receiver must coexist with exact registered lifecycle rows."""

    def test_complete_kernel_stream_can_contain_valid_lifecycle_failed_prefix(self):
        text = "".join(row(case, values) for case, values in ROWS) + lifecycle_row()
        observed = MODULE.combination_counts(text, require_complete=True)
        self.assertEqual(observed["combination_observation_status"], "complete-source-recipe")
        self.assertEqual(observed["combination_observed_calls"]["combinations"], 60)

    def test_lifecycle_receiver_keeps_a_registered_failed_prefix_partial(self):
        observed = MODULE.lifecycle_counts(lifecycle_row())
        self.assertEqual(observed["lifecycle_counter_records"], 1)
        self.assertEqual(observed["lifecycle_recorded_call_subtotals"]["histories"], 1)
        self.assertIsNone(observed["lifecycle_observed_calls"])
        self.assertEqual(observed["lifecycle_observation_status"], "partial-source-recipe")

    def test_all_thirteen_literal_rows_and_mixed_kernel_stream_complete(self):
        plain = complete_lifecycle_text()
        prefixed = "".join("test " + case + " ... " + lifecycle_row(case, values)
                           for case, values in LIFECYCLE_ROWS)
        prefixed += "test " + BRIDGE_TEST + " ... " + BRIDGE_ROW
        prefixed += "test " + COUNTER_TEST + " ... " + COUNTER_ROW
        kernel = "".join(row(case, values) for case, values in ROWS)
        for text in (plain, plain.replace("\n", "\r\n"), prefixed, kernel + plain, plain + kernel):
            with self.subTest(prefix=text[:70]):
                observed = MODULE.lifecycle_counts(text, require_complete=True)
                self.assertEqual(observed["lifecycle_counter_records"], 13)
                self.assertEqual(observed["lifecycle_missing_cases"], [])
                self.assertEqual(observed["lifecycle_partial_cases"], [])
                self.assertEqual(observed["lifecycle_observed_calls"]["histories"], 41)
                self.assertEqual(observed["lifecycle_observed_calls"]["appends"], 184)
                self.assertEqual(observed["lifecycle_observed_calls"]["advances"], 161)
                self.assertEqual(observed["lifecycle_observed_bridge_calls"],
                                 {"appends": 6, "admissions": 5, "masks": 5, "garde": 4})
                self.assertEqual(observed["lifecycle_observed_counter_calls"], {"attempts": 4})
                self.assertNotIn("native_outcome", observed)

    def test_missing_private_or_public_families_are_not_complete(self):
        missing = MODULE.lifecycle_counts("unrelated core output\n")
        self.assertEqual(missing["lifecycle_observation_status"], "missing")
        self.assertIsNone(missing["lifecycle_recorded_call_subtotals"])
        self.assertEqual(len(missing["lifecycle_missing_cases"]), 13)
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
        self.assertEqual(partial["lifecycle_recorded_call_subtotals"]["getters"], 366)
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
                   COUNTER_ROW.replace("attempts=4", "attempts=0"), COUNTER_ROW.rstrip("\n"))
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
        self.assertEqual(observation["lifecycle_observed_calls"]["appends"], 184)
        for record in (MODULE.lifecycle_counts(""), MODULE.lifecycle_counts(lifecycle_row())):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.complete_lifecycle_attempts(record)
        raw = dict(zip(LIFECYCLE_FIELDS, LIFECYCLE_ROWS[0][1], strict=True))
        zero_bridge = {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}
        for bad in ({}, {**raw, "extra": 0}, {**raw, "advances": True},
                    {**raw, "equalities": -1}, {**raw, "seeds": 1 << 64}):
            for function in (MODULE.lifecycle_producer_calls,
                             lambda value: MODULE.lifecycle_source_work_uppers(value, zero_bridge, {"attempts": 0})):
                with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                    function(bad)
        for bad in ({}, {**zero_bridge, "garde": True}, {**zero_bridge, "extra": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_source_work_uppers(raw, bad, {"attempts": 0})
        for bad in ({}, {"attempts": True}, {"attempts": -1}, {"attempts": 1 << 64}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_source_work_uppers(raw, zero_bridge, bad)

    def test_source_reservation_charges_inherited_work_once_and_module_units_separately(self):
        reserved = MODULE.lifecycle_full_package_reservation()
        calls = reserved["lifecycle_reserved_calls"]
        self.assertEqual(calls["appends"], 184)
        self.assertEqual(calls["getters"], 38912)
        self.assertEqual(calls["advances"], 161)
        producer = MODULE.lifecycle_producer_calls(calls)
        self.assertEqual(producer["appends"], 184)
        self.assertEqual(producer["state_clones"], 161)
        self.assertEqual(producer["parameters"], 27)
        self.assertNotIn("combination_parameters", producer)
        private = MODULE.lifecycle_bridge_producer_reservation()
        self.assertEqual(private["appends"], 6)
        self.assertEqual(private["histories"], 2)
        self.assertEqual(private["projections"], 2)
        own = reserved["lifecycle_reserved_module_work_uppers"]
        self.assertNotIn("charged_fit_work_upper", own)
        self.assertNotIn("point_admissions_upper", own)
        self.assertEqual(own["lifecycle_compatibility_checked_binary_calls_upper"], 9982)
        self.assertEqual(own["lifecycle_checked_next_calls_upper"], 249)
        self.assertEqual(own["bridge_complete_receiving_passes_upper"], 336)
        self.assertEqual(own["uncertainty_constructor_attempts_upper"], 186)
        zero = dict.fromkeys(LIFECYCLE_FIELDS, 0)
        zero_work = MODULE.lifecycle_source_work_uppers(
            zero, {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}, {"attempts": 0})
        self.assertTrue(all(value == 0 for value in zero_work.values()))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.lifecycle_source_work_uppers({**zero, "advances": (1 << 64) - 1},
                {"appends": 0, "admissions": 0, "masks": 0, "garde": 0}, {"attempts": 0})

    def test_pending_or_malformed_new_source_metadata_fails_closed(self):
        for recipes in (None, (), (("bad",),), (("bad-name", (1,) * 22),),
                        (("good", (True,) + (0,) * 21),),
                        ((LIFECYCLE_FIRST, LIFECYCLE_ROWS[0][1]),) * 2):
            with patch.object(MODULE, "LIFECYCLE_SOURCE_RECIPES", recipes), self.subTest(recipes=recipes), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.lifecycle_full_package_reservation()
        with patch.object(MODULE, "LIFECYCLE_SOURCE_TESTS", ()), self.assertRaises(MODULE.Incomplete):
            MODULE.lifecycle_counts(complete_lifecycle_text())
        with patch.object(MODULE, "LIFECYCLE_SOURCE_LIMITS", ()), self.assertRaises(MODULE.Incomplete):
            MODULE.combination_counts(row())


if __name__ == "__main__":
    unittest.main()
