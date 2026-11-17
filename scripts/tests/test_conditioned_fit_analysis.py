"""Focused N2 role controls; synthetic observations execute no product jobs."""
from __future__ import annotations

import copy
import importlib.util
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("conditioned_fit_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
FIT_CASES = frozenset({"exact_fit_control", "conditioned_fit::arithmetic::tests::validator_control"})
FIT_DOCS = (("crates/time-core/src/conditioned_fit.rs", "conditioned_fit::ConditionedFit", False, 1),
            ("crates/time-core/src/conditioned_fit.rs", "conditioned_fit::ConditionedFit", True, 1))


def native_log(names):
    """Complete literal native records, not execution evidence."""
    return ("running " + str(len(names)) + " tests\n"
            + "".join("test " + name + " ... ok\n" for name in names)
            + "test result: ok. " + str(len(names))
            + " passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


class ConditionedFitAnalysisTests(unittest.TestCase):
    """Challenge role isolation, exact inventories, owned logs and gate ordering."""

    def setUp(self):
        """Synthetic minima exercise admission mechanics, never numerical truth."""
        self.addCleanup(patch.stopall)
        patch.object(MODULE, "CONDITIONED_FIT_REQUIRED_TESTS", FIT_CASES).start()
        patch.object(MODULE, "CONDITIONED_FIT_REQUIRED_DOCTESTS", FIT_DOCS).start()

    def policy(self):
        """Fresh synthetic N2 data contains no N1 bindings or equivalences."""
        ordinary = sorted(FIT_CASES | {"other_library_control"})
        docs = ["crates/time-core/src/conditioned_fit.rs - conditioned_fit::ConditionedFit (line 10)",
                "crates/time-core/src/conditioned_fit.rs - conditioned_fit::ConditionedFit (line 20) - compile fail"]
        return {**copy.deepcopy(MODULE.CONDITIONED_FIT_FIXED),
                "source_file_count": 1, "candidate_sha256": "a" * 64,
                "mutation_inventories": {name: {"count": 1, "sha256": "b" * 64}
                                         for name in MODULE.CONDITIONED_FIT_SUBJECTS},
                "mutation_equivalences": [],
                "test_inventories": {"coverage": ordinary, "baseline": sorted([*ordinary, *docs])}}

    def test_scope_and_dispatch_cannot_import_historical_credit(self):
        role = MODULE._conditioned_fit_role()
        self.assertEqual(role.scope.select("all"), {
            "conditioned_fit": "crates/time-core/src/conditioned_fit.rs",
            "conditioned_arithmetic": "crates/time-core/src/conditioned_fit/arithmetic.rs"})
        self.assertEqual(role.coverage_tests, ("conditioned_fit",))
        self.assertNotEqual(role.scope.policy, MODULE.CONDITIONED_WINDOW_SCOPE.policy)
        self.assertEqual(set(MODULE.CONDITIONED_WINDOW_SCOPE.select("all")),
                         {"conditioned_window", "window_variance"})
        self.assertEqual(MODULE.CONDITIONED_WINDOW_FIXED["mutation_jobs"], 8)
        with patch.object(MODULE, "main_conditioned_fit", return_value=0) as selected, \
             patch.object(MODULE, "main_conditioned_window") as historical:
            self.assertEqual(MODULE.main(["--subject", "conditioned_fit", "--lane", "baseline",
                                          "--output-dir", "/owned/output"]), 0)
            selected.assert_called_once()
            historical.assert_not_called()
        with patch.object(MODULE, "execute_conditioned_fit") as selected, \
             patch.object(MODULE, "read_policy") as historical:
            MODULE.execute(SimpleNamespace(subject="conditioned_fit"), {})
            selected.assert_called_once()
            historical.assert_not_called()

    def test_required_controls_are_call_time_and_empty_minima_fail_closed(self):
        policy = self.policy()
        self.assertEqual(MODULE._conditioned_test_inventories(policy, MODULE._conditioned_fit_role()),
                         policy["test_inventories"])
        for name, empty in (("CONDITIONED_FIT_REQUIRED_TESTS", frozenset()),
                            ("CONDITIONED_FIT_REQUIRED_DOCTESTS", ())):
            with patch.object(MODULE, name, empty), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(policy, MODULE._conditioned_fit_role())
        with patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_TESTS", frozenset({"runtime_n1_control"})):
            self.assertIn("runtime_n1_control", MODULE._conditioned_window_role().required_tests)
            self.assertNotIn("runtime_n1_control", MODULE._conditioned_fit_role().required_tests)
        with patch.object(MODULE, "CONDITIONED_WINDOW_REQUIRED_TESTS", frozenset()), \
             self.assertRaises(MODULE.Incomplete):
            MODULE._conditioned_window_role()

    def test_mutant_bindings_require_both_new_owners_and_combined_cap(self):
        healthy = self.policy()
        role = MODULE._conditioned_fit_role()
        self.assertEqual(set(MODULE._conditioned_inventory_bindings(healthy, role, required=True)),
                         set(MODULE.CONDITIONED_FIT_SUBJECTS))
        for bindings in ({}, {"conditioned_fit": {"count": 1, "sha256": "b" * 64}},
                         {name: {"count": 1, "sha256": "b" * 64}
                          for name in MODULE.CONDITIONED_WINDOW_SUBJECTS},
                         {name: {"count": 300, "sha256": "b" * 64}
                          for name in MODULE.CONDITIONED_FIT_SUBJECTS}):
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_inventory_bindings({**healthy, "mutation_inventories": bindings},
                                                       role, required=True)

    def test_actual_test_identities_reject_missing_duplicates_and_failures(self):
        policy, role = self.policy(), MODULE._conditioned_fit_role()
        names = policy["test_inventories"]["coverage"]
        self.assertEqual(MODULE._conditioned_test_execution(native_log(names), policy, role, doctests=False)
                         ["tests_executed"], len(names))
        for raw in (native_log(names[:-1]), native_log([*names, names[0]]),
                    native_log(names).replace(" ... ok", " ... FAILED", 1)):
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_execution(raw, policy, role, doctests=False)

    def test_baselines_require_each_actual_success_and_reject_linked_log(self):
        policy, role = self.policy(), MODULE._conditioned_fit_role()
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            for subject in MODULE.CONDITIONED_FIT_SUBJECTS:
                directory = work / "mutation" / subject / "mutants.out"
                directory.mkdir(parents=True)
                (directory / "baseline.log").write_text(native_log(policy["test_inventories"]["baseline"]))
                (directory / "linked.log").symlink_to(directory / "baseline.log")
                baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"}
                output = directory / "outcomes.json"
                output.write_text(json.dumps({"outcomes": [baseline]}))
                self.assertEqual(MODULE._conditioned_baseline(work, policy, subject, role)["doctests"], 2)
                for rows in ([], [baseline, baseline], [{**baseline, "summary": "Failure"}],
                             [{**baseline, "log_path": "../escape.log"}],
                             [{**baseline, "log_path": "linked.log"}]):
                    output.write_text(json.dumps({"outcomes": rows}))
                    with self.subTest(subject=subject, rows=rows), self.assertRaises(MODULE.Incomplete):
                        MODULE._conditioned_baseline(work, policy, subject, role)

    def test_each_owner_coverage_and_finalization_failure_prevent_pass(self):
        for failure in (None, "conditioned_fit", "conditioned_arithmetic", "finish"):
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                role, policy, report, events = MODULE._conditioned_fit_role(), self.policy(), {}, []
                identity = {"synthetic.rs": "c" * 64}
                policy.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                              inputs_sha256={name: "d" * 64 for name in role.bound_inputs})
                def run(label, command, timeout, **_options):
                    events.append(label)
                    if label == "baseline":
                        return native_log(policy["test_inventories"]["baseline"])
                    self.assertEqual(label, "coverage")
                    self.assertEqual([command[i + 1] for i, value in enumerate(command) if value == "--test"],
                                     ["conditioned_fit"])
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename,
                         "summary": {"lines": {"count": 10, "covered": 8 if owner == failure else 9}}}
                        for owner, filename in role.scope.select("all").items()]}]}))
                    return native_log(policy["test_inventories"]["coverage"])
                def finish():
                    events.append("finish")
                    if failure == "finish":
                        raise MODULE.Incomplete("final custody changed")
                session = SimpleNamespace(product_identity=identity, source=work, work=work,
                                          tools=work / "tools", run=run, finish=finish)
                def mutation(scope, subjects, admitted, actual_work, tools, execute, actual_report, lane):
                    self.assertEqual((scope, subjects, lane), (role.scope, role.scope.select("all"), "mutation"))
                    self.assertEqual(set(actual_report["coverage"]), set(subjects))
                    events.append("mutation")
                args = SimpleNamespace(subject="conditioned_fit", lane="all", tool_admission_profile="full",
                                       output_dir=work / "output")
                with patch.object(MODULE, "read_conditioned_fit_policy", return_value=policy), \
                     patch.object(MODULE, "inventory_document", return_value=policy), \
                     patch.object(MODULE.PRODUCT, "sha256", return_value="d" * 64), \
                     patch.object(MODULE.COMMON, "_require_private_output"), \
                     patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=6 * 1024 ** 3)), \
                     patch.object(MODULE.COMMON, "Session", return_value=session), \
                     patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=mutation) as mutations:
                    if failure is None:
                        MODULE.execute_conditioned_fit(args, report)
                        self.assertTrue(report["qualification_passed"])
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute_conditioned_fit(args, report)
                        self.assertFalse(report["qualification_passed"])
                    self.assertEqual(events[-1], "finish")
                    self.assertEqual(mutations.call_count, int(failure in (None, "finish")))


