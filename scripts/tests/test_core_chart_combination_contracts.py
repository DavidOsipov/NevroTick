"""Pure source/inventory controls; these tests execute no Rust consumer."""

import hashlib
import importlib.util
import pathlib
import re
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "core_chart_combination_contracts", ROOT / "scripts/core-chart-combination-contracts.py")
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

EXPECTED = {
    "labels": 5, "contexts": 2, "monotonic": 4, "coordinates": 1, "exchanges": 1,
    "observations": 3, "histories": 1, "appends": 3, "frames": 1, "domains": 1,
    "charts": 1, "prepares": 1, "terms": 1, "tracked_projections": 1,
    "chart_enclosures": 1, "raw_enclosures": 1, "raw_batches": 1,
    "principal_parameters": 1, "combination_parameters": 1, "minima": 1,
    "strata": 1, "combination_seeds": 2, "chart_batches": 1, "chart_advances": 3,
    "bootstrap_getters": 5, "outcome_getters": 17, "estimate_getters": 21,
    "iterator_next": 2, "state_equalities": 2,
}


class ChartCombinationContractControls(unittest.TestCase):
    """Bind the planned external sources and reject misleading output records."""

    @staticmethod
    def healthy():
        return (
            "CORE_CHART_COMBINATION_CONTRACT_PROFILE labels=5 contexts=2 monotonic=4 "
            "coordinates=1 exchanges=1 observations=3 histories=1 appends=3 frames=1 "
            "domains=1 charts=1 prepares=1 terms=1 tracked_projections=1 chart_enclosures=1 "
            "raw_enclosures=1 raw_batches=1 principal_parameters=1 combination_parameters=1 "
            "minima=1 strata=1 combination_seeds=2 chart_batches=1 chart_advances=3 "
            "bootstrap_getters=5 outcome_getters=17 estimate_getters=21 iterator_next=2 "
            "state_equalities=2\nlegal-chart-combination-contract\n"
        )

    def test_legal_recipe_has_exact_direct_calls_and_no_extra_producers(self):
        self.assertEqual(MODULE.legal_expected(), EXPECTED)
        self.assertEqual(sum(EXPECTED.values()), 86)
        source = MODULE.legal_contract_source()
        self.assertEqual(hashlib.sha256(source.encode()).hexdigest(),
                         "28d8e269bffaf9bb808d826ff31607966c0d4171ab4d60293e00417bb6fff50f")
        for index, (name, count) in enumerate(EXPECTED.items()):
            written = len(re.findall(r"call!\(c," + str(index) + ",", source))
            # These two operations occur once per each of three literal instants.
            self.assertEqual(written, 1 if name in {"observations", "appends"} else count, name)
        self.assertIn("for at in [t0,t1,t2]", source)
        self.assertNotIn(".clone()", source)
        self.assertNotIn("FixturePrincipalState::", source)
        self.assertEqual(source.count("prepare_chart("), 1)
        self.assertEqual(source.count("project_tracking("), 1)
        self.assertEqual(source.count("FixtureCombinationState::seed("), 2)
        self.assertEqual(source.count("advance_chart("), 3)

    def test_known_facts_separate_source_target_and_actual_custody(self):
        source = MODULE.legal_contract_source()
        for text in (
                "FixtureAffineChart::new(frames,domain,1.0/128.0,1.0/1024.0)",
                "FixturePrincipalParameters::new(1.0/1024.0,0.0)",
                "estimate.source_context()),raw", "estimate.context()),target",
                "estimate.frame()),z", "estimate.mapping_generation()),j",
                "estimate.phase_seconds()),-5.0/512.0",
                "estimate.phase_rate_seconds_per_second()),-1.0/1024.0",
                "estimate.phase_standard_deviation_seconds()),0.0",
                "estimate.rate_standard_deviation_seconds_per_second()),0.0",
                "estimate.tracking_skew_seconds_per_second()),0.0",
                "estimate.batch()", "estimate.chart()", "estimate.principal_entry()",
                "estimate.anchor_observed_at()),t2", "estimate.latest_observed_at()),t2",
                "estimate.fit_reference()),t2", "estimate.projected_at()),t4",
                "FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate",
                "replay.combined()).is_none()", "replay.emitted_round()),None",
                "FixtureCombinationSourceStatus::NotEvaluated"):
            with self.subTest(text=text):
                self.assertIn(text, source)
        self.assertEqual(source.count("owner==control"), 2)

    def test_parser_accepts_only_complete_lf_crlf_records_in_either_pipe_order(self):
        row, sentinel = self.healthy().splitlines(keepends=True)
        for text in (row + sentinel, sentinel + row,
                     (row + sentinel).replace("\n", "\r\n"),
                     (sentinel + row).replace("\n", "\r\n")):
            with self.subTest(text=text):
                self.assertEqual(MODULE.contract_counts(text), EXPECTED)

    def test_parser_rejects_prefixes_duplicates_extra_fields_and_unrelated_success(self):
        text = self.healthy()
        row, sentinel = text.splitlines(keepends=True)
        for bad in ("", row, sentinel, text.rstrip("\n"), row + row, sentinel + sentinel,
                    text + "extra\n", text.replace("appends=3", "appends=2"),
                    text.replace("chart_advances=3", "chart_advances=03"),
                    text.replace("chart_advances=3", "chart_advances=true"),
                    text.replace(" charts=1", ""), text.replace(" prepares=1", " prepares=1 other=0"),
                    text.replace("labels=5 contexts=2", "contexts=2 labels=5"),
                    text.replace("legal-chart-combination-contract", "legal-chart-contract"),
                    text.replace("CORE_CHART_COMBINATION_CONTRACT_PROFILE", "CORE_CHART_CONTRACT_PROFILE"),
                    "x" * 4097, None, 1, True, b"output", [text]):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.contract_counts(bad)

    def test_closed_inventory_and_all_intended_diagnostic_descriptors(self):
        codes = {
            "chart_combination_estimate_private_update": "E0451",
            "chart_combination_outcome_private_update": "E0451",
            "chart_combination_estimate_default": "E0277",
            "chart_combination_outcome_default": "E0277",
            "chart_combination_estimate_unchecked_new": "E0599",
            "chart_combination_outcome_unchecked_new": "E0599",
            "chart_combination_bound_private_update": "E0616",
            "chart_combination_batch_lifetime": "E0515",
            "chart_combination_entries_lifetime": "E0515",
            "chart_combination_descriptor_lifetime": "E0515",
            "chart_combination_live_entries_borrow": "E0502",
            "chart_combination_estimate_to_legacy": "E0308",
            "chart_combination_outcome_to_legacy": "E0308",
            "chart_combination_estimate_into_legacy": "E0277",
            "chart_combination_principal_entry_legacy_uncertainty": "E0308",
            "chart_combination_principal_legacy_consumer": "E0308",
        }
        cases = MODULE.contract_examples()
        self.assertEqual(set(cases), {*codes, MODULE.LEGAL_NAME})
        self.assertEqual(len({case["source"] for case in cases.values()}), 17)
        legal = cases[MODULE.LEGAL_NAME]
        self.assertEqual(legal, {
            "source": MODULE.legal_contract_source(), "code": None,
            "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
            "chart_combination_calls": EXPECTED})
        neighbor = {
            "file": "crates/time-core/tests/chart_combination_lifecycle.rs",
            "test": "chart_combination_retains_real_provenance_and_observation_credit"}
        self.assertIn("fn " + neighbor["test"] + "(", (ROOT / neighbor["file"]).read_text())
        for name, code in codes.items():
            case = cases[name]
            with self.subTest(name=name):
                self.assertEqual(set(case), {"source", "code", "tokens", "line", "external_legal_neighbor"})
                self.assertEqual(case["code"], code)
                self.assertTrue(case["tokens"] and all(type(token) is str and token for token in case["tokens"]))
                self.assertEqual(case["external_legal_neighbor"], neighbor)
                lines = case["source"].splitlines()
                self.assertTrue(type(case["line"]) is int and 1 <= case["line"] <= len(lines))
                self.assertTrue(lines[case["line"] - 1].startswith(" "))
                self.assertTrue(lines[case["line"] - 1].strip())

    def test_lifetime_and_conversion_probes_target_actual_public_entrypoints(self):
        cases = MODULE.contract_examples()
        primary = lambda name: cases[name]["source"].splitlines()[cases[name]["line"] - 1]
        for name in ("batch", "entries", "descriptor"):
            self.assertIn("state.advance_chart(", primary("chart_combination_" + name + "_lifetime"))
        entry = cases["chart_combination_entries_lifetime"]
        descriptor = cases["chart_combination_descriptor_lifetime"]
        self.assertIn("->&'c FixtureChartInterval<'c>{", entry["source"])
        self.assertIn(".combined().unwrap().principal_entry()", primary("chart_combination_entries_lifetime"))
        self.assertEqual(entry["tokens"], ["entries", "cannot return value referencing"])
        self.assertIn("->&'c FixtureAffineChart{", descriptor["source"])
        self.assertIn(".combined().unwrap().chart()", primary("chart_combination_descriptor_lifetime"))
        self.assertEqual(descriptor["tokens"], ["chart", "cannot return value referencing"])
        for case in (entry, descriptor):
            self.assertNotIn("->FixtureChartCombinationOutcome", case["source"])
            self.assertEqual(case["code"], "E0515")
        self.assertIn("entries.reverse()", primary("chart_combination_live_entries_borrow"))
        self.assertIn("value.principal_entry()", primary("chart_combination_principal_entry_legacy_uncertainty"))
        self.assertIn("value.principal()", primary("chart_combination_principal_legacy_consumer"))
        self.assertIn("value.quantities.phase_error", primary("chart_combination_bound_private_update"))


