"""Controls for fixed external frequency specimens, not Rust execution evidence."""
from __future__ import annotations

import ast
import copy
import importlib.util
from pathlib import Path
import re
import unittest
from unittest.mock import Mock, patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_frequency_contracts", ROOT / "scripts/core-frequency-contracts.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def phase_examples():
    """Read only the existing code-owned pure factory, without importing a runner."""
    path = ROOT / "scripts/verify-core-analysis.py"
    node = next(item for item in ast.parse(path.read_text()).body
                if isinstance(item, ast.FunctionDef) and item.name == "phase_plan_contract_examples")
    namespace = {"Incomplete": ValueError}
    exec(compile(ast.Module(body=[node], type_ignores=[]), str(path), "exec"), namespace)
    return namespace["phase_plan_contract_examples"]()


class FrequencyContractControls(unittest.TestCase):
    """Challenge omission/overwrite/causal metadata before any expensive consumer run."""

    def setUp(self):
        self.session, self.report = object(), {}
        self.policy = {"external_consumer_limits": dict(MODULE.LIMITS)}
        self.execute = Mock(return_value="delegated")

    def invoke(self, factory=phase_examples):
        return MODULE.contracts(self.session, self.policy, self.report, self.execute, factory)

    def test_closed_new_registry_has_one_legal_and_eleven_unique_negatives(self):
        rows = MODULE.examples()
        self.assertIs(type(rows), tuple)
        self.assertEqual(len(rows), 12)
        self.assertEqual(len({name for name, _ in rows}), 12)
        cases = dict(rows)
        self.assertEqual(sum(row["code"] is None for row in cases.values()), 1)
        self.assertEqual(sum(row["code"] is not None for row in cases.values()), 11)
        self.assertEqual(MODULE.NEW_LIMITS, {"legal_executions": 1, "negative_compilations": 11})
        self.assertEqual(MODULE.INPUTS, ("scripts/core-frequency-contracts.py", "scripts/tests/test_core_frequency_contracts.py"))

    def test_new_causal_roles_pin_unique_actual_attempt_lines(self):
        expected = {
            "frequency_profile_literal": ("E0451", "FixtureFrequencyProfile"),
            "frequency_profile_mutation": ("E0616", "profile.growth_cap"),
            "frequency_association_literal": ("E0451", "FixtureFrequencyClockAssociation"),
            "frequency_association_mutation": ("E0616", "association.monotonic_origin"),
            "frequency_history_mutation": ("E0616", "history.model"),
            "frequency_projection_mutation": ("E0616", "projection.evaluated"),
            "frequency_raw_learn": ("E0599", "owner.learn"),
            "frequency_projection_as_reference": ("E0308", "owner.receive_reference(&projection"),
            "frequency_enabled_producer_lease": ("E0502", "producer.emitted_rounds"),
            "frequency_live_proposal_projection": ("E0499", "owner.project_model"),
            "frequency_owner_model_mutation": ("E0616", "owner.frequency"),
        }
        cases = dict(MODULE.examples())
        self.assertEqual(set(cases) - {MODULE.LEGAL_NAME}, set(expected))
        for name, (code, attempted) in expected.items():
            with self.subTest(name=name):
                row = cases[name]
                self.assertEqual(row["code"], code)
                self.assertRegex(row["code"], r"^E[0-9]{4}$")
                lines = row["source"].splitlines()
                primary = lines[row["line"] - 1]
                self.assertIn(attempted, primary)
                self.assertEqual(lines.count(primary), 1)
                self.assertTrue(row["tokens"])
                self.assertNotIn("rendered", row)
                self.assertNotIn("expected_output", row)

    def test_each_new_rejection_has_a_compiling_typed_neighbor_in_legal_source(self):
        cases = dict(MODULE.examples())
        legal = cases[MODULE.LEGAL_NAME]["source"]
        functions = set(re.findall(r"\bfn ([a-z_]+)(?:<|\()", legal))
        for name, row in cases.items():
            if row["code"] is None:
                continue
            with self.subTest(name=name):
                neighbor = row["external_legal_neighbor"]
                self.assertEqual(neighbor["specimen"], MODULE.LEGAL_NAME)
                self.assertIn(neighbor["function"], functions)
                self.assertIn("compiled typed neighbor", neighbor["scope"])
        self.assertIn("legal_resume", functions)
        self.assertIn("enabled_lease", functions)

    def test_legal_main_checks_profiles_actual_association_and_empty_receiving(self):
        # Durations are admitted by ordered endpoints; their fields stay private.
        for name, row in {**phase_examples(), **dict(MODULE.examples())}.items():
            for number, line in enumerate(row["source"].splitlines(), start=1):
                with self.subTest(specimen=name, line=number):
                    self.assertNotRegex(line, r"\b(?:ActiveAwake|Monotonic)DurationNs\s*::\s*new\b")
        legal = dict(MODULE.examples())[MODULE.LEGAL_NAME]["source"]
        self.assertIn("Err(FixtureFrequencyError::InvalidProfile)", legal)
        self.assertIn("FixtureFrequencyError::ClockAssociationMismatch", legal)
        self.assertIn("FixturePhasePlanOwner::new_with_frequency", legal)
        # A producer without observations still needs a nonempty registered roster.
        self.assertIn("FixtureCombinationState::seed(&[nz(11)?]", legal)
        self.assertIn("FixturePhaseReferenceAdmission::AdmittedNoPlan{emitted_round:None", legal)
        self.assertIn("FixtureFrequencyError::NoModel", legal)
        self.assertIn("assert_eq!(producer.emitted_rounds(),0)", legal)
        self.assertIn('println!("legal-frequency-contract");', legal)
        self.assertIn("retirement helpers compile but are not called", MODULE.LEGAL_SCOPE)
        self.assertIn("typed intake helper executes only for that empty batch", MODULE.LEGAL_SCOPE)
        self.assertNotIn("learning trajectory is executed", MODULE.LEGAL_SCOPE)

    def test_borrow_negatives_keep_the_authority_live_beyond_the_attempt(self):
        cases = dict(MODULE.examples())
        lease = cases["frequency_enabled_producer_lease"]
        self.assertGreater(lease["source"].index("black_box(owner)"),
                           lease["source"].index("black_box(producer.emitted_rounds())"))
        proposal = cases["frequency_live_proposal_projection"]
        self.assertGreater(proposal["source"].index("proposal.ordinal()"),
                           proposal["source"].index("owner.project_model(now)"))

    def test_union_preserves_all_legacy_sources_and_delegates_once(self):
        legacy = phase_examples()
        before = copy.deepcopy(legacy)
        self.assertEqual(self.invoke(lambda: legacy), "delegated")
        self.execute.assert_called_once()
        session, policy, report, cases, markers, scope = self.execute.call_args.args
        self.assertIs(session, self.session)
        self.assertIs(policy, self.policy)
        self.assertIs(report, self.report)
        self.assertEqual(legacy, before)
        self.assertEqual({name: cases[name] for name in legacy}, before)
        self.assertEqual(len(cases), 20)
        self.assertEqual(sum(row["code"] is None for row in cases.values()), 2)
        self.assertEqual(sum(row["code"] is not None for row in cases.values()), 18)
        self.assertEqual(markers, {"phase_plan_legal": "legal-phase-plan-contract\n",
                                   "frequency_legal": "legal-frequency-contract\n"})
        self.assertIn("compile but are not called", scope)

    def test_legacy_transfer_and_escape_sources_retain_their_exact_cause_context(self):
        legacy = phase_examples()
        self.invoke(lambda: legacy)
        cases = self.execute.call_args.args[3]
        for name, code, token in [
            ("phase_plan_double_transfer", "E0499", "retired.resume_fresh"),
            ("phase_plan_retirement_escape", "E0515", "retired.resume"),
        ]:
            with self.subTest(name=name):
                row = cases[name]
                self.assertEqual(row, legacy[name])
                self.assertEqual(row["code"], code)
                self.assertIn(token, row["source"].splitlines()[row["line"] - 1])

    def test_omitted_added_or_recoded_legacy_control_stops_before_executor(self):
        healthy = phase_examples()
        altered = []
        missing = copy.deepcopy(healthy)
        del missing["phase_plan_retirement_escape"]
        altered.append(missing)
        extra = copy.deepcopy(healthy)
        extra["unreviewed"] = {"source": "fn main() {}", "code": None}
        altered.append(extra)
        recoded = copy.deepcopy(healthy)
        recoded["phase_plan_double_transfer"]["code"] = "E0308"
        altered.append(recoded)
        for value in altered:
            with self.subTest(names=tuple(value)):
                with self.assertRaisesRegex(ValueError, "unchanged affected phase"):
                    self.invoke(lambda: value)
        self.execute.assert_not_called()

    def test_omitted_new_case_or_identity_collision_stops_before_executor(self):
        healthy = MODULE.examples()
        for altered in [healthy[:-1], healthy + (("phase_plan_retirement_escape", healthy[0][1]),)]:
            with self.subTest(size=len(altered)), patch.object(MODULE, "examples", return_value=altered):
                with self.assertRaises(ValueError):
                    self.invoke()
        self.execute.assert_not_called()

    def test_closed_limits_reject_bool_weakening_omission_and_extra_key(self):
        for limits in [None, {"legal_executions": 1, "negative_compilations": 18},
                       {"legal_executions": 2, "negative_compilations": 17},
                       {"legal_executions": 2, "negative_compilations": True},
                       {"legal_executions": 2}, {**MODULE.LIMITS, "ignored": 0}]:
            with self.subTest(limits=limits):
                self.policy["external_consumer_limits"] = limits
                with self.assertRaisesRegex(ValueError, r"fixed actual2\+18"):
                    self.invoke()
        self.execute.assert_not_called()

    def test_policy_source_fields_cannot_replace_fixed_specimen_truth(self):
        self.policy.update(examples={"injected": {"source": "fn main(){}", "code": None}},
                           expected_marker="pretend-pass")
        self.invoke()
        cases, markers = self.execute.call_args.args[3:5]
        self.assertNotIn("injected", cases)
        self.assertNotIn("pretend-pass", markers.values())
        self.assertEqual(set(markers), {"phase_plan_legal", "frequency_legal"})

    def test_registry_calls_return_fresh_metadata_without_mutable_shared_truth(self):
        first = MODULE.examples()
        first[1][1]["tokens"].append("poison")
        first[1][1]["code"] = "E0000"
        second = MODULE.examples()
        self.assertNotIn("poison", second[1][1]["tokens"])
        self.assertEqual(second[1][1]["code"], "E0451")

    def test_delegated_failure_propagates_to_the_existing_session_finalizer(self):
        error = RuntimeError("intended strict compiler cause is absent")
        self.execute.side_effect = error
        with self.assertRaisesRegex(RuntimeError, "intended strict compiler cause"):
            self.invoke()
        self.assertEqual(self.report, {})
        self.execute.assert_called_once()


if __name__ == "__main__":
    unittest.main()
