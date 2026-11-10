"""Five-owner admission/custody controls; no Rust or analysis tools are executed."""
from __future__ import annotations

import copy
from contextlib import ExitStack
import difflib
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("baseline_feedback_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
ROLE = MODULE._baseline_feedback_role()
OWNERS = MODULE.BASELINE_FEEDBACK_SUBJECTS


def policy():
    """Use the actual roster document as test data, never as executed evidence."""
    result = json.loads((ROOT / MODULE.BASELINE_FEEDBACK_POLICY).read_text())
    result["mutation_inventories"] = {name: {"count": 200, "sha256": "b" * 64} for name in OWNERS}
    return result


def successes(names):
    """Literal native-format successes with one complete harness footer."""
    return (f"running {len(names)} tests\n" + "".join(f"test {name} ... ok\n" for name in names)
            + f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


def receipt(owner, p, identity, admitted):
    """Synthetic terminal metadata; independent raw validators remain mandatory."""
    return {"cohort_id": owner, "status": "passed", "selected_lane": "mutation", "cohort_complete": True,
            "subjects": {owner: OWNERS[owner]}, "complete_subjects": OWNERS,
            "candidate_sha256": p["candidate_sha256"], "source_sha256": identity,
            "tool_admission_profile": "stable-core", "all_reviewed_inventory_bindings": p["mutation_inventories"],
            "analysis_policy_sha256": admitted["policy_sha256"],
            "analysis_admission": {"profile": "stable-core", "inventories": admitted["inventories"]},
            "analysis_admission_after": {"profile": "stable-core", "inventories": admitted["inventories"], "unchanged": True}}


def compiler_fixture(directory):
    """Owned chart diagnostic with exact bytes; no compiler outcome is presumed."""
    source = Path(directory)
    values = {"crates/time-core/src/lib.rs": b"pub mod chart;\n", OWNERS["chart"]: b"pub fn value() -> u8 { 1 }\n"}
    for name, data in values.items():
        path = source / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    identity = {name: hashlib.sha256(data).hexdigest() for name, data in values.items()}
    workspace = "/work/tmp/cargo-mutants-source-ABC123.tmp"
    record = {"reason": "compiler-message", "package_id": "path+file://" + workspace + "/crates/time-core#0.0.0",
        "manifest_path": workspace + "/crates/time-core/Cargo.toml",
        "target": {"kind": ["lib"], "crate_types": ["lib"], "name": "time_core",
                   "src_path": workspace + "/crates/time-core/src/lib.rs", "edition": "2024",
                   "doc": True, "doctest": True, "test": True},
        "message": {"$message_type": "diagnostic", "level": "error", "message": "mismatched types",
            "code": {"code": "E0308", "explanation": None}, "children": [], "rendered": None,
            "spans": [{"file_name": OWNERS["chart"], "is_primary": True, "line_start": 1, "line_end": 1,
                       "column_start": 1, "column_end": 2, "byte_start": 0, "byte_end": 1,
                       "text": [{"text": "pub fn value() -> u8 { 1 }", "highlight_start": 1, "highlight_end": 2}]}]}}
    return source, identity, record


def observed_rate_compiler_case(directory, line):
    """Reconstruct fourteen captured 27.1.0 payloads and unrendered rustc 1.99.1 records.

    Only repository-owned source/manifest bytes are read. No private receipt,
    compiler, mutation worker or historical four-owner exception is borrowed.
    """
    source = Path(directory)
    filename = (OWNERS["chart"] if line in (427, 433, 439, 445) else
                OWNERS["phase_plan"] if line in (426, 517, 2294, 2299) else
                OWNERS["frequency_state"] if line in (474, 743, 811) else OWNERS["rate_delivery"])
    identity = {}
    for name in (filename, "crates/time-core/src/lib.rs", "policy/tool-manifest.json"):
        data = (ROOT / name).read_bytes()
        path = source / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        identity[name] = hashlib.sha256(data).hexdigest()
    before = (source / filename).read_text().splitlines(keepends=True)
    after = before.copy()
    if line in (426, 427, 433, 439, 445, 474, 517, 2294, 2299):
        replacement = "1.try_into().unwrap()"
        getter = {427: "FixtureChartFrames::source_frame", 433: "FixtureChartFrames::source_generation",
                  439: "FixtureChartFrames::target_frame", 445: "FixtureChartFrames::target_generation",
                  426: "FixturePhaseProvenance::emitted_round", 517: "FixturePhaseProposal<'_>::ordinal",
                  2294: "FixtureBaselineProposal<'_>::ordinal", 2299: "FixtureBaselineProposal<'_>::emitted_round",
                  474: "FixtureFrequencyHistory::emitted_round"}[line]
        description = "replace " + getter + " -> NonZeroU64 with " + replacement
        function = {"function_name": getter, "return_type": "-> NonZeroU64",
            "span": {"start": {"line": line - 3, "column": 5}, "end": {"line": line + 1, "column": 6}}}
        end_column = {426: 19, 427: 26, 433: 31, 439: 26, 445: 31, 474: 25, 517: 28, 2294: 28, 2299: 26}[line]
        span = {"start": {"line": line, "column": 9}, "end": {"line": line, "column": end_column}}
        after[line - 1] = "        " + replacement + " /* ~ changed by cargo-mutants ~ */\n"
        offset = {427: 18207, 433: 18392, 439: 18585, 445: 18777,
                  426: 17517, 517: 20838, 2294: 93352, 2299: 93526, 474: 18716}[line]
        expected = [
            ("E0658", "cannot call conditionally-const method `<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions", 11, 21, offset + 2, offset + 12),
            (None, "`TryInto` is not yet stable as a const trait", 9, 21, offset, offset + 12),
            ("E0015", "cannot call non-const method `Result::<NonZero<u64>, TryFromIntError>::unwrap` in constant functions", 22, 30, offset + 13, offset + 21),
        ]
    elif line == 461:
        replacement = "Some(1.try_into().unwrap())"
        description = "replace FixtureRateSettlementCursor::next_ordinal -> Option<NonZeroU64> with " + replacement
        function = {"function_name": "FixtureRateSettlementCursor::next_ordinal", "return_type": "-> Option<NonZeroU64>",
                    "span": {"start": {"line": 458, "column": 5}, "end": {"line": 465, "column": 6}}}
        span = {"start": {"line": 461, "column": 9}, "end": {"line": 464, "column": 10}}
        after[460:464] = ["        Some(1.try_into().unwrap()) /* ~ changed by cargo-mutants ~ */\n"]
        expected = [
            ("E0658", "cannot call conditionally-const method `<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions", 16, 26, 18211, 18221),
            (None, "`TryInto` is not yet stable as a const trait", 14, 26, 18209, 18221),
            ("E0015", "cannot call non-const method `Result::<NonZero<u64>, TryFromIntError>::unwrap` in constant functions", 27, 35, 18222, 18230),
        ]
    elif line in (605, 610):
        replacement = "||"
        description = "replace && with || in known_relationship"
        function = {"function_name": "known_relationship", "return_type": "-> Result<(), FixtureRateDeliveryError>",
                    "span": {"start": {"line": 585, "column": 1}, "end": {"line": 615, "column": 2}}}
        span = {"start": {"line": line, "column": 9}, "end": {"line": line, "column": 11}}
        after[line - 1] = after[line - 1].replace("&&", "|| /* ~ changed by cargo-mutants ~ */", 1)
        offset = 24026 if line == 605 else 24186
        expected = [(None, "`||` operators are not supported in let chain conditions", 9, 11, offset, offset + 2)]
    elif line in (743, 811):
        replacement = "||"
        name = "FrequencyState::no_ready" if line == 743 else "FrequencyState::refresh_previous"
        description = "replace && with || in " + name
        first, last = (737, 754) if line == 743 else (800, 821)
        function = {"function_name": name, "return_type": "",
            "span": {"start": {"line": first, "column": 5}, "end": {"line": last, "column": 6}}}
        span = {"start": {"line": line, "column": 13}, "end": {"line": line, "column": 15}}
        after[line - 1] = after[line - 1].replace("&&", "|| /* ~ changed by cargo-mutants ~ */", 1)
        offset = 29033 if line == 743 else 31457
        expected = [(None, "`||` operators are not supported in let chain conditions", 13, 15, offset, offset + 2)]
    else:
        raise ValueError("No observed baseline compiler case at this line.")
    mutant = {"file": filename, "function": function, "genre": "FnValue" if line in (426, 427, 433, 439, 445, 461, 474, 517, 2294, 2299) else "BinaryOperator",
              "name": f"{filename}:{line}:{span['start']['column']}: {description}", "package": "time-core", "replacement": replacement, "span": span}
    diff = "".join(difflib.unified_diff(before, after, fromfile=filename, tofile=description, n=8))
    workspace = "/work/tmp/cargo-mutants-source-ABC123.tmp"
    records = []
    for code, message, column, end, offset, byte_end in expected * 2:
        records.append({"reason": "compiler-message", "package_id": "path+file://" + workspace + "/crates/time-core#0.0.0",
            "manifest_path": workspace + "/crates/time-core/Cargo.toml",
            "target": {"kind": ["lib"], "crate_types": ["lib"], "name": "time_core",
                       "src_path": workspace + "/crates/time-core/src/lib.rs", "edition": "2024",
                       "doc": True, "doctest": True, "test": True},
            "message": {"$message_type": "diagnostic", "level": "error", "message": message,
                "code": None if code is None else {"code": code, "explanation": None}, "children": [], "rendered": None,
                "spans": [{"file_name": filename, "is_primary": True, "line_start": line, "line_end": line,
                    "column_start": column, "column_end": end, "byte_start": offset, "byte_end": byte_end,
                    "text": [{"text": after[line - 1].rstrip("\n"), "highlight_start": column, "highlight_end": end}]}]}})
    return source, identity, mutant, diff, records


def unviable_phase(mutant, records):
    """Literal owned Build failure; no Test phase or behavioral kill is manufactured."""
    argv = ["cargo", "test", "--no-run", "--verbose", "--package=time-core@0.0.0", "--message-format=json", "--frozen"]
    outcome = {"scenario": {"Mutant": mutant}, "summary": "Unviable", "phase_results": [
        {"phase": "Build", "argv": argv, "duration": 0.01, "process_status": {"Failure": 101}}]}
    raw = "*** " + " ".join(argv) + "\n" + "\n".join(json.dumps(record) for record in records)
    raw += '\n{"reason":"build-finished","success":false}\n'
    return outcome, raw


# Frozen native descriptors for parser controls only; no private evidence is read.
BASELINE_REVIEW_MUTANTS = {'BF-STATE-GROWTH-NORMAL-POSITIVE': {'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:180:19: '
                                             'replace > with >= in profile_valid',
                                     'package': 'time-core',
                                     'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                                     'function': {'function_name': 'profile_valid',
                                                  'return_type': '-> bool',
                                                  'span': {'start': {'line': 173, 'column': 1},
                                                           'end': {'line': 183, 'column': 2}}},
                                     'span': {'start': {'line': 180, 'column': 19},
                                              'end': {'line': 180, 'column': 20}},
                                     'replacement': '>=',
                                     'genre': 'BinaryOperator'},
 'BF-STATE-REBASE-INTERMEDIATE-LABELS': {'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:1097:9: '
                                                 'replace || with && in rebase_model',
                                         'package': 'time-core',
                                         'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                                         'function': {'function_name': 'rebase_model',
                                                      'return_type': '-> Result<Model, '
                                                                     'FixtureFrequencyError>',
                                                      'span': {'start': {'line': 1080, 'column': 1},
                                                               'end': {'line': 1132, 'column': 2}}},
                                         'span': {'start': {'line': 1097, 'column': 9},
                                                  'end': {'line': 1097, 'column': 11}},
                                         'replacement': '&&',
                                         'genre': 'BinaryOperator'},
 'BF-STATE-REBASE-STEP-GENERATION-SOURCE-FRAME': {'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:1098:9: '
                                                          'replace || with && in rebase_model',
                                                  'package': 'time-core',
                                                  'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                                                  'function': {'function_name': 'rebase_model',
                                                               'return_type': '-> Result<Model, '
                                                                              'FixtureFrequencyError>',
                                                               'span': {'start': {'line': 1080,
                                                                                  'column': 1},
                                                                        'end': {'line': 1132,
                                                                                'column': 2}}},
                                                  'span': {'start': {'line': 1098, 'column': 9},
                                                           'end': {'line': 1098, 'column': 11}},
                                                  'replacement': '&&',
                                                  'genre': 'BinaryOperator'},
 'BF-STATE-REBASE-SOURCE-LABELS': {'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:1099:9: '
                                           'replace || with && in rebase_model',
                                   'package': 'time-core',
                                   'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                                   'function': {'function_name': 'rebase_model',
                                                'return_type': '-> Result<Model, FixtureFrequencyError>',
                                                'span': {'start': {'line': 1080, 'column': 1},
                                                         'end': {'line': 1132, 'column': 2}}},
                                   'span': {'start': {'line': 1099, 'column': 9},
                                            'end': {'line': 1099, 'column': 11}},
                                   'replacement': '&&',
                                   'genre': 'BinaryOperator'},
 'BF-STATE-REBASE-SOURCE-GENERATION-TARGET-FRAME': {'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:1100:9: '
                                                            'replace || with && in rebase_model',
                                                    'package': 'time-core',
                                                    'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                                                    'function': {'function_name': 'rebase_model',
                                                                 'return_type': '-> Result<Model, '
                                                                                'FixtureFrequencyError>',
                                                                 'span': {'start': {'line': 1080,
                                                                                    'column': 1},
                                                                          'end': {'line': 1132,
                                                                                  'column': 2}}},
                                                    'span': {'start': {'line': 1100, 'column': 9},
                                                             'end': {'line': 1100, 'column': 11}},
                                                    'replacement': '&&',
                                                    'genre': 'BinaryOperator'},
 'BF-STATE-REBASE-TARGET-LABELS': {'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:1101:9: '
                                           'replace || with && in rebase_model',
                                   'package': 'time-core',
                                   'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                                   'function': {'function_name': 'rebase_model',
                                                'return_type': '-> Result<Model, FixtureFrequencyError>',
                                                'span': {'start': {'line': 1080, 'column': 1},
                                                         'end': {'line': 1132, 'column': 2}}},
                                   'span': {'start': {'line': 1101, 'column': 9},
                                            'end': {'line': 1101, 'column': 11}},
                                   'replacement': '&&',
                                   'genre': 'BinaryOperator'},
 'BF-NUMERIC-ANCHOR-BOUND-ABS-SIGN': {'name': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs:980:64: '
                                              'delete - in FrequencyAnchor::rebase',
                                      'package': 'time-core',
                                      'file': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs',
                                      'function': {'function_name': 'FrequencyAnchor::rebase',
                                                   'return_type': '-> Result<Self, '
                                                                  'FixtureFrequencyNumericError>',
                                                   'span': {'start': {'line': 960, 'column': 5},
                                                            'end': {'line': 987, 'column': 6}}},
                                      'span': {'start': {'line': 980, 'column': 64},
                                               'end': {'line': 980, 'column': 65}},
                                      'replacement': '',
                                      'genre': 'UnaryOperator'},
 'BF-NUMERIC-INTERVAL-WHOLE-ZERO-PRODUCT': {'name': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs:1551:27: '
                                                    'replace || with && in Interval::multiply',
                                            'package': 'time-core',
                                            'file': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs',
                                            'function': {'function_name': 'Interval::multiply',
                                                         'return_type': '-> Result<Self, '
                                                                        'FixtureFrequencyNumericError>',
                                                         'span': {'start': {'line': 1543, 'column': 5},
                                                                  'end': {'line': 1576, 'column': 6}}},
                                            'span': {'start': {'line': 1551, 'column': 27},
                                                     'end': {'line': 1551, 'column': 29}},
                                            'replacement': '&&',
                                            'genre': 'BinaryOperator'},
 'BF-PHASE-PROFILE-NORMAL-POSITIVE': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:187:42: '
                                              'replace > with >= in profile_valid',
                                      'package': 'time-core',
                                      'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                                      'function': {'function_name': 'profile_valid',
                                                   'return_type': '-> bool',
                                                   'span': {'start': {'line': 180, 'column': 1},
                                                            'end': {'line': 188, 'column': 2}}},
                                      'span': {'start': {'line': 187, 'column': 42},
                                               'end': {'line': 187, 'column': 43}},
                                      'replacement': '>=',
                                      'genre': 'BinaryOperator'}}


