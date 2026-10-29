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
                   "--test-workspace", "false", "--cap-lints", "false", "--jobs", "1", "--baseline", "run",
                   "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/era"]
        result = {"exit_code": 2, "failure": None, "output": "two independently visible survivors"}
        self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result), result)
        for changed in (["/other/tool", *command[1:]], [*command, "--skip-calls-defaults"],
                        [word.replace(filename, FILES[0][1]) for word in command],
                        [word.replace("/work/mutation/era", "/work/other") for word in command]):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", changed, result)
        for changed in ({**result, "exit_code": code} for code in (1, 3, 4, 70, -9, None, True)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, changed)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_campaign_result(SCOPE, tools, "era", command, {**result, "failure": "deadline"})
        session = MODULE.Session.__new__(MODULE.Session)
        session.scope, session.tools = SCOPE, tools
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
                    return json.dumps([mutant])
                directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 8 passed; 0 failed; 0 ignored;")
                (directory / "outcomes.json").write_text(json.dumps(complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": [{"phase": "Test"}]}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            MODULE.run_analysis_lanes(SCOPE, dict(FILES), policy, work, tools, run, report, "all")
            self.assertEqual(report["mutation_enumerated_total"], 5)
            self.assertEqual(set(report["mutation"]), set(dict(FILES)))
            self.assertTrue(all(value["viable"] == 1 for value in report["mutation"].values()))
            self.assertEqual(len(calls), 11)
            self.assertTrue(all(command[command.index("--package") + 1] == "time-domain" for _, command, _ in calls))

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
            def runner(*args, **options):
                captured.update(options)
                return {"exit_code": 0, "failure": None, "output": "observed"}
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", side_effect=runner):
                result = session.run("control", ["/tool", "arg"], 30,
                                     sealed=[base / "work/deps/lib.rlib", base / "work/consumers"])
            self.assertEqual(result, "observed")
            self.assertEqual(set(captured["sealed"]), {session.source, session.config, base / "work/deps", base / "work/consumers"})
            self.assertEqual((captured["timeout"], captured["cpu_seconds"]), (30, 30))


if __name__ == "__main__":
    unittest.main()
