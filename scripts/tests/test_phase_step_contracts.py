"""Causal registration controls for Step consumers; these do not execute Rust."""
from __future__ import annotations

import ast
import copy
import hashlib
import importlib.util
from pathlib import Path
import re
import tempfile
import time
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "phase_step_contract_registry", ROOT / "scripts/core-frequency-contracts.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def phase_examples():
    """Load only the code-owned pure legacy factory, without invoking a runner."""
    path = ROOT / "scripts/verify-core-analysis.py"
    node = next(item for item in ast.parse(path.read_text()).body
                if isinstance(item, ast.FunctionDef)
                and item.name == "phase_plan_contract_examples")
    namespace = {"Incomplete": ValueError}
    exec(compile(ast.Module(body=[node], type_ignores=[]), str(path), "exec"), namespace)
    return namespace["phase_plan_contract_examples"]()


class PhaseStepContractControls(unittest.TestCase):
    """Reject causal/owner/count drift before the separately admitted compiler lane."""

    def setUp(self):
        self.session, self.report = object(), {}
        self.policy = {
            "critical_files": list(MODULE.PHASE_STEP_CRITICAL_FILES),
            "tool_admission_profile": "stable-core", "contracts_admitted": True,
            "inputs_sha256": {name: "0" * 64 for name in
                              (*MODULE.PHASE_STEP_INPUTS, *MODULE.PHASE_STEP_CRITICAL_FILES)},
            "external_consumer_limits": dict(MODULE.PHASE_STEP_LIMITS),
        }
        self.execute = Mock(return_value="delegated-step")

    def invoke(self, factory=phase_examples):
        return MODULE.phase_step_contracts(
            self.session, self.policy, self.report, self.execute, factory)

    def test_exact_seven_complete_owners_and_new_helper_inputs_are_declared(self):
        self.assertEqual(MODULE.PHASE_STEP_CRITICAL_FILES, (
            "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs",
            "crates/time-core/src/regression/chart/roundoff/phase_plan.rs",
            "crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs",
            "crates/time-core/src/reference_input/rate_delivery.rs",
            "crates/time-core/src/reference_input.rs",
            "crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs",
            "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs",
        ))
        self.assertEqual(MODULE.PHASE_STEP_INPUTS, (
            "scripts/core-frequency-contracts.py", "scripts/tests/test_phase_step_contracts.py"))
        self.assertEqual(MODULE.INPUTS, (
            "scripts/core-frequency-contracts.py", "scripts/tests/test_core_frequency_contracts.py"))

    def test_closed_registry_has_one_legal_and_nine_attributable_negative_cases(self):
        rows = MODULE.phase_step_examples()
        self.assertIs(type(rows), tuple)
        self.assertEqual(len(rows), 10)
        cases = dict(rows)
        self.assertEqual(len(cases), 10)
        self.assertEqual(MODULE.PHASE_STEP_NEW_LIMITS,
                         {"legal_executions": 1, "negative_compilations": 9})
        self.assertEqual(MODULE.PHASE_STEP_LIMITS,
                         {"legal_executions": 3, "negative_compilations": 32})
        self.assertEqual(sum(row["code"] is None for row in cases.values()), 1)
        self.assertEqual(set(cases) - {MODULE.PHASE_STEP_LEGAL_NAME}, set(MODULE.PHASE_STEP_CODES))
        expected = {
            "phase_step_adaptive_literal": ("E0451", "FixtureAdaptiveHorizonProfile {"),
            "phase_step_adaptive_mutation": ("E0616", "profile.correction_time_ratio ="),
            "phase_step_limits_literal": ("E0451", "FixtureStepLimits {"),
            "phase_step_limits_mutation": ("E0616", "limits.maximum ="),
            "phase_step_completion_literal": ("E0451", "FixtureStepCompletion {"),
            "phase_step_completion_mutation": ("E0616", "completion.achieved ="),
            "phase_step_live_proposal_settlement": ("E0505", "owner.settle_step(event,now)"),
            "phase_step_double_transfer": ("E0499", "let second=retired.resume_fresh(now)"),
            "phase_step_retirement_escape": ("E0515", "retired.resume_fresh(now)"),
        }
        self.assertEqual(set(expected), set(MODULE.PHASE_STEP_CODES))
        for name, (code, attempt) in expected.items():
            with self.subTest(name=name):
                row = cases[name]
                self.assertEqual(row["code"], code)
                primary = row["source"].splitlines()[row["line"] - 1]
                self.assertIn(attempt, primary)
                self.assertEqual(row["source"].splitlines().count(primary), 1)
                self.assertGreaterEqual(len(row["tokens"]), 2)
                self.assertNotIn("rendered", row)

    def test_every_restriction_has_a_real_public_api_legal_neighbor(self):
        cases = dict(MODULE.phase_step_examples())
        legal = cases[MODULE.PHASE_STEP_LEGAL_NAME]["source"]
        functions = set(re.findall(r"\bfn ([a-z_]+)(?:<|\()", legal))
        for name, row in cases.items():
            if row["code"] is None:
                continue
            with self.subTest(name=name):
                neighbor = row["external_legal_neighbor"]
                self.assertEqual(neighbor["specimen"], MODULE.PHASE_STEP_LEGAL_NAME)
                self.assertIn(neighbor["function"], functions)
                self.assertIn("compiled typed neighbor", neighbor["scope"])
        self.assertIn("retired.resume_fresh(now)", legal)
        self.assertIn("owner.settle_step(event,now)", legal)
        self.assertIn("proposal.requested_phase_error_seconds()", legal)

    def test_legal_binary_observes_admission_and_preserves_actual_no_reference_custody(self):
        legal = dict(MODULE.phase_step_examples())[MODULE.PHASE_STEP_LEGAL_NAME]["source"]
        for actual in ["legal_adaptive_profile(minimum,maximum)?", "let limits=legal_step_limits()?",
                       "legal_step_completion(delivery,final_frames)?",
                       ".with_adaptive_horizon(adaptive)?", "FixtureStepError::InvalidLimits",
                       "FixturePhaseError::InvalidProfile", "FixturePhaseError::NoReference",
                       "assert!(owner.step_proposal().is_none())",
                       "assert_eq!(owner.cursor().next_ordinal(),Some(nz(1)?))",
                       "assert_eq!(producer.emitted_rounds(),0)",
                       'println!("legal-phase-step-contract")']:
            with self.subTest(actual=actual):
                self.assertIn(actual, legal)
        self.assertIn("for actual in [0.0,-0.0]", legal)
        self.assertIn("known.achieved_step_seconds().to_bits(),actual.to_bits()", legal)
        self.assertIn("FixtureStepError::Receipt", legal)
        self.assertNotRegex(legal, r"(?:ActiveAwake|Monotonic)DurationNs\s*::\s*new")
        self.assertNotIn("completion.delivery().ordinal()", legal)
        self.assertIn("helpers compile but are not called", MODULE.PHASE_STEP_LEGAL_SCOPE)
        self.assertIn("no native clock operation", MODULE.PHASE_STEP_LEGAL_SCOPE)

    def test_lifetime_controls_keep_the_first_borrow_live_after_the_rejected_attempt(self):
        cases = dict(MODULE.phase_step_examples())
        proposal = cases["phase_step_live_proposal_settlement"]["source"]
        self.assertGreater(proposal.index("proposal.ordinal()"),
                           proposal.index("owner.settle_step(event,now)"))
        transfer = cases["phase_step_double_transfer"]["source"]
        self.assertGreater(transfer.index("first.cursor()"),
                           transfer.index("let second=retired.resume_fresh(now)"))
        escape = cases["phase_step_retirement_escape"]["source"]
        self.assertIn("fn escape<'producer:'chart,'chart>", escape)
        self.assertIn("mut retired:FixtureStepRetirement<'producer>", escape)
        self.assertIn("FixturePhasePlanOwner<'producer,'chart>", escape)
        self.assertEqual(escape.splitlines()[3], "    retired.resume_fresh(now)")
        self.assertEqual(cases["phase_step_retirement_escape"]["code"], "E0515")
        self.assertEqual(cases["phase_step_retirement_escape"]["line"], 4)
        self.assertEqual(cases["phase_step_retirement_escape"]["external_legal_neighbor"]["function"],
                         "legal_step_resume")
        self.assertNotIn("unsafe", escape)
        self.assertEqual(cases["phase_step_retirement_escape"]["tokens"],
                         ["retired", "owned by the current function"])

    def test_additive_route_preserves_every_baseline_row_and_delegates_once(self):
        baseline_execute = Mock()
        baseline_policy = {"external_consumer_limits": dict(MODULE.BASELINE_LIMITS)}
        MODULE.baseline_feedback_contracts(self.session, baseline_policy, {}, baseline_execute, phase_examples)
        old_rows, old_markers = copy.deepcopy(baseline_execute.call_args.args[3:5])
        old_policy = copy.deepcopy(self.policy)
        old_phase = phase_examples()
        before_phase = copy.deepcopy(old_phase)
        self.assertEqual(self.invoke(lambda: old_phase), "delegated-step")
        self.execute.assert_called_once()
        session, policy, report, cases, markers, scope = self.execute.call_args.args
        self.assertIs(session, self.session)
        self.assertIs(policy, self.policy)
        self.assertIs(report, self.report)
        self.assertEqual(self.policy, old_policy)
        self.assertEqual(old_phase, before_phase)
        self.assertEqual({name: cases[name] for name in old_rows}, old_rows)
        self.assertEqual({name: markers[name] for name in old_markers}, old_markers)
        self.assertEqual(len(cases), 35)
        self.assertEqual(sum(row["code"] is None for row in cases.values()), 3)
        self.assertEqual(sum(row["code"] is not None for row in cases.values()), 32)
        self.assertEqual(markers[MODULE.PHASE_STEP_LEGAL_NAME], MODULE.PHASE_STEP_LEGAL_MARKER)
        self.assertEqual(scope, MODULE.PHASE_STEP_LEGAL_SCOPE)

    def test_owner_order_profile_and_closed_admission_are_enforced_before_execution(self):
        for change in [{"critical_files": list(reversed(MODULE.PHASE_STEP_CRITICAL_FILES))},
                       {"critical_files": list(MODULE.PHASE_STEP_CRITICAL_FILES[:-1])},
                       {"critical_files": [*MODULE.PHASE_STEP_CRITICAL_FILES, "unreviewed.rs"]},
                       {"tool_admission_profile": "full"}, {"contracts_admitted": False},
                       {"contracts_admitted": 1}]:
            with self.subTest(change=change):
                healthy = self.policy
                self.policy = {**healthy, **change}
                with self.assertRaisesRegex(ValueError, "admitted seven-file role"):
                    self.invoke()
                self.policy = healthy
        self.execute.assert_not_called()

    def test_every_new_helper_and_complete_owner_requires_a_protected_input_binding(self):
        for name in (*MODULE.PHASE_STEP_INPUTS, *MODULE.PHASE_STEP_CRITICAL_FILES):
            with self.subTest(name=name):
                digest = self.policy["inputs_sha256"].pop(name)
                with self.assertRaisesRegex(ValueError, "protected inputs"):
                    self.invoke()
                self.policy["inputs_sha256"][name] = digest
        self.execute.assert_not_called()

    def test_exact_limits_reject_bool_weakening_omission_and_extra_key(self):
        for limits in [None, {"legal_executions": 2, "negative_compilations": 32},
                       {"legal_executions": 3, "negative_compilations": 31},
                       {"legal_executions": 3, "negative_compilations": True},
                       {"legal_executions": 3}, {**MODULE.PHASE_STEP_LIMITS, "extra": 0}]:
            with self.subTest(limits=limits):
                self.policy["external_consumer_limits"] = limits
                with self.assertRaisesRegex(ValueError, r"fixed actual3\+32"):
                    self.invoke()
        self.execute.assert_not_called()

    def test_missing_extra_recoded_and_legal_to_negative_new_cases_stop_before_execution(self):
        rows = MODULE.phase_step_examples()
        recoded = copy.deepcopy(rows)
        recoded[1][1]["code"] = "E0308"
        legal_recoded = copy.deepcopy(rows)
        legal_recoded[0][1]["code"] = "E0000"
        for changed in [rows[:-1], rows + (("unreviewed", rows[0][1]),), recoded, legal_recoded]:
            with self.subTest(size=len(changed)), patch.object(
                    MODULE, "phase_step_examples", return_value=changed):
                with self.assertRaisesRegex(ValueError, "closed current registry"):
                    self.invoke()
        self.execute.assert_not_called()

    def test_legacy_missing_control_is_still_rejected_in_the_additive_route(self):
        cases = phase_examples()
        del cases["phase_plan_retirement_escape"]
        with self.assertRaisesRegex(ValueError, "unchanged affected phase"):
            self.invoke(lambda: cases)
        self.execute.assert_not_called()

    def test_registry_is_fresh_and_policy_cannot_supply_its_source_truth(self):
        rows = MODULE.phase_step_examples()
        rows[1][1]["tokens"].append("poison")
        rows[1][1]["external_legal_neighbor"]["function"] = "forged"
        clean = MODULE.phase_step_examples()
        self.assertNotIn("poison", clean[1][1]["tokens"])
        self.assertNotEqual(clean[1][1]["external_legal_neighbor"]["function"], "forged")
        self.policy["examples"] = {"forged": {"source": "fn main() {}", "code": None}}
        self.invoke()
        self.assertNotIn("forged", self.execute.call_args.args[3])

    def test_garde_omission_is_an_explicit_unexecuted_control_gap(self):
        scope = MODULE.PHASE_STEP_VALIDATION_SCOPE
        self.assertIn("do not establish mandatory Garde-call execution", scope)
        self.assertIn("Removing only validate", scope)
        self.assertIn("generation/control gap", scope)
        self.assertIn("sentinel refusal", scope)
        self.assertIn("none are claimed here", scope)
        for owner in ("RawAdaptiveProfile", "RawAdaptiveArea", "RawLimits", "RawStepRequest",
                      "RawStepCompletion", "RawStepTerminal", "numeric RawStep"):
            self.assertIn(owner, scope)
        self.assertIn(scope, MODULE.PHASE_STEP_LEGAL_SCOPE)
        self.assertFalse(any("garde" in name.lower() for name, _ in MODULE.phase_step_examples()))

    def test_executor_failure_propagates_without_manufactured_results(self):
        self.execute.side_effect = RuntimeError("expected primary compiler cause is absent")
        with self.assertRaisesRegex(RuntimeError, "expected primary compiler cause"):
            self.invoke()
        self.execute.assert_called_once()
        self.assertEqual(self.report, {})


