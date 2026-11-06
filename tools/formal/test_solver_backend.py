"""Closed backend/mount/JSON controls; execute no Kani, CBMC or solver."""
from __future__ import annotations

import copy
import importlib.util
import io
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


HERE = Path(__file__).resolve().parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


VERIFY = load("formal_solver_consumer_test", "verify.py")
FIXTURE = load("formal_solver_result_fixture", "test_verify.py")
PROFILE = load("formal_solver_admission_fixture", "test_admission_profile.py")
BASELINE_FLAGS = ["--default-unwind", "1", "--output-format", "regular", "--solver", "cadical",
                  "-Z", "unstable-options", "--run-sanity-checks", "--harness-timeout", "60s"]
Z3_HASH = "fcd0ca765d1c0b4eac776f442ec0f10294b9a6416be91fad4e52fa19012c8d01"


class SolverBackendTests(unittest.TestCase):
    """Tiny mocked admissions establish consumer behavior, never real tool identity."""

    def setUp(self):
        PROFILE.AdmissionProfileTests.setUp(self)
        self.work = self.folder / "z3-work"
        self.work.mkdir(mode=0o700)
        self.source = self.work / "source"
        self.source.mkdir()
        (self.source / "fixture.rs").write_text("// frozen fixture-only source\n")
        self.solver = self.folder / "z3-store"
        (self.solver / "bin").mkdir(parents=True)
        (self.solver / "bin/z3").write_bytes(b"fixture only: never executed")
        (self.solver / "bin/z3").chmod(0o500)
        self.solver_metadata = {
            "name": "z3", "version": "5.3.3", "readonly_root": self.solver,
            "executable": self.solver / "bin/z3", "path_dir": self.solver / "bin",
            "policy_path": self.source / "policy/formal-solvers.json", "policy_sha256": "a" * 64,
            "manifest_sha256": "b" * 64, "executable_sha256": Z3_HASH,
            "inventory": {"bin/z3": {"sha256": Z3_HASH}},
            "closure": {"host_runtime": "existing-public-host-trusted-base-not-rehashed",
                        "per_command_library_hashes_checked": False},
            "store_identity_before": {"": [1, 2, 3], "bin": [4, 5, 6]},
            "store_identity_after": {"": [1, 2, 3], "bin": [4, 5, 6]},
            "ancestor_identity_before": {"private-parent": [7, 8, 9]},
            "ancestor_identity_after": {"private-parent": [7, 8, 9]},
        }

    fixture = PROFILE.AdmissionProfileTests.fixture

    def test_z3_closure_is_added_only_for_explicit_backend(self):
        original = set(VERIFY.input_names("rounding"))
        explicit = set(VERIFY.input_names("rounding", "z3"))
        self.assertEqual(explicit - original, {"tools/formal/solver_admission.py", "policy/formal-solvers.json",
                                              "tools/crypto-aes-siv-witness/fuzz/input_custody.py"})
        self.assertEqual(set(VERIFY.input_names("rounding", "cadical")), original)
        with self.assertRaises(VERIFY.Incomplete):
            VERIFY.input_names("rounding", "arbitrary-solver")

    def test_flags_change_only_solver_value_and_never_modify_baseline(self):
        check = SimpleNamespace(PROFILES={"kani_flags": list(BASELINE_FLAGS)})
        z3 = VERIFY.solver_flags(check, "z3")
        expected = list(BASELINE_FLAGS)
        expected[5] = "z3"
        self.assertEqual(z3, expected)
        self.assertEqual(VERIFY.solver_flags(check), BASELINE_FLAGS)
        self.assertEqual(check.PROFILES["kani_flags"], BASELINE_FLAGS)
        for invalid in (BASELINE_FLAGS + ["--solver", "z3"], BASELINE_FLAGS[:-1],
                        [*BASELINE_FLAGS[:-1], "3600s"], ["--external-sat-solver", "/tmp/unadmitted"],
                        [*BASELINE_FLAGS[:5], "z3", *BASELINE_FLAGS[6:]]):
            check.PROFILES["kani_flags"] = invalid
            with self.subTest(invalid=invalid), self.assertRaises(VERIFY.Incomplete):
                VERIFY.solver_flags(check, "z3")

    def test_unknown_modes_and_cadical_root_fail_before_output_or_admission(self):
        invalid = (("--solver-backend", "unknown"), ("--solver-root", str(self.solver)),
                   ("--solver-backend", "z3", "--solver-root", "relative-root"))
        for options in invalid:
            with self.subTest(options=options), patch.object(VERIFY, "fresh_output") as output, \
                 patch.object(VERIFY, "admission") as admission, patch("sys.stderr", new=io.StringIO()), \
                 self.assertRaises(SystemExit) as result:
                VERIFY.main(["--output-dir", str(self.folder / "must-not-exist"), *options])
            self.assertEqual(result.exception.code, 2)
            output.assert_not_called()
            admission.assert_not_called()

    def test_root_alias_path_separator_and_control_characters_are_rejected(self):
        alias = self.folder / "solver-alias"
        alias.symlink_to(self.solver, target_is_directory=True)
        for invalid in (alias, str(self.solver) + "/../z3-store", str(self.solver) + ":other",
                        str(self.solver) + "\nother", str(self.solver) + "\x7fother", str(self.solver) + "\x85other"):
            with self.subTest(root=repr(invalid)), self.assertRaises(VERIFY.Incomplete):
                VERIFY.solver_selection("z3", invalid)

    def test_z3_admission_loads_frozen_helper_and_adds_exactly_one_root(self):
        check, admitted = self.fixture()
        helper = SimpleNamespace(admit=Mock(return_value=self.solver_metadata))
        with patch.object(VERIFY, "module", return_value=helper) as loader:
            result, receipt = VERIFY.admission(check, self.folder / "z3-admission.json", profile="kani-proof",
                                               solver_backend="z3", solver_root=self.solver,
                                               source=self.source, work=self.work)
        self.assertIs(result, admitted)
        loader.assert_called_once_with("formal_z3_admission", self.source / "tools/formal/solver_admission.py")
        helper.admit.assert_called_once_with(self.source, solver_root=self.solver)
        self.assertEqual(receipt["readonly_roots"], (self.payload, self.compiler, self.solver))
        self.assertEqual(receipt["solver_admission"]["executable_sha256"], Z3_HASH)
        self.assertEqual(receipt["profile"]["kani_flags"][5], "z3")

    def test_consumer_accepts_actual_helper_shaped_record_from_tiny_byte_admission(self):
        helper_tests = load("formal_actual_helper_fixture", "test_solver_admission.py")
        fixture = helper_tests.SolverAdmissionTests()
        fixture.setUp()
        try:
            # Only the fixture payload pin is substituted. The actual helper's
            # complete byte/membership/custody checks and output shape run.
            actual = fixture.admit()
            _, admitted = self.fixture()
            with patch.object(VERIFY, "Z3_EXECUTABLE_SHA256", actual["executable_sha256"]):
                checked = VERIFY.checked_z3_admission(actual, admitted, fixture.checkout,
                                                       solver_root=fixture.folder, work=self.work)
            self.assertIs(checked, actual)
            self.assertEqual(checked["closure"]["host_runtime"],
                             "existing-public-host-trusted-base-not-rehashed")
            self.assertFalse(checked["closure"]["per_command_library_hashes_checked"])
        finally:
            fixture.doCleanups()

    def test_bad_identity_paths_and_manifest_hashes_are_rejected(self):
        check, admitted = self.fixture()
        changes = (("name", "cadical"), ("version", "unreviewed"), ("executable_sha256", "0" * 64),
                   ("executable", self.solver / "bin/other"), ("path_dir", self.solver),
                   ("readonly_root", self.solver.parent), ("policy_path", self.folder / "unfrozen-policy.json"),
                   ("manifest_sha256", "invalid"), ("inventory", {}), ("closure", {}))
        for field, value in changes:
            metadata = copy.deepcopy(self.solver_metadata)
            metadata[field] = value
            with self.subTest(field=field), self.assertRaises(VERIFY.Incomplete):
                VERIFY.checked_z3_admission(metadata, admitted, self.source,
                                            solver_root=self.solver, work=self.work)

    def test_solver_cannot_overlap_source_work_or_other_tool_stores(self):
        _, admitted = self.fixture()
        for forbidden in (self.source, self.work, self.base, self.stable):
            metadata = copy.deepcopy(self.solver_metadata)
            root = forbidden / "solver-candidate"
            (root / "bin").mkdir(parents=True)
            (root / "bin/z3").write_bytes(b"fixture: no execution")
            (root / "bin/z3").chmod(0o500)
            metadata.update(readonly_root=root, executable=root / "bin/z3", path_dir=root / "bin")
            with self.subTest(forbidden=forbidden), self.assertRaises(VERIFY.Incomplete):
                VERIFY.checked_z3_admission(metadata, admitted, self.source, work=self.work)

    def test_execute_z3_uses_three_roots_prepends_one_bin_and_readmits_afterward(self):
        check, admitted = self.fixture()
        PROFILE.custody_runtime(check, self.pointer)
        job = VERIFY.jobs("rounding")[0]
        receipt = {"seconds": 0.0, "solver_admission": self.solver_metadata,
                   "readonly_roots": (self.payload, self.compiler, self.solver),
                   "installation_pointer_sha256": VERIFY.digest(self.pointer)}

        def prepare(source, inputs, case):
            entry = inputs / "proof.rs"
            entry.write_text("// fixture-only input\n")
            return {"entrypoint": entry}

        with patch.object(VERIFY, "require_sources") as sources, \
             patch.object(VERIFY, "prepare_rounding", side_effect=prepare), \
             patch.object(VERIFY, "admission", side_effect=PROFILE.receipt_admission(admitted, receipt)), \
             patch.object(VERIFY, "admit_z3_solver", return_value=self.solver_metadata) as readmission, \
             patch.object(VERIFY, "qualify", return_value={"solver_seconds": None,
                               "decision_procedure_seconds": 0.2}) as qualify:
            result = VERIFY.execute_job(check, self.work, self.source, {}, "rounding", job, 1,
                                        tool_admission_profile="kani-proof", solver_backend="z3", solver_root=self.solver)
        self.assertTrue(result["expected_outcome_qualified"], result)
        check.RUNTIME.run.assert_called_once()
        check.RUNTIME._owned_work.assert_called_once_with(self.work, VERIFY.ROOT)
        options = check.RUNTIME.run.call_args.kwargs
        self.assertIn(self.source, options["sealed"])
        self.assertIn(Path(result["output"]) / "admission.json", options["sealed"])
        self.assertEqual(options["readonly"], (self.payload, self.compiler, self.solver))
        self.assertEqual(options["path_dirs"], [self.solver / "bin", self.payload / "bin", self.compiler / "bin"])
        command = check.RUNTIME.run.call_args.args[2]
        self.assertEqual(command[command.index("--solver") + 1], "z3")
        self.assertEqual(options["timeout"], 180)
        self.assertTrue(all(call.args[-1] == "z3" for call in sources.call_args_list))
        self.assertEqual(qualify.call_args.kwargs, {"expected_solver": "z3"})
        readmission.assert_called_once_with(admitted, self.source, solver_root=self.solver, work=self.work)
        self.assertIsNone(result["execute_other_seconds"])

    def test_extra_runtime_mount_or_postcommand_identity_drift_is_incomplete(self):
        faults = ("extra-mount", "changed-inventory", "changed-directory", "changed-ancestor",
                  "changed-pointer", "changed-source")
        for fault in faults:
            check, admitted = self.fixture()
            PROFILE.custody_runtime(check, self.pointer, after_native=(
                lambda: (self.source / "fixture.rs").write_text("// changed during native command\n")
                ) if fault == "changed-source" else None)
            receipt = {"seconds": 0.0, "solver_admission": self.solver_metadata,
                       "readonly_roots": (self.payload, self.compiler, self.solver),
                       "installation_pointer_sha256": VERIFY.digest(self.pointer)}
            after = copy.deepcopy(self.solver_metadata)
            if fault == "extra-mount":
                receipt["readonly_roots"] += (self.folder,)
            elif fault == "changed-inventory":
                after["inventory"]["bin/z3"]["sha256"] = "0" * 64
            elif fault == "changed-directory":
                after["store_identity_before"][""] = [10, 11, 12]
                after["store_identity_after"][""] = [10, 11, 12]
            elif fault == "changed-ancestor":
                after["ancestor_identity_before"]["private-parent"] = [10, 11, 12]
                after["ancestor_identity_after"]["private-parent"] = [10, 11, 12]
            elif fault == "changed-pointer":
                check.read_once.side_effect = lambda path, **options: (
                    PROFILE.ANALYSIS.read_once(path, **options) if path.name == "kani.json" else b'{"changed":true}')

            def prepare(source, inputs, case):
                entry = inputs / "proof.rs"
                entry.write_text("// fixture-only input\n")
                return {"entrypoint": entry}

            with self.subTest(fault=fault), patch.object(VERIFY, "require_sources"), \
                 patch.object(VERIFY, "prepare_rounding", side_effect=prepare), \
                 patch.object(VERIFY, "admission", side_effect=PROFILE.receipt_admission(admitted, receipt)), \
                 patch.object(VERIFY, "admit_z3_solver", return_value=after) as readmission, \
                 patch.object(VERIFY, "qualify") as qualify:
                result = VERIFY.execute_job(check, self.work, self.source, {}, "rounding",
                                            VERIFY.jobs("rounding")[0], 1 + faults.index(fault),
                                            tool_admission_profile="kani-proof", solver_backend="z3", solver_root=self.solver)
            self.assertFalse(result["expected_outcome_qualified"])
            qualify.assert_not_called()
            if fault == "extra-mount":
                check.RUNTIME.run.assert_not_called()
                readmission.assert_not_called()
                self.assertIn("Z3 runtime mounts differ", result["qualification_error"])
            else:
                check.RUNTIME.run.assert_called_once()
                check.RUNTIME._owned_work.assert_called_once_with(self.work, VERIFY.ROOT)
                if fault == "changed-source":
                    readmission.assert_not_called()
                    self.assertIn("earlier proof input island", result["qualification_error"])
                else:
                    readmission.assert_called_once_with(admitted, self.source,
                                                        solver_root=self.solver, work=self.work)
                    expected = ("Installation pointer changed" if fault == "changed-pointer"
                                else "Z3 solver policy or payload identity changed")
                    self.assertIn(expected, result["qualification_error"])

    def test_main_forwards_backend_and_root_to_commands_and_final_readmission(self):
        arguments = ["--output-dir", str(self.work), "--only-jobs", "1", "--solver-backend", "z3",
                     "--solver-root", str(self.solver), "--tool-admission-profile", "kani-proof"]
        job_result = {"output": str(self.work / "job-1"), "expected_outcome_qualified": True,
                      "native": {"exit_code": 0}, "solver_seconds": None, "decision_procedure_seconds": 0.25,
                      "timing_field": "runtime_decision_procedure_s", "timing_meaning": "decision procedure"}
        # This forwarding control uses main's actual source freeze and closeout
        # audit; only its mathematical command and installed tool identity are mocked.
        check = PROFILE.source_custody_check()
        actual_loader = VERIFY.module

        def local_loader(name, path):
            return check if name == "formal_analysis_admission" else actual_loader(name, path)

        def executed(check, work, source, records, subject, job, index, **options):
            self.assertEqual(set(options), {"tool_admission_profile", "solver_backend", "solver_root", "retained"})
            PROFILE.assert_source_anchor(self, check, work, source, options["retained"])
            return job_result

        # freeze_inputs requires its fresh source directory, as in a real run.
        (self.source / "fixture.rs").unlink()
        self.source.rmdir()
        with patch.object(VERIFY, "fresh_output", return_value=self.work), \
             patch.object(VERIFY, "module", side_effect=local_loader), \
             patch.object(VERIFY, "input_names", return_value=["tools/formal/verify.py"]), \
             patch.object(VERIFY, "require_sources", wraps=VERIFY.require_sources) as sources, \
             patch.object(VERIFY, "execute_job", side_effect=executed) as execute, \
             patch.object(VERIFY, "admission", side_effect=PROFILE.receipt_admission({}, {"seconds": 0.0})) as admission, \
             patch("sys.stdout", new=io.StringIO()):
            self.assertEqual(VERIFY.main(arguments), 0)
        options = dict(execute.call_args.kwargs)
        retained = options.pop("retained")
        self.assertEqual(options, {"tool_admission_profile": "kani-proof", "solver_backend": "z3",
                                   "solver_root": self.solver})
        PROFILE.assert_source_anchor(self, check, self.work, self.source, retained)
        self.assertEqual(admission.call_args.kwargs, {"profile": "kani-proof", "solver_backend": "z3",
            "solver_root": self.solver, "source": self.work / "source", "work": self.work})
        self.assertTrue(all(call.args[-1] == "z3" for call in sources.call_args_list))
        summary = json.loads((self.work / "summary.json").read_text())
        self.assertEqual(summary["solver_backend"], "z3")
        self.assertIsNone(summary["jobs"][0]["solver_seconds"])
        self.assertEqual(summary["jobs"][0]["decision_procedure_seconds"], 0.25)
        self.assertTrue(summary["all_proof_inputs_and_required_receipts_unchanged"])