class ReferenceInputContractControls(unittest.TestCase):
    """Keep the singleton registry and complete transport separate from S158."""

    @staticmethod
    def expected():
        return {**EXPECTED, "reference_states": 1, "reference_readiness": 2,
                "reference_admissions": 4, "reference_feedback": 4,
                "reference_combined": 1, "reference_rounds": 2,
                "reference_estimate_getters": 2}

    @classmethod
    def healthy(cls):
        return ("CORE_REFERENCE_INPUT_CONTRACT_PROFILE" + "".join(
            f" {name}={value}" for name, value in cls.expected().items())
                + "\nlegal-reference-input-contract\n")

    def test_reference_registry_is_closed_and_keeps_old_source_identity(self):
        examples = MODULE.reference_contract_examples()
        self.assertEqual(set(examples), {
            "legal_reference_input_and_feedback", "reference_input_token_private_update",
            "reference_input_token_clone", "reference_input_estimate_move",
            "reference_input_chart_feedback_borrow", "reference_input_entry_feedback_borrow",
            "reference_input_projection_feedback_borrow"})
        self.assertEqual(sum(row["code"] is None for row in examples.values()), 1)
        self.assertEqual(sum(row["code"] is not None for row in examples.values()), 6)
        self.assertEqual(hashlib.sha256(MODULE.legal_contract_source().encode()).hexdigest(),
                         "28d8e269bffaf9bb808d826ff31607966c0d4171ab4d60293e00417bb6fff50f")
        self.assertEqual(len(MODULE.contract_examples()), 17)

    def test_reference_source_funds_added_calls_without_new_upstream_producers(self):
        self.assertEqual(MODULE.reference_legal_expected(), self.expected())
        self.assertEqual(sum(self.expected().values()), 102)
        source = MODULE.reference_legal_contract_source()
        for index, (name, count) in enumerate(self.expected().items()):
            written = len(re.findall(r"call!\(c," + str(index) + ",", source))
            self.assertEqual(written, 1 if name in {"observations", "appends"} else count, name)
        for text, count in (("prepare_chart(", 1), ("project_tracking(", 1),
                            ("FixtureAdaptiveHistory::new(", 1),
                            ("FixtureCombinationState::seed(", 2), ("advance_chart(", 3)):
            self.assertEqual(source.count(text), count)
        self.assertEqual(source.count("reference.admit_reference_input(&ready)"), 4)
        self.assertNotIn("CORE_CHART_COMBINATION_CONTRACT_PROFILE", source)
        self.assertNotIn(".clone()", source)

    def test_reference_complete_healthy_transport_has_lf_crlf_neighbors(self):
        for text in (self.healthy(), "\n".join(reversed(self.healthy().splitlines())) + "\n"):
            for candidate in (text, text.replace("\n", "\r\n")):
                with self.subTest(candidate=candidate):
                    self.assertEqual(MODULE.reference_contract_counts(candidate), self.expected())

    def test_reference_transport_rejects_partial_replay_and_foreign_counter_rows(self):
        healthy = self.healthy()
        malformed = [None, "", healthy[:-1], healthy + healthy, healthy + "\n",
                     healthy.replace("reference_admissions=4", "reference_admissions=0"),
                     healthy.replace("reference_admissions=4", "reference_admissions=+4"),
                     healthy.replace("reference_admissions=4", "reference_admissions=4.0"),
                     healthy.replace("reference_states=1", "reference_states=1 extra=1"),
                     healthy.replace(" reference_rounds=2", ""),
                     healthy.replace("legal-reference-input-contract", "legal-chart-combination-contract"),
                     ChartCombinationContractControls.healthy(), "x" * 4097]
        for text in malformed:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.reference_contract_counts(text)
        self.assertEqual(MODULE.reference_contract_counts(healthy), self.expected())

    def test_reference_negative_primary_spans_bind_each_intended_bypass(self):
        examples = MODULE.reference_contract_examples()
        intended = {"reference_input_token_private_update": ("E0451", "FixtureReferenceInput{..value}"),
                    "reference_input_token_clone": ("E0599", "value.clone()"),
                    "reference_input_estimate_move": ("E0507", "*input.combined()")}
        intended.update({f"reference_input_{name}_feedback_borrow":
                         ("E0502", "owner.receive_feedback")
                         for name in ("chart", "entry", "projection")})
        for name, (code, primary) in intended.items():
            row = examples[name]
            self.assertEqual(row["code"], code)
            self.assertIn(primary, row["source"].splitlines()[row["line"] - 1])
            self.assertEqual(row["external_legal_neighbor"], {
                "file": "crates/time-core/tests/private/reference_input_readiness.rs",
                "test": "genuine_ready_input_is_readonly_and_replay_has_no_new_credit"})
        self.assertIn("let view=*input.combined().principal_entry();",
                      examples["reference_input_entry_feedback_borrow"]["source"])
        self.assertIn("let view=*input.combined().principal_entry().projection();",
                      examples["reference_input_projection_feedback_borrow"]["source"])


if __name__ == "__main__":
    unittest.main()
