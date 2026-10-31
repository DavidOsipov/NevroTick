"""Independent refusal/healthy controls for the state-guidance supervisor."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("state_guidance_run", Path(__file__).with_name("run.py"))
RUN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUN)


class AdmissionTests(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="state-guidance-policy-test-")
        self.addCleanup(self.scratch.cleanup)
        self.base = Path(self.scratch.name)

    def test_complete_json_and_duplicate_keys_are_distinct(self):
        path = self.base / "record.json"
        path.write_text('{"schema_version":1}')
        self.assertEqual(RUN.read_object(path), {"schema_version": 1})
        path.write_text('{"schema_version":1,"schema_version":2}')
        with self.assertRaisesRegex(RUN.Incomplete, "Duplicate"):
            RUN.read_object(path)

    def test_binding_challenges_current_bytes_and_symlink_redirection(self):
        path = self.base / "owned"
        path.write_bytes(b"independent input")
        bindings = {"owned": RUN.digest(path)}
        RUN.check_bindings(self.base, bindings)
        path.write_bytes(b"substituted input")
        with self.assertRaises(RUN.Incomplete):
            RUN.check_bindings(self.base, bindings)
        path.unlink()
        target = self.base / "other"
        target.write_bytes(b"independent input")
        path.symlink_to(target)
        with self.assertRaises(RUN.Incomplete):
            RUN.check_bindings(self.base, bindings)

    def test_parent_escape_and_absolute_binding_are_rejected(self):
        for value in ("../outside", "/tmp/absolute", ""):
            with self.subTest(value=value), self.assertRaises(RUN.Incomplete):
                RUN.relative_path(value)
        self.assertEqual(RUN.relative_path("engine/src/main.rs"), Path("engine/src/main.rs"))

    def test_fresh_output_preserves_existing_evidence(self):
        path = self.base / "evidence"
        path.write_text("retain original")
        with self.assertRaises(RUN.Incomplete):
            RUN.fresh_work(str(path))
        self.assertEqual(path.read_text(), "retain original")
        output = RUN.fresh_work(str(self.base / "fresh"))
        self.assertEqual(output.stat().st_mode & 0o777, 0o700)

    def test_seed_bytes_are_admitted_before_execution(self):
        work = self.base / "work"
        work.mkdir()
        RUN.write_seeds(work)
        actual = sorted((work / "seeds").glob("*.trace"))
        self.assertTrue(actual)
        self.assertEqual(actual[0].read_bytes(), bytes([1, 0, 240]))

    def test_advisory_warning_cannot_be_relabelled_as_a_passing_receipt(self):
        lock = self.base / "engine/Cargo.lock"
        lock.parent.mkdir()
        lock.write_text("independent locked graph")
        receipt = self.base / "advisory.json"
        report = {"status": "passed", "native_exit": 0, "supervisor_failure": None,
                  "security_vulnerability_count": 0, "warnings": [], "ignored_advisory_ids": [],
                  "lockfile_sha256": RUN.digest(lock)}
        receipt.write_text(json.dumps(report))
        policy = {"advisory_relative": "advisory.json", "advisory_sha256": RUN.digest(receipt)}
        with patch.object(RUN, "HERE", self.base):
            RUN.require_advisory(self.base, policy)
            report["warnings"] = [{"kind": "unmaintained"}]
            receipt.write_text(json.dumps(report))
            policy["advisory_sha256"] = RUN.digest(receipt)
            with self.assertRaisesRegex(RUN.Incomplete, "passing exact-lock"):
                RUN.require_advisory(self.base, policy)

    def test_advisory_evidence_is_bound_to_the_current_lock(self):
        lock = self.base / "engine/Cargo.lock"
        lock.parent.mkdir()
        lock.write_text("original graph")
        receipt = self.base / "advisory.json"
        receipt.write_text(json.dumps({"status": "passed", "native_exit": 0,
            "supervisor_failure": None, "security_vulnerability_count": 0,
            "warnings": [], "ignored_advisory_ids": [], "lockfile_sha256": RUN.digest(lock)}))
        policy = {"advisory_relative": "advisory.json", "advisory_sha256": RUN.digest(receipt)}
        lock.write_text("changed graph")
        with patch.object(RUN, "HERE", self.base), self.assertRaises(RUN.Incomplete):
            RUN.require_advisory(self.base, policy)


class ObservationTests(unittest.TestCase):
    @staticmethod
    def healthy():
        return {"status": "complete", "mode": "coverage", "seed": 7,
                "executions_requested": 1024, "executions_completed": 1024,
                "oracle_failures": 0, "observed_coverage_edges": [1, 2],
                "observed_semantic_features": [3, 4], "retained_semantic_features": [3],
                "registered_guards": 8, "corpus_count": 2, "saved_queue_files": 2,
                "completed_trace_steps": 2048, "malformed_inputs": 0, "completions": 3}

    def test_observed_complete_arm_is_accepted(self):
        RUN.validate_campaign(self.healthy(), "coverage", 7)

    def test_partial_wrong_owner_or_boolean_counts_never_become_complete(self):
        for name, value in (("status", "partial"), ("mode", "semantic"), ("seed", True),
                            ("executions_completed", 1023), ("executions_requested", 2048),
                            ("oracle_failures", False), ("oracle_failures", 1)):
            report = self.healthy()
            report[name] = value
            with self.subTest(field=name, value=value), self.assertRaises(RUN.Incomplete):
                RUN.validate_campaign(report, "coverage", 7)

    def test_empty_duplicate_out_of_range_or_unobserved_features_are_rejected(self):
        for name, value in (("observed_coverage_edges", []),
                            ("observed_coverage_edges", [1, 1]),
                            ("observed_coverage_edges", [65536]),
                            ("observed_semantic_features", [True]),
                            ("retained_semantic_features", [9])):
            report = self.healthy()
            report[name] = value
            with self.subTest(field=name), self.assertRaises(RUN.Incomplete):
                RUN.validate_campaign(report, "coverage", 7)

    def test_corpus_and_reached_work_cannot_be_missing_or_inconsistent(self):
        for name, value in (("registered_guards", 0), ("corpus_count", 0),
                            ("saved_queue_files", 3), ("completed_trace_steps", 0),
                            ("malformed_inputs", 1)):
            report = self.healthy()
            report[name] = value
            with self.subTest(field=name), self.assertRaises(RUN.Incomplete):
                RUN.validate_campaign(report, "coverage", 7)


if __name__ == "__main__":
    unittest.main()