class CoreH02PreparationTests(unittest.TestCase):
    """Independent synthetic reader controls; no product or compiler execution."""

    def policy(self):
        """Declare nonempty fixture identities independently of checkout growth."""
        ordinary = sorted({
            "principal::private_decision_guards::principal_equality_keeps_checked_bits_and_independently_seeded_sources",
            "principal_seed_and_genuine_update_hysteresis",
            "lifecycle_raw_parameter_and_ordinal_admission_neighbors",
            "lifecycle_rejection_rolls_back_and_same_event_recovers",
            "lifecycle_maturity_countdown_relapse_and_event_order",
            "lifecycle_capacity_neighbors_preserve_pending_credit",
            "principal::private_decision_guards::principal_raw_reselect_garde_rejects_zero",
            "principal::private_decision_guards::principal_raw_stratum_weight_garde_rejects_negative",
            "principal::private_decision_guards::principal_raw_roster_garde_rejects_empty",
            "principal::private_decision_guards::principal_raw_receiver_garde_rejects_foreign_frame",
            "healthy_independent_core_consumer"})
        docs = ["crates/time-core/src/lib.rs - fixture_only (line 10)"]
        product = {"synthetic/declared.rs": "a" * 64}
        digest = __import__("hashlib").sha256(json.dumps(product, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        return {**copy.deepcopy(MODULE.COMMON.core_h02_role_facts()),
                "source_file_count": len(product), "candidate_sha256": digest,
                "inputs_sha256": {name: "b" * 64 for name in MODULE.CORE_H02_BOUND_INPUTS},
                "mutation_inventories": {}, "test_inventories": {
                    "baseline": sorted([*ordinary, *docs]), "coverage": ordinary},
                "scope": "Synthetic H02 reader fixture; no current product evidence."}, product

    def campaign_fixture(self, root):
        """Fixed synthetic38-Core/graph evidence; no checkout or compiler execution."""
        import hashlib
        digest = lambda value: hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        def save(path, value):
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(value, sort_keys=True) + "\n")
            return {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
        for name in MODULE.CORE_H02_BOUND_INPUTS:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("// Independent synthetic input.\n")
        core_names = (
            "crates/time-core/src/lib.rs", "crates/time-core/src/principal.rs",
            "crates/time-core/src/combination/lifecycle.rs", "crates/time-core/src/reference_input.rs",
            "crates/time-core/src/reference_input/rate_delivery.rs", "crates/time-core/src/regression.rs",
            "crates/time-core/src/regression/chart.rs", "crates/time-core/src/regression/chart/roundoff.rs",
            "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs",
            "crates/time-core/src/regression/chart/roundoff/phase_plan.rs",
        ) + tuple("crates/time-core/src/synthetic_" + str(index) + ".rs" for index in range(28))
        graph = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml",
                 "crates/time-core/Cargo.toml", "crates/time-domain/Cargo.toml",
                 "policy/ui-m0-dependencies.json", "policy/tool-manifest.json", "policy/rust-workspace.toml",
                 "crates/time-domain/src/clock.rs")
        for name in (*core_names, *graph):
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("pub fn fixture() -> bool { true }\n")
        (root / "crates/time-core/src/lib.rs").write_text("\n" * 9
            + "/// ```\n/// let _ = 1_u8;\n/// ```\npub fn fixture_only() {}\n")
        owners = {"principal": "crates/time-core/src/principal.rs",
                  "combination_lifecycle": "crates/time-core/src/combination/lifecycle.rs"}
        manifest = root / "policy/verification-manifest.toml"
        manifest.write_text("critical_files = " + json.dumps(list(owners.values()))
            + "\n[[security_modules]]\npackage = \"time-core\"\nfiles = "
            + json.dumps(list(owners.values())) + "\n")
        product = {name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in (*core_names, *graph)}
        policy, _ = self.policy()
        policy.update(source_file_count=len(product), candidate_sha256=digest(product), campaigns_admitted=True,
            inputs_sha256={name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                          for name in MODULE.CORE_H02_BOUND_INPUTS})
        expected = {**product, **policy["inputs_sha256"]}
        stable = [{"name": "stable", "sha256": "d" * 64, "entries": 1, "root": "stable"}]
        evidence = root / "evidence"
        controls = evidence / "contract-controls-v1"
        rows = []
        kinds = ("FixturePrincipalParameters", "FixturePrincipalSourceState", "FixturePrincipalState",
                 "FixtureCombinationParameters", "FixtureCombinationState")
        for name in ("h02-core-legal", *("h02_" + kind + suffix for kind in kinds
                    for suffix in ("-field-added", "-reviewed-repair"))):
            code = ("E0369" if "FixtureCombinationState" in name else "E0027") if name.endswith("-field-added") else None
            errors = ([{"reason": "compiler-message", "message": {"level": "error", "code": {"code": code},
                       "message": ("__contract_review_field" if code == "E0027" else "ContractReviewNoPartialEq")}}]
                       * (2 if code == "E0027" else 1)) if code else []
            raw = {"exit_code": 101 if code else 0, "failure": None,
                   "command": ["cargo", "check", "--package", "time-core", "--lib"],
                   "output": "\n".join(json.dumps(error) for error in errors)}
            raw_ref = save(controls / (name + ".json"), raw)
            captured = {}
            if name != "h02-core-legal":
                owner = owners["combination_lifecycle" if "Combination" in name else "principal"]
                specimen = controls / (name + "--" + owner.replace("/", "_"))
                specimen.write_text("// Synthetic added/repaired specimen.\n")
                captured[owner] = hashlib.sha256(specimen.read_bytes()).hexdigest()
            rows.append({"name": name, "package": "time-core", "target": "x86_64-unknown-linux-gnu",
                "cfg_test": False, "status": "passed", "generated_mutation_credit": 0,
                "expected_code": code, "actual_codes": [code] * len(errors),
                "raw_sha256": raw_ref["sha256"], "captured_sources_sha256": captured})
        compiler = {"status": "workload-passed", "custody_unchanged": True, "source_sha256": product,
                    "tool_inventories": stable, "reused_h02_controls": rows, "candidate_sha256": "c" * 64}
        refs = {"compiler_controls": save(evidence / "contract-controls-v2/verification.json", compiler)}
        catalogs = {}
        for owner, filename in owners.items():
            point = {"start": {"line": 1, "column": 27}, "end": {"line": 1, "column": 31}}
            item = {"name": filename + ":1:27: replace fixture -> bool with false", "package": "time-core",
                    "file": filename, "function": {"function_name": "fixture", "return_type": "bool", "span": point},
                    "span": point, "replacement": "false", "genre": "FnValue", "diff": "independent rendered fixture"}
            catalogs[owner] = {item["name"]: item}
        policy["mutation_inventories"] = {owner: {"count": 1,
            "sha256": digest({key: {field: value for field, value in item.items() if field != "diff"}
                              for key, item in inventory.items()})} for owner, inventory in catalogs.items()}
        reports, paths = {}, {}
        for lane in ("baseline", "coverage", "enumeration"):
            work = evidence / lane / "work"
            snapshot = work / "source"
            for name in expected:
                path = snapshot / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes((root / name).read_bytes())
            closed = {**policy, "campaigns_admitted": False}
            policy_path = snapshot / MODULE.COMMON.CORE_H02_POLICY
            save(policy_path, closed)
            source = {**expected, MODULE.COMMON.CORE_H02_POLICY: hashlib.sha256(policy_path.read_bytes()).hexdigest()}
            report = {"status": "enumerated" if lane == "enumeration" else "passed",
                "selected_lane": "enumerate" if lane == "enumeration" else lane, "selected_lane_passed": True,
                "candidate_sha256": policy["candidate_sha256"], "subjects": owners, "tool_admission_profile": "stable-core",
                "qualification_passed": False, "source_sha256": source, "work": str(work),
                "analysis_admission": {"profile": "stable-core", "inventories": stable},
                "analysis_admission_after": {"profile": "stable-core", "inventories": stable, "unchanged": True}}
            if lane in ("baseline", "coverage"):
                names = policy["test_inventories"]["baseline" if lane == "baseline" else "coverage"]
                report["lanes"] = [{"lane": lane, "exit_code": 0, "failure": None, "output": native_log(names)}]
                argv = (["cargo", "test", "--package", "time-core", "--frozen", "--jobs", "6",
                         "--", "--test-threads=1"] if lane == "baseline" else
                    [str(work.parent / "tools/cargo-llvm-cov"), "llvm-cov", "--package", "time-core",
                     "--frozen", "--jobs", "6", "--lib", "--tests", "--json", "--output-path", "/work/coverage.json",
                     "--failure-mode", "any", "--ignore-filename-regex", "(^|/)tests/|/rustlib/src/rust/library/",
                     "--", "--test-threads=1"])
                report["lanes"][0]["command"] = ["/usr/bin/bwrap", "--", *argv]
                report[lane + "_execution"] = {"tests_executed": len(names), "test_names": names,
                                               "inventory_sha256": digest(names)}
                if lane == "coverage":
                    data = {"data": [{"files": [{"filename": "/work/source/" + filename,
                        "summary": {"lines": {"count": 1, "covered": 1}}} for filename in owners.values()]}]}
                    save(work / "coverage.json", data)
                    report["coverage"] = {owner: {"file": filename, "executable_lines": 1, "covered_lines": 1,
                        "line_percent": 100.0, "branch_evidence": None} for owner, filename in owners.items()}
            else:
                report.update(listed_inventory_bindings=policy["mutation_inventories"], listed_inventories=catalogs,
                              campaign_admission=False, mutation_enumeration={}, lanes=[])
                for owner, filename in owners.items():
                    output = json.dumps(list(catalogs[owner].values()))
                    argv = [str(work.parent / "tools/cargo-mutants"), "mutants", "--package", "time-core",
                        "--file", filename, "--test-workspace", "false", "--cap-lints", "false", "--jobs", "6",
                        "--jobserver", "true", "--jobserver-tasks", "0", "--cargo-arg=--message-format=json", "--list", "--json"]
                    report["lanes"].append({"lane": "mutation-enumeration:" + owner, "exit_code": 0,
                        "failure": None, "output": output, "command": ["/usr/bin/bwrap", "--", *argv]})
                    report["mutation_enumeration"][owner] = {"raw_output": output, "warning": None}
            paths[lane] = evidence / lane / "verification.json"
            reports[lane] = report
            refs[lane] = save(paths[lane], report)
        policy["campaign_admission"] = refs
        save(root / MODULE.COMMON.CORE_H02_POLICY, policy)
        return policy, product, paths, reports

    def test_h02_dispatch_adds_one_role_without_replacing_the_old_role(self):
        with patch.object(MODULE, "main_core_h02", return_value=0) as current, \
             patch.object(MODULE, "main_conditioned_fit") as previous:
            self.assertEqual(MODULE.main(["--subject", "principal_combination", "--lane", "inventory",
                "--tool-admission-profile", "stable-core", "--output-dir", "/owned/output"]), 0)
            current.assert_called_once()
            previous.assert_not_called()
        self.assertEqual(MODULE._conditioned_fit_role().coverage_tests, ("conditioned_fit",))
        self.assertEqual(set(MODULE._core_h02_role().scope.select("all")), {"principal", "combination_lifecycle"})

    def test_h02_closed_role_refuses_policy_relaxation_and_direct_campaigns(self):
        policy, _ = self.policy()
        common, scope = MODULE.COMMON, MODULE.CORE_H02_SCOPE
        common.require_core_h02_role(scope, policy)
        self.assertEqual(common.admission_profile(SimpleNamespace(tool_admission_profile="stable-core", lane="contracts"), scope, policy), "stable-core")
        for field, value in [("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                             ("campaigns_admitted", True), ("contracts_admitted", True),
                             ("mutation_jobs", True), ("mutation_debug_info", False),
                             ("mutation_debug_info", 0.0), ("mutation_debug_info", 1),
                             ("mutation_equivalences", [{"id": "copied-historical"}])]:
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                common.require_core_h02_role(scope, {**policy, field: value})
        with self.assertRaises(MODULE.Incomplete):
            common.mutation_campaign_command(scope, Path("/tools"), "principal", [], expected_jobs=6,
                expected_message_format="json", policy=policy)
        with self.assertRaises(MODULE.Incomplete):
            common.require_core_h02_role(scope._replace(policy="policy/time-core-analysis.json"), policy)
        refs = {name: {"path": "/synthetic/" + name + ".json", "sha256": "d" * 64}
                for name in ("compiler_controls", "baseline", "coverage", "enumeration")}
        refs["compiler_controls"]["sha256"] = MODULE.COMMON.CORE_H02_STRUCTURAL_CONTROL_SHA256
        private = {**policy, "campaigns_admitted": True, "campaign_admission": refs,
                   "mutation_inventories": {name: {"count": 1, "sha256": "e" * 64}
                                            for name in scope.select("all")}}
        common.require_core_h02_role(scope, private)
        command = ["/tools/cargo-mutants", "mutants", "--package", "time-core", "--file",
                   dict(scope.subjects)["principal"], "--test-workspace", "false", "--cap-lints", "false",
                   "--jobs", "6", "--jobserver", "true", "--jobserver-tasks", "0",
                   "--cargo-arg=--message-format=json", "--baseline", "run", "--build-timeout", "90",
                   "--timeout", "60", "--output", "/work/mutation/principal"]
        with patch.object(common, "_CORE_H02_CAMPAIGN_ADMISSION") as proof:
            common.mutation_campaign_command(scope, Path("/tools"), "principal", command,
                expected_jobs=6, expected_message_format="json", policy=private)
            proof.assert_called_once_with(common.ROOT, private)
        with patch.object(common, "_CORE_H02_CAMPAIGN_ADMISSION", side_effect=MODULE.Incomplete("stale proof")), \
             self.assertRaises(MODULE.Incomplete):
            common.mutation_campaign_command(scope, Path("/tools"), "principal", command,
                expected_jobs=6, expected_message_format="json", policy=private)
        with patch.object(common, "_CORE_H02_CAMPAIGN_ADMISSION", None), self.assertRaises(MODULE.Incomplete):
            common.mutation_campaign_command(scope, Path("/tools"), "principal", command,
                expected_jobs=6, expected_message_format="json", policy=private)
        for changed in ({**private, "contracts_admitted": True},
                        {**private, "campaign_admission": {}}):
            with self.subTest(private=changed), self.assertRaises(MODULE.Incomplete):
                common.require_core_h02_role(scope, changed)

    def test_h02_requires_all_measured_consumers_and_rejects_owner_doctest_growth(self):
        policy, _ = self.policy()
        self.assertEqual(MODULE.core_h02_test_inventories(policy), policy["test_inventories"])
        for rows in [
            {"baseline": [], "coverage": []},
            {"baseline": policy["test_inventories"]["baseline"], "coverage": []},
            {"baseline": sorted([*policy["test_inventories"]["baseline"],
                "crates/time-core/src/principal.rs - unreviewed (line 1)"]),
             "coverage": policy["test_inventories"]["coverage"]},
        ]:
            with self.subTest(rows=rows), self.assertRaises(MODULE.Incomplete):
                MODULE.core_h02_test_inventories({**policy, "test_inventories": rows})
        raw_controls = (
            "principal::private_decision_guards::principal_raw_reselect_garde_rejects_zero",
            "principal::private_decision_guards::principal_raw_stratum_weight_garde_rejects_negative",
            "principal::private_decision_guards::principal_raw_roster_garde_rejects_empty",
            "principal::private_decision_guards::principal_raw_receiver_garde_rejects_foreign_frame",
        )
        self.assertTrue(set(raw_controls) <= set(policy["test_inventories"]["coverage"]))
        for omitted in raw_controls:
            rows = {lane: [name for name in names if name != omitted]
                    for lane, names in policy["test_inventories"].items()}
            with self.subTest(omitted=omitted), self.assertRaises(MODULE.Incomplete):
                MODULE.core_h02_test_inventories({**policy, "test_inventories": rows})
        pending = {**policy, "test_inventories": {"baseline": [], "coverage": []}}
        self.assertEqual(MODULE.core_h02_test_inventories(pending, allow_pending=True), pending["test_inventories"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.core_h02_test_inventories(pending, allow_pending=1)

    def test_h02_execution_refuses_zero_or_substituted_selection_and_error_output(self):
        policy, _ = self.policy()
        healthy = native_log(policy["test_inventories"]["baseline"])
        result = MODULE.core_h02_test_execution(healthy, policy, doctests=True)
        self.assertEqual(result["tests_executed"], len(policy["test_inventories"]["baseline"]))
        for text in [native_log([]), native_log(["wrong_healthy_neighbor"]), "Error: refused\n" + healthy]:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.core_h02_test_execution(text, policy, doctests=True)

    def test_h02_real_reader_binds_declared_source_and_every_protected_input(self):
        policy, product = self.policy()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            # Explicit synthetic bytes support reader admission only; none is compiled.
            for name in MODULE.CORE_H02_BOUND_INPUTS:
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("// Synthetic reader fixture.\n")
            manifest = root / "policy/verification-manifest.toml"
            manifest.parent.mkdir(parents=True, exist_ok=True)
            owners = list(MODULE.CORE_H02_SCOPE.select("all").values())
            manifest.write_text("critical_files = " + json.dumps(owners) + "\n[[security_modules]]\npackage = \"time-core\"\nfiles = " + json.dumps(owners) + "\n")
            policy["inputs_sha256"] = {name: __import__("hashlib").sha256((root / name).read_bytes()).hexdigest()
                                      for name in MODULE.CORE_H02_BOUND_INPUTS}
            path = root / MODULE.COMMON.CORE_H02_POLICY
            path.write_text(json.dumps(policy))
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, product)):
                self.assertEqual(MODULE.read_core_h02_policy(root), policy)
                refs = {name: {"path": "/synthetic/" + name + ".json", "sha256": "d" * 64}
                        for name in ("compiler_controls", "baseline", "coverage", "enumeration")}
                refs["compiler_controls"]["sha256"] = MODULE.COMMON.CORE_H02_STRUCTURAL_CONTROL_SHA256
                private = {**policy, "campaigns_admitted": True, "campaign_admission": refs,
                    "mutation_inventories": {name: {"count": 1, "sha256": "e" * 64}
                                             for name in MODULE.CORE_H02_SCOPE.select("all")}}
                path.write_text(json.dumps(private))
                with self.assertRaises((MODULE.Incomplete, OSError)):
                    MODULE.read_core_h02_policy(root)
                path.write_text(json.dumps(policy))
                for field, value in [("source_file_count", True), ("candidate_sha256", "c" * 64)]:
                    path.write_text(json.dumps({**policy, field: value}))
                    with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                        MODULE.read_core_h02_policy(root)
                # Exercise the real empty-roster inventory route and readers;
                # only process execution and full-product admission are synthetic.
                ordinary = policy["test_inventories"]["coverage"]
                lib = "crates/time-core/src/lib.rs"
                (root / lib).write_text("/// ```\n/// let _ = 1_u8;\n/// ```\n")
                product.clear()
                product[lib] = __import__("hashlib").sha256((root / lib).read_bytes()).hexdigest()
                policy["candidate_sha256"] = __import__("hashlib").sha256(
                    json.dumps(product, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
                policy["inputs_sha256"][lib] = product[lib]
                policy["test_inventories"] = {"baseline": [], "coverage": []}
                path.write_text(json.dumps(policy))
                doc = lib + " - (line 1)"
                listings = {
                    "inventory-native": "".join(name + ": test\n" for name in ordinary)
                        + str(len(ordinary)) + " tests, 0 benchmarks\n",
                    "inventory-doctests": doc + ": test\n1 test, 0 benchmarks\n",
                }
                calls, profiles, finished = [], [], []
                case = self

                class InventorySession:
                    def __init__(session, source, selected, report, scope, admitted, extra):
                        case.assertEqual(source, root)
                        profiles.append(MODULE.COMMON.admission_profile(selected, scope, admitted))
                        session.source, session.product_identity = root, product

                    def run(session, label, command, timeout):
                        case.assertIn(label, listings)
                        case.assertEqual(command[:7], ["cargo", "test", "--package", "time-core",
                                                      "--frozen", "--jobs", "6"])
                        case.assertEqual(command[-2:], ["--", "--list"])
                        calls.append(label)
                        return listings[label]

                    def finish(session):
                        finished.append(True)

                args = SimpleNamespace(subject=MODULE.CORE_H02_SUBJECT, lane="inventory",
                    tool_admission_profile="stable-core", output_dir=root / "owned-output")
                report = {}
                with patch.object(MODULE, "ROOT", root), patch.object(MODULE.COMMON, "Session", InventorySession):
                    MODULE.execute_core_h02(args, report)
                    for lane in ("baseline", "enumerate", "coverage"):
                        args.lane = lane
                        with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                            MODULE.execute_core_h02(args, {})
                observed = {"baseline": sorted([*ordinary, doc]), "coverage": ordinary}
                self.assertEqual(report["test_inventory_discovery"]["inventories"], observed)
                self.assertEqual(report["test_inventory_discovery"]["native_tests_executed"], 0)
                self.assertEqual(calls, ["inventory-native", "inventory-doctests"])
                self.assertEqual(profiles, ["stable-core"])
                self.assertEqual(finished, [True])
                self.assertTrue(report["selected_lane_passed"])
                self.assertFalse(report["qualification_passed"])
                policy["test_inventories"] = observed
                path.write_text(json.dumps(policy))
                target = root / owners[0]
                target.write_text("// Altered but individually well-formed source.\n")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_core_h02_policy(root)

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy, product, paths, reports = self.campaign_fixture(root)
            compiler_sha = policy["campaign_admission"]["compiler_controls"]["sha256"]
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, product)), \
                 patch.object(MODULE.COMMON, "CORE_H02_STRUCTURAL_CONTROL_SHA256", compiler_sha):
                self.assertEqual(MODULE.read_core_h02_policy(root), policy)
                bad_cases = []
                malformed = copy.deepcopy(reports["baseline"])
                malformed["source_sha256"] = []
                bad_cases.append(("baseline", malformed))
                for names in ([], ["default_roster_substitute"]):
                    value = copy.deepcopy(reports["baseline"])
                    value["lanes"][0]["output"] = native_log(names)
                    bad_cases.append(("baseline", value))
                value = copy.deepcopy(reports["enumeration"])
                value["lanes"][0]["command"] += ["--features", "foreign-feature"]
                bad_cases.append(("enumeration", value))
                value = copy.deepcopy(reports["enumeration"])
                value["lanes"][0]["output"] = "[]"
                value["mutation_enumeration"]["principal"]["raw_output"] = "[]"
                bad_cases.append(("enumeration", value))
                value = copy.deepcopy(reports["enumeration"])
                items = json.loads(value["lanes"][0]["output"])
                items[0]["replacement"] = "true"
                value["lanes"][0]["output"] = json.dumps(items)
                value["mutation_enumeration"]["principal"]["raw_output"] = value["lanes"][0]["output"]
                bad_cases.append(("enumeration", value))
                value = copy.deepcopy(reports["enumeration"])
                value["lanes"][0]["lane"] = "mutation-enumeration:foreign-owner"
                bad_cases.append(("enumeration", value))
                value = copy.deepcopy(reports["coverage"])
                old_work = Path(value["work"])
                bad_work = old_work.parent / "below-line-work"
                __import__("shutil").copytree(old_work / "source", bad_work / "source")
                data = json.loads((old_work / "coverage.json").read_text())
                data["data"][0]["files"][0]["summary"]["lines"]["covered"] = 0
                (bad_work / "coverage.json").write_text(json.dumps(data))
                value["work"] = str(bad_work)
                value["coverage"]["principal"].update(covered_lines=0, line_percent=0.0)
                bad_cases.append(("coverage", value))
                for index, (lane, value) in enumerate(bad_cases):
                    path = root / ("bad-proof-" + str(index) + ".json")
                    path.write_text(json.dumps(value))
                    changed = copy.deepcopy(policy)
                    changed["campaign_admission"][lane] = {"path": str(path), "sha256":
                        __import__("hashlib").sha256(path.read_bytes()).hexdigest()}
                    (root / MODULE.COMMON.CORE_H02_POLICY).write_text(json.dumps(changed))
                    with self.subTest(lane=lane, case=index), self.assertRaises(MODULE.Incomplete):
                        MODULE.read_core_h02_policy(root)
                (root / MODULE.COMMON.CORE_H02_POLICY).write_text(json.dumps(policy))
                for field, value in (("source_file_count", len(product) + 1), ("candidate_sha256", "f" * 64)):
                    with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_core_h02_campaign(root, {**policy, field: value})
                self.assertEqual(MODULE.read_core_h02_policy(root), policy)

    def test_h02_no_pending_compiler_or_mutation_lane_can_publish_success(self):
        for lane in ("contracts", "cohort-union", "all"):
            args = SimpleNamespace(subject=MODULE.CORE_H02_SUBJECT, lane=lane,
                tool_admission_profile="stable-core", output_dir=Path("/owned/output"))
            with self.subTest(lane=lane), patch.object(MODULE.COMMON, "Session") as session, self.assertRaises(MODULE.Incomplete):
                MODULE.execute_core_h02(args, {})
            session.assert_not_called()
        policy, _ = self.policy()
        args = SimpleNamespace(subject=MODULE.CORE_H02_SUBJECT, lane="mutation",
            tool_admission_profile="stable-core", output_dir=Path("/owned/output"))
        with patch.object(MODULE, "read_core_h02_policy", return_value=policy), \
             patch.object(MODULE.COMMON, "Session") as session, self.assertRaises(MODULE.Incomplete):
            MODULE.execute_core_h02(args, {})
        session.assert_not_called()

        # Reach the real reader/configuration, stopping before the synthetic process boundary.
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy, product, _, _ = self.campaign_fixture(root)
            compiler_sha = policy["campaign_admission"]["compiler_controls"]["sha256"]
            protected = {name: MODULE.PRODUCT.sha256(root / name) for name in
                         {*product, *MODULE.CORE_H02_BOUND_INPUTS, MODULE.COMMON.CORE_H02_POLICY}}
            before = ('[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory='
                      + json.dumps(str(root / "vendor")) + '\n')
            after = before + "\n[profile.dev]\ndebug = 0\n[profile.test]\ndebug = 0\n"
            original_configure = MODULE.COMMON.Session.configure
            sessions, finished = [], []

            class DebugSession:
                configure = original_configure

                def __init__(session, source, selected, report, scope, admitted, extra):
                    session.source, session.product_identity = root, product
                    session.work, session.tools, session.vendor = root / "debug-work", root / "tools", root / "vendor"
                    (session.work / "cargo-home").mkdir(parents=True)
                    session.configure(before)
                    sessions.append(session)

                def finish(session):
                    finished.append(True)

            def before_campaign(scope, subjects, admitted, work, tools, run, report, lane):
                session = sessions[0]
                self.assertEqual(session.config.read_text(), after)
                self.assertEqual(session.config_sha256, MODULE.PRODUCT.sha256(session.config))
                observed = report["mutation_debug_configuration"]
                self.assertEqual((observed["before_text"], observed["after_text"]), (before, after))
                digest = __import__("hashlib").sha256
                self.assertEqual(observed["before_sha256"], digest(before.encode()).hexdigest())
                self.assertEqual(observed["after_sha256"], digest(after.encode()).hexdigest())
                self.assertEqual(observed["profile_settings"], {"dev": {"debug": 0}, "test": {"debug": 0}})
                self.assertFalse(observed["other_profile_settings_changed"])
                raise MODULE.Incomplete("synthetic before-campaign stop")

            args.output_dir = root / "owned-output"
            report = {}
            with patch.object(MODULE, "ROOT", root), \
                 patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, product)), \
                 patch.object(MODULE.COMMON, "CORE_H02_STRUCTURAL_CONTROL_SHA256", compiler_sha), \
                 patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=8 * 1024 ** 3)), \
                 patch.object(MODULE.COMMON, "Session", DebugSession), \
                 patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=before_campaign) as campaign, \
                 self.assertRaisesRegex(MODULE.Incomplete, "synthetic before-campaign stop"):
                MODULE.execute_core_h02(args, report)
            campaign.assert_called_once()
            self.assertEqual(finished, [True])
            session, role = sessions[0], MODULE._core_h02_role()
            for value in (False, True, 0.0, 1, -1, "0"):
                session.configure(before)
                with self.subTest(debug_info=value), self.assertRaises(MODULE.Incomplete):
                    MODULE.configure_phase_plan_mutation(session, {**policy, "mutation_debug_info": value}, {}, role)
                self.assertEqual(session.config.read_text(), before)
            for scope in (role.scope._replace(policy="policy/foreign.json"),
                          role.scope._replace(subjects=role.scope.subjects[:-1]), tuple(role.scope)):
                with self.subTest(scope=scope), self.assertRaises(MODULE.Incomplete):
                    MODULE.configure_phase_plan_mutation(session, policy, {}, role._replace(scope=scope))
                self.assertEqual(session.config.read_text(), before)
            # Even a hash-rebound extra profile setting cannot become the vendor-only preimage.
            session.configure(before + "\n[profile.test]\noverflow-checks = false\n")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.configure_phase_plan_mutation(session, policy, {}, role)
            self.assertEqual({name: MODULE.PRODUCT.sha256(root / name) for name in protected}, protected)



if __name__ == "__main__":
    unittest.main()
