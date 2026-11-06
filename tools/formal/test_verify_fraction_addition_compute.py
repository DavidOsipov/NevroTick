"""Closed compute-consumer controls; execute no Rust compiler or proof solver."""
from __future__ import annotations

import copy
from contextlib import ExitStack, redirect_stderr
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


HERE = Path(__file__).resolve().parent


def load(name, filename):
    specification = importlib.util.spec_from_file_location(name, HERE / filename)
    module = importlib.util.module_from_spec(specification)
    sys.modules[name] = module
    specification.loader.exec_module(module)
    return module


RUNNER = load("addition_compute_controller_controls", "verify_fraction_addition_compute.py")
FIXTURE = load("addition_compute_ordinary_json_fixture", "test_verify.py")
START_UNIX_NS = 1793915211361402794
START_MONOTONIC_NS = 11879194829883


class ClosedComputeControls(unittest.TestCase):
    def test_flags_change_only_solver_and_the_dedicated_harness_timeout(self):
        baseline = list(RUNNER.BASE.KANI_BASELINE_FLAGS)
        check = SimpleNamespace(PROFILES={"kani_flags": list(baseline)})
        expected = list(baseline)
        expected[expected.index("--solver") + 1] = "z3"
        expected[expected.index("--harness-timeout") + 1] = "300s"
        self.assertEqual(RUNNER.compute_flags(check), expected)
        self.assertEqual(check.PROFILES["kani_flags"], baseline)
        self.assertEqual(list(RUNNER.BASE.KANI_BASELINE_FLAGS), baseline)
        self.assertEqual(baseline[baseline.index("--harness-timeout") + 1], "60s")

    def test_changed_baseline_flags_never_enter_the_compute_role(self):
        baseline = list(RUNNER.BASE.KANI_BASELINE_FLAGS)
        changed_timeout = list(baseline)
        changed_timeout[changed_timeout.index("--harness-timeout") + 1] = "300s"
        for flags in (baseline + ["-Z", "function-contracts"], baseline + ["-Z", "stubbing"],
                      changed_timeout, baseline[:-1], tuple(baseline), None):
            check = SimpleNamespace(PROFILES={"kani_flags": flags})
            with self.subTest(flags=flags), self.assertRaises(RUNNER.Incomplete):
                RUNNER.compute_flags(check)

    def test_only_the_original_complete_addition_group_is_selected(self):
        actual = RUNNER.schedule()
        self.assertEqual([index for index, _ in actual], [6, 7, 8, 9, 10])
        expected = [("healthy", "fraction_addition_full_domain", None, False),
                    ("always_error", "fraction_addition_control_success", "addition required success", False),
                    ("wrong_carry", "fraction_addition_control_carry", "addition exact carry", False),
                    ("wrong_residual", "fraction_addition_control_residual", "addition exact residual", False),
                    ("healthy", "fraction_addition_full_domain", None, True)]
        for (_, job), (case, harness, failure, restored) in zip(actual, expected, strict=True):
            self.assertEqual(job, {"subject": "fraction_addition", "case": case,
                                  "harness": "fraction_arithmetic::" + harness,
                                  "intended_failure": failure, "restored_healthy": restored})
        self.assertEqual([job for _, job in actual], RUNNER.BASE.jobs("fraction-arithmetic")[5:10])
        self.assertEqual(RUNNER.BASE.MAX_JOBS, 12)
        self.assertEqual(len(RUNNER.BASE.jobs("all")), 12)

    def test_original_source_start_bounds_both_wall_and_monotonic_elapsed_time(self):
        self.assertEqual(RUNNER.PREPARATION_STARTED_UNIX_NS, START_UNIX_NS)
        self.assertEqual(RUNNER.PREPARATION_STARTED_MONOTONIC_NS, START_MONOTONIC_NS)
        for wall_elapsed, monotonic_elapsed, remaining in ((10, 20, 3580), (20, 10, 3580),
                                                           (3599.5, 3599, 0.5)):
            with self.subTest(wall=wall_elapsed, monotonic=monotonic_elapsed), \
                 patch.object(RUNNER.time, "time_ns", return_value=START_UNIX_NS + int(wall_elapsed * 1e9)), \
                 patch.object(RUNNER.time, "monotonic_ns",
                              return_value=START_MONOTONIC_NS + int(monotonic_elapsed * 1e9)):
                self.assertAlmostEqual(RUNNER.remaining_budget(), remaining)
        for wall_elapsed, monotonic_elapsed in ((3600, 0), (0, 3600), (3601, 1), (1, 3601), (-1, 1), (1, -1)):
            with self.subTest(wall=wall_elapsed, monotonic=monotonic_elapsed), \
                 patch.object(RUNNER.time, "time_ns", return_value=START_UNIX_NS + wall_elapsed * 10**9), \
                 patch.object(RUNNER.time, "monotonic_ns", return_value=START_MONOTONIC_NS + monotonic_elapsed * 10**9):
                with self.assertRaises(RUNNER.Incomplete):
                    RUNNER.remaining_budget()

    def test_fixed_output_preflight_has_a_legal_neighbor_and_no_retry_path(self):
        with tempfile.TemporaryDirectory() as directory, patch.object(Path, "home", return_value=Path(directory)):
            expected = Path(directory) / ".local/share/nevrotick" / RUNNER.FIXED_OUTPUT_LEAF
            expected.parent.mkdir(parents=True)
            with patch.object(RUNNER, "remaining_budget", return_value=100):
                self.assertEqual(RUNNER.output_preflight(str(expected)), expected)
                for value in (str(expected) + "-retry", "/tmp/" + RUNNER.FIXED_OUTPUT_LEAF,
                              RUNNER.FIXED_OUTPUT_LEAF, str(expected.parent) + "//" + RUNNER.FIXED_OUTPUT_LEAF):
                    with self.subTest(output=value), self.assertRaises(RUNNER.Incomplete):
                        RUNNER.output_preflight(value)
            self.assertFalse(expected.exists())

    def test_cli_admits_no_solver_budget_harness_or_job_overrides(self):
        class StopBeforeOutput(RuntimeError):
            pass
        with tempfile.TemporaryDirectory() as directory, patch.object(Path, "home", return_value=Path(directory)):
            output = Path(directory) / ".local/share/nevrotick" / RUNNER.FIXED_OUTPUT_LEAF
            output.parent.mkdir(parents=True)
            arguments = ["--output-dir", str(output)]
            with patch.object(RUNNER, "remaining_budget", return_value=100), \
                 patch.object(RUNNER.BASE.os, "umask"), \
                 patch.object(RUNNER.BASE, "fresh_output", side_effect=StopBeforeOutput) as fresh:
                with self.assertRaises(StopBeforeOutput):
                    RUNNER.main(arguments)
                fresh.assert_called_once_with(str(output))
            for extra in (["--solver-backend", "cadical"], ["--harness-timeout", "600s"],
                          ["--timeout", "900"], ["--only-jobs", "6"], ["--start-at", "7"],
                          ["--subject", "fraction-addition-partitions"]):
                with self.subTest(extra=extra), redirect_stderr(io.StringIO()), \
                     patch.object(RUNNER.BASE, "fresh_output") as fresh:
                    with self.assertRaises(SystemExit) as stopped:
                        RUNNER.main(arguments + extra)
                    self.assertEqual(stopped.exception.code, 2)
                    fresh.assert_not_called()