def reviewed_equivalence_policy():
    """Build explicit synthetic admission data, never a product/proof receipt."""
    value = policy()
    proof_path = "tasks/phase-plan-analysis-equivalences.md"
    proof_sha256 = "e" * 64
    value["inputs_sha256"][proof_path] = proof_sha256
    entries = []
    for identifier, mutant in BASELINE_REVIEW_MUTANTS.items():
        source, _, rationale, _ = MODULE.COMMON.BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS[identifier]
        entries.append({"id": identifier, "candidate_sha256": value["candidate_sha256"],
            "source_sha256": source, "mutant": copy.deepcopy(mutant),
            "requirement": "REQ-8/10/11/14/15; CHECK-7/8/10/11/14/25; AC-19", "rationale": rationale,
            "proof": {"path": proof_path, "sha256": proof_sha256, "section": identifier,
                "reviewer": "/root; /root/handoff_final_review; /root/handoff_runner",
                "verdict": "independently-justified-equivalent"}})
    value["mutation_equivalences"] = entries
    return value


def equivalence_source_fixture(directory):
    """Real source/document hashing around a synthetic tiny product-inventory fixture.

    No candidate admission is simulated as product evidence. Only the inventory
    producer is isolated; COMMON reads all actual source and document bytes.
    """
    root = Path(directory)
    value = reviewed_equivalence_policy()
    names = {name for entry in value["mutation_equivalences"]
             for name in MODULE.COMMON.BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS[entry["id"]][3]}
    identity = {}
    for name in names:
        data = (ROOT / name).read_bytes()
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        identity[name] = hashlib.sha256(data).hexdigest()
    proof = root / "tasks/phase-plan-analysis-equivalences.md"
    proof.parent.mkdir(parents=True, exist_ok=True)
    proof.write_text("Synthetic parser mechanics only.\n\n" + "".join(
        "## " + entry["id"] + "\nSynthetic section-presence fixture.\n\n"
        for entry in value["mutation_equivalences"]))
    candidate = MODULE.COMMON.canonical_sha256(identity)
    digest = hashlib.sha256(proof.read_bytes()).hexdigest()
    value["candidate_sha256"] = candidate
    value["inputs_sha256"]["tasks/phase-plan-analysis-equivalences.md"] = digest
    for entry in value["mutation_equivalences"]:
        entry["candidate_sha256"] = candidate
        entry["proof"]["sha256"] = digest
    return root, value, identity


