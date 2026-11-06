"""Challenge compatibility admission with local traces and synthetic Verus output.

These controls never install, launch or require a Verus payload. They verify the
guard's evidence interpretation, not the product mathematics or verifier itself.
"""
from __future__ import annotations

from copy import deepcopy
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("verus_pilot_test", HERE / "verus_pilot.py")
PILOT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PILOT)
SOURCE = "/work/jobs/03-assertion_fault/inputs/proof.rs"
LINE = 37


def report(*, fault=False):
    """Independent expected upstream protocol fixture for the two named controls."""
    return {
        "verification-results": {
            "encountered-error": fault,
            "encountered-vir-error": False,
            "success": not fault,
            "verified": 1 if fault else 2,
            "errors": 1 if fault else 0,
            "is-verifying-entire-crate": True,
        },
        "func-details": {"obligation_proof_notes": [], "failed_proof_notes": []},
        "verus": {"version": "fixture-only"},
        "times-ms": {"smt": {"smt-run-module-times": [{
            "function-breakdown": [
                {"function": "proof::propagated_failure", "success": True, "mode:": "exec"},
                {"function": "proof::compatibility_healthy", "success": not fault, "mode:": "exec"},
            ],
        }]}},
    }


def diagnostic(message="assertion failed", *, source=SOURCE, line=LINE, primary=True):
    return {"$message_type": "diagnostic", "level": "error", "message": message,
            "spans": [{"file_name": source, "line_start": line, "line_end": line,
                       "is_primary": primary}]}


def native(value, *, fault=False, diagnostics=()):
    return {"failure": None, "exit_code": 1 if fault else 0,
            "output": "\n".join([*(json.dumps(item) for item in diagnostics), json.dumps(value, indent=2)])}


def combined_report():
    """Two healthy controls plus a separately verified, deliberately false assertion."""
    value = report(fault=True)
    value["verification-results"]["verified"] = 2
    functions = value["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"]
    functions[1]["success"] = True
    functions.append({"function": "proof::compatibility_fault", "success": False, "mode:": "exec"})
    return value


class TraceAdmissionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-verus-trace-controls-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.payload = self.base / "payload"
        self.trace = self.base / "exec.trace"

    def inspect(self, text):
        self.trace.write_text(text)
        return PILOT.check_trace(self.trace, self.payload)

    def call(self, path):
        return f'1001 execve({json.dumps(str(path))}, ["fixture"], 0x1234 /* 4 vars */) = 0\n'

    def test_literal_launcher_verifier_self_exec_and_solver_closure_is_accepted(self):
        expected = ["/usr/bin/env", str(self.payload / "verus"), str(self.payload / "rust_verify"),
                    str(self.payload / "rust_verify"), str(self.payload / "z3")]
        self.assertEqual(self.inspect("".join(self.call(path) for path in expected)), expected)

    def test_escaped_newline_cannot_hide_an_unexpected_child(self):
        expected = self.call("/usr/bin/env")
        with self.assertRaises(RuntimeError):
            self.inspect(expected + self.call(self.payload / "unexpected\nchild"))

    def test_unexpected_child_attempt_is_rejected_even_when_execution_fails(self):
        unexpected = self.call("/bin/sh").replace(" = 0", " = -1 ENOENT (No such file or directory)")
        with self.assertRaises(RuntimeError):
            self.inspect(self.call("/usr/bin/env") + unexpected)

    def test_malformed_unparsed_execve_and_execveat_are_rejected(self):
        for extra in ('1002 execve(0xdeadbeef, [], 0) = -1 EFAULT\n',
                      '1002 execve("unterminated\n',
                      '1002 execveat(3, "z3", [], 0, AT_EMPTY_PATH) = 0\n'):
            with self.subTest(extra=extra), self.assertRaises(RuntimeError):
                self.inspect(self.call("/usr/bin/env") + extra)
        with self.assertRaises(RuntimeError):
            self.inspect("1001 +++ exited with 0 +++\n")


