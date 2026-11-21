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


def policy_fixture(*, diagnostics=False, codec=False):
    """Supply independent literal native identities and a complete inert freeze."""
    policy = json.loads((ROOT / "policy/service-analysis.json").read_text())
    if not codec:
        policy.pop("coverage_guided_fuzzing", None)
        policy.pop("external_contracts", None)
        policy["schema_version"] = 6
    if not diagnostics and not codec:
        policy.update(schema_version=5, subjects={
            "lifecycle": "crates/windows-platform/src/service.rs",
            "scm": "crates/windows-platform/src/scm.rs",
            "privilege": "crates/windows-platform/src/privilege_observation.rs"}, critical_files=[
            "crates/windows-platform/src/service.rs", "crates/windows-platform/src/scm.rs",
            "crates/windows-platform/src/privilege_observation.rs"])
    policy["mutation_inventories"] = {}
    policy["binding"] = {"source_file_count": 1, "candidate_sha256": "a" * 64,
        "inputs_sha256": {name: "b" * 64 for name in (
            MODULE.CODEC_BOUND_INPUTS if codec else MODULE.BOUND_INPUTS if diagnostics else MODULE.LEGACY_BOUND_INPUTS)},
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
        self.assertEqual(policy["schema_version"], 7)
        self.assertEqual(policy["subjects"], {
            "diagnostics": "crates/windows-platform/src/diagnostics.rs"})
        self.assertEqual(policy["critical_files"], ["crates/windows-platform/src/diagnostics.rs"])
        self.assertIsNone(policy["binding"])
        self.assertEqual(policy["mutation_inventories"], {})
        for path in ("crates/windows-platform/src/diagnostics.rs",
                     "crates/windows-platform/src/diagnostics_tests.rs",
                     "crates/windows-platform/tests/diagnostics.rs"):
            self.assertIn(path, MODULE.BOUND_INPUTS)
            self.assertTrue(MODULE.selected_span(path))
        MODULE.validate_policy(policy, require_binding=False)
        for current in (policy, policy_fixture(), policy_fixture(diagnostics=True)):
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

    def test_schema6_is_exactly_one_shared_owner_with_unchanged_gates_and_limits(self):
        healthy = policy_fixture(diagnostics=True)
        MODULE.validate_policy(healthy)
        self.assertEqual(MODULE.selected_subjects(healthy), {
            "diagnostics": "crates/windows-platform/src/diagnostics.rs"})
        self.assertEqual((healthy["coverage_minimum_line_percent"],
                          healthy["mutation_minimum_viable_killed_percent"]), (90, 95))
        for key, value in (("schema_version", True), ("schema_version", 6.0),
                ("schema_version", 5), ("package", "nevrotick-windows-service-fixture"),
                ("subjects", {"diagnostics": "tools/windows-service-fixture/src/diagnostics.rs"}),
                ("subjects", {**healthy["subjects"], "lifecycle": MODULE.SOURCE}),
                ("critical_files", []), ("critical_files", [MODULE.SOURCE]),
                ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                ("tool_admission_profile", "full"), ("mutation_jobs", True), ("mutation_jobs", 1),
                ("mutation_process_limit", 128), ("coverage_wall_seconds", 301),
                ("enumeration_wall_seconds", 121), ("mutation_wall_seconds", 1201),
                ("mutation_build_seconds", 91), ("mutation_test_seconds", 61), ("jobs", 7),
                ("test_threads", 2), ("coverage_test_threads", True), ("maximum_mutants", 513),
                ("mutation_equivalences", [{"excluded": True}]), ("mutation_error_values", {})):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy({**healthy, key: value})
        for key in healthy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy({name: value for name, value in healthy.items() if name != key})

    def test_schema6_requires_actual_shared_inputs_and_complete_nonempty_inventories(self):
        healthy = policy_fixture(diagnostics=True)
        for source in ("crates/windows-platform/src/diagnostics.rs",
                       "crates/windows-platform/src/diagnostics_tests.rs",
                       "crates/windows-platform/tests/diagnostics.rs"):
            changed = copy.deepcopy(healthy); changed["binding"]["inputs_sha256"].pop(source)
            with self.subTest(missing=source), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy(changed)
        for key in ("ordinary_tests", "doctests"):
            changed = copy.deepcopy(healthy); changed["binding"][key] = []
            with self.subTest(empty=key), self.assertRaisesRegex(MODULE.Incomplete, "nonempty"):
                MODULE.validate_policy(changed)
        with self.assertRaisesRegex(MODULE.Incomplete, "reviewed and bound"):
            MODULE.validate_policy(healthy, campaign=True)
        for count in (1, 512):
            healthy["mutation_inventories"] = {"diagnostics": {"count": count, "sha256": "c" * 64}}
            MODULE.validate_policy(healthy, campaign=True)
        for inventories in ({"diagnostics": {"count": 0, "sha256": "c" * 64}},
                            {"diagnostics": {"count": 513, "sha256": "c" * 64}},
                            {"diagnostics": {"count": True, "sha256": "c" * 64}},
                            {"diagnostics": {"count": 1, "sha256": "bad"}},
                            {"lifecycle": {"count": 1, "sha256": "c" * 64}},
                            {**healthy["mutation_inventories"], "scm": {"count": 1, "sha256": "d" * 64}}):
            with self.subTest(inventories=inventories), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy({**healthy, "mutation_inventories": inventories}, campaign=True)

    def test_schema6_checks_ordinary_shared_tests_and_privacy_doctests_by_name(self):
        policy = policy_fixture(diagnostics=True)
        ordinary = ["catalogue_contract", "diagnostics::tests::private_constructor"]
        docs = ["crates/windows-platform/src/diagnostics.rs - diagnostics (line 10)",
                "crates/windows-platform/src/diagnostics.rs - diagnostics (line 20) - compile fail"]
        policy["binding"].update(ordinary_tests=ordinary, doctests=docs)
        text = "".join(f"test {name} ... ok\n" for name in [*ordinary, *docs])
        text += "test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
        self.assertEqual(MODULE.validate_execution(text, policy, doctests=True)["tests_executed"], 4)
        for name in [*ordinary, *docs]:
            with self.subTest(missing=name), self.assertRaisesRegex(MODULE.Incomplete, "complete bound"):
                MODULE.validate_execution(text.replace(f"test {name} ... ok\n", ""), policy, doctests=True)

    def test_schema6_attributes_real_shared_targets_and_gives_compiler_failures_zero_credit(self):
        policy = policy_fixture(diagnostics=True)
        row = outcome("CaughtMutant")
        row["scenario"]["Mutant"].update(name="diagnostic_guard", file="crates/windows-platform/src/diagnostics.rs")
        cases = (("windows_platform", "crates/windows-platform/src/diagnostics.rs"),
                 ("windows_platform", "crates/windows-platform/src/diagnostics_tests.rs"),
                 ("diagnostics", "crates/windows-platform/tests/diagnostics.rs"))
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary); path = directory / "cause.log"
            with self.assertRaisesRegex(MODULE.Incomplete, "unavailable or violates file custody"):
                MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="diagnostics")
            for target, source in cases:
                row["summary"], row["phase_results"] = "CaughtMutant", phases("CaughtMutant")
                path.write_text("test healthy_readiness ... FAILED\n---- healthy_readiness stdout ----\n"
                    f"thread 'healthy_readiness' panicked at {source}:80:5:\nassertion `left == right` failed\nfailures:\n")
                result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="diagnostics")
                self.assertEqual(result["failures"]["diagnostic_guard"]["kind"], "behavioral")
                row["summary"], row["phase_results"] = "Unviable", phases("Unviable")
                diagnostic = {"reason": "compiler-message", "target": {"name": target},
                    "message": {"level": "error", "spans": [{"is_primary": True, "file_name": source}]}}
                path.write_text(json.dumps(diagnostic))
                result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="diagnostics")
                self.assertEqual(result["failures"]["diagnostic_guard"]["kill_credit"], 0)
                for wrong in ("nevrotick_windows_service_fixture", "foreign"):
                    diagnostic["target"]["name"] = wrong
                    path.write_text(json.dumps(diagnostic))
                    with self.subTest(target=wrong), self.assertRaisesRegex(MODULE.Incomplete, "foreign compiler"):
                        MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="diagnostics")
            for wrong in ("lifecycle", "scm", "privilege", "all"):
                with self.subTest(subject=wrong), self.assertRaisesRegex(MODULE.Incomplete, "exact portable subject"):
                    MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject=wrong)
            self.assertFalse(MODULE.selected_span("tools/windows-service-fixture/src/diagnostics.rs", policy))
            self.assertFalse(MODULE.selected_span("crates/windows-platform/src/diagnostics.rs", policy_fixture()))

    def test_diagnostics_schemas_reject_legacy_refusals_as_behavioral_credit(self):
        """Unit-returning diagnostics tests cannot report lifecycle Result errors."""
        name = "decoder_versions_and_complete_length_have_no_fallback_or_partial_acceptance"
        for codec in (False, True):
            policy = policy_fixture(diagnostics=True, codec=codec)
            policy["binding"]["ordinary_tests"] = [name]
            row = outcome("CaughtMutant")
            row["scenario"]["Mutant"].update(
                name="diagnostic_guard", file=MODULE.DIAGNOSTICS_SOURCE)
            with tempfile.TemporaryDirectory() as temporary:
                directory = Path(temporary)
                for error in ("DeadlineElapsed", "EnginePrivilegeError", "Bounds",
                              "Forbidden(EnginePrivilegeError { privilege: Debug, state: PresentEnabled })",
                              "MissingRequired(ChangeNotify)"):
                    (directory / "cause.log").write_text(
                        f"test {name} ... FAILED\n---- {name} stdout ----\nError: {error}\nfailures:\n")
                    with self.subTest(schema=policy["schema_version"], error=error), self.assertRaisesRegex(
                            MODULE.Incomplete, "actual selected behavioral test cause"):
                        MODULE.validate_causes(directory, {"outcomes": [row]}, policy,
                                               subject="diagnostics")

    def test_diagnostics_schemas_preserve_owned_assertions_and_reject_foreign_panics(self):
        """Legal diagnostics assertions qualify; foreign and unlisted failures do not."""
        name = "required_info_off_has_literal_original_fields_and_real_consumer_acceptance"
        for codec in (False, True):
            policy = policy_fixture(diagnostics=True, codec=codec)
            policy["binding"]["ordinary_tests"] = [name]
            row = outcome("CaughtMutant")
            row["scenario"]["Mutant"].update(
                name="diagnostic_guard", file=MODULE.DIAGNOSTICS_SOURCE)
            with tempfile.TemporaryDirectory() as temporary:
                directory = Path(temporary)
                for source in MODULE.DIAGNOSTICS_ATTRIBUTED_SOURCES:
                    text = (f"test {name} ... FAILED\n---- {name} stdout ----\n"
                        f"thread '{name}' panicked at {source}:80:5:\n"
                        "assertion `left == right` failed\nfailures:\n")
                    (directory / "cause.log").write_text(text)
                    result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy,
                                                    subject="diagnostics")
                    cause = result["failures"]["diagnostic_guard"]["causes"][0]
                    self.assertEqual((cause["kind"], cause["source"], cause["line"]),
                                     ("assertion", source, 80))
                    for bad in (text.replace(source, "dependency.rs"),
                                text.replace(name, "unlisted_test"),
                                text.replace("assertion `left == right` failed", "unrelated panic")):
                        (directory / "cause.log").write_text(bad)
                        with self.subTest(schema=policy["schema_version"], source=source), self.assertRaisesRegex(
                                MODULE.Incomplete, "actual selected behavioral test cause"):
                            MODULE.validate_causes(directory, {"outcomes": [row]}, policy,
                                                   subject="diagnostics")

    def test_schema6_read_policy_requires_its_own_exact_registration_and_fresh_binding(self):
        policy = policy_fixture(diagnostics=True); identity = {"one": "b" * 64}
        policy["binding"]["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
        source = "crates/windows-platform/src/diagnostics.rs"
        healthy = (f'critical_files=["{source}"]\n'
                   f'security_modules=[{{package="windows-platform",files=["{source}"]}}]\n')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); (root / "policy").mkdir()
            manifest = root / "policy/verification-manifest.toml"; manifest.write_text(healthy)
            with patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
                    patch.object(MODULE.COMMON.PRODUCT, "admit_product", return_value=({}, identity)), \
                    patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64):
                MODULE.read_policy(root, campaign=False)
                for text in (healthy.replace(f'critical_files=["{source}"]', 'critical_files=[]'),
                             healthy.replace(f'critical_files=["{source}"]', f'critical_files=["{source}","{source}"]'),
                             healthy.replace('package="windows-platform"', 'package="other"'),
                             healthy.replace('security_modules=[', f'security_modules=[{{package="windows-platform",files=["{source}"]}},'),
                             healthy.replace(source, "crates/windows-platform/src/service.rs")):
                    manifest.write_text(text)
                    with self.subTest(manifest=text), self.assertRaisesRegex(MODULE.Incomplete, "registration"):
                        MODULE.read_policy(root, campaign=False)
                manifest.write_text(healthy)
                with patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="c" * 64), \
                        self.assertRaisesRegex(MODULE.Incomplete, "changed after freezing"):
                    MODULE.read_policy(root, campaign=False)

    def test_schema6_compiler_target_and_source_must_belong_to_the_same_owner(self):
        """A crossed compiler record supplies no basis for excluding a viable fault."""
        policy = policy_fixture(diagnostics=True)
        row = outcome("Unviable")
        row["scenario"]["Mutant"].update(
            name="diagnostic_guard", file="crates/windows-platform/src/diagnostics.rs")
        bad_pairs = [
            ("diagnostics", "crates/windows-platform/src/service.rs"),
            ("windows_platform", "crates/windows-platform/tests/diagnostics.rs"),
            ("diagnostics", "crates/windows-platform/src/diagnostics.rs"),
            ("diagnostics", "crates/windows-platform/src/diagnostics_tests.rs"),
            ("windows_platform", "crates/windows-platform/src/scm.rs"),
        ]
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            for target, source in bad_pairs:
                message = {"reason": "compiler-message", "target": {"name": target},
                    "message": {"level": "error", "spans": [
                        {"is_primary": True, "file_name": source}]}}
                (directory / "cause.log").write_text(json.dumps(message))
                with self.subTest(target=target, source=source), self.assertRaisesRegex(
                        MODULE.Incomplete, "foreign compiler"):
                    MODULE.validate_causes(directory, {"outcomes": [row]}, policy,
                                           subject="diagnostics")

    def test_schema6_execution_passes_only_the_new_subject_and_all_bound_inputs_to_custody(self):
        policy = policy_fixture(diagnostics=True); identity = {"one": "b" * 64}
        policy["binding"]["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
        observed = SimpleNamespace(finished=False)
        class Session:
            def __init__(self, root, args, report, scope, selected_policy, inputs):
                observed.scope, observed.inputs, observed.lane = scope, inputs, args.lane
                self.product_identity = identity; self.work = Path("/owned/work")
                self.source = Path("/owned/source"); self.tools = Path("/owned/tools")
                self.run = lambda *a, **k: None
            def finish(self): observed.finished = True
        listing = SimpleNamespace(inventories={"diagnostics": {"diagnostic_guard": {"name": "diagnostic_guard"}}})
        report = {}
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64), \
                patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
                patch.object(MODULE.COMMON, "enumerate_mutation_inventories", return_value=listing) as enumerate_, \
                patch.object(MODULE.COMMON, "run_analysis_lanes") as campaigns:
            MODULE.execute(SimpleNamespace(lane="enumerate"), report)
        self.assertTrue(observed.finished)
        self.assertEqual(observed.lane, "mutation")
        self.assertEqual(observed.scope.package, "windows-platform")
        self.assertEqual(observed.scope.subjects, (("diagnostics", "crates/windows-platform/src/diagnostics.rs"),))
        for path in ("crates/windows-platform/src/diagnostics.rs",
                     "crates/windows-platform/src/diagnostics_tests.rs", "crates/windows-platform/tests/diagnostics.rs"):
            self.assertIn(path, observed.inputs)
        self.assertEqual(enumerate_.call_args.args[1], {"diagnostics": "crates/windows-platform/src/diagnostics.rs"})
        self.assertEqual(set(report["listed_inventory_bindings"]), {"diagnostics"})
        self.assertFalse(report["qualification_passed"]); self.assertFalse(report["campaign_admission"])
        campaigns.assert_not_called()

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

    def test_schema7_requires_fresh_subject_artifacts_in_each_viable_build(self):
        policy = policy_fixture(diagnostics=True, codec=True)
        root = "/work/tmp/cargo-mutants-source-control.tmp/crates/windows-platform"
        artifacts = [{"reason": "compiler-artifact", "package_id": f"path+file://{root}#0.0.0",
                      "manifest_path": f"{root}/Cargo.toml", "target": {
                          "name": name, "kind": [kind], "src_path": f"{root}/{source}"},
                      "profile": {"test": test}, "fresh": False}
                     for name, kind, source, test in (
                         ("windows_platform", "lib", "src/lib.rs", False),
                         ("windows_platform", "lib", "src/lib.rs", True),
                         ("diagnostics", "test", "tests/diagnostics.rs", True))]
        caught = outcome("CaughtMutant")
        caught["scenario"]["Mutant"].update(name="diagnostics_guard", file=MODULE.DIAGNOSTICS_SOURCE)
        missed = outcome("MissedMutant")
        missed["scenario"]["Mutant"].update(name="diagnostics_survivor", file=MODULE.DIAGNOSTICS_SOURCE)
        missed["log_path"] = "survivor.log"
        baseline = {"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success"),
                    "log_path": "baseline.log"}
        assertion = ("test healthy_readiness ... FAILED\n---- healthy_readiness stdout ----\n"
                     "thread 'healthy_readiness' panicked at crates/windows-platform/tests/diagnostics.rs:55:5:\n"
                     "assertion `left == right` failed\nfailures:\n")

        def log(build_artifacts, test_artifacts=(), *, behavior=assertion):
            build, test = ("*** " + " ".join(phase["argv"]) for phase in caught["phase_results"])
            return "\n".join([build, *(json.dumps(row) for row in build_artifacts),
                             "*** result: Success", test, *(json.dumps(row) for row in test_artifacts), behavior])

        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            directory = work / "mutation/diagnostics/mutants.out"
            directory.mkdir(parents=True)
            (directory / "baseline.log").write_text(execution(doctests=True))
            (directory / "outcomes.json").write_text(json.dumps({"outcomes": [baseline, caught, missed]}))
            (directory / "survivor.log").write_text(log(artifacts, behavior=execution()))
            path = directory / "cause.log"
            path.write_text(log(artifacts))
            result = MODULE.validate_baseline(work, policy, subject="diagnostics")
            self.assertEqual(result["causes"]["failures"]["diagnostics_guard"]["kind"], "behavioral")
            foreign = {**artifacts[2], "package_id": "path+file:///work/foreign#0.0.0"}
            cases = (
                ("stale", [artifacts[0], {**artifacts[1], "fresh": True}, artifacts[2]], (), "stale"),
                ("missing", artifacts[:2], (), "complete unique"),
                ("wrong phase", artifacts[:2], artifacts[2:], "complete unique"),
                ("duplicate", [*artifacts, artifacts[2]], (), "complete unique"),
                ("foreign package", [*artifacts[:2], foreign], (), "complete unique"),
            )
            for name, build_artifacts, test_artifacts, message in cases:
                with self.subTest(name=name):
                    path.write_text(log(build_artifacts, test_artifacts))
                    with self.assertRaisesRegex(MODULE.Incomplete, message):
                        MODULE.validate_baseline(work, policy, subject="diagnostics")
            path.write_text(log(artifacts))
            (directory / "survivor.log").write_text(log(artifacts[:2], behavior=execution()))
            with self.assertRaisesRegex(MODULE.Incomplete, "complete unique.*diagnostics_survivor"):
                MODULE.validate_baseline(work, policy, subject="diagnostics")
            legacy = policy_fixture(diagnostics=True)
            MODULE.validate_baseline(work, legacy, subject="diagnostics")

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
                          "RemovedEntry", "EnginePrivilegeError { privilege: Debug, state: PresentDisabled }",
                          "Forbidden(EnginePrivilegeError { privilege: Debug, state: PresentDisabled })",
                          "MissingRequired(ChangeNotify)", "MissingRequired(SystemTime)",
                          "UnexpectedPrivilege", "BrokerClockEnabled"):
                (directory / "cause.log").write_text(f"test {name} ... FAILED\n---- {name} stdout ----\nError: {error}\nfailures:\n")
                result = MODULE.validate_causes(directory, {"outcomes": [row]}, policy, subject="privilege")
                self.assertEqual(result["failures"]["privilege_guard"]["causes"][0]["kind"], "typed-positive-path-refusal")
            for error in ("Unknown", "MissingRequired(Unknown)", "UnexpectedPrivilegeOther",
                          "BrokerClockEnabledOther", "Forbidden(Unknown)",
                          "Forbidden(EnginePrivilegeError { privilege: ChangeNotify, state: Absent })"):
                with self.subTest(error=error):
                    (directory / "cause.log").write_text(f"test {name} ... FAILED\n---- {name} stdout ----\nError: {error}\nfailures:\n")
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




