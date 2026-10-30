"""Independent literal controls for selection observation transport; no Cargo.

All streams are synthetic. Passing these controls does not execute a Rust test,
an application gate or a mutation. The coordinator owns all execution.
"""
from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_selection_controls", ROOT / "scripts/core-selection-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class SelectionAccountingTests(unittest.TestCase):
    """Known healthy facts accompany malformed, missing and partial streams."""

    @staticmethod
    def healthy():
        return (
            "S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0\n"
            "S13_SELECTION_PROFILE scope=minimum intervals=0 minima=6 batches=0 selections=0\n"
            "S13_SELECTION_PROFILE scope=batch intervals=19 minima=1 batches=8 selections=1\n"
            "S13_SELECTION_PROFILE scope=literal_permutations intervals=446 minima=65 batches=65 selections=65\n"
            'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":1}\n'
            'S13_BATCH_GUARD_PROFILE {"case":"capacity_neighbor","intervals":16,"batches":2}\n'
            'S13_BATCH_GUARD_PROFILE {"case":"duplicate_recovery","intervals":2,"batches":5}\n'
        )

    @staticmethod
    def helper_healthy():
        return (
            "SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=2 prefix_sorts=3 "
            "cell_depths=2 label_projections=5 endpoint_collections=2 depth_builds=2 "
            "component_scans=3 witness_builds=4\n"
            "SELECTION_HELPER_PROFILE case=duplicate_recovery read_adapters=0 prefix_sorts=0 "
            "cell_depths=0 label_projections=0 endpoint_collections=3 depth_builds=3 "
            "component_scans=2 witness_builds=0\n"
        )

    def test_complete_package_records_have_literal_totals_and_no_outcome_claim(self):
        result = MODULE.selection_counts(self.healthy(), require_complete=True)
        self.assertEqual(result["selection_observed_calls"],
                         {"intervals": 492, "minima": 72, "batches": 81, "selections": 66})
        self.assertEqual(result["selection_external_records"], 4)
        self.assertEqual(result["selection_guard_records"], 3)
        self.assertTrue(result["selection_counter_observation_complete"])
        self.assertFalse(result["selection_observation_pending"])
        self.assertEqual(result["selection_observation_status"], "complete-source-recipe")
        self.assertEqual(result["selection_missing_external_scopes"], [])
        self.assertEqual(result["selection_missing_guard_cases"], [])
        self.assertNotIn("native_exit", result)
        self.assertNotIn("killed", result)
        self.assertNotIn("selection_read_attempts", result)
        self.assertEqual((MODULE.PACKAGE_ORDINARY_TESTS, MODULE.PACKAGE_DOCTESTS,
                          MODULE.PACKAGE_BASELINE_TESTS), (94, 2, 96))

    def test_exact_progress_prefixes_and_crlf_preserve_separate_records(self):
        names = [
            "minimum_construction_preserves_valid_values",
            "interval_construction_rejects_invalid_raw_values_and_accepts_neighbors",
            "batch_admission_rejects_duplicate_and_capacity_before_recovery",
            "interval_majority_literal_cases_and_permutations",
            "selection::selection_guards::capacity_neighbor_copies_all_values_in_order",
            "selection::selection_guards::over_capacity_rejects_before_adapter_reads",
            "selection::selection_guards::duplicate_stops_at_earliest_index_and_recovers",
        ]
        lines = self.healthy().splitlines()
        mixed = "running 94 tests\r\n" + "".join(
            "test " + name + " ... " + line + "\r\n" for name, line in zip(names, lines, strict=True))
        mixed += "test unrelated_legacy_owner ... ok\r\n"
        result = MODULE.selection_counts(mixed, require_complete=True)
        self.assertEqual(result["selection_observed_calls"]["selections"], 66)
        reversed_result = MODULE.selection_counts("\n".join(reversed(lines)) + "\n", require_complete=True)
        self.assertEqual(result["selection_observed_calls"], reversed_result["selection_observed_calls"])

    def test_missing_records_remain_pending_and_never_become_zero_calls(self):
        one = "S13_SELECTION_PROFILE scope=literal_permutations intervals=0 minima=1 batches=1 selections=1\n"
        result = MODULE.selection_counts(one)
        self.assertIsNone(result["selection_observed_calls"])
        self.assertEqual(result["selection_recorded_call_subtotals"],
                         {"intervals": 0, "minima": 1, "batches": 1, "selections": 1})
        self.assertTrue(result["selection_observation_pending"])
        self.assertFalse(result["selection_counter_observation_complete"])
        self.assertEqual(result["selection_missing_external_scopes"], ["interval", "minimum", "batch"])
        self.assertEqual(result["selection_missing_guard_cases"], ["capacity", "capacity_neighbor", "duplicate_recovery"])
        empty = MODULE.selection_counts("running 94 tests\n")
        self.assertEqual(empty["selection_observation_status"], "missing")
        self.assertIsNone(empty["selection_recorded_call_subtotals"])
        self.assertIsNone(empty["selection_observed_calls"])
        for output in (one, ""):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.selection_counts(output, require_complete=True)

    def test_all_present_failed_test_prefixes_are_partial_not_full_recipe(self):
        output = (
            "S13_SELECTION_PROFILE scope=interval intervals=1 minima=0 batches=0 selections=0\n"
            "S13_SELECTION_PROFILE scope=minimum intervals=0 minima=1 batches=0 selections=0\n"
            "S13_SELECTION_PROFILE scope=batch intervals=2 minima=0 batches=0 selections=0\n"
            "S13_SELECTION_PROFILE scope=literal_permutations intervals=0 minima=1 batches=1 selections=1\n"
            'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":0}\n'
            'S13_BATCH_GUARD_PROFILE {"case":"capacity_neighbor","intervals":1,"batches":0}\n'
            'S13_BATCH_GUARD_PROFILE {"case":"duplicate_recovery","intervals":1,"batches":0}\n'
        )
        result = MODULE.selection_counts(output)
        self.assertEqual(result["selection_observed_calls"],
                         {"intervals": 6, "minima": 2, "batches": 1, "selections": 1})
        self.assertFalse(result["selection_observation_pending"])
        self.assertFalse(result["selection_counter_observation_complete"])
        self.assertEqual(result["selection_observation_status"], "partial-source-recipe")
        self.assertEqual(len(result["selection_partial_external_scopes"]), 4)
        self.assertEqual(len(result["selection_partial_guard_cases"]), 3)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts(output, require_complete=True)

    def test_external_schema_scalars_and_prefixes_reject_with_healthy_neighbor(self):
        row = self.healthy().splitlines()[0]
        invalid = [
            row.replace("scope=interval", "scope=unknown"),
            row.replace("intervals=8", "intervals=08"),
            row.replace("intervals=8", "intervals=-1"),
            row.replace("intervals=8", "intervals=+1"),
            row.replace("intervals=8", "intervals=8.0"),
            row.replace("intervals=8", "intervals=true"),
            row.replace("intervals=8", "intervals=NaN"),
            row.replace("intervals=8", "intervals=18446744073709551616"),
            row.replace("intervals=8", "intervals=100000000000000000000"),
            row.replace("intervals=8", "intervals=9"),
            row.replace("minima=0", "minima=1"),
            row.replace(" scope=", "  scope="),
            row + " extra=1",
            " " + row,
            "untrusted " + row,
            "test unrelated_test ... " + row,
            "test selection::selection_guards::over_capacity_rejects_before_adapter_reads ... " + row,
            '{"reason":"compiler-message","message":"' + row + '"}',
            "untrusted\v" + row,
            row + "\rtrailing",
        ]
        neighbor = 'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":1}\n'
        for value in invalid:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_counts(neighbor + value + "\n")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts(row)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts(row + "\r")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts(row + "\n" + row + "\n")

    def test_guard_closed_json_has_no_coercion_extra_fields_or_duplicate_case(self):
        row = 'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":1}'
        invalid = [
            row.replace('"capacity"', '"unknown"'),
            row.replace('"intervals":1', '"intervals":true'),
            row.replace('"intervals":1', '"intervals":null'),
            row.replace('"intervals":1', '"intervals":NaN'),
            row.replace('"intervals":1', '"intervals":Infinity'),
            row.replace('"intervals":1', '"intervals":1.0'),
            row.replace('"intervals":1', '"intervals":"1"'),
            row.replace('"intervals":1', '"intervals":-1'),
            row.replace('"intervals":1', '"intervals":01'),
            row.replace('"intervals":1', '"intervals":2'),
            row.replace('"batches":1', '"batches":2'),
            row.replace('"batches":1', '"batches":1,"batches":1'),
            row.replace('"batches":1', '"extra":1,"batches":1'),
            row.replace(',"batches":1', ''),
            row.replace(',"intervals"', ', "intervals"'),
            row.replace('"intervals":1,"batches":1', '"batches":1,"intervals":1'),
            row + " trailing",
        ]
        neighbor = "S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0\n"
        for value in invalid:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_counts(neighbor + value + "\n")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts(row + "\n" + row + "\n")

    def test_counter_order_rejects_impossible_receiving_paths(self):
        rows = [
            "S13_SELECTION_PROFILE scope=interval intervals=0 minima=0 batches=0 selections=0",
            "S13_SELECTION_PROFILE scope=batch intervals=2 minima=0 batches=1 selections=0",
            "S13_SELECTION_PROFILE scope=batch intervals=10 minima=0 batches=5 selections=0",
            "S13_SELECTION_PROFILE scope=batch intervals=19 minima=1 batches=7 selections=0",
            "S13_SELECTION_PROFILE scope=batch intervals=19 minima=0 batches=8 selections=1",
            "S13_SELECTION_PROFILE scope=literal_permutations intervals=4 minima=2 batches=1 selections=1",
            "S13_SELECTION_PROFILE scope=literal_permutations intervals=4 minima=2 batches=2 selections=0",
            'S13_BATCH_GUARD_PROFILE {"case":"capacity_neighbor","intervals":15,"batches":1}',
            'S13_BATCH_GUARD_PROFILE {"case":"duplicate_recovery","intervals":1,"batches":1}',
        ]
        for row in rows:
            with self.subTest(row=row), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_counts(row + "\n")

    def test_input_contract_and_marker_limits_are_enforced(self):
        for value in (None, b"", 1, [], {}):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_counts(value)
        for value in (None, 0, 1, "true"):
            with self.subTest(mode=value), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_counts(self.healthy(), require_complete=value)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts("x" * (16 * 1024 * 1024 + 1))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_counts("S13_SELECTION_PROFILE" + "x" * 2000 + "\n")

    def test_precharged_source_projection_is_separate_and_fresh(self):
        reserve = MODULE.full_package_reservation()
        self.assertEqual(reserve, {"intervals": 492, "minima": 72, "batches": 81,
                                   "selections": 66, "garde_validator_invocations_upper": 1056})
        reserve["intervals"] = 0
        self.assertEqual(MODULE.full_package_reservation()["intervals"], 492)
        upper = MODULE.source_work_uppers(66, 77)
        self.assertEqual(upper, {
            "selection_endpoint_collection_writes_upper": 2112,
            "selection_endpoint_sort_comparisons_upper": 32736,
            "selection_endpoint_dedup_comparisons_upper": 2046,
            "selection_cell_visits_upper": 4158,
            "selection_membership_probes_upper": 66528,
            "selection_component_visits_upper": 4158,
            "selection_witness_probes_upper": 1056,
            "selection_label_sort_comparisons_upper": 7920,
            "selection_intersection_reductions_upper": 1980,
            "selection_batch_copy_adapter_attempts_upper": 1232,
            "selection_batch_label_comparisons_upper": 9240,
        })
        self.assertTrue(all(value == 0 for value in MODULE.source_work_uppers(0, 0).values()))
        for value in (True, False, None, -1, 1.0, "1", 1 << 64, (1 << 64) - 1):
            for selector_count, batch_count in ((value, 1), (1, value)):
                with self.subTest(counts=(selector_count, batch_count)), self.assertRaises(MODULE.Incomplete):
                    MODULE.source_work_uppers(selector_count, batch_count)

    def test_raw_helper_complete_record_has_literal_direct_recipe(self):
        stream = self.healthy() + self.helper_healthy()
        result = MODULE.helper_counts(stream, require_complete=True)
        self.assertEqual(result["selection_helper_observed_calls"],
                         {"read_adapters": 2, "prefix_sorts": 3, "cell_depths": 2, "label_projections": 5,
                          "endpoint_collections": 5, "depth_builds": 5, "component_scans": 5, "witness_builds": 4})
        self.assertEqual(result["selection_helper_records"], 2)
        self.assertEqual(result["selection_helper_missing_cases"], [])
        self.assertEqual(result["selection_helper_partial_cases"], [])
        self.assertEqual(set(result["selection_helper_profiles"]), {"capacity_neighbor", "duplicate_recovery"})
        self.assertFalse(result["selection_helper_observation_pending"])
        self.assertTrue(result["selection_helper_counter_observation_complete"])
        self.assertEqual(result["selection_helper_observation_status"], "complete-source-recipe")
        self.assertNotIn("native_exit", result)
        self.assertNotIn("intervals", result["selection_helper_observed_calls"])
        self.assertEqual(MODULE.selection_counts(stream, require_complete=True)["selection_observed_calls"],
                         {"intervals": 492, "minima": 72, "batches": 81, "selections": 66})
        reserve = MODULE.full_package_helper_reservation()
        self.assertEqual(reserve, {"read_adapters": 2, "prefix_sorts": 3,
                                   "cell_depths": 2, "label_projections": 5,
                                   "endpoint_collections": 5, "depth_builds": 5,
                                   "component_scans": 5, "witness_builds": 4})
        reserve["prefix_sorts"] = 0
        self.assertEqual(MODULE.full_package_helper_reservation()["prefix_sorts"], 3)
        self.assertEqual(MODULE.full_package_reservation()["intervals"], 492)

    def test_raw_helper_progress_and_missing_have_independent_semantics(self):
        rows = self.helper_healthy().splitlines()
        names = [
            "selection::selection_guards::capacity_neighbor_copies_all_values_in_order",
            "selection::selection_guards::over_capacity_rejects_before_adapter_reads",
            "selection::selection_guards::duplicate_stops_at_earliest_index_and_recovers",
        ]
        for name in names:
            stream = "".join("test " + name + " ... " + row + "\r\n" for row in rows)
            result = MODULE.helper_counts(stream, require_complete=True)
            self.assertEqual(result["selection_helper_observed_calls"]["label_projections"], 5)
        reverse = MODULE.helper_counts("\n".join(reversed(rows)) + "\n", require_complete=True)
        self.assertEqual(reverse["selection_helper_observed_calls"]["component_scans"], 5)
        missing = MODULE.helper_counts(self.healthy())
        self.assertIsNone(missing["selection_helper_observed_calls"])
        self.assertTrue(missing["selection_helper_observation_pending"])
        self.assertFalse(missing["selection_helper_counter_observation_complete"])
        self.assertEqual(missing["selection_helper_records"], 0)
        self.assertEqual(missing["selection_helper_observation_status"], "missing")
        self.assertEqual(missing["selection_helper_missing_cases"], ["capacity_neighbor", "duplicate_recovery"])
        single = MODULE.helper_counts(rows[0] + "\n")
        self.assertIsNone(single["selection_helper_observed_calls"])
        self.assertEqual(single["selection_helper_recorded_call_subtotals"]["witness_builds"], 4)
        self.assertEqual(single["selection_helper_missing_cases"], ["duplicate_recovery"])
        self.assertEqual(single["selection_helper_partial_cases"], [])
        self.assertEqual(single["selection_helper_observation_status"], "partial-source-recipe")
        self.assertTrue(MODULE.selection_counts(self.healthy(), require_complete=True)
                        ["selection_counter_observation_complete"])
        for output in ("", self.healthy(), rows[0] + "\n", rows[1] + "\n"):
            with self.subTest(output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.helper_counts(output, require_complete=True)
        for value in (None, b"", 1, [], {}):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.helper_counts(value)
        for value in (None, 0, 1, "true"):
            with self.subTest(mode=value), self.assertRaises(MODULE.Incomplete):
                MODULE.helper_counts(self.helper_healthy(), require_complete=value)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.helper_counts("x" * (16 * 1024 * 1024 + 1))

    def test_raw_helper_closed_lines_reject_forged_schema_and_scalars(self):
        row, duplicate = self.helper_healthy().splitlines()
        invalid = [
            row.replace("case=capacity_neighbor", "case=unknown"),
            row.replace("case=capacity_neighbor", "case=duplicate_recovery"),
            row.replace(" case=capacity_neighbor", ""),
            row.replace("read_adapters=2", "read_adapters=02"),
            row.replace("read_adapters=2", "read_adapters=-1"),
            row.replace("read_adapters=2", "read_adapters=+1"),
            row.replace("read_adapters=2", "read_adapters=2.0"),
            row.replace("read_adapters=2", "read_adapters=true"),
            row.replace("read_adapters=2", "read_adapters=null"),
            row.replace("read_adapters=2", "read_adapters=NaN"),
            row.replace("read_adapters=2", "read_adapters=18446744073709551616"),
            row.replace("read_adapters=2", "read_adapters=3"),
            row.replace("prefix_sorts=3", "prefix_sorts=4"),
            row.replace("cell_depths=2", "cell_depths=3"),
            row.replace("label_projections=5", "label_projections=6"),
            row.replace("endpoint_collections=2", "endpoint_collections=3"),
            row.replace("depth_builds=2", "depth_builds=3"),
            row.replace("component_scans=3", "component_scans=4"),
            row.replace("witness_builds=4", "witness_builds=5"),
            duplicate.replace("endpoint_collections=3", "endpoint_collections=4"),
            duplicate.replace("depth_builds=3", "depth_builds=4"),
            duplicate.replace("component_scans=2", "component_scans=3"),
            duplicate.replace("witness_builds=0", "witness_builds=1"),
            row.replace(" read_adapters=", "  read_adapters="),
            row.replace(" read_adapters=2 prefix_sorts=3", " prefix_sorts=3 read_adapters=2"),
            row.replace(" cell_depths=2", ""),
            row.replace(" witness_builds=4", ""),
            row + " extra=1",
            row + " label_projections=5",
            " " + row,
            "untrusted " + row,
            "test unrelated_test ... " + row,
            "test interval_majority_literal_cases_and_permutations ... " + row,
            '{"reason":"compiler-message","rendered":"' + row + '"}',
            row + "\rtrailing",
        ]
        neighbor = self.healthy().splitlines()[0] + "\n"
        for value in invalid:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.helper_counts(neighbor + value + "\n")
        for output in (row, row + "\r", row + "\n" + row + "\n",
                       "SELECTION_HELPER_PROFILE" + "x" * 2000 + "\n"):
            with self.subTest(output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.helper_counts(output)

    def test_raw_helper_failed_prefixes_remain_partial_in_fixed_order(self):
        fields = ("read_adapters", "prefix_sorts", "cell_depths", "label_projections",
                  "endpoint_collections", "depth_builds", "component_scans", "witness_builds")
        healthy_capacity, healthy_duplicate = self.helper_healthy().splitlines()
        recipes = [
            ("capacity_neighbor", healthy_duplicate, [
                (1, 0, 0, 0, 0, 0, 0, 0), (2, 0, 0, 0, 0, 0, 0, 0),
                (2, 1, 0, 0, 0, 0, 0, 0), (2, 3, 0, 0, 0, 0, 0, 0),
                (2, 3, 1, 0, 0, 0, 0, 0), (2, 3, 2, 0, 0, 0, 0, 0),
                (2, 3, 2, 4, 0, 0, 0, 0), (2, 3, 2, 5, 1, 0, 0, 0),
                (2, 3, 2, 5, 2, 1, 0, 0), (2, 3, 2, 5, 2, 2, 2, 0),
                (2, 3, 2, 5, 2, 2, 3, 0), (2, 3, 2, 5, 2, 2, 3, 3),
            ]),
            ("duplicate_recovery", healthy_capacity, [
                (0, 0, 0, 0, 1, 0, 0, 0), (0, 0, 0, 0, 3, 0, 0, 0),
                (0, 0, 0, 0, 3, 2, 0, 0), (0, 0, 0, 0, 3, 3, 1, 0),
            ]),
        ]
        for case, healthy_other, prefixes in recipes:
            for counts in prefixes:
                row = "SELECTION_HELPER_PROFILE case=" + case + "".join(
                    f" {field}={value}" for field, value in zip(fields, counts, strict=True)) + "\n"
                missing = MODULE.helper_counts(row)
                self.assertIsNone(missing["selection_helper_observed_calls"])
                self.assertTrue(missing["selection_helper_observation_pending"])
                self.assertEqual(missing["selection_helper_recorded_call_subtotals"],
                                 dict(zip(fields, counts, strict=True)))
                stream = row + healthy_other + "\n"
                result = MODULE.helper_counts(stream)
                self.assertEqual(result["selection_helper_profiles"][case],
                                 dict(zip(fields, counts, strict=True)))
                self.assertIsNotNone(result["selection_helper_observed_calls"])
                self.assertFalse(result["selection_helper_observation_pending"])
                self.assertFalse(result["selection_helper_counter_observation_complete"])
                self.assertEqual(result["selection_helper_partial_cases"], [case])
                self.assertEqual(result["selection_helper_observation_status"], "partial-source-recipe")
                with self.subTest(row=row), self.assertRaises(MODULE.Incomplete):
                    MODULE.helper_counts(stream, require_complete=True)
        invalid = [
            ("capacity_neighbor", (0, 0, 0, 0, 0, 0, 0, 0)),
            ("capacity_neighbor", (1, 1, 0, 0, 0, 0, 0, 0)),
            ("capacity_neighbor", (2, 2, 1, 0, 0, 0, 0, 0)),
            ("capacity_neighbor", (2, 3, 2, 5, 1, 1, 0, 0)),
            ("capacity_neighbor", (2, 3, 2, 5, 2, 1, 1, 0)),
            ("capacity_neighbor", (2, 3, 2, 5, 2, 2, 2, 1)),
            ("duplicate_recovery", (0, 0, 0, 0, 1, 1, 0, 0)),
            ("duplicate_recovery", (0, 0, 0, 0, 3, 2, 1, 0)),
            ("duplicate_recovery", (1, 0, 0, 0, 1, 0, 0, 0)),
        ]
        for case, counts in invalid:
            row = "SELECTION_HELPER_PROFILE case=" + case + "".join(
                f" {field}={value}" for field, value in zip(fields, counts, strict=True)) + "\n"
            with self.subTest(row=row), self.assertRaises(MODULE.Incomplete):
                MODULE.helper_counts(row)


if __name__ == "__main__":
    unittest.main()