class CommandContractTests(unittest.TestCase):
    def test_loader_and_proof_options_are_fixed_despite_ambient_injection(self):
        payload = Path("/fixture/payload")
        with patch.dict(os.environ, {"LD_LIBRARY_PATH": "/untrusted", "VERUS_Z3_PATH": "/untrusted/z3"}):
            command = PILOT.command(payload, SOURCE, "/work/exec.trace")
        env_position = command.index("/usr/bin/env")
        loader = command[env_position + 1]
        expected_dirs = [str(PILOT.SDK / "lib"), str(PILOT.SDK / "lib/rustlib/x86_64-unknown-linux-gnu/lib")]
        self.assertEqual(loader, "LD_LIBRARY_PATH=" + ":".join(expected_dirs))
        self.assertEqual(command[env_position + 2], str(payload / "verus"))
        self.assertIn("--num-threads=1", command)
        self.assertIn("--no-cheating", command)
        self.assertIn("--error-format=json", command)
        self.assertEqual(command[command.index("--smt-option") + 1], "timeout=60000")
        self.assertFalse(any(value.startswith("--verify-") for value in command))
        version = PILOT.command(payload, SOURCE, "/work/exec.trace", version=True)
        self.assertEqual(version[version.index(str(payload / "verus")) + 1:], ["--version"])


class VerificationResultTests(unittest.TestCase):
    def test_healthy_whole_crate_result_requires_both_verified_functions(self):
        result = PILOT.verification(native(report()), False, SOURCE, 0)
        self.assertTrue(result["qualified"])
        self.assertEqual(result["functions"], {"propagated_failure": True, "compatibility_healthy": True})

    def test_omitted_unexpected_duplicate_and_failed_function_are_rejected(self):
        for change in ("omitted", "unexpected", "duplicate", "failed"):
            value = report()
            functions = value["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"]
            if change == "omitted":
                functions.pop()
            elif change == "unexpected":
                functions[1]["function"] = "proof::unrelated"
            elif change == "duplicate":
                functions.append(deepcopy(functions[0]))
            else:
                functions[1]["success"] = False
            with self.subTest(change=change), self.assertRaises(RuntimeError):
                PILOT.verification(native(value), False, SOURCE, 0)

    def test_filtered_missing_or_vacuous_whole_crate_results_are_rejected(self):
        for key, wrong in (("is-verifying-entire-crate", False), ("verified", 0),
                           ("verified", 1), ("verified", 3), ("errors", 1),
                           ("errors", False), ("encountered-vir-error", True), ("success", 1)):
            value = report()
            value["verification-results"][key] = wrong
            with self.subTest(key=key, wrong=wrong), self.assertRaises(RuntimeError):
                PILOT.verification(native(value), False, SOURCE, 0)
        value = report()
        del value["verification-results"]["verified"]
        with self.assertRaises(RuntimeError):
            PILOT.verification(native(value), False, SOURCE, 0)

    def test_missing_aggregate_or_multiple_reports_are_rejected(self):
        value = native({"verus": {"version": "fixture"}})
        with self.assertRaises(RuntimeError):
            PILOT.verification(value, False, SOURCE, 0)
        value = native(report())
        value["output"] += "\n" + json.dumps(report())
        with self.assertRaises(RuntimeError):
            PILOT.verification(value, False, SOURCE, 0)

    def test_intended_assertion_fault_accepts_only_its_primary_source_span(self):
        assertion = diagnostic()
        abort = diagnostic("aborting due to 1 previous error", primary=False)
        result = PILOT.verification(native(report(fault=True), fault=True, diagnostics=[assertion, abort]),
                                    True, SOURCE, LINE)
        self.assertTrue(result["qualified"])
        self.assertEqual(result["functions"], {"propagated_failure": True, "compatibility_healthy": False})

    def test_wrong_missing_and_ambiguous_assertion_primary_spans_are_rejected(self):
        cases = [diagnostic(source="/work/other.rs"), diagnostic(line=LINE + 1), diagnostic(primary=False)]
        ambiguous = diagnostic()
        ambiguous["spans"].append(deepcopy(ambiguous["spans"][0]))
        cases.append(ambiguous)
        for assertion in cases:
            with self.subTest(diagnostic=assertion), self.assertRaises(RuntimeError):
                PILOT.verification(native(report(fault=True), fault=True, diagnostics=[assertion]),
                                   True, SOURCE, LINE)
        with self.assertRaises(RuntimeError):
            PILOT.verification(native(report(fault=True), fault=True), True, SOURCE, LINE)

    def test_resource_and_unsupported_api_errors_are_not_assertion_sensitivity(self):
        for message in ("resource limit exceeded", "query timed out", "unsupported function: checked_shl"):
            for diagnostics in ([diagnostic(message)], [diagnostic(), diagnostic(message, primary=False)]):
                with self.subTest(message=message, diagnostics=diagnostics), self.assertRaises(RuntimeError):
                    PILOT.verification(native(report(fault=True), fault=True, diagnostics=diagnostics),
                                       True, SOURCE, LINE)

    def test_native_custody_failure_wrong_exit_and_healthy_error_are_rejected(self):
        for failure, code in (("wall timeout (180s)", 0), (None, 1), (None, None)):
            value = native(report())
            value.update(failure=failure, exit_code=code)
            with self.subTest(failure=failure, code=code), self.assertRaises(RuntimeError):
                PILOT.verification(value, False, SOURCE, 0)
        with self.assertRaises(RuntimeError):
            PILOT.verification(native(report(), diagnostics=[diagnostic()]), False, SOURCE, 0)


