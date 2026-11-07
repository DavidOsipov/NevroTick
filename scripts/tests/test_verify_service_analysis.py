"""Independent service-analysis admission/accounting controls; no analyzer runs."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("service_analysis_control", ROOT / "scripts/verify-service-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def policy_fixture():
    """Supply independent literal native identities and a complete inert freeze."""
    policy = json.loads((ROOT / "policy/service-analysis.json").read_text())
    policy["mutation_inventories"] = {}
    policy["binding"] = {"source_file_count": 1, "candidate_sha256": "a" * 64,
        "inputs_sha256": {name: "b" * 64 for name in MODULE.BOUND_INPUTS},
        "ordinary_tests": ["healthy_readiness", "status_failure_fences_work"],
        "doctests": ["crates/windows-platform/src/lib.rs - (line 10)"]}
    return policy


def execution(*, doctests=False):
    """Native-format fixture with names authored independently of checker output."""
    text = ("test healthy_readiness ... ok\ntest status_failure_fences_work ... ok\n"
            "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n")
    if doctests:
        text += ("test crates/windows-platform/src/lib.rs - (line 10) ... ok\n"
                 "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n")
    return text


def phases(summary):
    """Model the pinned actual Cargo phase/status grammar, including build failures."""
    result = []
    for phase in (["Build"] if summary == "Unviable" else ["Build", "Test"]):
        argv = ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                "--verbose", "--package=windows-platform@0.0.0", "--message-format=json", "--frozen"]
        failed = summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test")
        result.append({"phase": phase, "argv": argv, "process_status": {"Failure": 101} if failed else "Success", "duration": 0.1})
    return result


def outcome(summary):
    """One fixed source mutant; counts are never treated as an actual campaign."""
    return {"scenario": {"Mutant": {"name": "lifecycle_guard", "package": "windows-platform",
                "file": "crates/windows-platform/src/service.rs"}},
            "summary": summary, "phase_results": phases(summary), "log_path": "cause.log"}


class ServiceAnalysisControls(unittest.TestCase):
    """Challenge policy admission and actual execution/cause evidence independently."""

    def test_current_subject_inputs_and_cleared_preparation_copy(self):
        policy = json.loads((ROOT / "policy/service-analysis.json").read_text())
        self.assertEqual(policy["schema_version"], 5)
        self.assertEqual(policy["subjects"], {
            "lifecycle": "crates/windows-platform/src/service.rs",
            "scm": "crates/windows-platform/src/scm.rs",
            "privilege": "crates/windows-platform/src/privilege_observation.rs"})
        for path in ("crates/windows-platform/src/privilege_observation.rs",
                     "crates/windows-platform/tests/privilege_observation.rs",
                     "crates/windows-platform/tests/privilege_observation_contracts.rs"):
            self.assertIn(path, MODULE.BOUND_INPUTS)
            self.assertTrue(MODULE.selected_span(path))
        MODULE.validate_policy(policy, require_binding=False)
        for current in (policy, policy_fixture()):
            before = copy.deepcopy(current)
            preparation = copy.deepcopy(current)
            preparation["binding"] = None
            preparation["mutation_inventories"] = {}
            MODULE.validate_policy(preparation, require_binding=False)
            with self.assertRaisesRegex(MODULE.Incomplete, "not bound"):
                MODULE.validate_policy(preparation)
            self.assertEqual(current, before)

    def test_schema5_admits_only_the_selected_four_worker_task_profile(self):
        policy = {**policy_fixture(), 'schema_version': 5, 'mutation_jobs': 4,
                  'mutation_process_limit': 512}
        MODULE.validate_policy(policy)
        for change in ({'mutation_jobs': 1}, {'mutation_jobs': 8}, {'mutation_jobs': 12},
                       {'mutation_process_limit': 128}, {'mutation_process_limit': 1024},
                       {'schema_version': 3}, {'schema_version': 4}, {'schema_version': 6},
                       {'schema_version': True}, {'schema_version': 5.0}):
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy({**policy, **change})

    def test_ambient_compiler_flags_fail_before_policy_or_product_admission(self):
        for name in ('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTDOCFLAGS', 'CARGO_ENCODED_RUSTDOCFLAGS',
                     'CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS',
                     'CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS',
                     'CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTDOCFLAGS'):
            for value in ('', 'private-flag-marker'):
                with self.subTest(name=name, empty=not value), patch.dict(os.environ, {name: value}), \
                        patch.object(MODULE.COMMON, 'read_analysis_json', side_effect=AssertionError('unexpected policy I/O')) as read, \
                        self.assertRaisesRegex(MODULE.Incomplete, 'ambient compiler') as failure:
                    MODULE.read_policy(ROOT, campaign=False)
                read.assert_not_called()
                self.assertNotIn('private-flag-marker', str(failure.exception))

    def test_parallel_campaign_uses_worker_targets_and_one_inner_slot_without_flag_overrides(self):
        with tempfile.TemporaryDirectory() as temporary:
            common = MODULE.COMMON
            session = common.Session.__new__(common.Session)
            session.root, session.output = ROOT, Path(temporary)
            session.work = session.output / 'work'
            session.source, session.config = session.work / 'source', session.work / 'cargo-home/config.toml'
            session.base, session.stable = Path('/admitted/base'), Path('/admitted/stable')
            session.tools, session.vendor = Path('/admitted/tools'), Path('/admitted/vendor')
            session.scope, session.policy = MODULE.SCOPE, policy_fixture()
            session.admission_profile, session.payload_readonly = 'stable-core', (session.stable,)
            session.admitted = {'policy': {'common_nightly': 'common/nightly'}}
            session.protected_islands, session.protected_tools = {}, {}
            session.report, session.check = {'lanes': []}, lambda: None
            command = ['/admitted/tools/cargo-mutants', 'mutants', '--package', 'windows-platform',
                       '--file', 'crates/windows-platform/src/service.rs', '--test-workspace', 'false',
                       '--cap-lints', 'false', '--jobs', '4', '--jobserver', 'true', '--jobserver-tasks', '4',
                       '--cargo-arg=--message-format=json', '--baseline', 'run', '--build-timeout', '90',
                       '--timeout', '60', '--output', '/work/mutation/lifecycle']
            result = {'exit_code': 0, 'failure': None, 'output': 'observed output'}
            with patch.dict(os.environ, {'RUSTFLAGS': 'private-flag-marker', 'CARGO_ENCODED_RUSTFLAGS': 'marker'}), \
                    patch.object(common.RUNTIME, 'run', return_value=result) as execute:
                session.run('mutation-campaign:lifecycle', command, 1200, mutation_subject='lifecycle')
                options = execute.call_args.kwargs
                self.assertEqual(execute.call_args.args[2], command)
                self.assertEqual(options['process_limit'], 512)
                self.assertEqual(options['cargo_target_mode'], 'worker')
                self.assertEqual(options['build_jobs'], 1)
                self.assertEqual(options['environment'], {'RUSTC': '/admitted/stable/bin/rustc',
                                 'RUSTDOC': '/admitted/stable/bin/rustdoc', 'RUST_TEST_THREADS': '1'})
                self.assertEqual(options['mode'], 'ordinary')
                self.assertEqual(options['timeout'], 1200)
                session.run('coverage', ['/admitted/tools/cargo-llvm-cov', 'llvm-cov'], 300)
                ordinary = execute.call_args.kwargs
                self.assertNotIn('process_limit', ordinary)
                self.assertNotIn('cargo_target_mode', ordinary)
                self.assertEqual(ordinary['build_jobs'], 6)

    def test_preparation_is_valid_but_cannot_execute_or_borrow_a_campaign(self):
        policy = json.loads((ROOT / "policy/service-analysis.json").read_text())
        policy["binding"] = None
        policy["mutation_inventories"] = {}
        MODULE.validate_policy(policy, require_binding=False)
        with self.assertRaisesRegex(MODULE.Incomplete, "not bound"):
            MODULE.validate_policy(policy)
        policy = policy_fixture()
        MODULE.validate_policy(policy)
        with self.assertRaisesRegex(MODULE.Incomplete, "reviewed and bound"):
            MODULE.validate_policy(policy, campaign=True)
        policy["mutation_inventories"] = {"lifecycle": {"count": 2, "sha256": "c" * 64},
                                         "scm": {"count": 3, "sha256": "d" * 64},
                                         "privilege": {"count": 1, "sha256": "e" * 64}}
        MODULE.validate_policy(policy, campaign=True)

    def test_three_owner_profile_rejects_omissions_and_combined_budget_overflow(self):
        healthy = policy_fixture()
        self.assertEqual(healthy["schema_version"], 5)
        self.assertEqual(healthy["mutation_jobs"], 4)
        self.assertEqual(healthy["mutation_process_limit"], 512)
        for key in ("subjects", "critical_files"):
            changed = copy.deepcopy(healthy)
            if key == "subjects": changed[key].pop("privilege")
            else: changed[key].remove("crates/windows-platform/src/privilege_observation.rs")
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete): MODULE.validate_policy(changed)
        healthy["mutation_inventories"] = {"lifecycle": {"count": 255, "sha256": "c" * 64},
                                            "scm": {"count": 255, "sha256": "d" * 64},
                                            "privilege": {"count": 2, "sha256": "e" * 64}}
        MODULE.validate_policy(healthy, campaign=True)
        for key in ("lifecycle", "scm", "privilege"):
            changed = copy.deepcopy(healthy); changed["mutation_inventories"].pop(key)
            with self.subTest(omitted=key), self.assertRaises(MODULE.Incomplete): MODULE.validate_policy(changed, campaign=True)
        healthy["mutation_inventories"]["privilege"]["count"] = 3
        with self.assertRaisesRegex(MODULE.Incomplete, "combined 512"):
            MODULE.validate_policy(healthy, campaign=True)

    def test_scope_threshold_budget_profile_and_exception_changes_fail_closed(self):
        healthy = policy_fixture()
        for key, value in (("package", "nevrotick-engine"), ("subjects", {"other": "unowned.rs"}),
                ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                ("mutation_jobs", True), ("mutation_jobs", 1), ("mutation_jobs", 8), ("mutation_jobs", 12), ("jobs", 12),
                ("mutation_process_limit", 128), ("tool_admission_profile", "full"),
                ("mutation_wall_seconds", 3600), ("mutation_error_values", {}),
                ("mutation_equivalences", [{"name": "excluded"}])):
            changed = copy.deepcopy(healthy); changed[key] = value
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy(changed)
        MODULE.validate_policy(healthy)

    def test_bindings_require_all_helpers_and_unique_nonempty_native_inventories(self):
        for mutate in (lambda b: b.update(source_file_count=True),
                       lambda b: b["inputs_sha256"].pop("scripts/portable-analysis.py"),
                       lambda b: b["inputs_sha256"].update({MODULE.SOURCE: "bad"}),
                       lambda b: b.update(ordinary_tests=[]),
                       lambda b: b.update(ordinary_tests=["same", "same"]),
                       lambda b: b.update(doctests=b["ordinary_tests"])):
            changed = policy_fixture(); mutate(changed["binding"])
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_policy(changed)

    def test_named_execution_rejects_omissions_duplicate_skips_and_wrong_summaries(self):
        policy = policy_fixture()
        self.assertEqual(MODULE.validate_execution(execution(), policy, doctests=False)["tests_executed"], 2)
        self.assertEqual(MODULE.validate_execution(execution(doctests=True), policy, doctests=True)["tests_executed"], 3)
        for text in ("", execution().replace("status_failure_fences_work", "other"),
                     execution().replace("0 ignored", "1 ignored"), execution().replace("0 filtered out", "1 filtered out"),
                     execution().replace("2 passed", "1 passed"), execution() + "test healthy_readiness ... ok\n"):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_execution(text, policy, doctests=False)
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_execution(execution(), policy, doctests=True)

    def test_actual_behavioral_cause_is_required_beside_a_successful_build(self):
        policy = policy_fixture()
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary); path = directory / "cause.log"
            healthy = ("test healthy_readiness ... FAILED\n---- healthy_readiness stdout ----\n"
                "thread 'healthy_readiness' panicked at crates/windows-platform/tests/service.rs:80:5:\n"
                "assertion `left == right` failed\nfailures:\n")
            path.write_text(healthy)
            result = MODULE.validate_causes(directory, {"outcomes": [outcome("CaughtMutant")]}, policy)
            self.assertEqual(result["failures"]["lifecycle_guard"]["kind"], "behavioral")
            for text in ("unclassified failure", healthy.replace("healthy_readiness", "other"),
                         healthy.replace("crates/windows-platform/tests/service.rs", "dependency.rs"),
                         healthy + "error[E0308]: could not compile\n", healthy + "HARNESS_INCOMPLETE\n"):
                path.write_text(text)
                with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_causes(directory, {"outcomes": [outcome("CaughtMutant")]}, policy)
            path.write_text(healthy.replace("thread 'healthy_readiness' panicked at crates/windows-platform/tests/service.rs:80:5:\nassertion `left == right` failed", "Error: DeadlineElapsed"))
            MODULE.validate_causes(directory, {"outcomes": [outcome("CaughtMutant")]}, policy)
            broken = outcome("CaughtMutant"); broken["phase_results"][0]["process_status"] = {"Failure": 101}
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_causes(directory, {"outcomes": [broken]}, policy)

    def test_unviable_needs_an_owned_compiler_cause_and_gets_no_kill_credit(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary); path = directory / "cause.log"
            diagnostic = {"reason": "compiler-message", "target": {"name": "windows_platform"},
                "message": {"level": "error", "spans": [{"is_primary": True,
                    "file_name": "crates/windows-platform/src/service.rs"}]}}
            path.write_text(json.dumps(diagnostic, separators=(",", ":")))
            result = MODULE.validate_causes(directory, {"outcomes": [outcome("Unviable")]}, policy_fixture())
            self.assertEqual(result["failures"]["lifecycle_guard"]["kill_credit"], 0)
            for text in ("resource failure", json.dumps({**diagnostic, "message": None}),
                         json.dumps(diagnostic).replace("crates/windows-platform/src/service.rs", "dependency.rs")):
                path.write_text(text)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_causes(directory, {"outcomes": [outcome("Unviable")]}, policy_fixture())

    def test_unviable_rejects_mixed_foreign_infrastructure_and_harness_failures(self):
        diagnostic = {"reason": "compiler-message", "target": {"name": "windows_platform"},
            "message": {"level": "error", "message": "owned type error", "spans": [{"is_primary": True,
                "file_name": "crates/windows-platform/src/service.rs"}]}}
        healthy = json.dumps(diagnostic, separators=(",", ":")) + "\n"
        foreign = copy.deepcopy(diagnostic); foreign["message"]["spans"][0]["file_name"] = "dependency.rs"
        mixed = copy.deepcopy(diagnostic); mixed["message"]["spans"].append({"is_primary": True, "file_name": "foreign.rs"})
        summary = copy.deepcopy(diagnostic); summary["message"].update(message="aborting due to 1 previous error", spans=[])
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary); path = directory / "cause.log"
            path.write_text(healthy + json.dumps(summary, separators=(",", ":")))
            MODULE.validate_causes(directory, {"outcomes": [outcome("Unviable")]}, policy_fixture())
            for extra in ("SERVICE_HARNESS_INCOMPLETE", "Cannot allocate memory", "No space left on device",
                          "error: foreign infrastructure failure", json.dumps(foreign, separators=(",", ":")),
                          json.dumps(mixed, separators=(",", ":"))):
                path.write_text(healthy + extra)
                with self.subTest(extra=extra), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_causes(directory, {"outcomes": [outcome("Unviable")]}, policy_fixture())
            path.write_text(json.dumps(summary, separators=(",", ":")))
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_causes(directory, {"outcomes": [outcome("Unviable")]}, policy_fixture())

    def test_fixed_json_phase_policy_rejects_human_output_for_every_outcome(self):
        for summary in ("Success", "CaughtMutant", "Unviable", "MissedMutant"):
            row = {"summary": summary, "phase_results": phases(summary)}
            MODULE.validate_phases(row)
            for phase in row["phase_results"]: phase["argv"].remove("--message-format=json")
            with self.subTest(summary=summary), self.assertRaisesRegex(MODULE.Incomplete, "fixed JSON"):
                MODULE.validate_phases(row)

    def test_read_policy_rejects_candidate_helper_drift_and_bad_owner_registration(self):
        policy = policy_fixture(); identity = {"one": "b" * 64}
        policy["binding"]["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); (root / "policy").mkdir()
            manifest = root / "policy/verification-manifest.toml"
            healthy = ('critical_files = ["crates/windows-platform/src/service.rs", "crates/windows-platform/src/scm.rs", "crates/windows-platform/src/privilege_observation.rs"]\n'
                'security_modules = [{package="windows-platform",files=["crates/windows-platform/src/service.rs"]},'
                '{package="windows-platform",files=["crates/windows-platform/src/scm.rs"]},'
                '{package="windows-platform",files=["crates/windows-platform/src/privilege_observation.rs"]}]\n')
            manifest.write_text(healthy)
            with patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
                    patch.object(MODULE.COMMON.PRODUCT, "admit_product", return_value=({}, identity)), \
                    patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64):
                MODULE.read_policy(root, campaign=False)
                for text in ('critical_files=[]\n' + healthy.split('\n', 1)[1],
                             healthy.split('\n', 1)[0] + '\nsecurity_modules=[]\n',
                             healthy.replace(', "crates/windows-platform/src/scm.rs"', ''),
                             healthy.replace(',{package="windows-platform",files=["crates/windows-platform/src/scm.rs"]}', ''),
                             healthy.replace(', "crates/windows-platform/src/privilege_observation.rs"', ''),
                             healthy.replace(',{package="windows-platform",files=["crates/windows-platform/src/privilege_observation.rs"]}', '')):
                    manifest.write_text(text)
                    with self.assertRaisesRegex(MODULE.Incomplete, "registration"):
                        MODULE.read_policy(root, campaign=False)
                manifest.write_text(healthy)
                with patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="c" * 64), self.assertRaisesRegex(MODULE.Incomplete, "changed after freezing"):
                    MODULE.read_policy(root, campaign=False)
                with patch.object(MODULE.COMMON.PRODUCT, "admit_product", return_value=({}, {"changed": "c" * 64})), self.assertRaisesRegex(MODULE.Incomplete, "changed after freezing"):
                    MODULE.read_policy(root, campaign=False)

    def test_log_traversal_and_foreign_source_paths_are_refused(self):
        for value in ("/tmp/foreign.log", "../foreign.log", ""):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete): MODULE.owned_log(Path("/tmp"), value)
        self.assertTrue(MODULE.selected_span("/work/tmp/mutant/crates/windows-platform/src/service.rs"))
        for value in ("/tmp/crates/windows-platform/src/service.rs", "crates/windows-platform/src/../src/service.rs", None):
            self.assertFalse(MODULE.selected_span(value))

    def test_enumeration_is_listing_only_and_keeps_the_shared_custodian(self):
        policy = policy_fixture(); identity = {"one": "b" * 64}
        policy["binding"]["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
        state = SimpleNamespace(finished=False, ran=False)
        class Session:
            def __init__(self, root, args, report, scope, selected_policy, inputs):
                self.product_identity = identity; self.work = Path("/owned/work")
                self.source = Path("/owned/source"); self.tools = Path("/owned/tools")
                self.run = lambda *a, **k: None
            def finish(self): state.finished = True
        listing = SimpleNamespace(inventories={"lifecycle": {"fault": {"name": "fault"}},
                                              "scm": {"scm_fault": {"name": "scm_fault"}},
                                              "privilege": {"privilege_fault": {"name": "privilege_fault"}}})
        report = {}
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64), \
                patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
                patch.object(MODULE.COMMON, "enumerate_mutation_inventories", return_value=listing) as enumerate_, \
                patch.object(MODULE.COMMON, "run_analysis_lanes") as campaigns:
            MODULE.execute(SimpleNamespace(lane="enumerate"), report)
        self.assertTrue(state.finished); self.assertFalse(report["qualification_passed"])
        self.assertFalse(report["campaign_admission"]); campaigns.assert_not_called()
        self.assertEqual(enumerate_.call_args.kwargs["maximum_total"], 512)
        descriptors = {"fault": {"name": "fault"}}
        expected = hashlib.sha256(json.dumps(descriptors, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        self.assertEqual(report["listed_inventory_bindings"]["lifecycle"], {"count": 1, "sha256": expected})
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64), \
                patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
                patch.object(MODULE.COMMON, "enumerate_mutation_inventories", return_value=SimpleNamespace(
                    inventories={"lifecycle": {"fault": {"name": "fault"}}})), self.assertRaisesRegex(MODULE.Incomplete, "omitted"):
            MODULE.execute(SimpleNamespace(lane="enumerate"), {})

    def test_scm_causes_are_subject_bound_and_baseline_checks_every_selected_test(self):
        policy = policy_fixture()
        row = outcome("CaughtMutant")
        row["scenario"]["Mutant"]["file"] = "crates/windows-platform/src/scm.rs"
        row["scenario"]["Mutant"]["name"] = "scm_projection_guard"
        baseline = {"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success"), "log_path": "baseline.log"}
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); directory = work / "mutation/scm/mutants.out"; directory.mkdir(parents=True)
            (directory / "baseline.log").write_text(execution(doctests=True))
            (directory / "cause.log").write_text("test healthy_readiness ... FAILED\n---- healthy_readiness stdout ----\n"
                "thread 'healthy_readiness' panicked at crates/windows-platform/tests/scm.rs:55:5:\nassertion `left == right` failed\nfailures:\n")
            outcomes = {"outcomes": [baseline, row]}
            (directory / "outcomes.json").write_text(json.dumps(outcomes))
            result = MODULE.validate_baseline(work, policy, subject="scm")
            self.assertEqual(result["tests_executed"], 3)
            self.assertEqual(result["causes"]["failures"]["scm_projection_guard"]["kind"], "behavioral")
            with self.assertRaisesRegex(MODULE.Incomplete, "out-of-scope"):
                MODULE.validate_causes(directory, outcomes, policy, subject="lifecycle")
            foreign = copy.deepcopy(outcomes); foreign["outcomes"][1]["scenario"]["Mutant"]["file"] = "crates/windows-platform/src/foreign.rs"
            with self.assertRaisesRegex(MODULE.Incomplete, "out-of-scope"):
                MODULE.validate_causes(directory, foreign, policy, subject="scm")
            (directory / "baseline.log").write_text(execution())
            with self.assertRaisesRegex(MODULE.Incomplete, "complete bound"):
                MODULE.validate_baseline(work, policy, subject="scm")
            (directory / "baseline.log").write_text(execution(doctests=True))
            broken = copy.deepcopy(outcomes)
            broken["outcomes"][0]["phase_results"][0]["argv"] = ["cargo", "test", "--no-run", "--verbose", "--package=other@0.0.0", "--message-format=json", "--frozen"]
            (directory / "outcomes.json").write_text(json.dumps(broken))
            with self.assertRaises(MODULE.Incomplete): MODULE.validate_baseline(work, policy, subject="scm")
            for subject in ("all", "../lifecycle", "foreign"):
                with self.subTest(subject=subject), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_baseline(work, policy, subject=subject)

    def test_metrics_do_not_claim_qualification_and_copy_failures_still_finish(self):
        policy = policy_fixture(); identity = {"one": "b" * 64}
        policy["binding"]["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
        state = SimpleNamespace(finishes=0, fail_finish=False)
        class Session:
            def __init__(self, root, args, report, scope, selected_policy, inputs):
                self.product_identity = identity; self.work = Path("/owned/work")
                self.source = Path("/owned/source"); self.tools = Path("/owned/tools")
                self.run = lambda *a, **k: None
            def finish(self):
                state.finishes += 1
                if state.fail_finish: raise MODULE.Incomplete("changed final custody")
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64), \
                patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
                patch.object(MODULE.COMMON, "run_analysis_lanes"):
            report = {}; MODULE.execute(SimpleNamespace(lane="coverage"), report)
            self.assertTrue(report["metrics_passed"]); self.assertFalse(report["qualification_passed"])
            self.assertEqual(len(report["remaining_independent_qualification"]), 2)
            with patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="c" * 64), self.assertRaisesRegex(MODULE.Incomplete, "Captured service candidate"):
                MODULE.execute(SimpleNamespace(lane="coverage"), {})
            state.fail_finish = True; report = {}
            with self.assertRaisesRegex(MODULE.Incomplete, "changed final custody"):
                MODULE.execute(SimpleNamespace(lane="coverage"), report)
            self.assertFalse(report["metrics_passed"]); self.assertFalse(report["qualification_passed"])
        self.assertEqual(state.finishes, 3)

    def test_privilege_primary_and_contract_targets_are_exact_and_subject_bound(self):
        policy = policy_fixture()
        cases = [
            ("complete_zero_and_unrelated_captures_allow_the_engine", "privilege_observation",
             "crates/windows-platform/tests/privilege_observation.rs"),
            ("each_isolated_lookup_collision_is_rejected", "privilege_observation_contracts",
             "crates/windows-platform/tests/privilege_observation_contracts.rs"),
        ]
        policy["binding"]["ordinary_tests"] = sorted(policy["binding"]["ordinary_tests"] + [case[0] for case in cases])
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            for name, target, source in cases:
                row = outcome("CaughtMutant")
                row["scenario"]["Mutant"].update(name="privilege_guard", file=MODULE.PRIVILEGE_SOURCE)
                path = directory / "cause.log"
                path.write_text(f"test {name} ... FAILED\n---- {name} stdout ----\n"
                    f"thread '{name}' panicked at {source}:80:5:\nassertion `left == right` failed\nfailures:\n")
                result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="privilege")
                self.assertEqual(result["failures"]["privilege_guard"]["kind"], "behavioral")
                with self.assertRaisesRegex(MODULE.Incomplete, "out-of-scope"):
                    MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="scm")
                row["summary"], row["phase_results"] = "Unviable", phases("Unviable")
                diagnostic = {"reason": "compiler-message", "target": {"name": target},
                    "message": {"level": "error", "spans": [{"is_primary": True, "file_name": source}]}}
                path.write_text(json.dumps(diagnostic))
                result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="privilege")
                self.assertEqual(result["failures"]["privilege_guard"]["kill_credit"], 0)
                for wrong in ("foreign", "privilege", "nevrotick_windows_service_fixture"):
                    diagnostic["target"]["name"] = wrong
                    path.write_text(json.dumps(diagnostic))
                    with self.subTest(target=wrong), self.assertRaisesRegex(MODULE.Incomplete, "foreign compiler"):
                        MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="privilege")

    def test_privilege_typed_positive_refusal_never_borrows_unknown_errors(self):
        policy = policy_fixture()
        name = "complete_zero_and_unrelated_captures_allow_the_engine"
        policy["binding"]["ordinary_tests"] = sorted([*policy["binding"]["ordinary_tests"], name])
        row = outcome("CaughtMutant")
        row["scenario"]["Mutant"].update(name="privilege_guard", file=MODULE.PRIVILEGE_SOURCE)
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            for error in ("Bounds", "Length", "DuplicatePrivilege", "DuplicateTargetIds", "UnsupportedAttributes",
                          "RemovedEntry", "EnginePrivilegeError { privilege: Debug, state: PresentDisabled }"):
                (directory / "cause.log").write_text(f"test {name} ... FAILED\n---- {name} stdout ----\nError: {error}\nfailures:\n")
                result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="privilege")
                self.assertEqual(result["failures"]["privilege_guard"]["causes"][0]["kind"], "typed-positive-path-refusal")
            (directory / "cause.log").write_text(f"test {name} ... FAILED\n---- {name} stdout ----\nError: Unknown\nfailures:\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "actual selected behavioral"):
                MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="privilege")

    def test_privilege_named_tests_and_privacy_doctests_cannot_be_omitted(self):
        policy = policy_fixture()
        ordinary = ["complete_zero_and_unrelated_captures_allow_the_engine", "each_isolated_lookup_collision_is_rejected"]
        docs = ["crates/windows-platform/src/privilege_observation.rs - privilege_observation (line 10) - compile fail",
                "crates/windows-platform/src/privilege_observation.rs - privilege_observation (line 15) - compile fail"]
        policy["binding"]["ordinary_tests"] = sorted([*policy["binding"]["ordinary_tests"], *ordinary])
        policy["binding"]["doctests"] = sorted([*policy["binding"]["doctests"], *docs])
        extra = "".join(f"test {name} ... ok\n" for name in [*ordinary, *docs])
        text = execution(doctests=True) + extra + "test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
        self.assertEqual(MODULE.validate_execution(text, policy, doctests=True)["tests_executed"], 7)
        for name in [*ordinary, *docs]:
            with self.subTest(omitted=name), self.assertRaisesRegex(MODULE.Incomplete, "complete bound"):
                MODULE.validate_execution(text.replace(f"test {name} ... ok\n", ""), policy, doctests=True)


if __name__ == "__main__":
    unittest.main()
