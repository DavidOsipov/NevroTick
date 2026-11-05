"""Preparation controls only: no solver, native compiler or product claim."""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import stat
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("calendar_period_smt_test", HERE / "calendar_period_smt.py")
PERIOD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PERIOD)


class PreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-calendar-period-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        self.source_path = self.source / PERIOD.SOURCE
        self.source_path.parent.mkdir(parents=True)
        self.original = (ROOT / PERIOD.SOURCE).read_bytes()
        self.source_path.write_bytes(self.original)
        self.inputs = self.base / "inputs"

    def prepare(self, case="healthy", **kwargs):
        return PERIOD.prepare_calendar_period(self.source, self.inputs, case, **kwargs)

    def test_exact_source_helpers_caller_and_shared_finite_model_prefix(self):
        result = self.prepare()
        correspondence = result["correspondence"]
        self.assertEqual(correspondence["whole_sha256"], hashlib.sha256(self.original).hexdigest())
        self.assertEqual(correspondence["shared_model_prefix_bytes"], 4_059)
        self.assertEqual(correspondence["shared_model_prefix_sha256"],
                         hashlib.sha256(self.original[:4_059]).hexdigest())
        for record in correspondence["selected"]:
            first, last = record["byte_offsets_half_open"]
            selected = self.original[first:last]
            self.assertEqual(selected.decode(), record["exact_source"])
            self.assertEqual(hashlib.sha256(selected).hexdigest(), record["sha256"])
        self.assertEqual([record["role"] for record in correspondence["selected"]],
                         ["forward_start", "cycle_start", "period_call", "cycle_days"])
        self.assertEqual(correspondence["units"]["cycle_days"], 146_097)
        self.assertEqual(self.source_path.read_bytes(), self.original)

    def test_exact_rust_precedence_and_left_association(self):
        variable = ("var", "years")
        expected = ("add", ("sub", ("add", ("mul", ("int", 365), variable),
                                              ("div", variable, ("int", 4))),
                                    ("div", variable, ("int", 100))),
                          ("div", variable, ("int", 400)))
        self.assertEqual(PERIOD.expression_tree("365 * years + years / 4 - years / 100 + years / 400", "years"), expected)
        for expression, name in (("365*years+years/4-years/100+years/400", "years"),
                                 ("365 * years + years / 4 - (years / 100 + years / 400)", "years"),
                                 ("365 * years + years / 4 - years / 100 + years / 401", "years"),
                                 ("365 * other + other / 4 - other / 100 + other / 400", "other")):
            with self.subTest(expression=expression), self.assertRaises(ValueError):
                PERIOD.expression_tree(expression, name)

    def test_healthy_negates_all_required_results_without_assuming_them(self):
        result = self.prepare()
        text = (self.inputs / "proof.smt2").read_text()
        self.assertEqual(result["query"]["expected_status"], "unsat")
        self.assertIn("(assert (not (and machine_bounds_ok period_identity)))", text)
        self.assertEqual(text.count("(check-sat)"), 1)
        self.assertEqual(text.count("(assert "), 2)
        self.assertNotIn("(get-value", text)
        self.assertNotIn("(assert period_identity)", text)
        self.assertNotIn("(assert machine_bounds_ok)", text)
        self.assertEqual(result["query"]["assumed_results"], [])
        self.assertFalse(result["native_profile_qualified"])
        self.assertEqual(result["native_commands_executed"], 0)

    def test_full_raw_cycle_envelope_and_independent_complete_g_domain(self):
        result = self.prepare()
        query = result["query"]
        self.assertEqual(query["domain"], {"cycles": [0, 2**64 - 1], "year": [0, 399], "cycle_year": [0, 400]})
        text = (self.inputs / "proof.smt2").read_text()
        self.assertIn("(<= cycles 18446744073709551615)", text)
        self.assertIn("(<= year 399)", text)
        self.assertIn("(<= cycle_year 400)", text)
        self.assertNotIn("(forall", text)
        self.assertEqual(PERIOD.MAX_YEARS, 400 * (2**64 - 1) + 399)
        self.assertEqual(PERIOD.U64_MAX, 2**64 - 1)
        self.assertEqual(PERIOD.U128_MAX, 2**128 - 1)
        self.assertEqual(PERIOD.U32_MAX, 2**32 - 1)
        self.assertIn("forward_source_precondition", {check["label"] for check in query["required_machine_checks"]})

    def test_each_typed_arithmetic_node_has_range_and_division_premises(self):
        query = self.prepare()["query"]
        labels = {check["label"] for check in query["required_machine_checks"]}
        nodes = query["arithmetic_nodes"]
        self.assertEqual(len(nodes), 27)
        self.assertEqual(sum(node["rust_width"] == 32 for node in nodes), 14)
        for node in nodes:
            self.assertIn(node["name"] + "_fits_u" + str(node["rust_width"]), labels)
            if node["operation"] == "div":
                self.assertIn(node["name"] + "_positive_divisor", labels)
                self.assertIn(node["name"] + "_nonnegative_numerator", labels)
        self.assertEqual(len(query["unsigned_widenings"]), 5)
        self.assertEqual({entry["source"] for entry in query["unsigned_widenings"]}, {
            "u128::from(u64::MAX)", "u128::from(cycles)", "u128::from(year)",
            "u128::from(CYCLE_DAYS)", "u128::from(cycle_start(year))"})
        for index in range(5):
            self.assertIn("exact_unsigned_widening_" + str(index), labels)

    def test_three_distinct_semantic_faults_have_independent_intended_witnesses(self):
        expected = {"wrong_year_coefficient": (145_697, 146_097),
                    "omit_century_subtraction": (146_101, 146_097),
                    "wrong_period": (146_097, 146_096)}
        scripts = set()
        for case, outputs in expected.items():
            folder = self.base / case
            result = PERIOD.prepare_calendar_period(self.source, folder, case)
            query = result["query"]
            witness = query["independently_evaluated_witness"]
            self.assertEqual((witness["cycles"], witness["year"], witness["cycle_year"]), (1, 0, 0))
            self.assertEqual((witness["forward_result"], witness["rhs_result"]), outputs)
            self.assertTrue(witness["machine_bounds_hold"])
            self.assertFalse(witness["period_holds"])
            self.assertEqual(witness["violated_property"], "period_identity")
            self.assertEqual(query["expected_status"], "sat")
            text = (folder / "proof.smt2").read_text()
            self.assertIn("(assert (not (and machine_bounds_ok period_identity)))", text)
            self.assertNotIn("(= cycles 1)", text)
            self.assertIn("(get-value (cycles year cycle_year", text)
            self.assertIn("Rust source unchanged", query["mutation"]["kind"])
            scripts.add(text)
        self.assertEqual(len(scripts), 3)
        self.assertEqual(self.source_path.read_bytes(), self.original)

    def test_separate_endpoint_feasibility_queries_retain_positive_contract(self):
        for case, values in (("endpoint_min", (0, 0, 0)), ("endpoint_max", (2**64 - 1, 399, 400))):
            folder = self.base / case
            result = PERIOD.prepare_calendar_period(self.source, folder, case)
            witness = result["query"]["independently_evaluated_witness"]
            self.assertEqual(tuple(witness[key] for key in ("cycles", "year", "cycle_year")), values)
            self.assertTrue(witness["period_holds"])
            self.assertTrue(witness["machine_bounds_hold"])
            self.assertEqual(result["query"]["expected_status"], "sat")
            self.assertTrue(result["query"]["endpoint_feasibility"])
            text = (folder / "proof.smt2").read_text()
            self.assertIn("(assert (and machine_bounds_ok period_identity))", text)
            self.assertIn(f"(= cycles {values[0]})", text)
            self.assertEqual(text.count("(check-sat)"), 1)

    def test_schedule_requires_both_endpoints_three_faults_and_exact_restoration(self):
        jobs = PERIOD.schedule()
        self.assertEqual([job["case"] for job in jobs], ["endpoint_min", "endpoint_max", "healthy",
                         "wrong_year_coefficient", "omit_century_subtraction", "wrong_period", "healthy"])
        self.assertEqual([job["expected_status"] for job in jobs], ["sat", "sat", "unsat", "sat", "sat", "sat", "unsat"])
        self.assertFalse(jobs[2]["restored_healthy"])
        self.assertTrue(jobs[-1]["restored_healthy"])
        original = self.prepare()
        restored = PERIOD.prepare_calendar_period(self.source, self.base / "restored", "healthy", restored_healthy=True)
        self.assertEqual(original["script_sha256"], restored["script_sha256"])
        self.assertEqual(original["correspondence"], restored["correspondence"])

    def test_source_drift_unknown_case_and_existing_evidence_fail_before_writes(self):
        self.source_path.write_bytes(self.original + b"\n// drift\n")
        with self.assertRaisesRegex(ValueError, "complete calendar harness changed"):
            self.prepare()
        self.assertFalse(self.inputs.exists())
        self.source_path.write_bytes(self.original)
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("assume_period")
        with self.assertRaisesRegex(ValueError, "Only the full healthy"):
            self.prepare("wrong_period", restored_healthy=True)
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        (self.inputs / "evidence").write_bytes(b"preserved")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual((self.inputs / "evidence").read_bytes(), b"preserved")

    def test_function_pattern_drift_is_rejected_even_after_a_whole_hash_rebind(self):
        modified = self.original.replace(b"years <= 400 *", b"years < 400 *", 1)
        self.source_path.write_bytes(modified)
        with patch.object(PERIOD, "SOURCE_SHA256", hashlib.sha256(modified).hexdigest()), \
                patch.object(PERIOD, "MODEL_PREFIX_SHA256", hashlib.sha256(modified[:4_059]).hexdigest()):
            with self.assertRaisesRegex(ValueError, "exact source pattern"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_two_sealed_outputs_hashes_and_explicit_qualification_limits(self):
        result = self.prepare()
        self.assertEqual({path.name for path in self.inputs.iterdir()}, {"proof.smt2", "correspondence.json"})
        self.assertEqual(stat.S_IMODE(self.inputs.stat().st_mode), 0o500)
        for path in self.inputs.iterdir():
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o400)
        self.assertEqual(result["script_sha256"], hashlib.sha256((self.inputs / "proof.smt2").read_bytes()).hexdigest())
        self.assertEqual(result["metadata_sha256"], hashlib.sha256((self.inputs / "correspondence.json").read_bytes()).hexdigest())
        receipt = json.loads((self.inputs / "correspondence.json").read_text())
        self.assertEqual(receipt["entrypoint"], "proof.smt2")
        self.assertTrue(receipt["preparation_only"])
        self.assertIn("trusted", receipt["query"]["correspondence_limit"])
        self.assertIn("product Rust calendar kernel proof", receipt["omitted"])
        self.assertIn("strict status/model", " ".join(receipt["remaining_prerequisites"]))
        returned = {key: value for key, value in result.items() if key != "metadata_sha256"}
        returned["entrypoint"] = "proof.smt2"
        self.assertEqual(receipt, returned)

    def test_consumer_api_freezes_exact_read_inputs_and_returns_top_level_contract(self):
        self.assertEqual(PERIOD.input_names(), (PERIOD.SOURCE, "tools/formal/calendar_period_smt.py"))
        result = PERIOD.prepare(self.source, self.inputs, "endpoint_min")
        self.assertEqual(result["entrypoint"], self.inputs / "proof.smt2")
        self.assertEqual(result["expected_status"], "sat")
        self.assertEqual(result["required_model_terms"], list(PERIOD.MODEL_TERMS))


