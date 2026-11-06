"""Fixed cohort/source-custody controls; no admission, compiler or native job."""
from __future__ import annotations

import importlib.util
import io
import json
import os
from pathlib import Path
import py_compile
import shutil
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch
from contextlib import redirect_stdout


ROOT = Path(__file__).resolve().parents[2]


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


BASE = load(ROOT / "tools/formal/verify.py", "request_timing_unbound_test_runner")


class RequestTimingConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-request-timing-consumer-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.checkout = self.root / "checkout"
        for name in BASE.input_names("request-timing"):
            target = self.checkout / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            shutil.copy2(ROOT / name, target)
        self.entry = load(self.checkout / "tools/formal/verify_request_timing.py", "request_timing_test_entry")
        self.runner = self.entry._runner
        self.check = self.runner.module("request_timing_test_admission", self.checkout / "scripts/check-analysis.py")
        self.names = self.runner.input_names("request-timing")
        self.projection = self.runner.module("request_timing_test_projection", self.checkout / "tools/formal/request_timing_projection.py")
        self.work = self.root / "work"
        self.work.mkdir(mode=0o700)
        self.source = self.work / "source"

    def freeze(self):
        with patch.object(self.runner, "cohort_output", return_value=self.work):
            return self.runner.freeze_inputs(self.check, self.source, self.names)

    def test_healthy_captured_closure_matches_frozen_manifest(self):
        records = self.freeze()
        self.runner.require_sources(self.check, self.source, records, "request-timing")
        snapshot = self.entry._binding.snapshot()
        self.assertEqual(set(snapshot), self.runner.REQUEST_TIMING_PYTHON_NAMES)
        self.assertLess(set(snapshot), set(records))
        for name, identity in snapshot.items():
            self.assertEqual(identity, records[name])

    def test_each_loaded_python_edit_fails_before_source_freezing(self):
        for name in sorted(self.runner.REQUEST_TIMING_PYTHON_NAMES):
            path = self.checkout / name
            original = path.read_bytes()
            with self.subTest(name=name):
                try:
                    path.write_bytes(original + b"\n# changed after execution\n")
                    with self.assertRaisesRegex(RuntimeError, "changed"):
                        self.freeze()
                    self.assertFalse(self.source.exists())
                finally:
                    path.write_bytes(original)

    def test_frozen_product_harness_and_python_drift_fail_existing_guard(self):
        records = self.freeze()
        for name in ("crates/ntp-exchange/src/context.rs", "tools/formal/request_timing_harness.rs",
                     "tools/formal/request_timing_projection.py"):
            path = self.source / name
            original = path.read_bytes()
            with self.subTest(name=name):
                try:
                    path.write_bytes(original + b"\n")
                    with self.assertRaisesRegex(self.runner.Incomplete, "drift"):
                        self.runner.require_sources(self.check, self.source, records, "request-timing")
                finally:
                    path.write_bytes(original)
        self.runner.require_sources(self.check, self.source, records, "request-timing")

    def test_mapped_loader_executes_current_captured_code_and_keeps_frozen_data_path(self):
        self.freeze()
        current = self.checkout / "tools/formal/request_timing_projection.py"
        frozen = self.source / "tools/formal/request_timing_projection.py"
        cached = object()
        with patch.dict(sys.modules, {"request_timing_mapped_projection": cached}):
            result = self.runner.module("request_timing_mapped_projection", frozen)
        self.assertEqual(result.__file__, str(current))
        self.assertIsNot(result, cached)
        self.assertEqual(frozen.read_bytes(), current.read_bytes())
        self.assertEqual(self.runner.BOUND_SOURCE_ROOT, self.source)

    def test_mapped_loader_rejects_foreign_undeclared_and_redirected_paths(self):
        self.freeze()
        foreign = self.root / "outside.py"
        foreign.write_text("raise RuntimeError('must not execute')\n")
        undeclared = self.checkout / "tools/formal/undeclared.py"
        undeclared.write_text("raise RuntimeError('must not execute')\n")
        redirected = self.source / "tools/formal/redirected.py"
        redirected.symlink_to(self.source / "tools/formal/request_timing_projection.py")
        traversal = self.source / "tools/formal/../formal/request_timing_projection.py"
        for path in (foreign, undeclared, redirected, traversal):
            with self.subTest(path=path), self.assertRaises(self.runner.Incomplete):
                self.runner.module("request_timing_bad_mapping", path)

    def test_bound_loader_bypasses_timestamp_valid_stale_bytecode(self):
        path = self.checkout / "tools/formal/request_timing_projection.py"
        original = path.read_bytes()
        old = original + b"\nSTALE_CONTROL = 1\n"
        current = original + b"\nSTALE_CONTROL = 2\n"
        self.assertEqual(len(old), len(current))
        path.write_bytes(old)
        before = path.stat()
        py_compile.compile(str(path), doraise=True, invalidation_mode=py_compile.PycInvalidationMode.TIMESTAMP)
        path.write_bytes(current)
        os.utime(path, ns=(before.st_atime_ns, before.st_mtime_ns))
        self.assertEqual(load(path, "request_timing_stale_control").STALE_CONTROL, 1)
        # A fresh captured wrapper has not previously admitted either projection.
        entry = load(self.checkout / "tools/formal/verify_request_timing.py", "request_timing_stale_entry")
        self.assertEqual(entry._runner.module("request_timing_fresh_projection", path).STALE_CONTROL, 2)

    def test_fixed_entrypoint_rejects_cli_and_supplies_only_fixed_arguments(self):
        with patch.object(self.runner, "main", return_value=23) as dispatch:
            with patch.object(sys, "argv", ["verify_request_timing.py"]):
                self.assertEqual(self.entry.main(), 23)
            self.assertEqual(dispatch.call_args.args, ([
                "--output-dir", str(self.runner.request_timing_output()), "--subject", "request-timing",
                "--tool-admission-profile", "kani-proof", "--solver-backend", "cadical"],))
            dispatch.reset_mock()
            with patch.object(sys, "argv", ["verify_request_timing.py", "--help"]), \
                 self.assertRaisesRegex(self.runner.Incomplete, "no CLI"):
                self.entry.main()
            dispatch.assert_not_called()
        with self.assertRaises(TypeError):
            self.entry.run_request_timing_campaign(output="elsewhere")

    def test_full_only_new_selection_and_old_limits_remain(self):
        self.assertEqual(self.runner.job_limit("request-timing"), 5)
        self.assertEqual(len(self.runner.selected_jobs("request-timing", 1)), 5)
        for start, only in ((2, None), (1, [1, 2, 3, 4, 5]), (1, [5]), (1, [2, 1])):
            with self.subTest(start=start, only=only), self.assertRaises(self.runner.Incomplete):
                self.runner.selected_jobs("request-timing", start, only)
        self.assertEqual(BASE.job_limit("all"), 12)
        self.assertEqual(BASE.job_limit("fraction-addition-partitions"), 16)
        self.assertEqual(len(BASE.jobs("all")), 12)
        self.assertEqual(BASE.TOOL_ADMISSION_PROFILES, ("full", "kani-proof"))
        self.assertEqual(BASE.SOLVER_BACKENDS, ("cadical", "z3"))
        self.assertFalse(set(BASE.input_names("all")) & self.runner.REQUEST_TIMING_PYTHON_NAMES
                         - {"tools/formal/verify.py", "scripts/check-analysis.py", "scripts/analysis-runtime.py",
                            "scripts/analysis-controls.py", "scripts/check-environment.py"})

    def test_unbound_or_overridden_programmatic_entry_fails_before_effects(self):
        with self.assertRaisesRegex(BASE.Incomplete, "captured fixed wrapper"):
            BASE.partition_preflight("request-timing", "kani-proof", "cadical", None, str(BASE.request_timing_output()))
        for profile, solver, root, output in (("full", "cadical", None, str(self.runner.request_timing_output())),
                                              ("kani-proof", "z3", None, str(self.runner.request_timing_output())),
                                              ("kani-proof", "cadical", "/tmp/solver", str(self.runner.request_timing_output())),
                                              ("kani-proof", "cadical", None, "/tmp/other")):
            with self.subTest(profile=profile, solver=solver, root=root, output=output), \
                 self.assertRaisesRegex(self.runner.Incomplete, "fixed output"):
                self.runner.partition_preflight("request-timing", profile, solver, root, output)
        with self.assertRaisesRegex(self.runner.Incomplete, "complete fixed campaign"):
            self.runner.execute_job(self.check, self.work, self.source, {}, "request-timing",
                                    self.runner.jobs("request-timing")[0], 1, tool_admission_profile="kani-proof")
        self.assertFalse((self.work / "jobs").exists())

    def test_first_admission_reserve_is_required_without_creating_output(self):
        output = str(self.runner.request_timing_output())
        with patch.object(self.runner, "partition_remaining", return_value=1019), \
             patch.object(self.runner, "cohort_resources") as resources, \
             self.assertRaisesRegex(self.runner.Incomplete, "1020 seconds"):
            self.runner.partition_preflight("request-timing", "kani-proof", "cadical", None, output)
        resources.assert_not_called()
        with patch.object(self.runner, "partition_remaining", return_value=1020), \
             patch.object(self.runner, "cohort_resources") as resources:
            self.runner.partition_preflight("request-timing", "kani-proof", "cadical", None, output)
        resources.assert_called_once_with()

    def test_new_healthy_contract_requires_every_declared_assertion_and_cover(self):
        fixture = load(ROOT / "tools/formal/test_verify.py", "request_timing_json_fixture")
        job = self.runner.jobs("request-timing")[0]
        assertions, covers = self.runner.additional_suites().expected_contract(job["subject"], job["harness"])
        checks = [{"category": "assertion", "description": name, "status": "Success",
                   "location": {"file": fixture.ENTRY}} for name in sorted(assertions)]
        checks += [{"category": "cover", "description": name, "status": "Satisfied"}
                   for name in sorted(covers)]
        report = fixture.result(job, checks, True)
        self.assertEqual(self.runner.qualify(report, {"exit_code": 0, "failure": None}, job,
                                            fixture.ENTRY)["checks"], len(checks))
        for index in range(len(checks)):
            altered = fixture.result(job, checks[:index] + checks[index + 1:], True)
            with self.subTest(missing=checks[index]["description"]), self.assertRaises(self.runner.Incomplete):
                self.runner.qualify(altered, {"exit_code": 0, "failure": None}, job, fixture.ENTRY)

    def test_resource_limit_excludes_only_exact_targets_and_cleanup_preserves_receipts(self):
        job = self.runner.jobs("request-timing")[0]
        folder = self.work / "jobs" / f"01-{job['subject']}-{job['case']}-{job['harness']}"
        target = folder / "target"
        target.mkdir(mode=0o700, parents=True)
        with (target / "disposable").open("wb") as stream:
            stream.truncate(129 * 1024**2)
        receipt = folder / "execution.json"
        receipt.write_text('{"failure":"retained"}')
        custody = SimpleNamespace(RUNTIME=SimpleNamespace(_owned_work=lambda work, root: None))
        disk = SimpleNamespace(free=4 * 1024**3)
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=disk):
            self.assertEqual(self.runner.cohort_resources(self.work)["retained_bytes"], receipt.stat().st_size)
            removed = self.runner.cleanup_cohort_targets(custody, self.work)
            self.assertEqual(removed, [target.relative_to(self.work).as_posix()])
            self.assertFalse(target.exists())
            self.assertEqual(receipt.read_text(), '{"failure":"retained"}')
            with (folder / "unexpected-target").open("wb") as stream:
                stream.truncate(129 * 1024**2)
            with self.assertRaisesRegex(self.runner.Incomplete, "128 MiB"):
                self.runner.cohort_resources(self.work)

    def test_low_reserve_or_aliased_target_is_rejected(self):
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=SimpleNamespace(free=3 * 1024**3 - 1)), \
             self.assertRaisesRegex(self.runner.Incomplete, "reserve"):
            self.runner.cohort_resources(self.work)
        job = self.runner.jobs("request-timing")[0]
        folder = self.work / "jobs" / f"01-{job['subject']}-{job['case']}-{job['harness']}"
        folder.mkdir(mode=0o700, parents=True)
        (folder / "target").symlink_to(self.checkout, target_is_directory=True)
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=SimpleNamespace(free=4 * 1024**3)), \
             self.assertRaisesRegex(self.runner.Incomplete, "aliases"):
            self.runner.cohort_resources(self.work)

    def test_low_reserve_does_not_block_safe_target_recovery_or_erase_evidence(self):
        job = self.runner.jobs("request-timing")[0]
        folder = self.work / "jobs" / f"01-{job['subject']}-{job['case']}-{job['harness']}"
        target = folder / "target"
        target.mkdir(mode=0o700, parents=True)
        (target / "disposable").write_bytes(b"discard only this target")
        receipt = folder / "execution.json"
        receipt.write_text('{"failure":"retained"}')
        self.source.mkdir(mode=0o700)
        source = self.source / "retained.rs"
        source.write_bytes(b"// frozen source evidence\n")
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=SimpleNamespace(free=1)):
            removed = self.runner.cleanup_cohort_targets(self.check, self.work)
            self.assertEqual(removed, [target.relative_to(self.work).as_posix()])
            self.assertFalse(target.exists())
            self.assertEqual(receipt.read_text(), '{"failure":"retained"}')
            self.assertEqual(source.read_bytes(), b"// frozen source evidence\n")
            with self.assertRaisesRegex(self.runner.Incomplete, "reserve"):
                self.runner.cohort_resources(self.work)

    def test_cleanup_rejects_foreign_root_and_alias_before_deleting_safe_targets(self):
        targets = []
        for index, job in enumerate(self.runner.jobs("request-timing")[:2], 1):
            folder = self.work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}"
            folder.mkdir(mode=0o700, parents=True)
            targets.append(folder / "target")
        targets[0].mkdir(mode=0o700)
        sentinel = targets[0] / "still-here"
        sentinel.write_bytes(b"preserved")
        targets[1].symlink_to(self.checkout, target_is_directory=True)
        # Even a stubbed runtime owner cannot authorize a foreign root or alias.
        custody = SimpleNamespace(RUNTIME=SimpleNamespace(_owned_work=lambda work, root: None))
        with self.assertRaisesRegex(self.runner.Incomplete, "exact canonical"):
            self.runner.cleanup_cohort_targets(custody, self.work)
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             self.assertRaisesRegex(self.runner.Incomplete, "aliased"):
            self.runner.cleanup_cohort_targets(custody, self.work)
        self.assertEqual(sentinel.read_bytes(), b"preserved")
        self.assertTrue(targets[1].is_symlink())

    def test_campaign_retains_incomplete_status_after_low_space_target_recovery(self):
        self.work.rmdir()
        disk = SimpleNamespace(free=4 * 1024**3)
        target_paths = []

        def failed_job(check, work, source, records, subject, job, index, **options):
            folder = work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}"
            target = folder / "target"
            target.mkdir(mode=0o700, parents=True)
            (target / "disposable").write_bytes(b"owned target")
            (folder / "execution.json").write_text('{"failure":"retained diagnostic"}')
            target_paths.append(target)
            disk.free = 1
            return {"output": str(folder), "expected_outcome_qualified": False,
                    "qualification_error": "controlled no-native setup failure"}

        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner, "request_timing_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=disk), \
             patch.object(self.runner, "partition_remaining", return_value=2000), \
             patch.object(self.runner, "execute_job", side_effect=failed_job), \
             patch.object(self.runner, "admission") as admission, redirect_stdout(io.StringIO()):
            result = self.entry.run_request_timing_campaign()
        admission.assert_not_called()
        self.assertEqual(result, 1)
        summary = json.loads((self.work / "summary.json").read_text())
        self.assertEqual(summary["status"], "incomplete")
        self.assertIn("reserve", summary["final_error"])
        self.assertIn("reserve", summary["cleanup_error"])
        self.assertEqual(summary["removed_disposable_targets"],
                         [target_paths[0].relative_to(self.work).as_posix()])
        self.assertFalse(target_paths[0].exists())
        self.assertEqual((target_paths[0].parent / "execution.json").read_text(),
                         '{"failure":"retained diagnostic"}')
        self.assertEqual((self.source / "crates/ntp-exchange/src/context.rs").read_bytes(),
                         (self.checkout / "crates/ntp-exchange/src/context.rs").read_bytes())


if __name__ == "__main__":
    unittest.main()