class CodecAnalysisControls(unittest.TestCase):
    def setUp(self):
        MODULE.codec_module()

    def test_legacy_roles_never_load_the_new_unbound_codec_helper(self):
        original_spec = importlib.util.spec_from_file_location

        def refuse_unbound_helper(name, path, *args, **kwargs):
            if Path(path).name == "service_diagnostics_analysis.py":
                raise AssertionError("legacy role executed unbound codec helper")
            return original_spec(name, path, *args, **kwargs)

        with patch.object(importlib.util, "spec_from_file_location", side_effect=refuse_unbound_helper):
            spec = original_spec("legacy_service_without_codec", ROOT / "scripts/verify-service-analysis.py")
            legacy = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(legacy)
            for diagnostics in (False, True):
                policy = policy_fixture(diagnostics=diagnostics)
                legacy.validate_policy(policy)
                self.assertNotIn("scripts/service_diagnostics_analysis.py", legacy.bound_inputs(policy))
            self.assertIsNone(legacy.CODEC)

    def test_codec_closure_retains_the_exact_required_native_notice(self):
        notice = "policy/dependency-notices/libfuzzer-sys-0.4.13-LLVM-LICENSE.txt"
        self.assertIn(notice, MODULE.bound_inputs(policy_fixture(codec=True)))
        self.assertEqual(MODULE.CODEC.INPUTS.count(notice), 1)
        self.assertEqual(MODULE.COMMON.PRODUCT.sha256(ROOT / notice),
                         "1a8f1058753f1ba890de984e48f0242a3a5c29a6a8f2ed9fd813f36985387e8d")

    """Synthetic harness controls; none qualifies a product/compiler/fuzz run."""

    def test_schema7_specs_and_source_binding_are_mandatory_and_exact(self):
        healthy = policy_fixture(codec=True)
        MODULE.validate_policy(healthy)
        for name in ("policy/service-diagnostics-fuzz-manifest.toml", "policy/service-diagnostics-fuzz.lock",
                     "policy/service-diagnostics-fuzz-target.rs", "scripts/service_diagnostics_analysis.py",
                     "scripts/tests/test_verify_service_analysis.py", "scripts/tests/test_portable_analysis.py"):
            self.assertIn(name, MODULE.bound_inputs(healthy))
        for key in ("coverage_guided_fuzzing", "external_contracts"):
            for change in (None, {}, {**healthy[key], "foreign": True}):
                with self.subTest(key=key, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_policy({**healthy, key: change})
        for key, value in (("target", "x86_64-unknown-linux-musl"), ("native_workers", True),
                           ("features", ["seeded-defect"]), ("flags", ["-runs=0"]),
                           ("maximum_callbacks", 0), ("cargo_fuzz_sha256", "a" * 64)):
            changed = copy.deepcopy(healthy)
            changed["coverage_guided_fuzzing"][key] = value
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy(changed)
        for schema in (5, 6):
            old = policy_fixture(diagnostics=schema == 6)
            MODULE.validate_policy(old)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_policy({**old, "coverage_guided_fuzzing": healthy["coverage_guided_fuzzing"]})
        broken = copy.deepcopy(healthy)
        broken["binding"]["inputs_sha256"].pop("policy/service-diagnostics-fuzz-target.rs")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_policy(broken)

    def test_callback_observations_require_real_parity_counts_and_ordinal(self):
        text = ("LOGGER_CODEC_CALLBACK ordinal=1 length=128 expected=accepted actual=accepted calls=1\n"
                "LOGGER_CODEC_CALLBACK ordinal=2 length=128 expected=semantic actual=semantic calls=1\nDone 256 runs\n")
        result = MODULE.CODEC.callback_observations(text, accepted=True, rejected=True)
        self.assertEqual((result["callback_count"], result["accepted"], result["rejected"]), (2, 1, 1))
        for bad in ("Done 256 runs", text.replace("calls=1", "calls=0"), text.replace("ordinal=2", "ordinal=1"),
                    text.replace("length=128", "length=4097"), text.replace("actual=semantic", "actual=accepted"),
                    text.replace("actual=semantic", "actual=unexpected"), text.splitlines()[0]):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.CODEC.callback_observations(bad, accepted=True, rejected=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.CODEC.callback_observations(text, expected_class="accepted")
        with patch.object(MODULE.CODEC.time, "monotonic", return_value=100.0):
            self.assertEqual(MODULE.CODEC.remaining(105.0, 60), 5)
            for deadline in (100.0, 99.0, float("inf"), float("nan")):
                with self.subTest(deadline=deadline), self.assertRaises(MODULE.Incomplete):
                    MODULE.CODEC.remaining(deadline, 60)

    @staticmethod
    def metadata_fixture():
        selected = MODULE.CODEC.FUZZ_SPEC
        packages, nodes = [], []
        locked = {row["name"]: row for row in selected["locked_registry_packages"]}
        # The full lock includes jobserver's Windows RNG and its UEFI leaf.
        # Cargo's GNU-filtered metadata excludes both from packages and resolve.
        for name in [*(name for name in locked if name not in {"getrandom", "r-efi"}),
                     "windows-platform", "windows-platform-diagnostics-fuzz"]:
            version = locked.get(name, {}).get("version", "0.0.0")
            packages.append({"id": name, "name": name, "version": version,
                "source": locked.get(name, {}).get("source"), "edition": "2024",
                "manifest_path": "/work/fuzz/Cargo.toml" if name.endswith("-fuzz") else "/work/source/crates/windows-platform/Cargo.toml",
                "targets": [{"name": "codec", "kind": ["bin"], "src_path": "/work/fuzz/target.rs"}]})
            nodes.append({"id": name, "features": selected["gnu_reachable_features"].get(name, []),
                "deps": [{"pkg": child} for child in selected["gnu_dependency_edges"].get(name, [])]})
        return {"packages": packages, "resolve": {"root": "windows-platform-diagnostics-fuzz", "nodes": nodes}}

    def test_complete_graph_rejects_wrong_source_version_features_edges_and_target(self):
        healthy = self.metadata_fixture()
        MODULE.CODEC.fuzz_metadata(healthy, MODULE.CODEC.FUZZ_SPEC)
        cases = []
        for key, value in (("version", "0.0.1"), ("source", "registry+foreign")):
            bad = copy.deepcopy(healthy); bad["packages"][0][key] = value; cases.append(bad)
        for key, value in (("name", "foreign"), ("kind", ["lib"]), ("src_path", "/work/foreign.rs")):
            bad = copy.deepcopy(healthy); bad["packages"][-1]["targets"][0][key] = value; cases.append(bad)
        bad = copy.deepcopy(healthy); bad["resolve"]["nodes"][-1]["features"] = ["forwarding-defect"]; cases.append(bad)
        bad = copy.deepcopy(healthy); bad["resolve"]["nodes"][-1]["deps"] = []; cases.append(bad)
        bad = copy.deepcopy(healthy); bad["packages"].pop(0); cases.append(bad)
        bad = copy.deepcopy(healthy)
        bad["packages"].append({"id": "r-efi", "name": "r-efi", "version": "6.0.0",
            "source": "registry+https://github.com/rust-lang/crates.io-index"})
        bad["resolve"]["nodes"].append({"id": "r-efi", "features": [], "deps": []})
        cases.append(bad)
        bad = copy.deepcopy(healthy); bad["resolve"]["root"] = "windows-platform"; cases.append(bad)
        for bad in cases:
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.CODEC.fuzz_metadata(bad, MODULE.CODEC.FUZZ_SPEC)

    def test_explicit_codec_dispatch_keeps_all_metrics_and_late_custody_failure(self):
        policy = policy_fixture(codec=True)
        identity = {"source": "a" * 64}
        policy["binding"]["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
        class Session:
            fail_finish = False
            def __init__(self, root, args, report, scope, actual, inputs):
                self.source, self.product_identity = root, identity
                self.work, self.tools = root, root
                self.args = args
            def run(self, *args, **kwargs):
                raise AssertionError("Synthetic dispatch must not execute commands")
            def finish(self):
                if self.fail_finish:
                    raise MODULE.Incomplete("changed final custody")
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", Session), \
             patch.object(MODULE.COMMON.PRODUCT, "sha256", return_value="b" * 64), \
             patch.object(MODULE.COMMON, "read_analysis_json", return_value=policy), \
             patch.object(MODULE.CODEC, "contracts") as contracts, patch.object(MODULE.CODEC, "fuzz") as fuzz, \
             patch.object(MODULE.COMMON, "run_analysis_lanes") as metrics:
            for lane, called in (("contracts", contracts), ("fuzz", fuzz), ("all", metrics)):
                report = {}
                MODULE.execute(SimpleNamespace(lane=lane, deadline_monotonic=MODULE.time.monotonic() + 60.0), report)
                self.assertEqual(report["metrics_passed"], lane == "all")
                self.assertEqual(called.call_count, 1)
            contracts.reset_mock(); fuzz.reset_mock(); metrics.reset_mock()
            Session.fail_finish = True
            with self.assertRaisesRegex(MODULE.Incomplete, "final custody"):
                MODULE.execute(SimpleNamespace(lane="fuzz", deadline_monotonic=MODULE.time.monotonic() + 60.0), {})
            self.assertEqual(fuzz.call_count, 1)
            self.assertFalse(metrics.called)
            primary = MODULE.Incomplete("original codec graph mismatch")
            fuzz.side_effect = primary
            report = {}
            with self.assertRaises(MODULE.Incomplete) as refused:
                MODULE.execute(SimpleNamespace(lane="fuzz", deadline_monotonic=MODULE.time.monotonic() + 60.0), report)
            self.assertIs(refused.exception, primary)
            self.assertEqual(report["secondary_failure"], {
                "stage": "session.finish", "type": "Incomplete", "error": "changed final custody"})
            self.assertFalse(report["metrics_passed"])
            self.assertFalse(report["qualification_passed"])




class CodecContractDispatchControls(unittest.TestCase):
    """Keep raw refusal validators in a synthetic stable orchestration control."""

    def setUp(self):
        MODULE.codec_module()

    @staticmethod
    def diagnostic(path, line, active, passive):
        span = {"file_name": path, "byte_start": 0, "byte_end": 1, "line_start": line, "line_end": line,
            "column_start": 1, "column_end": 2, "is_primary": True,
            "text": [{"text": "fixture", "highlight_start": 1, "highlight_end": 2}], "label": None,
            "suggested_replacement": None, "suggestion_applicability": None, "expansion": None}
        return {"$message_type": "diagnostic", "message": f"expected `{active}`, found `{passive}`",
            "code": {"code": "E0308", "explanation": None}, "level": "error", "spans": [span],
            "children": [], "rendered": "error[E0308]: mismatched types\n"}

    def test_stable_contracts_retain_raw_cause_and_execute_legal_neighbor(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            deps = work / "contract-deps"; deps.mkdir()
            module = MODULE
            state = SimpleNamespace(mode="healthy", executions=0)
            class Session:
                def __init__(self):
                    self.work, self.stable, self.source = work, work / "stable", work / "source"
                    self.report = {"lanes": []}
                def protect(self, path):
                    if not path.is_dir():
                        raise AssertionError("Common protection requires a directory")
                def run(self, label, command, timeout, **options):
                    if label == "codec-contract-library":
                        return json.dumps({"reason": "compiler-artifact", "manifest_path": "/work/source/crates/windows-platform/Cargo.toml",
                            "package_id": "path+file:///work/source/crates/windows-platform#0.0.0",
                            "target": {"name": "windows_platform", "kind": ["lib"]},
                            "filenames": ["/work/target/debug/libwindows_platform.rlib"]})
                    if options.get("expected_error"):
                        path = command[command.index("--error-format=json") + 1]
                        component = "component_claim" in path
                        active, passive = ("Component", "ComponentClaim") if component else ("ServiceEvent", "ServiceEventClaim")
                        line = 5 if component else 6
                        diagnostic = CodecContractDispatchControls.diagnostic(path, line, active, passive)
                        if state.mode == "foreign": diagnostic["spans"][0]["file_name"] = "/work/foreign.rs"
                        if state.mode == "wrong-cause": diagnostic["message"] = "expected u8, found u16"
                        raw = {"exit_code": 1, "failure": None, "output": json.dumps(diagnostic)}
                        module.COMMON.privacy_error(raw, "E0308")
                        self.report["lanes"].append({"lane": label, **raw})
                        return raw["output"]
                    if label == "codec-contract-legal-build":
                        (work / "contracts/codec_legal").write_bytes(b"synthetic legal binary")
                        return ""
                    if label == "codec-contract-legal-execution":
                        state.executions += 1
                        return "LOGGER_CODEC_LEGAL_PASS\n" if state.mode != "zero-legal" else ""
                    raise AssertionError("Unexpected compiler control dispatch")
            with patch.object(MODULE.COMMON, "copy_cargo_contract_dependencies", return_value=deps):
                report = {}
                MODULE.CODEC.contracts(Session(), MODULE.CODEC.CONTRACT_SPEC, report, MODULE.time.monotonic() + 60)
                self.assertEqual(report["external_contracts"]["negative_count"], 2)
                self.assertEqual(state.executions, 1)
                for mode in ("foreign", "wrong-cause", "zero-legal"):
                    for name in ("contract-inputs", "contracts"):
                        import shutil
                        shutil.rmtree(work / name)
                    state.mode = mode
                    with self.subTest(mode=mode), self.assertRaises(MODULE.Incomplete):
                        MODULE.CODEC.contracts(Session(), MODULE.CODEC.CONTRACT_SPEC, {}, MODULE.time.monotonic() + 60)


if __name__ == "__main__":
    unittest.main()
