"""Independent inert enclosure transport controls; no Rust/native execution."""
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "core_enclosure_accounting", Path(__file__).resolve().parents[1] / "core-enclosure-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class EnclosureAccountingTests(unittest.TestCase):
    """Test receiving parsers with literal source recipes and hostile neighbors."""

    @staticmethod
    def row(case="linear_latest", values=(1, 3, 3, 1, 5, 12, 3, 211, 1, 1, 1)):
        fields = ("histories", "appends", "observations", "contexts", "monotonic",
                  "coordinates", "exchanges", "getters", "terms", "projections", "enclosures")
        return "CORE_ENCLOSURE_PROFILE case=" + case + "".join(
            " " + name + "=" + str(value) for name, value in zip(fields, values, strict=True)) + "\n"

    @classmethod
    def rows(cls):
        cases = (
            ("linear_latest", (1,3,3,1,5,12,3,211,1,1,1)),
            ("linear_older", (1,3,3,1,5,12,3,211,1,1,1)),
            ("tiny_radius", (1,3,3,1,5,12,3,211,1,1,1)),
            ("fractional_signs", (2,6,6,2,10,24,6,422,2,2,2)),
            ("asymmetric_endpoints", (2,6,6,2,10,24,6,422,2,2,2)),
            ("subtick_overlap", (2,6,6,2,10,24,6,422,2,2,2)),
            ("signed_zero", (2,6,6,2,10,24,6,422,2,2,2)),
            ("large_radius", (1,3,3,1,7,12,3,261,2,2,2)),
            ("identity_labels", (3,9,9,3,15,36,9,633,3,3,3)),
        )
        return "".join(cls.row(case, counts) for case, counts in cases) + "".join(
            cls.row("enclosure_private_" + case, (0,)*11)
            for case in ("rules", "admission", "facts", "ranges", "fractions", "errors"))

    def test_exact_complete_source_rows_have_independent_attempt_totals(self):
        observed = MODULE.enclosure_counts(self.rows(), require_complete=True)
        self.assertEqual(observed["enclosure_counter_records"], 15)
        self.assertEqual(observed["enclosure_observed_calls"], {
            "histories":15, "appends":45, "observations":45, "contexts":15,
            "monotonic":77, "coordinates":180, "exchanges":45, "getters":3215,
            "terms":16, "projections":16, "enclosures":16})
        self.assertFalse(observed["enclosure_observation_pending"])
        self.assertNotIn("native_outcome", observed)

    def test_missing_and_failed_attempt_prefixes_never_become_observed_zero(self):
        absent = MODULE.enclosure_counts("unrelated native text\n")
        self.assertIsNone(absent["enclosure_recorded_call_subtotals"])
        self.assertIsNone(absent["enclosure_observed_calls"])
        prefix = MODULE.enclosure_counts(self.row(values=(1,0,0,0,0,0,0,0,0,0,0)))
        self.assertEqual(prefix["enclosure_recorded_call_subtotals"]["histories"], 1)
        self.assertEqual(prefix["enclosure_partial_cases"], ["linear_latest"])
        self.assertIsNone(prefix["enclosure_observed_calls"])
        self.assertTrue(prefix["enclosure_observation_pending"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.enclosure_counts(self.row(), require_complete=True)

    def test_raw_private_zero_scope_is_distinct_from_public_zero_forgery(self):
        parsed = MODULE.enclosure_counts(self.row("enclosure_private_rules", (0,)*11))
        self.assertEqual(parsed["enclosure_recorded_call_subtotals"]["enclosures"], 0)
        self.assertIsNone(parsed["enclosure_observed_calls"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.enclosure_counts(self.row(values=(0,)*11))

    def test_unknown_duplicate_truncated_reordered_nondecimal_and_overbound_rows_reject(self):
        good = self.row()
        invalid = [good + good, good.rstrip("\n"), good.replace("linear_latest", "unowned"),
                   good.replace("histories=1 appends=3", "appends=3 histories=1"),
                   good.replace("histories=1", "histories=01"), good.replace("histories=1", "histories=-1"),
                   good.replace("histories=1", "histories=2"), good.replace("enclosures=1", "enclosures=18446744073709551616"),
                   good.rstrip("\n") + " extra=0\n", "replayed: " + good]
        for value in invalid:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.enclosure_counts(value)

    def test_exact_libtest_source_name_and_case_pair_are_required(self):
        good = "test latest_linear_anchor_has_exact_q33_endpoints ... " + self.row()
        self.assertEqual(MODULE.enclosure_counts(good)["enclosure_counter_records"], 1)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.enclosure_counts(good.replace("linear_latest", "linear_older"))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.enclosure_counts(good.replace("latest_linear_anchor_has_exact_q33_endpoints", "unowned"))

    def test_complete_reservation_cannot_borrow_legacy_or_projection_telemetry(self):
        reservation = MODULE.full_package_reservation()
        work = reservation["enclosure_reserved_source_work_uppers"]
        self.assertEqual(work["charged_fit_work_upper"], 45)
        self.assertEqual(work["point_admissions_upper"], 2880)
        self.assertEqual(work["projection_term_predicate_calls_upper"], 96)
        # Fifteen current successful wrappers format once; every attempted
        # enclosure reserves one formatting call before its native result.
        self.assertEqual(work["enclosure_wrapper_debug_format_calls_upper"], 16)
        self.assertTrue(reservation["enclosure_reserved_private_source_work"])
        self.assertNotIn("projection_reserved_calls", reservation)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.source_work_uppers({"enclosures":16})


if __name__ == "__main__":
    unittest.main()
