"""Formal consumer admission controls; no installed payload or solver executes."""
from __future__ import annotations

import importlib.util
import io
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


SPEC = importlib.util.spec_from_file_location("formal_profile_test", Path(__file__).with_name("verify.py"))
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)
ANALYSIS_SPEC = importlib.util.spec_from_file_location(
    "formal_profile_custody_primitives", VERIFY.ROOT / "scripts/check-analysis.py")
ANALYSIS = importlib.util.module_from_spec(ANALYSIS_SPEC)
ANALYSIS_SPEC.loader.exec_module(ANALYSIS)
KANI_ENTRY = "kani/payload/kani-0.68.0/bin/kani-driver"
COMPILER = "kani/rustup/toolchains/nightly-2026-08-21-x86_64-unknown-linux-gnu"


def custody_runtime(check, pointer: bytes, *, after_native=None):
    """Keep real file/snapshot guards around the deliberately mocked native seam."""
    check.input_snapshot = Mock(wraps=ANALYSIS.input_snapshot)
    check.read_once.side_effect = lambda path, **options: (
        ANALYSIS.read_once(path, **options) if path.name == "kani.json" else pointer)

    def native(root, work, command, **options):
        ANALYSIS.RUNTIME._sealed_inputs(work, options["sealed"])
        VERIFY.write_json(options["cwd"].parent / "kani.json", {"fixture": "mock native result"})
        if after_native is not None:
            after_native()
        return {"exit_code": 0, "failure": None}

    check.RUNTIME = SimpleNamespace(run=Mock(side_effect=native),
        _owned_work=Mock(wraps=ANALYSIS.RUNTIME._owned_work))


def receipt_admission(admitted, record):
    """Mock tool identity while retaining its actual required JSON artifact."""
    def admit(check, output, **options):
        VERIFY.write_json(output, record)
        return admitted, record
    return admit


def source_custody_check():
    """Use real source reads and snapshots without invoking tool admission."""
    return SimpleNamespace(read_once=ANALYSIS.read_once,
                           input_snapshot=Mock(wraps=ANALYSIS.input_snapshot))


def assert_source_anchor(test, check, work, source, retained):
    """Challenge a meaningful frozen source/manifest anchor forwarded by main."""
    test.assertIsInstance(retained, list)
    test.assertGreaterEqual(len(retained), 1)
    anchor = retained[0]
    test.assertEqual(anchor["roots"], [source, work / "source-manifest.json"])
    test.assertEqual(anchor["snapshot"], ANALYSIS.input_snapshot(work, anchor["roots"]))
    key = (source / "tools/formal/verify.py").relative_to(work).as_posix()
    test.assertEqual(anchor["snapshot"][key]["sha256"], VERIFY.digest(
        (VERIFY.ROOT / "tools/formal/verify.py").read_bytes()))
    test.assertIn("source-manifest.json", anchor["snapshot"])


class AdmissionProfileTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-formal-profile-test-")
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.base = self.folder / "installation"
        self.stable = self.folder / "stable"
        self.payload = self.base / "kani/payload/kani-0.68.0"
        self.compiler = self.base / COMPILER
        for path in (self.stable, self.payload, self.compiler):
            path.mkdir(parents=True)
        self.pointer = b'{"fixture": "private installation pointer"}'

    def fixture(self, profile="kani-proof"):
        admitted = {"admission_profile": profile, "base": self.base, "stable": self.stable,
                    "policy": {"tools": {"kani": {"entrypoint": KANI_ENTRY, "toolchain": COMPILER}}},
                    "readonly_roots": (self.payload, self.compiler) if profile == "kani-proof"
                    else (self.base, self.stable)}
        check = SimpleNamespace(
            admit=Mock(return_value=admitted), admit_kani_proof=Mock(return_value=admitted),
            read_once=Mock(return_value=self.pointer), decode=json.loads,
            digest=Mock(return_value="a" * 64), PROFILES={"kani_flags": [
                "--default-unwind", "1", "--output-format", "regular", "--solver", "cadical",
                "-Z", "unstable-options", "--run-sanity-checks", "--harness-timeout", "60s"]})
        return check, admitted

    def test_explicit_kani_profile_uses_only_its_dedicated_api_and_records_roots(self):
        check, admitted = self.fixture()
        result, receipt = VERIFY.admission(check, self.folder / "admission.json", profile="kani-proof")
        self.assertIs(result, admitted)
        check.admit_kani_proof.assert_called_once_with(VERIFY.ROOT)
        check.admit.assert_not_called()
        self.assertEqual(receipt["expected_admission_profile"], "kani-proof")
        self.assertEqual(receipt["readonly_roots"], (self.payload, self.compiler))
        saved = json.loads((self.folder / "admission.json").read_text())
        self.assertEqual(saved["readonly_roots"], [str(self.payload), str(self.compiler)])

    def test_default_keeps_full_legacy_api_and_both_full_roots(self):
        check, _ = self.fixture("full")
        _, receipt = VERIFY.admission(check, self.folder / "admission.json")
        check.admit.assert_called_once_with(VERIFY.ROOT)
        check.admit_kani_proof.assert_not_called()
        self.assertEqual(receipt["expected_admission_profile"], "full")
        self.assertEqual(receipt["readonly_roots"], (self.base, self.stable))

    def test_unknown_modes_are_rejected_before_reading_pointer_or_calling_apis(self):
        for mode in ("stable-core", "", None, True, []):
            check, _ = self.fixture()
            with self.subTest(mode=mode), self.assertRaises(VERIFY.Incomplete):
                VERIFY.admission(check, self.folder / "admission.json", profile=mode)
            check.read_once.assert_not_called()
            check.admit.assert_not_called()
            check.admit_kani_proof.assert_not_called()

    def test_wrong_returned_mode_never_falls_back(self):
        for expected, observed in (("kani-proof", "full"), ("full", "kani-proof"), ("kani-proof", "stable-core")):
            check, admitted = self.fixture(expected)
            admitted["admission_profile"] = observed
            with self.subTest(expected=expected, observed=observed), self.assertRaises(VERIFY.Incomplete):
                VERIFY.admission(check, self.folder / "admission.json", profile=expected)
            if expected == "kani-proof":
                check.admit.assert_not_called()
            else:
                check.admit_kani_proof.assert_not_called()

    def test_kani_roots_reject_broad_extra_missing_reordered_and_string_paths(self):
        invalid = [(self.base, self.stable), (self.base, self.compiler), (self.payload, self.stable),
                   (self.payload, self.compiler, self.stable), (self.payload,), (),
                   (self.compiler, self.payload), (self.payload, self.payload),
                   (str(self.payload), str(self.compiler))]
        for roots in invalid:
            check, admitted = self.fixture()
            admitted["readonly_roots"] = roots
            with self.subTest(roots=roots), self.assertRaises(VERIFY.Incomplete):
                VERIFY.admission(check, self.folder / "admission.json", profile="kani-proof")
            check.digest.assert_not_called()

    def test_kani_root_symlink_alias_is_rejected_even_for_identical_contents(self):
        check, admitted = self.fixture()
        alias = self.folder / "payload-alias"
        alias.symlink_to(self.payload, target_is_directory=True)
        admitted["readonly_roots"] = (alias, self.compiler)
        with self.assertRaises(VERIFY.Incomplete):
            VERIFY.admission(check, self.folder / "admission.json", profile="kani-proof")

    def test_kani_compiler_and_entrypoint_paths_cannot_redirect(self):
        for field, value in (("toolchain", "common/unchecked-compiler"),
                             ("entrypoint", "common/bin/unchecked-tool")):
            check, admitted = self.fixture()
            admitted["policy"]["tools"]["kani"][field] = value
            with self.subTest(field=field), self.assertRaises(VERIFY.Incomplete):
                VERIFY.admission(check, self.folder / "admission.json", profile="kani-proof")
            check.digest.assert_not_called()

    def test_pointer_bookend_drift_is_still_incomplete(self):
        check, _ = self.fixture()
        check.read_once.side_effect = [self.pointer, b'{"changed": true}']
        with self.assertRaises(VERIFY.Incomplete):
            VERIFY.admission(check, self.folder / "admission.json", profile="kani-proof")

    def test_execute_job_passes_exact_admitted_roots_into_runtime(self):
        check, admitted = self.fixture()
        custody_runtime(check, self.pointer)
        work = self.folder / "work"
        work.mkdir(mode=0o700)
        source = work / "source"
        source.mkdir()
        (source / "fixture.rs").write_text("// frozen fixture-only source\n")
        job = VERIFY.jobs("rounding")[0]

        def prepare(source, inputs, case):
            entry = inputs / "proof.rs"
            entry.write_text("// fixture-only prepared input\n")
            return {"entrypoint": entry}

        with patch.object(VERIFY, "require_sources"), patch.object(VERIFY, "prepare_rounding", side_effect=prepare), \
             patch.object(VERIFY, "admission", side_effect=receipt_admission(admitted, {"seconds": 0.0})) as admission, \
             patch.object(VERIFY, "qualify", return_value={"solver_seconds": 0.0}):
            result = VERIFY.execute_job(check, work, source, {}, "rounding", job, 1,
                                        tool_admission_profile="kani-proof")
        self.assertTrue(result["expected_outcome_qualified"], result)
        check.RUNTIME.run.assert_called_once()
        check.RUNTIME._owned_work.assert_called_once_with(work, VERIFY.ROOT)
        self.assertEqual(admission.call_args.kwargs, {"profile": "kani-proof"})
        options = check.RUNTIME.run.call_args.kwargs
        self.assertIn(source, options["sealed"])
        self.assertIn(Path(result["output"]) / "admission.json", options["sealed"])
        self.assertEqual(options["readonly"], (self.payload, self.compiler))
        self.assertEqual(options["path_dirs"], [self.payload / "bin", self.compiler / "bin"])
        self.assertEqual((options["timeout"], options["cpu_seconds"], options["process_limit"],
                          options["build_jobs"], options["address_space_bytes"]),
                         (180, 180, 128, 2, 8 * 1024 ** 3))

    def test_main_uses_selected_profile_for_commands_and_final_readmission(self):
        for selected in (None, "kani-proof"):
            work = self.folder / ("full-main" if selected is None else "kani-main")
            work.mkdir(mode=0o700)
            expected = "full" if selected is None else selected
            arguments = ["--output-dir", str(work), "--subject", "coordinate-exchange",
                         "--only-jobs", "9", "3", "1", "5", "4"]
            if selected is not None:
                arguments += ["--tool-admission-profile", selected]
            check = source_custody_check()
            actual_loader = VERIFY.module

            def local_loader(name, path):
                return check if name == "formal_analysis_admission" else actual_loader(name, path)

            def executed(check, work, source, records, subject, job, index, **options):
                self.assertEqual(set(options), {"tool_admission_profile", "retained"})
                self.assertEqual(options["tool_admission_profile"], expected)
                assert_source_anchor(self, check, work, source, options["retained"])
                return {"output": str(work / f"job-{index}"), "expected_outcome_qualified": True,
                        "native": {"exit_code": 0}}

            with patch.object(VERIFY, "fresh_output", return_value=work), \
                 patch.object(VERIFY, "module", side_effect=local_loader), \
                 patch.object(VERIFY, "input_names", return_value=["tools/formal/verify.py"]), \
                 patch.object(VERIFY, "execute_job", side_effect=executed) as execute, \
                 patch.object(VERIFY, "admission", side_effect=receipt_admission({}, {"seconds": 0.0})) as admission, \
                 patch("sys.stdout", new=io.StringIO()):
                self.assertEqual(VERIFY.main(arguments), 0)
            self.assertEqual(execute.call_count, 5)
            self.assertEqual(admission.call_args.kwargs, {"profile": expected})
            summary = json.loads((work / "summary.json").read_text())
            self.assertFalse(summary["complete_schedule"])
            self.assertEqual([job["index"] for job in summary["jobs"]], [1, 3, 4, 5, 9])
            self.assertEqual(summary["tool_admission_profile"], expected)
            self.assertTrue(summary["all_proof_inputs_and_required_receipts_unchanged"])

    def test_cli_invalid_selection_never_creates_output_or_admits_tools(self):
        for selection in (("--only-jobs", "1", "1"), ("--only-jobs", "0"),
                          ("--only-jobs", "13"), ("--start-at", "2", "--only-jobs", "3")):
            arguments = ["--output-dir", str(self.folder / "must-not-exist"), *selection]
            with self.subTest(selection=selection), patch.object(VERIFY, "fresh_output") as output, \
                 patch.object(VERIFY, "admission") as admission, patch("sys.stderr", new=io.StringIO()), \
                 self.assertRaises(SystemExit) as exited:
                VERIFY.main(arguments)
            self.assertEqual(exited.exception.code, 2)
            output.assert_not_called()
            admission.assert_not_called()


if __name__ == "__main__":
    unittest.main()