class ComputeEffectControls(unittest.TestCase):
    """Use real preparation/qualification with synthetic admission and native JSON."""

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-compute-controller-")
        self.addCleanup(temporary.cleanup)
        self.parent = Path(temporary.name)
        self.work, self.home = self.parent / "work", self.parent / "home"
        self.work.mkdir(mode=0o700)
        self.stack = ExitStack()
        self.addCleanup(self.stack.close)
        self.stack.enter_context(patch.object(Path, "home", return_value=self.home))
        self.stack.enter_context(patch.object(RUNNER, "remaining_budget", return_value=1200))
        self.pointer = self.home / ".local/share/nevrotick/analysis-installation.json"
        self.pointer.parent.mkdir(parents=True)
        self.pointer.write_bytes(b'{"fixture":"compute admission"}\n')
        self.pointer.chmod(0o600)
        self.store = self.parent / "store"
        self.payload = self.store / "kani/payload/kani-0.68.0"
        self.compiler = self.store / RUNNER.BASE.KANI_COMPILER
        self.stable = self.store / "stable"
        self.solver_root = self.parent / "solver"
        for path in (self.store / RUNNER.BASE.KANI_ENTRYPOINT, self.compiler / "bin/rustc",
                     self.compiler / "bin/rustdoc", self.solver_root / "bin/z3"):
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"# synthetic file: never executed\n")
            path.chmod(0o700)
        self.stable.mkdir()
        self.source = self.work / "source"
        self.check = SimpleNamespace(
            PROFILES={"kani_flags": list(RUNNER.BASE.KANI_BASELINE_FLAGS)},
            read_once=lambda path, **kwargs: Path(path).read_bytes(), decode=json.loads,
            digest=lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest(),
            input_snapshot=self.snapshot,
            RUNTIME=SimpleNamespace(run=Mock(side_effect=self.native), _owned_work=Mock()))
        self.admitted = {"admission_consumer": RUNNER.CONSUMER, "admission_profile": "kani-proof",
                         "base": self.store, "stable": self.stable, "policy_sha256": "a" * 64,
                         "inventories": [{"root": "kani", "sha256": "b" * 64}],
                         "readonly_roots": (self.payload, self.compiler),
                         "policy": {"tools": {"kani": {"entrypoint": RUNNER.BASE.KANI_ENTRYPOINT,
                                                        "toolchain": RUNNER.BASE.KANI_COMPILER}}}}
        self.check.admit_fraction_addition_compute_pilot = Mock(return_value=self.admitted)
        self.solver = {"name": "z3", "version": RUNNER.BASE.Z3_VERSION,
                       "readonly_root": self.solver_root, "executable": self.solver_root / "bin/z3",
                       "path_dir": self.solver_root / "bin", "policy_path": self.source / "policy/formal-solvers.json",
                       "policy_sha256": "c" * 64, "manifest_sha256": "d" * 64,
                       "executable_sha256": RUNNER.BASE.Z3_EXECUTABLE_SHA256,
                       "inventory": {"bin/z3": {"sha256": "e" * 64}},
                       "closure": {"host_runtime": "existing-public-host-trusted-base-not-rehashed",
                                   "per_command_library_hashes_checked": False},
                       "store_identity_before": {"fixture": "store"}, "store_identity_after": {"fixture": "store"},
                       "ancestor_identity_before": {"fixture": "ancestor"},
                       "ancestor_identity_after": {"fixture": "ancestor"}}
        self.solver_admission = self.stack.enter_context(
            patch.object(RUNNER.BASE, "admit_z3_solver", return_value=self.solver))
        self.records = RUNNER.BASE.freeze_inputs(self.check, self.source, RUNNER.input_names())
        self.baseline = self.work / "baseline-inputs"
        projection = load("compute_effect_real_projection", "fraction_arithmetic_projection.py")
        projection.prepare_fraction_arithmetic(RUNNER.ROOT, self.baseline, "fraction_addition", "healthy",
                                              RUNNER.ROOT / "tools/formal/fraction_arithmetic_harness.rs")
        self.assertEqual({name: self.check.digest(self.baseline / name) for name in RUNNER.MODEL_SHA256},
                         RUNNER.MODEL_SHA256)
        self.retained, self.anchor = [], {}
        RUNNER.BASE.retain_artifacts(self.check, self.work, self.retained, [self.source, self.baseline])

    @staticmethod
    def snapshot(work, roots):
        result = {}
        for root in roots:
            for path in [root, *sorted(root.rglob("*"))] if root.is_dir() else [root]:
                key = path.relative_to(work).as_posix()
                if path.is_dir():
                    result[key] = {"kind": "directory"}
                else:
                    data = path.read_bytes()
                    result[key] = {"kind": "file", "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}
        return result

    def native(self, root, work, command, **options):
        """Persist synthetic ordinary JSON; never spawn the supplied command."""
        entry = command[1]
        job = self.current_job
        healthy = job["case"] == "healthy"
        if healthy:
            assertions, covers = RUNNER.BASE.additional_suites(self.source).expected_contract(job["subject"], job["harness"])
            checks = [{"category": "assertion", "description": label, "status": "Success",
                       "location": {"file": entry}} for label in sorted(assertions)]
            checks += [{"category": "cover", "description": label, "status": "Satisfied"} for label in sorted(covers)]
        else:
            checks = [{"category": "assertion", "description": job["intended_failure"], "status": "Failure",
                       "location": {"file": entry}}]
        report = FIXTURE.result(job, checks, healthy)
        report["tools"]["solvers"][0]["name"] = "z3"
        report["cbmc"][0]["configuration"]["solver"] = "z3"
        report["harness_metadata"][0]["source"]["file"] = entry
        output = work / command[command.index("--export-json") + 1].removeprefix("/work/")
        output.write_text(json.dumps(report))
        return {"exit_code": 0 if healthy else 1, "failure": None, "output": "synthetic controller control"}

    def invoke(self, index=6):
        job = dict(RUNNER.schedule(self.source)[index - 6][1])
        self.current_job = job
        return RUNNER.execute_job(self.check, self.work, self.source, self.records, job, index,
                                  self.retained, self.baseline, anchor=self.anchor)

    def test_five_fixed_effect_boundaries_reuse_ordinary_qualification_and_restore_complete_bytes(self):
        with patch.object(RUNNER.BASE, "qualify", wraps=RUNNER.BASE.qualify) as qualify:
            records = [self.invoke(index) for index in range(6, 11)]
        self.assertTrue(all(record["expected_outcome_qualified"] for record in records),
                        [record.get("qualification_error") for record in records])
        self.assertEqual(qualify.call_count, 5)
        self.assertTrue(all(call.kwargs == {"expected_solver": "z3"} for call in qualify.call_args_list))
        self.assertEqual(self.check.RUNTIME.run.call_count, 5)
        self.assertEqual(self.solver_admission.call_count, 10)
        for call in self.check.RUNTIME.run.call_args_list:
            command, options = call.args[2], call.kwargs
            self.assertEqual(command[command.index("--harness-timeout") + 1], "300s")
            self.assertEqual(command[command.index("--solver") + 1], "z3")
            self.assertEqual(options["readonly"], (self.payload, self.compiler, self.solver_root))
            self.assertEqual(options["path_dirs"], [self.solver_root / "bin", self.payload / "bin", self.compiler / "bin"])
            self.assertEqual({name: options[name] for name in ("timeout", "cpu_seconds", "build_jobs", "address_space_bytes", "process_limit")},
                             {"timeout": 600, "cpu_seconds": 600, "build_jobs": 2,
                              "address_space_bytes": 8 * 1024**3, "process_limit": 128})
            self.assertTrue(options["sealed"])
        self.assertTrue(records[-1]["restored_generated_input_matches_healthy"])
        first = Path(records[0]["output"]) / "inputs"
        restored = Path(records[-1]["output"]) / "inputs"
        self.assertEqual(RUNNER.BASE.generated_identity(self.check, first),
                         RUNNER.BASE.generated_identity(self.check, restored))

    def test_wrong_consumer_is_refused_before_any_runtime_effect(self):
        self.admitted["admission_consumer"] = "ordinary-kani-proof"
        result = self.invoke()
        self.assertFalse(result["expected_outcome_qualified"])
        self.assertIn("consumer", result["qualification_error"])
        self.check.RUNTIME.run.assert_not_called()

    def test_duplicate_or_unadmitted_solver_root_is_refused_before_runtime(self):
        self.solver["readonly_root"] = self.payload
        result = self.invoke()
        self.assertFalse(result["expected_outcome_qualified"])
        self.check.RUNTIME.run.assert_not_called()

    def test_stale_frozen_source_is_refused_before_runtime(self):
        changed = self.source / "tools/formal/verify.py"
        changed.write_bytes(changed.read_bytes() + b"\n# changed frozen source\n")
        result = self.invoke()
        self.assertFalse(result["expected_outcome_qualified"])
        self.check.RUNTIME.run.assert_not_called()

    def test_changed_generated_model_is_refused_before_runtime(self):
        registry = RUNNER.BASE.additional_suites(self.source)
        original = registry.prepare
        def changed_prepare(loader, source, inputs, job, index):
            metadata = original(loader, source, inputs, job, index)
            path = inputs / "fraction_arithmetic_selected.rs"
            path.write_bytes(path.read_bytes() + b"\n// altered generated model\n")
            return metadata
        registry.prepare = changed_prepare
        with patch.object(RUNNER.BASE, "additional_suites", return_value=registry):
            result = self.invoke()
        self.assertFalse(result["expected_outcome_qualified"])
        self.assertIn("Actual Rust", result["qualification_error"])
        self.check.RUNTIME.run.assert_not_called()

    def test_expired_source_card_is_refused_before_runtime(self):
        with patch.object(RUNNER, "remaining_budget", side_effect=RUNNER.Incomplete("original card expired")):
            result = self.invoke()
        self.assertFalse(result["expected_outcome_qualified"])
        self.check.RUNTIME.run.assert_not_called()

    def test_remaining_budget_clips_the_actual_runtime_wall_limit(self):
        with patch.object(RUNNER, "remaining_budget", return_value=12.5):
            result = self.invoke()
        self.assertTrue(result["expected_outcome_qualified"], result.get("qualification_error"))
        self.assertEqual(self.check.RUNTIME.run.call_args.kwargs["timeout"], 12.5)
        self.assertEqual(result["runtime_effective_wall_seconds"], 12.5)

    def test_solver_readmission_drift_prevents_qualification(self):
        changed = copy.deepcopy(self.solver)
        changed["manifest_sha256"] = "f" * 64
        self.solver_admission.side_effect = [self.solver, changed]
        with patch.object(RUNNER.BASE, "qualify", wraps=RUNNER.BASE.qualify) as qualify:
            result = self.invoke()
        self.assertFalse(result["expected_outcome_qualified"])
        self.assertIn("Z3 solver changed", result["qualification_error"])
        self.check.RUNTIME.run.assert_called_once()
        qualify.assert_not_called()


if __name__ == "__main__":
    unittest.main()
