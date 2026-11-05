"""Direct-SMT fail-closed protocol and custody controls; execute no solver."""
from __future__ import annotations

from contextlib import redirect_stdout
import copy
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


V = load("direct_period_consumer_test", "verify_calendar_period.py")
SPEC = load("direct_period_spec_test", "calendar_period_smt.py")


def protocol(model):
    return "sat\n(" + "\n".join(f"({key} {value})" for key, value in model.items()) + ")\n"


def model_for(case):
    coordinates = (0, 0, 0) if case == "endpoint_min" else (
        (SPEC.U64_MAX, 399, 400) if case == "endpoint_max" else (1, 0, 0))
    witness = SPEC.evaluate_witness(case, *coordinates)
    return {key: witness[key] for key in V.MODEL_TERMS}


class ProtocolTests(unittest.TestCase):
    def test_one_exact_unsat_and_reordered_sat_values_are_accepted(self):
        self.assertIsNone(V.parse_output(" \nunsat\n", "unsat"))
        model = model_for("endpoint_max")
        self.assertEqual(V.parse_output(protocol(model), "sat"), model)

    def test_unknown_partial_error_and_extra_results_are_rejected(self):
        for text in ("", "unknown\n", "sat\n", "unsat\nunsat\n", "(error failure)\nunsat",
                     "warning\nunsat", "unsat\x00", "unsat\x1b[0m", "unsat\u2028", "x" * 8193):
            with self.subTest(text=text[:40]), self.assertRaises(V.Incomplete):
                V.parse_output(text, "unsat")

    def test_model_requires_every_unique_bounded_nonnegative_integer(self):
        good = protocol(model_for("endpoint_min"))
        key = next(iter(V.MODEL_TERMS))
        for text in ("sat\n", good + "sat", good.replace(f"({key} 0)", "", 1),
                     good.replace(f"({key} 0)", f"({key} 0) ({key} 0)", 1),
                     good.replace(f"({key} 0)", f"({key} true)", 1),
                     good.replace(f"({key} 0)", f"({key} (- 1))", 1),
                     good.replace(f"({key} 0)", f"({key} 00)", 1),
                     good.replace(f"({key} 0)", f"({key} {'1' * 129})", 1),
                     "sat " + "(" * 10 + ")" * 10):
            with self.subTest(text=text[:60]), self.assertRaises(V.Incomplete):
                V.parse_output(text, "sat")

    def test_noncompletion_signal_bool_exit_and_wrong_case_never_qualify(self):
        healthy = {"exit_code": 0, "output": "unsat\n", "failure": None}
        for changed in ({"exit_code": -9}, {"exit_code": 124}, {"exit_code": False},
                        {"failure": "timeout"}, {"output": "unknown\n"}):
            with self.subTest(changed=changed), self.assertRaises(V.Incomplete):
                V.qualify({**healthy, **changed}, "healthy", "unsat", SPEC)
        with self.assertRaises(V.Incomplete):
            V.qualify(healthy, "unknown", "unsat", SPEC)
        incomplete = dict(healthy)
        del incomplete["failure"]
        with self.assertRaises(V.Incomplete):
            V.qualify(incomplete, "healthy", "unsat", SPEC)

    def test_sat_requires_a_real_independently_validated_semantic_failure(self):
        for case in ("endpoint_min", "endpoint_max", "wrong_year_coefficient",
                     "omit_century_subtraction", "wrong_period"):
            model = model_for(case)
            native = {"exit_code": 0, "failure": None, "output": protocol(model)}
            self.assertTrue(V.qualify(native, case, "sat", SPEC)["qualified"])
            bad = dict(model)
            bad["forward_result"] += 1
            with self.assertRaises(ValueError):
                V.qualify({**native, "output": protocol(bad)}, case, "sat", SPEC)
            with self.assertRaises(V.Incomplete):
                V.qualify(native, case, "sat", SimpleNamespace(check_model=lambda *_: None))

    def test_expected_generator_schedule_and_smt_statuses_are_closed(self):
        self.assertEqual([(job["case"], job["restored_healthy"], job["expected_status"])
                          for job in SPEC.schedule()], list(V.CASES))
        self.assertEqual(set(SPEC.MODEL_TERMS), V.MODEL_TERMS)


class FrozenConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="period-consumer-controls-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.actual = self.base / "checkout"
        self.output = self.base / "evidence"
        self.frozen = self.output / "source"
        self.output.mkdir(mode=0o700)
        self.records = {}
        for name in V.INPUTS:
            data = (ROOT / name).read_bytes()
            for target in (self.actual / name, self.frozen / name):
                target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
                target.write_bytes(data)
            self.records[name] = {"sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}
        self.store = self.base / "solver"
        (self.store / "bin").mkdir(mode=0o700, parents=True)
        self.calls = []
        self.admission_calls = 0
        self.action = None
        self.run_action = None
        self.tool = {key: {} for key in (
            "inventory", "closure", "store_identity_before", "store_identity_after",
            "ancestor_identity_before", "ancestor_identity_after")}
        self.tool.update(name="z3", version="5.3.3", readonly_root=self.store,
                         executable=self.store / "bin/z3", path_dir=self.store / "bin",
                         sha256="1" * 64, policy_path=self.frozen / "policy/formal-solvers.json",
                         policy_sha256="2" * 64, manifest_sha256="3" * 64)

        def read_once(path, **_):
            return Path(path).read_bytes()

        def snapshot(work, folders):
            return {str(path.relative_to(work)): hashlib.sha256(path.read_bytes()).hexdigest()
                    for folder in folders for path in folder.rglob("*") if path.is_file()}

        def admit(*_):
            self.admission_calls += 1
            if self.action:
                self.action(self.admission_calls)
            return copy.deepcopy(self.tool)

        def run(root, work, command, **options):
            self.assertEqual(root, self.frozen)
            self.assertFalse(root.is_relative_to(work))
            self.assertEqual(options["readonly"], [self.store])
            self.assertEqual(options["path_dirs"], [self.store / "bin"])
            self.assertEqual(command[:2], [str(self.store / "bin/z3"), "-smt2"])
            expected_islands = list(work.glob("jobs/*/inputs"))
            self.assertTrue(all(any(path == seal or path.is_relative_to(seal)
                                    for seal in options["sealed"]) for path in expected_islands))
            self.assertTrue(any(options["cwd"].parent == seal or options["cwd"].parent.is_relative_to(seal)
                                for seal in options["sealed"]))
            for earlier in self.calls:
                folder = work / Path(earlier[2]).relative_to("/work").parent.parent
                self.assertTrue(any(folder == seal or folder.is_relative_to(seal)
                                    for seal in options["sealed"]))
            self.assertEqual(options["process_limit"], 128)
            self.assertEqual(options["build_jobs"], 1)
            index = len(self.calls)
            self.calls.append(command)
            if self.run_action:
                result = self.run_action(work, options)
                if result is not None:
                    return result
            case, _, status = V.CASES[index]
            output = "unsat\n" if status == "unsat" else protocol(model_for(case))
            return {"command": command, "exit_code": 0, "failure": None, "output": output}

        check = SimpleNamespace(read_once=read_once, input_snapshot=snapshot,
                                RUNTIME=SimpleNamespace(run=run))
        self.admission = SimpleNamespace(CHECK=check, admit=admit,
                                         _ancestors=lambda _: {"fixed": (1, 2, 3)})

    def execute(self):
        def loader(name, path):
            if name == "period_frozen_solver_admission":
                return self.admission
            self.assertEqual(path, self.frozen / "tools/formal/calendar_period_smt.py")
            return SPEC
        with patch.object(V, "module", side_effect=loader), redirect_stdout(io.StringIO()):
            result = V.run_frozen(self.actual, self.output, self.frozen, self.records)
        return result, json.loads((self.output / "summary.json").read_text())

    def test_fixed_seven_job_control_uses_only_its_frozen_policy_and_solver_scope(self):
        result, summary = self.execute()
        self.assertEqual(result, 0)
        self.assertEqual(summary["status"], "passed")
        self.assertEqual(len(self.calls), 7)
        self.assertTrue(all(job["qualified"] for job in summary["jobs"]))
        self.assertTrue(summary["original_and_frozen_inputs_unchanged"])
        self.assertTrue(summary["all_generated_inputs_and_completed_receipts_unchanged"])

    def test_generated_script_tamper_during_admission_stops_before_execution(self):
        def tamper(count):
            if count == 1:
                path = next((self.output / "work").rglob("proof.smt2"))
                path.chmod(0o600)
                path.write_bytes(path.read_bytes() + b"(assert false)\n")
        self.action = tamper
        result, summary = self.execute()
        self.assertEqual(result, 1)
        self.assertEqual(self.calls, [])
        self.assertFalse(summary["jobs"][0]["qualified"])

    def test_source_drift_during_solver_execution_never_qualifies(self):
        def drift(*_):
            path = self.actual / V.INPUTS[0]
            path.write_bytes(path.read_bytes() + b"\n// changed\n")
        self.run_action = drift
        result, summary = self.execute()
        self.assertEqual(result, 1)
        self.assertEqual(len(self.calls), 1)
        self.assertFalse(summary["jobs"][0]["qualified"])
        self.assertIn("final_error", summary)

    def test_solver_identity_drift_after_execution_never_qualifies(self):
        def drift(count):
            if count == 2:
                self.tool["sha256"] = "4" * 64
        self.action = drift
        result, summary = self.execute()
        self.assertEqual(result, 1)
        self.assertEqual(len(self.calls), 1)
        self.assertFalse(summary["jobs"][0]["qualified"])

    def test_timeout_stops_schedule_without_retry(self):
        self.run_action = lambda *_: {"exit_code": 124, "failure": "timeout", "output": ""}
        result, summary = self.execute()
        self.assertEqual(result, 1)
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(len(summary["jobs"]), 1)

    def test_a_later_job_cannot_silently_change_an_earlier_proof(self):
        def tamper(work, _):
            if len(self.calls) == 2:
                path = work / "jobs/01-endpoint_min/inputs/proof.smt2"
                path.chmod(0o600)
                path.write_bytes(path.read_bytes() + b"; later-job modification\n")
                path.chmod(0o400)
        self.run_action = tamper
        result, summary = self.execute()
        self.assertEqual(result, 1)
        self.assertEqual(summary["status"], "incomplete")
        self.assertEqual(len(self.calls), 2)

    def test_closeout_rechecks_prior_inputs_after_final_solver_admission(self):
        def tamper(count):
            if count == len(V.CASES) * 2 + 1:
                path = self.output / "work/jobs/01-endpoint_min/inputs/proof.smt2"
                path.chmod(0o600)
                path.write_bytes(path.read_bytes() + b"; closeout modification\n")
                path.chmod(0o400)
        self.action = tamper
        result, summary = self.execute()
        self.assertEqual(result, 1)
        self.assertEqual(summary["status"], "incomplete")
        self.assertIn("final_error", summary)

    def test_artifact_writer_preserves_targets_of_symlink_and_hardlink_aliases(self):
        good = self.output / "ordinary.json"
        V.write(good, {"healthy": True})
        self.assertEqual(json.loads(good.read_text()), {"healthy": True})
        for kind in ("symlink", "hardlink"):
            with self.subTest(kind=kind):
                sentinel = self.base / f"outside-{kind}.txt"
                sentinel.write_text("PRESERVE")
                alias = self.output / f"alias-{kind}.json"
                if kind == "symlink":
                    alias.symlink_to(sentinel)
                else:
                    os.link(sentinel, alias)
                rejected = False
                try:
                    V.write(alias, {"should_not_escape": True})
                except (OSError, V.Incomplete):
                    rejected = True
                self.assertEqual(sentinel.read_text(), "PRESERVE")
                self.assertTrue(rejected)

    def test_native_output_alias_cannot_redirect_the_host_artifact_write(self):
        sentinel = self.base / "outside-native.txt"
        sentinel.write_text("PRESERVE")
        def alias_output(_, options):
            if len(self.calls) == len(V.CASES):
                path = options["cwd"].parent / "execution.json"
                path.unlink()
                path.symlink_to(sentinel)
        self.run_action = alias_output
        result, summary = self.execute()
        self.assertEqual(sentinel.read_text(), "PRESERVE")
        self.assertEqual(result, 1)
        self.assertEqual(summary["status"], "incomplete")


if __name__ == "__main__":
    unittest.main()
