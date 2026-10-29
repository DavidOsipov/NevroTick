"""Status aggregation controls independent of GitHub's aggregate matrix result."""

import importlib.util
from contextlib import redirect_stderr, redirect_stdout
from io import StringIO
import json
import os
from pathlib import Path
import unittest
from unittest import mock


SPEC = importlib.util.spec_from_file_location("security_gate", Path(__file__).with_name("check_gate.py"))
GATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GATE)
COMMIT = "a" * 40


def healthy():
    result = {name: {"result": "success", "outputs": {"candidate": COMMIT}}
              for name in ("source-inventory", "workflow-policy", "secrets")}
    result["source-inventory"]["outputs"].update(
        python="true", javascript="true", rust="false", codeql="deferred")
    result.update({name: {"result": "skipped", "outputs": {}}
                   for name in ("codeql-actions", "codeql-python", "codeql-javascript", "codeql-rust")})
    return result


class FinalGateControls(unittest.TestCase):
    def test_explicit_deferral_accepts_skipped_codeql_with_known_sources(self):
        GATE.evaluate(healthy(), COMMIT)
        candidate = healthy()
        candidate["source-inventory"]["outputs"]["rust"] = "true"
        GATE.evaluate(candidate, COMMIT)

    def test_failed_cancelled_skipped_or_missing_mandatory_job_blocks(self):
        for job in ("source-inventory", "workflow-policy", "secrets"):
            for result in ("failure", "cancelled", "skipped", "neutral"):
                candidate = healthy()
                candidate[job]["result"] = result
                with self.assertRaises(ValueError):
                    GATE.evaluate(candidate, COMMIT)
            candidate = healthy()
            del candidate[job]
            with self.assertRaises(ValueError):
                GATE.evaluate(candidate, COMMIT)

    def test_only_explicitly_deferred_policy_is_accepted(self):
        for status in (None, "enabled", "disabled", "", "deferred "):
            candidate = healthy()
            candidate["source-inventory"]["outputs"]["codeql"] = status
            with self.assertRaises(ValueError):
                GATE.evaluate(candidate, COMMIT)
        candidate = healthy()
        del candidate["source-inventory"]["outputs"]["codeql"]
        with self.assertRaises(ValueError):
            GATE.evaluate(candidate, COMMIT)

    def test_unexpected_codeql_execution_or_missing_job_blocks(self):
        for job in ("codeql-actions", "codeql-python", "codeql-javascript", "codeql-rust"):
            for result in ("success", "failure", "cancelled", "neutral"):
                candidate = healthy()
                candidate[job] = {"result": result, "outputs": {"candidate": COMMIT}}
                with self.assertRaises(ValueError):
                    GATE.evaluate(candidate, COMMIT)
            candidate = healthy()
            del candidate[job]
            with self.assertRaises(ValueError):
                GATE.evaluate(candidate, COMMIT)
            candidate = healthy()
            candidate[job]["outputs"]["candidate"] = COMMIT
            with self.assertRaises(ValueError):
                GATE.evaluate(candidate, COMMIT)

    def test_invalid_source_presence_or_stale_candidate_blocks(self):
        candidate = healthy()
        candidate["source-inventory"]["outputs"]["candidate"] = "b" * 40
        with self.assertRaises(ValueError):
            GATE.evaluate(candidate, COMMIT)
        for language in ("python", "javascript", "rust"):
            for present in (None, "", "unknown", False):
                candidate = healthy()
                candidate["source-inventory"]["outputs"][language] = present
                with self.assertRaises(ValueError):
                    GATE.evaluate(candidate, COMMIT)

    def test_success_message_reports_codeql_deferral(self):
        output = StringIO()
        with mock.patch.dict(os.environ, REQUIRED_RESULTS=json.dumps(healthy()), EXPECTED_COMMIT=COMMIT):
            with redirect_stdout(output):
                self.assertEqual(GATE.main(), 0)
        self.assertIn("CodeQL is deferred", output.getvalue())
        self.assertIn("no CodeQL SAST evidence", output.getvalue())

    def test_failure_does_not_print_a_success_claim(self):
        candidate = healthy()
        candidate["codeql-rust"]["result"] = "success"
        output, errors = StringIO(), StringIO()
        with mock.patch.dict(os.environ, REQUIRED_RESULTS=json.dumps(candidate), EXPECTED_COMMIT=COMMIT):
            with redirect_stdout(output), redirect_stderr(errors):
                self.assertEqual(GATE.main(), 1)
        self.assertEqual(output.getvalue(), "")
        self.assertIn("FAILED", errors.getvalue())


if __name__ == "__main__":
    unittest.main()
