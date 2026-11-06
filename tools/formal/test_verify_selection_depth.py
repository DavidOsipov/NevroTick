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
from contextlib import redirect_stderr, redirect_stdout


ROOT = Path(__file__).resolve().parents[2]


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


BASE = load(ROOT / "tools/formal/verify.py", "selection_depth_unbound_test_runner")


class SelectionDepthConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-selection-depth-consumer-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.checkout = self.root / "checkout"
        for name in BASE.input_names("selection-depth"):
            target = self.checkout / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            shutil.copy2(ROOT / name, target)
        self.entry = load(self.checkout / "tools/formal/verify_selection_depth.py", "selection_depth_test_entry")
        self.runner = self.entry._runner
        self.check = self.runner.module("selection_depth_test_admission", self.checkout / "scripts/check-analysis.py")
        self.names = self.runner.input_names("selection-depth")
        self.projection = self.runner.module("selection_depth_test_projection", self.checkout / "tools/formal/selection_depth_projection.py")
        self.work = self.root / "work"
        self.work.mkdir(mode=0o700)
        self.source = self.work / "source"

    def freeze(self):
        with patch.object(self.runner, "cohort_output", return_value=self.work):
            return self.runner.freeze_inputs(self.check, self.source, self.names)

    def test_three_immutable_recipes_preserve_fixed_timing_and_selection_metadata(self):
        recipes = self.runner._CLOSED_COHORTS
        self.assertIs(type(recipes), tuple)
        self.assertEqual(recipes, (
            ("request-timing", "tools/formal/verify_request_timing.py",
             "formal-request-timing-20261106-v1", "tools/formal/request_timing_projection.py",
             frozenset({
                 "tools/formal/verify_request_timing.py", "tools/formal/python_source_binding.py",
                 "tools/formal/verify.py", "tools/formal/additional_suites.py",
                 "tools/formal/request_timing_projection.py", "scripts/check-analysis.py",
                 "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
             }), "tools/formal/test_verify_request_timing.py", 5, 1020),
            ("selection-depth", "tools/formal/verify_selection_depth.py",
             "formal-selection-depth-20261106-v1", "tools/formal/selection_depth_projection.py",
             frozenset({
                 "tools/formal/verify_selection_depth.py", "tools/formal/python_source_binding.py",
                 "tools/formal/verify.py", "tools/formal/additional_suites.py",
                 "tools/formal/selection_depth_projection.py", "scripts/check-analysis.py",
                 "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
             }), "tools/formal/test_verify_selection_depth.py", 6, 1200),
            ("lifecycle-ordinal", "tools/formal/verify_lifecycle_ordinal.py",
             "formal-lifecycle-ordinal-20261106-v1", "tools/formal/lifecycle_ordinal_projection.py",
             frozenset({
                 "tools/formal/verify_lifecycle_ordinal.py", "tools/formal/python_source_binding.py",
                 "tools/formal/verify.py", "tools/formal/additional_suites.py",
                 "tools/formal/lifecycle_ordinal_projection.py", "scripts/check-analysis.py",
                 "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
             }), "tools/formal/test_verify_lifecycle_ordinal.py", 5, 1020),
            ("lifecycle-source-fields", "tools/formal/verify_lifecycle_source_fields.py",
             "formal-lifecycle-source-fields-20261106-v1", "tools/formal/lifecycle_source_fields_projection.py",
             frozenset({
                 "tools/formal/verify_lifecycle_source_fields.py", "tools/formal/python_source_binding.py",
                 "tools/formal/verify.py", "tools/formal/additional_suites.py",
                 "tools/formal/lifecycle_source_fields_projection.py", "scripts/check-analysis.py",
                 "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
             }), "tools/formal/test_verify_lifecycle_source_fields.py", 5, 1020),
            ("supporter-labels", "tools/formal/verify_supporter_labels.py",
             "formal-supporter-labels-20261106-v1", "tools/formal/supporter_labels_projection.py",
             frozenset({
                 "tools/formal/verify_supporter_labels.py", "tools/formal/python_source_binding.py",
                 "tools/formal/verify.py", "tools/formal/additional_suites.py",
                 "tools/formal/supporter_labels_projection.py", "scripts/check-analysis.py",
                 "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
             }), "tools/formal/test_verify_supporter_labels.py", 5, 1020),
        ))
        self.assertEqual(self.runner.captured_cohort().selector, "selection-depth")
        self.assertEqual(self.runner.cohort_output(), self.runner.selection_depth_output())
        self.assertEqual(self.runner.request_timing_output(), Path.home() / ".local/share/nevrotick"
                         / "formal-request-timing-20261106-v1")
        with self.assertRaises(AttributeError):
            recipes[1].job_count = 5
        self.assertIs(type(recipes[1].python_names), frozenset)

    def test_captured_wrapper_rejects_every_other_subject_before_effects(self):
        for subject in ("request-timing", "all", "fraction-addition-partitions"):
            with self.subTest(subject=subject):
                with self.assertRaisesRegex(self.runner.Incomplete, "another cohort subject"):
                    self.runner.input_names(subject)
                with self.assertRaisesRegex(self.runner.Incomplete, "another cohort subject"):
                    self.runner.jobs(subject)
                with patch.object(self.runner, "fresh_output") as output, \
                     patch.object(self.runner, "admission") as admission, redirect_stdout(io.StringIO()), \
                     redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                    self.runner.main(["--output-dir", str(self.runner.selection_depth_output()),
                                      "--subject", subject])
                output.assert_not_called()
                admission.assert_not_called()
                with self.assertRaisesRegex(self.runner.Incomplete, "another cohort subject"):
                    self.runner.execute_job(self.check, self.work, self.source, {}, subject,
                                            {}, 1, tool_admission_profile="kani-proof")
        self.assertFalse((self.work / "jobs").exists())

    def test_actual_foreign_wrapper_and_both_captured_wrappers_fail_closed(self):
        foreign = self.checkout / "tools/formal/verify_foreign_cohort.py"
        foreign.write_bytes((self.checkout / "tools/formal/verify_selection_depth.py").read_bytes())
        entry = load(foreign, "selection_depth_foreign_entry")
        with self.assertRaisesRegex(entry._runner.Incomplete, "exactly one captured fixed wrapper"):
            entry._runner.require_cohort_binding()
        other = self.checkout / "tools/formal/verify_request_timing.py"
        shutil.copy2(ROOT / "tools/formal/verify_request_timing.py", other)
        self.entry._binding.load_module("selection_depth_other_wrapper", other)
        with self.assertRaisesRegex(self.runner.Incomplete, "exactly one captured fixed wrapper"):
            self.runner.require_cohort_binding()

    def test_cross_role_module_and_source_inventory_are_rejected(self):
        other = self.checkout / "tools/formal/request_timing_projection.py"
        other.write_text("raise RuntimeError('foreign role must not execute')\n")
        with self.assertRaisesRegex(self.runner.Incomplete, "Undeclared captured Python input"):
            self.runner.module("selection_depth_cross_role", other)
        with patch.object(self.runner, "cohort_output", return_value=self.work):
            with self.assertRaisesRegex(self.runner.Incomplete, "complete fixed source inventory"):
                self.runner.freeze_inputs(self.check, self.source, self.names +
                                          ["tools/formal/request_timing_projection.py"])
        self.assertFalse(self.source.exists())
        with self.assertRaisesRegex(self.runner.Incomplete, "fixed fresh source root"):
            self.runner.freeze_inputs(self.check, self.source, self.names)
        self.assertFalse(self.source.exists())

    def test_clock_delegation_uses_only_the_captured_selection_projection(self):
        projection = SimpleNamespace(remaining_budget=lambda: 2000)
        with patch.object(self.runner, "module", return_value=projection) as loader:
            self.assertEqual(self.runner.partition_remaining("selection-depth"), 2000)
        self.assertEqual(loader.call_args.args[1], self.checkout / "tools/formal/selection_depth_projection.py")
        for value in (True, float("nan"), float("inf"), 0, -1, 3601):
            with self.subTest(value=value), \
                 patch.object(self.runner, "module", return_value=SimpleNamespace(remaining_budget=lambda: value)), \
                 self.assertRaisesRegex(self.runner.Incomplete, "budget expired or malformed"):
                self.runner.partition_remaining("selection-depth")

    def test_campaign_guard_rejects_reordered_extended_changed_and_retried_jobs(self):
        records = self.freeze()
        retained = []
        cohort = self.runner.captured_cohort()
        campaign = {"cohort": cohort, "work": self.work, "source": self.source,
                    "retained": retained, "next_index": 1}
        self.runner.COHORT_CAMPAIGN = campaign
        schedule = self.runner.jobs("selection-depth")
        invalid = [(2, schedule[1]), (7, schedule[-1]), (1, schedule[1]),
                   (1, dict(schedule[0], case="unreviewed"))]
        with patch.object(self.runner, "cohort_output", return_value=self.work):
            for index, job in invalid:
                with self.subTest(index=index, job=job), \
                     self.assertRaisesRegex(self.runner.Incomplete, "complete fixed campaign"):
                    self.runner.execute_job(self.check, self.work, self.source, records,
                                            "selection-depth", job, index,
                                            tool_admission_profile="kani-proof", retained=retained)
            self.assertEqual(campaign["next_index"], 1)
            with patch.object(self.runner, "audit_retained", side_effect=RuntimeError("before native")), \
                 patch.object(self.runner, "admission") as admission:
                result = self.runner.execute_job(self.check, self.work, self.source, records,
                                                 "selection-depth", schedule[0], 1,
                                                 tool_admission_profile="kani-proof", retained=retained)
            admission.assert_not_called()
            self.assertFalse(result["expected_outcome_qualified"])
            self.assertEqual(campaign["next_index"], 2)
            with self.assertRaisesRegex(self.runner.Incomplete, "complete fixed campaign"):
                self.runner.execute_job(self.check, self.work, self.source, records,
                                        "selection-depth", schedule[0], 1,
                                        tool_admission_profile="kani-proof", retained=retained)

    def test_selection_resource_exclusion_and_cleanup_preserve_timing_named_targets(self):
        old = BASE.jobs("request-timing")[0]
        timing = self.work / "jobs" / f"01-{old['subject']}-{old['case']}-{old['harness']}" / "target"
        timing.mkdir(mode=0o700, parents=True)
        retained = timing / "must-remain"
        retained.write_bytes(b"timing data is retained selection evidence")
        selected = self.runner.jobs("selection-depth")[0]
        target = self.work / "jobs" / f"01-{selected['subject']}-{selected['case']}-{selected['harness']}" / "target"
        target.mkdir(mode=0o700, parents=True)
        (target / "disposable").write_bytes(b"selected scratch")
        custody = SimpleNamespace(RUNTIME=SimpleNamespace(_owned_work=lambda work, root: None))
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=SimpleNamespace(free=4 * 1024**3)):
            resources = self.runner.cohort_resources(self.work)
            self.assertEqual(resources["retained_bytes"], retained.stat().st_size)
            self.assertEqual(self.runner.cleanup_cohort_targets(custody, self.work),
                             [target.relative_to(self.work).as_posix()])
            self.assertEqual(retained.read_bytes(), b"timing data is retained selection evidence")
            with retained.open("wb") as stream:
                stream.truncate(128 * 1024**2 + 1)
            with self.assertRaisesRegex(self.runner.Incomplete, "128 MiB"):
                self.runner.cohort_resources(self.work)

    def test_healthy_captured_closure_matches_frozen_manifest(self):
        records = self.freeze()
        self.runner.require_sources(self.check, self.source, records, "selection-depth")
        snapshot = self.entry._binding.snapshot()
        self.assertEqual(set(snapshot), self.runner.SELECTION_DEPTH_PYTHON_NAMES)
        self.assertLess(set(snapshot), set(records))
        for name, identity in snapshot.items():
            self.assertEqual(identity, records[name])

    def test_each_loaded_python_edit_fails_before_source_freezing(self):
        for name in sorted(self.runner.SELECTION_DEPTH_PYTHON_NAMES):
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
        for name in ("crates/time-core/src/selection.rs", "tools/formal/selection_depth_harness.rs",
                     "tools/formal/selection_depth_projection.py"):
            path = self.source / name
            original = path.read_bytes()
            with self.subTest(name=name):
                try:
                    path.write_bytes(original + b"\n")
                    with self.assertRaisesRegex(self.runner.Incomplete, "drift"):
                        self.runner.require_sources(self.check, self.source, records, "selection-depth")
                finally:
                    path.write_bytes(original)
        self.runner.require_sources(self.check, self.source, records, "selection-depth")

    def test_mapped_loader_executes_current_captured_code_and_keeps_frozen_data_path(self):
        self.freeze()
        current = self.checkout / "tools/formal/selection_depth_projection.py"
        frozen = self.source / "tools/formal/selection_depth_projection.py"
        cached = object()
        with patch.dict(sys.modules, {"selection_depth_mapped_projection": cached}):
            result = self.runner.module("selection_depth_mapped_projection", frozen)
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
        redirected.symlink_to(self.source / "tools/formal/selection_depth_projection.py")
        traversal = self.source / "tools/formal/../formal/selection_depth_projection.py"
        for path in (foreign, undeclared, redirected, traversal):
            with self.subTest(path=path), self.assertRaises(self.runner.Incomplete):
                self.runner.module("selection_depth_bad_mapping", path)

    def test_bound_loader_bypasses_timestamp_valid_stale_bytecode(self):
        path = self.checkout / "tools/formal/selection_depth_projection.py"
        original = path.read_bytes()
        old = original + b"\nSTALE_CONTROL = 1\n"
        current = original + b"\nSTALE_CONTROL = 2\n"
        self.assertEqual(len(old), len(current))
        path.write_bytes(old)
        before = path.stat()
        py_compile.compile(str(path), doraise=True, invalidation_mode=py_compile.PycInvalidationMode.TIMESTAMP)
        path.write_bytes(current)
        os.utime(path, ns=(before.st_atime_ns, before.st_mtime_ns))
        self.assertEqual(load(path, "selection_depth_stale_control").STALE_CONTROL, 1)
        # A fresh captured wrapper has not previously admitted either projection.
        entry = load(self.checkout / "tools/formal/verify_selection_depth.py", "selection_depth_stale_entry")
        self.assertEqual(entry._runner.module("selection_depth_fresh_projection", path).STALE_CONTROL, 2)

    def test_fixed_entrypoint_rejects_cli_and_supplies_only_fixed_arguments(self):
        with patch.object(self.runner, "main", return_value=23) as dispatch:
            with patch.object(sys, "argv", ["verify_selection_depth.py"]):
                self.assertEqual(self.entry.main(), 23)
            self.assertEqual(dispatch.call_args.args, ([
                "--output-dir", str(self.runner.selection_depth_output()), "--subject", "selection-depth",
                "--tool-admission-profile", "kani-proof", "--solver-backend", "cadical"],))
            dispatch.reset_mock()
            with patch.object(sys, "argv", ["verify_selection_depth.py", "--help"]), \
                 self.assertRaisesRegex(self.runner.Incomplete, "no CLI"):
                self.entry.main()
            dispatch.assert_not_called()
        with self.assertRaises(TypeError):
            self.entry.run_selection_depth_campaign(output="elsewhere")

    def test_full_only_new_selection_and_old_limits_remain(self):
        self.assertEqual(self.runner.job_limit("selection-depth"), 6)
        self.assertEqual(len(self.runner.selected_jobs("selection-depth", 1)), 6)
        for start, only in ((2, None), (1, [1, 2, 3, 4, 5, 6]), (1, [5]), (1, [2, 1])):
            with self.subTest(start=start, only=only), self.assertRaises(self.runner.Incomplete):
                self.runner.selected_jobs("selection-depth", start, only)
        self.assertEqual(BASE.job_limit("all"), 12)
        self.assertEqual(BASE.job_limit("fraction-addition-partitions"), 16)
        self.assertEqual(len(BASE.jobs("all")), 12)
        self.assertEqual(BASE.TOOL_ADMISSION_PROFILES, ("full", "kani-proof"))
        self.assertEqual(BASE.SOLVER_BACKENDS, ("cadical", "z3"))
        self.assertFalse(set(BASE.input_names("all")) & self.runner.SELECTION_DEPTH_PYTHON_NAMES
                         - {"tools/formal/verify.py", "scripts/check-analysis.py", "scripts/analysis-runtime.py",
                            "scripts/analysis-controls.py", "scripts/check-environment.py"})

    def test_unbound_or_overridden_programmatic_entry_fails_before_effects(self):
        with self.assertRaisesRegex(BASE.Incomplete, "captured fixed wrapper"):
            BASE.partition_preflight("selection-depth", "kani-proof", "cadical", None, str(BASE.selection_depth_output()))
        for profile, solver, root, output in (("full", "cadical", None, str(self.runner.selection_depth_output())),
                                              ("kani-proof", "z3", None, str(self.runner.selection_depth_output())),
                                              ("kani-proof", "cadical", "/tmp/solver", str(self.runner.selection_depth_output())),
                                              ("kani-proof", "cadical", None, "/tmp/other")):
            with self.subTest(profile=profile, solver=solver, root=root, output=output), \
                 self.assertRaisesRegex(self.runner.Incomplete, "fixed output"):
                self.runner.partition_preflight("selection-depth", profile, solver, root, output)
        with self.assertRaisesRegex(self.runner.Incomplete, "complete fixed campaign"):
            self.runner.execute_job(self.check, self.work, self.source, {}, "selection-depth",
                                    self.runner.jobs("selection-depth")[0], 1, tool_admission_profile="kani-proof")
        self.assertFalse((self.work / "jobs").exists())

    def test_first_admission_reserve_is_required_without_creating_output(self):
        output = str(self.runner.selection_depth_output())
        with patch.object(self.runner, "partition_remaining", return_value=1199), \
             patch.object(self.runner, "cohort_resources") as resources, \
             self.assertRaisesRegex(self.runner.Incomplete, "1200 seconds"):
            self.runner.partition_preflight("selection-depth", "kani-proof", "cadical", None, output)
        resources.assert_not_called()
        with patch.object(self.runner, "partition_remaining", return_value=1200), \
             patch.object(self.runner, "cohort_resources") as resources:
            self.runner.partition_preflight("selection-depth", "kani-proof", "cadical", None, output)
        resources.assert_called_once_with()

    def test_new_healthy_contract_requires_every_declared_assertion_and_cover(self):
        fixture = load(ROOT / "tools/formal/test_verify.py", "selection_depth_json_fixture")
        job = self.runner.jobs("selection-depth")[0]
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
        job = self.runner.jobs("selection-depth")[0]
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
        job = self.runner.jobs("selection-depth")[0]
        folder = self.work / "jobs" / f"01-{job['subject']}-{job['case']}-{job['harness']}"
        folder.mkdir(mode=0o700, parents=True)
        (folder / "target").symlink_to(self.checkout, target_is_directory=True)
        with patch.object(self.runner, "cohort_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=SimpleNamespace(free=4 * 1024**3)), \
             self.assertRaisesRegex(self.runner.Incomplete, "aliases"):
            self.runner.cohort_resources(self.work)

    def test_low_reserve_does_not_block_safe_target_recovery_or_erase_evidence(self):
        job = self.runner.jobs("selection-depth")[0]
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
        for index, job in enumerate(self.runner.jobs("selection-depth")[:2], 1):
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
             patch.object(self.runner, "selection_depth_output", return_value=self.work), \
             patch.object(self.runner.shutil, "disk_usage", return_value=disk), \
             patch.object(self.runner, "partition_remaining", return_value=2000), \
             patch.object(self.runner, "execute_job", side_effect=failed_job), \
             patch.object(self.runner, "admission") as admission, redirect_stdout(io.StringIO()):
            result = self.entry.run_selection_depth_campaign()
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
        self.assertEqual((self.source / "crates/time-core/src/selection.rs").read_bytes(),
                         (self.checkout / "crates/time-core/src/selection.rs").read_bytes())


if __name__ == "__main__":
    unittest.main()
