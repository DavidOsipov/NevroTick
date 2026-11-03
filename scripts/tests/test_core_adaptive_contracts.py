"""Literal controls for adaptive external fixtures and record admission; no Rust."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "core_adaptive_contract_controls", ROOT / "scripts/core-adaptive-contracts.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class AdaptiveContractTests(unittest.TestCase):
    """Independent literal case identities and malformed-record controls."""

    @staticmethod
    def healthy() -> str:
        """Return the independently spelled complete legal-call observation."""
        return (
            "CORE_ADAPTIVE_CONTRACT_PROFILE retained_points=0 retained_inputs=0 "
            "retained_selections=2 adaptive_observations=3 adaptive_appends=3 "
            "adaptive_histories=1 raw_parameter_validations=0 raw_ordinal_validations=0 "
            "raw_event_validations=0 external_normalizations=1 contexts=1 monotonic=3 "
            "coordinates=1 exchanges=1 getters=29\nlegal-core-contract\n")

    def test_literal_external_recipe_and_independent_auxiliary_work(self):
        expected = {
            "retained_points": 0, "retained_inputs": 0, "retained_selections": 2,
            "adaptive_observations": 3, "adaptive_appends": 3, "adaptive_histories": 1,
            "raw_parameter_validations": 0, "raw_ordinal_validations": 0,
            "raw_event_validations": 0, "external_normalizations": 1,
        }
        self.assertEqual(MODULE.legal_expected(), expected)
        self.assertEqual(MODULE.legal_auxiliary_expected(), {
            "contexts": 1, "monotonic": 3, "coordinates": 1, "exchanges": 1, "getters": 29})
        self.assertEqual(MODULE.contract_counts(self.healthy()), expected)
        self.assertNotIn("killed", MODULE.contract_counts(self.healthy()))
        self.assertNotIn("native_exit", MODULE.contract_counts(self.healthy()))

    def test_pipe_order_and_crlf_are_explicitly_permitted(self):
        lines = self.healthy().splitlines(keepends=True)
        for text in (self.healthy().replace("\n", "\r\n"), "".join(reversed(lines))):
            with self.subTest(text=text):
                self.assertEqual(MODULE.contract_counts(text), MODULE.legal_expected())

    def test_missing_partial_duplicate_and_unrelated_records_reject(self):
        healthy = self.healthy()
        marker = healthy.splitlines(keepends=True)[0]
        for text in (
            "", marker, "legal-core-contract\n", healthy.rstrip("\n"),
            healthy + marker, marker + marker, healthy + "unrelated\n",
            healthy.replace("adaptive_appends=3", "adaptive_appends=2"),
            healthy.replace("getters=29", "getters=28"),
            healthy.replace("contexts=1", "contexts=01"),
            healthy.replace("contexts=1", "contexts=true"),
            healthy.replace(" coordinates=1", ""),
            healthy.replace("getters=29", "getters=29 injected=1"),
            healthy.replace("retained_points=0 retained_inputs=0", "retained_inputs=0 retained_points=0"),
            healthy.replace("CORE_ADAPTIVE_CONTRACT_PROFILE", "CORE_ADAPTIVE_HISTORY_PROFILE"),
            "x" * 4097,
        ):
            with self.subTest(text=text):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.contract_counts(text)
        for value in (None, 1, b"not text", [healthy]):
            with self.subTest(value=value):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.contract_counts(value)

    def test_registry_has_thirteen_distinct_contract_diagnostics(self):
        cases = MODULE.contract_examples()
        expected = {
            "adaptive_history_private_update": "E0451",
            "adaptive_history_minimum_mutation": "E0616",
            "adaptive_report_private_update": "E0451",
            "adaptive_observation_shared_mutation": "E0594",
            "adaptive_report_shared_mutation": "E0594",
            "adaptive_fitted_row_escape": "E0515",
            "adaptive_decision_row_escape": "E0515",
            "adaptive_report_escape": "E0515",
            "tracking_payload_private_update": "E0451",
            "tracking_projection_override": "E0616",
            "tracking_shared_mutation": "E0594",
            "tracking_borrow_escape": "E0515",
            "tracking_payload_default": "E0277",
        }
        self.assertEqual({name: item["code"] for name, item in cases.items()
                          if item["code"] is not None}, expected)
        self.assertEqual(sum(item["code"] is None for item in cases.values()), 1)
        for name, specimen in cases.items():
            if specimen["code"] is None:
                continue
            with self.subTest(name=name):
                line = specimen["source"].splitlines()[specimen["line"] - 1]
                self.assertNotIn("use time_core", line)
                self.assertTrue(line.strip())
                neighbor = specimen["external_legal_neighbor"]
                expected_file = ("crates/time-core/tests/combination_lifecycle.rs"
                                 if name.startswith("tracking_")
                                 else "crates/time-core/tests/adaptive_history.rs")
                self.assertEqual(neighbor["file"], expected_file)
                self.assertIn("fn " + neighbor["test"] + "(",
                              (ROOT / expected_file).read_text())
                self.assertNotIn("unsafe", specimen["source"])

    def test_tracking_probes_target_exact_operations_and_readonly_neighbor(self):
        cases = MODULE.contract_examples()
        expected = {
            "tracking_payload_private_update": (
                "let _=FixtureTrackingEstimate{..value};",
                ["FixtureTrackingEstimate", "private"]),
            "tracking_projection_override": (
                "projection.tracking=Some(value);", ["tracking", "private"]),
            "tracking_shared_mutation": (
                "*projection.tracking().unwrap()=value;", ["cannot assign", "reference"]),
            "tracking_borrow_escape": (
                "projection.tracking().unwrap()",
                ["projection", "cannot return value referencing"]),
            "tracking_payload_default": (
                "let _:FixtureTrackingEstimate=Default::default();",
                ["FixtureTrackingEstimate", "Default"]),
        }
        for name, (expression, tokens) in expected.items():
            with self.subTest(name=name):
                specimen = cases[name]
                self.assertEqual(
                    specimen["source"].splitlines()[specimen["line"] - 1].strip(), expression)
                self.assertEqual(specimen["tokens"], tokens)
                self.assertEqual(specimen["external_legal_neighbor"], {
                    "file": "crates/time-core/tests/combination_lifecycle.rs",
                    "test": "distinct_same_context_histories_keep_their_own_tracking_payloads"})
                self.assertIn(
                    "use time_core::{FixtureAnchorProjection,FixtureTrackingEstimate};",
                    specimen["source"])
                self.assertNotIn("project_fixture_adaptive_tracking(", specimen["source"])
                self.assertNotIn("adaptive_calls", specimen)

    def test_legal_source_contains_actual_receivers_and_fixed_record(self):
        specimen = MODULE.contract_examples()[MODULE.LEGAL_NAME]
        source = specimen["source"]
        self.assertEqual(specimen["adaptive_calls"], MODULE.legal_expected())
        self.assertEqual(specimen["adaptive_auxiliary_calls"], MODULE.legal_auxiliary_expected())
        self.assertEqual(specimen["calls"], {
            "direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0})
        for owner in (
            "FixtureAdaptiveHistory::new(context,64)", "history.try_append(row)",
            "prepare_fixture_input(context,&rows)",
            "select_fixture_runs_with_retained(input,0,3)",
            "select_fixture_runs_with_retained(input,usize::MAX,3)",
            "Error::source(&wrapped)", "std::ptr::eq(fitted,decision)",
        ):
            self.assertIn(owner, source)
        self.assertIn("assert_eq!(calls.0,EXPECTED)", source)
        self.assertEqual(source.count("observation(&mut calls,context,phase,"), 3)
        self.assertEqual(source.count("fn main()"), 1)


if __name__ == "__main__":
    unittest.main()
