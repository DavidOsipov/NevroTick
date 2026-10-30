"""Independent finite evidence-accounting and artifact-custody controls."""
from __future__ import annotations

import importlib.util
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("portable_analysis_test", ROOT / "scripts/portable-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
FILES = tuple((name, f"crates/time-domain/src/{name}.rs")
              for name in ("coordinate", "era", "exchange", "clock", "rounding"))
SCOPE = MODULE.Scope("time-domain", FILES, "policy/time-domain-analysis.json")


def phase_records(summary="Success"):
    """Pinned literal Cargo phase schema, independent of the parser under test."""
    build = {"phase": "Build", "duration": 0.25, "process_status": "Success",
             "argv": ["cargo", "test", "--no-run", "--verbose", "--package=time-domain@0.0.0", "--frozen"]}
    test = {"phase": "Test", "duration": 0.5, "process_status": "Success",
            "argv": ["cargo", "test", "--verbose", "--package=time-domain@0.0.0", "--frozen"]}
    if summary == "Unviable":
        return [{**build, "process_status": {"Failure": 101}}]
    if summary == "CaughtMutant":
        test["process_status"] = {"Failure": 101}
    return [build, test]


def complete_report(value):
    """Complete older minimal fixtures with independently specified native facts."""
    value = json.loads(json.dumps(value))
    for outcome in value["outcomes"]:
        if not outcome.get("phase_results") or outcome["phase_results"] in ([{"phase": "Test"}], [{"phase": "Build"}]):
            outcome["phase_results"] = phase_records(outcome["summary"])
    summaries = [item["summary"] for item in value["outcomes"] if item.get("scenario") != "Baseline"]
    value.update(caught=summaries.count("CaughtMutant"), missed=summaries.count("MissedMutant"),
                 timeout=summaries.count("Timeout"), unviable=summaries.count("Unviable"), success=0)
    return value


class EvidenceTests(unittest.TestCase):
    """Literal healthy controls accompany wrong-file/empty/cause failures."""

    def test_mutation_json_policy_emits_flag_in_listing_and_campaign(self):
        """Core's JSON mode reaches both commands and exact command guards."""
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                      "mutation_jobs": 2, "mutation_message_format": "json"}
            calls = []
            def run(label, command, timeout, **options):
                calls.append((label, command))
                self.assertEqual(command.count("--cargo-arg=--message-format=json"), 1)
                self.assertLess(command.index("--cargo-arg=--message-format=json"),
                                command.index("--list" if label.startswith("mutation-enumeration:") else "--baseline"))
                mutant = {"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return json.dumps([mutant])
                self.assertEqual(options, {"mutation_subject": "era"})
                directory = work / "mutation/era/mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                def json_phases(summary):
                    phases = phase_records(summary)
                    for phase in phases:
                        phase["argv"].insert(phase["argv"].index("--frozen"), "--message-format=json")
                    return phases
                (directory / "outcomes.json").write_text(json.dumps(complete_report({
                    "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log",
                                  "phase_results": json_phases("Success")},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant",
                                  "phase_results": json_phases("CaughtMutant")}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy, work, tools, run, {}, "mutation")
            self.assertEqual([label for label, _ in calls], ["mutation-enumeration:era", "mutation-campaign:era"])
            command = calls[1][1]
            MODULE.mutation_campaign_command(SCOPE, tools, "era", command,
                                             expected_jobs=2, expected_message_format="json")
            result = {"exit_code": 2, "failure": None, "output": "survivor"}
            self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result,
                         expected_jobs=2, expected_message_format="json"), result)
            without_flag = [item for item in command if item != "--cargo-arg=--message-format=json"]
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(SCOPE, tools, "era", without_flag,
                                                 expected_jobs=2, expected_message_format="json")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result,
                                                expected_jobs=2, expected_message_format="human")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(SCOPE, tools, "era", command,
                                                 expected_jobs=2, expected_message_format=True)

    def test_invalid_mutation_message_format_stops_before_command(self):
        self.assertEqual(MODULE.mutation_message_format({}), "human")
        self.assertEqual(MODULE.mutation_message_format({"mutation_message_format": "json"}), "json")
        for value in ("", "JSON", "xml", None, True, 2):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); run = unittest.mock.Mock()
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                          "mutation_message_format": value}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                              work, work / "tools", run, {}, "mutation")
                run.assert_not_called()

    def test_mutation_phases_accept_exact_json_or_legacy_human_arguments(self):
        for summary in ("Success", "CaughtMutant", "Unviable"):
            human = {"summary": summary, "phase_results": phase_records(summary)}
            MODULE.mutation_phases(SCOPE, human)
            encoded = json.loads(json.dumps(human))
            for phase in encoded["phase_results"]:
                phase["argv"].insert(phase["argv"].index("--frozen"), "--message-format=json")
            MODULE.mutation_phases(SCOPE, encoded)
            corrupted = json.loads(json.dumps(encoded))
            corrupted["phase_results"][0]["argv"].insert(-1, "--message-format=human")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_phases(SCOPE, corrupted)
            if len(encoded["phase_results"]) == 2:
                mixed = json.loads(json.dumps(encoded))
                mixed["phase_results"][0]["argv"].remove("--message-format=json")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_phases(SCOPE, mixed)

    def test_mutation_policy_uses_two_workers_for_enumeration_and_campaign(self):
        """The selected policy controls both real command arguments and guard."""
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512, "mutation_jobs": 2}
            calls = []
            def run(label, command, timeout, **options):
                calls.append((label, command))
                self.assertEqual(command[command.index("--jobs") + 1], "2")
                self.assertEqual(command[command.index("--jobserver-tasks") + 1], "2")
                mutant = {"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return json.dumps([mutant])
                self.assertEqual(options, {"mutation_subject": "era"})
                directory = work / "mutation/era/mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                (directory / "outcomes.json").write_text(json.dumps(complete_report({
                    "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            report = {}
            MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy, work, tools, run, report, "mutation")
            self.assertEqual([label for label, _ in calls], ["mutation-enumeration:era", "mutation-campaign:era"])
            command = calls[1][1]
            MODULE.mutation_campaign_command(SCOPE, tools, "era", command, expected_jobs=2)
            result = {"exit_code": 2, "failure": None, "output": "survivor"}
            self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result,
                                                          expected_jobs=2), result)
            for index in (command.index("--jobs") + 1, command.index("--jobserver-tasks") + 1):
                changed = list(command); changed[index] = "12"
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_campaign_command(SCOPE, tools, "era", changed, expected_jobs=2)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_campaign_result(SCOPE, tools, "era", changed, result, expected_jobs=2)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result, expected_jobs=12)
            for invalid in (True, 0, 13, "2"):
                with self.subTest(invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_campaign_command(SCOPE, tools, "era", command, expected_jobs=invalid)

    def test_invalid_mutation_worker_counts_stop_before_command(self):
        self.assertEqual(MODULE.mutation_jobs({}), 12)
        self.assertEqual(MODULE.mutation_jobs({"mutation_jobs": 1}), 1)
        self.assertEqual(MODULE.mutation_jobs({"mutation_jobs": 12}), 12)
        for value in (True, False, 0, -1, 13, "2", 2.0, None):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); run = unittest.mock.Mock()
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512, "mutation_jobs": value}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                              work, work / "tools", run, {}, "mutation")
                run.assert_not_called()

    def test_mutation_process_profiles_are_fixed_and_invalid_values_stop_before_command(self):
        self.assertEqual(MODULE.mutation_process_limit({}), 128)
        self.assertEqual(MODULE.mutation_process_limit({"mutation_process_limit": 128}), 128)
        self.assertEqual(MODULE.mutation_process_limit({"mutation_process_limit": 512}), 512)
        for value in (True, False, None, "512", 512.0, 0, -1, 127, 129, 256, 513, 1024):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                run = unittest.mock.Mock()
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                          "mutation_process_limit": value}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                              work, work / "tools", run, {}, "mutation")
                run.assert_not_called()

    def test_mutation_campaign_selects_worker_target_isolation(self):
        """The guarded campaign must reach the runtime's worker-owned target mode."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.check = lambda: None
            session.root = root
            session.work = root / "work"
            session.output = root
            session.base = root / "base"
            session.stable = root / "stable"
            session.vendor = root / "vendor"
            session.tools = root / "tools"
            session.source = root / "source"
            session.config = root / "config"
            session.protected_islands = {}
            session.admitted = {"policy": {"common_nightly": "night"}}
            session.report = {"lanes": []}
            session.scope = SCOPE
            session.policy = {"mutation_jobs": 2, "mutation_message_format": "json"}
            result = {"exit_code": 0, "failure": None, "output": "owned campaign", "command": ["fixture"]}
            with (patch.object(MODULE, "mutation_campaign_command") as admission,
                  patch.object(MODULE, "mutation_campaign_result", return_value="checked outcome") as checked_result,
                  patch.object(MODULE.RUNTIME, "run", return_value=result) as run):
                self.assertEqual(session.run("mutation-campaign:clock", ["fixture"], 1,
                                             mutation_subject="clock"), "checked outcome")
                admission.assert_called_once_with(SCOPE, session.tools, "clock", ["fixture"],
                                                  expected_jobs=2, expected_message_format="json")
                checked_result.assert_called_once_with(SCOPE, session.tools, "clock", ["fixture"], result,
                                                       expected_jobs=2, expected_message_format="json")
                self.assertEqual(run.call_args.kwargs["cargo_target_mode"], "worker")
                self.assertEqual(run.call_args.kwargs["process_limit"], 128)
                self.assertEqual(run.call_args.kwargs["build_jobs"], 1)
                self.assertEqual(run.call_args.kwargs["environment"]["RUST_TEST_THREADS"], "1")
            session.policy["mutation_process_limit"] = 512
            with (patch.object(MODULE, "mutation_campaign_command") as admission,
                  patch.object(MODULE, "mutation_campaign_result", return_value="checked outcome"),
                  patch.object(MODULE.RUNTIME, "run", return_value=result) as run):
                self.assertEqual(session.run("mutation-campaign:clock", ["fixture"], 1,
                                             mutation_subject="clock"), "checked outcome")
                admission.assert_called_once_with(SCOPE, session.tools, "clock", ["fixture"],
                                                  expected_jobs=2, expected_message_format="json")
                self.assertEqual(run.call_args.kwargs["process_limit"], 512)
                self.assertEqual(run.call_args.kwargs["cargo_target_mode"], "worker")
                self.assertEqual(run.call_args.kwargs["build_jobs"], 1)
            with patch.object(MODULE.RUNTIME, "run", return_value=result) as run:
                self.assertEqual(session.run("ordinary-control", ["fixture"], 1), "owned campaign")
                self.assertNotIn("cargo_target_mode", run.call_args.kwargs)
                self.assertNotIn("process_limit", run.call_args.kwargs)
                self.assertEqual(session.run("sanitizer-control", ["fixture"], 1, mode="sanitizer"),
                                 "owned campaign")
                self.assertNotIn("process_limit", run.call_args.kwargs)
            with (patch.object(MODULE, "mutation_campaign_command",
                               side_effect=MODULE.Incomplete("substituted campaign")),
                  patch.object(MODULE.RUNTIME, "run") as run):
                with self.assertRaisesRegex(MODULE.Incomplete, "substituted campaign"):
                    session.run("mutation-campaign:clock", ["fixture"], 1, mutation_subject="clock")
                run.assert_not_called()
            with patch.object(MODULE.RUNTIME, "run") as run:
                for options in ({"mode": "sanitizer"}, {"negative": "borrowed failure"},
                                {"sanitizer_profile": "time-core-history-s12d"}):
                    with self.subTest(options=options), self.assertRaises(MODULE.Incomplete):
                        session.run("mutation-campaign:clock", ["fixture"], 1,
                                    mutation_subject="clock", **options)
                run.assert_not_called()

    def test_legal_consumer_preserves_complete_counter_transport(self):
        """A successful child must not lose the final strict counter delimiter."""
        raw = "legal-core-contract\nCORE_RUNS_PROFILE points=3 inputs=1 selections=2\n"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.check = lambda: None
            session.root = root
            session.work = root / "work"
            session.output = root
            session.base = root / "base"
            session.stable = root / "stable"
            session.vendor = root / "vendor"
            session.tools = root / "tools"
            session.source = root / "source"
            session.config = root / "config"
            session.protected_islands = {}
            session.admitted = {"policy": {"common_nightly": "night"}}
            session.report = {"lanes": []}
            success = {"exit_code": 0, "failure": None, "output": raw, "command": ["fixture"]}
            with patch.object(MODULE.RUNTIME, "run", return_value=success):
                self.assertEqual(session.run("external-legal-execution", ["fixture"], 1), raw)
                self.assertEqual(session.run("ordinary-control", ["fixture"], 1), raw.strip())
            failure = {"exit_code": 1, "failure": None, "output": raw, "command": ["fixture"]}
            with patch.object(MODULE.RUNTIME, "run", return_value=failure):
                with self.assertRaises(MODULE.PRODUCT.ENV.Incomplete):
                    session.run("external-legal-execution", ["fixture"], 1)

    def test_mutation_listing_admits_only_exact_warning_and_complete_strict_json(self):
        warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        listing = '[{"name":"one","file":"selected","package":"fixture"}]'
        expected = [{"name": "one", "file": "selected", "package": "fixture"}]
        self.assertEqual(MODULE.parse_mutation_listing(listing), (expected, None))
        self.assertEqual(MODULE.parse_mutation_listing(warning + listing + "\n"), (expected, warning))
        warning_ten = (" WARN --jobs=10 is probably too high and may overload your machine: each job runs a separate "
                       "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                       "are usually safe\n")
        self.assertEqual(MODULE.parse_mutation_listing(warning_ten + listing, expected_jobs=10),
                         (expected, warning_ten))
        self.assertEqual(MODULE.parse_mutation_listing(listing, expected_jobs=2), (expected, None))
        for output, workers in ((warning, 10), (warning_ten, 12), (warning_ten, 2), (warning, 8)):
            with self.subTest(workers=workers), self.assertRaises(MODULE.Incomplete):
                MODULE.parse_mutation_listing(output + listing, expected_jobs=workers)
        for workers in (True, False, 0, 13, 10.0, "10", None):
            with self.subTest(workers=workers), self.assertRaises(MODULE.Incomplete):
                MODULE.parse_mutation_listing(listing, expected_jobs=workers)
        invalid = [
            "unexpected diagnostic\n" + listing,
            " " + warning + listing,
            warning.replace("--jobs=12", "--jobs=13") + listing,
            warning.replace(" WARN", "WARN") + listing,
            warning.rstrip("\n") + listing,
            warning + warning + listing,
            warning + "warning: another message\n" + listing,
            warning + listing + "\ntrailing diagnostic",
            listing + " []",
            listing[:-1],
            '{"name":"not a list"}',
            '[{"name":"first","name":"second"}]',
            '[{"value":NaN}]',
            '[{"value":Infinity}]',
            '[{"value":1e999}]',
            "",
            None,
        ]
        for output in invalid:
            with self.subTest(output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.parse_mutation_listing(output)

    def test_ten_worker_listing_retains_native_warning_and_reaches_campaign(self):
        warning = (" WARN --jobs=10 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            report = {}
            calls = []
            listed = warning + json.dumps([{"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}])
            def run(label, command, _timeout, **options):
                calls.append(label)
                self.assertEqual(command[command.index("--jobs") + 1], "10")
                if label == "mutation-enumeration:era":
                    return listed
                self.assertEqual(label, "mutation-campaign:era")
                self.assertEqual(options, {"mutation_subject": "era"})
                raise RuntimeError("controlled stop after admitted ten-worker listing")
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                      "mutation_jobs": 10, "mutation_process_limit": 512}
            with self.assertRaisesRegex(RuntimeError, "controlled stop after admitted ten-worker listing"):
                MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                          work, work / "tools", run, report, "mutation")
            self.assertEqual(calls, ["mutation-enumeration:era", "mutation-campaign:era"])
            self.assertEqual(report["mutation_enumeration"]["era"], {"raw_output": listed, "warning": warning})

    def test_failed_mutation_listing_retains_raw_warning_and_starts_no_campaign(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                       "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                       "are usually safe\n")
            raw = warning + "["
            calls, report = [], {}
            def run(label, _command, _timeout, **_options):
                calls.append(label)
                return raw
            with self.assertRaises(MODULE.Incomplete):
                MODULE.run_analysis_lanes(SCOPE, dict(FILES[:1]),
                    {"mutation_wall_seconds": 1200, "maximum_mutants": 512},
                    work, tools, run, report, "mutation")
            self.assertEqual(calls, ["mutation-enumeration:coordinate"])
            self.assertEqual(report["mutation_enumeration"]["coordinate"],
                             {"raw_output": raw, "warning": warning})

    def test_inventory_only_lists_exact_owners_without_baselines_or_campaigns(self):
        """A larger discovery reservation reports, but never admits, a campaign."""
        subjects = {"era": dict(FILES)["era"], "clock": dict(FILES)["clock"]}
        mutants = {name: {"file": filename, "name": name, "package": "time-domain"}
                   for name, filename in subjects.items()}
        warning = (" WARN --jobs=10 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        report, calls = {}, []
        tools = Path("/sealed/tools")
        def run(label, command, timeout, **options):
            calls.append(label)
            name = label.removeprefix("mutation-enumeration:")
            self.assertIn(name, subjects)
            self.assertEqual(command, [str(tools / "cargo-mutants"), "mutants", "--package", "time-domain",
                "--file", subjects[name], "--test-workspace", "false", "--cap-lints", "false", "--jobs", "10",
                "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json",
                "--list", "--json"])
            self.assertEqual(options, {})
            self.assertGreater(timeout, 0)
            self.assertLessEqual(timeout, 120)
            return (warning if name == "era" else "") + json.dumps([mutants[name]])
        policy = {"maximum_mutants": 1, "mutation_jobs": 10,
                  "mutation_process_limit": 512, "mutation_message_format": "json"}
        found = MODULE.enumerate_mutation_inventories(SCOPE, subjects, policy, tools, run, report,
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=2, maximum_total=4)
        self.assertEqual(calls, ["mutation-enumeration:era", "mutation-enumeration:clock"])
        self.assertEqual(found.inventories, {name: {name: mutant} for name, mutant in mutants.items()})
        self.assertEqual(found.total, 2)
        self.assertTrue(found.campaign_budget_exceeded)
        self.assertEqual(report["mutation_enumerated_total"], 2)
        self.assertTrue(report["mutation_campaign_budget_exceeded"])
        self.assertEqual(found.observations, report["mutation_enumeration"])
        self.assertEqual(found.observations["era"], {"raw_output": warning + json.dumps([mutants["era"]]),
                                                  "warning": warning})
        self.assertIsNone(found.observations["clock"]["warning"])
        self.assertTrue(all("--baseline" not in command and "--output" not in command
                            for command in found.commands.values()))
        self.assertNotIn("mutation", report)
        self.assertNotIn("mutation_reviewed_inventories", report)

    def test_inventory_only_requires_exact_scope_and_explicit_finite_allowances(self):
        subjects = {"era": dict(FILES)["era"]}
        policy = {"maximum_mutants": 2}
        valid = {"deadline": MODULE.time.monotonic() + 60, "maximum_per_file": 2, "maximum_total": 2}
        for field, values in {
            "maximum_per_file": (True, False, None, "2", 2.0, 0, -1, 513),
            "maximum_total": (True, None, "2", 2.0, 0, -1, 3),
            "deadline": (True, None, "later", float("inf"), float("nan"), 10**500, 0),
        }.items():
            for value in values:
                with self.subTest(field=field, value=value):
                    run = unittest.mock.Mock()
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.enumerate_mutation_inventories(SCOPE, subjects, policy, Path("/tools"), run, {},
                                                              **{**valid, field: value})
                    run.assert_not_called()
        for selected in ({}, {"unknown": dict(FILES)["era"]}, {"unknown": None},
                         {"era": dict(FILES)["clock"]}):
            with self.subTest(subjects=selected):
                run = unittest.mock.Mock()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.enumerate_mutation_inventories(SCOPE, selected, policy, Path("/tools"), run, {}, **valid)
                run.assert_not_called()
        for value in (True, None, "2", 2.0, 0, -1, 513):
            with self.subTest(campaign_cap=value):
                run = unittest.mock.Mock()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": value},
                                                          Path("/tools"), run, {}, **valid)
                run.assert_not_called()

    def test_inventory_only_retains_rejected_native_listings(self):
        mutant = {"name": "one", "file": dict(FILES)["era"], "package": "time-domain"}
        warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        invalid = [("[", 1), ("{}", 1), ("[]", 1), (json.dumps([mutant, mutant]), 2),
                   (json.dumps([{**mutant, "file": dict(FILES)["clock"]}]), 1),
                   (json.dumps([{**mutant, "package": "other"}]), 1),
                   (json.dumps([mutant, {**mutant, "name": "two"}]), 1),
                   (warning + json.dumps([mutant]), 1)]
        for raw, allowance in invalid:
            with self.subTest(raw=raw):
                report = {"mutation_enumerated_total": 1, "mutation_campaign_budget_exceeded": False}
                run = unittest.mock.Mock(return_value=raw)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.enumerate_mutation_inventories(SCOPE, {"era": dict(FILES)["era"]},
                        {"maximum_mutants": 512, "mutation_jobs": 10}, Path("/tools"), run, report,
                        deadline=MODULE.time.monotonic() + 60,
                        maximum_per_file=allowance, maximum_total=allowance)
                self.assertEqual(run.call_count, 1)
                self.assertEqual(run.call_args.args[0], "mutation-enumeration:era")
                self.assertEqual(run.call_args.args[1][-2:], ["--list", "--json"])
                self.assertEqual(report["mutation_enumeration"]["era"]["raw_output"], raw)
                self.assertNotIn("mutation", report)
                self.assertNotIn("mutation_enumerated_total", report)
                self.assertNotIn("mutation_campaign_budget_exceeded", report)

    def test_inventory_only_stops_on_its_separate_aggregate_listing_allowance(self):
        subjects = dict(FILES[:2])
        calls, report = [], {}
        def run(label, command, _timeout):
            calls.append(label)
            name = label.split(":")[1]
            self.assertEqual(command[-2:], ["--list", "--json"])
            return json.dumps([{"name": name, "file": subjects[name], "package": "time-domain"}])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": 512},
                Path("/tools"), run, report, deadline=MODULE.time.monotonic() + 60,
                maximum_per_file=1, maximum_total=1)
        self.assertEqual(calls, ["mutation-enumeration:coordinate", "mutation-enumeration:era"])
        self.assertEqual(set(report["mutation_enumeration"]), set(subjects))
        self.assertNotIn("mutation", report)

    def test_inventory_only_consumes_one_unchanged_deadline(self):
        subjects = dict(FILES[:2])
        for clocks, expected_timeouts, exhausted in [([90.25, 98.1], [9, 1], False),
                                                      ([90.25, 100.1], [9], True)]:
            with self.subTest(clocks=clocks):
                timeouts, calls, report = [], [], {}
                def run(label, command, timeout):
                    calls.append(label)
                    timeouts.append(timeout)
                    self.assertEqual(command[-2:], ["--list", "--json"])
                    name = label.split(":")[1]
                    return json.dumps([{"name": name, "file": subjects[name], "package": "time-domain"}])
                with patch.object(MODULE.time, "monotonic", side_effect=clocks):
                    if exhausted:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": 2},
                                Path("/tools"), run, report, deadline=100.0, maximum_per_file=1, maximum_total=2)
                    else:
                        found = MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": 2},
                            Path("/tools"), run, report, deadline=100.0, maximum_per_file=1, maximum_total=2)
                        self.assertEqual(found.total, 2)
                        self.assertFalse(found.campaign_budget_exceeded)
                self.assertEqual(timeouts, expected_timeouts)
                self.assertEqual(calls, ["mutation-enumeration:" + name
                                         for name in list(subjects)[:len(expected_timeouts)]])
                if exhausted:
                    self.assertNotIn("mutation_enumerated_total", report)

    def test_campaign_lanes_keep_the_aggregate_gate_before_every_campaign(self):
        subjects = dict(FILES[:2])
        for lane in ("mutation", "all"):
            with self.subTest(lane=lane), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                calls, report = [], {}
                def run(label, command, _timeout, **_options):
                    calls.append(label)
                    self.assertNotIn("--baseline", command)
                    if label == "coverage":
                        (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                            {"filename": "/work/source/" + filename,
                             "summary": {"lines": {"count": 10, "covered": 9}}}
                            for filename in subjects.values()]}]}))
                        return "test result: ok. 1 passed; 0 failed; 0 ignored;"
                    name = label.removeprefix("mutation-enumeration:")
                    self.assertIn(name, subjects)
                    return json.dumps([{"name": name, "file": subjects[name], "package": "time-domain"}])
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, subjects,
                        {"coverage_wall_seconds": 300, "mutation_wall_seconds": 1200, "maximum_mutants": 1},
                        work, work / "tools", run, report, lane)
                self.assertEqual(calls, (["coverage"] if lane == "all" else [])
                                 + ["mutation-enumeration:coordinate", "mutation-enumeration:era"])
                self.assertNotIn("mutation", report)

    def test_scope_is_immutable(self):
        with self.assertRaises(AttributeError):
            SCOPE.package = "other"

    def test_each_file_requires_its_actual_nonempty_line_denominator(self):
        for _, filename in FILES:
            full = "/work/source/" + filename
            report = {"data": [{"files": [{"filename": full, "summary": {
                "lines": {"count": 10, "covered": 9}}}]}]}
            self.assertEqual(MODULE.coverage_counts(SCOPE, report, full)["line_percent"], 90)
            for count, covered in [(0, 0), (10, 8), (True, 1), (10, 11)]:
                bad = {"data": [{"files": [{"filename": full, "summary": {
                    "lines": {"count": count, "covered": covered}}}]}]}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.coverage_counts(SCOPE, bad, full)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.coverage_counts(SCOPE, report, "/work/source/crates/other.rs")

    def test_viable_mutants_require_actual_tests_and_exact_full_inventory(self):
        filename = FILES[0][1]
        items = [{"name": f"fault-{index}", "file": filename, "package": "time-domain"}
                 for index in range(2)]
        frozen = MODULE.mutation_inventory(SCOPE, items, 512, filename)
        report = {"cargo_mutants_version": "27.1.0", "total_mutants": 2, "outcomes": [
            {"scenario": "Baseline", "summary": "Success"},
            {"scenario": {"Mutant": items[0]}, "summary": "CaughtMutant", "phase_results": [{"phase": "Test"}]},
            {"scenario": {"Mutant": items[1]}, "summary": "Unviable"}]}
        report = complete_report(report)
        actual = MODULE.mutation_counts(SCOPE, report, frozen, filename)
        self.assertEqual((actual["viable"], actual["killed"], actual["unviable"]), (1, 1, 1))
        for replacement in ["MissedMutant", "Timeout"]:
            bad = json.loads(json.dumps(report))
            bad["outcomes"][2].update(summary=replacement, phase_results=[{"phase": "Test"}])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, bad, frozen, filename)
        for bad_items in [[], [items[0], items[0]], [{**items[0], "package": "other"}]]:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_inventory(SCOPE, bad_items, 512, filename)
        bad = json.loads(json.dumps(report)); bad["outcomes"].pop()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, bad, frozen, filename)

    def test_reviewed_inventories_require_full_scope_shapes_and_exact_normalized_hash(self):
        filename = FILES[0][1]
        first = {"name": "one", "file": filename, "package": "time-domain",
                 "replacement": "x", "nested": {"diff": "retained"}}
        second = {"name": "two", "file": filename, "package": "time-domain", "replacement": "y"}
        normalized = {"one": first, "two": second}
        digest = hashlib.sha256(json.dumps(normalized, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        specs = {name: {"count": 1, "sha256": "0" * 64} for name, _ in FILES}
        specs["coordinate"] = {"count": 2, "sha256": digest}
        selected = {"coordinate": {"two": {**second, "diff": "rendered"}, "one": first}}
        policy = {"maximum_mutants": 512, "mutation_inventories": specs}
        admitted = MODULE.admit_reviewed_mutation_inventories(SCOPE, policy, selected)
        self.assertEqual(admitted, {"coordinate": {"count": 2, "sha256": digest}})
        self.assertIsNone(MODULE.admit_reviewed_mutation_inventories(SCOPE, {"maximum_mutants": 512}, selected))
        altered = {"coordinate": {"one": {**first, "nested": {"diff": "changed"}}, "two": second}}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_reviewed_mutation_inventories(SCOPE, policy, altered)
        for replacement in (None, {}, {**specs, "extra": specs["era"]},
                            {name: spec for name, spec in specs.items() if name != "era"}):
            with self.subTest(mapping=replacement), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reviewed_mutation_inventories(
                    SCOPE, {"maximum_mutants": 512, "mutation_inventories": replacement}, selected)
        for field, value in (("count", True), ("count", 0), ("count", 513), ("count", 3),
                             ("sha256", "f" * 63), ("sha256", "A" * 64), ("sha256", "0" * 64)):
            bad = {name: dict(spec) for name, spec in specs.items()}
            bad["coordinate"][field] = value
            with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reviewed_mutation_inventories(
                    SCOPE, {"maximum_mutants": 512, "mutation_inventories": bad}, selected)
        for spec in ({"count": 1}, {"count": 1, "sha256": "0" * 64, "extra": 1}):
            bad = {name: dict(item) for name, item in specs.items()}; bad["era"] = spec
            with self.subTest(unselected=spec), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reviewed_mutation_inventories(
                    SCOPE, {"maximum_mutants": 512, "mutation_inventories": bad}, selected)

    def test_reviewed_inventory_mismatch_stops_all_campaigns_after_all_listings(self):
        subjects = dict(FILES[:2])
        mutants = {name: {"name": name, "file": filename, "package": "time-domain", "replacement": "x"}
                   for name, filename in subjects.items()}
        def digest_for(mutant):
            payload = json.dumps({mutant["name"]: mutant}, sort_keys=True, separators=(",", ":")).encode()
            return hashlib.sha256(payload).hexdigest()
        specs = {name: {"count": 1, "sha256": "0" * 64} for name, _ in FILES}
        for name, mutant in mutants.items():
            specs[name] = {"count": 1, "sha256": digest_for(mutant)}
        for mismatch in (False, True):
            with self.subTest(mismatch=mismatch), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); tools = work / "tools"; tools.mkdir()
                reviewed = {name: dict(spec) for name, spec in specs.items()}
                if mismatch:
                    reviewed["era"]["sha256"] = "0" * 64
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                          "mutation_inventories": reviewed}
                calls, report = [], {}
                def run(label, _command, _timeout, **_options):
                    calls.append(label)
                    name = label.split(":")[1]
                    mutant = mutants[name]
                    if label.startswith("mutation-enumeration:"):
                        return json.dumps([{**mutant, "diff": "ignored rendered diff"}])
                    directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                    (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                    (directory / "outcomes.json").write_text(json.dumps(complete_report({
                        "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                        "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                     {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})))
                    return {"exit_code": 0, "failure": None, "output": "completed"}
                if mismatch:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
                    self.assertEqual(calls, ["mutation-enumeration:coordinate", "mutation-enumeration:era"])
                else:
                    MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
                    self.assertEqual(calls, ["mutation-enumeration:coordinate", "mutation-enumeration:era",
                                             "mutation-campaign:coordinate", "mutation-campaign:era"])
                    self.assertEqual(report["mutation_reviewed_inventories"],
                                     {name: specs[name] for name in subjects})

    def test_caught_summary_cannot_borrow_a_successful_test_result(self):
        filename = FILES[0][1]
        mutant = {"name": "phase-status", "file": filename, "package": "time-domain"}
        phases = phase_records()
        report = {"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                  "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases},
                               {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": phases}]}
        report = complete_report(report)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, report, {mutant["name"]: mutant}, filename)

    def test_complete_missed_file_does_not_hide_later_campaign(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            subjects = dict(FILES[:2]); report = {}; observed = []
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512}
            def run(label, command, timeout, **options):
                name = label.split(":")[1]; filename = subjects[name]
                mutant = {"name": name, "file": filename, "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return json.dumps([mutant])
                observed.append(name)
                phases = phase_records()
                summary = "MissedMutant" if name == "coordinate" else "CaughtMutant"
                actual = phase_records(summary)
                directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                (directory / "outcomes.json").write_text(json.dumps(complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases, "log_path": "baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": summary, "phase_results": actual}]})))
                return {"exit_code": 2 if summary == "MissedMutant" else 0, "failure": None, "output": "observed"}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
            self.assertEqual(observed, ["coordinate", "era"])
            self.assertEqual(set(report["mutation"]), set(subjects))

    def test_equivalence_admission_binds_source_candidate_proof_and_exact_review_section(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); filename = FILES[1][1]
            source = root / filename; source.parent.mkdir(parents=True); source.write_text("frozen numerical owner\n")
            proof = root / "tasks/proof.md"; proof.parent.mkdir(); proof.write_text("## TD-E-001 — proof\nIndependent rationale.\n")
            digest = lambda data: hashlib.sha256(data).hexdigest()
            identity = {filename: digest(source.read_bytes()), "other-construction-owner": "a" * 64}
            candidate = digest(json.dumps(identity, sort_keys=True, separators=(",", ":")).encode())
            mutant = {"name": "complete-era-fault", "file": filename, "package": "time-domain",
                      "function": {"function_name": "resolve", "return_type": "-> Result<_, _>",
                                   "span": {"start": {"line": 1, "column": 1}, "end": {"line": 5, "column": 2}}},
                      "span": {"start": {"line": 3, "column": 1}, "end": {"line": 3, "column": 2}},
                      "replacement": "^", "genre": "BinaryOperator"}
            entry = {"id": "TD-E-001", "candidate_sha256": candidate, "source_sha256": identity[filename],
                     "mutant": mutant, "requirement": "CHECK-6/7/25", "rationale": "Disjoint bit fields preserve every input.",
                     "proof": {"path": "tasks/proof.md", "sha256": digest(proof.read_bytes()), "section": "TD-E-001",
                               "reviewer": "independent-reviewer", "verdict": "independently-justified-equivalent"}}
            policy = {"mutation_equivalences": [entry], "maximum_mutants": 512}
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.admit_mutation_equivalences(root, SCOPE, policy), [entry])
                for change in ("source", "candidate", "proof", "section", "review", "file", "identity", "duplicate", "unknown"):
                    bad = json.loads(json.dumps(policy)); case = bad["mutation_equivalences"][0]
                    if change == "source": case["source_sha256"] = "f" * 64
                    if change == "candidate": case["candidate_sha256"] = "f" * 64
                    if change == "proof": case["proof"]["sha256"] = "f" * 64
                    if change == "section": case["id"] = case["proof"]["section"] = "TD-E-002"
                    if change == "review": case["proof"]["verdict"] = "probably-safe"
                    if change == "file": case["mutant"]["file"] = "crates/other/src/era.rs"
                    if change == "identity": case["mutant"].pop("span")
                    if change == "duplicate": bad["mutation_equivalences"].append(dict(case))
                    if change == "unknown": case["skip_function"] = "resolve"
                    with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_mutation_equivalences(root, SCOPE, bad)
                source.write_text("changed numerical owner\n")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_mutation_equivalences(root, SCOPE, policy)
                source.write_text("frozen numerical owner\n")
                proof.write_text("## TD-E-001 — changed proof\n")
                with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                    MODULE.admit_mutation_equivalences(root, SCOPE, policy)
                proof.write_text("## TD-E-001 — proof\nIndependent rationale.\n")
                with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, {**identity, "other-construction-owner": "b" * 64})):
                    with self.assertRaisesRegex(MODULE.Incomplete, "complete candidate"):
                        MODULE.admit_mutation_equivalences(root, SCOPE, policy)
            self.assertEqual(MODULE.admit_mutation_equivalences(root, SCOPE, {}), [])

    def test_exact_unary_deletion_equivalence_preserves_empty_replacement(self):
        # Literal cargo-mutants 27.1.0 descriptor from the actual rounding campaign.
        mutant = {"name": "crates/time-domain/src/rounding.rs:59:26: delete - in round",
                  "package": "time-domain", "file": "crates/time-domain/src/rounding.rs",
                  "function": {"function_name": "round", "return_type": "-> Result<(i64, i128), DomainError>",
                               "span": {"start": {"line": 26, "column": 1}, "end": {"line": 64, "column": 2}}},
                  "span": {"start": {"line": 59, "column": 26}, "end": {"line": 59, "column": 27}},
                  "replacement": "", "genre": "UnaryOperator"}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); filename = mutant["file"]
            source = root / filename; source.parent.mkdir(parents=True); source.write_text("frozen rounding owner\n")
            proof = root / "tasks/proof.md"; proof.parent.mkdir(); proof.write_text("## TD-E-006 — proof\nIndependent rationale.\n")
            digest = lambda data: hashlib.sha256(data).hexdigest()
            identity = {filename: digest(source.read_bytes())}
            candidate = digest(json.dumps(identity, sort_keys=True, separators=(",", ":")).encode())
            entry = {"id": "TD-E-006", "candidate_sha256": candidate, "source_sha256": identity[filename],
                     "mutant": mutant, "requirement": "CHECK-6/7/25", "rationale": "Exact independently reviewed deletion.",
                     "proof": {"path": "tasks/proof.md", "sha256": digest(proof.read_bytes()), "section": "TD-E-006",
                               "reviewer": "independent-reviewer", "verdict": "independently-justified-equivalent"}}
            policy = {"mutation_equivalences": [entry], "maximum_mutants": 512}
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                admitted = MODULE.admit_mutation_equivalences(root, SCOPE, policy)
                self.assertEqual(admitted, [entry])
                self.assertEqual(admitted[0]["mutant"]["replacement"], "")
                for key, value in (("genre", "BinaryOperator"), ("genre", "FnValue"), ("genre", ""),
                                   ("genre", None), ("genre", False), ("replacement", None),
                                   ("replacement", False), ("replacement", []),
                                   ("file", "crates/other/src/rounding.rs")):
                    bad = json.loads(json.dumps(policy)); bad["mutation_equivalences"][0]["mutant"][key] = value
                    with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_mutation_equivalences(root, SCOPE, bad)
            caught = {**mutant, "name": "caught rounding neighbor", "replacement": "-"}
            report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 2,
                "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                             {"scenario": {"Mutant": caught}, "summary": "CaughtMutant"},
                             {"scenario": {"Mutant": mutant}, "summary": "MissedMutant"}]})
            frozen = MODULE.mutation_inventory(SCOPE, [caught, mutant], 512, filename)
            result = MODULE.mutation_counts(SCOPE, report, frozen, filename,
                                            native_exit_code=2, equivalences=admitted)
            self.assertTrue(result["gate_passed"])
            self.assertEqual(result["excluded_cases"][0]["mutant"], mutant)
            for key, value in (("replacement", "-"), ("genre", "BinaryOperator")):
                bad = json.loads(json.dumps(admitted)); bad[0]["mutant"][key] = value
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, equivalences=bad)

    def test_exact_missed_equivalences_keep_raw_and_adjusted_denominators_distinct(self):
        filename = FILES[1][1]
        mutants = [{"name": f"era-{index}", "file": filename, "package": "time-domain", "replacement": str(index)}
                   for index in range(25)]
        outcomes = [{"scenario": "Baseline", "summary": "Success"}]
        for index, mutant in enumerate(mutants):
            summary = "CaughtMutant" if index < 17 else "MissedMutant" if index < 19 else "Unviable"
            outcomes.append({"scenario": {"Mutant": mutant}, "summary": summary})
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 25, "outcomes": outcomes})
        frozen = MODULE.mutation_inventory(SCOPE, mutants, 512, filename)
        equivalents = [{"id": f"TD-E-00{index - 16}", "mutant": mutants[index]} for index in (17, 18)]
        actual = MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, equivalences=equivalents)
        self.assertEqual((actual["generated"], actual["killed"], actual["survived"], actual["unviable"], actual["excluded"]),
                         (25, 17, 2, 6, 2))
        self.assertEqual((actual["raw_denominator"], actual["adjusted_denominator"], actual["native_exit_code"]), (19, 17, 2))
        self.assertAlmostEqual(actual["raw_viable_killed_percent"], 89.47368421052632)
        self.assertEqual(actual["adjusted_viable_killed_percent"], 100)
        self.assertTrue(actual["gate_passed"])
        self.assertEqual(actual["excluded_cases"], equivalents)
        for bad in (equivalents[:1], equivalents + [equivalents[0]],
                    [{"mutant": {**mutants[17], "replacement": "changed"}}], [{"mutant": mutants[0]}],
                    [{"mutant": {**mutants[17], "file": FILES[0][1]}}]):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, equivalences=bad)
        unresolved = MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, defer_gate=True)
        self.assertFalse(unresolved["gate_passed"])
        self.assertEqual(unresolved["unresolved_survivors"], ["era-17", "era-18"])

    def test_passing_raw_percentage_does_not_discharge_an_unknown_survivor(self):
        filename = FILES[0][1]
        mutants = [{"name": f"fault-{index}", "file": filename, "package": "time-domain"} for index in range(21)]
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 21,
            "outcomes": [{"scenario": "Baseline", "summary": "Success"}] + [
                {"scenario": {"Mutant": mutant}, "summary": "MissedMutant" if index == 20 else "CaughtMutant"}
                for index, mutant in enumerate(mutants)]})
        frozen = MODULE.mutation_inventory(SCOPE, mutants, 512, filename)
        result = MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, defer_gate=True)
        self.assertGreater(result["raw_viable_killed_percent"], 95)
        self.assertFalse(result["gate_passed"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2)

    def test_native_exit_counter_phase_baseline_and_inventory_mismatches_reject(self):
        filename = FILES[0][1]; mutant = {"name": "one", "file": filename, "package": "time-domain"}
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                                 "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                                              {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})
        frozen = {"one": mutant}
        self.assertTrue(MODULE.mutation_counts(SCOPE, report, frozen, filename)["gate_passed"])
        for change in ("counter", "missing", "duplicate", "baseline", "test_status", "no_test", "test_args", "duration", "timeout", "bool_total"):
            bad = json.loads(json.dumps(report))
            if change == "counter": bad["caught"] = 0
            if change == "missing": bad["outcomes"].pop()
            if change == "duplicate": bad["outcomes"].append(bad["outcomes"][1])
            if change == "baseline": bad["outcomes"][0]["phase_results"][1]["process_status"] = {"Failure": 101}
            if change == "test_status": bad["outcomes"][1]["phase_results"][1]["process_status"] = "Success"
            if change == "no_test": bad["outcomes"][1]["phase_results"].pop()
            if change == "test_args": bad["outcomes"][1]["phase_results"][1]["argv"].append("--ignored")
            if change == "duration": bad["outcomes"][1]["phase_results"][1]["duration"] = float("nan")
            if change == "timeout": bad["outcomes"][1]["summary"] = "Timeout"
            if change == "bool_total": bad["total_mutants"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, bad, frozen, filename)
        for native in (2, 3, 4, 70, -9, None, True):
            with self.subTest(native=native), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=native)
        missed = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                                  "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                                               {"scenario": {"Mutant": mutant}, "summary": "MissedMutant"}]})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, missed, frozen, filename, native_exit_code=0, defer_gate=True)

    def test_only_the_exact_campaign_may_retain_native_exit_two(self):
        tools = Path("/admitted/tools"); filename = FILES[1][1]
        command = ["/admitted/tools/cargo-mutants", "mutants", "--package", "time-domain", "--file", filename,
                   "--test-workspace", "false", "--cap-lints", "false", "--jobs", "12",
                   "--jobserver", "true", "--jobserver-tasks", "12", "--baseline", "run",
                   "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/era"]
        result = {"exit_code": 2, "failure": None, "output": "two independently visible survivors"}
        self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result), result)
        for changed in (["/other/tool", *command[1:]], [*command, "--skip-calls-defaults"],
                        [word.replace(filename, FILES[0][1]) for word in command],
                        [word.replace("/work/mutation/era", "/work/other") for word in command],
                        [word if index != command.index("--jobs") + 1 else "13"
                         for index, word in enumerate(command)],
                        [word if index != command.index("--jobserver-tasks") + 1 else "13"
                         for index, word in enumerate(command)],
                        [word if index != command.index("--jobserver") + 1 else "false"
                         for index, word in enumerate(command)]):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", changed, result)
        for changed in ({**result, "exit_code": code} for code in (1, 3, 4, 70, -9, None, True)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, changed)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_campaign_result(SCOPE, tools, "era", command, {**result, "failure": "deadline"})
        session = MODULE.Session.__new__(MODULE.Session)
        session.scope, session.tools = SCOPE, tools
        session.policy = {}
        with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run") as runtime:
            with self.assertRaises(MODULE.Incomplete):
                session.run("mutation-campaign:era", ["/other/tool"], 30, mutation_subject="era")
            runtime.assert_not_called()

    def test_artifact_must_be_the_real_library_of_the_exact_manifest(self):
        manifest = "/work/source/crates/time-domain/Cargo.toml"
        artifact = {"reason": "compiler-artifact", "package_id": "path+file:///work/source/crates/time-domain#0.0.0",
                    "manifest_path": manifest, "target": {"name": "time_domain", "kind": ["lib"]},
                    "filenames": ["/work/target/debug/libtime_domain.rlib"]}
        self.assertEqual(MODULE.library_artifact(json.dumps(artifact), "time-domain", manifest),
                         "/work/target/debug/libtime_domain.rlib")
        for patch in [{"manifest_path": "/unrelated/Cargo.toml"},
                      {"package_id": "registry+https://example#time-domain@0.0.0"},
                      {"filenames": ["/work/target/../stolen.rlib"]},
                      {"target": {"name": "time_domain", "kind": ["test"]}}]:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.library_artifact(json.dumps({**artifact, **patch}), "time-domain", manifest)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.library_artifact(json.dumps(artifact) + "\n" + json.dumps(artifact), "time-domain", manifest)

    def test_compiler_error_requires_exact_code_primary_span_and_type(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        diagnostic = {"$message_type": "diagnostic", "level": "error", "code": {"code": "E0308"},
                      "message": "expected MonotonicNs, found ActiveAwakeNs", "children": [],
                      "spans": [{"file_name": path, "line_start": 3, "is_primary": True}]}
        result = {"exit_code": 1, "failure": None, "output": json.dumps(diagnostic)}
        MODULE.compiler_cause(result, specimen, path)
        for change in [{"code": {"code": "E0432"}}, {"message": "unrelated expected u8, found u16"},
                       {"spans": [{"file_name": path, "line_start": 2, "is_primary": True}]}]:
            bad = {**result, "output": json.dumps({**diagnostic, **change})}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.compiler_cause(bad, specimen, path)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.compiler_cause({**result, "exit_code": 0}, specimen, path)

    def test_cause_tokens_can_come_from_real_rustc_span_labels(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 4}
        diagnostic = {"level": "error", "code": {"code": "E0308"}, "message": "mismatched types",
                      "spans": [{"file_name": path, "line_start": 4, "is_primary": True,
                                 "label": "expected `MonotonicNs`, found `ActiveAwakeNs`"}]}
        MODULE.compiler_cause({"exit_code": 1, "failure": None, "output": json.dumps(diagnostic)}, specimen, path)

    def test_session_rechecks_new_copied_inputs_config_and_dependency_changes(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp); root = base / "checkout"; root.mkdir()
            work = base / "work"; work.mkdir(); source = work / "source"; source.mkdir()
            (root / "input").write_text("frozen"); (source / "input").write_text("frozen")
            config = work / "config.toml"; config.write_text("fixed")
            dependency = work / "deps"; dependency.mkdir(); (dependency / "lib.rlib").write_text("compiled")
            session = MODULE.Session.__new__(MODULE.Session)
            session.root, session.source = root, source
            session.identity = {"input": MODULE.PRODUCT.sha256(root / "input")}
            session.product_identity = dict(session.identity)
            session.source_inventory = MODULE.PRODUCT.inventory(source)
            session.config, session.config_sha256 = config, MODULE.PRODUCT.sha256(config)
            session.protected_tools = {}; session.protected_islands = {}
            session.protect(dependency)
            with patch.object(MODULE, "require_current_candidate"):
                session.check()
                added = source / "unexpected"; added.write_text("new")
                with self.assertRaises(MODULE.Incomplete): session.check()
                added.unlink(); config.write_text("changed")
                with self.assertRaises(MODULE.Incomplete): session.check()
                config.write_text("fixed"); (dependency / "lib.rlib").write_text("changed")
                with self.assertRaises(MODULE.Incomplete): session.check()

    def test_shared_lane_orchestration_retains_five_separate_denominators(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); tools = work / "tools"; tools.mkdir()
            report = {}; calls = []
            policy = {"coverage_wall_seconds": 300, "mutation_wall_seconds": 1200, "maximum_mutants": 512}
            warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                       "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                       "are usually safe\n")
            def run(label, command, timeout, **options):
                calls.append((label, command, timeout))
                if label == "coverage":
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename, "summary": {"lines": {"count": 10, "covered": 9}}}
                        for _, filename in FILES]}]}))
                    return "test result: ok. 8 passed; 0 failed; 0 ignored;"
                name = label.split(":")[1]; filename = dict(FILES)[name]
                mutant = {"file": filename, "name": "one", "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return (warning if name == "era" else "") + json.dumps([mutant])
                directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 8 passed; 0 failed; 0 ignored;")
                (directory / "outcomes.json").write_text(json.dumps(complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": [{"phase": "Test"}]}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            MODULE.run_analysis_lanes(SCOPE, dict(FILES), policy, work, tools, run, report, "all")
            self.assertEqual(report["mutation_enumerated_total"], 5)
            self.assertEqual(report["mutation_enumeration"]["era"]["warning"], warning)
            self.assertEqual(report["mutation_enumeration"]["era"]["raw_output"], warning + json.dumps(
                [{"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}]))
            self.assertIsNone(report["mutation_enumeration"]["coordinate"]["warning"])
            self.assertEqual(set(report["mutation"]), set(dict(FILES)))
            self.assertTrue(all(value["viable"] == 1 for value in report["mutation"].values()))
            self.assertEqual(len(calls), 11)
            self.assertTrue(all(command[command.index("--package") + 1] == "time-domain" for _, command, _ in calls))
            for label, command, _ in calls:
                if label.startswith("mutation-"):
                    self.assertEqual(command[command.index("--jobserver") + 1], "true")
                    self.assertEqual(command[command.index("--jobserver-tasks") + 1], "12")

    def test_session_seals_all_copied_dependencies_without_overlapping_binds(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            session = MODULE.Session.__new__(MODULE.Session)
            session.root, session.work, session.source = base / "root", base / "work", base / "work/source"
            session.output, session.config = base / "output", base / "work/cargo-home/config.toml"
            session.output.mkdir()
            session.base, session.stable, session.vendor, session.tools = (base / n for n in ("payload", "stable", "vendor", "tools"))
            session.admitted = {"policy": {"common_nightly": "nightly"}}
            session.protected_islands = {base / "work/deps": {}, base / "work/consumers": {}}
            session.protected_tools = {session.tools: {}}
            session.report = {"lanes": []}
            captured = {}
            raw_listing = " WARN exact captured output\n[]\n"
            def runner(*args, **options):
                captured.update(options)
                output = raw_listing if "--list" in args[2] else "observed"
                return {"exit_code": 0, "failure": None, "output": output}
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", side_effect=runner):
                result = session.run("control", ["/tool", "arg"], 30,
                                     sealed=[base / "work/deps/lib.rlib", base / "work/consumers"])
            self.assertEqual(result, "observed")
            self.assertEqual(set(captured["sealed"]), {session.source, session.config, base / "work/deps", base / "work/consumers"})
            self.assertEqual((captured["timeout"], captured["cpu_seconds"]), (30, 30))
            self.assertEqual(captured["build_jobs"], 12)
            self.assertEqual(captured["environment"]["RUST_TEST_THREADS"], "12")
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", side_effect=runner):
                with patch.object(MODULE.PRODUCT.ENV, "require_ok", return_value="trimmed") as require_ok:
                    result = session.run("mutation-enumeration:era", ["/tool", "--list"], 30)
            require_ok.assert_called_once()
            self.assertEqual(result, raw_listing)
            self.assertEqual(session.report["lanes"][-1]["output"], raw_listing)
            self.assertEqual(captured["build_jobs"], 1)
            self.assertEqual(captured["environment"]["RUST_TEST_THREADS"], "1")


if __name__ == "__main__":
    unittest.main()