class BaselineFeedbackAnalysisTests(unittest.TestCase):
    """Discriminate exact role, current evidence, resource caps and five-owner custody."""

    def test_exact_role_and_default_four_owner_contract_remain_distinct(self):
        self.assertEqual(tuple(OWNERS), ("frequency_state", "numeric", "phase_plan", "rate_delivery", "chart"))
        self.assertEqual(MODULE._frequency_analysis_role(), MODULE._frequency_prior_role())
        self.assertEqual(len(MODULE._frequency_analysis_role().scope.subjects), 4)
        self.assertEqual(ROLE.coverage_tests, ("frequency_prior", "phase_plan", "rate_delivery", "chart"))
        self.assertEqual(len(ROLE.bound_inputs), 59)
        self.assertEqual(len(set(ROLE.bound_inputs)), 59)
        self.assertIn("tasks/phase-plan-analysis-equivalences.md", ROLE.bound_inputs)
        self.assertIn("scripts/tests/test_baseline_feedback_analysis.py", ROLE.bound_inputs)
        with self.assertRaises(MODULE.Incomplete):
            MODULE._frequency_analysis_role(ROLE._replace(selector="frequency_prior"))

    def test_current_review_catalog_is_complete_separate_and_not_policy_extensible(self):
        p = reviewed_equivalence_policy()
        self.assertEqual(len(p["mutation_equivalences"]), 9)
        self.assertEqual(set(BASELINE_REVIEW_MUTANTS), set(MODULE.COMMON.BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS))
        self.assertTrue(MODULE.COMMON.baseline_feedback_equivalences_valid(p))
        MODULE.COMMON.require_baseline_feedback_role(ROLE.scope, p)
        empty = copy.deepcopy(p)
        empty["mutation_equivalences"] = []
        self.assertTrue(MODULE.COMMON.baseline_feedback_equivalences_valid(empty))
        self.assertFalse(MODULE.COMMON.frequency_prior_equivalences_valid(p))
        for key in ("mutation_jobs", "mutation_test_seconds", "coverage_minimum_line_percent",
                    "mutation_minimum_viable_killed_percent"):
            self.assertEqual(p[key], policy()[key])

    def test_current_review_fields_dependencies_and_exact_complete_set_are_mandatory(self):
        p = reviewed_equivalence_policy()
        for fault in ("partial", "duplicate", "foreign", "extra-entry", "extra-field", "source", "descriptor",
                      "function", "span", "replacement", "requirement", "rationale", "candidate", "path",
                      "proof-hash", "section", "reviewer", "verdict", "chart-dependency", "roundoff-dependency"):
            changed = copy.deepcopy(p)
            entries = changed["mutation_equivalences"]
            entry = entries[0]
            if fault == "partial": entries.pop()
            elif fault == "duplicate": entries[-1] = copy.deepcopy(entry)
            elif fault == "foreign": entry["id"] = "FREQUENCY-GROWTH-NORMAL-POSITIVE"
            elif fault == "extra-entry": entries.append(copy.deepcopy(entry))
            elif fault == "extra-field": entry["unreviewed"] = True
            elif fault == "source": entry["source_sha256"] = "0" * 64
            elif fault == "descriptor": entry["mutant"]["name"] += " changed"
            elif fault == "function": entry["mutant"]["function"]["return_type"] = "-> usize"
            elif fault == "span": entry["mutant"]["span"]["start"]["column"] += 1
            elif fault == "replacement": entry["mutant"]["replacement"] = "<="
            elif fault == "requirement": entry["requirement"] = "foreign"
            elif fault == "rationale": entry["rationale"] += " policy-controlled extension"
            elif fault == "candidate": entry["candidate_sha256"] = "0" * 64
            elif fault == "path": entry["proof"]["path"] = "tasks/foreign.md"
            elif fault == "proof-hash": entry["proof"]["sha256"] = "0" * 64
            elif fault == "section": entry["proof"]["section"] = "FOREIGN"
            elif fault == "reviewer": entry["proof"]["reviewer"] = "unreviewed"
            elif fault == "verdict": entry["proof"]["verdict"] = "assumed"
            elif fault == "chart-dependency": changed["inputs_sha256"]["crates/time-core/src/regression/chart.rs"] = "0" * 64
            else: changed["inputs_sha256"]["crates/time-core/src/regression/chart/roundoff.rs"] = "0" * 64
            with self.subTest(fault=fault):
                self.assertFalse(MODULE.COMMON.baseline_feedback_equivalences_valid(changed))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.require_baseline_feedback_role(ROLE.scope, changed)

    def test_common_admission_rehashes_actual_candidate_source_and_review_document(self):
        with tempfile.TemporaryDirectory() as directory:
            root, p, identity = equivalence_source_fixture(directory)
            with patch.object(MODULE.COMMON.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, ROLE.scope, p),
                                 p["mutation_equivalences"])
                for fault in ("candidate", "source", "document"):
                    changed = copy.deepcopy(p)
                    if fault == "candidate":
                        changed["candidate_sha256"] = "0" * 64
                        for entry in changed["mutation_equivalences"]: entry["candidate_sha256"] = "0" * 64
                        self.assertTrue(MODULE.COMMON.baseline_feedback_equivalences_valid(changed))
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.COMMON.admit_mutation_equivalences(root, ROLE.scope, changed)
                    else:
                        path = root / (p["mutation_equivalences"][0]["mutant"]["file"] if fault == "source"
                                       else "tasks/phase-plan-analysis-equivalences.md")
                        before = path.read_bytes()
                        path.write_bytes(before + b"\nchanged actual bytes\n")
                        try:
                            with self.assertRaises(MODULE.Incomplete):
                                MODULE.COMMON.admit_mutation_equivalences(root, ROLE.scope, changed)
                        finally: path.write_bytes(before)

    def test_common_admission_requires_the_exact_review_section_even_with_fresh_digest(self):
        with tempfile.TemporaryDirectory() as directory:
            root, p, identity = equivalence_source_fixture(directory)
            proof = root / "tasks/phase-plan-analysis-equivalences.md"
            missing = p["mutation_equivalences"][0]["id"]
            proof.write_text(proof.read_text().replace("## " + missing + "\n", "## OTHER\n", 1))
            digest = hashlib.sha256(proof.read_bytes()).hexdigest()
            p["inputs_sha256"]["tasks/phase-plan-analysis-equivalences.md"] = digest
            for entry in p["mutation_equivalences"]: entry["proof"]["sha256"] = digest
            self.assertTrue(MODULE.COMMON.baseline_feedback_equivalences_valid(p))
            with patch.object(MODULE.COMMON.PRODUCT, "admit_product", return_value=({}, identity)), \
                 self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admit_mutation_equivalences(root, ROLE.scope, p)

    def test_equivalence_stays_actual_missed_only_and_earns_zero_kills(self):
        entry = reviewed_equivalence_policy()["mutation_equivalences"][0]
        mutant = entry["mutant"]
        neighbor = copy.deepcopy(mutant)
        neighbor.update(name=mutant["name"] + " synthetic-caught-neighbor", replacement="<")
        def outcome(summary, scenario):
            phases = []
            for phase in (["Build"] if summary == "Unviable" else ["Build", "Test"]):
                argv = ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                        "--verbose", "--package=time-core@0.0.0", "--message-format=json", "--frozen"]
                failed = summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test")
                phases.append({"phase": phase, "argv": argv, "duration": 0.01,
                               "process_status": {"Failure": 101} if failed else "Success"})
            return {"scenario": scenario, "summary": summary, "phase_results": phases}
        baseline = outcome("Success", "Baseline")
        caught = outcome("CaughtMutant", {"Mutant": neighbor})
        expected = {mutant["name"]: mutant, neighbor["name"]: neighbor}
        for summary in ("MissedMutant", "CaughtMutant", "Unviable"):
            raw = {"cargo_mutants_version": "27.1.0", "total_mutants": 2, "success": 0, "timeout": 0,
                   "caught": 2 if summary == "CaughtMutant" else 1,
                   "missed": int(summary == "MissedMutant"), "unviable": int(summary == "Unviable"),
                   "outcomes": [baseline, caught, outcome(summary, {"Mutant": mutant})]}
            arguments = (ROLE.scope, raw, expected, mutant["file"])
            if summary == "MissedMutant":
                result = MODULE.COMMON.mutation_counts(*arguments, native_exit_code=2, equivalences=[entry])
                self.assertEqual((result["killed"], result["survived"], result["excluded"]), (1, 1, 1))
                self.assertEqual((result["raw_denominator"], result["adjusted_denominator"]), (2, 1))
            else:
                with self.subTest(summary=summary), self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.mutation_counts(*arguments, native_exit_code=0, equivalences=[entry])

    def test_cli_and_executor_dispatch_select_only_the_additive_role(self):
        with patch.object(MODULE, "main_baseline_feedback", return_value=0) as selected, \
             patch.object(MODULE, "main_frequency_prior") as historical:
            self.assertEqual(MODULE.main(["--subject", "baseline_feedback", "--lane", "enumerate",
                "--tool-admission-profile", "stable-core", "--output-dir", "/owned/output"]), 0)
        selected.assert_called_once()
        historical.assert_not_called()
        with patch.object(MODULE, "execute_baseline_feedback") as selected, \
             patch.object(MODULE, "execute_frequency_prior") as historical:
            MODULE.execute(SimpleNamespace(subject="baseline_feedback"), {})
        selected.assert_called_once()
        historical.assert_not_called()

    def test_portable_admission_is_explicit_closed_and_stable_only(self):
        p = policy()
        args = SimpleNamespace(tool_admission_profile="stable-core", lane="mutation")
        self.assertEqual(MODULE.COMMON.admission_profile(args, ROLE.scope, p), "stable-core")
        for key, value in (("mutation_jobs", 8), ("mutation_process_limit", 128), ("jobs", 5),
                           ("mutation_test_seconds", 59), ("mutation_minimum_free_bytes", 5 * 1024 ** 3 - 1),
                           ("mutation_debug_info", False), ("maximum_listed_total", 2049),
                           ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                           ("external_consumer_limits", {"legal_executions": 2, "negative_compilations": 18}),
                           ("mutation_equivalences", [{}]), ("extra", 0)):
            changed = copy.deepcopy(p)
            changed[key] = value
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.require_baseline_feedback_role(ROLE.scope, changed)
        for scope in (MODULE.FREQUENCY_PRIOR_SCOPE, ROLE.scope._replace(policy="policy/foreign.json")):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.require_baseline_feedback_role(scope, p)
        for args in (SimpleNamespace(tool_admission_profile="full", lane="mutation"),
                     SimpleNamespace(tool_admission_profile="stable-core", lane="enumerate")):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admission_profile(args, ROLE.scope, p)

    def test_historical_equivalences_refuse_before_product_or_session_admission(self):
        p = policy()
        p["mutation_equivalences"] = [{"id": "historical"}]
        with patch.object(MODULE, "inventory_document", return_value=p), \
             patch.object(MODULE.PRODUCT, "admit_product") as product, \
             patch.object(MODULE.COMMON, "admit_mutation_equivalences") as equivalences, \
             patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_baseline_feedback_policy(ROOT)
        for call in (product, equivalences, session):
            call.assert_not_called()

    def test_source_or_helper_pin_drift_stops_before_session(self):
        identity = {"Cargo.toml": "a" * 64}
        for fault in ("candidate", "missing", "changed"):
            p = policy()
            p.update(source_file_count=1, candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                     inputs_sha256={name: "b" * 64 for name in ROLE.bound_inputs})
            if fault == "candidate": p["candidate_sha256"] = "c" * 64
            if fault == "missing": p["inputs_sha256"].pop(OWNERS["chart"])
            with self.subTest(fault=fault), patch.object(MODULE, "inventory_document", return_value=p), \
                 patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)), \
                 patch.object(MODULE.PRODUCT, "sha256", return_value="c" * 64 if fault == "changed" else "b" * 64), \
                 patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_baseline_feedback_policy(ROOT)
                session.assert_not_called()

    def test_actual_rosters_and_chart_doctests_are_complete_code_owned_sets(self):
        p = policy()
        actual = MODULE._conditioned_test_inventories(p, ROLE)
        self.assertEqual({key: len(value) for key, value in actual.items()}, {"baseline": 660, "coverage": 335})
        chart_docs = [name for name in actual["baseline"] if name.startswith(OWNERS["chart"] + " - ")]
        self.assertEqual(len(chart_docs), 4)
        for lane, fault in (("baseline", "missing-chart"), ("coverage", "replacement"),
                            ("coverage", "duplicate"), ("baseline", "order")):
            changed = copy.deepcopy(p)
            names = changed["test_inventories"][lane]
            if fault == "missing-chart": names.remove(chart_docs[0])
            elif fault == "replacement": names[0] = "unreviewed_same_count"; names.sort()
            elif fault == "duplicate": names[1] = names[0]
            else: names.reverse()
            with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(changed, ROLE)

    def test_successful_logs_require_all_exact_names_without_errors_or_filtering(self):
        p = policy()
        for lane, docs in (("baseline", True), ("coverage", False)):
            text = successes(p["test_inventories"][lane])
            result = MODULE._conditioned_test_execution(text, p, ROLE, doctests=docs)
            self.assertEqual(result["tests_executed"], 660 if docs else 335)
            for bad in (text.replace(" ... ok", " ... FAILED", 1), text.replace("0 filtered out", "1 filtered out"),
                        text + "Error: native work failed\n", text + "thread 'foreign' panicked at foreign.rs:1:1:\n"):
                with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                    MODULE._conditioned_test_execution(bad, p, ROLE, doctests=docs)

    def test_complete_five_owner_inventories_have_distinct_listing_and_campaign_caps(self):
        p = policy()
        self.assertEqual(len(MODULE._conditioned_inventory_bindings(p, ROLE, required=True)), 5)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.frequency_selected_owners(p, None, role=ROLE)
        for name, filename in OWNERS.items():
            self.assertEqual(MODULE.frequency_selected_owners(p, name, role=ROLE), {name: filename})
        for fault in ("partial", "extra", "zero", "bool", "per-file", "hash", "total"):
            changed = copy.deepcopy(p)
            rows = changed["mutation_inventories"]
            if fault == "partial": rows.pop("chart")
            elif fault == "extra": rows["foreign"] = rows["chart"]
            elif fault == "hash": rows["chart"]["sha256"] = "unbound"
            elif fault == "total":
                for row in rows.values(): row["count"] = 410
            else: rows["chart"]["count"] = {"zero": 0, "bool": True, "per-file": 513}[fault]
            with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                MODULE.frequency_inventory_bindings(changed, required=True, role=ROLE)
        for name, count in zip(OWNERS, (512, 512, 512, 511, 1), strict=True):
            p["mutation_inventories"][name]["count"] = count
        MODULE.frequency_inventory_bindings(p, required=True, role=ROLE)
        for row in p["mutation_inventories"].values(): row["count"] = 100
        self.assertEqual(MODULE.frequency_selected_owners(p, None, role=ROLE), OWNERS)
        for bad in ("frequency_numeric", "chart+numeric", True):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.frequency_selected_owners(p, bad, role=ROLE)

    def test_full_discovery_has_no_campaign_or_qualification_credit(self):
        p = policy()
        p["mutation_inventories"] = {}
        self.assertEqual(MODULE._conditioned_inventory_bindings(p, ROLE), {})
        with self.assertRaises(MODULE.Incomplete):
            MODULE._conditioned_inventory_bindings(p, ROLE, required=True)
        discovered = SimpleNamespace(inventories={name: {name: {"name": name, "diff": "synthetic"}} for name in OWNERS},
                                     total=5, campaign_budget_exceeded=False)
        report = {}
        with patch.object(MODULE.COMMON, "enumerate_mutation_inventories", return_value=discovered) as call:
            MODULE._enumerate_conditioned(SimpleNamespace(tools=Path("/tools"), run=object()), p, report, ROLE)
        self.assertEqual(call.call_args.args[1], OWNERS)
        self.assertEqual((call.call_args.kwargs["maximum_per_file"], call.call_args.kwargs["maximum_total"]), (512, 2048))
        self.assertEqual(set(report["listed_inventory_bindings"]), set(OWNERS))
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        self.assertNotIn("mutation", report)

    def test_external_contracts_use_new_closed_registry_only(self):
        helper = SimpleNamespace(baseline_feedback_contracts=Mock(), contracts=Mock())
        session = SimpleNamespace(source=Path("/captured"))
        p, report = policy(), {}
        with patch.object(MODULE.COMMON, "load_module", return_value=helper):
            MODULE._baseline_feedback_contracts(session, p, report)
        helper.baseline_feedback_contracts.assert_called_once_with(session, p, report,
            MODULE._external_contract_registry, MODULE.phase_plan_contract_examples)
        helper.contracts.assert_not_called()

    def test_chart_coded_compiler_cause_is_owned_and_no_null_exception_is_inherited(self):
        with tempfile.TemporaryDirectory() as directory:
            source, identity, record = compiler_fixture(directory)
            causes = MODULE._frequency_compiler_errors([record], source, identity, role=ROLE)
            self.assertEqual(causes[0]["code"], "E0308")
            self.assertEqual(causes[0]["primary"][0]["source"], OWNERS["chart"])
            for fault in ("uncoded", "foreign", "infra", "drift"):
                changed = copy.deepcopy(record)
                if fault == "uncoded": changed["message"]["code"] = None
                elif fault == "foreign": changed["package_id"] = "registry+foreign#0.0.0"
                elif fault == "infra": changed["message"]["message"] = "No space left on device"
                else: changed["message"]["spans"][0]["text"][0]["text"] = "wrong source bytes"
                with self.subTest(fault=fault), patch.object(MODULE, "_frequency_null_let_chain", return_value=True) as old1, \
                     patch.object(MODULE, "_frequency_const_trait_companion", return_value=True) as old2:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_compiler_errors([changed], source, identity, role=ROLE)
                    old1.assert_not_called()
                    old2.assert_not_called()

    def test_observed_rate_compiler_cases_are_exact_unviable_causes(self):
        pins = {
            461: ("fd87b21449306d77cfa29e7a99928670583aae2368798a4e5ec69abd34ca443b", "439cf789cd67ea67edb83613239d4b080748482d6a2a78732d1f417b9b4ac3b5"),
            605: ("412100c884b9b681f80ede6c570786e79abf5bac3fc2130242e0f3471fa3e826", "7e580fc37648bcfd98eb0ea397a4693a3284a7691c3fccf2946c139b4fcfa90d"),
            610: ("9c6c592b11836df7ce5c334456d3565f842ecec215aee954b96883f6800e5c2a", "ba64fc918dffc69337bd1b2fd1f2933586d3b11fb1f7b3cf7882bfa4cff7bec4"),
        }
        for line, (descriptor, payload) in pins.items():
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                self.assertEqual(MODULE.COMMON.canonical_sha256(mutant), descriptor)
                self.assertEqual(hashlib.sha256(diff.encode()).hexdigest(), payload)
                outcome, raw = unviable_phase(mutant, records)
                before = copy.deepcopy(outcome)
                with patch.object(MODULE, "_frequency_null_let_chain", side_effect=AssertionError("old case borrowed")), \
                     patch.object(MODULE, "_frequency_const_trait_companion", side_effect=AssertionError("old case borrowed")):
                    causes = MODULE._frequency_log_cause(outcome, raw, source, identity, policy(), mutant_diff=diff, role=ROLE)
                self.assertEqual([cause["code"] for cause in causes], ["E0658", None, "E0015"] * 2 if line == 461 else [None, None])
                self.assertEqual(outcome, before)
                self.assertEqual(outcome["summary"], "Unviable")
                self.assertEqual([phase["phase"] for phase in outcome["phase_results"]], ["Build"])

    def test_observed_compiler_cases_refuse_foreign_or_changed_custody(self):
        for line in (461, 605, 610):
            for fault in ("source", "identity", "file", "descriptor", "diff", "span", "message", "text",
                          "package", "target", "manifest", "compiler", "infrastructure"):
                with self.subTest(line=line, fault=fault), tempfile.TemporaryDirectory() as directory:
                    source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                    null = next(record for record in records if record["message"]["code"] is None)
                    if fault == "source":
                        path = source / OWNERS["rate_delivery"]
                        path.write_bytes(path.read_bytes() + b"\n")
                        identity[OWNERS["rate_delivery"]] = hashlib.sha256(path.read_bytes()).hexdigest()
                    elif fault == "identity": identity[OWNERS["rate_delivery"]] = "0" * 64
                    elif fault == "file": mutant["file"] = OWNERS["numeric"]
                    elif fault == "descriptor": mutant["function"]["return_type"] = "-> u64"
                    elif fault == "diff":
                        diff = diff.replace("changed by cargo-mutants", "changed by another tool")
                        for record in records:
                            text = record["message"]["spans"][0]["text"][0]
                            text["text"] = text["text"].replace("changed by cargo-mutants", "changed by another tool")
                    elif fault == "span":
                        span = null["message"]["spans"][0]
                        span["column_start"] += 1
                        span["byte_start"] += 1
                        span["text"][0]["highlight_start"] += 1
                    elif fault == "message": null["message"]["message"] += " "
                    elif fault == "text": null["message"]["spans"][0]["text"][0]["text"] = "foreign source"
                    elif fault == "package": null["package_id"] = "registry+foreign#0.0.0"
                    elif fault == "target":
                        for record in records: record["target"]["test"] = False
                    elif fault == "manifest": null["manifest_path"] = "/foreign/Cargo.toml"
                    elif fault == "compiler":
                        path = source / "policy/tool-manifest.json"
                        manifest = json.loads(path.read_text())
                        manifest["rust"]["channel"] = "1.99.2"
                        path.write_text(json.dumps(manifest))
                        identity["policy/tool-manifest.json"] = hashlib.sha256(path.read_bytes()).hexdigest()
                    else:
                        null["message"]["children"] = [{"message": "No space left on device", "level": "note", "children": []}]
                    with patch.object(MODULE, "_frequency_null_let_chain", side_effect=AssertionError("old case borrowed")), \
                         patch.object(MODULE, "_frequency_const_trait_companion", side_effect=AssertionError("old case borrowed")):
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE._frequency_compiler_errors(records, source, identity,
                                mutant=mutant, mutant_diff=diff, role=ROLE)

    def test_const_null_cause_requires_complete_balanced_exact_coded_companions(self):
        for fault in ("missing_e0658", "missing_e0015", "unbalanced", "extra", "mismatched", "extra_owned_error"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as directory:
                source, identity, mutant, diff, records = observed_rate_compiler_case(directory, 461)
                if fault.startswith("missing_"):
                    absent = "E0658" if fault == "missing_e0658" else "E0015"
                    records = [record for record in records if (record["message"]["code"] or {}).get("code") != absent]
                elif fault == "unbalanced": records.pop(0)
                elif fault == "extra": records.append(copy.deepcopy(records[1]))
                elif fault == "mismatched": records[2]["message"]["message"] += " "
                else:
                    extra = copy.deepcopy(records[0])
                    extra["message"].update(code={"code": "E0308", "explanation": None}, message="mismatched types")
                    records.append(extra)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_compiler_errors(records, source, identity, mutant=mutant, mutant_diff=diff, role=ROLE)

    def test_let_chain_null_cause_rejects_an_additional_owned_compiler_error(self):
        for line in (605, 610):
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                extra = copy.deepcopy(records[0])
                extra["message"].update(code={"code": "E0308", "explanation": None}, message="mismatched types")
                records.append(extra)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_compiler_errors(records, source, identity, mutant=mutant, mutant_diff=diff, role=ROLE)

    def test_new_compiler_cases_bind_compiled_view_and_do_not_enter_the_old_role(self):
        for line in (461, 605, 610):
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                compiled = MODULE._frequency_mutation_source(source, identity, mutant, diff, role=ROLE)
                null = next(record for record in records if record["message"]["code"] is None)
                self.assertTrue(MODULE._baseline_feedback_compiler_cause(records, null, mutant, diff, compiled, source, identity))
                self.assertFalse(MODULE._baseline_feedback_compiler_cause(records, null, mutant, diff,
                    (compiled[0], compiled[1] + b"\n"), source, identity))
                with patch.object(MODULE, "_baseline_feedback_compiler_cause", side_effect=AssertionError("wrong role")):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_compiler_errors(records, source, identity, mutant=mutant,
                            mutant_diff=diff, role=MODULE._frequency_prior_role())

    def test_five_owner_metadata_cannot_borrow_old_or_stale_receipts(self):
        p, identity = policy(), {"source": "e" * 64}
        admitted = {"inventories": {"stable": "c" * 64}, "policy_sha256": "d" * 64}
        for owner in OWNERS:
            row = receipt(owner, p, identity, admitted)
            self.assertEqual(MODULE.frequency_cohort_receipt_owner(row, p, identity, admitted, role=ROLE), owner)
            for key, value in (("complete_subjects", MODULE.FREQUENCY_PRIOR_SUBJECTS), ("status", "incomplete"),
                               ("source_sha256", {}), ("all_reviewed_inventory_bindings", {}),
                               ("analysis_admission_after", {"unchanged": False})):
                bad = copy.deepcopy(row)
                bad[key] = value
                with self.subTest(owner=owner, key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_receipt_owner(bad, p, identity, admitted, role=ROLE)

    def test_missing_or_duplicate_union_receipts_start_no_process_or_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "verification.json"
            path.write_text("{}")
            for paths in ([path] * 4, [path] * 5):
                args = SimpleNamespace(cohort_id=None, cohort_receipt=paths, output_dir=root / "union")
                with patch("subprocess.Popen") as process, self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_union(args, {}, policy(), ROLE)
                process.assert_not_called()
                self.assertFalse(args.output_dir.exists())

    def test_five_leaf_union_reuses_raw_guards_and_chart_failure_prevents_publication(self):
        # Plumbing controls only: all native/raw leaf validators below are mocked.
        p = policy()
        # Synthetic one-fault inventories carry no reviewed product equivalents.
        # Exact equivalent identities and actual Missed admission have dedicated controls.
        p["mutation_equivalences"] = []
        admitted = {"inventories": {"stable": "c" * 64}, "policy_sha256": "d" * 64}
        identity = {name: "b" * 64 for name in ("Cargo.toml", ROLE.scope.policy, *ROLE.bound_inputs)}
        result, baseline = {"gate_passed": True}, {"tests_executed": 660}
        payloads = {name: {"name": name + "_fault", "file": filename, "package": "time-core", "diff": "synthetic"}
                    for name, filename in OWNERS.items()}
        p["mutation_inventories"] = {name: {"count": 1, "sha256": MODULE.COMMON.canonical_sha256({
            item["name"]: {key: value for key, value in item.items() if key != "diff"}})}
            for name, item in payloads.items()}
        with tempfile.TemporaryDirectory() as directory, ExitStack() as stack:
            root, documents, paths = Path(directory), {}, []
            for owner in OWNERS:
                parent = root / owner
                parent.mkdir()
                path = parent / "verification.json"
                path.write_text("{}")
                paths.append(path)
                row = receipt(owner, p, identity, admitted)
                row.update(owned_output=str(parent), work=str(parent / "work"), tools_sha256={"cargo-mutants": "b" * 64},
                    mutation_enumeration={owner: {"raw_output": json.dumps([payloads[owner]])}},
                    lanes=[{"lane": "mutation-campaign:" + owner, "command": ["synthetic"], "exit_code": 0}],
                    mutation={owner: {**result, "baseline_tests_executed": 660}}, baseline_executions={owner: baseline},
                    baseline_feedback_artifacts={owner: {"synthetic": owner}})
                documents[str(path)] = row
            def read(path):
                if path.name == "outcomes.json": return {"synthetic_outcomes": True}
                MODULE.COMMON.read_analysis_bytes(path, maximum=16 * 1024 * 1024)
                return documents[str(path)]
            def artifacts(*args, **kwargs):
                self.assertEqual(kwargs["role"], ROLE)
                return {"synthetic": args[3]}
            for target, name, options in (
                (MODULE.PRODUCT, "admit_product", {"return_value": ({}, {"Cargo.toml": "b" * 64})}),
                (MODULE.PRODUCT, "sha256", {"return_value": "b" * 64}),
                (MODULE.COMMON.ANALYSIS, "admit", {"return_value": admitted}),
                (MODULE.COMMON, "read_analysis_json", {"side_effect": read}),
                (MODULE.COMMON, "require_unchanged", {}),
                (MODULE.COMMON, "tool_path", {"return_value": Path("/tools/cargo-mutants")}),
                (MODULE.COMMON, "_require_private_output", {})):
                stack.enter_context(patch.object(target, name, **options))
            envelopes = stack.enter_context(patch.object(MODULE, "frequency_campaign_result"))
            counts = stack.enter_context(patch.object(MODULE.COMMON, "mutation_counts", return_value=result))
            baselines = stack.enter_context(patch.object(MODULE, "_conditioned_baseline", return_value=baseline))
            raw = stack.enter_context(patch.object(MODULE, "frequency_artifact_manifest", side_effect=artifacts))
            process = stack.enter_context(patch("subprocess.Popen"))
            args = SimpleNamespace(cohort_id=None, cohort_receipt=paths, output_dir=root / "union")
            report = {}
            MODULE.frequency_cohort_union(args, report, p, ROLE)
            self.assertEqual(set(report["mutation_cohort_union"]), set(OWNERS))
            self.assertTrue(report["mutation_union_passed"])
            self.assertFalse(report["full_role_qualification"])
            for guard in (envelopes, counts, baselines, raw): self.assertEqual(guard.call_count, 5)
            raw.side_effect = lambda *args, **kwargs: (_ for _ in ()).throw(MODULE.Incomplete("chart raw cause")) \
                if args[3] == "chart" else artifacts(*args, **kwargs)
            args.output_dir = root / "rejected"
            with self.assertRaisesRegex(MODULE.Incomplete, "chart raw cause"):
                MODULE.frequency_cohort_union(args, {}, p, ROLE)
            self.assertFalse(args.output_dir.exists())
            process.assert_not_called()

    def test_baseline_failure_finishes_the_existing_owned_session(self):
        args = SimpleNamespace(subject="baseline_feedback", tool_admission_profile="stable-core", lane="baseline",
                               inventory_admission=None, cohort_plan=None, cohort_receipt=[], cohort_id=None)
        session = SimpleNamespace(run=Mock(side_effect=MODULE.Incomplete("raw failure")), finish=Mock())
        with patch.object(MODULE, "read_baseline_feedback_policy", return_value=policy()), \
             patch.object(MODULE.COMMON, "Session", return_value=session), patch.object(MODULE, "_frequency_captured"):
            with self.assertRaisesRegex(MODULE.Incomplete, "raw failure"):
                MODULE.execute_baseline_feedback(args, {})
        session.finish.assert_called_once()
        self.assertEqual(session.run.call_args.args[1][-1], "--test-threads=1")


def chart_result_fixture(directory):
    """Synthetic parser mechanics, with actual byte/diff/assertion validation.

    Only the finite review data is replaced. No reader, source admission, target
    decoder, hash implementation or assertion recognizer is mocked, and this
    fixture claims no historical compiler or product mutation outcome.
    """
    root = Path(directory)
    filename = OWNERS["chart"]
    test_source = "crates/time-core/tests/private/chart_result.rs"
    original = "pub fn actual() -> bool { true }\n"
    changed = original.replace("true", "false /* ~ changed by cargo-mutants ~ */")
    values = {filename: original.encode(), "crates/time-core/src/lib.rs":
        b'#[path = "../tests/private/chart_result.rs"] mod tests;\n',
        test_source: b'fn result_error() -> Result<(), ()> { Err(()) }\nfn actual_assertion() {\n    assert_eq!(1, 2);\n}\n'}
    for name, data in values.items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    identity = {name: hashlib.sha256(data).hexdigest() for name, data in values.items()}
    column = original.index("true") + 1
    description = "replace true with false in actual"
    mutant = {"file": filename, "function": {"function_name": "actual", "return_type": "-> bool",
        "span": {"start": {"line": 1, "column": 1}, "end": {"line": 1, "column": len(original.rstrip()) + 1}}},
        "genre": "BoolValue", "name": f"{filename}:1:{column}: {description}", "package": "time-core",
        "replacement": "false", "span": {"start": {"line": 1, "column": column},
                                          "end": {"line": 1, "column": column + 4}}}
    diff = "".join(difflib.unified_diff(original.splitlines(keepends=True), changed.splitlines(keepends=True),
        fromfile=filename, tofile=description, n=8))
    body = "Error: InvalidDomain"
    result_test, assertion_test = "tests::result_error", "tests::actual_assertion"
    data = {MODULE.COMMON.canonical_sha256(mutant): (hashlib.sha256(diff.encode()).hexdigest(),
        hashlib.sha256(changed.encode()).hexdigest(), MODULE.COMMON.canonical_sha256({result_test: body}))}
    review = {"_BASELINE_CHART_RESULT_SOURCE": (filename, identity[filename]),
        "_BASELINE_CHART_RESULT_CASES": data, "_BASELINE_CHART_RESULT_TESTS": {result_test: test_source},
        "_BASELINE_CHART_RESULT_SOURCES": {test_source: identity[test_source]}}
    text = ("running 2 tests\n" + f"test {assertion_test} ... FAILED\ntest {result_test} ... FAILED\n\nfailures:\n\n"
        + f"---- {assertion_test} stdout ----\n\nthread '{assertion_test}' panicked at {test_source}:3:5:\n"
        + "assertion `left == right` failed\n  left: 1\n right: 2\n\n"
        + f"---- {result_test} stdout ----\n{body}\n\nfailures:\n    {assertion_test}\n    {result_test}\n\n"
        + "test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
        + "error: test failed, to rerun pass `-p time-core --lib`\n")
    return root, identity, mutant, diff, review, text, {"test_inventories": {"baseline": [assertion_test, result_test]}}


class BaselineChartResultControls(unittest.TestCase):
    """Separate complete owned assertions from exact zero-credit auxiliary Results."""

    def test_review_catalog_is_finite_current_and_uses_actual_source_files(self):
        self.assertEqual(len(MODULE._BASELINE_CHART_RESULT_CASES), 25)
        self.assertEqual(len(MODULE._BASELINE_CHART_RESULT_TESTS), 21)
        self.assertEqual(len(MODULE._BASELINE_CHART_RESULT_SOURCES), 5)
        name, digest = MODULE._BASELINE_CHART_RESULT_SOURCE
        self.assertEqual(name, OWNERS["chart"])
        self.assertEqual(hashlib.sha256((ROOT / name).read_bytes()).hexdigest(), digest)
        for name, digest in MODULE._BASELINE_CHART_RESULT_SOURCES.items():
            self.assertEqual(hashlib.sha256((ROOT / name).read_bytes()).hexdigest(), digest)

    def test_mixed_result_has_one_actual_assertion_and_explicit_zero_credit(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, mutant, diff, review, text, p = chart_result_fixture(directory)
            observations = []
            with patch.multiple(MODULE, **review):
                outside, causes = MODULE._frequency_caught_text(text, root, identity, p,
                    mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=observations)
            self.assertEqual([cause["test"] for cause in causes], ["tests::actual_assertion"])
            self.assertEqual(causes[0]["kind"], "assert_eq!")
            self.assertEqual(len(observations), 1)
            self.assertEqual(observations[0]["test"], "tests::result_error")
            self.assertEqual(observations[0]["kill_credit"], 0)
            self.assertEqual(observations[0]["cargo_target"], "--lib")
            section = text[text.index("---- tests::result_error stdout ----"):text.index("failures:\n    tests::actual_assertion")]
            self.assertEqual(observations[0]["section_sha256"], hashlib.sha256(section.encode()).hexdigest())
            self.assertNotIn("Error:", outside)
            with patch.multiple(MODULE, **review), self.assertRaises(MODULE.Incomplete):
                MODULE._frequency_caught_text(text, root, identity, p, mutant=mutant, mutant_diff=diff, role=ROLE)

    def test_log_reader_requires_explicit_result_custody_and_retains_nonmixed_callers(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, mutant, diff, review, text, p = chart_result_fixture(directory)
            phases = []
            raw = ""
            for phase, status in (("Build", "Success"), ("Test", {"Failure": 101})):
                argv = ["cargo", "test", *(["--no-run"] if phase == "Build" else []), "--verbose",
                    "--package=time-core@0.0.0", "--message-format=json", "--frozen"]
                phases.append({"phase": phase, "argv": argv, "duration": 0.01, "process_status": status})
                raw += "*** " + " ".join(argv) + '\n{"reason":"build-finished","success":true}\n'
                if phase == "Test": raw += text
            outcome = {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": phases}
            observations = []
            with patch.multiple(MODULE, **review):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_log_cause(outcome, raw, root, identity, p, mutant_diff=diff, role=ROLE)
                causes = MODULE._frequency_log_cause(outcome, raw, root, identity, p,
                    mutant_diff=diff, role=ROLE, uncredited=observations)
                # Removing the auxiliary Result still admits the historical
                # assertion-only helper call without introducing a new collector.
                start = raw.index("---- tests::result_error stdout ----")
                end = raw.index("failures:\n    tests::actual_assertion")
                assertion_only = (raw[:start] + raw[end:]).replace("test tests::result_error ... FAILED\n", "")
                assertion_only = assertion_only.replace("    tests::result_error\n", "").replace("running 2 tests", "running 1 tests")
                assertion_only = assertion_only.replace("0 passed; 2 failed", "0 passed; 1 failed")
                plain = MODULE._frequency_log_cause(outcome, assertion_only, root, identity, p, role=ROLE)
            self.assertEqual(causes, plain)
            self.assertEqual(len(causes), 1)
            self.assertEqual(len(observations), 1)
            self.assertEqual(observations[0]["kill_credit"], 0)

    def test_result_adjudication_refuses_wrong_source_descriptor_diff_test_body_and_role(self):
        for fault in ("source", "test_source", "descriptor", "diff", "body", "test", "role", "compiled", "roster"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as directory:
                root, identity, mutant, diff, review, text, p = chart_result_fixture(directory)
                role = ROLE
                if fault in {"source", "test_source"}:
                    name = OWNERS["chart"] if fault == "source" else "crates/time-core/tests/private/chart_result.rs"
                    (root / name).write_bytes((root / name).read_bytes() + b"\n")
                    identity[name] = hashlib.sha256((root / name).read_bytes()).hexdigest()
                elif fault == "descriptor": mutant["function"]["return_type"] = "-> u8"
                elif fault == "diff": diff += "\n"
                elif fault == "body": text = text.replace("Error: InvalidDomain", "Error: InvalidFrames")
                elif fault == "test":
                    text = text.replace("tests::result_error", "tests::foreign_result")
                    p["test_inventories"]["baseline"][1] = "tests::foreign_result"
                elif fault == "role": role = MODULE._frequency_prior_role()
                else:
                    key = next(iter(review["_BASELINE_CHART_RESULT_CASES"]))
                    entry = list(review["_BASELINE_CHART_RESULT_CASES"][key])
                    entry[1 if fault == "compiled" else 2] = "0" * 64
                    review["_BASELINE_CHART_RESULT_CASES"][key] = tuple(entry)
                with patch.multiple(MODULE, **review), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_caught_text(text, root, identity, p,
                        mutant=mutant, mutant_diff=diff, role=role, uncredited=[])

    def test_result_adjudication_keeps_all_complete_block_and_global_vetoes(self):
        for fault in ("body_error", "same_panic", "outside_error", "resource", "duplicate", "truncated", "summary", "footer", "missing_assertion"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as directory:
                root, identity, mutant, diff, review, text, p = chart_result_fixture(directory)
                if fault == "body_error": text = text.replace("Error: InvalidDomain", "Error: InvalidDomain\nError: InvalidFrames")
                elif fault == "same_panic": text = text.replace("Error: InvalidDomain", "Error: InvalidDomain\nthread 'tests::result_error' panicked at crates/time-core/tests/private/chart_result.rs:3:5:\nassertion `left == right` failed")
                elif fault == "outside_error": text += "Error: setup failed\n"
                elif fault == "resource": text += "No space left on device\n"
                elif fault == "duplicate": text = text.replace("    tests::result_error\n", "    tests::actual_assertion\n")
                elif fault == "truncated": text = text[:text.index("failures:\n    tests::actual_assertion")]
                elif fault == "summary": text = text.replace("0 passed; 2 failed", "0 passed; 3 failed")
                elif fault == "footer": text = text.replace("--lib`", "--test foreign`")
                else:
                    start = text.index("---- tests::actual_assertion stdout ----")
                    end = text.index("---- tests::result_error stdout ----")
                    text = text[:start] + text[end:]
                    text = text.replace("test tests::actual_assertion ... FAILED\n", "").replace("    tests::actual_assertion\n", "")
                    text = text.replace("running 2 tests", "running 1 tests").replace("0 passed; 2 failed", "0 passed; 1 failed")
                with patch.multiple(MODULE, **review), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_caught_text(text, root, identity, p,
                        mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=[])

    def test_actual_chart_and_phase_getter_const_companions_are_build_only_owned_causes(self):
        for line in (426, 427, 433, 439, 445, 517, 2294, 2299):
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                outcome, raw = unviable_phase(mutant, records)
                with patch.object(MODULE, "_frequency_null_let_chain", side_effect=AssertionError("historical case borrowed")), \
                     patch.object(MODULE, "_frequency_const_trait_companion", side_effect=AssertionError("historical case borrowed")):
                    causes = MODULE._frequency_log_cause(outcome, raw, source, identity, policy(), mutant_diff=diff, role=ROLE)
                self.assertEqual(len(causes), 6)
                self.assertEqual([cause["code"] for cause in causes], ["E0658", None, "E0015"] * 2)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_compiler_errors(records, source, identity, mutant=mutant,
                        mutant_diff=diff, role=MODULE._frequency_prior_role())

    def test_chart_and_phase_getter_const_companions_refuse_incomplete_and_foreign_neighbors(self):
        for line in (426, 427, 433, 439, 445, 517, 2294, 2299):
            for fault in ("unbalanced", "extra", "message", "span", "target", "tool", "source", "descriptor", "diff"):
                with self.subTest(line=line, fault=fault), tempfile.TemporaryDirectory() as directory:
                    source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                    if fault == "unbalanced": records.pop(0)
                    elif fault == "extra":
                        extra = copy.deepcopy(records[0]);extra["message"].update(message="mismatched types", code={"code": "E0308", "explanation": None});records.append(extra)
                    elif fault == "message": records[1]["message"]["message"] += " "
                    elif fault == "span": records[1]["message"]["spans"][0]["byte_start"] += 1
                    elif fault == "target": records[1]["target"]["test"] = False
                    elif fault in {"tool", "source"}:
                        name = "policy/tool-manifest.json" if fault == "tool" else mutant["file"]
                        (source / name).write_bytes((source / name).read_bytes() + b"\n")
                        identity[name] = hashlib.sha256((source / name).read_bytes()).hexdigest()
                    elif fault == "descriptor": mutant["replacement"] += " "
                    else: diff += "\n"
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_compiler_errors(records, source, identity, mutant=mutant, mutant_diff=diff, role=ROLE)


def auxiliary_panic_fixture(directory, mode):
    """Exercise parser mechanics with real finite source/diff/assertion checks.

    The review data alone is replaced by an explicit synthetic case. No real
    compiler, source custody validator, assertion recognizer or hash is mocked.
    These literal logs grant no actual product or mutation evidence.
    """
    root = Path(directory)
    filename = OWNERS["phase_plan"]
    test_source = "crates/time-core/tests/private/auxiliary.rs"
    original = "pub fn observe(v: u64) -> u64 { v - v }\n"
    changed = original.replace("-", "/ /* ~ changed by cargo-mutants ~ */")
    values = {filename: original.encode(), "crates/time-core/src/lib.rs":
        b'#[path = "../tests/private/auxiliary.rs"] mod tests;\n',
        test_source: b'fn observed_assertion() {\n    assert!(1 == 2, "direct clock neighbor must retain Map");\n}\nfn dependent_setup() {}\n',
        "policy/tool-manifest.json": (ROOT / "policy/tool-manifest.json").read_bytes()}
    for name, data in values.items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    identity = {name: hashlib.sha256(data).hexdigest() for name, data in values.items()}
    column = original.index("-") + 1
    description = "replace - with / in observe"
    mutant = {"file": filename, "function": {"function_name": "observe", "return_type": "-> u64",
        "span": {"start": {"line": 1, "column": 1}, "end": {"line": 1, "column": len(original.rstrip()) + 1}}},
        "genre": "BinaryOperator", "name": f"{filename}:1:{column}: {description}", "package": "time-core",
        "replacement": "/", "span": {"start": {"line": 1, "column": column}, "end": {"line": 1, "column": column + 1}}}
    diff = "".join(difflib.unified_diff(original.splitlines(keepends=True), changed.splitlines(keepends=True),
        fromfile=filename, tofile=description, n=8))
    witness = "tests::observed_assertion"
    auxiliary = witness if mode == "recovered" else "tests::dependent_setup"
    binding = (hashlib.sha256(diff.encode()).hexdigest(), hashlib.sha256(changed.encode()).hexdigest(),
        1, column, "attempt to divide by zero", (auxiliary,), (witness, test_source, 2, 5, "assert!"))
    review = {"_BASELINE_PHASE_AUXILIARY_SOURCE": (filename, identity[filename]),
        "_BASELINE_PHASE_AUXILIARY_TEST_SOURCE": (test_source, identity[test_source]),
        "_BASELINE_PHASE_AUXILIARY_CASES": {MODULE.COMMON.canonical_sha256(mutant): binding}}
    frame = (f"thread '{auxiliary}' (123) panicked at {filename}:1:{column}:\nattempt to divide by zero\n"
        + "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n")
    assertion = (f"thread '{witness}' (123) panicked at {test_source}:2:5:\n"
        + "direct clock neighbor must retain Map\n")
    sections = ({witness: frame + "\n" + assertion} if mode == "recovered" else
                {witness: assertion, auxiliary: frame})
    names = sorted(sections)
    text = (f"running {len(names)} tests\n" + "".join(f"test {name} ... FAILED\n" for name in names)
        + "\nfailures:\n\n" + "".join(f"---- {name} stdout ----\n\n{sections[name]}\n" for name in names)
        + "failures:\n" + "".join("    " + name + "\n" for name in names)
        + f"\ntest result: FAILED. 0 passed; {len(names)} failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
        + "error: test failed, to rerun pass `-p time-core --lib`\n")
    return root, identity, mutant, diff, review, text, {"test_inventories": {"baseline": names}}


class BaselineAuxiliaryPanicControls(unittest.TestCase):
    """Require exact direct assertion credit while retaining reviewed auxiliary facts."""

    def test_auxiliary_review_catalog_has_only_five_actual_current_cases(self):
        self.assertEqual(len(MODULE._BASELINE_PHASE_AUXILIARY_CASES), 5)
        for name, digest in (MODULE._BASELINE_PHASE_AUXILIARY_SOURCE, MODULE._BASELINE_PHASE_AUXILIARY_TEST_SOURCE):
            self.assertEqual(hashlib.sha256((ROOT / name).read_bytes()).hexdigest(), digest)
        self.assertEqual(sorted(len(row[5]) for row in MODULE._BASELINE_PHASE_AUXILIARY_CASES.values()), [1, 1, 1, 6, 7])
        self.assertEqual({row[6][2] for row in MODULE._BASELINE_PHASE_AUXILIARY_CASES.values()}, {1425, 1443, 1466})

    def test_recovered_and_separate_auxiliary_frames_earn_no_assertion_credit(self):
        for mode in ("recovered", "separate"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as directory:
                root, identity, mutant, diff, review, text, p = auxiliary_panic_fixture(directory, mode)
                observations = []
                with patch.multiple(MODULE, **review):
                    outside, causes = MODULE._frequency_caught_text(text, root, identity, p,
                        mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=observations)
                    no_note = text.replace("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n", "")
                    MODULE._frequency_caught_text(no_note, root, identity, p,
                        mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=[])
                self.assertEqual([cause["test"] for cause in causes], ["tests::observed_assertion"])
                self.assertEqual(causes[0]["kind"], "assert!")
                self.assertEqual(len(observations), 1)
                self.assertEqual(observations[0]["kill_credit"], 0)
                self.assertEqual(observations[0]["kind"], "auxiliary_arithmetic_panic")
                self.assertEqual(observations[0]["message"], "attempt to divide by zero")
                self.assertRegex(observations[0]["panic_sha256"], r"^[0-9a-f]{64}$")
                self.assertRegex(observations[0]["section_sha256"], r"^[0-9a-f]{64}$")
                self.assertNotIn("panicked at", outside)

    def test_auxiliary_frames_require_explicit_retention_and_the_exact_direct_witness(self):
        for mode in ("recovered", "separate"):
            for fault in ("collector", "witness-test", "witness-source", "witness-line", "witness-column", "witness-kind", "roster"):
                with self.subTest(mode=mode, fault=fault), tempfile.TemporaryDirectory() as directory:
                    root, identity, mutant, diff, review, text, p = auxiliary_panic_fixture(directory, mode)
                    key = next(iter(review["_BASELINE_PHASE_AUXILIARY_CASES"]))
                    row = list(review["_BASELINE_PHASE_AUXILIARY_CASES"][key])
                    if fault == "roster": row[5] += ("tests::unobserved_auxiliary",)
                    elif fault != "collector":
                        witness = list(row[6]);index = {"witness-test": 0, "witness-source": 1,
                            "witness-line": 2, "witness-column": 3, "witness-kind": 4}[fault]
                        witness[index] = witness[index] + 1 if type(witness[index]) is int else witness[index] + "_changed"
                        row[6] = tuple(witness)
                    review["_BASELINE_PHASE_AUXILIARY_CASES"][key] = tuple(row)
                    with patch.multiple(MODULE, **review), self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_caught_text(text, root, identity, p, mutant=mutant,
                            mutant_diff=diff, role=ROLE, uncredited=None if fault == "collector" else [])

    def test_auxiliary_frames_reject_changed_source_tool_mutant_and_applied_bytes(self):
        for fault in ("source", "test-source", "tool", "descriptor", "diff", "compiled", "role"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as directory:
                root, identity, mutant, diff, review, text, p = auxiliary_panic_fixture(directory, "recovered")
                role = ROLE
                if fault in {"source", "test-source", "tool"}:
                    name = (OWNERS["phase_plan"] if fault == "source" else "policy/tool-manifest.json" if fault == "tool"
                            else "crates/time-core/tests/private/auxiliary.rs")
                    (root / name).write_bytes((root / name).read_bytes() + b"\n")
                    identity[name] = hashlib.sha256((root / name).read_bytes()).hexdigest()
                elif fault == "descriptor": mutant["function"]["return_type"] = "-> u32"
                elif fault == "diff": diff += "\n"
                elif fault == "compiled":
                    key = next(iter(review["_BASELINE_PHASE_AUXILIARY_CASES"]));row = list(review["_BASELINE_PHASE_AUXILIARY_CASES"][key]);row[1] = "0" * 64
                    review["_BASELINE_PHASE_AUXILIARY_CASES"][key] = tuple(row)
                else: role = MODULE._frequency_prior_role()
                with patch.multiple(MODULE, **review), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_caught_text(text, root, identity, p, mutant=mutant, mutant_diff=diff, role=role, uncredited=[])

    def test_fresh_state_compiler_cases_never_inherit_historical_exception_admission(self):
        for line in (474, 743, 811):
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                source, identity, mutant, diff, records = observed_rate_compiler_case(directory, line)
                outcome, raw = unviable_phase(mutant, records)
                with patch.object(MODULE, "_frequency_null_let_chain", side_effect=AssertionError("old role borrowed")), \
                     patch.object(MODULE, "_frequency_const_trait_companion", side_effect=AssertionError("old role borrowed")):
                    causes = MODULE._frequency_log_cause(outcome, raw, source, identity, policy(), mutant_diff=diff, role=ROLE)
                self.assertEqual(len(causes), 6 if line == 474 else 2)
                for fault in ("source", "extra-owned-error", "companion", "target", "span", "diff"):
                    changed = copy.deepcopy(records)
                    changed_diff = diff
                    if fault == "source": identity[mutant["file"]] = "0" * 64
                    elif fault == "extra-owned-error":
                        extra = copy.deepcopy(records[0]);extra["message"].update(code={"code": "E0308", "explanation": None}, message="mismatched types");changed.append(extra)
                    elif fault == "companion":
                        if line == 474: changed.pop()
                        else: changed.append(copy.deepcopy(records[0]))
                    elif fault == "target": changed[0]["target"]["test"] = False
                    elif fault == "span": changed[0]["message"]["spans"][0]["byte_start"] += 1
                    else: changed_diff += "\n"
                    with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_compiler_errors(changed, source, identity, mutant=mutant, mutant_diff=changed_diff, role=ROLE)
                    identity[mutant["file"]] = hashlib.sha256((source / mutant["file"]).read_bytes()).hexdigest()

    def test_auxiliary_frames_keep_other_panics_errors_resources_and_complete_joins_fatal(self):
        for mode in ("recovered", "separate"):
            for fault in ("message", "message-suffix", "line", "column", "thread", "prefix", "extra-panic", "extra-error", "resource", "outside-error", "truncated", "footer", "summary"):
                with self.subTest(mode=mode, fault=fault), tempfile.TemporaryDirectory() as directory:
                    root, identity, mutant, diff, review, text, p = auxiliary_panic_fixture(directory, mode)
                    filename = OWNERS["phase_plan"];column = review["_BASELINE_PHASE_AUXILIARY_CASES"][next(iter(review["_BASELINE_PHASE_AUXILIARY_CASES"]))][3]
                    if fault == "message": text = text.replace("attempt to divide by zero", "attempt to subtract with overflow")
                    elif fault == "message-suffix": text = text.replace("attempt to divide by zero", "attempt to divide by zero WRONG")
                    elif fault == "line": text = text.replace(filename + ":1:", filename + ":2:")
                    elif fault == "column": text = text.replace(f"{filename}:1:{column}:", f"{filename}:1:{column + 1}:")
                    elif fault == "thread": text = text.replace("' (123) panicked at " + filename, "_foreign' (123) panicked at " + filename)
                    elif fault == "prefix": text = text.replace("attempt to divide by zero", "attempt to divide by zero\nunknown panic tail")
                    elif fault == "extra-panic": text = text.replace("attempt to divide by zero", "attempt to divide by zero\nthread 'foreign' panicked at foreign.rs:1:1:\nforeign panic")
                    elif fault == "extra-error": text = text.replace("attempt to divide by zero", "attempt to divide by zero\nError: cleanup failed")
                    elif fault == "resource": text = text.replace("attempt to divide by zero", "attempt to divide by zero\nNo space left on device")
                    elif fault == "outside-error": text += "Error: unrelated native setup failed\n"
                    elif fault == "truncated": text = text[:text.index("test result: FAILED.")]
                    elif fault == "footer": text = text.replace("--lib`", "--test foreign`")
                    else: text = text.replace("failed; 0 ignored", "failed; 1 ignored")
                    with patch.multiple(MODULE, **review), self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_caught_text(text, root, identity, p, mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=[])


class ChartAssertionTargetTests(unittest.TestCase):
    """Actual chart test owners retain their exact Cargo target and source guard."""

    @staticmethod
    def fixture(directory, embedded):
        root = Path(directory)
        source = ("crates/time-core/src/combination/chart/receiving_tests.rs" if embedded else
                  "crates/time-core/tests/private/chart_composition.rs")
        including = ("crates/time-core/src/combination/chart/receiving.rs" if embedded else
                     "crates/time-core/tests/chart.rs")
        name = ("combination::admission::chart_receiving::tests::chart_tracking_receiver_keeps_actual_slots_and_complete_witness"
                if embedded else "chart_composition::composition_rejects_relationships_and_ordered_arithmetic_failures")
        identity = {}
        for filename in (source, including):
            data = (ROOT / filename).read_bytes()
            path = root / filename
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            identity[filename] = hashlib.sha256(data).hexdigest()
        line, column = (450, 9) if embedded else (369, 9)
        message = "assertion `left == right` failed" if embedded else "assertion failed: matches!(left.compose(&right), Err(error) if error == expected)"
        target = "--lib" if embedded else "--test chart"
        text = (f"running 1 tests\ntest {name} ... FAILED\n\nfailures:\n\n---- {name} stdout ----\n"
                f"\nthread '{name}' panicked at {source}:{line}:{column}:\n{message}\n\n"
                f"failures:\n    {name}\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
                f"error: test failed, to rerun pass `-p time-core {target}`\n")
        return root, identity, source, including, name, text, {"test_inventories": {"baseline": [name]}}

    def test_private_integration_assertion_uses_the_actual_chart_test_binary(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, source, _, name, text, p = self.fixture(directory, False)
            outside, causes = MODULE._frequency_caught_text(text, root, identity, p, role=ROLE)
            self.assertEqual([(c["source"], c["test"]) for c in causes], [(source, name)])
            self.assertNotIn("panicked at", outside)

    def test_embedded_cfg_test_assertion_is_checked_without_admitting_product_panics(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, source, _, name, text, p = self.fixture(directory, True)
            outside, causes = MODULE._frequency_caught_text(text, root, identity, p, role=ROLE)
            self.assertEqual([(c["source"], c["test"], c["kind"]) for c in causes], [(source, name, "assert_eq!")])
            self.assertNotIn("panicked at", outside)


    def test_chart_test_admission_rejects_foreign_namespace_function_role_and_footer(self):
        for embedded in (False, True):
            for fault in ("namespace", "function", "role", "footer", "message"):
                with self.subTest(embedded=embedded, fault=fault), tempfile.TemporaryDirectory() as directory:
                    root, identity, _, _, name, text, p = self.fixture(directory, embedded)
                    role = ROLE
                    if fault in ("namespace", "function"):
                        changed = ("foreign::" + name.rsplit("::", 1)[1] if fault == "namespace" else
                                   name.rsplit("::", 1)[0] + "::not_an_actual_test")
                        text = text.replace(name, changed)
                        p["test_inventories"]["baseline"] = [changed]
                    elif fault == "role": role = MODULE._frequency_prior_role()
                    elif fault == "footer": text = text.replace("-p time-core --", "-p time-core --test foreign --")
                    else: text = text.replace("assertion `left == right` failed", "attempt to divide by zero") if embedded else text.replace("assertion failed:", "attempt to divide by zero:")
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_caught_text(text, root, identity, p, role=role)

    def test_chart_test_source_and_including_module_each_require_exact_identity_and_bytes(self):
        for embedded in (False, True):
            for which in ("test", "including"):
                for fault in ("identity", "bytes"):
                    with self.subTest(embedded=embedded, which=which, fault=fault), tempfile.TemporaryDirectory() as directory:
                        root, identity, source, including, _, text, p = self.fixture(directory, embedded)
                        name = source if which == "test" else including
                        if fault == "identity": identity[name] = "0" * 64
                        else: (root / name).write_bytes((root / name).read_bytes() + b"\n")
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE._frequency_caught_text(text, root, identity, p, role=ROLE)

    def test_chart_test_semantics_reject_changed_cfg_path_module_and_test_attribute(self):
        for embedded, fault in ((True, "cfg"), (True, "path"), (True, "module"), (True, "test"),
                                (False, "path"), (False, "module"), (False, "test")):
            with self.subTest(embedded=embedded, fault=fault), tempfile.TemporaryDirectory() as directory:
                root, identity, source, including, name, text, p = self.fixture(directory, embedded)
                binding = list(MODULE._BASELINE_CHART_ASSERTION_OWNERS[source])
                filename = source if fault == "test" else including
                data = (root / filename).read_text()
                if fault == "cfg": data = data.replace("#[cfg(test)]", "#[cfg(any())]")
                elif fault == "path": data = data.replace('#[path = "' + binding[3] + '"]', '#[path = "foreign.rs"]')
                elif fault == "module": data = data.replace("mod " + binding[4] + ";", "mod foreign;")
                else: data = data.replace("#[test]\nfn " + name.rsplit("::", 1)[1], "fn " + name.rsplit("::", 1)[1])
                (root / filename).write_text(data)
                digest = hashlib.sha256(data.encode()).hexdigest()
                identity[filename] = digest
                binding[0 if fault == "test" else 2] = digest
                reviews = dict(MODULE._BASELINE_CHART_ASSERTION_OWNERS)
                reviews[source] = tuple(binding)
                # Deliberately rebind the synthetic hash so the structural guard,
                # rather than the preceding byte-identity guard, must reject.
                with patch.object(MODULE, "_BASELINE_CHART_ASSERTION_OWNERS", reviews), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_caught_text(text, root, identity, p, role=ROLE)

    def test_chart_assertion_target_rejects_a_second_captured_include(self):
        for embedded in (False, True):
            with self.subTest(embedded=embedded), tempfile.TemporaryDirectory() as directory:
                root, identity, source, including, _, text, p = self.fixture(directory, embedded)
                binding = MODULE._BASELINE_CHART_ASSERTION_OWNERS[source]
                duplicate = str(Path(including).parent / "foreign_owner.rs")
                data = ('#[path = "' + binding[3] + '"]\nmod ' + binding[4] + ';\n').encode()
                (root / duplicate).write_bytes(data)
                identity[duplicate] = hashlib.sha256(data).hexdigest()
                with self.assertRaisesRegex(MODULE.Incomplete, "ambiguous or foreign"):
                    MODULE._frequency_caught_text(text, root, identity, p, role=ROLE)

    def test_arbitrary_product_test_named_source_still_cannot_earn_assertion_credit(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, source, _, _, text, p = self.fixture(directory, True)
            foreign = "crates/time-core/src/combination/chart/foreign_tests.rs"
            data = (root / source).read_bytes()
            (root / foreign).write_bytes(data)
            identity[foreign] = hashlib.sha256(data).hexdigest()
            with self.assertRaisesRegex(MODULE.Incomplete, "product panic"):
                MODULE._frequency_caught_text(text.replace(source, foreign), root, identity, p, role=ROLE)


    def test_chart_actual_assertion_cannot_be_relabelled_as_an_existing_unrelated_test(self):
        for embedded in (False, True):
            with self.subTest(embedded=embedded), tempfile.TemporaryDirectory() as directory:
                root, identity, _, _, name, text, p = self.fixture(directory, embedded)
                other = ("chart_tracking_receiver_absence_requires_empty_metadata" if embedded else
                         "composition_retains_both_operand_errors_through_signed_elapsed")
                changed = name.rsplit("::", 1)[0] + "::" + other
                p["test_inventories"]["baseline"] = [name, changed]
                with self.assertRaisesRegex(MODULE.Incomplete, "reviewed assertion location"):
                    MODULE._frequency_caught_text(text.replace(name, changed), root, identity, p, role=ROLE)

    def test_chart_shared_helper_assertions_keep_only_their_actual_caller_and_location(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, _, _, _, text, p = self.fixture(directory, True)
            # These three independent helper assertions are called by the same
            # source-pinned genuine receiving test, not by every test in its file.
            for line in (416, 450, 459):
                with self.subTest(line=line):
                    _, causes = MODULE._frequency_caught_text(text.replace(":450:9:", f":{line}:9:"), root, identity, p, role=ROLE)
                    self.assertEqual(causes[0]["line"], line)
            with self.assertRaisesRegex(MODULE.Incomplete, "reviewed assertion location"):
                MODULE._frequency_caught_text(text.replace(":450:9:", ":450:10:"), root, identity, p, role=ROLE)


if __name__ == "__main__":
    unittest.main()