class PhaseStepValidationControls(unittest.TestCase):
    """Pure source/admission/causal controls; mocked output earns no Rust credit."""

    @staticmethod
    def make_session(root):
        source, work, output = root / "source", root / "work", root / "output"
        for folder in (source, work, output):
            folder.mkdir()
        inputs = (*MODULE.PHASE_STEP_INPUTS, *MODULE.PHASE_STEP_CRITICAL_FILES,
                  MODULE.PHASE_STEP_VALIDATION_FIXTURE)
        identity = {}
        for name in inputs:
            destination = source / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            payload = (ROOT / name).read_bytes()
            destination.write_bytes(payload)
            identity[name] = hashlib.sha256(payload).hexdigest()
        session = SimpleNamespace(source=source, work=work, output=output,
                                  stable=Path("/admitted/stable"), identity=identity, calls=[])

        def run(label, command, timeout, *, sealed):
            session.calls.append((label, command, timeout, sealed))
            _, name, mode = label.split("-", 2)
            return ("running 1 test\ntest " + MODULE.PHASE_STEP_VALIDATION_TEST
                    + " ... PHASE_STEP_VALIDATION " + name + " " + mode
                    + "\nok\ntest result: ok. 1 passed; 0 failed; 0 ignored; "
                    + "0 measured; 243 filtered out; finished in 0.00s\n")

        session.run = run
        policy = {"critical_files": list(MODULE.PHASE_STEP_CRITICAL_FILES),
                  "tool_admission_profile": "stable-core", "contracts_admitted": True,
                  "validation_mechanisms_admitted": True,
                  "validation_mechanism_limits": dict(MODULE.PHASE_STEP_VALIDATION_LIMITS),
                  "inputs_sha256": dict(identity), "contracts_wall_seconds": 300}
        return session, policy

    def test_exact_seven_actual_raw_boundaries_have_fresh_source_bound_modes(self):
        rows = MODULE.phase_step_validation_cases()
        self.assertEqual(tuple(row["name"] for row in rows), MODULE.PHASE_STEP_VALIDATION_NAMES)
        self.assertEqual([row["raw_type"] for row in rows], [
            "RawAdaptiveProfile", "RawAdaptiveArea", "RawLimits", "RawStepRequest",
            "RawStepCompletion", "RawStepTerminal", "RawStep"])
        self.assertEqual(MODULE.PHASE_STEP_VALIDATION_LIMITS,
                         {"mechanisms": 7, "specimens_per_mechanism": 3, "test_executions": 21})
        for row in rows:
            with self.subTest(row=row["name"]):
                source = (ROOT / row["owner_path"]).read_text()
                raw_start = source.index("struct " + row["raw_type"] + " {")
                raw_end = source.index("\n}", raw_start)
                self.assertLessEqual(raw_start, source.index(row["sentinel_from"]))
                self.assertLess(source.index(row["sentinel_from"]), raw_end)
                variants = {mode: MODULE.phase_step_validation_variant(row, source, mode)
                            for mode in MODULE.PHASE_STEP_VALIDATION_MODES}
                self.assertEqual(variants["original"], source)
                self.assertEqual(variants["sentinel"].replace(row["sentinel_to"], row["sentinel_from"]), source)
                self.assertEqual(variants["omission"].replace(row["validate_to"], row["validate_from"]), variants["sentinel"])
                self.assertEqual(variants["omission"].count(row["validate_to"]), 1)
                for mode in MODULE.PHASE_STEP_VALIDATION_MODES:
                    specimen = MODULE.phase_step_validation_test(row, mode)
                    self.assertIn("fn observed() -> TestResult", specimen)
                    self.assertIn("PHASE_STEP_VALIDATION " + row["name"] + " " + mode, specimen)
                    self.assertNotIn("REFUSAL", specimen)
                    self.assertNotIn(".validate()", specimen)
                    self.assertIn("ValidationInvariant", specimen)
        rows[0]["name"] = "forged"
        self.assertEqual(MODULE.phase_step_validation_cases()[0]["name"], "adaptive_profile")

    def test_consumer_oracles_distinguish_before_and_after_known_consumption(self):
        rows = {row["name"]: row for row in MODULE.phase_step_validation_cases()}
        for name, required in {
            "adaptive_profile": ["FixtureAdaptiveHorizonProfile::new(4.0", "new(2.0", "InvalidProfile"],
            "adaptive_area": ["owner.receive_reference", "AdmittedNoPlan", "last_accepted_reference_active", "UnsupportedStatistic", "emitted_rounds(), 1"],
            "step_limits": ["FixtureStepLimits::new(1.0 / 16.0", "new(1.0 / 8.0", "InvalidLimits"],
            "step_request": ["owner.issue_step", "Magnitude", "step_proposal().is_some()", "next_ordinal(), Some(label(1)?)"],
            "step_completion": ["FixtureStepCompletion::new", "FixtureStepError::Receipt", "owner.step_proposal().is_some()"],
            "step_terminal": ["owner.settle_step", "OuterRejected", "FixtureStepError::Receipt", "ProvenNoEffect", "9.0 / 512.0", "Some(1.0 / 64.0)"],
            "numeric_step": ["owner.settle_step", "Known(retired)", "retired.debt_error()", "AchievedStep", "NonFinite", "pre_step_phase_seconds", "5.0 / 512.0", "retired.provenance()"],
        }.items():
            for token in required:
                with self.subTest(name=name, token=token):
                    self.assertIn(token, rows[name]["body"])

    def test_missing_or_ambiguous_actual_source_anchors_refuse(self):
        row = MODULE.phase_step_validation_cases()[0]
        source = (ROOT / row["owner_path"]).read_text()
        for changed in [source.replace(row["sentinel_from"], "removed"),
                        source + row["sentinel_from"],
                        source.replace(row["scope_start"], "removed"),
                        source.replace(row["validate_from"], "removed")]:
            with self.subTest(length=len(changed)):
                with self.assertRaises(ValueError):
                    MODULE.phase_step_validation_variant(row, changed, "omission")
        for mode in [None, "unknown"]:
            with self.assertRaises(ValueError):
                MODULE.phase_step_validation_variant(row, source, mode)

    def test_exact_separate_admission_and_deadline_precede_any_source_copy(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, policy = self.make_session(Path(temporary))
            for change in [
                {"validation_mechanisms_admitted": False},
                {"validation_mechanisms_admitted": 1}, {"contracts_admitted": False},
                {"validation_mechanism_limits": {**MODULE.PHASE_STEP_VALIDATION_LIMITS, "test_executions": 20}},
                {"validation_mechanism_limits": {**MODULE.PHASE_STEP_VALIDATION_LIMITS, "mechanisms": True}},
                {"validation_mechanism_limits": {**MODULE.PHASE_STEP_VALIDATION_LIMITS, "extra": 1}},
                {"critical_files": list(reversed(MODULE.PHASE_STEP_CRITICAL_FILES))},
                {"tool_admission_profile": "full"}, {"inputs_sha256": {}},
            ]:
                with self.subTest(change=change), self.assertRaisesRegex(ValueError, "separate exact admission"):
                    MODULE.phase_step_validation_mechanisms(session, {**policy, **change}, {}, deadline=time.monotonic() + 200)
            for deadline in [0, True, float("nan"), float("inf"), time.monotonic() + 1000]:
                with self.subTest(deadline=deadline), self.assertRaisesRegex(ValueError, "shared finite contract deadline"):
                    MODULE.phase_step_validation_mechanisms(session, policy, {}, deadline=deadline)
            self.assertFalse((session.work / "phase-step-validation-source").exists())
            self.assertEqual(session.calls, [])

    def test_mocked_complete_kernel_is_sealed_exact_restored_and_credit_free(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, policy = self.make_session(Path(temporary))
            report = {}
            MODULE.phase_step_validation_mechanisms(session, policy, report, deadline=time.monotonic() + 200)
            evidence = report["validation_mechanisms"]
            self.assertEqual(evidence["status"], "passed")
            self.assertEqual(evidence["generated_mutation_credit"], 0)
            self.assertEqual(len(session.calls), 21)
            self.assertEqual(len(evidence["specimens"]), 21)
            for label, command, timeout, sealed in session.calls:
                self.assertEqual(command[0], "/admitted/stable/bin/cargo")
                self.assertIn("--lib", command)
                self.assertIn("--exact", command)
                self.assertEqual(command[command.index("--jobs") + 1], "6")
                self.assertEqual(command[-3:], ["--exact", "--test-threads=1", "--nocapture"])
                self.assertGreater(timeout, 0)
                self.assertLessEqual(timeout, 90)
                self.assertEqual(sealed, [session.work / "phase-step-validation-source"])
                self.assertTrue(label.startswith("validation-"))
            for name, digest in session.identity.items():
                actual = hashlib.sha256((session.work / "phase-step-validation-source" / name).read_bytes()).hexdigest()
                self.assertEqual(actual, digest)

    def test_zero_wrong_duplicate_or_incomplete_execution_does_not_pass(self):
        variants = [
            lambda output: output.replace("running 1 test", "running 0 tests"),
            lambda output: output.replace(MODULE.PHASE_STEP_VALIDATION_TEST, "foreign::test"),
            lambda output: output.replace("1 passed; 0 failed; 0 ignored;", "0 passed; 0 failed; 1 ignored;"),
            lambda output: output + output,
            lambda output: output.replace("PHASE_STEP_VALIDATION", "MISSING_MARKER"),
        ]
        for change in variants:
            with self.subTest(change=change), tempfile.TemporaryDirectory() as temporary:
                session, policy = self.make_session(Path(temporary))
                original_run = session.run
                session.run = lambda *args, **kwargs: change(original_run(*args, **kwargs))
                report = {}
                with self.assertRaisesRegex(ValueError, "exactly once"):
                    MODULE.phase_step_validation_mechanisms(session, policy, report, deadline=time.monotonic() + 200)
                self.assertEqual(report["validation_mechanisms"]["status"], "incomplete")
                self.assertEqual(report["validation_mechanisms"]["specimens"], [])
                for name, digest in session.identity.items():
                    actual = hashlib.sha256((session.work / "phase-step-validation-source" / name).read_bytes()).hexdigest()
                    self.assertEqual(actual, digest)

    def test_actual_executor_failure_is_preserved_and_source_is_restored(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, policy = self.make_session(Path(temporary))
            session.run = Mock(side_effect=RuntimeError("actual isolated compiler failure"))
            report = {}
            with self.assertRaisesRegex(RuntimeError, "actual isolated compiler failure"):
                MODULE.phase_step_validation_mechanisms(session, policy, report, deadline=time.monotonic() + 200)
            self.assertEqual(report["validation_mechanisms"]["status"], "incomplete")
            self.assertEqual(report["validation_mechanisms"]["specimens"], [])
            for name, digest in session.identity.items():
                actual = hashlib.sha256((session.work / "phase-step-validation-source" / name).read_bytes()).hexdigest()
                self.assertEqual(actual, digest)


    def test_all_source_anchors_are_preflighted_before_the_first_dispatch(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, policy = self.make_session(Path(temporary))
            rows = list(MODULE.phase_step_validation_cases())
            rows[-1] = {**rows[-1], "validate_from": "missing selected validation"}
            with patch.object(MODULE, "phase_step_validation_cases", return_value=tuple(rows)):
                with self.assertRaisesRegex(ValueError, "missing or ambiguous"):
                    MODULE.phase_step_validation_mechanisms(session, policy, {}, deadline=time.monotonic() + 200)
            self.assertEqual(session.calls, [])
            self.assertFalse((session.work / "phase-step-validation-source").exists())

    def test_completion_after_shared_deadline_stays_incomplete(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, policy = self.make_session(Path(temporary))
            original_run, clock = session.run, [100.0]

            def run(*args, **kwargs):
                output = original_run(*args, **kwargs)
                if len(session.calls) == 21:
                    clock[0] = 101.1
                return output

            session.run = run
            report = {}
            with patch("time.monotonic", side_effect=lambda: clock[0]):
                with self.assertRaisesRegex(ValueError, "contract budget is exhausted"):
                    MODULE.phase_step_validation_mechanisms(session, policy, report, deadline=101.0)
            self.assertEqual(report["validation_mechanisms"]["status"], "incomplete")
            self.assertEqual(len(session.calls), 21)


if __name__ == "__main__":
    unittest.main()
