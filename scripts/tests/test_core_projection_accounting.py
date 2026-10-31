"""Finite transport/reservation controls; no product/tool execution."""
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location(
    "core_projection_accounting", Path(__file__).resolve().parents[1] / "core-projection-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ProjectionAccountingTests(unittest.TestCase):
    """Explicit synthetic source recipes test the reader independently."""

    def setUp(self):
        recipes = (("public_case", (1, 3, 3, 1, 4, 12, 3, 15, 1, 2)),
                   ("private_case", (0,) * 10))
        for name, value in (("SOURCE_RECIPES", recipes), ("SOURCE_TESTS", ("public_test", "module::private_test")),
                            ("SOURCE_PRIVATE_WORK", {"predicate_attempts_upper": 12})):
            temporary = patch.object(MODULE, name, value)
            temporary.start(); self.addCleanup(temporary.stop)

    @staticmethod
    def row(case="public_case", values=(1, 3, 3, 1, 4, 12, 3, 15, 1, 2)):
        fields = ("histories", "appends", "observations", "contexts", "monotonic",
                  "coordinates", "exchanges", "getters", "terms", "projections")
        return "CORE_PROJECTION_PROFILE case=" + case + "".join(
            f" {field}={value}" for field, value in zip(fields, values, strict=True)) + "\n"

    def complete(self):
        return self.row() + self.row("private_case", (0,) * 10)

    def test_complete_lf_crlf_and_exact_selected_libtest_prefix(self):
        for text in (self.complete(), self.complete().replace("\n", "\r\n"),
                     "test public_test ... " + self.complete()):
            result = MODULE.projection_counts(text, require_complete=True)
            self.assertTrue(result["projection_counter_observation_complete"])
            self.assertEqual(result["projection_observed_calls"]["projections"], 2)
            self.assertEqual(result["projection_profiles"]["private_case"]["projections"], 0)

    def test_missing_partial_and_present_zero_private_records_are_distinct(self):
        missing = MODULE.projection_counts("unrelated native output\n")
        self.assertIsNone(missing["projection_recorded_call_subtotals"])
        self.assertIsNone(missing["projection_observed_calls"])
        self.assertTrue(missing["projection_observation_pending"])
        partial = MODULE.projection_counts(self.row(values=(1, 0, 0, 0, 0, 0, 0, 0, 0, 0)))
        self.assertEqual(partial["projection_recorded_call_subtotals"]["histories"], 1)
        self.assertIsNone(partial["projection_observed_calls"])
        self.assertEqual(partial["projection_partial_cases"], ["public_case"])
        private = MODULE.projection_counts(self.row("private_case", (0,) * 10))
        self.assertEqual(private["projection_recorded_call_subtotals"]["histories"], 0)
        self.assertIsNone(private["projection_observed_calls"])
        for text in ("", self.row(), self.row("private_case", (0,) * 10)):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.projection_counts(text, require_complete=True)

    def test_bounded_raw_marker_grammar_rejects_malformed_neighbors(self):
        row = self.row()
        invalid = (row.rstrip("\n"), "prefix " + row, row + row, row.replace("case=public_case", "case=unknown"),
                   row.replace("histories=1", "histories=01"), row.replace("histories=1", "histories=-1"),
                   row.replace("histories=1", "histories=true"), row.replace("histories=1", "histories=18446744073709551616"),
                   row.replace("histories=1", "histories=1.0"), row.replace("getters=15", "getters=16"),
                   row.replace(" terms=1", " terms=1 extra=0"), row.replace(" exchanges=3", ""),
                   '"' + row.replace("\n", r"\n") + '"\n', "test unknown_test ... " + row,
                   self.row(values=(0,) * 10), " " * 1024 + row)
        for text in invalid:
            with self.subTest(text=text[:90]), self.assertRaises(MODULE.Incomplete):
                MODULE.projection_counts(self.complete() + text)

    def test_every_public_field_exact_ceiling_and_above_ceiling_rejection(self):
        maximum = (1, 3, 3, 1, 4, 12, 3, 15, 1, 2)
        for index in range(10):
            values = list(maximum); values[index] += 1
            with self.subTest(index=index), self.assertRaises(MODULE.Incomplete):
                MODULE.projection_counts(self.row(values=values))
        MODULE.projection_counts(self.complete(), require_complete=True)

    def test_full_reservation_does_not_shrink_for_missing_or_partial_observations(self):
        before = MODULE.full_package_reservation()
        MODULE.projection_counts(self.row(values=(1, 0, 0, 0, 0, 0, 0, 0, 0, 0)))
        after = MODULE.full_package_reservation()
        self.assertEqual(before, after)
        self.assertEqual(after["projection_reserved_calls"]["appends"], 3)
        work = after["projection_reserved_source_work_uppers"]
        self.assertEqual(work["charged_fit_work_upper"], 3)
        self.assertEqual(work["point_admissions_upper"], 192)
        self.assertEqual(work["history_staging_Copy_assignments_upper"], 381)
        self.assertEqual(work["projection_duration_operations_upper"], 4)
        self.assertEqual(work["projection_internal_product_getter_requests_upper"], 24)
        self.assertEqual(after["projection_reserved_private_source_work"], {"predicate_attempts_upper": 12})

    def test_source_recipe_pending_duplicate_unknown_shape_and_u64_limits_reject(self):
        for recipe in ((), (("bad-name", (0,) * 10),), (("public_case", (0,) * 9),),
                       (("public_case", (True,) + (0,) * 9),),
                       (("public_case", ((1 << 64),) + (0,) * 9),),
                       (("public_case", (0,) * 10),) * 2):
            with patch.object(MODULE, "SOURCE_RECIPES", recipe):
                with self.subTest(recipe=recipe), self.assertRaises(MODULE.Incomplete):
                    MODULE.full_package_reservation()

    def test_source_work_accepts_only_complete_exact_nonnegative_call_vector(self):
        calls = dict(zip(MODULE.FIELDS, (1, 3, 3, 1, 4, 12, 3, 15, 1, 2), strict=True))
        for bad in ({}, {**calls, "extra": 0}, {**calls, "appends": True},
                    {**calls, "projections": -1}, {**calls, "projections": 1 << 64}):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.source_work_uppers(bad)


class CurrentProjectionRecipeTests(unittest.TestCase):
    """Independent retained-rate and appended-history source facts."""

    def test_current_complete_source_reservation_keeps_new_rate_getters_and_append(self):
        calls = MODULE.full_package_reservation()["projection_reserved_calls"]
        self.assertEqual(calls, {"histories": 12, "appends": 94, "observations": 94,
                                "contexts": 11, "monotonic": 122, "coordinates": 376,
                                "exchanges": 94, "getters": 3450, "terms": 68, "projections": 49})
        work = MODULE.full_package_reservation()["projection_reserved_source_work_uppers"]
        self.assertEqual(work["charged_fit_work_upper"], 94)
        self.assertEqual(work["point_admissions_upper"], 6016)
        row = ProjectionAccountingTests.row("linear_identity", (1, 4, 4, 1, 8, 16, 4, 280, 1, 6))
        parsed = MODULE.projection_counts(row)
        self.assertEqual(parsed["projection_partial_cases"], [])
        self.assertEqual(parsed["projection_profiles"]["linear_identity"]["appends"], 4)
        self.assertTrue(parsed["projection_observation_pending"])


if __name__ == "__main__":
    unittest.main()
