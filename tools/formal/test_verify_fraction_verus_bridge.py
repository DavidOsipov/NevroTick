"""Synthetic bridge-controller controls; never execute admission, Verus or the SDK."""
from __future__ import annotations

from copy import deepcopy
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("fraction_bridge_controller_under_test",
                                            Path(__file__).with_name("verify_fraction_verus_bridge.py"))
BRIDGE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRIDGE)
SOURCE = "/work/inputs/01-healthy/proof.rs"


def report(job):
    """Independent native-protocol fixture for all nine expected named bodies."""
    fault = job["target"] is not None
    return {
        "verification-results": {"encountered-error": fault, "encountered-vir-error": False,
            "success": not fault, "verified": 8 if fault else 9, "errors": 1 if fault else 0,
            "is-verifying-entire-crate": True},
        "verus": deepcopy(BRIDGE.EXPECTED_IDENTITY),
        "times-ms": {"smt": {"smt-run-module-times": [{"function-breakdown": [
            {"function": name, "success": name != job["target"], "mode:": "exec"}
            for name in BRIDGE.FUNCTIONS]}]}},
    }


def diagnostic(job, *, primary_clause=False, exit_line=None):
    """Synthetic Post links in the observed or original diagnostic orientation."""
    spans = job["spans"][job["target"]]
    return {"$message_type": "diagnostic", "level": "error", "message": "postcondition not satisfied",
            "spans": [{"file_name": SOURCE,
                       "line_start": spans["return_start"] if exit_line is None else exit_line,
                       "line_end": spans["return_end"] if exit_line is None else exit_line,
                       "is_primary": not primary_clause,
                       "label": ("at this exit" if exit_line is not None
                                 else "at the end of the function body") if primary_clause else None},
                      {"file_name": SOURCE, "line_start": spans["returns_line"],
                       "line_end": spans["returns_line"], "is_primary": primary_clause,
                       "label": "failed this postcondition"}]}


def native(value, job, diagnostics=None):
    if diagnostics is None:
        diagnostics = [diagnostic(job)] if job["target"] is not None else []
    return {"failure": None, "exit_code": 1 if job["target"] is not None else 0,
            "output": "\n".join(json.dumps(item, indent=2) for item in [*diagnostics, value])}


class BridgeControllerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-bridge-controller-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        # Restore only this test's owned copies so TemporaryDirectory can remove
        # deliberately sealed trees after either a successful or failing control.
        self.addCleanup(self.restore_permissions)
        self.generated = self.root / "baseline"
        self.details = BRIDGE.PROJECTION.prepare(BRIDGE.ROOT, self.generated)
        self.healthy = (self.generated / "bridge.rs").read_bytes()
        self.ghost = (BRIDGE.ROOT / "tools/formal/fraction_verus_bridge_spec.rs").read_bytes()

    def restore_permissions(self):
        for directory, subdirs, files in os.walk(self.root):
            os.chmod(directory, 0o700)
            for name in subdirs:
                if not (Path(directory) / name).is_symlink():
                    os.chmod(Path(directory) / name, 0o700)
            for name in files:
                if not (Path(directory) / name).is_symlink():
                    os.chmod(Path(directory) / name, 0o600)

    def job(self, index=0):
        job = {**BRIDGE.JOBS[index], "index": index + 1}
        data = BRIDGE.control_bytes(self.healthy, self.ghost, job)
        job["spans"] = BRIDGE.function_spans(data)
        return job

    def qualify(self, value, job=None):
        job = self.job() if job is None else job
        return BRIDGE.qualify(value, job, BRIDGE.EXPECTED_IDENTITY, SOURCE)

    def rejected(self, value, job=None):
        with self.assertRaises((RuntimeError, ValueError)):
            self.qualify(value, job)

    def test_healthy_and_restored_require_all_nine_complete_native_bodies(self):
        for index in (0, 4):
            with self.subTest(index=index):
                job = self.job(index)
                value = report(job)
                value["verification-results"]["verified"] = 12
                qualified = self.qualify(native(value, job), job)
                self.assertEqual(len(qualified["functions"]), 9)
                self.assertTrue(all(qualified["functions"].values()))

    def test_each_ghost_control_requires_its_sole_named_semantic_failure(self):
        for index in (1, 2, 3):
            with self.subTest(index=index):
                job = self.job(index)
                qualified = self.qualify(native(report(job), job), job)
                self.assertEqual([name for name, success in qualified["functions"].items() if not success],
                                 [job["target"]])

    def test_typed_complete_whole_crate_query_flags_and_minima(self):
        for field, value in (("verified", True), ("verified", 8), ("errors", False),
                             ("is-verifying-entire-crate", False), ("success", False),
                             ("encountered-vir-error", True), ("encountered-error", 0)):
            with self.subTest(field=field, value=value):
                job = self.job()
                changed = report(job)
                changed["verification-results"][field] = value
                self.rejected(native(changed, job), job)
        changed = report(self.job())
        del changed["verification-results"]["verified"]
        self.rejected(native(changed, self.job()))
        fault = self.job(1)
        changed = report(fault)
        changed["verification-results"]["verified"] = 7
        self.rejected(native(changed, fault), fault)

    def test_missing_duplicate_and_truncated_json_never_become_success(self):
        job = self.job()
        valid = native(report(job), job)
        for output in ("", valid["output"] + "\n" + valid["output"], valid["output"][:-2],
                       valid["output"].replace('"verified": 9', '"verified": 9, "verified": 9'),
                       '{"broken":' + valid["output"], "unparsed warning\n" + valid["output"]):
            with self.subTest(output=output[:40]):
                self.rejected({**valid, "output": output}, job)

    def test_function_inventory_preserves_three_distinct_clone_paths_and_exec_modes(self):
        for mutation in ("missing", "extra", "duplicate", "clone_basename", "bad_mode", "wrong_mode_key", "status_int"):
            with self.subTest(mutation=mutation):
                job = self.job()
                changed = report(job)
                functions = changed["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"]
                if mutation == "missing":
                    functions.pop()
                elif mutation == "extra":
                    functions.append({"function": "proof::unselected", "mode:": "exec", "success": True})
                elif mutation == "duplicate":
                    functions.append(deepcopy(functions[-1]))
                elif mutation == "clone_basename":
                    functions[-1]["function"] = "clone"
                elif mutation == "bad_mode":
                    functions[-1]["mode:"] = "spec"
                elif mutation == "wrong_mode_key":
                    functions[-1]["mode"] = functions[-1].pop("mode:")
                else:
                    functions[-1]["success"] = 1
                self.rejected(native(changed, job), job)

    def test_aggregate_identity_is_exact_and_cannot_be_supplied_by_another_tool(self):
        for field in BRIDGE.EXPECTED_IDENTITY:
            with self.subTest(field=field):
                job = self.job()
                changed = report(job)
                changed["verus"][field] = "other"
                self.rejected(native(changed, job), job)
        job = self.job()
        changed = report(job)
        del changed["verus"]
        self.rejected(native(changed, job), job)

    def test_competing_errors_native_custody_and_other_failed_functions_are_rejected(self):
        job = self.job(1)
        for message in ("assertion failed", "arithmetic overflow", "aborting due to 2 previous errors"):
            other = {"$message_type": "diagnostic", "level": "error", "message": message, "spans": []}
            self.rejected(native(report(job), job, [diagnostic(job), other]), job)
        for mutation in ({"failure": "wall timeout (180s)"}, {"exit_code": 0}, {"exit_code": True}):
            self.rejected({**native(report(job), job), **mutation}, job)
        changed = report(job)
        changed["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"][-1]["success"] = False
        self.rejected(native(changed, job), job)

    def test_postcondition_error_requires_both_target_return_and_returns_label(self):
        job = self.job(2)
        for primary_clause in (False, True):
            for mutation in ("file", "return", "label", "clause", "no_primary", "message",
                             "outside_primary", "no_return", "broad_clause",
                             "malformed_secondary", "foreign_secondary"):
                with self.subTest(primary_clause=primary_clause, mutation=mutation):
                    error = diagnostic(job, primary_clause=primary_clause)
                    if mutation == "file":
                        error["spans"][0]["file_name"] = "/work/unrelated.rs"
                    elif mutation == "return":
                        error["spans"][0]["line_start"] = error["spans"][0]["line_end"] = 1
                    elif mutation == "label":
                        error["spans"][1]["label"] = None
                    elif mutation == "clause":
                        error["spans"][1]["line_start"] = error["spans"][1]["line_end"] = 1
                    elif mutation == "no_primary":
                        for span in error["spans"]:
                            span["is_primary"] = False
                    elif mutation == "message":
                        error["message"] = "assertion failed"
                    elif mutation == "outside_primary":
                        error["spans"].append({"file_name": SOURCE, "line_start": 1,
                                               "line_end": 1, "is_primary": True, "label": None})
                    elif mutation == "no_return":
                        error["spans"].pop(0)
                    elif mutation == "broad_clause":
                        error["spans"][1]["line_start"] -= 1
                    elif mutation == "malformed_secondary":
                        error["spans"].append({"is_primary": False})
                    else:
                        error["spans"].append({"file_name": "/work/unrelated.rs",
                                               "line_start": error["spans"][0]["line_start"],
                                               "line_end": error["spans"][0]["line_end"],
                                               "is_primary": False, "label": None})
                    self.rejected(native(report(job), job, [error]), job)

    def test_postcondition_spans_accept_observed_and_original_orientations(self):
        for index in (1, 2, 3):
            job = self.job(index)
            for primary_clause in (True, False):
                with self.subTest(index=index, primary_clause=primary_clause):
                    errors = [diagnostic(job, primary_clause=primary_clause),
                              {"$message_type": "diagnostic", "level": "error",
                               "message": "aborting due to 1 previous error", "spans": []}]
                    qualified = self.qualify(native(report(job), job, errors), job)
                    self.assertTrue(qualified["qualified"])
                    self.assertEqual([name for name, passed in qualified["functions"].items() if not passed],
                                     [job["target"]])

    def test_multiple_semantic_diagnostics_require_one_owner_and_exact_abort_count(self):
        job = self.job(2)
        whole = diagnostic(job, primary_clause=True)
        exit_error = diagnostic(job, primary_clause=True,
                                exit_line=job["spans"][job["target"]]["return_start"])
        abort = {"$message_type": "diagnostic", "level": "error",
                 "message": "aborting due to 2 previous errors", "spans": []}
        qualified = self.qualify(native(report(job), job, [whole, exit_error, abort]), job)
        self.assertTrue(qualified["qualified"])
        self.assertEqual(qualified["verification_results"]["errors"], 1)
        wrong_abort = {**abort, "message": "aborting due to 1 previous error"}
        other_owner = diagnostic(self.job(1), primary_clause=True)
        for mutation, errors in (
            ("duplicate_semantic", [whole, deepcopy(whole), abort]),
            ("mixed_owner", [whole, other_owner, abort]),
            ("wrong_abort_count", [whole, exit_error, wrong_abort]),
            ("duplicate_abort", [whole, exit_error, abort, deepcopy(abort)]),
        ):
            with self.subTest(mutation=mutation):
                self.rejected(native(report(job), job, errors), job)
        changed = report(job)
        changed["verification-results"]["errors"] = 2
        self.rejected(native(changed, job, [whole, exit_error, abort]), job)

    def test_malformed_timing_and_unknown_native_objects_are_rejected(self):
        job = self.job()
        for times in (None, [], {}, {"smt": []}, {"smt": {"smt-run-module-times": []}},
                      {"smt": {"smt-run-module-times": [{"function-breakdown": None}]}}):
            changed = report(job)
            changed["times-ms"] = times
            self.rejected(native(changed, job), job)
        value = native(report(job), job)
        self.rejected({**value, "output": value["output"] + '\n{"other": true}'}, job)

    def test_ghost_leaf_controls_preserve_executable_duplicates_and_restore_all_bytes(self):
        executable = self.healthy.split(self.ghost, 1)[1]
        for index in (1, 2, 3):
            job = self.job(index)
            changed = BRIDGE.control_bytes(self.healthy, self.ghost, job)
            altered_ghost = self.ghost.replace(job["old"], job["new"], 1)
            self.assertEqual(changed.split(altered_ghost, 1)[1], executable)
            self.assertEqual(changed.replace(altered_ghost, self.ghost, 1), self.healthy)
            self.assertNotEqual(changed, self.healthy)
        self.assertEqual(BRIDGE.control_bytes(self.healthy, self.ghost, self.job(4)), self.healthy)
        with self.assertRaisesRegex(RuntimeError, "Ghost segment"):
            BRIDGE.control_bytes(self.healthy, self.ghost + b"\n", self.job(1))

    def test_projection_metadata_binds_all_six_relations_and_nine_source_spans(self):
        BRIDGE.validate_metadata(self.details, BRIDGE.ROOT, self.generated)
        for field in ("predicate_relations", "source_files_sha256", "correspondence", "original_sources_sha256"):
            changed = deepcopy(self.details)
            changed[field] = []
            with self.subTest(field=field), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated)

    def test_frozen_permission_cache_and_content_drift_are_observed_without_imports(self):
        island = self.root / "island"
        island.mkdir()
        value = island / "source.py"
        value.write_bytes(b"raise RuntimeError('must never import')\n")
        with patch.object(BRIDGE, "WORK", self.root):
            BRIDGE.seal(island)
        before = BRIDGE.tree(island)
        self.assertEqual(before["source.py"]["mode"], 0o400)
        os.chmod(value, 0o600)
        with self.assertRaisesRegex(RuntimeError, "permission"):
            BRIDGE.tree(island)
        value.write_bytes(b"changed source bytes\n")
        os.chmod(value, 0o400)
        self.assertNotEqual(BRIDGE.tree(island), before)
        os.chmod(island, 0o700)
        (island / "__pycache__").mkdir(mode=0o500)
        os.chmod(island, 0o500)
        with self.assertRaisesRegex(RuntimeError, "cache"):
            BRIDGE.tree(island)

    def test_source_redirects_hardlinks_and_seal_redirect_preserve_external_modes(self):
        original = self.root / "original"
        original.write_bytes(b"source")
        alias = self.root / "alias"
        alias.symlink_to(original)
        with self.assertRaisesRegex(RuntimeError, "redirect"):
            BRIDGE.plain(alias)
        alias.unlink()
        os.link(original, alias)
        with self.assertRaisesRegex(RuntimeError, "hardlink"):
            BRIDGE.identity(original)
        alias.unlink()
        external = self.root / "external"
        external.mkdir(mode=0o700)
        value = external / "must-remain-writable"
        value.write_bytes(b"outside owned work")
        work = self.root / "work"
        work.mkdir(mode=0o700)
        redirect = work / "redirect"
        redirect.symlink_to(external, target_is_directory=True)
        before = (external.stat().st_mode, value.stat().st_mode)
        with patch.object(BRIDGE, "WORK", work), self.assertRaisesRegex(RuntimeError, "redirect"):
            BRIDGE.seal(redirect)
        with patch.object(BRIDGE, "WORK", work), self.assertRaisesRegex(RuntimeError, "redirect"):
            BRIDGE.write(redirect / "execution.json", {"must_not_escape": True})
        self.assertEqual((external.stat().st_mode, value.stat().st_mode), before)
        self.assertFalse((external / "execution.json").exists())

    def test_one_preparation_clock_rejects_expiry_rollback_and_changed_boot(self):
        with patch.object(Path, "read_text", return_value=BRIDGE.BOOT_ID), \
             patch.object(BRIDGE.time, "monotonic_ns", return_value=BRIDGE.STARTED_MONOTONIC_NS + 10**9), \
             patch.object(BRIDGE.time, "time_ns", return_value=BRIDGE.STARTED_UNIX_NS + 10**9):
            self.assertEqual(BRIDGE.remaining(), 3599)
            with patch.object(BRIDGE.time, "monotonic_ns", return_value=BRIDGE.DEADLINE_MONOTONIC_NS):
                with self.assertRaisesRegex(RuntimeError, "clock"):
                    BRIDGE.remaining()
            with patch.object(BRIDGE.time, "time_ns", return_value=BRIDGE.STARTED_UNIX_NS - 1):
                with self.assertRaisesRegex(RuntimeError, "clock"):
                    BRIDGE.remaining()
            with patch.object(Path, "read_text", return_value="other boot"):
                with self.assertRaisesRegex(RuntimeError, "boot"):
                    BRIDGE.remaining()

    def test_retained_space_and_free_reserve_limits_are_independent(self):
        with patch.object(BRIDGE, "remaining", return_value=100), \
             patch.object(BRIDGE.shutil, "disk_usage") as usage:
            usage.return_value.free = BRIDGE.RESERVE_BYTES
            self.assertGreaterEqual(BRIDGE.capacity(self.root), 0)
            usage.return_value.free -= 1
            with self.assertRaisesRegex(RuntimeError, "reserve"):
                BRIDGE.capacity(self.root)
            usage.return_value.free = BRIDGE.RESERVE_BYTES
            with patch.object(BRIDGE, "MAXIMUM_BYTES", 0):
                with self.assertRaisesRegex(RuntimeError, "retained"):
                    BRIDGE.capacity(self.root)

    def test_failed_first_native_proof_retains_receipt_and_never_launches_later_jobs(self):
        work = self.root / "attempt"
        payload = self.root / "payload"
        payload.mkdir()
        (payload / "version.json").write_text(json.dumps({"verus": BRIDGE.EXPECTED_IDENTITY}))
        stable = {"readonly_roots": [self.root / "sdk"], "admission_profile": "stable-core"}
        observed = ["/usr/bin/env", *(str(payload / name) for name in ("verus", "rust_verify", "z3"))]
        failure = {"failure": None, "exit_code": 1, "output": '{"$message_type":"diagnostic","level":"error",'
                   '"message":"unrelated compiler error","spans":[]}'}
        with patch.object(BRIDGE, "WORK", work), patch.object(BRIDGE, "remaining", return_value=100), \
             patch.object(BRIDGE, "capacity", return_value=4096), \
             patch.object(BRIDGE.PILOT, "admit", return_value=(stable, payload)) as admit, \
             patch.object(BRIDGE.PILOT, "check_trace", return_value=observed), \
             patch.object(BRIDGE.CHECK.RUNTIME, "run", return_value=failure) as run, \
             patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(BRIDGE.run_campaign(), 1)
        self.assertEqual(run.call_count, 1)
        self.assertGreaterEqual(admit.call_count, 2)
        summary = json.loads((work / "summary.json").read_text())
        self.assertEqual(summary["status"], "DEFER")
        self.assertEqual(len(summary["jobs"]), 1)
        self.assertTrue(summary["jobs"][0]["command_attempted"])
        self.assertEqual(summary["jobs"][0]["native"], failure)
        self.assertTrue((work / "jobs/01-healthy/execution.json").is_file())
        self.assertEqual((work / "jobs/01-healthy").stat().st_mode & 0o777, 0o500)
        self.assertEqual((work / "jobs/01-healthy/execution.json").stat().st_mode & 0o777, 0o400)
        self.assertTrue(summary["source_input_receipt_bookends_unchanged"])
        self.assertFalse((work / "jobs/02-constructor_wrong_zero").exists())
        options = run.call_args.kwargs
        self.assertEqual((options["cpu_seconds"], options["address_space_bytes"], options["build_jobs"],
                          options["process_limit"]), (180, 8 * 1024**3, 1, 128))


if __name__ == "__main__":
    unittest.main()
