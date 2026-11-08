"""Independent fixture metric controls; these fixtures never invoke analyzers."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("fixture_metrics_control", Path(__file__).with_name("metrics.py"))
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)


def descriptor(filename="src/host.rs", *, name="host guard"):
    """Author a complete native descriptor independently of the inventory parser."""
    span = {"start": {"line": 14, "column": 1}, "end": {"line": 18, "column": 2}}
    return {"name": name, "package": "nevrotick-windows-service-fixture", "file": filename,
            "function": {"function_name": "owner_guard", "return_type": "-> bool", "span": span},
            "span": {"start": {"line": 16, "column": 5}, "end": {"line": 16, "column": 9}},
            "replacement": "false", "genre": "FnValue"}


def inventories():
    """Keep all three source subjects explicit in every fixture."""
    return {"host": {"host guard": descriptor()},
            "error": {"error source": descriptor("src/error.rs", name="error source")},
            "diagnostics": {"diagnostics output": descriptor("src/diagnostics.rs", name="diagnostics output")}}


def execution(names=("healthy_owner", "failed_cleanup")):
    """Native test identities and zero-skipped summaries with independent facts."""
    return "".join(f"test {name} ... ok\n" for name in names) + (
        f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n")


def phases(summary):
    """Literal pinned Cargo phase grammar, distinct from assertion evidence."""
    result = []
    for phase in (["Build"] if summary == "Unviable" else ["Build", "Test"]):
        command = ["cargo", "test", *(["--no-run"] if phase == "Build" else []), "--verbose",
                   "--package=nevrotick-windows-service-fixture@0.0.0", "--message-format=json", "--frozen"]
        failed = summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test")
        result.append({"phase": phase, "argv": command, "process_status": {"Failure": 101} if failed else "Success",
                       "duration": 0.01})
    return result


def outcomes(summary="CaughtMutant"):
    """One actual-format native outcome with exactly one ordinary baseline."""
    row = {"scenario": {"Mutant": descriptor()}, "summary": summary,
           "phase_results": phases(summary), "log_path": "mutant.log"}
    report = {"cargo_mutants_version": "27.1.0", "total_mutants": 1,
              "caught": int(summary == "CaughtMutant"), "missed": int(summary == "MissedMutant"),
              "unviable": int(summary == "Unviable"), "timeout": int(summary == "Timeout"), "success": 0,
              "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success"),
                             "log_path": "baseline.log"}, row]}
    return report


def review_fixture():
    """Concrete independent review entries, never automatic generation credit."""
    reconciliation = {"functions_and_guards": ["owner_guard: readiness cannot precede acknowledgement"],
                      "derives_and_macros": ["Eq/Debug derive: compared against independent categories"],
                      "unsafe_and_cfg_omissions": ["No unsafe fn in these owners; native module is Windows-only"],
                      "independent_tests": ["healthy_owner", "failed_cleanup"], "remaining_gaps": []}
    return {"schema_version": 1, "candidate_sha256": "a" * 64,
            "inventories": M.inventory_summary(inventories()), "reviewer": "independent-fixture-review",
            "reconciliation": {name: copy.deepcopy(reconciliation) for name in ("host", "error", "diagnostics")}}


def equivalence_fixture(root):
    """Independent exact source/proof facts, including a large existing task."""
    source_name = "tools/windows-service-fixture/src/host.rs"
    source = root / source_name
    source.parent.mkdir(parents=True)
    source.write_text("fn owner_guard() -> bool { false }\n")
    proof = root / "tasks/windows11-integration.md"
    proof.parent.mkdir()
    proof.write_text("Existing maintained task context.\n" * 2200 +
                     "#### FX-EQ-001 — exact fault review\n"
                     "The helper domain, returned value, errors and effects are identical for every admitted input.\n"
                     "Counterexample boundary search and independent reviewer retained these assumptions.\n")
    entry = {"id": "FX-EQ-001", "candidate_sha256": "a" * 64,
             "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(), "mutant": descriptor(),
             "requirement": "CHECK-25", "rationale": "Exact helper-boundary equivalence with reviewed effect preservation.",
             "proof": {"path": "tasks/windows11-integration.md", "sha256": hashlib.sha256(proof.read_bytes()).hexdigest(),
                       "section": "FX-EQ-001", "reviewer": "independent-reviewer", "verdict": "independently-justified-equivalent"}}
    review = {"schema_version": 1, "candidate_sha256": "a" * 64,
              "inventories": M.inventory_summary(inventories()), "mutation_equivalences": [entry]}
    return review, {source_name: entry["source_sha256"]}


class FixtureMetricControls(unittest.TestCase):
    """Challenge false passes without installing or invoking any analysis tool."""

    def test_diagnostics_is_an_exact_required_metric_owner_with_bounded_attribution(self):
        """The new output boundary has its own gate and only its actual source/tests qualify."""
        self.assertEqual(M.SUBJECTS, {"host": "src/host.rs", "error": "src/error.rs",
                                     "diagnostics": "src/diagnostics.rs"})
        item = descriptor("src/diagnostics.rs", name="diagnostics output guard")
        self.assertEqual(M.inventory_listing(json.dumps([item]), "diagnostics"), {item["name"]: item})
        admitted = {"extensions": {"cargo-mutants": Path("/selected/cargo-mutants")}}
        command = M.mutation_base_command(admitted, "diagnostics")
        self.assertEqual(command[command.index("--file") + 1], "src/diagnostics.rs")
        self.assertIn("--cap-lints", command)
        self.assertEqual(command[command.index("--cap-lints") + 1], "false")
        for filename in ("src/diagnostics.rs", "src/diagnostics_tests.rs"):
            for prefix in ("", "tools/windows-service-fixture/", "/work/source/tools/windows-service-fixture/"):
                self.assertTrue(M.selected_span(prefix + filename))
        for filename in ("../src/diagnostics.rs", "src/diagnostics.rs/foreign", "src/diagnostics_extra.rs",
                         "/foreign/src/diagnostics.rs", "src/../src/diagnostics_tests.rs"):
            self.assertFalse(M.selected_span(filename))

    def test_received_record_is_parsed_from_one_strict_owned_snapshot(self):
        self.assertEqual(M.decode_record(b'{"status":"passed","count":1}'), {"status": "passed", "count": 1})
        for data in (b'{"count":1,"count":2}', b'{"value":NaN}', b'{"value":1e999}',
                     b'[]', b'{"value":' + b'9' * 129 + b'}',
                     b'{"value":' + b'[' * 65 + b'0' + b']' * 65 + b'}'):
            with self.subTest(data=data), self.assertRaises(M.Incomplete):
                M.decode_record(data)

    def test_exact_coverage_owners_pass_at_boundary_and_retained_unrelated_data_is_not_a_gate(self):
        report = {"data": [{"files": [
            {"filename": "/work/source/tools/windows-service-fixture/src/host.rs",
             "summary": {"lines": {"count": 100, "covered": 90}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/error.rs",
             "summary": {"lines": {"count": 10, "covered": 10}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/diagnostics.rs",
             "summary": {"lines": {"count": 100, "covered": 95}}},
            {"filename": "/work/source/unrelated.rs", "summary": {"lines": {"count": 1000, "covered": 0}}}]}]}
        before = copy.deepcopy(report)
        result = M.metric_coverage(report)
        self.assertEqual(result["host"]["line_percent"], 90)
        self.assertEqual(result["error"]["line_percent"], 100)
        self.assertEqual(result["diagnostics"]["line_percent"], 95)
        self.assertEqual(report, before)
        for change in ("missing", "empty", "low", "duplicate", "foreign"):
            broken = copy.deepcopy(report)
            files = broken["data"][0]["files"]
            if change == "missing":
                del files[0]
            elif change == "empty":
                files[0]["summary"]["lines"] = {"count": 0, "covered": 0}
            elif change == "low":
                files[0]["summary"]["lines"]["covered"] = 89
            elif change == "duplicate":
                files.append(copy.deepcopy(files[0]))
            else:
                files[0]["filename"] = "/work/source/foreign/src/host.rs"
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.metric_coverage(broken)

    def test_diagnostics_cannot_borrow_coverage_or_inventory_from_other_owners(self):
        """A green host/error pair cannot hide a missing or weak diagnostics owner."""
        report = {"data": [{"files": [
            {"filename": "/work/source/tools/windows-service-fixture/src/host.rs",
             "summary": {"lines": {"count": 100, "covered": 100}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/error.rs",
             "summary": {"lines": {"count": 100, "covered": 100}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/diagnostics.rs",
             "summary": {"lines": {"count": 100, "covered": 90}}}]}]}
        self.assertEqual(M.metric_coverage(report)["diagnostics"]["line_percent"], 90)
        for change in ("low", "missing", "duplicate", "foreign"):
            bad = copy.deepcopy(report)
            files = bad["data"][0]["files"]
            if change == "low":
                files[2]["summary"]["lines"]["covered"] = 89
            elif change == "missing":
                del files[2]
            elif change == "duplicate":
                files.append(copy.deepcopy(files[2]))
            else:
                files[2]["filename"] = "/foreign/src/diagnostics.rs"
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.metric_coverage(bad)
        rows = inventories()
        del rows["diagnostics"]
        with self.assertRaises(M.Incomplete):
            M.inventory_summary(rows)
        review = review_fixture()
        del review["reconciliation"]["diagnostics"]
        with self.assertRaises(M.Incomplete):
            M.receive_review(review, "a" * 64, M.inventory_summary(inventories()))

    def test_diagnostics_actual_assertion_and_compiler_sources_keep_distinct_credit(self):
        """Only selected diagnostics assertions can kill; compiler faults remain zero credit."""
        item = descriptor("src/diagnostics.rs", name="diagnostics output")
        expected = {item["name"]: item}
        text = ("test bounded_output ... FAILED\n\n---- bounded_output stdout ----\n"
                "thread 'bounded_output' panicked at src/diagnostics_tests.rs:120:5:\n"
                "assertion `left == right` failed\n  left: 1\n right: 2\n\nfailures:\n")
        diagnostic = {"reason": "compiler-message", "target": {"name": "nevrotick_windows_service_fixture"},
                      "message": {"level": "error", "message": "mismatched types",
                                  "spans": [{"is_primary": True, "file_name": "src/diagnostics.rs"}]}}
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            for summary in ("CaughtMutant", "Unviable"):
                report = outcomes(summary)
                report["outcomes"][1]["scenario"]["Mutant"] = item
                (directory / "mutant.log").write_text(text if summary == "CaughtMutant" else json.dumps(diagnostic) + "\n")
                causes = M.mutation_causes(directory, report, ["bounded_output"])
                if summary == "CaughtMutant":
                    self.assertEqual(causes[item["name"]]["kind"], "behavioral-assertion")
                    self.assertEqual(causes[item["name"]]["causes"][0]["source"], "src/diagnostics_tests.rs")
                    result = M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs", defer_gate=True)
                    self.assertEqual((result["killed"], result["unviable"]), (1, 0))
                else:
                    self.assertEqual(causes[item["name"]]["kind"], "compiler-unviable")
                    self.assertEqual(causes[item["name"]]["kill_credit"], 0)
                    with self.assertRaisesRegex(M.Incomplete, "no viable denominator"):
                        M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs", defer_gate=True)
            report = outcomes("MissedMutant")
            report["outcomes"][1]["scenario"]["Mutant"] = item
            result = M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs",
                                         native_exit_code=2, defer_gate=True)
            self.assertFalse(result["gate_passed"])
            self.assertEqual(result["unresolved_survivors"], ["diagnostics output"])

    def test_native_test_names_counts_and_failures_cannot_be_substituted(self):
        self.assertEqual(M.executed_tests(execution()), ["failed_cleanup", "healthy_owner"])
        for text in ("", execution(()), execution().replace("0 ignored", "1 ignored"),
                     execution().replace("0 filtered out", "1 filtered out"),
                     execution().replace("healthy_owner ... ok", "healthy_owner ... FAILED"),
                     execution(("healthy_owner", "healthy_owner")),
                     execution().replace("2 passed", "3 passed")):
            with self.subTest(text=text), self.assertRaises(M.Incomplete):
                M.executed_tests(text)
        with self.assertRaisesRegex(M.Incomplete, "omits or adds"):
            M.executed_tests(execution(), ["healthy_owner"])
        docs = execution() + execution(("src/lib.rs - (line 9)",))
        self.assertEqual(len(M.executed_tests(docs, ["healthy_owner", "failed_cleanup"], allow_doctests=True)), 3)

    def test_command_failure_and_timeout_receive_no_metric_credit(self):
        for result in ({}, {"exit_code": True, "failure": None, "output": ""},
                       {"exit_code": 0, "failure": "wall timeout", "output": ""},
                       {"exit_code": 101, "failure": None, "output": "assertion failed"}):
            with self.subTest(result=result), self.assertRaises(M.Incomplete):
                M.successful(result, mutation=True)
        M.successful({"exit_code": 2, "failure": None, "output": "complete native missed report"}, mutation=True)
        with self.assertRaises(M.Incomplete):
            M.successful({"exit_code": 2, "failure": None, "output": ""})

    def test_complete_descriptor_inventory_is_exact_finite_and_unfiltered(self):
        item = descriptor()
        self.assertEqual(M.inventory_listing(json.dumps([item]), "host"), {item["name"]: item})
        for text in ("[]", "{}", "[NaN]", "[] trailing", '[{"file":"a","file":"b"}]',
                     json.dumps([item, item]), json.dumps([descriptor("src/error.rs")]),
                     json.dumps([{**item, "package": "time-core"}]),
                     json.dumps([{key: value for key, value in item.items() if key != "span"}]),
                     json.dumps([{**item, "function": None}])):
            with self.subTest(text=text), self.assertRaises(M.Incomplete):
                M.inventory_listing(text, "host")
        oversized = [descriptor(name=f"fault {index}") for index in range(513)]
        with self.assertRaises(M.Incomplete):
            M.inventory_listing(json.dumps(oversized), "host")
        with self.assertRaises(M.Incomplete):
            M.inventory_summary({"host": inventories()["host"]})

    def test_review_hashes_bind_all_descriptor_members_and_require_omission_reconciliation(self):
        rows = inventories()
        summaries = M.inventory_summary(rows)
        rows["host"]["host guard"]["diff"] = "rendered preview"
        self.assertEqual(M.inventory_summary(rows), summaries)
        rows["host"]["host guard"]["replacement"] = "true"
        self.assertNotEqual(M.inventory_summary(rows), summaries)
        review = review_fixture()
        self.assertIs(M.receive_review(review, "a" * 64, summaries), review)
        for change in ("candidate", "count", "hash", "reviewer", "omissions", "functions", "tests"):
            bad = copy.deepcopy(review)
            if change == "candidate":
                bad["candidate_sha256"] = "b" * 64
            elif change == "count":
                bad["inventories"]["host"]["count"] += 1
            elif change == "hash":
                bad["inventories"]["host"]["sha256"] = "b" * 64
            elif change == "reviewer":
                bad["reviewer"] = ""
            else:
                field = {"omissions": "unsafe_and_cfg_omissions", "functions": "functions_and_guards",
                         "tests": "independent_tests"}[change]
                bad["reconciliation"]["host"][field] = []
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.receive_review(bad, "a" * 64, summaries)

    def test_per_file_mutation_rejects_missing_outcomes_compiler_only_denominator_and_survivors(self):
        expected = inventories()["host"]
        report = outcomes()
        result = M.C.mutation_counts(M.SCOPE, report, expected, "src/host.rs")
        self.assertEqual((result["killed"], result["viable"], result["viable_killed_percent"]), (1, 1, 100))
        for bad in (outcomes("Unviable"), {**report, "total_mutants": 2},
                    {**report, "outcomes": report["outcomes"][:1]}):
            with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                M.C.mutation_counts(M.SCOPE, bad, expected, "src/host.rs")
        survived = M.C.mutation_counts(M.SCOPE, outcomes("MissedMutant"), expected, "src/host.rs",
                                      native_exit_code=2, defer_gate=True)
        self.assertFalse(survived["gate_passed"])
        self.assertEqual(survived["unresolved_survivors"], ["host guard"])

    def test_exact_equivalence_admission_binds_complete_inventory_source_and_existing_task_proof(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            review, identity = equivalence_fixture(root)
            before = copy.deepcopy(review)
            admitted = M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)
            self.assertEqual(admitted, review["mutation_equivalences"])
            self.assertEqual(review, before)
            self.assertIsNot(admitted, review["mutation_equivalences"])
            self.assertGreater((root / "tasks/windows11-integration.md").stat().st_size, 65536)
            for change in ("candidate", "source", "inventory", "bool-count", "unknown", "replacement", "duplicate-id",
                           "duplicate-fault", "proof-hash", "proof-section", "no-proof", "no-reviewer", "no-rationale"):
                bad = copy.deepcopy(review)
                entry = bad["mutation_equivalences"][0]
                if change == "candidate":
                    bad["candidate_sha256"] = "b" * 64
                elif change == "source":
                    entry["source_sha256"] = "b" * 64
                elif change == "inventory":
                    bad["inventories"]["host"]["sha256"] = "b" * 64
                elif change == "bool-count":
                    bad["inventories"]["host"]["count"] = True
                elif change == "unknown":
                    entry["mutant"]["name"] = "unlisted fault"
                elif change == "replacement":
                    entry["mutant"]["replacement"] = "true"
                elif change == "duplicate-id":
                    bad["mutation_equivalences"].append(copy.deepcopy(entry))
                elif change == "duplicate-fault":
                    duplicate = copy.deepcopy(entry)
                    duplicate["id"] = "FX-EQ-002"
                    bad["mutation_equivalences"].append(duplicate)
                elif change == "proof-hash":
                    entry["proof"]["sha256"] = "b" * 64
                elif change == "proof-section":
                    entry["proof"]["section"] = "FX-EQ-002"
                elif change == "no-proof":
                    del entry["proof"]
                elif change == "no-reviewer":
                    entry["proof"]["reviewer"] = ""
                else:
                    entry["rationale"] = ""
                with self.subTest(change=change), self.assertRaises(M.Incomplete):
                    M.admit_equivalence_review(bad, "a" * 64, inventories(), identity, root)
            source = root / "tools/windows-service-fixture/src/host.rs"
            source.write_text("changed source after review\n")
            with self.assertRaisesRegex(M.Incomplete, "source or full fixture candidate"):
                M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)

    def test_equivalence_requires_a_nonempty_canonical_proof_section_and_whole_file_hash(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            review, identity = equivalence_fixture(root)
            proof = root / "tasks/windows11-integration.md"
            for text in ("## unrelated section\nargument\n", "#### FX-EQ-001\n\n#### NEXT\nforeign argument\n"):
                proof.write_text(text)
                review["mutation_equivalences"][0]["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
                with self.subTest(text=text), self.assertRaises(M.Incomplete):
                    M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)
            for path in ("../tasks/windows11-integration.md", "/tasks/windows11-integration.md", "artifacts/proof.md"):
                review["mutation_equivalences"][0]["proof"]["path"] = path
                with self.subTest(path=path), self.assertRaisesRegex(M.Incomplete, "canonical maintained task"):
                    M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)

    def test_equivalent_survivor_changes_only_adjusted_denominator_and_never_gets_kill_credit(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            review, identity = equivalence_fixture(root)
            entry = M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)[0]
            report = outcomes("MissedMutant")
            caught = descriptor(name="discriminated neighboring fault")
            report["outcomes"].append({"scenario": {"Mutant": caught}, "summary": "CaughtMutant",
                                       "phase_results": phases("CaughtMutant"), "log_path": "caught.log"})
            report.update(total_mutants=2, caught=1)
            expected = {"host guard": descriptor(), caught["name"]: caught}
            before = copy.deepcopy(report)
            result = M.C.mutation_counts(M.SCOPE, report, expected, "src/host.rs", native_exit_code=2,
                                         equivalences=[entry], defer_gate=True)
            self.assertTrue(result["gate_passed"])
            self.assertEqual((result["generated"], result["killed"], result["survived"], result["excluded"]), (2, 1, 1, 1))
            self.assertEqual((result["raw_denominator"], result["adjusted_denominator"]), (2, 1))
            self.assertEqual((result["raw_viable_killed_percent"], result["adjusted_viable_killed_percent"]), (50, 100))
            self.assertEqual(report, before)
            (root / "mutant.log").write_text(execution())
            cause = M.mutation_causes(root, outcomes("MissedMutant"), ["healthy_owner", "failed_cleanup"], [entry])
            self.assertEqual(cause["host guard"]["kind"], "reviewed-equivalent-survivor")
            self.assertEqual(cause["host guard"]["kill_credit"], 0)

    def test_equivalence_cannot_hide_unknown_changed_duplicate_nonmissed_faults_or_timeouts(self):
        entry = {"id": "FX-EQ-001", "mutant": descriptor(), "proof": {"reviewer": "reviewer"}}
        M.equivalent_misses(outcomes("MissedMutant"), [entry])
        for summary in ("CaughtMutant", "Unviable", "Timeout"):
            with self.subTest(summary=summary), self.assertRaises(M.Incomplete):
                M.equivalent_misses(outcomes(summary), [entry])
        for entries in ([entry, entry], [{**entry, "mutant": descriptor(name="unknown")}],
                        [{**entry, "mutant": {**descriptor(), "replacement": "true"}}]):
            with self.subTest(entries=entries), self.assertRaises(M.Incomplete):
                M.equivalent_misses(outcomes("MissedMutant"), entries)
        with self.assertRaises(M.Incomplete):
            M.C.mutation_counts(M.SCOPE, outcomes("Timeout"), inventories()["host"], "src/host.rs",
                                native_exit_code=3, equivalences=[entry], defer_gate=True)

    def test_only_actual_selected_semantic_assertion_is_a_kill_cause(self):
        text = ("test healthy_owner ... FAILED\n\n---- healthy_owner stdout ----\n"
                "thread 'healthy_owner' panicked at src/host_tests.rs:120:5:\n"
                "assertion `left == right` failed\n  left: 1\n right: 2\n\nfailures:\n")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "mutant.log").write_text(text)
            checked = M.mutation_causes(directory, outcomes(), ["healthy_owner"])
            self.assertEqual(checked["host guard"]["kind"], "behavioral-assertion")
            for bad in (text.replace("src/host_tests.rs", "foreign.rs"),
                        text.replace("assertion `left == right` failed", "test fixture construction failed"),
                        text + "No space left on device\n", text.replace("healthy_owner", "foreign_case")):
                (directory / "mutant.log").write_text(bad)
                with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                    M.mutation_causes(directory, outcomes(), ["healthy_owner"])
            with self.assertRaises(M.Incomplete):
                M.owned_log(directory, "../mutant.log")

    def test_compiler_unviable_is_source_attributed_and_never_killed(self):
        diagnostic = {"reason": "compiler-message", "target": {"name": "nevrotick_windows_service_fixture"},
                      "message": {"level": "error", "message": "mismatched types",
                                  "spans": [{"is_primary": True, "file_name": "src/host.rs"}]}}
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "mutant.log").write_text(json.dumps(diagnostic) + "\n")
            causes = M.mutation_causes(directory, outcomes("Unviable"), ["healthy_owner"])
            self.assertEqual(causes["host guard"]["kill_credit"], 0)
            for file in ("foreign.rs", "../src/host.rs"):
                diagnostic["message"]["spans"][0]["file_name"] = file
                (directory / "mutant.log").write_text(json.dumps(diagnostic) + "\n")
                with self.subTest(file=file), self.assertRaises(M.Incomplete):
                    M.mutation_causes(directory, outcomes("Unviable"), ["healthy_owner"])

    def test_nocapture_assertion_before_fatal_abort_is_distinct_from_abort_or_generic_panic(self):
        text = ("test healthy_owner ... \n"
                "thread 'healthy_owner' (12345) panicked at src/host_tests.rs:120:5:\n"
                "assertion `left == right` failed\n  left: 1\n right: 2\n"
                "process didn't exit successfully (signal: 6, SIGABRT: process abort signal)\n")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "mutant.log").write_text(text)
            checked = M.mutation_causes(directory, outcomes(), ["healthy_owner"])
            self.assertEqual(checked["host guard"]["causes"], [{"test": "healthy_owner", "source": "src/host_tests.rs",
                              "line": 120, "kind": "unbuffered-semantic-assertion"}])
            for bad in ("process abort signal SIGABRT\n", text.replace("healthy_owner'", "foreign_thread'"),
                        text.replace("src/host_tests.rs", "foreign.rs"),
                        text.replace("assertion `left == right` failed", "test fixture construction failed"),
                        text.replace("assertion `left == right` failed", "arbitrary panic message"),
                        text + "Cannot allocate memory\n"):
                (directory / "mutant.log").write_text(bad)
                with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                    M.mutation_causes(directory, outcomes(), ["healthy_owner"])

    def test_fixed_commands_preserve_all_files_lints_baselines_and_resource_roles(self):
        admitted = {"extensions": {"cargo-mutants": Path("/selected/cargo-mutants")}}
        command = M.mutation_base_command(admitted, "host")
        self.assertEqual(command, ["/selected/cargo-mutants", "mutants", "--package",
            "nevrotick-windows-service-fixture", "--file", "src/host.rs", "--test-workspace", "false",
            "--cap-lints", "false", "--jobs", "4", "--jobserver", "true", "--jobserver-tasks", "4",
            "--cargo-arg=--message-format=json", "--cargo-arg=--frozen"])
        self.assertFalse(any(arg in command for arg in ("--exclude", "--skip", "--skip-call", "--in-place")))
        with self.assertRaises(M.Incomplete):
            M.mutation_base_command(admitted, "engine")

    def test_failed_native_command_is_retained_before_rejection_with_exact_resource_role(self):
        session = M.Session.__new__(M.Session)
        session.work, session.source, session.vendor = Path("/owned/work"), Path("/owned/work/source"), Path("/vendor")
        session.admitted, session.record = {"selected": "admitted"}, {"commands": []}
        session.check = lambda: None
        result = {"exit_code": 101, "failure": None, "output": "actual failed baseline"}
        with patch.object(M.Q, "run_command", return_value=result) as run, \
                patch.object(M.shutil, "disk_usage", return_value=type("Disk", (), {"free": 2**60})()), \
                self.assertRaises(M.Incomplete):
            session.run("mutation:host", ["literal-mutator", "mutants"], 117, mutation=True)
        run.assert_called_once_with(session.work, ["literal-mutator", "mutants"], session.admitted,
                                    session.vendor, session.source, timeout=117, cpu_seconds=117,
                                    mutation=True, cwd=session.source / "tools/windows-service-fixture")
        self.assertEqual(session.record["commands"][0]["output"], "actual failed baseline")

    def test_retention_preserves_profiles_binaries_outcomes_and_failed_runs(self):
        for path in ("work/target/llvm-cov-target/debug/fixture", "work/target/a.profraw",
                     "work/target/a.profdata", "work/mutation/host/mutants.out/outcomes.json",
                     "work/mutation/error/mutants.out/log/fault.log", "work/coverage.json"):
            with self.subTest(path=path):
                self.assertTrue(M.E.retained(path))
        self.assertFalse(M.E.retained("work/target/debug/derived-only"))
        with tempfile.TemporaryDirectory() as temporary:
            origin, ledger = Path(temporary) / "origin", Path(temporary) / "ledger"
            origin.mkdir(mode=0o700)
            ledger.mkdir(mode=0o700)
            target = origin / "work/target/llvm-cov-target/debug"
            target.mkdir(parents=True)
            (target / "fixture").write_bytes(b"LLVM instrumented executable")
            (origin / "failed.log").write_text("actual first failed attempt\n")
            with patch.object(M.E, "RECORDS", ledger), patch.object(M.E, "DISK_FLOOR", 0):
                M.E.export_tree(origin, ledger / "raw", ledger)
            self.assertEqual((ledger / "raw/failed.log").read_text(), "actual first failed attempt\n")
            self.assertEqual((ledger / "raw/work/target/llvm-cov-target/debug/fixture").read_bytes(),
                             b"LLVM instrumented executable")
            self.assertTrue(json.loads((ledger / "export-map.json").read_text())["complete"])


if __name__ == "__main__":
    unittest.main()
