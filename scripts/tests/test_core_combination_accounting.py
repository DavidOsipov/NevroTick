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


if __name__ == "__main__":
    unittest.main()
