"""Fail-closed transport and disjoint source reservation controls."""
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "core_adaptive_accounting", Path(__file__).resolve().parents[1] / "core-adaptive-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

# Independent fixed literal transport: not copied from a running helper's output.
ROWS = (
    "CORE_RETAINED_RUNS_PROFILE case=retained_prefix_preserves_second_round_information points=37 inputs=3 selections=3",
    "CORE_RETAINED_RUNS_PROFILE case=zero_prefix_preserves_legacy_selection points=13 inputs=1 selections=2",
    "CORE_RETAINED_RUNS_PROFILE case=invalid_prefix_and_minimum_boundaries points=11 inputs=2 selections=7",
    "CORE_RETAINED_RUNS_PROFILE case=maximum_combined_window_and_prefix_boundary points=64 inputs=1 selections=2",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=adaptive_retains_decision_prefix_across_two_rounds observations=13 appends=13 histories=1",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=configured_minimum_warms_up_without_claiming_eligibility observations=3 appends=3 histories=6",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=rejected_appends_preserve_complete_state_and_recover observations=20 appends=20 histories=1",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=selector_failure_preserves_cause_and_committed_history observations=13 appends=13 histories=1",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=adaptive_successful_compaction_preserves_decision_rows observations=12 appends=12 histories=1",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=adaptive_minimum_fallback_commits_full_checked_window observations=9 appends=9 histories=1",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=full_history_evicts_retained_auxiliary_first observations=65 appends=65 histories=1",
    "CORE_ADAPTIVE_HISTORY_PROFILE case=full_fitted_history_evicts_oldest_fitted_row observations=65 appends=65 histories=1",
    "CORE_LIFECYCLE_RAW_VALIDATION_PROFILE case=lifecycle_raw_parameter_validation validations=26",
    "CORE_LIFECYCLE_RAW_VALIDATION_PROFILE case=lifecycle_raw_ordinal_validation validations=3",
    "CORE_LIFECYCLE_RAW_VALIDATION_PROFILE case=lifecycle_raw_event_validation validations=8",
)
COMPLETE_PROFILE = "\n".join(ROWS) + "\n"


def stream(rows=ROWS):
    """Build a fixed native-style line stream for one isolated test."""
    return "".join(row + "\n" for row in rows)


