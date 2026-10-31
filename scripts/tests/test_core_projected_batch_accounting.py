"""Literal synthetic transport controls; no Rust or product work executes here."""
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location(
    "core_projected_batch_accounting", Path(__file__).resolve().parents[1] / "core-projected-batch-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

FIELDS = ("histories", "appends", "observations", "contexts", "monotonic",
          "coordinates", "exchanges", "getters", "terms", "projections", "enclosures",
          "batches", "minima", "selections")
VALUES = (1, 3, 3, 1, 4, 12, 3, 8, 1, 1, 1, 2, 3, 5)


class ProjectedBatchAccountingTests(unittest.TestCase):
    """Challenge the receiver with an explicitly synthetic two-scope source model."""

    def setUp(self):
        self.fixture = patch.multiple(
            MODULE,
            SOURCE_RECIPES=(("supported", VALUES), ("raw_guard", (0,) * 14)),
            SOURCE_TESTS=("external_supported", "projected_batch::guards::raw_guard"),
            SOURCE_PRIVATE_CASES=("raw_guard",),
            GUARD_RECIPES=(("raw_guard", (1, 2)),),
            GUARD_TESTS=("projected_batch::guards::raw_guard",),
            SOURCE_WORK_PER_BATCH={"delegated_batch_attempts_upper": 1,
                                   "synthetic_helper_entries_upper": 7},
            SOURCE_WORK_PER_GETTER={"synthetic_format_entries_upper": 1},
            SOURCE_PRIVATE_WORK={"synthetic_direct_private_calls_upper": 11},
        )
        self.fixture.start()
        self.addCleanup(self.fixture.stop)

    @staticmethod
    def row(case="supported", values=VALUES):
        return "CORE_PROJECTED_BATCH_PROFILE case=" + case + "".join(
            " " + field + "=" + str(value) for field, value in zip(FIELDS, values, strict=True)) + "\n"

    @staticmethod
    def guard(callbacks=1, reads=2):
        return ("CORE_PROJECTED_BATCH_GUARD_PROFILE case=raw_guard garde_callbacks=" + str(callbacks)
                + " entry_read_attempts=" + str(reads) + "\n")

    def rows(self):
        return self.row() + self.row("raw_guard", (0,) * 14)

    def test_complete_attempts_and_guard_observations_are_disjoint(self):
        output = self.rows() + self.guard()
        public = MODULE.projected_batch_counts(output, require_complete=True)
        guards = MODULE.guard_counts(output, require_complete=True)
        self.assertEqual(public["projected_batch_observed_calls"], dict(zip(FIELDS, VALUES, strict=True)))
        self.assertEqual(public["projected_batch_counter_records"], 2)
        self.assertEqual(guards["projected_batch_guard_observed_calls"],
                         {"garde_callbacks": 1, "entry_read_attempts": 2})
        self.assertNotIn("native_outcome", public)
        self.assertNotIn("source_work", guards)

    def test_missing_and_failed_prefix_data_never_become_zero_observations(self):
        for parser, prefix in ((MODULE.projected_batch_counts, "projected_batch"),
                               (MODULE.guard_counts, "projected_batch_guard")):
            missing = parser("unrelated native text\n")
            self.assertIsNone(missing[prefix + "_recorded_call_subtotals"])
            self.assertIsNone(missing[prefix + "_observed_calls"])
            self.assertTrue(missing[prefix + "_observation_pending"])
        values = (1,) + (0,) * 13
        partial = MODULE.projected_batch_counts(self.row(values=values))
        self.assertEqual(partial["projected_batch_recorded_call_subtotals"]["histories"], 1)
        self.assertEqual(partial["projected_batch_partial_cases"], ["supported"])
        self.assertIsNone(partial["projected_batch_observed_calls"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.projected_batch_counts(self.row(), require_complete=True)
        guard = MODULE.guard_counts(self.guard(reads=1))
        self.assertEqual(guard["projected_batch_guard_partial_cases"], ["raw_guard"])
        self.assertIsNone(guard["projected_batch_guard_observed_calls"])

    def test_private_zero_source_scope_does_not_erase_direct_private_work(self):
        observed = MODULE.projected_batch_counts(self.row("raw_guard", (0,) * 14))
        self.assertEqual(observed["projected_batch_recorded_call_subtotals"]["batches"], 0)
        reservation = MODULE.full_package_reservation()
        self.assertEqual(reservation["projected_batch_reserved_private_source_work"],
                         {"synthetic_direct_private_calls_upper": 11})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.projected_batch_counts(self.row(values=(0,) * 14))

    def test_constructor_minimum_and_repeat_selection_attempts_are_distinct(self):
        calls = MODULE.full_package_reservation()["projected_batch_reserved_calls"]
        self.assertEqual((calls["batches"], calls["minima"], calls["selections"]), (2, 3, 5))
        work = MODULE.source_work_uppers(calls)
        self.assertEqual(work, {"delegated_batch_attempts_upper": 2,
                                "synthetic_helper_entries_upper": 14,
                                "synthetic_format_entries_upper": 8})
        # Producer fits/interval admission and selection algorithms belong to
        # existing calculators; this helper does not charge them a second time.
        self.assertNotIn("charged_fit_work_upper", work)
        self.assertNotIn("enclosure_interval_constructor_attempts_upper", work)

    def test_unknown_duplicate_truncated_and_noncanonical_rows_reject(self):
        good = self.row()
        invalid = [good + good, good.rstrip("\n"), good.replace("supported", "unknown"),
                   good.replace("histories=1 appends=3", "appends=3 histories=1"),
                   good.replace("histories=1", "histories=01"),
                   good.replace("histories=1", "histories=-1"),
                   good.replace("histories=1", "histories=2"),
                   good.replace("histories=1", "histories=true"),
                   good.replace("selections=5", "selections=18446744073709551616"),
                   good.rstrip("\n") + " extra=0\n", "replayed: " + good]
        for text in invalid:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.projected_batch_counts(text)
        for text in (self.guard() + self.guard(), self.guard().rstrip("\n"),
                     self.guard().replace("raw_guard", "supported"),
                     self.guard(callbacks=2), self.guard(reads=3),
                     self.guard().replace("garde_callbacks=1", "garde_callbacks=01")):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.guard_counts(text)

    def test_exact_libtest_name_and_case_pair_are_required(self):
        public = "test external_supported ... " + self.row()
        private = "test projected_batch::guards::raw_guard ... " + self.guard()
        self.assertEqual(MODULE.projected_batch_counts(public)["projected_batch_counter_records"], 1)
        self.assertEqual(MODULE.guard_counts(private)["projected_batch_guard_counter_records"], 1)
        for text, parser in ((public.replace("external_supported", "unowned"), MODULE.projected_batch_counts),
                             (public.replace("supported histories", "raw_guard histories"), MODULE.projected_batch_counts),
                             (private.replace("projected_batch::guards::raw_guard", "external_supported"), MODULE.guard_counts)):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                parser(text)

    def test_unbounded_text_modes_and_missing_recipes_fail_closed(self):
        for parser in (MODULE.projected_batch_counts, MODULE.guard_counts):
            for value in (None, b"", True, 0):
                with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                    parser(value)
            with self.assertRaises(MODULE.Incomplete):
                parser("", require_complete=1)
            with patch.object(MODULE, 'MAX_OUTPUT_CHARACTERS', 8), self.assertRaises(MODULE.Incomplete):
                parser("x" * 9)
        for recipes in (None, (), (("bad",),), (("same", VALUES), ("same", VALUES))):
            with patch.object(MODULE, 'SOURCE_RECIPES', recipes), self.assertRaises(MODULE.Incomplete):
                MODULE.case_recipes()
        with patch.object(MODULE, 'SOURCE_PRIVATE_CASES', ("missing",)), self.assertRaises(MODULE.Incomplete):
            MODULE.case_recipes()
        with patch.object(MODULE, 'GUARD_RECIPES', (("supported", (1, 2)),)), self.assertRaises(MODULE.Incomplete):
            MODULE.guard_recipes()
        with patch.multiple(MODULE, GUARD_RECIPES=(), GUARD_TESTS=()), self.assertRaises(MODULE.Incomplete):
            MODULE.guard_recipes()

    def test_work_requires_complete_finite_source_bounds_without_boolean_coercion(self):
        calls = dict(zip(FIELDS, VALUES, strict=True))
        invalid = [None, {}, {**calls, "batches": True}, {**calls, "batches": -1},
                   {**calls, "unknown": 0}, {key: value for key, value in calls.items() if key != "minima"}]
        for value in invalid:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.source_work_uppers(value)
        for value in (None, {}, {"delegated_batch_attempts_upper": True},
                      {"delegated_batch_attempts_upper": 2}, {"missing_delegate": 1}):
            with patch.object(MODULE, 'SOURCE_WORK_PER_BATCH', value), self.assertRaises(MODULE.Incomplete):
                MODULE.source_work_uppers(calls)
        with patch.object(MODULE, 'SOURCE_WORK_PER_BATCH', {
                'delegated_batch_attempts_upper': 1, 'too_large': MODULE.U64_MAX}), self.assertRaises(MODULE.Incomplete):
            MODULE.source_work_uppers(calls)
        with patch.object(MODULE, 'SOURCE_PRIVATE_WORK', None), self.assertRaises(MODULE.Incomplete):
            MODULE.full_package_reservation()
        for value in (None, {}, {'bad': True}, {'synthetic_helper_entries_upper': 1}):
            with patch.object(MODULE, 'SOURCE_WORK_PER_GETTER', value), self.assertRaises(MODULE.Incomplete):
                MODULE.source_work_uppers(calls)


class NativeProjectedBatchRecipeTests(unittest.TestCase):
    """Captured native attempt facts; this receiver test executes no Rust."""

    def test_historical_native_linear_row_stays_partial_beside_unchanged_single_case(self):
        # Frozen focused-v2 receipt 2f5eff3c...f05c93, exit0 public lane.
        # Literal rows are independent of the exact-recipe table under test.
        output = (
            'test genuine_compatible_projection_reaches_real_batch_selection ... '
            'CORE_PROJECTED_BATCH_PROFILE case=single_compatible histories=1 appends=3 observations=3 '
            'contexts=1 monotonic=6 coordinates=12 exchanges=3 getters=24 terms=1 projections=1 '
            'enclosures=1 batches=1 minima=1 selections=1\n'
            'test linear_sources_preserve_original_ages_and_configured_minimum ... '
            'CORE_PROJECTED_BATCH_PROFILE case=linear_selection histories=3 appends=9 observations=9 '
            'contexts=3 monotonic=16 coordinates=36 exchanges=9 getters=414 terms=3 projections=3 '
            'enclosures=3 batches=2 minima=2 selections=3\n')
        observed = MODULE.projected_batch_counts(output)
        self.assertEqual(observed['projected_batch_profiles']['single_compatible']['getters'], 24)
        self.assertEqual(observed['projected_batch_profiles']['linear_selection']['getters'], 414)
        self.assertEqual(observed['projected_batch_partial_cases'], ['linear_selection'])
        self.assertEqual(observed['projected_batch_recorded_call_subtotals']['getters'], 438)
        # The two exact rows cannot synthesize the other nine native scopes.
        self.assertEqual(len(observed['projected_batch_missing_cases']), 9)
        self.assertTrue(observed['projected_batch_observation_pending'])
        self.assertIsNone(observed['projected_batch_observed_calls'])
        reservation = MODULE.full_package_reservation()
        self.assertEqual(reservation['projected_batch_reserved_calls']['getters'], 4655)
        self.assertEqual(reservation['projected_batch_reserved_module_work_uppers']['wrapper_debug_format_calls_upper'], 4655)

    def test_current_retained_rate_linear_recipe_has_its_full_getter_count(self):
        # Authored current source recipe; this test makes no native-result claim.
        row = ('CORE_PROJECTED_BATCH_PROFILE case=linear_selection histories=3 appends=9 '
               'observations=9 contexts=3 monotonic=16 coordinates=36 exchanges=9 getters=525 '
               'terms=3 projections=3 enclosures=3 batches=2 minima=2 selections=3\n')
        parsed = MODULE.projected_batch_counts(row)
        self.assertEqual(parsed['projected_batch_partial_cases'], [])
        self.assertEqual(parsed['projected_batch_profiles']['linear_selection']['getters'], 525)
        self.assertIsNone(parsed['projected_batch_observed_calls'])
        self.assertEqual(len(parsed['projected_batch_missing_cases']), 10)


if __name__ == "__main__":
    unittest.main()