class CombinedControlTests(unittest.TestCase):
    def check(self, value, *, assertion=None):
        return PILOT.verification(
            native(value, fault=True, diagnostics=[diagnostic() if assertion is None else assertion]),
            True, SOURCE, LINE, combined=True)

    def test_two_healthy_controls_and_distinct_assertion_fault_are_accepted(self):
        result = self.check(combined_report())
        self.assertTrue(result["qualified"])
        self.assertEqual(result["verification_results"]["verified"], 2)
        self.assertEqual(result["verification_results"]["errors"], 1)
        self.assertEqual(result["functions"], {"propagated_failure": True, "compatibility_healthy": True,
                                              "compatibility_fault": False})

    def test_either_failed_healthy_control_is_rejected(self):
        for index in (0, 1):
            value = combined_report()
            value["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"][index]["success"] = False
            with self.subTest(index=index), self.assertRaises(RuntimeError):
                self.check(value)

    def test_missing_or_successful_fault_function_is_rejected(self):
        for change in ("missing", "successful"):
            value = combined_report()
            functions = value["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"]
            if change == "missing":
                functions.pop()
            else:
                functions[2]["success"] = True
            with self.subTest(change=change), self.assertRaises(RuntimeError):
                self.check(value)

    def test_incorrect_combined_aggregates_and_filtered_scope_are_rejected(self):
        for key, wrong in (("verified", 1), ("verified", 3), ("errors", 0), ("errors", 2),
                           ("success", True), ("encountered-vir-error", True),
                           ("is-verifying-entire-crate", False)):
            value = combined_report()
            value["verification-results"][key] = wrong
            with self.subTest(key=key, wrong=wrong), self.assertRaises(RuntimeError):
                self.check(value)

    def test_combined_mode_needs_explicit_fault_mode_and_does_not_change_default(self):
        with self.assertRaises(RuntimeError):
            PILOT.verification(native(combined_report(), fault=True, diagnostics=[diagnostic()]),
                               False, SOURCE, LINE, combined=True)
        with self.assertRaises(RuntimeError):
            PILOT.verification(native(combined_report(), fault=True, diagnostics=[diagnostic()]),
                               True, SOURCE, LINE)

    def test_combined_fault_still_requires_the_exact_primary_assertion_span(self):
        with self.assertRaises(RuntimeError):
            self.check(combined_report(), assertion=diagnostic(line=LINE + 1))


if __name__ == "__main__":
    unittest.main()
