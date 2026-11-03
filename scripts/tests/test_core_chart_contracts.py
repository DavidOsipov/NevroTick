"""Pure controls for chart external-fixture inventory and output accounting."""

import hashlib
import importlib.util
import json
import pathlib
import re
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_chart_contracts", ROOT / "scripts/core-chart-contracts.py")
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ChartContractControls(unittest.TestCase):
    """Check literal fixtures without claiming compiler or native execution."""

    @staticmethod
    def healthy():
        return (
            "CORE_CHART_CONTRACT_PROFILE labels=5 contexts=2 monotonic=4 "
            "coordinates=1 exchanges=1 observations=3 histories=1 appends=3 "
            "frames=1 domains=1 charts=1 prepares=4 terms=1 tracked_projections=1 "
            "enclosures=1 getters=22 batches=1 minima=1 selections=1 batch_getters=22\nlegal-chart-contract\n"
        )

    def test_fixed_legal_recipe_and_literally_bound_source(self):
        self.assertEqual(MODULE.legal_expected(), {
            "labels": 5, "contexts": 2, "monotonic": 4, "coordinates": 1,
            "exchanges": 1, "observations": 3, "histories": 1, "appends": 3,
            "frames": 1, "domains": 1, "charts": 1, "prepares": 4,
            "terms": 1, "tracked_projections": 1, "enclosures": 1, "getters": 22,
            "batches": 1, "minima": 1, "selections": 1, "batch_getters": 22,
        })
        self.assertEqual(sum(MODULE.legal_expected().values()), 77)
        self.assertEqual(MODULE.contract_counts(self.healthy()), MODULE.legal_expected())
        source = MODULE.legal_contract_source()
        self.assertEqual(hashlib.sha256(source.encode()).hexdigest(),
                         "6f1d16eeaa436ef04e6d513ecb1de1f95e69d75ed66151228d8ddc24196d1bce")
        self.assertEqual(len(re.findall(r"call!\(c,15,", source)), 22)
        self.assertIn("enclose_fixture_chart_projection(&p)", source)
        self.assertIn("FixtureChartBatch::new(&chart,t4,&entries)", source)
        self.assertIn("FixtureMinimum::new(1)", source)
        self.assertIn("batch.select(minimum)", source)
        self.assertEqual(len(re.findall(r"call!\(c,19,", source)), 22)
        self.assertIn("FixtureChartOutcome::Refused", source)
        self.assertIn("FixtureChartOutcome::Unknown", source)
        self.assertIn("FixtureChartError::ContextMismatch", source)

    def test_exact_lf_crlf_and_both_record_orders(self):
        a, b = self.healthy().splitlines(keepends=True)
        for text in (a + b, b + a, (a + b).replace("\n", "\r\n"),
                     (b + a).replace("\n", "\r\n")):
            with self.subTest(text=text):
                self.assertEqual(MODULE.contract_counts(text), MODULE.legal_expected())

    def test_incomplete_duplicate_malformed_and_unrelated_records_reject(self):
        text = self.healthy()
        marker, sentinel = text.splitlines(keepends=True)
        bad = (
            "", marker, sentinel, text.rstrip("\n"), marker + marker,
            text + marker, text + "unrelated\n", text.replace("appends=3", "appends=2"),
            text.replace("getters=22", "getters=022"), text.replace("getters=22", "getters=true"),
            text.replace(" charts=1", ""), text.replace("getters=22", "getters=22 injected=1"),
            text.replace("labels=5 contexts=2", "contexts=2 labels=5"),
            text.replace("CORE_CHART_CONTRACT_PROFILE", "CORE_ADAPTIVE_CONTRACT_PROFILE"),
            text.replace("legal-chart-contract", "legal-core-contract"), "x" * 4097,
        )
        for item in bad:
            with self.subTest(item=item):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.contract_counts(item)
        for item in (None, 1, b"invalid", [text], True):
            with self.subTest(item=item):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.contract_counts(item)

    def test_closed_inventory_has_one_legal_and_twenty_distinct_negatives(self):
        cases = MODULE.contract_examples()
        expected = {
            "legal_chart_tracking_and_enclosure", "chart_frames_private_update",
            "chart_domain_private_update", "chart_chart_private_update", "chart_view_private_update",
            "chart_projection_private_update", "chart_interval_private_update", "chart_chart_default",
            "chart_projection_default", "chart_live_history_borrow", "chart_descriptor_lifetime",
            "chart_to_legacy_projection", "chart_to_legacy_interval", "chart_interval_in_old_batch",
            "chart_batch_private_update", "chart_selection_private_update",
            "chart_selection_batch_lifetime", "chart_batch_descriptor_lifetime",
            "chart_batch_entries_lifetime", "chart_interval_into_legacy", "chart_batch_live_slice_borrow",
        }
        self.assertEqual(set(cases), expected)
        self.assertEqual(sum(item["code"] is None for item in cases.values()), 1)
        self.assertEqual(sum(item["code"] is not None for item in cases.values()), 20)
        self.assertEqual(cases[MODULE.LEGAL_NAME]["chart_calls"], MODULE.legal_expected())
        self.assertEqual(cases[MODULE.LEGAL_NAME]["source"], MODULE.legal_contract_source())
        self.assertEqual(len({item["source"] for item in cases.values()}), 21)

    def test_negative_descriptors_bind_error_kind_context_line_and_neighbor(self):
        expected_codes = {
            "chart_frames_private_update": "E0451", "chart_domain_private_update": "E0451",
            "chart_chart_private_update": "E0451", "chart_view_private_update": "E0451",
            "chart_projection_private_update": "E0451", "chart_interval_private_update": "E0451",
            "chart_chart_default": "E0277", "chart_projection_default": "E0277",
            "chart_live_history_borrow": "E0502", "chart_descriptor_lifetime": "E0515",
            "chart_to_legacy_projection": "E0308", "chart_to_legacy_interval": "E0308",
            "chart_interval_in_old_batch": "E0308",
            "chart_batch_private_update": "E0451", "chart_selection_private_update": "E0451",
            "chart_selection_batch_lifetime": "E0515", "chart_batch_descriptor_lifetime": "E0515",
            "chart_batch_entries_lifetime": "E0515", "chart_interval_into_legacy": "E0277",
            "chart_batch_live_slice_borrow": "E0502",
        }
        for name, code in expected_codes.items():
            with self.subTest(name=name):
                case = MODULE.contract_examples()[name]
                self.assertEqual(set(case), {"source", "code", "tokens", "line", "external_legal_neighbor"})
                self.assertEqual(case["code"], code)
                self.assertTrue(case["tokens"] and all(type(token) is str and token for token in case["tokens"]))
                new_cases = {"chart_batch_private_update", "chart_selection_private_update",
                             "chart_selection_batch_lifetime", "chart_batch_descriptor_lifetime",
                             "chart_batch_entries_lifetime", "chart_interval_into_legacy", "chart_batch_live_slice_borrow"}
                neighbor = ({"file": "crates/time-core/tests/private/chart_batch_cases.rs",
                             "test": "chart_batch_cases::chart_batch_selects_genuine_majority_at_common_time"}
                            if name in new_cases else {
                                "file": "crates/time-core/tests/chart.rs",
                                "test": "weighted_tracking_uses_one_common_chart_reference_and_genuine_owner"})
                self.assertEqual(case["external_legal_neighbor"], neighbor)
                lines = case["source"].splitlines()
                self.assertTrue(type(case["line"]) is int and 1 <= case["line"] <= len(lines))
                primary = lines[case["line"] - 1]
                self.assertTrue(primary.startswith(" ") and primary.strip())
                if name == "chart_live_history_borrow":
                    self.assertIn("history.try_append(row)", primary)
                elif name == "chart_descriptor_lifetime":
                    self.assertIn("Known(&chart)", primary)
                elif name == "chart_interval_in_old_batch":
                    self.assertIn("FixtureProjectedBatch::new", primary)


    def test_predecessor_thirteen_negatives_retain_exact_meanings_and_sources(self):
        cases = MODULE.contract_examples()
        old_names = {"chart_frames_private_update", "chart_domain_private_update", "chart_chart_private_update",
                     "chart_view_private_update", "chart_projection_private_update", "chart_interval_private_update",
                     "chart_chart_default", "chart_projection_default", "chart_live_history_borrow",
                     "chart_descriptor_lifetime", "chart_to_legacy_projection", "chart_to_legacy_interval",
                     "chart_interval_in_old_batch"}
        original = {name: cases[name] for name in old_names}
        self.assertEqual(hashlib.sha256(json.dumps(original, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
                         "129b52c99dfb66754db82cf121efae3642a2df5b762804febf6bdab68ea8113f")

    def test_new_batch_record_fields_are_closed_and_cannot_be_omitted(self):
        raw = self.healthy()
        for field in ("batches", "minima", "selections", "batch_getters"):
            value = 22 if field == "batch_getters" else 1
            for bad in (raw.replace(f" {field}={value}", ""),
                        raw.replace(f" {field}={value}", f" {field}=0"),
                        raw.replace(f" {field}={value}", f" {field}={value} injected=1")):
                with self.subTest(field=field, bad=bad):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.contract_counts(bad)

    def test_new_lifetime_and_mutation_primary_lines_name_the_actual_boundary(self):
        expected = {"chart_selection_batch_lifetime": "batch.select(minimum)",
                    "chart_batch_descriptor_lifetime": "FixtureChartBatch::new(&chart",
                    "chart_batch_entries_lifetime": "FixtureChartBatch::new(chart,at,&entries)",
                    "chart_interval_into_legacy": "value.into()",
                    "chart_batch_live_slice_borrow": "values.reverse()"}
        for name, expression in expected.items():
            with self.subTest(name=name):
                case = MODULE.contract_examples()[name]
                self.assertIn(expression, case["source"].splitlines()[case["line"] - 1])
                self.assertTrue(case["source"].endswith("fn main(){}\n"))
        mutable = MODULE.contract_examples()["chart_batch_live_slice_borrow"]["source"]
        self.assertIn("let selected=batch.select(minimum)", mutable)
        self.assertLess(mutable.index("values.reverse()"), mutable.index("selected.selection()"))

    def test_descriptor_lifetime_probe_owns_its_non_copy_chart(self):
        case = MODULE.contract_examples()["chart_batch_descriptor_lifetime"]
        source = case["source"]
        # The descriptor is already checked, but intentionally local to bypass.
        # Copying it from &FixtureAffineChart adds an unrelated E0507 before the
        # intended returned-borrow diagnostic can qualify this restriction.
        signature = next(line for line in source.splitlines() if line.startswith("fn bypass"))
        self.assertIn("original:FixtureAffineChart", signature)
        self.assertNotIn("original:&", signature)
        self.assertIn(" let chart=original;\n", source)
        self.assertNotIn("*original", source)
        self.assertEqual(case["code"], "E0515")
        self.assertEqual(case["tokens"], ["chart", "cannot return value referencing"])
        self.assertEqual(source.splitlines()[case["line"] - 1],
                         " FixtureChartBatch::new(&chart,at,values).unwrap()")

    def test_six_garde_carriers_are_a_gap_map_not_mutation_credit(self):
        self.assertEqual(set(MODULE.GARDE_CARRIERS), {
            "FramesCandidate", "DomainCandidate", "AffineCandidate", "HistoryCandidate",
            "ProjectionCandidate", "TrackingPhaseCandidate",
        })
        self.assertTrue(all(type(value) is str and value for value in MODULE.GARDE_CARRIERS.values()))
        self.assertNotIn("killed", MODULE.contract_counts(self.healthy()))
        self.assertNotIn("native_exit", MODULE.contract_counts(self.healthy()))


if __name__ == "__main__":
    unittest.main()
