"""Fixed current three-owner acquisition-session admission controls; no compiler/analyzer execution.

Literal synthetic native logs challenge parser contracts and claim no product
execution. Actual ordinary/inventory admission remains pending independently.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("session_analysis_test", ROOT / "scripts/verify-engine-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
PRIVATE_TEST = "session::tests::private_control"
EXTERNAL_TEST = "session_external_control"


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def fixture():
    ordinary = sorted([PRIVATE_TEST, EXTERNAL_TEST])
    docs = [MODULE.SESSION_FILE + " - session::AcquisitionSession (line 1)",
            MODULE.SESSION_FILE + " - session::PreparedAcquisition (line 2) - compile fail"]
    inventory = {"ordinary": ordinary, "doctests": docs,
                 "sha256": {"ordinary": digest(ordinary), "doctests": digest(docs),
                            "baseline": digest(sorted(ordinary + docs))}}
    admission = {"candidate_sha256": digest({"fixture.rs": "1" * 64}), "source_file_count": 1,
                 "ordinary_evidence": {"receipt": "/not-created/ordinary.json", "sha256": "2" * 64,
                                       "status": MODULE.SESSION_ORDINARY_STATUS, "recipes_sha256": {"/not-created/recipe.py": "3" * 64}},
                 "test_inventory": inventory,
                 "semantic_tests": {PRIVATE_TEST: {"source": MODULE.SESSION_PRIVATE, "helpers": []},
                                    EXTERNAL_TEST: {"source": MODULE.SESSION_EXTERNAL, "helpers": []}}}
    policy = {**copy.deepcopy(MODULE.SESSION_FIXED_POLICY),
              **{key: copy.deepcopy(admission[key]) for key in
                 ("candidate_sha256", "source_file_count", "ordinary_evidence", "test_inventory")},
              "inputs_sha256": {name: "4" * 64 for name in MODULE.SESSION_BOUND_INPUTS},
              "mutation_inventories": {}, "mutation_witnesses": None,
              "scope": "Synthetic parser controls only."}
    return admission, policy


def execution(names):
    return (f"running {len(names)} tests\n" + "".join(f"test {name} ... ok\n" for name in names)
            + f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


def failure(test, site):
    return (f"running 1 test\ntest {test} ... FAILED\n\nfailures:\n\n---- {test} stdout ----\n"
            + f"thread '{test}' (7) panicked at {site['source']}:{site['line']}:{site['column']}:\n{site['marker']}\n"
            + f"\nfailures:\n\n    {test}\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")


def phases():
    return [{"phase": phase, "duration": 0.1, "process_status": "Success",
             "argv": ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                      "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"]}
            for phase in ("Build", "Test")]


def singleton(rosters, selected, *, site=None):
    """Literal selected/zero-match output; no product or compiler is executed."""
    result = []
    for index, names in enumerate(rosters):
        filtered = len(names) - (index == 0)
        if index == 0:
            block = failure(selected, site) if site else execution([selected])
            result.append(block.replace("0 filtered out;", f"{filtered} filtered out;"))
        else:
            result.append(f"running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; {filtered} filtered out; finished in 0.01s\n")
        if site:
            break
    return "".join(result)


def witness_phases(selected, *, failed=False):
    """Independent literal argv/status for the two fixed native tool phases."""
    result = []
    for phase in ["Build", "Test"]:
        argv = ["cargo", "test", *(["--no-run"] if phase == "Build" else []), "--verbose",
                "--package=nevrotick-engine@0.0.0", "--message-format=json", "--frozen"]
        if phase == "Test":
            argv.extend([selected, "--", "--exact", "--test-threads=1"])
        result.append({"phase": phase, "duration": 0.1, "argv": argv,
                       "process_status": {"Failure": 101} if failed and phase == "Test" else "Success"})
    return result


def equivalences(admission):
    """Literal current native identities; synthetic custody records claim no E proof."""
    series = "crates/nevrotick-engine/src/series.rs"
    measurement = "crates/nevrotick-engine/src/measurement.rs"
    specs = [
        ("SERIES-UNAVAILABLE-ADMITTED-ZERO", series, "SeriesReport::admitted_sample_count", "-> usize",
         (473, 5), (479, 6), (476, 9), (478, 10), "0", "FnValue"),
        ("SERIES-UNAVAILABLE-ELIGIBLE-ZERO", series, "SeriesReport::eligible_sample_count", "-> usize",
         (480, 5), (486, 6), (483, 9), (485, 10), "0", "FnValue"),
        ("COHERENCE-LOWER-EQUALITY", measurement, "coherence_residual", "-> Result<u64, MeasurementRefusal>",
         (524, 1), (570, 2), (558, 29), (558, 30), "<=", "BinaryOperator"),
        ("COHERENCE-UPPER-EQUALITY", measurement, "coherence_residual", "-> Result<u64, MeasurementRefusal>",
         (524, 1), (570, 2), (562, 26), (562, 27), ">=", "BinaryOperator"),
        ("NOMINAL-ZERO-SIGN", measurement, "nominal_seconds", "-> Result<RoundedRationalSeconds, MeasurementRefusal>",
         (786, 1), (807, 2), (799, 29), (799, 30), "<=", "BinaryOperator"),
    ]
    def span(start, end):
        return {"start": {"line": start[0], "column": start[1]}, "end": {"line": end[0], "column": end[1]}}
    entries = []
    for suffix, filename, function, returns, fstart, fend, start, end, replacement, genre in specs:
        identifier = "ENGINE-SESSION-" + suffix
        action = (f"replace {function} {returns} with {replacement}" if genre == "FnValue" else
                  f"replace {'>' if replacement == '>=' else '<'} with {replacement} in {function}")
        mutant = {"name": f"{filename}:{start[0]}:{start[1]}: {action}", "package": "nevrotick-engine",
                  "file": filename, "function": {"function_name": function, "return_type": returns,
                                                 "span": span(fstart, fend)},
                  "span": span(start, end), "replacement": replacement, "genre": genre}
        entries.append({"id": identifier, "candidate_sha256": admission["candidate_sha256"],
                        "source_sha256": ("3444eeb5e950c77ac738bbb268ef43e2dd68b6984adac1454e2d8754b174da7c"
                                          if filename == series else
                                          "0b97439bbb6b6e83db74b046a7140b0030e3c97b7aa69b67423f60c454889ff7"),
                        "mutant": mutant, "requirement": "CHECK-15/23/25/26",
                        "rationale": "Synthetic schema/custody control only; no native kill or semantic proof credit.",
                        "proof": {"path": "tasks/engine-analysis-equivalences.md", "sha256": "4" * 64,
                                  "section": identifier, "reviewer": "independent source reviewer",
                                  "verdict": "independently-justified-equivalent"}})
    return entries


class SessionAnalysisTests(unittest.TestCase):
    def setUp(self):
        # Fixture identities model the same mandatory-owner check without
        # importing actual product execution into these synthetic controls.
        selected = patch.object(MODULE, "SESSION_REQUIRED_TESTS", frozenset({PRIVATE_TEST, EXTERNAL_TEST}))
        selected.start()
        self.addCleanup(selected.stop)
        for name, value in (
            ("SESSION_TEST_SOURCES", {PRIVATE_TEST: MODULE.SESSION_PRIVATE, EXTERNAL_TEST: MODULE.SESSION_EXTERNAL}),
            ("SESSION_SOURCE_HELPERS", {MODULE.SESSION_PRIVATE: (), MODULE.SESSION_EXTERNAL: ()}),
        ):
            helpers = patch.object(MODULE, name, value)
            helpers.start()
            self.addCleanup(helpers.stop)

    def test_unconditional_repository_import_closure_is_pinned(self):
        imported = set()
        prefix = str(ROOT) + "/"

        def observe(frame, event, _argument):
            if event == "call" and frame.f_code.co_name == "<module>":
                filename = frame.f_code.co_filename
                if filename.startswith(prefix):
                    imported.add(Path(filename).relative_to(ROOT).as_posix())
                    if len(imported) > 64:
                        raise AssertionError("Repository import closure exceeded its fixture bound.")

        specification = importlib.util.spec_from_file_location(
            "session_unconditional_import_control", ROOT / MODULE.WRAPPER)
        fresh = importlib.util.module_from_spec(specification)
        previous = sys.getprofile()
        with patch("subprocess.Popen", side_effect=AssertionError("Import admission cannot start a native job.")):
            sys.setprofile(observe)
            try:
                specification.loader.exec_module(fresh)
                _policy, product = fresh.PRODUCT.admit_product(ROOT)
            finally:
                sys.setprofile(previous)

        pinned = set(product) | set(fresh.SESSION_BOUND_INPUTS)
        self.assertIn("scripts/analysis-controls.py", imported)
        self.assertEqual(imported - pinned, set())
        self.assertEqual(imported - (pinned - {"scripts/analysis-controls.py"}),
                         {"scripts/analysis-controls.py"})


    def test_unexecuted_ordinary_role_fails_before_any_session(self):
        pending = json.loads((ROOT / MODULE.SESSION_POLICY).read_text())
        args = SimpleNamespace(tool_admission_profile="full", lane="enumerate", subject="session")
        with patch.object(MODULE, "SESSION_ADMISSION", None), patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "not been executed"):
                MODULE.validate_session_policy(pending)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.execute_session_profile(args, {})
            session.assert_not_called()

    def test_equivalences_require_none_or_all_five_current_native_identities(self):
        admission, policy = fixture()
        entries = equivalences(admission)
        with patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch("subprocess.Popen", side_effect=AssertionError("No native work in equivalence controls")):
            MODULE.validate_session_equivalence_scope([])
            MODULE.validate_session_equivalence_scope(entries)
            policy["mutation_equivalences"] = entries
            MODULE.validate_session_policy(policy)
            controls = [entries[:i] + entries[i + 1:] for i in range(5)]
            controls.extend([None, {}, [*entries[:-1], entries[0]]])
            for field, value in [("id", "ENGINE-SERIES-COMMIT-DURATION-FALLBACK-SUB"),
                                 ("source_sha256", "174ab3b4538356c5134cb850baac19640baf27c1e14ad3c12f953948549a43f7")]:
                changed = copy.deepcopy(entries); changed[0][field] = value; controls.append(changed)
            for key, value in [("path", "tasks/historical.md"), ("section", "historical"),
                               ("verdict", "caught"), ("sha256", "not-a-hash"), ("reviewer", " ")]:
                changed = copy.deepcopy(entries); changed[0]["proof"][key] = value; controls.append(changed)
            changed = copy.deepcopy(entries); changed[0]["mutant"]["diff"] = "historical diff"; controls.append(changed)
            changed = copy.deepcopy(entries); changed[0]["mutant"]["replacement"] = "1"; controls.append(changed)
            for changed in controls:
                with self.subTest(entries=changed), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_equivalence_scope(changed)
            for field, value in [("candidate_sha256", "f" * 64)]:
                changed = copy.deepcopy(policy); changed["mutation_equivalences"][0][field] = value
                with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_policy(changed)
            changed = copy.deepcopy(policy); changed["mutation_equivalences"][0]["proof"]["sha256"] = "f" * 64
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_policy(changed)

    def test_equivalence_shared_admission_checks_actual_source_candidate_and_proof_bytes(self):
        admission, policy = fixture()
        entries = equivalences(admission)
        # Only the product-discovery result is modeled; shared admission reads
        # actual current Rust copies and real bounded proof-fixture bytes.
        with tempfile.TemporaryDirectory() as temporary, \
             patch.object(MODULE.COMMON.PRODUCT, "admit_product", return_value=({}, {"fixture.rs": "1" * 64})), \
             patch("subprocess.Popen", side_effect=AssertionError("No native work in custody controls")):
            root = Path(temporary)
            for name in {entry["mutant"]["file"] for entry in entries}:
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes((ROOT / name).read_bytes())
            # These entries exercise synthetic custody of the bytes just copied,
            # not the historical native identities retained by validation tests.
            for entry in entries:
                entry["source_sha256"] = hashlib.sha256((root / entry["mutant"]["file"]).read_bytes()).hexdigest()
            proof = root / MODULE.SESSION_EQUIVALENCE_PROOF; proof.parent.mkdir(parents=True)
            content = "\n".join("## " + entry["id"] + "\nSynthetic custody control only.\n" for entry in entries)
            proof.write_text(content)
            for entry in entries: entry["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
            policy["mutation_equivalences"] = entries
            self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SESSION_SCOPE, policy), entries)
            for field in ("source_sha256", "candidate_sha256"):
                changed = copy.deepcopy(policy); changed["mutation_equivalences"][0][field] = "f" * 64
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SESSION_SCOPE, changed)
            source = root / entries[0]["mutant"]["file"]
            original = source.read_bytes()
            source.write_bytes(original + b"\n// Synthetic custody alteration after binding.\n")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SESSION_SCOPE, policy)
            source.write_bytes(original)
            proof.write_text(content + "Changed after binding.\n")
            with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SESSION_SCOPE, policy)
            proof.write_text(content.replace("## " + entries[0]["id"], "### " + entries[0]["id"]))
            for entry in entries: entry["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
            with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SESSION_SCOPE, policy)
            proof.write_text("x" * 65537)
            for entry in entries: entry["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
            with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SESSION_SCOPE, policy)

    def test_common_score_keeps_unresolved_miss_failed_even_above_ninety_five(self):
        admission, _policy = fixture()
        entries = equivalences(admission)
        series_entries = entries[:2]
        mutants = [entry["mutant"] for entry in series_entries]
        for index in range(62):
            mutants.append({**copy.deepcopy(mutants[0]), "name": f"synthetic score control {index}"})
        rows = [{"scenario": "Baseline", "summary": "Success", "phase_results": phases()}]
        for index, mutant in enumerate(mutants):
            missed = index < 3
            actual = phases()
            if not missed: actual[-1]["process_status"] = {"Failure": 101}
            rows.append({"scenario": {"Mutant": mutant}, "summary": "MissedMutant" if missed else "CaughtMutant",
                         "phase_results": actual})
        report = {"cargo_mutants_version": "27.1.0", "outcomes": rows, "total_mutants": 64,
                  "caught": 61, "missed": 3, "unviable": 0, "timeout": 0, "success": 0}
        expected = {item["name"]: item for item in mutants}
        MODULE.validate_session_equivalence_scope(entries)
        result = MODULE.COMMON.mutation_counts(MODULE.SESSION_SCOPE, report, expected, MODULE.SESSION_SUBJECTS["series"],
                                              native_exit_code=2, equivalences=series_entries, defer_gate=True)
        self.assertGreater(result["raw_viable_killed_percent"], 95)
        self.assertGreater(result["adjusted_viable_killed_percent"], 95)
        self.assertFalse(result["gate_passed"])
        self.assertEqual(result["unresolved_survivors"], [mutants[2]["name"]])
        self.assertEqual((result["killed"], result["excluded"]), (61, 2))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_counts(MODULE.SESSION_SCOPE, report, expected, MODULE.SESSION_SUBJECTS["series"],
                                          native_exit_code=2, equivalences=series_entries)
        for selected in [[*series_entries, series_entries[0]],
                         [{**series_entries[0], "mutant": mutants[3]}]]:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.SESSION_SCOPE, report, expected, MODULE.SESSION_SUBJECTS["series"],
                                              native_exit_code=2, equivalences=selected, defer_gate=True)
        report["caught"], report["missed"] = 62, 2
        rows[3]["summary"] = "CaughtMutant"; rows[3]["phase_results"][-1]["process_status"] = {"Failure": 101}
        result = MODULE.COMMON.mutation_counts(MODULE.SESSION_SCOPE, report, expected, MODULE.SESSION_SUBJECTS["series"],
                                              native_exit_code=2, equivalences=series_entries)
        self.assertTrue(result["gate_passed"])
        self.assertEqual((result["killed"], result["excluded"], result["adjusted_denominator"]), (62, 2, 62))

    def test_fixed_scope_thresholds_and_complete_frozen_inventory(self):
        admission, policy = fixture()
        with patch.object(MODULE, "SESSION_ADMISSION", admission):
            self.assertEqual(MODULE.validate_session_policy(policy), policy)
            self.assertEqual(MODULE.SESSION_SCOPE.select("all"), MODULE.SESSION_SUBJECTS)
            for key, value in [("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                               ("mutation_jobs", 9), ("mutation_wall_seconds", 1201), ("critical_files", ["foreign.rs"]),
                               ("mutation_equivalences", [{}]), ("candidate_sha256", "a" * 64),
                               ("mutation_error_values", {})]:
                bad = copy.deepcopy(policy); bad[key] = value
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_policy(bad)
            for field in ("ordinary", "doctests"):
                bad = copy.deepcopy(policy); bad["test_inventory"][field][0] = "foreign"
                with self.assertRaises(MODULE.Incomplete): MODULE.session_test_inventory(bad)
            bad = copy.deepcopy(policy); bad["inputs_sha256"].pop(MODULE.SESSION_PRIVATE)
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_policy(bad)
            altered = copy.deepcopy(admission)
            altered["semantic_tests"]["session::tests::unrelated"] = altered["semantic_tests"].pop(PRIVATE_TEST)
            names = sorted(["session::tests::unrelated", EXTERNAL_TEST])
            altered["test_inventory"]["ordinary"] = names
            altered["test_inventory"]["sha256"]["ordinary"] = digest(names)
            altered["test_inventory"]["sha256"]["baseline"] = digest(sorted(names + altered["test_inventory"]["doctests"]))
            bad = {**policy, "test_inventory": altered["test_inventory"]}
            with patch.object(MODULE, "SESSION_ADMISSION", altered), self.assertRaises(MODULE.Incomplete):
                MODULE.session_test_inventory(bad)
            altered = copy.deepcopy(admission)
            altered["semantic_tests"][PRIVATE_TEST]["helpers"] = ["serve"]
            with patch.object(MODULE, "SESSION_ADMISSION", altered), self.assertRaises(MODULE.Incomplete):
                MODULE.session_test_inventory(policy)

    def test_full_execution_rejects_missing_duplicate_filtered_or_truncated_records(self):
        admission, policy = fixture(); names = policy["test_inventory"]["ordinary"] + policy["test_inventory"]["doctests"]
        healthy = execution(names)
        with patch.object(MODULE, "SESSION_ADMISSION", admission):
            self.assertEqual(MODULE.validate_session_test_execution(healthy, policy, doctests=True)["tests_executed"], len(names))
            for bad in [execution(names[:-1]), execution(names + [names[0]]), healthy.replace(" ... ok", " ... FAILED", 1),
                        healthy.replace("0 filtered out", "1 filtered out"), healthy[:healthy.index("test result:")],
                        healthy + "running 1 test\n", healthy.replace("running 4 tests", "running 5 tests"),
                        healthy + "N3_HARNESS_INCOMPLETE\n", healthy + "Error: fixture failed\n"]:
                with self.subTest(log=bad), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_test_execution(bad, policy, doctests=True)

    def test_campaign_requires_one_complete_reviewed_inventory(self):
        admission, policy = fixture()
        self.assertEqual(MODULE.session_inventory_bindings(policy), {})
        with self.assertRaises(MODULE.Incomplete): MODULE.session_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64} for name in MODULE.SESSION_SUBJECTS}
        self.assertEqual(MODULE.session_inventory_bindings(policy, required=True), policy["mutation_inventories"])
        for value in [{"quality": {"count": 1, "sha256": "a" * 64}},
                      {"session": {"count": 0, "sha256": "a" * 64}},
                      {"session": {"count": True, "sha256": "a" * 64}},
                      {"session": {"count": 513, "sha256": "a" * 64}}]:
            policy["mutation_inventories"] = value
            with self.assertRaises(MODULE.Incomplete): MODULE.session_inventory_bindings(policy, required=True)

    def test_assertion_map_is_bound_to_both_exact_test_sources(self):
        admission, policy = fixture()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name, body in [(MODULE.SESSION_PRIVATE, "fn private_control() { assert_eq!(1, 2); }\n"),
                               (MODULE.SESSION_EXTERNAL, "fn session_external_control() { assert!(1 == 2); }\n")]:
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True); target.write_text(body)
                policy["inputs_sha256"][name] = hashlib.sha256(body.encode()).hexdigest()
            with patch.object(MODULE, "SESSION_ADMISSION", admission):
                mapping = MODULE.session_semantic_assertion_sites(root, policy)
                self.assertEqual(set(mapping["tests"]), {PRIVATE_TEST, EXTERNAL_TEST})
                self.assertEqual(mapping["tests"][PRIVATE_TEST][0]["owner"], "private_control")
                (root / MODULE.SESSION_PRIVATE).write_text("changed")
                with self.assertRaisesRegex(MODULE.Incomplete, "admitted bytes"):
                    MODULE.session_semantic_assertion_sites(root, policy)

    def test_source_map_distinguishes_condition_strings_from_static_failure_fact(self):
        admission, policy = fixture()
        sources = {
            MODULE.SESSION_PRIVATE: 'fn private_control() { assert!(1 == 2, "fixed private refusal"); }\n',
            MODULE.SESSION_EXTERNAL: 'fn session_external_control() { check(format!("{:?}", 1) == "1", "fixed semantic fact"); }\n',
        }
        with tempfile.TemporaryDirectory() as temporary, patch.object(MODULE, "SESSION_ADMISSION", admission):
            root = Path(temporary)
            for name, body in sources.items():
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True); target.write_text(body)
                policy["inputs_sha256"][name] = hashlib.sha256(body.encode()).hexdigest()
            mapping = MODULE.session_semantic_assertion_sites(root, policy)
            private = mapping["tests"][PRIVATE_TEST][0]; public = mapping["tests"][EXTERNAL_TEST][0]
            self.assertEqual((private["marker"], private["exact_message"]), ("fixed private refusal", True))
            self.assertEqual(public["kind"], "typed_semantic_assertion")
            self.assertEqual(public["marker"], 'Error: Custom { kind: Other, error: "fixed semantic fact" }')
            changed = sources[MODULE.SESSION_EXTERNAL].replace('"fixed semantic fact"', 'supplied_fact')
            (root / MODULE.SESSION_EXTERNAL).write_text(changed)
            policy["inputs_sha256"][MODULE.SESSION_EXTERNAL] = hashlib.sha256(changed.encode()).hexdigest()
            with self.assertRaisesRegex(MODULE.Incomplete, "static semantic fact"):
                MODULE.session_semantic_assertion_sites(root, policy)

    def test_caught_causes_require_complete_source_owned_primary(self):
        admission, policy = fixture()
        private = {"source": MODULE.SESSION_PRIVATE, "line": 2, "column": 5, "owner": "private_control",
                   "kind": "panic_assertion", "marker": "assertion `left == right` failed"}
        mapping = {"candidate_sha256": admission["candidate_sha256"], "source_sha256": {},
                   "tests": {PRIVATE_TEST: [private], EXTERNAL_TEST: []}}
        rows = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {
            "name": "synthetic-session-fault", "file": MODULE.SESSION_FILE, "package": "nevrotick-engine"}}, "log_path": "caught.log"}]}
        healthy = failure(PRIVATE_TEST, private)
        with tempfile.TemporaryDirectory() as temporary, patch.object(MODULE, "SESSION_ADMISSION", admission):
            root = Path(temporary); log = root / "caught.log"; log.write_text(healthy)
            result = MODULE.validate_session_caught_causes(root, rows, mapping)
            self.assertEqual(len(result["checked_caught_logs"]["synthetic-session-fault"]["causes"]), 1)
            for bad in [healthy[:healthy.rindex("\nfailures:")], healthy[:healthy.index("test result:")],
                        healthy + "signal: 9, SIGKILL\n", healthy + "fatal runtime error: aborting\n",
                        healthy + f"test {PRIVATE_TEST} ... FAILED\n",
                        healthy + f"running 1 test\ntest {PRIVATE_TEST} ... FAILED\n",
                        healthy + "test foreign::case ... FAIL\n", healthy + "test foreign::case ... \n",
                        healthy.replace("0 passed; 1 failed", "1 passed; 1 failed"),
                        healthy.replace(":2:5:", ":999:5:"), healthy.replace("private/session.rs", "private/foreign.rs"),
                        healthy.replace("thread '" + PRIVATE_TEST + "'", "thread 'foreign'"),
                        healthy + "N3_HARNESS_INCOMPLETE\n", healthy + "Error: Driver\n",
                        healthy.replace("\n\nfailures:", "\nError: Busy\n\nfailures:")]:
                log.write_text(bad)
                with self.subTest(log=bad[-160:]), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(root, rows, mapping)

    def test_expected_unwind_control_never_earns_primary_credit(self):
        admission, _ = fixture(); test = MODULE.SESSION_CONTROL_PANIC_TEST
        admission["semantic_tests"][test] = {"source": MODULE.SESSION_PRIVATE, "helpers": []}
        primary = {"source": MODULE.SESSION_PRIVATE, "line": 20, "column": 9,
                   "owner": "synthetic_execution_unwind_retires_and_releases_prepared_resources",
                   "kind": "panic_assertion", "marker": "assertion `left == right` failed"}
        control = {**primary, "line": 10, "kind": "expected_control_panic", "marker": MODULE.SESSION_CONTROL_PANIC_MARKER}
        mapping = {"candidate_sha256": admission["candidate_sha256"], "source_sha256": {},
                   "tests": {PRIVATE_TEST: [], EXTERNAL_TEST: [], test: [primary, control]}}
        expected = (f"thread '{test}' panicked at {MODULE.SESSION_PRIVATE}:10:9:\n"
                    + MODULE.SESSION_CONTROL_PANIC_MARKER + "\n")
        healthy = failure(test, primary).replace(" stdout ----\n", " stdout ----\n" + expected)
        rows = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {
            "name": "synthetic-unwind-fault", "file": MODULE.SESSION_FILE, "package": "nevrotick-engine"}}, "log_path": "caught.log"}]}
        with patch.object(MODULE, "SESSION_ADMISSION", admission):
            with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=healthy.encode()):
                out = MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping)
            result = out["checked_caught_logs"]["synthetic-unwind-fault"]
            self.assertEqual(len(result["causes"]), 1)
            self.assertEqual(len(result["expected_controls_without_credit"]), 1)
            for bad in (failure(test, control), healthy.replace(":10:9:", ":11:9:"),
                        healthy.replace(MODULE.SESSION_CONTROL_PANIC_MARKER, "foreign unwind"),
                        healthy.replace(expected, expected + expected)):
                with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=bad.encode()), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping)

    def test_fixed_public_fact_is_primary_but_native_or_driver_errors_are_not(self):
        admission, _ = fixture(); test = EXTERNAL_TEST
        site = {"source": MODULE.SESSION_EXTERNAL, "line": 20, "column": 5, "owner": "session_external_control",
                "kind": "typed_semantic_assertion",
                "marker": 'Error: Custom { kind: Other, error: "session historical facts disappeared" }'}
        mapping = {"candidate_sha256": admission["candidate_sha256"], "source_sha256": {},
                   "tests": {PRIVATE_TEST: [], test: [site]}}
        healthy = failure(test, site)
        start, end = healthy.index("thread '"), healthy.rindex("\n\nfailures:")
        healthy = healthy[:start] + site["marker"] + "\n" + healthy[end:]
        rows = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {
            "name": "synthetic-fact-fault", "file": MODULE.SESSION_FILE, "package": "nevrotick-engine"}}, "log_path": "caught.log"}]}
        with patch.object(MODULE, "SESSION_ADMISSION", admission):
            with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=healthy.encode()):
                out = MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping)
            self.assertEqual(len(out["checked_caught_logs"]["synthetic-fact-fault"]["causes"]), 1)
            for bad in (healthy.replace(site["marker"], "Error: Driver"),
                        healthy.replace(site["marker"], "Error: Elapsed(())"),
                        healthy.replace(site["marker"], site["marker"] + "\nError: Busy")):
                with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=bad.encode()), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping)

    def test_ordinary_binding_requires_fixed_success_and_all_ten_lanes(self):
        admission, policy = fixture()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); recipe = root / "recipe.py"; recipe.write_text("synthetic recipe\n")
            receipt = root / "ordinary.json"
            binding = {"receipt": str(receipt), "sha256": "0" * 64, "status": MODULE.SESSION_ORDINARY_STATUS,
                       "recipes_sha256": {str(recipe): hashlib.sha256(recipe.read_bytes()).hexdigest()}}
            lanes = [{"lane": name, "failure": None, "exit_code": 0} for name in MODULE.SESSION_ORDINARY_LANES]
            for row in lanes:
                if row["lane"] in {"engine-tests", "engine-optimized-tests", "engine-doctests"}:
                    names = policy["test_inventory"]["doctests" if row["lane"] == "engine-doctests" else "ordinary"]
                    row["tests"] = {"executed": len(names), "test_names": names}
            data = {"status": MODULE.SESSION_ORDINARY_STATUS, "bookends_unchanged": True,
                    "candidate_sha256": admission["candidate_sha256"], "source_sha256": {"fixture.rs": "1" * 64},
                    "recipes_sha256": binding["recipes_sha256"], "lanes": lanes}
            def publish(value):
                raw = json.dumps(value).encode(); receipt.write_bytes(raw)
                binding["sha256"] = hashlib.sha256(raw).hexdigest()
                policy["ordinary_evidence"] = copy.deepcopy(binding)
                admission["ordinary_evidence"] = copy.deepcopy(binding)
            publish(data)
            with patch.object(MODULE, "SESSION_ADMISSION", admission):
                MODULE.validate_session_policy(policy)
                MODULE.admit_session_ordinary(policy)
                for status in ("incomplete", "failed", "passed-session-compiler"):
                    binding["status"] = status; bad = copy.deepcopy(data); bad["status"] = status; publish(bad)
                    with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_policy(policy)
                    with self.assertRaises(MODULE.Incomplete): MODULE.admit_session_ordinary(policy)
                binding["status"] = MODULE.SESSION_ORDINARY_STATUS
                for alter in (lambda value: value["lanes"].pop(),
                              lambda value: value["lanes"][0].update(exit_code=1),
                              lambda value: value.update(bookends_unchanged=False)):
                    bad = copy.deepcopy(data); alter(bad); publish(bad)
                    with self.assertRaises(MODULE.Incomplete): MODULE.admit_session_ordinary(policy)

    def test_baseline_hashes_same_bytes_and_rejects_missing_success(self):
        admission, policy = fixture()
        names = policy["test_inventory"]["ordinary"] + policy["test_inventory"]["doctests"]
        raw = execution(names).encode()
        outcomes = {"outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases(), "log_path": "baseline.log"}]}
        with patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE.COMMON, "read_analysis_json", return_value=outcomes), \
             patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=raw) as reads, \
             patch.object(MODULE, "session_semantic_assertion_sites", return_value={}), \
             patch.object(MODULE, "validate_session_caught_causes", return_value={}) as causes:
            observed = MODULE.validate_session_baseline(Path("/not-created/work"), policy)
            self.assertEqual(observed["log_sha256"], hashlib.sha256(raw).hexdigest())
            reads.assert_called_once(); causes.assert_called_once()
            outcomes["outcomes"][0]["summary"] = "CaughtMutant"
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_baseline(Path("/not-created/work"), policy)

    def test_external_contract_probes_retain_causal_line_and_legal_neighbor(self):
        examples = MODULE.session_contract_examples()
        self.assertEqual(sum(value["code"] is None for value in examples.values()), 1)
        self.assertEqual({value["code"] for value in examples.values() if value["code"]},
                         {"E0451", "E0616", "E0382", "E0499", "E0515", "E0505", "E0277", "E0603", "E0432"})
        self.assertTrue(all(value["line"] == 4 for value in examples.values()))
        self.assertIn("owner.retire(); owner.retire();", examples["session_legal"]["source"])
        self.assertIn("plan.execute(schedule)", examples["session_plan_twice"]["source"].splitlines()[3])
        self.assertIn("view.report(), view.attempt(), view.facts(), view.measurement(), view.midpoint()",
                      examples["session_legal"]["source"])
        for field in ("report", "attempt", "facts", "measurement", "midpoint"):
            example = examples["session_reference_" + field + "_retarget"]
            self.assertEqual(example["code"], "E0616")
            self.assertIn("view." + field + " = other", example["source"].splitlines()[3])
        self.assertEqual(examples["session_reference_report_drop"]["code"], "E0505")
        self.assertEqual(examples["session_reference_owner_escape"]["code"], "E0515")
        self.assertEqual(examples["session_reference_forge"]["code"], "E0451")
        self.assertEqual(examples["session_reference_default"]["code"], "E0277")
        self.assertEqual(examples["session_reference_unchecked_from"]["code"], "E0277")
        self.assertEqual(examples["session_reference_missing_export"]["code"], "E0432")
        self.assertIn("use nevrotick_engine::reference_from_instants;",
                      examples["session_reference_missing_export"]["source"].splitlines()[3])
        self.assertEqual(examples["session_reference_private_module"]["code"], "E0603")
        self.assertEqual(examples["session_coherence_retarget"]["code"], "E0616")

    def test_product_success_witness_maps_only_literal_calls_and_unchanged_result(self):
        admission, policy = fixture()
        helpers = {MODULE.SESSION_PRIVATE: ("product_ok",), MODULE.SESSION_EXTERNAL: ()}
        admission["semantic_tests"][PRIVATE_TEST]["helpers"] = ["product_ok"]
        source = ('fn product_ok<T,E>(result: Result<T,E>, fact: &\'static str) -> Result<T,E> {\n'
                  '    assert!(result.is_ok(), "{fact}");\n    result\n}\n'
                  'fn private_control() { product_ok(pure_product(), "valid product refused"); }\n')
        with tempfile.TemporaryDirectory() as temporary, patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE, "SESSION_SOURCE_HELPERS", helpers):
            root = Path(temporary)
            for filename, text in ((MODULE.SESSION_PRIVATE, source),
                                   (MODULE.SESSION_EXTERNAL, 'fn session_external_control() { assert!(true); }\n')):
                path = root / filename; path.parent.mkdir(parents=True, exist_ok=True); path.write_text(text)
                policy["inputs_sha256"][filename] = hashlib.sha256(text.encode()).hexdigest()
            sites = MODULE.session_semantic_assertion_sites(root, policy)["tests"][PRIVATE_TEST]
            self.assertEqual([(site["owner"], site["line"], site["marker"], site["exact_message"]) for site in sites],
                             [("product_ok", 2, "valid product refused", True)])
            for bad in (source.replace('"valid product refused"', "dynamic_fact"),
                        source.replace('    result\n}', '    changed_result()\n}'),
                        source.replace('assert!(result.is_ok()', 'assert!(true')):
                (root / MODULE.SESSION_PRIVATE).write_text(bad)
                policy["inputs_sha256"][MODULE.SESSION_PRIVATE] = hashlib.sha256(bad.encode()).hexdigest()
                with self.assertRaises(MODULE.Incomplete): MODULE.session_semantic_assertion_sites(root, policy)

    def test_completed_child_bridge_needs_its_named_source_primary_and_no_other_error(self):
        admission, _ = fixture()
        primary = {"source": MODULE.SESSION_PRIVATE, "line": 9, "column": 5, "owner": "private_control",
                   "kind": "panic_assertion", "marker": "valid product refused", "exact_message": True}
        bridge = {"source": MODULE.SERIES_PROCESS_SOURCE, "line": 20, "column": 9, "owner": "completed_child",
                  "kind": "child_bridge", "marker": "N3 exact child semantic failure assertion"}
        mapping = {"candidate_sha256": admission["candidate_sha256"], "source_sha256": {},
                   "tests": {PRIVATE_TEST: [primary, bridge], EXTERNAL_TEST: []}}
        evidence = f"N3_SEMANTIC_EVIDENCE test assertion/panic at {primary['source']}:9:5\n"
        healthy = failure(PRIVATE_TEST, bridge).replace(" stdout ----\n", " stdout ----\n" + evidence)
        rows = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {
            "name": "synthetic-downstream-fault", "file": MODULE.SESSION_SUBJECTS["measurement"],
            "package": "nevrotick-engine"}}, "log_path": "caught.log"}]}
        with patch.object(MODULE, "SESSION_ADMISSION", admission):
            with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=healthy.encode()):
                result = MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping, subject="measurement")
                self.assertTrue(result["checked_caught_logs"]["synthetic-downstream-fault"]["causes"][0]["completed_child_relay"])
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping, subject="series")
            for bad in (healthy.replace(evidence, ""), healthy.replace(evidence, evidence + evidence),
                        healthy.replace(":9:5\n", ":999:5\n"), healthy.replace("private/session.rs:9", "src/measurement.rs:9"),
                        healthy.replace(":20:9:", ":21:9:"),
                        healthy.replace(evidence, evidence + "Error: Arithmetic\n"), healthy + "signal: 9, SIGKILL\n"):
                with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=bad.encode()), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping, subject="measurement")

    def test_cli_argument_guard_accepts_only_its_exact_compiler_path_alias(self):
        admission, _ = fixture()
        source = "crates/nevrotick-engine/tests/private/argument_guards.rs"
        site = {"source": source, "line": 9, "column": 5, "owner": "product_ok",
                "kind": "panic_assertion", "marker": "valid decimal profile refused", "exact_message": True}
        mapping = {"candidate_sha256": admission["candidate_sha256"], "source_sha256": {},
                   "tests": {PRIVATE_TEST: [site], EXTERNAL_TEST: []}}
        rows = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {
            "name": "profile-constructor-fault", "file": MODULE.SESSION_SUBJECTS["measurement"],
            "package": "nevrotick-engine"}}, "log_path": "caught.log"}]}
        alias = "crates/nevrotick-engine/src/bin/../../tests/private/argument_guards.rs"
        healthy = failure(PRIVATE_TEST, site).replace(source, alias)
        with patch.object(MODULE, "SESSION_ADMISSION", admission):
            with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=healthy.encode()):
                result = MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping, subject="measurement")
                self.assertEqual(result["checked_caught_logs"]["profile-constructor-fault"]["causes"][0]["source"], source)
            for foreign in ("/foreign/" + alias, alias.replace("bin/../../", "bin/../../../"),
                            alias.replace("argument_guards", "foreign")):
                with patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=healthy.replace(alias, foreign).encode()), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(Path("/not-created"), rows, mapping, subject="measurement")

    def test_each_cohort_baseline_reads_its_own_complete_native_log(self):
        admission, policy = fixture()
        names = policy["test_inventory"]["ordinary"] + policy["test_inventory"]["doctests"]
        raw = execution(names).encode()
        outcomes = {"outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases(), "log_path": "baseline.log"}]}
        work = Path("/not-created/work")
        with patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE.COMMON, "read_analysis_json", return_value=outcomes) as outcome_reads, \
             patch.object(MODULE.COMMON, "read_analysis_bytes", return_value=raw) as log_reads, \
             patch.object(MODULE, "session_semantic_assertion_sites", return_value={}), \
             patch.object(MODULE, "validate_session_caught_causes", return_value={}) as causes:
            for subject in MODULE.SESSION_SUBJECTS:
                result = MODULE.validate_session_baseline(work, policy, subject)
                directory = work / "mutation" / subject / "mutants.out"
                self.assertEqual(outcome_reads.call_args.args[0], directory / "outcomes.json")
                self.assertEqual(log_reads.call_args.args[0], directory / "baseline.log")
                self.assertEqual(causes.call_args.kwargs["subject"], subject)
                self.assertEqual(result["log_sha256"], hashlib.sha256(raw).hexdigest())
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_baseline(work, policy, "foreign")

    def test_windows_surrogate_copies_exact_helper_and_removes_only_cross_guard(self):
        source = (ROOT / MODULE.SESSION_SUBJECTS["measurement"]).read_text()
        template = (ROOT / MODULE.SESSION_WINDOWS_CONTROL).read_text()
        pattern = r"\|\|\s*later\s*[.]\s*mono\s*<\s*base\s*[.]\s*mono_after\b"
        original = re.search(pattern, source)
        self.assertIsNotNone(original)
        for guard in ("|| later.mono < base.mono_after",
                      "||\n        later.mono\n            < base.mono_after",
                      "||later . mono<base . mono_after"):
            styled = source[:original.start()] + guard + source[original.end():]
            with self.subTest(guard=guard):
                variants = MODULE.session_windows_variants(styled, template)
                self.assertEqual(variants["missing_cross_guard"],
                                 variants["healthy"].replace(guard, "", 1))
                self.assertNotIn("NEVROTICK_COHERENCE_RESIDUAL_SOURCE", variants["healthy"])
        for changed in ("|| false", "|| later.mono <= base.mono_after",
                        "|| later/* changed */.mono < base.mono_after",
                        "|| later.mono < base.mono_after.clone()",
                        "|| later.mono < base.mono_after || later.mono < base.mono_after"):
            bad = source[:original.start()] + changed + source[original.end():]
            with self.subTest(changed=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.session_windows_variants(bad, template)
        for bad_source, bad_template in ((source, template + "\n// NEVROTICK_COHERENCE_RESIDUAL_SOURCE\n"),):
            with self.assertRaises(MODULE.Incomplete): MODULE.session_windows_variants(bad_source, bad_template)

    def test_windows_surrogate_requires_exact_native_red_after_healthy_completion(self):
        policy = {"inputs_sha256": {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest()
                                   for name in (MODULE.SESSION_WINDOWS_CONTROL, MODULE.SESSION_SUBJECTS["measurement"])}}
        for bad in (None, "wrong_exit", "foreign_site", "abort", "extra_panic", "raw_error"):
            with self.subTest(control=bad), tempfile.TemporaryDirectory() as temporary:
                work, report = Path(temporary), {"lanes": []}
                def run(label, command, timeout, **options):
                    native = {"lane": label, "failure": None, "exit_code": 0}; report["lanes"].append(native)
                    if label == "session-windows-surrogate-healthy":
                        return "measurement-coherence-windows-surrogate:healthy\n"
                    if "negative" not in options: return ""
                    text = (work / "coherence-control/inputs/missing_cross_guard.rs").read_text()
                    offset = text.rfind("assert_eq!(", 0, text.index('"WINDOWS_EPSILON_CROSS_GUARD_REQUIRED"'))
                    line, column = text.count("\n", 0, offset) + 1, offset - text.rfind("\n", 0, offset)
                    native["exit_code"] = 1 if bad == "wrong_exit" else 101
                    location = "foreign.rs:1:1" if bad == "foreign_site" else f"/work/coherence-control/inputs/missing_cross_guard.rs:{line}:{column}"
                    output = (f"thread 'main' (7) panicked at {location}:\n"
                              "assertion `left == right` failed: WINDOWS_EPSILON_CROSS_GUARD_REQUIRED\n"
                              "  left: Ok(0)\n right: Err(CaptureOrder)\n")
                    return output + ("signal: 6, SIGABRT\n" if bad == "abort" else
                                     "thread 'foreign' panicked at foreign.rs:1:1:\nfixture\n" if bad == "extra_panic" else
                                     "Error: Arithmetic\n" if bad == "raw_error" else "")
                owned = SimpleNamespace(source=ROOT, work=work, stable=Path("/not-created/toolchain"), run=run, protect=lambda path: None)
                if bad is None:
                    MODULE.session_windows_control(owned, policy, report, MODULE.time.monotonic() + 300)
                    self.assertEqual(report["windows_coherence_surrogate"]["removed_guard_exit_code"], 101)
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.session_windows_control(owned, policy, report, MODULE.time.monotonic() + 300)

    def test_explicit_dispatch_preserves_other_roles(self):
        with patch.object(MODULE, "execute_session_profile") as selected, patch.object(MODULE, "read_policy") as legacy:
            MODULE.execute(SimpleNamespace(profile="session"), {})
            selected.assert_called_once(); legacy.assert_not_called()

    def test_all_lane_uses_only_session_coverage_and_existing_mutation_custody(self):
        admission, policy = fixture()
        policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64} for name in MODULE.SESSION_SUBJECTS}
        commands, finished = [], []
        def run(label, command, timeout, **options):
            commands.append((label, command, timeout))
            return execution(policy["test_inventory"]["ordinary"])
        owned = SimpleNamespace(product_identity={"fixture.rs": "1" * 64},
            source=Path("/not-created/source"), work=Path("/not-created/work"),
            tools=Path("/not-created/tools"), run=run, finish=lambda: finished.append(True))
        coverage = {"data": [{"files": [{"filename": "/work/source/" + filename,
                                         "summary": {"lines": {"count": 100, "covered": 90}}}
                                        for filename in MODULE.SESSION_SUBJECTS.values()]}]}
        args = SimpleNamespace(tool_admission_profile="full", lane="all", subject="all",
                               output_dir=Path("/not-created/output"))
        with patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE, "read_session_policy", return_value=policy), \
             patch.object(MODULE, "admit_session_ordinary", return_value={}), \
             patch.object(MODULE, "read_json", return_value=policy), \
             patch.object(MODULE.PRODUCT, "sha256", return_value="4" * 64), \
             patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5 * 1024**3)), \
             patch.object(MODULE.COMMON, "Session", return_value=owned), \
             patch.object(MODULE.COMMON, "read_analysis_json", return_value=coverage), \
             patch.object(MODULE.COMMON, "run_analysis_lanes") as mutation, \
             patch.object(MODULE, "session_windows_control") as windows, \
             patch.object(MODULE, "contracts") as contracts:
            report = {}; MODULE.execute_session_profile(args, report)
        contracts.assert_called_once(); windows.assert_called_once(); mutation.assert_called_once()
        self.assertEqual(mutation.call_args.args[-1], "mutation")
        self.assertEqual(mutation.call_args.args[1], MODULE.SESSION_SUBJECTS)
        self.assertEqual(finished, [True])
        self.assertEqual(report["coverage"]["session"]["line_percent"], 90)
        command = commands[0][1]
        self.assertEqual(command[command.index("--fail-under-file-lines") + 1], "90")
        import re
        ignored = re.compile(command[command.index("--ignore-filename-regex") + 1])
        self.assertIsNone(ignored.search("/work/source/" + MODULE.SESSION_FILE))
        for filename in MODULE.SESSION_SUBJECTS.values():
            self.assertIsNone(ignored.search("/work/source/" + filename))
        for name in ("lib.rs", "quality.rs", "relative.rs"):
            self.assertIsNotNone(ignored.search("/work/source/crates/nevrotick-engine/src/" + name))
        self.assertIsNotNone(ignored.search("/work/source/crates/nevrotick-engine/src/quality/tests.rs"))


class SessionWitnessTests(unittest.TestCase):
    """Synthetic supplementary-log controls; never native mutation executions."""

    def setUp(self):
        for name, value in (
            ("SESSION_REQUIRED_TESTS", frozenset({PRIVATE_TEST, EXTERNAL_TEST})),
            ("SESSION_TEST_SOURCES", {PRIVATE_TEST: MODULE.SESSION_PRIVATE, EXTERNAL_TEST: MODULE.SESSION_EXTERNAL}),
            ("SESSION_SOURCE_HELPERS", {MODULE.SESSION_PRIVATE: (), MODULE.SESSION_EXTERNAL: ()}),
        ):
            selected = patch.object(MODULE, name, value)
            selected.start()
            self.addCleanup(selected.stop)

    def example(self):
        admission, policy = fixture()
        rosters = [[PRIVATE_TEST, EXTERNAL_TEST], *[[name] for name in admission["test_inventory"]["doctests"]]]
        site = {"kind": "panic_assertion", "source": MODULE.SESSION_PRIVATE,
                "line": 7, "column": 5, "marker": "assertion failed:"}
        sites = {"candidate_sha256": policy["candidate_sha256"], "source_sha256": {},
                 "tests": {PRIVATE_TEST: [site], EXTERNAL_TEST: []}}
        descriptor = {"name": "synthetic token_issue fault", "package": "nevrotick-engine",
                      "file": MODULE.SESSION_SUBJECTS["series"], "function": None,
                      "span": {"start": {"line": 171, "column": 25}, "end": {"line": 171, "column": 27}},
                      "replacement": "!=", "genre": "BinaryOperator", "diff": "literal synthetic diff\n"}
        cases = {descriptor["name"]: (digest(descriptor), PRIVATE_TEST)}
        profile = {**MODULE.SESSION_WITNESS_LIMITS, "candidate_sha256": policy["candidate_sha256"],
                   "ordinary_receipt_sha256": policy["ordinary_evidence"]["sha256"],
                   "cases": {descriptor["name"]: {"descriptor_sha256": digest(descriptor), "test": PRIVATE_TEST}}}
        policy["mutation_witnesses"] = profile
        return admission, policy, rosters, site, sites, descriptor, cases

    def test_closed_six_cases_and_pending_binding_never_launch(self):
        self.assertEqual(len(MODULE.SESSION_WITNESS_CASES), 6)
        self.assertEqual(MODULE.SESSION_WITNESS_LIMITS,
                         {"maximum_cases": 6, "case_wall_seconds": 120, "total_wall_seconds": 360,
                          "jobs": 1, "test_threads": 1})
        names = list(MODULE.SESSION_WITNESS_CASES)
        self.assertEqual(sum("token_issue" in name for name in names), 3)
        self.assertEqual(sum("refresh_slot" in name for name in names), 1)
        self.assertEqual(sum("-= in " in name for name in names), 2)
        admission, policy, *_ = self.example()
        with patch.object(MODULE, "SESSION_WITNESS_ADMISSION", None), \
             patch("subprocess.Popen", side_effect=AssertionError("No native execution allowed")):
            with self.assertRaises(MODULE.Incomplete): MODULE.session_witness_policy(policy)
        cases = self.example()[-1]
        with patch.object(MODULE, "SESSION_WITNESS_CASES", cases), \
             patch.object(MODULE, "SESSION_WITNESS_ADMISSION", policy["mutation_witnesses"]):
            MODULE.session_witness_policy(policy)
            for key, value in [("jobs", True), ("test_threads", 1.0), ("maximum_cases", 7),
                               ("total_wall_seconds", 361), ("candidate_sha256", "f" * 64),
                               ("ordinary_receipt_sha256", "f" * 64), ("cases", {})]:
                changed = copy.deepcopy(policy); changed["mutation_witnesses"][key] = value
                with self.assertRaises(MODULE.Incomplete): MODULE.session_witness_policy(changed)

    def test_singleton_exact_healthy_failed_and_zero_match_neighbors(self):
        admission, _policy, rosters, site, _sites, _descriptor, cases = self.example()
        with patch.object(MODULE, "SESSION_ADMISSION", admission), patch.object(MODULE, "SESSION_WITNESS_CASES", cases):
            healthy = MODULE.session_singleton_execution(singleton(rosters[:1], PRIVATE_TEST), PRIVATE_TEST, rosters, failed=False)
            self.assertEqual(healthy["entered_blocks"], 1)
            self.assertEqual(healthy["expected_filtered_counts"], [1])
            self.assertEqual(healthy["doctest_blocks_executed"], 0)
            failed = MODULE.session_singleton_execution(singleton(rosters, PRIVATE_TEST, site=site), PRIVATE_TEST, rosters, failed=True)
            self.assertEqual(failed["entered_blocks"], 1)
            self.assertEqual(failed["sections"][0][1], PRIVATE_TEST)

    def test_singleton_requires_all_ordinary_groups_and_no_doctest_groups(self):
        admission, _policy, rosters, _site, _sites, _descriptor, cases = self.example()
        # Explicit membership, rather than a fixed count or a removable suffix,
        # distinguishes the full baseline's ordinary and doctest groups.
        grouped = [[PRIVATE_TEST], [EXTERNAL_TEST], *rosters[1:]]
        healthy = singleton(grouped[:2], PRIVATE_TEST)
        mixed = [[PRIVATE_TEST, grouped[2][0]], [EXTERNAL_TEST], grouped[3]]
        with patch.object(MODULE, "SESSION_ADMISSION", admission), patch.object(MODULE, "SESSION_WITNESS_CASES", cases):
            observed = MODULE.session_singleton_execution(healthy, PRIVATE_TEST, grouped, failed=False)
            self.assertEqual(observed["entered_blocks"], 2)
            self.assertEqual(observed["expected_filtered_counts"], [0, 1])
            for text, baseline in [
                (singleton(grouped[:1], PRIVATE_TEST), grouped),
                (singleton(grouped, PRIVATE_TEST), grouped),
                (healthy + execution(["foreign"]), grouped),
                (healthy, mixed),
                (healthy, [[PRIVATE_TEST], ["foreign"], *grouped[2:]]),
                (healthy, [[PRIVATE_TEST], *grouped[2:]]),
                (healthy, [[PRIVATE_TEST], [], *grouped[1:]]),
            ]:
                with self.subTest(text=text[-90:], baseline=baseline), self.assertRaises(MODULE.Incomplete):
                    MODULE.session_singleton_execution(text, PRIVATE_TEST, baseline, failed=False)

    def test_singleton_rejects_missing_extra_filtered_raw_and_malformed_evidence(self):
        admission, _policy, rosters, site, _sites, _descriptor, cases = self.example()
        healthy = singleton(rosters[:1], PRIVATE_TEST)
        failed = singleton(rosters, PRIVATE_TEST, site=site)
        controls = [(healthy.replace("1 filtered out;", "0 filtered out;", 1), False),
                    (healthy.split("test result:", 1)[0], False),
                    (healthy + healthy, False), (healthy.replace(PRIVATE_TEST, "foreign"), False),
                    (healthy + "test unfinished ...\n", False), (healthy[:-15], False),
                    (failed.replace("1 failed;", "0 failed;"), True),
                    (failed + "test foreign ... ok\n", True),
                    (failed.replace("    " + PRIVATE_TEST, "    foreign"), True),
                    (failed.replace("1 filtered out;", "2 filtered out;"), True)]
        for marker in ["Error: Entropy", "N3_HARNESS_INCOMPLETE", "error[E0001]: error", "signal: 9", "fatal runtime error"]:
            controls.append((failed + marker + "\n", True))
        with patch.object(MODULE, "SESSION_ADMISSION", admission), patch.object(MODULE, "SESSION_WITNESS_CASES", cases):
            for text, failing in controls:
                with self.subTest(text=text[-90:], failed=failing), self.assertRaises(MODULE.Incomplete):
                    MODULE.session_singleton_execution(text, PRIVATE_TEST, rosters, failed=failing)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.session_singleton_execution(healthy, PRIVATE_TEST, [[PRIVATE_TEST], *rosters[1:]], failed=False)

    def test_singleton_reuses_source_cause_but_main_still_rejects_filtered_log(self):
        admission, _policy, rosters, site, sites, descriptor, cases = self.example()
        rows = {"outcomes": [{"scenario": {"Mutant": {key: value for key, value in descriptor.items() if key != "diff"}},
                               "summary": "CaughtMutant", "log_path": "caught.log"}]}
        text = singleton(rosters, PRIVATE_TEST, site=site)
        with tempfile.TemporaryDirectory() as temporary, patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE, "SESSION_WITNESS_CASES", cases):
            directory = Path(temporary); (directory / "caught.log").write_text(text)
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_session_caught_causes(directory, rows, sites, subject="series")
            observed = MODULE.validate_session_caught_causes(directory, rows, sites, subject="series", witness=(PRIVATE_TEST, rosters))
            self.assertEqual(len(observed["checked_caught_logs"][descriptor["name"]]["causes"]), 1)
            for changed in [text.replace(site["source"], "foreign.rs"), text.replace(":7:5:", ":8:5:"),
                            text.replace("assertion failed:", "unexpected panic")]:
                (directory / "caught.log").write_text(changed)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_caught_causes(directory, rows, sites, subject="series", witness=(PRIVATE_TEST, rosters))

    def test_witness_phases_require_exact_actual_argv_status_and_duration(self):
        # The first tool separator is consumed by clap. Cargo receives its own
        # separator after the filter; cargo-mutants inserts no separator here.
        self.assertEqual(witness_phases(PRIVATE_TEST)[-1]["argv"],
                         ["cargo", "test", "--verbose", "--package=nevrotick-engine@0.0.0",
                          "--message-format=json", "--frozen", PRIVATE_TEST, "--", "--exact", "--test-threads=1"])
        for failed in [False, True]:
            row = {"phase_results": witness_phases(PRIVATE_TEST, failed=failed)}
            MODULE.session_witness_phases(row, PRIVATE_TEST, failed=failed)
            controls = []
            for field, value in [("argv", ["cargo", "test"]), ("duration", -1), ("duration", float("inf")),
                                 ("process_status", "Timeout"), ("process_status", {"Failure": 9}),
                                 ("process_status", {"Failure": 101.0})]:
                changed = copy.deepcopy(row); changed["phase_results"][-1][field] = value; controls.append(changed)
            controls.extend([{"phase_results": []}, {"phase_results": [None]}])
            for changed in controls:
                with self.assertRaises(MODULE.Incomplete): MODULE.session_witness_phases(changed, PRIVATE_TEST, failed=failed)

    def test_main_eligibility_requires_reviewed_identity_complete_roster_and_reason(self):
        _admission, _policy, rosters, site, sites, descriptor, cases = self.example()
        text = failure(PRIVATE_TEST, site).replace("running 1 test\n", "running 2 tests\n")
        text = text.replace("failures:\n", f"test {EXTERNAL_TEST} ... ok\n\nfailures:\n", 1)
        text = text.replace("0 passed; 1 failed;", "1 passed; 1 failed;")
        # The raw error remains explicitly uncredited, despite a separate valid assertion.
        text = text.replace("\nfailures:\n\n    ", "\nError: DuplicateToken\n\nfailures:\n\n    ")
        reason = "Session failure contains an unrelated native, driver or fixture error."
        with patch.object(MODULE, "SESSION_WITNESS_CASES", cases):
            MODULE.session_witness_main_eligibility(text, descriptor, reason, rosters, sites)
            for changed, fault, why in [(text + "error[E0123]: compiler failure\n", descriptor, reason),
                                        (text.replace("Error: DuplicateToken", "Error: Entropy"), descriptor, reason),
                                        (text.replace(EXTERNAL_TEST, "foreign"), descriptor, reason),
                                        (text[:-50], descriptor, reason), (text, {**descriptor, "diff": "foreign"}, reason),
                                        (text, descriptor, "different integrity failure")]:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.session_witness_main_eligibility(changed, fault, why, rosters, sites)

    def test_rejected_main_harness_sections_require_global_containment_and_exact_causes(self):
        _admission, _policy, rosters, site, sites, descriptor, _cases = self.example()
        reason = "Session native log contains compiler, abort or harness failure."
        shapes = [
            ("refresh_slot", "series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions",
             "N3_HARNESS_CAUSE_CAPTURE case=series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions parent=7 limit_bytes=16384\n"
             "N3_HARNESS_INCOMPLETE\n"
             "N3_HARNESS_CAUSE N3 named child FAILED/101 lacked assertion or semantic evidence; bounded cause captures retained\n"
             'Error: Custom { kind: Other, error: "N3 named child FAILED/101 lacked assertion or semantic evidence; bounded cause captures retained" }\n'),
            ("handle_series_read", "series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor",
             "N3_HARNESS_INCOMPLETE\nN3_HARNESS_CAUSE owned process cleanup incomplete\n"
             "N3_HARNESS_INCOMPLETE\n"
             "N3_HARNESS_CAUSE N3 owned child global reap deadline exceeded; N3 owned cleanup also failed: N3 owned child global reap deadline exceeded\n"
             'Error: Custom { kind: Other, error: "N3 owned child global reap deadline exceeded; N3 owned cleanup also failed: N3 owned child global reap deadline exceeded" }\n'),
        ]
        for function, target, body in shapes:
            fault = {**descriptor, "name": "synthetic " + function + " fault"}
            cases = {fault["name"]: (digest(fault), PRIVATE_TEST)}
            expected_rosters = [[PRIVATE_TEST, target], *rosters[1:]]
            expected_sites = copy.deepcopy(sites); expected_sites["tests"][target] = []
            text = (f"running 2 tests\ntest {PRIVATE_TEST} ... FAILED\ntest {target} ... FAILED\n\nfailures:\n\n"
                    f"---- {PRIVATE_TEST} stdout ----\nthread '{PRIVATE_TEST}' (7) panicked at {site['source']}:7:5:\nassertion failed:\n\n"
                    f"---- {target} stdout ----\n{body}\nfailures:\n\n    {PRIVATE_TEST}\n    {target}\n\n"
                    "test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")
            with patch.object(MODULE, "SESSION_WITNESS_CASES", cases):
                MODULE.session_witness_main_eligibility(text, fault, reason, expected_rosters, expected_sites)
                with self.assertRaises(MODULE.Incomplete): MODULE.session_complete_failures(text)
                errors = [line for line in body.splitlines() if line.startswith("Error: ")]
                controls = [text + "thread 'foreign' panicked at foreign.rs:1:1:\nforeign panic\n",
                            text + "N3_HARNESS_CAUSE foreign native setup failure\n",
                            text + errors[0] + "\n", text + "N3_HARNESS_INCOMPLETE\n",
                            text.replace("N3_HARNESS_CAUSE ", "N3_HARNESS_CAUSE foreign ", 1),
                            text.replace("N3_HARNESS_INCOMPLETE\n", "N3_HARNESS_INCOMPLETE\nN3_HARNESS_INCOMPLETE\n", 1),
                            text.replace("N3_HARNESS_INCOMPLETE\n", "", 1) + "N3_HARNESS_INCOMPLETE\n",
                            text.replace(errors[0] + "\n", "", 1) + errors[0] + "\n"]
                if function == "refresh_slot":
                    controls.extend([text.replace("parent=7", "parent=0"), text.replace("limit_bytes=16384", "limit_bytes=99999"),
                                     text.replace("case=" + target, "case=foreign")])
                for changed in controls:
                    with self.subTest(function=function, tail=changed[-100:]), self.assertRaises(MODULE.Incomplete):
                        MODULE.session_witness_main_eligibility(changed, fault, reason, expected_rosters, expected_sites)
                if function == "handle_series_read":
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.session_witness_main_eligibility(text, fault,
                            "Session panic is not its named source-owned semantic assertion.", expected_rosters, expected_sites)

    def test_second_843_shape_requires_exact_raw_panic_and_completed_counter_relay(self):
        admission, _policy, rosters, _site, sites, descriptor, _cases = self.example()
        native = "series::tests::selected_native_datagram_after_retirement_is_charged_without_publication"
        counter = MODULE.SESSION_WITNESS_COUNTER_TEST
        primary = {"kind": "panic_assertion", "source": "crates/nevrotick-engine/tests/private/series.rs",
                   "line": 561, "column": 5, "marker": "synchronous series counter operations must not panic",
                   "exact_message": True}
        bridge = {"kind": "child_bridge", "source": MODULE.SERIES_PROCESS_SOURCE,
                  "line": 430, "column": 9, "marker": "N3 exact child semantic failure assertion"}
        sites["tests"].update({native: [], counter: [primary, bridge]})
        admission["semantic_tests"].update({native: {}, counter: {}})
        fault = {**descriptor, "name": "synthetic -= in handle_series_read fault",
                 "span": {"start": {"line": 843, "column": 27}, "end": {"line": 843, "column": 29}}}
        cases = {fault["name"]: (digest(fault), counter)}
        rosters = [[native, counter], *rosters[1:]]
        relay = "N3_SEMANTIC_EVIDENCE test assertion/panic at crates/nevrotick-engine/tests/private/series.rs:561:5"
        text = (f"running 2 tests\ntest {native} ... FAILED\ntest {counter} ... FAILED\n\nfailures:\n\n"
                f"---- {native} stdout ----\nthread '{native}' (7) panicked at crates/nevrotick-engine/src/series.rs:843:13:\n"
                "attempt to subtract with overflow\n\n"
                f"---- {counter} stdout ----\n{relay}\n\nthread '{counter}' (8) panicked at "
                "crates/nevrotick-engine/src/../tests/private/series_process.rs:430:9:\n"
                f"N3 exact child semantic failure assertion\n\nfailures:\n\n    {native}\n    {counter}\n\n"
                "test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")
        reason = "Session panic is not its named source-owned semantic assertion."
        with patch.object(MODULE, "SESSION_WITNESS_CASES", cases), patch.object(MODULE, "SESSION_ADMISSION", admission):
            MODULE.session_witness_main_eligibility(text, fault, reason, rosters, sites)
            with tempfile.TemporaryDirectory() as temporary:
                directory = Path(temporary); (directory / "fault.log").write_text(text)
                row = {"summary": "CaughtMutant", "scenario": {"Mutant": {k:v for k,v in fault.items() if k != "diff"}},
                       "log_path": "fault.log"}
                with self.assertRaisesRegex(MODULE.Incomplete, "not its named source-owned"):
                    MODULE.validate_session_caught_causes(directory, {"outcomes": [row]}, sites, subject="series")
            controls = [text.replace(":843:13:", ":843:14:"), text.replace("attempt to subtract with overflow", "foreign panic"),
                        text.replace(":430:9:", ":431:9:"), text.replace(relay + "\n", ""),
                        text.replace(relay, relay.replace(":561:5", ":562:5")),
                        text.replace(relay, "N3_SEMANTIC_EVIDENCE independent fact: foreign"),
                        text.replace(relay, relay + "\n" + relay),
                        text.replace(native, "foreign"), text.replace(counter, "foreign"),
                        text + "thread 'foreign' panicked at foreign.rs:1:1:\nforeign panic\n",
                        text + "Error: Entropy\n", text + relay + "\n", text + "N3_HARNESS_CAUSE foreign\n",
                        text.replace("attempt to subtract with overflow", "attempt to subtract with overflow\nN3_HARNESS_INCOMPLETE")]
            for changed in controls:
                with self.subTest(tail=changed[-90:]), self.assertRaises(MODULE.Incomplete):
                    MODULE.session_witness_main_eligibility(changed, fault, reason, rosters, sites)
            for absent in (primary, bridge):
                changed = copy.deepcopy(sites); changed["tests"][counter].remove(absent)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.session_witness_main_eligibility(text, fault, reason, rosters, changed)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.session_witness_main_eligibility(text, fault,
                    "Session native log contains compiler, abort or harness failure.", rosters, sites)

    def test_synthetic_executor_pair_has_no_full_score_and_rejects_wrong_diff(self):
        admission, policy, rosters, site, sites, descriptor, cases = self.example()
        protected, checks, launches = [], [], []
        profile = policy["mutation_witnesses"]
        with tempfile.TemporaryDirectory() as temporary, patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE, "SESSION_WITNESS_CASES", cases), patch.object(MODULE, "SESSION_WITNESS_ADMISSION", profile), \
             patch("subprocess.Popen", side_effect=AssertionError("Synthetic controls cannot launch native work")):
            work = Path(temporary)
            report = {"lanes": [], "tools_sha256": {"cargo-mutants": "5" * 64}}
            owned = SimpleNamespace(work=work, tools=work / "tools", report=report,
                                    identity={"source.rs": "6" * 64}, config_sha256="7" * 64,
                                    protect=lambda path: protected.append(path), check=lambda: checks.append(True))
            wrong_diff = [False]
            def run(label, command, timeout):
                launches.append((label, command, timeout))
                selected_output = command[command.index("--output") + 1].removeprefix("/work/")
                directory = work / selected_output / "mutants.out"; directory.mkdir(parents=True, exist_ok=True)
                (directory / "baseline.log").write_text(singleton(rosters[:1], PRIVATE_TEST))
                (directory / "caught.log").write_text(singleton(rosters, PRIVATE_TEST, site=site))
                (directory / "fault.diff").write_text("wrong" if wrong_diff[0] else descriptor["diff"])
                rows = [{"scenario": "Baseline", "summary": "Success", "phase_results": witness_phases(PRIVATE_TEST), "log_path": "baseline.log"},
                        {"scenario": {"Mutant": {key: value for key, value in descriptor.items() if key != "diff"}},
                         "summary": "CaughtMutant", "phase_results": witness_phases(PRIVATE_TEST, failed=True),
                         "log_path": "caught.log", "diff_path": "fault.diff"}]
                (directory / "outcomes.json").write_text(json.dumps({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "caught": 1, "missed": 0, "unviable": 0, "timeout": 0, "success": 0,
                    "end_time": "2000-01-01T00:00:01Z", "outcomes": rows}))
                report["lanes"].append({"lane": label, "command": ["/usr/bin/bwrap", "--", *command], "exit_code": 0, "failure": None})
            owned.run = run
            import time
            observed = MODULE.session_execute_witness(owned, policy, descriptor, rosters, sites, time.monotonic() + 10, 0)
            self.assertNotIn("main_rejection_preserved", observed); self.assertNotIn("main_log_path", observed)
            self.assertIs(observed["qualification_passed"], False); self.assertIs(observed["mutation_gate_claim"], False)
            self.assertEqual(observed["execution_attempts"], {"baselines": 1, "mutants": 1})
            self.assertEqual(len(launches), 1); self.assertEqual(len(protected), 1); self.assertEqual(checks, [True])
            command = launches[0][1]
            self.assertEqual(command[-5:], ["--", PRIVATE_TEST, "--", "--exact", "--test-threads=1"])
            self.assertEqual(command[command.index("--re") + 1], "^synthetic token_issue fault$")
            self.assertEqual(command[command.index("--jobs") + 1], "1")
            wrong_diff[0] = True
            with self.assertRaises(MODULE.Incomplete):
                MODULE.session_execute_witness(owned, policy, descriptor, rosters, sites, time.monotonic() + 10, 1)

    def test_main_adapter_preserves_actual_identity_and_executor_validation(self):
        _admission, policy, rosters, _site, sites, descriptor, _cases = self.example()
        row = {"scenario": {"Mutant": {key: value for key, value in descriptor.items() if key != "diff"}},
               "summary": "CaughtMutant", "log_path": "actual-rejected-main.log"}
        evidence = {"qualification_passed": False, "mutation_gate_claim": False, "opaque_evidence": {"bound": True}}
        owner = object()
        with patch.object(MODULE, "session_execute_witness", return_value=evidence) as execute:
            result = MODULE.session_run_witness(owner, policy, descriptor, row, "actual closed rejection", rosters, sites, 123.0, 0)
            execute.assert_called_once_with(owner, policy, descriptor, rosters, sites, 123.0, 0)
            self.assertEqual(result, {**evidence, "main_rejection_preserved": "actual closed rejection",
                                      "main_log_path": "actual-rejected-main.log"})
            self.assertIs(result["opaque_evidence"], evidence["opaque_evidence"])
            execute.reset_mock()
            for altered in [{"log_path": "fabricated"}, {**row, "summary": "MissedMutant"},
                            {**row, "scenario": {"Mutant": {"name": "foreign"}}}]:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.session_run_witness(owner, policy, descriptor, altered, "actual closed rejection", rosters, sites, 123.0, 0)
                execute.assert_not_called()
            execute.side_effect = MODULE.Incomplete("exact executor refusal")
            with self.assertRaisesRegex(MODULE.Incomplete, "exact executor refusal"):
                MODULE.session_run_witness(owner, policy, descriptor, row, "actual closed rejection", rosters, sites, 123.0, 0)

    def test_full_campaign_integrity_precedes_any_supplemental_execution(self):
        admission, policy, rosters, site, sites, descriptor, cases = self.example()
        baseline_text = "".join(execution(names) for names in rosters)
        main_text = failure(PRIVATE_TEST, site).replace("running 1 test\n", "running 2 tests\n")
        main_text = main_text.replace("failures:\n", f"test {EXTERNAL_TEST} ... ok\n\nfailures:\n", 1)
        main_text = main_text.replace("0 passed; 1 failed;", "1 passed; 1 failed;")
        main_text = main_text.replace("\nfailures:\n\n    ", "\nError: DuplicateToken\n\nfailures:\n\n    ")
        mutant = {key: value for key, value in descriptor.items() if key != "diff"}
        rows = [{"scenario": "Baseline", "summary": "Success", "phase_results": phases(), "log_path": "baseline.log"},
                {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": phases(),
                 "log_path": "caught.log", "diff_path": "fault.diff"}]
        rows[1]["phase_results"][-1]["process_status"] = {"Failure": 101}
        outcomes = {"cargo_mutants_version": "27.1.0", "total_mutants": 1, "caught": 1,
                    "missed": 0, "unviable": 0, "timeout": 0, "success": 0, "outcomes": rows}
        with tempfile.TemporaryDirectory() as temporary, patch.object(MODULE, "SESSION_ADMISSION", admission), \
             patch.object(MODULE, "SESSION_WITNESS_CASES", cases), \
             patch.object(MODULE, "session_semantic_assertion_sites", return_value=sites), \
             patch.object(MODULE, "session_run_witness", return_value={"synthetic_only": True}) as witness, \
             patch("subprocess.Popen", side_effect=AssertionError("No native work in integrity controls")):
            work = Path(temporary); directory = work / "mutation/series/mutants.out"; directory.mkdir(parents=True)
            owned = SimpleNamespace(work=work, source=work / "source", check=lambda: None)
            def prepare(changed=None, *, diff=None, log=None, baseline=None):
                (directory / "outcomes.json").write_text(json.dumps(outcomes if changed is None else changed))
                (directory / "fault.diff").write_text(descriptor["diff"] if diff is None else diff)
                (directory / "caught.log").write_text(main_text if log is None else log)
                (directory / "baseline.log").write_text(baseline_text if baseline is None else baseline)
                return {"mutation_enumeration": {"series": {"raw_output": json.dumps([descriptor])}}}
            import time
            report = prepare()
            result = MODULE.validate_session_campaign(owned, policy, "series", {"exit_code": 0, "failure": None}, report, time.monotonic() + 10)
            admission_result = result["caught_cause_admission"]
            self.assertEqual(admission_result["unique_verified_caught"], 1)
            self.assertEqual(admission_result["rejected_full_suite_logs"][descriptor["name"]]["credit"], 0)
            self.assertEqual(admission_result["supplemental_execution_attempts"], 2)
            witness.assert_called_once(); witness.reset_mock()
            malformed = copy.deepcopy(outcomes); malformed["outcomes"][1]["scenario"]["Mutant"]["replacement"] = "foreign"
            incomplete = copy.deepcopy(outcomes); incomplete["outcomes"] = incomplete["outcomes"][:1]
            for arguments in [{"changed": malformed}, {"changed": incomplete}, {"diff": "foreign"},
                              {"log": main_text[:-50]}, {"baseline": baseline_text[:-50]},
                              {"log": main_text + "error[E0001]: unknown compiler failure\n"}]:
                report = prepare(**arguments)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_session_campaign(owned, policy, "series", {"exit_code": 0, "failure": None}, report, time.monotonic() + 10)
                witness.assert_not_called()
            report = prepare()
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_session_campaign(owned, policy, "series", {"exit_code": 0, "failure": "interrupted"}, report, time.monotonic() + 10)
            witness.assert_not_called()
            report = prepare()
            witness.side_effect = MODULE.Incomplete("synthetic incomplete witness")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_session_campaign(owned, policy, "series", {"exit_code": 0, "failure": None}, report, time.monotonic() + 10)
            saved = report["baseline_execution"]["series"]["caught_cause_admission"]
            self.assertEqual(saved["supplemental_attempts"][descriptor["name"]]["status"], "incomplete")
            self.assertEqual(saved["supplemental_witnesses"], {})
            self.assertNotIn("unique_verified_caught", saved)


if __name__ == "__main__":
    unittest.main()
