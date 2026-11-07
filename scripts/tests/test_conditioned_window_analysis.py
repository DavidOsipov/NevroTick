"""Independent two-owner admission controls; never execute Cargo or analyzers."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("conditioned_window_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
N1_CASES = frozenset({"exact_conditioning_control", "conditioned_window::tests::validator_fault_control"})
N1_DOCS = (("crates/time-core/src/conditioned_window.rs", "conditioned_window::PreparedConditionedBatch", False, 1),
           ("crates/time-core/src/conditioned_window.rs", "conditioned_window::PreparedConditionedBatch", True, 1))


def digest(value):
    """Encode the documented canonical JSON independently of the shared helper."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def native_log(names):
    """Literal native test syntax, without running any product entry point."""
    return ("running " + str(len(names)) + " tests\n"
            + "".join("test " + name + " ... ok\n" for name in names)
            + "test result: ok. " + str(len(names))
            + " passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


class ConditionedWindowAnalysisTests(unittest.TestCase):
    """Discriminate custody, exact selections, two-owner bounds and failed lanes."""

    def setUp(self):
        """Synthetic code-owned minima test admission mechanics, not product truth."""
        self.addCleanup(patch.stopall)
        patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_TESTS", N1_CASES).start()
        patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_DOCTESTS", N1_DOCS).start()

    def inventories(self):
        """Keep selected/private controls and distinct source-owned privacy examples."""
        ordinary = sorted(N1_CASES | MODULE.WINDOW_VARIANCE_REQUIRED_TESTS)
        docs = ["crates/time-core/src/conditioned_window.rs - conditioned_window::PreparedConditionedBatch (line 10)",
                "crates/time-core/src/conditioned_window.rs - conditioned_window::PreparedConditionedBatch (line 20) - compile fail",
                "crates/time-core/src/lib.rs - (line 20)",
                "crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line 10)"]
        docs += ["crates/time-core/src/window_variance.rs - window_variance::WindowVarianceDivisors (line "
                 + str(index) + ") - compile fail" for index in (20, 30, 40, 50, 60)]
        return {"coverage": ordinary, "baseline": sorted([*ordinary, *docs, "historical_package_control"])}

    def fixture(self, root):
        """Copy actual admitted inputs and bind a synthetic nonexecuted candidate."""
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        names = set(identity) | set(MODULE.CONDITIONED_WINDOW_BOUND_INPUTS)
        for name in sorted(names):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            if (ROOT / name).is_file():
                shutil.copyfile(ROOT / name, target)
            else:
                target.write_text("// Synthetic unexecuted source-admission fixture.\n")
        manifest = root / "policy/verification-manifest.toml"
        text = manifest.read_text()
        for index, filename in enumerate(MODULE.CONDITIONED_WINDOW_SUBJECTS.values()):
            if filename not in text:
                text = text.replace("security_modules = [", "security_modules = [\n"
                    '    { name = "synthetic-n1-owner-' + str(index)
                    + '", package = "time-core", files = ["' + filename
                    + '"], scope = "unexecuted admission control" },', 1)
                text = text.replace("critical_files = [", 'critical_files = [\n    "' + filename + '",', 1)
        manifest.write_text(text)
        _, identity = MODULE.PRODUCT.admit_product(root)
        policy = {**copy.deepcopy(MODULE.CONDITIONED_WINDOW_FIXED), "source_file_count": len(identity),
                  "candidate_sha256": digest(identity),
                  "inputs_sha256": {name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                                    for name in MODULE.CONDITIONED_WINDOW_BOUND_INPUTS},
                  "mutation_inventories": {}, "mutation_equivalences": [],
                  "test_inventories": self.inventories(),
                  "scope": "Synthetic unexecuted two-owner admission controls."}
        self.write(root, policy)
        return policy

    def write(self, root, policy):
        """Publish only a temporary fixture policy."""
        path = root / MODULE.CONDITIONED_WINDOW_POLICY
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(policy))

    def bound_inventories(self):
        """Synthetic complete descriptor counts establish no actual mutant score."""
        return {name: {"count": 1, "sha256": "a" * 64} for name in MODULE.CONDITIONED_WINDOW_SUBJECTS}

    def test_healthy_candidate_uses_both_exact_owners_and_preserves_historical_scopes(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy = self.fixture(root)
            self.assertEqual(MODULE.read_conditioned_window_policy(root), policy)
            self.assertEqual(MODULE.CONDITIONED_WINDOW_SCOPE.select("all"), {
                "conditioned_window": "crates/time-core/src/conditioned_window.rs",
                "window_variance": "crates/time-core/src/window_variance.rs"})
            self.assertEqual(MODULE.WINDOW_VARIANCE_SCOPE.select("all"), {
                "window_variance": "crates/time-core/src/window_variance.rs"})
            self.assertEqual(MODULE.WINDOW_VARIANCE_FIXED["source_file_count"], 370)
            self.assertNotIn("conditioned_window", MODULE.SUBJECTS)
            self.assertEqual(policy["mutation_equivalences"], [])

    def test_policy_cannot_weaken_scope_thresholds_workers_budget_or_types(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            healthy = self.fixture(root)
            for field, value in [("package", "nevrotick-engine"), ("critical_files", []),
                                 ("source_file_count", True), ("coverage_minimum_line_percent", 89),
                                 ("mutation_minimum_viable_killed_percent", 94), ("jobs", True),
                                 ("test_threads", 2), ("mutation_jobs", 9),
                                 ("mutation_minimum_free_bytes", 5 * 1024 ** 3 - 1),
                                 ("mutation_process_limit", 511), ("maximum_mutants", 513),
                                 ("baseline_wall_seconds", 301), ("mutation_test_seconds", 61),
                                 ("tool_admission_profile", "stable-core"), ("unknown", 1)]:
                bad = copy.deepcopy(healthy)
                bad[field] = value
                self.write(root, bad)
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_conditioned_window_policy(root)

    def test_candidate_and_protected_inputs_fail_closed_after_local_pin_only_refresh(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            healthy = self.fixture(root)
            source = root / MODULE.CONDITIONED_WINDOW_TEST_FILE
            source.write_bytes(source.read_bytes() + b"// changed\n")
            bad = copy.deepcopy(healthy)
            bad["inputs_sha256"][MODULE.CONDITIONED_WINDOW_TEST_FILE] = hashlib.sha256(source.read_bytes()).hexdigest()
            self.write(root, bad)
            with self.assertRaisesRegex(MODULE.Incomplete, "complete product candidate"):
                MODULE.read_conditioned_window_policy(root)
            source.write_bytes(source.read_bytes().removesuffix(b"// changed\n"))
            for change in ("missing", "extra", "content"):
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["inputs_sha256"].pop("scripts/portable-analysis.py")
                if change == "extra": bad["inputs_sha256"]["foreign.py"] = "a" * 64
                if change == "content": (root / "scripts/portable-analysis.py").write_text("# changed helper\n")
                self.write(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "helper bindings"):
                    MODULE.read_conditioned_window_policy(root)

    def test_policy_cannot_supply_arbitrary_names_instead_of_code_owned_cases(self):
        policy = {"test_inventories": self.inventories()}
        self.assertEqual(MODULE.conditioned_window_test_inventories(policy), policy["test_inventories"])
        for change in ("missing", "private", "duplicate", "unsorted", "unknown_doc", "wrong_doc", "line_zero"):
            bad = copy.deepcopy(policy)
            if change == "missing": bad["test_inventories"]["coverage"] = ["arbitrary_test"]
            if change == "private":
                bad["test_inventories"]["coverage"].remove("conditioned_window::tests::validator_fault_control")
            if change == "duplicate": bad["test_inventories"]["baseline"].append(bad["test_inventories"]["baseline"][0])
            if change == "unsorted": bad["test_inventories"]["coverage"].reverse()
            if change == "unknown_doc":
                bad["test_inventories"]["baseline"].append("crates/time-core/src/conditioned_window.rs - Other (line 40)")
                bad["test_inventories"]["baseline"].sort()
            if change in {"wrong_doc", "line_zero"}:
                names = bad["test_inventories"]["baseline"]
                index = names.index("crates/time-core/src/conditioned_window.rs - conditioned_window::PreparedConditionedBatch (line 10)")
                names[index] = (names[index].replace("PreparedConditionedBatch", "Other") if change == "wrong_doc"
                                else names[index].replace("line 10", "line 0"))
                names.sort()
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.conditioned_window_test_inventories(bad)
        with patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_TESTS", frozenset()), self.assertRaises(MODULE.Incomplete):
            MODULE.conditioned_window_test_inventories(policy)

    def test_actual_execution_requires_every_exact_name_and_complete_success(self):
        policy = {"test_inventories": self.inventories()}
        for doctests, lane in [(False, "coverage"), (True, "baseline")]:
            names = policy["test_inventories"][lane]
            healthy = native_log(names)
            self.assertEqual(MODULE.conditioned_window_test_execution(healthy, policy, doctests=doctests)
                             ["tests_executed"], len(names))
            for bad in (native_log(names[1:]), native_log([*names, "unreviewed_extra"]),
                        healthy.replace(" ... ok", " ... ignored", 1),
                        healthy.replace("0 failed;", "1 failed;", 1),
                        healthy.replace("0 filtered out;", "1 filtered out;", 1),
                        healthy.replace(" ... ok", " ... FAILED", 1),
                        "\n".join(names), healthy + "ok\n"):
                with self.subTest(lane=lane, bad=bad[-80:]), self.assertRaises(MODULE.Incomplete):
                    MODULE.conditioned_window_test_execution(bad, policy, doctests=doctests)

    def test_split_private_telemetry_cannot_replace_named_success(self):
        policy = {"test_inventories": self.inventories()}
        healthy = native_log(policy["test_inventories"]["coverage"])
        split = healthy.replace("validator_fault_control ... ok", "validator_fault_control ... CORE_N1_PROFILE cases=1\nok")
        MODULE.conditioned_window_test_execution(split, policy, doctests=False)
        for bad in (split.replace("\nok\n", "\n"), split.replace("\nok\n", "\nFAILED\n"),
                    split.replace("\nok\n", "\nunattributed\nok\n")):
            with self.subTest(bad=bad[-60:]), self.assertRaises(MODULE.Incomplete):
                MODULE.conditioned_window_test_execution(bad, policy, doctests=False)

    def test_both_nonempty_inventories_share_the_512_campaign_cap(self):
        policy = {**MODULE.CONDITIONED_WINDOW_FIXED, "mutation_inventories": {}}
        self.assertEqual(MODULE.conditioned_window_inventory_bindings(policy), {})
        healthy = self.bound_inventories()
        for bindings in ({}, {"conditioned_window": healthy["conditioned_window"]},
                         {**healthy, "foreign": healthy["conditioned_window"]},
                         {**healthy, "window_variance": {"count": True, "sha256": "a" * 64}},
                         {**healthy, "window_variance": {"count": 0, "sha256": "a" * 64}},
                         {**healthy, "window_variance": {"count": 512, "sha256": "a" * 64}},
                         {**healthy, "window_variance": {"count": 1, "sha256": "bad"}}):
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.conditioned_window_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = healthy
        self.assertEqual(MODULE.conditioned_window_inventory_bindings(policy, required=True), healthy)

    def test_baseline_reads_actual_raw_log_and_rejects_missing_duplicate_escape_and_links(self):
        policy = {"test_inventories": self.inventories()}
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            for subject in MODULE.CONDITIONED_WINDOW_SUBJECTS:
                directory = work / "mutation" / subject / "mutants.out"
                directory.mkdir(parents=True)
                (directory / "baseline.log").write_text(native_log(policy["test_inventories"]["baseline"]))
                baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"}
                output = directory / "outcomes.json"
                output.write_text(json.dumps({"outcomes": [baseline]}))
                self.assertGreater(MODULE.conditioned_window_baseline(work, policy, subject)["doctests"], 0)
                (directory / "linked.log").symlink_to(directory / "baseline.log")
                for rows in ([], [baseline, baseline], [{**baseline, "summary": "Failure"}],
                             [{**baseline, "log_path": "../escape.log"}],
                             [{**baseline, "log_path": "/absolute.log"}],
                             [{**baseline, "log_path": "linked.log"}]):
                    output.write_text(json.dumps({"outcomes": rows}))
                    with self.subTest(subject=subject, rows=rows), self.assertRaises(MODULE.Incomplete):
                        MODULE.conditioned_window_baseline(work, policy, subject)

    def test_cli_and_direct_dispatch_are_separate_from_historical_families(self):
        with patch.object(MODULE, "main_conditioned_window", return_value=0) as selected, \
             patch.object(MODULE, "execute") as historical:
            self.assertEqual(MODULE.main(["--subject", "conditioned_window", "--lane", "baseline",
                                          "--output-dir", "/new/output"]), 0)
            self.assertEqual(selected.call_args.args[0].tool_admission_profile, "full")
            historical.assert_not_called()
        with patch.object(MODULE, "execute_conditioned_window") as selected, patch.object(MODULE, "read_policy") as historical:
            MODULE.execute(SimpleNamespace(subject="conditioned_window"), {})
            selected.assert_called_once()
            historical.assert_not_called()

    def test_generated_baseline_paths_reach_only_the_bounded_nofollow_reader(self):
        policy = {"test_inventories": self.inventories()}
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            directory = work / "mutation/conditioned_window/mutants.out"
            directory.mkdir(parents=True)
            raw = native_log(policy["test_inventories"]["baseline"]).encode()
            (directory / "baseline.log").write_bytes(raw)
            baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"}
            output = directory / "outcomes.json"
            output.write_text(json.dumps({"outcomes": [baseline]}))
            reader = MODULE.COMMON.read_analysis_bytes
            with patch.object(Path, "resolve", side_effect=AssertionError("unbounded generated-path resolution")), \
                 patch.object(Path, "open", side_effect=AssertionError("unbounded generated-path open")), \
                 patch.object(Path, "read_bytes", side_effect=AssertionError("unbounded generated-path read")), \
                 patch.object(MODULE.COMMON, "read_analysis_bytes", wraps=reader) as reads:
                result = MODULE.conditioned_window_baseline(work, policy, "conditioned_window")
                self.assertEqual(result["log_sha256"], hashlib.sha256(raw).hexdigest())
                self.assertEqual([call.args[0] for call in reads.call_args_list],
                                 [output, directory / "baseline.log"])
                self.assertEqual(reads.call_args.kwargs["maximum"], 16 * 1024 * 1024)
            logs = directory / "logs"
            logs.mkdir()
            (logs / "actual.log").write_bytes(raw)
            (directory / "linked-directory").symlink_to(logs, target_is_directory=True)
            output.write_text(json.dumps({"outcomes": [{**baseline, "log_path": "linked-directory/actual.log"}]}))
            with patch.object(Path, "resolve", side_effect=AssertionError("link traversal before admission")), \
                 patch.object(Path, "open", side_effect=AssertionError("link open before admission")), \
                 self.assertRaisesRegex(MODULE.Incomplete, "custody"):
                MODULE.conditioned_window_baseline(work, policy, "conditioned_window")

    def test_wrong_roles_and_unreviewed_mutation_reject_before_session(self):
        args = SimpleNamespace(subject="conditioned_window", lane="coverage", tool_admission_profile="full")
        for key, value in [("subject", "all"), ("lane", "contracts"),
                           ("tool_admission_profile", "stable-core"), ("cohort_plan", Path("/foreign"))]:
            bad = copy.copy(args)
            setattr(bad, key, value)
            with self.subTest(key=key), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete): MODULE.execute_conditioned_window(bad, {})
                session.assert_not_called()
        with patch.object(MODULE, "read_conditioned_window_policy", return_value={
                **MODULE.CONDITIONED_WINDOW_FIXED, "mutation_inventories": {}}), \
             patch.object(MODULE.COMMON, "Session") as session:
            args.lane = "mutation"
            with self.assertRaises(MODULE.Incomplete): MODULE.execute_conditioned_window(args, {})
            session.assert_not_called()

    def mutant(self, filename):
        """A complete synthetic descriptor tests listing without generated-fault credit."""
        return {"name": filename + ":2:1: synthetic fault", "package": "time-core", "file": filename,
                "function": {"function_name": "synthetic_control", "return_type": "-> Result<(), Error>",
                             "span": {"start": {"line": 1, "column": 1}, "end": {"line": 3, "column": 2}}},
                "span": {"start": {"line": 2, "column": 1}, "end": {"line": 2, "column": 2}},
                "replacement": ">=", "genre": "BinaryOperator"}

    def test_eight_worker_disk_floor_rejects_before_session_and_accepts_exact_boundary(self):
        minimum = 5 * 1024 ** 3
        policy = {**MODULE.CONDITIONED_WINDOW_FIXED,
                  "candidate_sha256": "a" * 64,
                  "mutation_inventories": self.bound_inventories()}
        args = SimpleNamespace(subject="conditioned_window", lane="mutation",
                               tool_admission_profile="full", output_dir=Path("/owned/output"))
        for available in (minimum - 1, minimum):
            with self.subTest(available=available), \
                 patch.object(MODULE, "read_conditioned_window_policy", return_value=policy), \
                 patch.object(MODULE.COMMON, "_require_private_output") as private_output, \
                 patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=available)), \
                 patch.object(MODULE.COMMON, "Session", side_effect=RuntimeError("session reached")) as session:
                if available < minimum:
                    with self.assertRaisesRegex(MODULE.Incomplete, "five GiB"):
                        MODULE.execute_conditioned_window(args, {})
                    session.assert_not_called()
                else:
                    with self.assertRaisesRegex(RuntimeError, "session reached"):
                        MODULE.execute_conditioned_window(args, {})
                    session.assert_called_once()
                private_output.assert_called_once_with(MODULE.ROOT, args.output_dir)

    def test_listing_uses_both_exact_owners_and_grants_no_campaign_or_score(self):
        calls = []
        def listing(label, command, timeout):
            calls.append((label, command, timeout))
            filename = command[command.index("--file") + 1]
            return (MODULE.COMMON.mutation_listing_warning(8) or "") + json.dumps([self.mutant(filename)])
        report = {}
        MODULE.enumerate_conditioned_window(SimpleNamespace(tools=Path("/owned/tools"), run=listing),
                                            MODULE.CONDITIONED_WINDOW_FIXED, report)
        self.assertEqual(len(calls), 2)
        self.assertEqual({call[1][call[1].index("--file") + 1] for call in calls},
                         set(MODULE.CONDITIONED_WINDOW_SUBJECTS.values()))
        for _, command, timeout in calls:
            self.assertEqual(command[-2:], ["--list", "--json"])
            self.assertNotIn("--baseline", command)
            self.assertEqual(command[command.index("--jobs") + 1], "8")
            self.assertLessEqual(timeout, 120)
        self.assertEqual(report["listed_total"], 2)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        policy = {**MODULE.CONDITIONED_WINDOW_FIXED, "mutation_inventories": report["listed_inventory_bindings"]}
        self.assertEqual(set(MODULE.conditioned_window_inventory_bindings(policy, required=True)),
                         set(MODULE.CONDITIONED_WINDOW_SUBJECTS))
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.CONDITIONED_WINDOW_SCOPE, policy,
                                                         report["listed_inventories"])
        changed = copy.deepcopy(report["listed_inventories"])
        next(iter(changed["window_variance"].values()))["replacement"] = "<="
        with self.assertRaisesRegex(MODULE.Incomplete, "differs from its reviewed"):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.CONDITIONED_WINDOW_SCOPE, policy, changed)

    def test_all_lane_checks_both_coverage_denominators_before_any_mutation(self):
        for failed_owner in (None, "conditioned_window", "window_variance"):
            with self.subTest(failed_owner=failed_owner), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                policy = self.fixture(root)
                policy["mutation_inventories"] = self.bound_inventories()
                self.write(root, policy)
                _, identity = MODULE.PRODUCT.admit_product(root)
                work = root / "owned-work"
                work.mkdir()
                events = []
                def run(label, command, timeout, **_options):
                    events.append(label)
                    if label == "baseline":
                        self.assertEqual(command[:4], ["cargo", "test", "--package", "time-core"])
                        return native_log(policy["test_inventories"]["baseline"])
                    self.assertEqual(label, "coverage")
                    self.assertIn("--lib", command)
                    self.assertEqual([command[index + 1] for index, value in enumerate(command) if value == "--test"],
                                     ["conditioned_window", "window_variance"])
                    self.assertEqual(timeout, 300)
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename,
                         "summary": {"lines": {"count": 10, "covered": 8 if name == failed_owner else 9}}}
                        for name, filename in MODULE.CONDITIONED_WINDOW_SUBJECTS.items()]}]}))
                    return native_log(policy["test_inventories"]["coverage"])
                def session_factory(actual_root, args, _report, scope, admitted, _extra):
                    self.assertEqual((actual_root, args.lane, scope), (root, "all", MODULE.CONDITIONED_WINDOW_SCOPE))
                    self.assertEqual(admitted["mutation_jobs"], 8)
                    events.append("session")
                    return SimpleNamespace(product_identity=identity, source=root, work=work,
                                           tools=root / "tools", run=run, finish=lambda: events.append("finish"))
                def mutation(scope, subjects, admitted, actual_work, tools, execute, report, lane):
                    self.assertEqual((scope, subjects, lane),
                                     (MODULE.CONDITIONED_WINDOW_SCOPE, MODULE.CONDITIONED_WINDOW_SUBJECTS, "mutation"))
                    self.assertEqual(set(report["coverage"]), set(MODULE.CONDITIONED_WINDOW_SUBJECTS))
                    self.assertEqual(admitted["maximum_mutants"], 512)
                    events.append("mutation")
                args = SimpleNamespace(subject="conditioned_window", lane="all", tool_admission_profile="full",
                                       output_dir=root.parent / (root.name + "-external-analysis"))
                report = {}
                with patch.object(MODULE, "ROOT", root), \
                     patch.object(MODULE.COMMON, "_require_private_output"), \
                     patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=6 * 1024 ** 3)), \
                     patch.object(MODULE.COMMON, "Session", side_effect=session_factory), \
                     patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=mutation) as mutations:
                    if failed_owner is None:
                        MODULE.execute_conditioned_window(args, report)
                        self.assertEqual(events, ["session", "baseline", "coverage", "mutation", "finish"])
                        self.assertTrue(report["qualification_passed"])
                        mutations.assert_called_once()
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, "below 90"):
                            MODULE.execute_conditioned_window(args, report)
                        self.assertEqual(events, ["session", "baseline", "coverage", "finish"])
                        self.assertFalse(report["qualification_passed"])
                        mutations.assert_not_called()


if __name__ == "__main__":
    unittest.main()
