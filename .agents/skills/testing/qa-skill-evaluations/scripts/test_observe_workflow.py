#!/usr/bin/env python3
"""Independent synthetic outcomes for the offline workflow observer, not product QA."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).with_name("observe_workflow.py")
SPEC = importlib.util.spec_from_file_location("workflow_observer", SCRIPT)
OBSERVER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(OBSERVER)


class ObserverTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.serial = 0

    def receipt(self, status="incomplete", *, kind="engine-analysis", lane="mutation", qualified=False, **extra):
        self.serial += 1
        path = self.root / f"receipt-{self.serial}.json"
        scope = ("Bounded host engine " + lane if kind == "engine-analysis" else
                 "UI-M0, wire, domain, exchange, synthetic regression and controlled-loopback engine pilot portable verification")
        value = {"scope": scope, "status": status, **extra}
        if kind == "engine-analysis":
            value.update(selected_lane=lane, qualification_passed=qualified)
        if kind == "product" and status == "passed":
            value.update(source_sha256={"source.rs": "a" * 64},
                         lanes=[{"lane": "compiler", "command": ["cargo", "check"], "exit_code": 0,
                                 "failure": None, "output": "healthy compiler control"}])
        path.write_text(json.dumps(value), encoding="utf-8")
        return {"path": str(path), "kind": kind, "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}

    def event(self, event_id, receipt=None, **extra):
        return {"id": event_id, "task": "change", "worker": "owner", "role": "implementation",
                "context": "candidate-harness-environment-A", "outcome": "unexpected-failure",
                "failure": "counterexample-A", "remedy": "repeat-config-change", "observation": "trace-A",
                **({"receipt": receipt} if receipt else {}), **extra}

    def ledger(self, events, *, workers=None, tasks=None):
        return {"schema_version": 1, "workers": workers or ["owner"],
                "tasks": tasks or [{"id": "change", "outcome": "incomplete", "accounting_complete": False}],
                "events": events}

    def cli(self, value):
        ledger = self.root / "ledger.json"
        ledger.write_text(value if isinstance(value, str) else json.dumps(value), encoding="utf-8")
        result = subprocess.run([sys.executable, "-B", str(SCRIPT), "--ledger", str(ledger)],
                                capture_output=True, text=True, timeout=5, check=False)
        self.assertEqual(result.stderr, "")
        return result.returncode, json.loads(result.stdout)

    def test_real_cli_healthy_and_repeated_failure_without_writes(self):
        good = self.receipt("passed", lane="coverage", qualified=True)
        bad = self.receipt()
        events = [self.event("healthy", good, outcome="passed"), self.event("attempt-1", bad), self.event("attempt-2", bad)]
        before = {path.name: path.read_bytes() for path in self.root.iterdir()}
        code, report = self.cli(self.ledger(events))
        self.assertEqual(code, 0)
        self.assertFalse(report["qualification"])
        self.assertEqual(report["observations"][0]["outcome"], "passed")
        self.assertEqual(report["advisories"][0]["events"], ["attempt-1", "attempt-2"])
        self.assertEqual(report["advisories"][0]["failure_cause"], "unclassified")
        self.assertEqual({path.name: path.read_bytes() for path in self.root.iterdir() if path.name != "ledger.json"}, before)

    def test_expected_controls_require_reviewed_cause_and_successful_wrapper(self):
        good = self.receipt("passed", lane="mutation", qualified=True,
                            lanes=[{"exit_code": 101, "lane": "caught-mutant"}])
        intended = {"reviewed": True, "intended": "bounds-rejection", "observed": "bounds-rejection"}
        events = [self.event("negative-1", good, outcome="expected-failure", expected_cause=intended),
                  self.event("negative-2", good, outcome="expected-failure", expected_cause=intended)]
        report = OBSERVER.analyze(self.ledger(events))
        self.assertEqual([item["outcome"] for item in report["observations"]], ["expected-failure"] * 2)
        self.assertEqual(report["advisories"], [])
        events[0]["expected_cause"]["observed"] = "unrelated-import-error"
        events[1]["receipt"] = self.receipt()
        report = OBSERVER.analyze(self.ledger(events))
        self.assertEqual([item["outcome"] for item in report["observations"]], ["unknown", "incomplete"])

    def test_enumeration_incomplete_and_missing_metadata_never_qualify(self):
        enum = self.receipt("enumerated", lane="enumerate", qualified=False)
        events = [self.event("list", enum, outcome="passed"), self.event("setup", self.receipt(), outcome="passed"),
                  self.event("unobserved", outcome="passed")]
        report = OBSERVER.analyze(self.ledger(events))
        self.assertEqual([item["outcome"] for item in report["observations"]], ["enumerated", "incomplete", "unknown"])
        self.assertFalse(report["qualification"])
        self.assertEqual(report["advisories"], [])

    def test_workers_do_not_share_failure_chains(self):
        bad = self.receipt()
        events = [self.event("one", bad), self.event("two", bad, worker="reviewer")]
        report = OBSERVER.analyze(self.ledger(events, workers=["owner", "reviewer"]))
        self.assertEqual(report["advisories"], [])
        events.append(self.event("three", bad, worker="reviewer"))
        report = OBSERVER.analyze(self.ledger(events, workers=["owner", "reviewer"]))
        self.assertEqual(report["advisories"][0]["worker"], "reviewer")
        self.assertEqual(report["advisories"][0]["events"], ["two", "three"])

    def test_new_context_observation_or_failure_resets_chain(self):
        bad = self.receipt()
        for field in ("context", "observation", "failure", "remedy"):
            with self.subTest(field=field):
                events = [self.event("one", bad), self.event("two", bad, **{field: "new-evidence"})]
                self.assertEqual(OBSERVER.analyze(self.ledger(events))["advisories"], [])

    def test_missing_drifted_unsupported_and_invalid_receipts_are_unknown(self):
        good = self.receipt("passed", kind="product")
        missing = {**good, "path": str(self.root / "absent.json")}
        drifted = {**good, "sha256": "0" * 64}
        unsupported = self.receipt("success", kind="product")
        malformed = self.receipt("passed", qualified="false")
        events = [self.event(str(index), receipt, outcome="passed")
                  for index, receipt in enumerate((good, missing, drifted, unsupported, malformed))]
        report = OBSERVER.analyze(self.ledger(events))
        self.assertEqual([item["outcome"] for item in report["observations"]], ["passed", "unknown", "unknown", "unknown", "unknown"])
        self.assertEqual(report["observations"][2]["receipt_state"], "drifted")

    def test_conflicting_or_unrelated_wrappers_are_not_recognized(self):
        def pinned(kind, report):
            path = self.root / f"conflict-{len(list(self.root.iterdir()))}.json"
            path.write_text(json.dumps(report), encoding="utf-8")
            return {"kind": kind, "path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
        product_scope = "UI-M0, wire, domain, exchange, synthetic regression and controlled-loopback engine pilot portable verification"
        healthy_product = {"scope": product_scope, "status": "passed", "source_sha256": {"source.rs": "a" * 64},
                           "lanes": [{"lane": "compiler", "command": ["cargo", "check"], "exit_code": 0,
                                      "failure": None, "output": "healthy compiler control"}]}
        healthy_engine = {"scope": "Bounded host engine coverage", "status": "passed",
                          "selected_lane": "coverage", "qualification_passed": True}
        cases = [("product", {**healthy_product, "scope": "unrelated program"}),
                 ("product", {**healthy_product, "source_sha256": {"source.rs": "not-a-hash"}}),
                 ("product", {**healthy_product, "error": "source drift detected"}),
                 ("product", {**healthy_product, "lanes": [{"exit_code": 101, "failure": "failed"}]}),
                 ("engine-analysis", {**healthy_engine, "scope": "unrelated program"}),
                 ("engine-analysis", {**healthy_engine, "error": "source drift detected"}),
                 ("engine-analysis", {**healthy_engine, "status": "incomplete", "qualification_passed": False,
                                      "scope": "unrelated program"})]
        for kind, report in cases:
            with self.subTest(kind=kind, report=report):
                receipt = pinned(kind, report)
                events = [self.event("one", receipt), self.event("two", receipt)]
                observed = OBSERVER.analyze(self.ledger(events))
                self.assertEqual([item["receipt_state"] for item in observed["observations"]], ["unknown", "unknown"])
                self.assertEqual(observed["advisories"], [])

    def test_all_tasks_and_roles_remain_in_accounting_with_unknown_usage(self):
        tasks = [{"id": "change", "outcome": "accepted", "accounting_complete": True, "wall_seconds": 10},
                 {"id": "unfinished", "outcome": "failed", "accounting_complete": False}]
        events = [self.event("one", effort_seconds=12, wait_seconds=8,
                             usage={"input_tokens": 100, "cached_input_tokens": 70, "output_tokens": 5,
                                    "cost_usd": 0.4, "pricing_basis": "recorded"}),
                  self.event("two", task="unfinished", role="review", effort_seconds=4,
                             usage={"cost_usd": 0.2, "pricing_basis": "estimate"}),
                  self.event("three", task="unfinished", role="repair"),
                  self.event("four", task="unfinished", role="observer"),
                  self.event("five", task="unfinished", role="verification")]
        report = OBSERVER.analyze(self.ledger(events, tasks=tasks))
        self.assertEqual(report["quality"], {"assigned": 2, "accepted": 1, "failed": 1, "incomplete": 0})
        account = report["accounting"]
        self.assertEqual(account["complete_cost_task_ids"], ["change"])
        self.assertEqual(account["quality_task_denominator"], 2)
        self.assertEqual(account["cost_task_denominator"], 1)
        self.assertEqual(account["metrics"]["effort_seconds"]["known_subtotal"], 16)
        self.assertEqual(account["tasks"]["change"]["wall_seconds"], 10)
        self.assertEqual(account["metrics"]["input_tokens"]["known_subtotal"], 100)
        self.assertEqual(account["metrics"]["cached_input_tokens"]["known_subtotal"], 70)
        self.assertEqual(account["costs_by_basis"]["recorded"]["known_subtotal"], 0.4)
        self.assertEqual(account["costs_by_basis"]["estimate"]["known_subtotal"], 0.2)
        self.assertEqual(account["complete_cost_subset_by_basis"]["recorded"]["known_subtotal"], 0.4)
        self.assertIsNone(account["complete_cost_subset_by_basis"]["estimate"]["known_subtotal"])
        self.assertFalse(account["cost_coverage"]["complete"])
        self.assertFalse(account["complete"])

    def test_unobserved_worker_and_zero_event_task_keep_accounting_incomplete(self):
        value = self.ledger([], workers=["owner", "hidden-worker"])
        value["tasks"][0]["accounting_complete"] = True
        report = OBSERVER.analyze(value)
        self.assertEqual(report["visibility"]["unobserved_tasks"], ["change"])
        self.assertEqual(report["visibility"]["unobserved_workers"], ["hidden-worker", "owner"])
        self.assertIsNone(report["accounting"]["metrics"]["input_tokens"]["known_subtotal"])
        self.assertFalse(report["accounting"]["complete"])

    def test_fully_accounted_control_and_added_unobserved_task(self):
        events = [self.event("one", effort_seconds=1, wait_seconds=2,
                             usage={"input_tokens": 4, "cached_input_tokens": 1, "output_tokens": 3,
                                    "cost_usd": 0.1, "pricing_basis": "recorded"})]
        value = self.ledger(events)
        value["tasks"][0]["accounting_complete"] = True
        self.assertTrue(OBSERVER.analyze(value)["accounting"]["complete"])
        value["tasks"].append({"id": "hidden-task", "outcome": "incomplete", "accounting_complete": True})
        report = OBSERVER.analyze(value)
        self.assertFalse(report["accounting"]["complete"])
        self.assertEqual(report["visibility"]["unobserved_tasks"], ["hidden-task"])
        value["tasks"].pop()
        value["workers"].append("hidden-worker")
        report = OBSERVER.analyze(value)
        self.assertFalse(report["accounting"]["complete"])
        self.assertEqual(report["accounting"]["complete_cost_task_ids"], [])

    def test_strict_validation_rejects_false_string_duplicate_replay_and_unknown_fields(self):
        valid = self.ledger([self.event("one")])
        variants = []
        value = copy.deepcopy(valid); value["tasks"][0]["accounting_complete"] = "false"; variants.append(value)
        value = copy.deepcopy(valid); value["events"] *= 2; variants.append(value)
        value = copy.deepcopy(valid); value["events"][0]["made_up"] = 1; variants.append(value)
        value = copy.deepcopy(valid); value["events"][0]["usage"] = {"input_tokens": True}; variants.append(value)
        value = copy.deepcopy(valid); value["events"][0]["usage"] = {"input_tokens": 1, "cached_input_tokens": 2}; variants.append(value)
        value = copy.deepcopy(valid); value["events"][0]["effort_seconds"] = float("inf"); variants.append(value)
        value = copy.deepcopy(valid); value["events"][0]["expected_cause"] = {"reviewed": "false", "intended": "x", "observed": "x"}; variants.append(value)
        for value in variants:
            with self.subTest(value=value):
                code, report = self.cli(value)
                self.assertEqual(code, 2)
                self.assertIn("error", report)
        code, report = self.cli('{"schema_version":1,"schema_version":1}')
        self.assertEqual(code, 2)
        self.assertIn("Duplicate", report["error"])

    def test_fifo_and_receipt_byte_bounds_do_not_block_or_overread_aggregate(self):
        fifo = self.root / "fifo"
        os.mkfifo(fifo)
        with self.assertRaises(ValueError):
            OBSERVER.read_json(fifo, 32)
        receipt = self.receipt()
        oversized = Path(receipt["path"])
        oversized.write_bytes(b" " * 200)
        events = [self.event("one", receipt), self.event("two", {**receipt, "sha256": "b" * 64})]
        with patch.object(OBSERVER, "MAX_RECEIPT_BYTES", 64), patch.object(OBSERVER, "MAX_TOTAL_RECEIPT_BYTES", 100):
            report = OBSERVER.analyze(self.ledger(events))
        self.assertLessEqual(report["limits"]["receipt_bytes_read"], 100)
        self.assertTrue(all(item["outcome"] == "unknown" for item in report["observations"]))

    def test_invalid_json_still_consumes_aggregate_read_allowance(self):
        events = []
        for index in range(4):
            path = self.root / f"invalid-{index}.json"
            path.write_bytes(b"{" * 40)
            receipt = {"kind": "product", "path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
            events.append(self.event(str(index), receipt))
        with patch.object(OBSERVER, "MAX_RECEIPT_BYTES", 64), patch.object(OBSERVER, "MAX_TOTAL_RECEIPT_BYTES", 100):
            report = OBSERVER.analyze(self.ledger(events))
        self.assertEqual(report["limits"]["receipt_bytes_read"], 100)
        self.assertEqual(report["advisories"], [])

    def test_ledger_and_event_caps_reject_excess_with_healthy_neighbors(self):
        raw = json.dumps(self.ledger([])).encode()
        path = self.root / "finite.json"
        path.write_bytes(raw)
        self.assertEqual(OBSERVER.read_json(path, len(raw))[0]["schema_version"], 1)
        with self.assertRaises(ValueError):
            OBSERVER.read_json(path, len(raw) - 1)
        with patch.object(OBSERVER, "MAX_EVENTS", 1):
            self.assertEqual(OBSERVER.analyze(self.ledger([self.event("one")]))["quality"]["assigned"], 1)
            with self.assertRaises(ValueError):
                OBSERVER.analyze(self.ledger([self.event("one"), self.event("two")]))


if __name__ == "__main__":
    unittest.main()