class SolverResultTests(unittest.TestCase):
    def z3_result(self):
        values = FIXTURE.healthy()
        values[1]["tools"]["solvers"][0]["name"] = "z3"
        values[1]["cbmc"][0]["configuration"]["solver"] = "z3"
        return values

    def qualify(self, values, expected="cadical"):
        job, report, native = values
        return VERIFY.qualify(report, native, job, FIXTURE.ENTRY, expected_solver=expected)

    def test_backend_is_explicit_and_both_json_identities_must_match(self):
        self.qualify(FIXTURE.healthy())
        self.qualify(self.z3_result(), "z3")
        for expected, values in (("cadical", self.z3_result()), ("z3", FIXTURE.healthy())):
            with self.subTest(expected=expected), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values, expected)
        for field in ("tools", "cbmc"):
            values = self.z3_result()
            if field == "tools":
                values[1]["tools"]["solvers"][0]["name"] = "cadical"
            else:
                values[1]["cbmc"][0]["configuration"]["solver"] = "cadical"
            with self.subTest(field=field), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values, "z3")
        with self.assertRaises(VERIFY.Incomplete):
            self.qualify(self.z3_result(), "unknown")

    def test_z3_only_decision_procedure_timing_is_recorded_without_fabricated_solver_time(self):
        values = self.z3_result()
        stats = values[1]["cbmc"][0]["cbmc_stats"]
        del stats["runtime_solver_s"]
        stats["runtime_decision_procedure_s"] = 0.25
        result = self.qualify(values, "z3")
        self.assertIsNone(result["solver_seconds"])
        self.assertEqual(result["decision_procedure_seconds"], 0.25)
        self.assertEqual(result["timing_field"], "runtime_decision_procedure_s")
        self.assertIn("not isolated solver", result["timing_meaning"])
        cadical = FIXTURE.healthy()
        cadical[1]["cbmc"][0]["cbmc_stats"] = dict(stats)
        with self.assertRaises(VERIFY.Incomplete):
            self.qualify(cadical)

    def test_missing_or_malformed_actual_timing_never_becomes_zero_or_a_pass(self):
        for field, value in (("runtime_solver_s", True), ("runtime_solver_s", float("nan")),
                             ("runtime_solver_s", -1), ("runtime_decision_procedure_s", None),
                             ("runtime_decision_procedure_s", False), ("runtime_decision_procedure_s", -1),
                             ("runtime_decision_procedure_s", float("inf"))):
            values = self.z3_result()
            values[1]["cbmc"][0]["cbmc_stats"] = {field: value}
            with self.subTest(field=field, value=value), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values, "z3")


if __name__ == "__main__":
    unittest.main()