class AdaptiveAccountingTests(unittest.TestCase):
    """The independently stated rows establish parser and reservation mechanics."""

    def test_complete_source_counts_and_indirect_work(self):
        result = MODULE.package_counts(stream(), require_complete=True)
        self.assertTrue(result["adaptive_observation_complete"])
        self.assertEqual(result["adaptive_missing_cases"], [])
        self.assertEqual(result["adaptive_partial_cases"], [])
        calls = result["adaptive_observed_calls"]
        self.assertEqual(calls, {
            "retained_points": 125, "retained_inputs": 7, "retained_selections": 14,
            "adaptive_observations": 200, "adaptive_appends": 200, "adaptive_histories": 13,
            "raw_parameter_validations": 26, "raw_ordinal_validations": 3, "raw_event_validations": 8,
            "external_normalizations": 0,
        })
        work = result["adaptive_recorded_source_work_upper"]
        self.assertEqual(work["selector_entries_upper"], 239)
        self.assertEqual(work["charged_fit_work_upper"], 14818)
        self.assertEqual(work["selector_point_copy_upper"], 496403)
        self.assertEqual(work["point_admissions_upper"], 510928)
        self.assertEqual(work["selector_decision_residuals_upper"], 740422)
        self.assertEqual(work["selector_checked_float_upper"], 12796060)
        self.assertEqual(work["adaptive_minimum_validations_upper"], 26)
        self.assertEqual(work["adaptive_minimum_predicates_upper"], 46)
        self.assertEqual(work["adaptive_error_display_requests_upper"], 7)
        self.assertEqual(work["adaptive_error_nested_display_calls_upper"], 2)
        self.assertEqual(work["adaptive_error_source_requests_upper"], 9)
        self.assertEqual(work["adaptive_error_source_downcasts_upper"], 3)
        self.assertEqual(work["raw_lifecycle_validations_upper"], 37)
        self.assertEqual(work["raw_lifecycle_rule_callbacks_upper"], 63)
        self.assertEqual(work["raw_event_update_constructions_upper"], 9)
        self.assertEqual(work["raw_event_fixture_entries_upper"], 1)
        self.assertEqual(work["adaptive_compaction_copies_upper"], 14175)
        self.assertEqual(work["adaptive_private_prefix_calls_upper"], 11)
        self.assertEqual(work["adaptive_private_final_validator_calls_upper"], 24)
        self.assertEqual(work["adaptive_private_context_checks_upper"], 83)
        self.assertEqual(work["adaptive_private_order_checks_upper"], 59)
        self.assertEqual(work["adaptive_private_observation_constructions_upper"], 29)
        self.assertEqual(work["adaptive_private_append_calls_upper"], 25)
        self.assertEqual(work["adaptive_private_history_constructions_upper"], 7)
        self.assertEqual(work["adaptive_private_array_init_slots_upper"], 128)
        self.assertEqual(work["adaptive_private_array_row_writes_upper"], 2)
        self.assertEqual(work["adaptive_private_array_transfer_slots_upper"], 256)

    def test_package_reservation_is_separate_and_flat(self):
        reservation = MODULE.full_package_reservation()
        self.assertEqual(reservation["adaptive.call.adaptive_appends"], 200)
        self.assertEqual(reservation["adaptive.call.retained_selections"], 14)
        self.assertEqual(reservation["adaptive.work.charged_fit_work_upper"], 14818)
        self.assertEqual(reservation["adaptive.work.point_admissions_upper"], 510928)
        self.assertTrue(all(type(value) is int and value >= 0 for value in reservation.values()))
        self.assertEqual(len(MODULE.SOURCE_TESTS), 23)
        self.assertEqual(len(set(MODULE.SOURCE_TESTS)), 23)
        self.assertEqual(len(MODULE.SOURCE_ONLY_TESTS), 8)
        self.assertTrue(set(MODULE.SOURCE_ONLY_TESTS) <= set(MODULE.SOURCE_TESTS))

    def test_missing_and_partial_are_not_observed_zero_or_complete(self):
        empty = MODULE.package_counts("")
        self.assertIsNone(empty["adaptive_recorded_calls"])
        self.assertIsNone(empty["adaptive_observed_calls"])
        self.assertIsNone(empty["adaptive_recorded_source_work_upper"])
        self.assertFalse(empty["adaptive_observation_complete"])
        partial = MODULE.package_counts(ROWS[0].replace("points=37", "points=36") + "\n")
        self.assertIsNone(partial["adaptive_observed_calls"])
        self.assertEqual(partial["adaptive_partial_cases"], ["retained_prefix_preserves_second_round_information"])
        self.assertTrue(partial["adaptive_missing_cases"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts(ROWS[0] + "\n", require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts(stream((ROWS[0].replace("points=37", "points=36"),) + ROWS[1:]), require_complete=True)

    def test_native_prefix_must_identify_same_case(self):
        case = "retained_prefix_preserves_second_round_information"
        self.assertIn(case, MODULE.package_counts("test " + case + " ... " + ROWS[0] + "\n")["adaptive_profiles"])
        self.assertIn(case, MODULE.package_counts("test retained_runs::" + case + " ... " + ROWS[0] + "\n")["adaptive_profiles"])
        raw_name = "combination::lifecycle::counter_tests::lifecycle_raw_event_validation"
        self.assertIn("lifecycle_raw_event_validation", MODULE.package_counts("test " + raw_name + " ... " + ROWS[-1] + "\n")["adaptive_profiles"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts("test wrong_case ... " + ROWS[0] + "\n")

    def test_unknown_duplicate_wrong_marker_and_truncation_reject(self):
        for text in (
            ROWS[0].replace("case=retained_prefix_preserves_second_round_information", "case=unknown") + "\n",
            stream((ROWS[0], ROWS[0])),
            ROWS[0].replace("CORE_RETAINED_RUNS_PROFILE", "CORE_ADAPTIVE_HISTORY_PROFILE") + "\n",
            ROWS[0],
            ROWS[0] + "\n" + "CORE_RETAINED_RUNS_PROFILE nonsense\n",
        ):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.package_counts(text)

    def test_malformed_extra_fields_zero_and_overflow_reject(self):
        for text in (
            ROWS[0] + " extra=1\n",
            ROWS[0].replace("points=37", "points=037") + "\n",
            ROWS[0].replace("points=37 inputs=3 selections=3", "points=0 inputs=0 selections=0") + "\n",
            ROWS[0].replace("points=37", "points=18446744073709551616") + "\n",
            ROWS[0].replace("points=37", "points=38") + "\n",
            ROWS[0].replace("inputs=3", "inputs=0") + "\n",
            ROWS[4].replace("observations=13", "observations=12") + "\n",
            ROWS[4].replace("histories=1", "histories=0") + "\n",
        ):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.package_counts(text)

    def test_source_work_rejects_unowned_or_malformed_vectors(self):
        calls = MODULE.package_counts(stream(), require_complete=True)["adaptive_observed_calls"]
        for bad in ({}, {**calls, "extra": 1}, {**calls, "adaptive_appends": True},
                    {**calls, "adaptive_appends": 201}, {**calls, "retained_points": -1}):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.source_work(bad)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts(stream(), require_complete=1)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts(b"not text")

    def test_event_fixture_is_charged_once_and_not_as_fit(self):
        event = MODULE.package_counts(ROWS[-1] + "\n")["adaptive_recorded_source_work_upper"]
        self.assertEqual(event["raw_lifecycle_validations_upper"], 8)
        self.assertEqual(event["raw_event_fixture_entries_upper"], 1)
        self.assertEqual(event["raw_event_update_constructions_upper"], 9)
        self.assertEqual(event["raw_event_state_equalities_upper"], 8)
        # The independently reserved private guard setup contributes 25 appends;
        # the event fixture itself creates no history, normalization or fit.
        self.assertEqual(event["charged_fit_work_upper"], 1550)
        no_event = MODULE.package_counts(ROWS[-2] + "\n")["adaptive_recorded_source_work_upper"]
        self.assertEqual(no_event["raw_event_fixture_entries_upper"], 0)
        self.assertEqual(no_event["raw_event_update_constructions_upper"], 0)

    def test_doctest_and_source_bound_external_consumer(self):
        doc = MODULE.literal_doctest_reservation()
        self.assertEqual(doc["adaptive_history_constructions_upper"], 1)
        self.assertEqual(doc["adaptive_minimum_validations_upper"], 1)
        self.assertEqual(doc["adaptive_minimum_predicates_upper"], 2)
        self.assertEqual(doc["adaptive_history_getter_requests_upper"], 3)
        self.assertEqual(doc["charged_fit_work_upper"], 0)
        legal = MODULE.external_legal_reservation()
        self.assertEqual(legal["adaptive.call.retained_selections"], 2)
        self.assertEqual(legal["adaptive.call.external_normalizations"], 1)
        self.assertEqual(legal["adaptive.work.adaptive_minimum_validations_upper"], 1)
        self.assertEqual(legal["adaptive.work.raw_event_fixture_entries_upper"], 0)
        self.assertEqual(legal["adaptive.work.adaptive_private_prefix_calls_upper"], 0)
        self.assertEqual(legal["adaptive.work.adaptive_private_append_calls_upper"], 0)
        self.assertEqual(legal["adaptive.work.adaptive_error_display_requests_upper"], 0)
        self.assertEqual(legal["adaptive.work.charged_fit_work_upper"], 310)
        self.assertEqual(legal["adaptive.work.point_admissions_upper"], 10641)
        self.assertEqual(set(legal), set(MODULE.full_package_reservation()))
        self.assertEqual(MODULE.EXTERNAL_LEGAL_SOURCE_SHA256,
                         "e33303f800c2358de41beb526958c98978c4e30f89408878ca32406761d32998")

    def test_external_normalizer_is_disjoint_from_package_guards(self):
        calls = dict.fromkeys(MODULE.FIELDS, 0)
        calls.update(retained_selections=2, adaptive_observations=3,
                     adaptive_appends=3, adaptive_histories=1, external_normalizations=1)
        work = MODULE.source_work(calls)
        self.assertEqual(work["adaptive_normalizations_upper"], 4)
        self.assertEqual(work["selector_entries_upper"], 5)
        self.assertEqual(work["charged_fit_work_upper"], 310)
        self.assertEqual(work["point_admissions_upper"], 10641)
        self.assertEqual(work["adaptive_minimum_validations_upper"], 1)
        self.assertEqual(work["raw_event_fixture_entries_upper"], 0)

    def test_output_size_and_mixed_marker_guards(self):
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts("x" * (MODULE.MAX_OUTPUT_CHARACTERS + 1))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts(ROWS[0] + " CORE_ADAPTIVE_HISTORY_PROFILE\n")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.package_counts(ROWS[0] + "x" * MODULE.MAX_MARKER_CHARACTERS + "\n")
