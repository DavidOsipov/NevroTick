"""Status aggregation controls independent of GitHub's aggregate matrix result."""

import importlib.util
from pathlib import Path
import unittest


SPEC = importlib.util.spec_from_file_location("security_gate", Path(__file__).with_name("check_gate.py"))
GATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GATE)
COMMIT = "a" * 40


def healthy():
    result = {name: {"result": "success", "outputs": {"candidate": COMMIT}}
              for name in ("source-inventory", "workflow-policy", "secrets", "codeql-actions",
                           "codeql-python", "codeql-javascript", "codeql-rust")}
    result["source-inventory"]["outputs"].update(python="true", javascript="true", rust="false")
    result["codeql-rust"] = {"result": "skipped"}
    return result


class FinalGateControls(unittest.TestCase):
    def test_known_sources_succeed_with_explicit_absent_rust(self):
        GATE.evaluate(healthy(), COMMIT)

    def test_rust_source_requires_its_own_success_and_identity(self):
        candidate = healthy()
        candidate["source-inventory"]["outputs"]["rust"] = "true"
        with self.assertRaises(ValueError):
            GATE.evaluate(candidate, COMMIT)
        candidate["codeql-rust"] = {"result": "success", "outputs": {"candidate": COMMIT}}
        GATE.evaluate(candidate, COMMIT)

    def test_any_failed_cancelled_skipped_or_missing_required_job_blocks(self):
        for job in healthy():
            for result in ("failure", "cancelled", "skipped", "neutral"):
                if job == "codeql-rust" and result == "skipped":
                    continue
                candidate = healthy()
                candidate[job]["result"] = result
                with self.assertRaises(ValueError):
                    GATE.evaluate(candidate, COMMIT)
            candidate = healthy()
            del candidate[job]
            with self.assertRaises(ValueError):
                GATE.evaluate(candidate, COMMIT)

    def test_missing_language_or_stale_candidate_evidence_blocks(self):
        for job in ("source-inventory", "codeql-actions", "codeql-python", "codeql-javascript"):
            candidate = healthy()
            candidate[job]["outputs"]["candidate"] = "b" * 40
            with self.assertRaises(ValueError):
                GATE.evaluate(candidate, COMMIT)
        candidate = healthy()
        del candidate["source-inventory"]["outputs"]["rust"]
        with self.assertRaises(ValueError):
            GATE.evaluate(candidate, COMMIT)


if __name__ == "__main__":
    unittest.main()
