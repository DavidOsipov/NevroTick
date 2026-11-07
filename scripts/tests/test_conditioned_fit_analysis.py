"""Focused N2 role controls; synthetic observations execute no product jobs."""
from __future__ import annotations

import copy
import importlib.util
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("conditioned_fit_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
FIT_CASES = frozenset({"exact_fit_control", "conditioned_fit::arithmetic::tests::validator_control"})
FIT_DOCS = (("crates/time-core/src/conditioned_fit.rs", "conditioned_fit::ConditionedFit", False, 1),
            ("crates/time-core/src/conditioned_fit.rs", "conditioned_fit::ConditionedFit", True, 1))


def native_log(names):
    """Complete literal native records, not execution evidence."""
    return ("running " + str(len(names)) + " tests\n"
            + "".join("test " + name + " ... ok\n" for name in names)
            + "test result: ok. " + str(len(names))
            + " passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


class ConditionedFitAnalysisTests(unittest.TestCase):
    """Challenge role isolation, exact inventories, owned logs and gate ordering."""

    def setUp(self):
        """Synthetic minima exercise admission mechanics, never numerical truth."""
        self.addCleanup(patch.stopall)
        patch.object(MODULE, "CONDITIONED_FIT_REQUIRED_TESTS", FIT_CASES).start()
        patch.object(MODULE, "CONDITIONED_FIT_REQUIRED_DOCTESTS", FIT_DOCS).start()

    def policy(self):
        """Fresh synthetic N2 data contains no N1 bindings or equivalences."""
        ordinary = sorted(FIT_CASES | {"other_library_control"})
        docs = ["crates/time-core/src/conditioned_fit.rs - conditioned_fit::ConditionedFit (line 10)",
                "crates/time-core/src/conditioned_fit.rs - conditioned_fit::ConditionedFit (line 20) - compile fail"]
        return {**copy.deepcopy(MODULE.CONDITIONED_FIT_FIXED),
                "source_file_count": 1, "candidate_sha256": "a" * 64,
                "mutation_inventories": {name: {"count": 1, "sha256": "b" * 64}
                                         for name in MODULE.CONDITIONED_FIT_SUBJECTS},
                "mutation_equivalences": [],
                "test_inventories": {"coverage": ordinary, "baseline": sorted([*ordinary, *docs])}}

    def test_scope_and_dispatch_cannot_import_historical_credit(self):
        role = MODULE._conditioned_fit_role()
        self.assertEqual(role.scope.select("all"), {
            "conditioned_fit": "crates/time-core/src/conditioned_fit.rs",
            "conditioned_arithmetic": "crates/time-core/src/conditioned_fit/arithmetic.rs"})
        self.assertEqual(role.coverage_tests, ("conditioned_fit",))
        self.assertNotEqual(role.scope.policy, MODULE.CONDITIONED_WINDOW_SCOPE.policy)
        self.assertEqual(set(MODULE.CONDITIONED_WINDOW_SCOPE.select("all")),
                         {"conditioned_window", "window_variance"})
        self.assertEqual(MODULE.CONDITIONED_WINDOW_FIXED["mutation_jobs"], 8)
        with patch.object(MODULE, "main_conditioned_fit", return_value=0) as selected, \
             patch.object(MODULE, "main_conditioned_window") as historical:
            self.assertEqual(MODULE.main(["--subject", "conditioned_fit", "--lane", "baseline",
                                          "--output-dir", "/owned/output"]), 0)
            selected.assert_called_once()
            historical.assert_not_called()
        with patch.object(MODULE, "execute_conditioned_fit") as selected, \
             patch.object(MODULE, "read_policy") as historical:
            MODULE.execute(SimpleNamespace(subject="conditioned_fit"), {})
            selected.assert_called_once()
            historical.assert_not_called()

    def test_required_controls_are_call_time_and_empty_minima_fail_closed(self):
        policy = self.policy()
        self.assertEqual(MODULE._conditioned_test_inventories(policy, MODULE._conditioned_fit_role()),
                         policy["test_inventories"])
        for name, empty in (("CONDITIONED_FIT_REQUIRED_TESTS", frozenset()),
                            ("CONDITIONED_FIT_REQUIRED_DOCTESTS", ())):
            with patch.object(MODULE, name, empty), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(policy, MODULE._conditioned_fit_role())
        with patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_TESTS", frozenset({"runtime_n1_control"})):
            self.assertIn("runtime_n1_control", MODULE._conditioned_window_role().required_tests)
            self.assertNotIn("runtime_n1_control", MODULE._conditioned_fit_role().required_tests)
        with patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_TESTS", frozenset()), \
             self.assertRaises(MODULE.Incomplete):
            MODULE._conditioned_window_role()

    def test_mutant_bindings_require_both_new_owners_and_combined_cap(self):
        healthy = self.policy()
        role = MODULE._conditioned_fit_role()
        self.assertEqual(set(MODULE._conditioned_inventory_bindings(healthy, role, required=True)),
                         set(MODULE.CONDITIONED_FIT_SUBJECTS))
        for bindings in ({}, {"conditioned_fit": {"count": 1, "sha256": "b" * 64}},
                         {name: {"count": 1, "sha256": "b" * 64}
                          for name in MODULE.CONDITIONED_WINDOW_SUBJECTS},
                         {name: {"count": 300, "sha256": "b" * 64}
                          for name in MODULE.CONDITIONED_FIT_SUBJECTS}):
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_inventory_bindings({**healthy, "mutation_inventories": bindings},
                                                       role, required=True)

    def test_actual_test_identities_reject_missing_duplicates_and_failures(self):
        policy, role = self.policy(), MODULE._conditioned_fit_role()
        names = policy["test_inventories"]["coverage"]
        self.assertEqual(MODULE._conditioned_test_execution(native_log(names), policy, role, doctests=False)
                         ["tests_executed"], len(names))
        for raw in (native_log(names[:-1]), native_log([*names, names[0]]),
                    native_log(names).replace(" ... ok", " ... FAILED", 1)):
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_execution(raw, policy, role, doctests=False)

    def test_baselines_require_each_actual_success_and_reject_linked_log(self):
        policy, role = self.policy(), MODULE._conditioned_fit_role()
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            for subject in MODULE.CONDITIONED_FIT_SUBJECTS:
                directory = work / "mutation" / subject / "mutants.out"
                directory.mkdir(parents=True)
                (directory / "baseline.log").write_text(native_log(policy["test_inventories"]["baseline"]))
                (directory / "linked.log").symlink_to(directory / "baseline.log")
                baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"}
                output = directory / "outcomes.json"
                output.write_text(json.dumps({"outcomes": [baseline]}))
                self.assertEqual(MODULE._conditioned_baseline(work, policy, subject, role)["doctests"], 2)
                for rows in ([], [baseline, baseline], [{**baseline, "summary": "Failure"}],
                             [{**baseline, "log_path": "../escape.log"}],
                             [{**baseline, "log_path": "linked.log"}]):
                    output.write_text(json.dumps({"outcomes": rows}))
                    with self.subTest(subject=subject, rows=rows), self.assertRaises(MODULE.Incomplete):
                        MODULE._conditioned_baseline(work, policy, subject, role)

    def test_each_owner_coverage_and_finalization_failure_prevent_pass(self):
        for failure in (None, "conditioned_fit", "conditioned_arithmetic", "finish"):
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                role, policy, report, events = MODULE._conditioned_fit_role(), self.policy(), {}, []
                identity = {"synthetic.rs": "c" * 64}
                policy.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                              inputs_sha256={name: "d" * 64 for name in role.bound_inputs})
                def run(label, command, timeout, **_options):
                    events.append(label)
                    if label == "baseline":
                        return native_log(policy["test_inventories"]["baseline"])
                    self.assertEqual(label, "coverage")
                    self.assertEqual([command[i + 1] for i, value in enumerate(command) if value == "--test"],
                                     ["conditioned_fit"])
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename,
                         "summary": {"lines": {"count": 10, "covered": 8 if owner == failure else 9}}}
                        for owner, filename in role.scope.select("all").items()]}]}))
                    return native_log(policy["test_inventories"]["coverage"])
                def finish():
                    events.append("finish")
                    if failure == "finish":
                        raise MODULE.Incomplete("final custody changed")
                session = SimpleNamespace(product_identity=identity, source=work, work=work,
                                          tools=work / "tools", run=run, finish=finish)
                def mutation(scope, subjects, admitted, actual_work, tools, execute, actual_report, lane):
                    self.assertEqual((scope, subjects, lane), (role.scope, role.scope.select("all"), "mutation"))
                    self.assertEqual(set(actual_report["coverage"]), set(subjects))
                    events.append("mutation")
                args = SimpleNamespace(subject="conditioned_fit", lane="all", tool_admission_profile="full",
                                       output_dir=work / "output")
                with patch.object(MODULE, "read_conditioned_fit_policy", return_value=policy), \
                     patch.object(MODULE, "inventory_document", return_value=policy), \
                     patch.object(MODULE.PRODUCT, "sha256", return_value="d" * 64), \
                     patch.object(MODULE.COMMON, "_require_private_output"), \
                     patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=6 * 1024 ** 3)), \
                     patch.object(MODULE.COMMON, "Session", return_value=session), \
                     patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=mutation) as mutations:
                    if failure is None:
                        MODULE.execute_conditioned_fit(args, report)
                        self.assertTrue(report["qualification_passed"])
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute_conditioned_fit(args, report)
                        self.assertFalse(report["qualification_passed"])
                    self.assertEqual(events[-1], "finish")
                    self.assertEqual(mutations.call_count, int(failure in (None, "finish")))


if __name__ == "__main__":
    unittest.main()