class IndependentWitnessTests(unittest.TestCase):
    def test_model_checker_requires_exact_exported_integers_and_intended_observation(self):
        for case in ("endpoint_min", "endpoint_max", *PERIOD.CONTROLS):
            values = ((0, 0, 0) if case == "endpoint_min" else
                      (PERIOD.U64_MAX, 399, 400) if case == "endpoint_max" else (1, 0, 0))
            observed = PERIOD.evaluate_witness(case, *values)
            model = {name: observed[name] for name in PERIOD.MODEL_TERMS}
            self.assertEqual(PERIOD.check_model(case, model), observed)
            for broken in ({**model, "forward_result": model["forward_result"] + 1},
                           {**model, "unexpected": 0}, {**model, "cycles": True}):
                with self.subTest(case=case, broken=broken), self.assertRaises(ValueError):
                    PERIOD.check_model(case, broken)
        positive = PERIOD.evaluate_witness("healthy", 0, 0, 0)
        model = {name: positive[name] for name in PERIOD.MODEL_TERMS}
        with self.assertRaisesRegex(ValueError, "requires UNSAT"):
            PERIOD.check_model("healthy", model)
        with self.assertRaisesRegex(ValueError, "intended period failure"):
            PERIOD.check_model("wrong_period", model)
        with self.assertRaisesRegex(ValueError, "required positive point"):
            PERIOD.check_model("endpoint_max", model)

    def test_all_cycle_year_values_and_reachable_large_cycle_neighbors(self):
        for years in range(401):
            witness = PERIOD.evaluate_witness("healthy", 0, min(years, 399), years)
            self.assertTrue(witness["period_holds"])
            self.assertTrue(witness["machine_bounds_hold"])
        for cycles in (0, 1, 2**32 - 1, 2**32, 2**63 - 1, 2**63, 2**64 - 1):
            for year in (0, 1, 99, 100, 399):
                witness = PERIOD.evaluate_witness("healthy", cycles, year, 400)
                self.assertTrue(witness["period_holds"])
                self.assertTrue(witness["machine_bounds_hold"])
        self.assertEqual(PERIOD.evaluate_witness("healthy", 1, 0, 400)["cycle_independent_result"], 146_097)

    def test_invalid_models_do_not_satisfy_witness_checker(self):
        for values in ((-1, 0, 0), (2**64, 0, 0), (0, 400, 0), (0, 0, 401), (True, 0, 0), (0, 0.0, 0)):
            with self.subTest(values=values), self.assertRaises(ValueError):
                PERIOD.evaluate_witness("healthy", *values)
        with self.assertRaisesRegex(ValueError, "Unknown"):
            PERIOD.evaluate_witness("assume_identity", 0, 0, 0)


if __name__ == "__main__":
    unittest.main()
