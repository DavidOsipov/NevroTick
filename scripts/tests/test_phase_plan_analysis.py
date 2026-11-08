"""Focused phase-plan runner controls; synthetic data executes no product jobs."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import tomllib
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("phase_plan_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
SOURCE_CASES = MODULE.PHASE_PLAN_REQUIRED_TESTS
CASES = frozenset({"healthy_phase_trajectory", "phase_numeric_private_control"})
DOCS = ((MODULE.PHASE_PLAN_FILE, "reference_input::rate_delivery::phase_plan::FixturePhaseBaseline", False, 1),
        (MODULE.PHASE_PLAN_FILE, "reference_input::rate_delivery::phase_plan::FixturePhasePlanOwner", True, 1))


def native_log(names):
    """Literal named successes are parser controls, not native execution evidence."""
    return ("running " + str(len(names)) + " tests\n"
            + "".join("test " + name + " ... ok\n" for name in names)
            + "test result: ok. " + str(len(names))
            + " passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


def mutation_session(work, events):
    """Own only temporary config bytes; no Cargo or product execution occurs."""
    config = work / "cargo-home/config.toml"
    config.parent.mkdir(parents=True)
    vendor = work / "vendor"
    session = SimpleNamespace(config=config, vendor=vendor)
    def configure(text):
        events.append("configure-mutation")
        config.write_text(text)
        session.config_sha256 = hashlib.sha256(config.read_bytes()).hexdigest()
    session.configure = configure
    configure('[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory='
              + json.dumps(str(vendor)) + '\n')
    events.clear()
    return session


def profile_equivalence():
    """Literal observed fault; test metadata grants no native outcome or kill."""
    return {
        "id": "PHASE-PROFILE-NORMAL-POSITIVE", "candidate_sha256": "a" * 64,
        "source_sha256": "7a332363cc72c569a0a99fa5adbba4b5695bbb1367ea60d08449ed122d5bee13",
        "mutant": {
            "file": "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs",
            "function": {"function_name": "profile_valid", "return_type": "-> bool",
                         "span": {"start": {"line": 169, "column": 1},
                                  "end": {"line": 177, "column": 2}}},
            "genre": "BinaryOperator", "package": "time-core", "replacement": ">=",
            "name": "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:176:42: replace > with >= in profile_valid",
            "span": {"start": {"line": 176, "column": 42}, "end": {"line": 176, "column": 43}},
        },
        "requirement": "CHECK-7/8/25; complete raw profile scalar domain",
        "rationale": "Normality excludes signed zeros, subnormals, infinities and NaNs; the two comparisons agree on both normal signs.",
        "proof": {"path": "tasks/phase-plan-analysis-equivalences.md",
                  "sha256": "a73747280f00ec21fe896d3e38d0fd1826f8d49fd93683ef01f7829d2eddb227",
                  "section": "PHASE-PROFILE-NORMAL-POSITIVE",
                  "reviewer": "Independent Python custody control",
                  "verdict": "independently-justified-equivalent"},
    }


def numeric_equivalences():
    """Three complete observed descriptors, independent of the runner allowlist."""
    filename = "crates/time-core/src/regression/chart/roundoff/phase_plan.rs"
    entries = []
    for identifier, name, line, column, replacement, genre, function, first, last, result in (
        ("PHASE-NUMERIC-ADDITION-ABS-SIGN", "delete -", 431, 13, "", "UnaryOperator",
         "PhaseDebt::settled", 375, 443, "-> Result<Self, FixturePhaseNumericError>"),
        ("PHASE-NUMERIC-POSITIVE-CLAMP-EQUALITY", "replace > with >=", 493, 37, ">=", "BinaryOperator",
         "PhaseDebt::proposal", 445, 527, "-> Result<PhaseRates, FixturePhaseNumericError>"),
        ("PHASE-NUMERIC-NEGATIVE-CLAMP-EQUALITY", "replace < with <=", 495, 28, "<=", "BinaryOperator",
         "PhaseDebt::proposal", 445, 527, "-> Result<PhaseRates, FixturePhaseNumericError>"),
    ):
        entry = profile_equivalence()
        entry.update(id=identifier,
                     source_sha256="2d2e5ecf16c6aed1131f63420bae97f7037074b4511f2f9744be9fcdc31cddc9",
                     requirement="CHECK-7/8/25; checked numerical result and enclosure",
                     rationale="The reviewed exact fault preserves every admitted result bit, bound and typed failure.")
        entry["proof"]["section"] = identifier
        entry["mutant"] = {
            "file": filename, "package": "time-core", "genre": genre, "replacement": replacement,
            "name": filename + ":" + str(line) + ":" + str(column) + ": " + name + " in " + function,
            "function": {"function_name": function, "return_type": result,
                         "span": {"start": {"line": first, "column": 5},
                                  "end": {"line": last, "column": 6}}},
            "span": {"start": {"line": line, "column": column},
                     "end": {"line": line, "column": column + 1}},
        }
        entries.append(entry)
    return entries


class PhasePlanAnalysisTests(unittest.TestCase):
    """Challenge role isolation, independent gates and current source custody."""

    def setUp(self):
        """Small synthetic minima exercise mechanics without production claims."""
        self.addCleanup(patch.stopall)
        patch.object(MODULE, "PHASE_PLAN_REQUIRED_TESTS", CASES).start()
        patch.object(MODULE, "PHASE_PLAN_REQUIRED_DOCTESTS", DOCS).start()

    def policy(self):
        """Use complete synthetic rosters, never inherited candidate bindings."""
        ordinary = sorted(CASES | {"unrelated_library_control"})
        docs = [filename + " - " + owner + " (line " + str(10 + index) + ")"
                + (" - compile fail" if negative else "")
                for index, (filename, owner, negative, _count) in enumerate(DOCS)]
        return {**copy.deepcopy(MODULE.PHASE_PLAN_FIXED),
                "source_file_count": 1, "candidate_sha256": "a" * 64,
                "inputs_sha256": {}, "scope": "Synthetic runner control only.",
                "mutation_inventories": {name: {"count": 1, "sha256": "b" * 64}
                                         for name in MODULE.PHASE_PLAN_SUBJECTS},
                "mutation_equivalences": [],
                "test_inventories": {"coverage": ordinary, "baseline": sorted([*ordinary, *docs])}}

    def test_role_and_dispatch_leave_historical_owners_unchanged(self):
        before = copy.deepcopy((MODULE.CONDITIONED_WINDOW_SUBJECTS, MODULE.CONDITIONED_FIT_SUBJECTS,
                                MODULE.CONDITIONED_WINDOW_FIXED, MODULE.CONDITIONED_FIT_FIXED,
                                MODULE.SUBJECTS, MODULE.FIXED))
        role = MODULE._phase_plan_role()
        self.assertEqual(role.scope.select("all"), {
            "phase_plan": "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs",
            "phase_plan_numeric": "crates/time-core/src/regression/chart/roundoff/phase_plan.rs"})
        self.assertEqual(role.coverage_tests, ("phase_plan",))
        self.assertEqual(role.fixed["mutation_jobs"], 10)
        self.assertEqual(role.fixed["mutation_debug_info"], 0)
        self.assertIs(type(role.fixed["mutation_debug_info"]), int)
        self.assertNotIn("mutation_debug_info", MODULE.CONDITIONED_WINDOW_FIXED)
        self.assertNotIn("mutation_debug_info", MODULE.CONDITIONED_FIT_FIXED)
        self.assertEqual(MODULE.CONDITIONED_WINDOW_FIXED["mutation_jobs"], 8)
        self.assertEqual(MODULE.CONDITIONED_FIT_FIXED["mutation_jobs"], 8)
        self.assertNotIn(role.scope.policy, {MODULE.CONDITIONED_WINDOW_POLICY, MODULE.CONDITIONED_FIT_POLICY,
                                           MODULE.POLICY})
        with patch.object(MODULE, "main_phase_plan", return_value=0) as selected, \
             patch.object(MODULE, "main_conditioned_fit") as old_fit, \
             patch.object(MODULE, "main_conditioned_window") as old_window:
            self.assertEqual(MODULE.main(["--subject", "phase_plan", "--lane", "contracts",
                                          "--output-dir", "/owned/output"]), 0)
            selected.assert_called_once()
            old_fit.assert_not_called()
            old_window.assert_not_called()
        with patch.object(MODULE, "execute_phase_plan") as selected, \
             patch.object(MODULE, "read_policy") as historical:
            MODULE.execute(SimpleNamespace(subject="phase_plan"), {})
            selected.assert_called_once()
            historical.assert_not_called()
        self.assertEqual(before, (MODULE.CONDITIONED_WINDOW_SUBJECTS, MODULE.CONDITIONED_FIT_SUBJECTS,
                                  MODULE.CONDITIONED_WINDOW_FIXED, MODULE.CONDITIONED_FIT_FIXED,
                                  MODULE.SUBJECTS, MODULE.FIXED))

    def test_pending_policy_stops_before_session_or_product_execution(self):
        pending = self.policy()
        pending.update(candidate_sha256=None, source_file_count=None, inputs_sha256={})
        with patch.object(MODULE, "inventory_document", return_value=pending), \
             patch.object(MODULE.COMMON, "Session") as session, \
             patch.object(MODULE.PRODUCT, "admit_product") as product:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_phase_plan_policy(ROOT)
            session.assert_not_called()
            product.assert_not_called()

    def test_equivalence_scope_allows_only_unique_subsets_of_four_observed_faults(self):
        approved = [profile_equivalence(), *numeric_equivalences()]
        expected = (
            "131b5b15383236d6dcc596b9750cd491c2ea4933decf5a4f2db38b4c819b5f9e",
            "0e3501f19639eca712ca0a6bbe423178d91415996edc3963b7f7f0bcadbb9a3a",
            "157595293ff60b8a3a3c036a78382a8b4a2102405edadcb6f4eccb36161aa229",
            "e74479b761cec558fdce8ce470058f36d48ad71afa8ba27d75d1fca6f3a10d1f",
        )
        inputs = {"crates/time-core/src/regression/chart/roundoff.rs":
                  "c8a05da2b622468426063a3addb17e2c27d8ba9856efee9c53ff7f6d7201166f"}
        self.assertEqual(tuple(MODULE.COMMON.canonical_sha256(e["mutant"]) for e in approved), expected)
        for mask in range(16):
            entries = [entry for index, entry in enumerate(approved) if mask & (1 << index)]
            with self.subTest(mask=mask):
                self.assertIsNone(MODULE.validate_phase_plan_equivalence_scope(entries, inputs))
        for entry in approved:
            for fault in ("wrong_id", "unhashable_id", "descriptor", "with_diff", "source", "proof_path",
                          "proof_hash", "section", "duplicate", "unknown_field", "tuple", "missing"):
                altered = copy.deepcopy(entry)
                entries = [altered]
                if fault == "wrong_id":
                    altered["id"] = "HISTORICAL-OR-UNREVIEWED"
                elif fault == "unhashable_id":
                    altered["id"] = [entry["id"]]
                elif fault == "descriptor":
                    altered["mutant"]["replacement"] = "unreviewed replacement"
                elif fault == "with_diff":
                    altered["mutant"]["diff"] = "foreign rendered diff"
                elif fault == "source":
                    altered["source_sha256"] = "b" * 64
                elif fault == "proof_path":
                    altered["proof"]["path"] = "tasks/s1.3-source-selection-readiness.md"
                elif fault == "proof_hash":
                    altered["proof"]["sha256"] = "b" * 64
                elif fault == "section":
                    altered["proof"]["section"] = "FOREIGN-SECTION"
                elif fault == "duplicate":
                    entries.append(copy.deepcopy(altered))
                elif fault == "unknown_field":
                    altered["extra"] = "not admitted"
                elif fault == "tuple":
                    entries = (altered,)
                else:
                    entries = None
                with self.subTest(identifier=entry["id"], fault=fault), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_phase_plan_equivalence_scope(entries, inputs)
        for parent in ({}, {next(iter(inputs)): "b" * 64}, None):
            with self.subTest(parent=parent), self.assertRaisesRegex(MODULE.Incomplete, "parent helper"):
                MODULE.validate_phase_plan_equivalence_scope(approved, parent)

    def test_equivalence_keeps_actual_common_source_candidate_and_proof_admission(self):
        # The generic custody boundary receives honestly rebound current bytes.
        # This is not adoption of these historical faults for the current role.
        _, current_identity = MODULE.PRODUCT.admit_product(ROOT)
        current_entries = [profile_equivalence(), *numeric_equivalences()]
        for entry in current_entries:
            entry["candidate_sha256"] = MODULE.COMMON.canonical_sha256(current_identity)
            entry["source_sha256"] = MODULE.PRODUCT.sha256(ROOT / entry["mutant"]["file"])
            entry["proof"]["sha256"] = MODULE.PRODUCT.sha256(ROOT / entry["proof"]["path"])
        self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(
            ROOT, MODULE.PHASE_PLAN_SCOPE, {"mutation_equivalences": current_entries}), current_entries)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            proof = "tasks/phase-plan-analysis-equivalences.md"
            parent = "crates/time-core/src/regression/chart/roundoff.rs"
            owners = tuple(MODULE.PHASE_PLAN_SUBJECTS.values())
            bound = (*owners, parent, proof)
            original = {name: (ROOT / name).read_bytes() for name in bound}
            for name, data in original.items():
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)
            (root / "policy").mkdir()
            (root / "policy/verification-manifest.toml").write_text(
                "critical_files = " + json.dumps(list(owners))
                + '\nsecurity_modules = [{package = "time-core", files = ' + json.dumps(list(owners)) + "}]\n")
            identity = {name: hashlib.sha256(original[name]).hexdigest() for name in owners}
            policy = self.policy()
            policy.update(source_file_count=len(identity), candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                          inputs_sha256={name: hashlib.sha256(data).hexdigest() for name, data in original.items()})
            entries = [profile_equivalence(), *numeric_equivalences()]
            for entry in entries:
                entry["candidate_sha256"] = policy["candidate_sha256"]
            historical_entries = copy.deepcopy(entries)
            for entry in entries:
                entry["source_sha256"] = identity[entry["mutant"]["file"]]
                entry["proof"]["sha256"] = hashlib.sha256(original[proof]).hexdigest()
            policy["mutation_equivalences"] = entries
            target = root / MODULE.PHASE_PLAN_POLICY
            with patch.object(MODULE, "PHASE_PLAN_BOUND_INPUTS", bound), \
                 patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                historical = {**policy, "mutation_equivalences": historical_entries}
                target.write_text(json.dumps(historical))
                with self.assertRaisesRegex(MODULE.Incomplete, "source or complete candidate"):
                    MODULE.read_phase_plan_policy(root)
                # Even honest rebinding passes only generic custody, not the
                # unchanged historical descriptor/source/proof whitelist.
                target.write_text(json.dumps(policy))
                with self.assertRaisesRegex(MODULE.Incomplete, "exact reviewed descriptor/source/proof"):
                    MODULE.read_phase_plan_policy(root)
                for fault in ("source_pin", "candidate", "actual_source", "actual_proof", "proof_heading", "proof_cap",
                              "actual_parent"):
                    altered = copy.deepcopy(policy)
                    if fault == "actual_parent":
                        # Historical metadata-only healthy neighbor, not admission
                        # of the currently changed parent implementation.
                        historical_parent = {parent: "c8a05da2b622468426063a3addb17e2c27d8ba9856efee9c53ff7f6d7201166f"}
                        self.assertIsNone(MODULE.validate_phase_plan_equivalence_scope(
                            historical_entries, historical_parent))
                    if fault == "source_pin":
                        altered["mutation_equivalences"][0]["source_sha256"] = "b" * 64
                    elif fault == "candidate":
                        altered["mutation_equivalences"][0]["candidate_sha256"] = "b" * 64
                    else:
                        name = owners[0] if fault == "actual_source" else parent if fault == "actual_parent" else proof
                        data = original[name] + b"\nchanged immutable input\n"
                        if fault == "proof_heading":
                            data = original[name].replace(b"## PHASE-PROFILE-NORMAL-POSITIVE", b"## FOREIGN-HEADING")
                        elif fault == "proof_cap":
                            data = original[name] + b" " * 65536
                        (root / name).write_bytes(data)
                        altered["inputs_sha256"][name] = hashlib.sha256(data).hexdigest()
                        if fault in {"proof_heading", "proof_cap"}:
                            for entry in altered["mutation_equivalences"]:
                                entry["proof"]["sha256"] = hashlib.sha256(data).hexdigest()
                    target.write_text(json.dumps(altered))
                    with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                        if fault == "actual_parent":
                            MODULE.validate_phase_plan_equivalence_scope(
                                historical_entries, altered["inputs_sha256"])
                        else:
                            MODULE.COMMON.admit_mutation_equivalences(root, MODULE.PHASE_PLAN_SCOPE, altered)
                    for name, data in original.items():
                        (root / name).write_bytes(data)

    def test_executed_repo_loader_closure_is_bound_without_historical_results(self):
        required = {
            "scripts/verify-core-analysis.py", "scripts/portable-analysis.py",
            "scripts/verify-time-domain-analysis.py",
            "scripts/core-selection-accounting.py", "scripts/core-selection-contracts.py",
            "scripts/core-projection-accounting.py", "scripts/core-analysis-cohorts.py",
            "scripts/core-enclosure-accounting.py", "scripts/core-projected-batch-accounting.py",
            "scripts/core-combination-accounting.py", "scripts/core-adaptive-accounting.py",
            "scripts/core-adaptive-contracts.py", "scripts/core-chart-accounting.py",
            "scripts/core-chart-contracts.py", "scripts/core-chart-combination-accounting.py",
            "scripts/core-chart-combination-contracts.py", "scripts/core-rate-delivery-accounting.py",
            "scripts/check-analysis.py", "scripts/analysis-runtime.py", "scripts/analysis-controls.py",
            "scripts/verify.py", "scripts/prepare-product.py", "scripts/check-environment.py",
            "scripts/check-strict.py", "scripts/slint_dependency_profile.py", "scripts/slint_linux_sdk.py",
        }
        bound = MODULE._phase_plan_role().bound_inputs
        self.assertEqual(set(MODULE.PHASE_PLAN_HELPER_INPUTS), required)
        self.assertTrue(required <= set(bound))
        self.assertEqual(len(bound), len(set(bound)))
        self.assertNotIn(MODULE.CONDITIONED_FIT_POLICY, bound)
        self.assertNotIn(MODULE.CONDITIONED_WINDOW_POLICY, bound)

    def test_each_owner_inventory_and_combined_mutant_cap_are_mandatory(self):
        role, healthy = MODULE._phase_plan_role(), self.policy()
        self.assertEqual(set(MODULE._conditioned_inventory_bindings(healthy, role, required=True)),
                         {"phase_plan", "phase_plan_numeric"})
        for bindings in ({}, {"phase_plan": healthy["mutation_inventories"]["phase_plan"]},
                         {name: {"count": 257, "sha256": "b" * 64}
                          for name in MODULE.PHASE_PLAN_SUBJECTS},
                         {name: {"count": 0, "sha256": "b" * 64}
                          for name in MODULE.PHASE_PLAN_SUBJECTS}):
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_inventory_bindings({**healthy, "mutation_inventories": bindings},
                                                       role, required=True)

    def test_rosters_reject_omissions_substitutions_duplicates_and_failed_records(self):
        policy, role = self.policy(), MODULE._phase_plan_role()
        names = policy["test_inventories"]["coverage"]
        self.assertEqual(MODULE._conditioned_test_execution(native_log(names), policy, role, doctests=False)
                         ["tests_executed"], len(names))
        for raw in (native_log(names[:-1]), native_log([*names, names[0]]),
                    native_log(["unreviewed_control", *names[1:]]),
                    native_log(names).replace(" ... ok", " ... FAILED", 1)):
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_execution(raw, policy, role, doctests=False)
        for lane in ("baseline", "coverage"):
            altered = copy.deepcopy(policy)
            altered["test_inventories"][lane] = []
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(altered, role)
        altered = copy.deepcopy(policy)
        altered["test_inventories"]["baseline"] = names
        with self.assertRaises(MODULE.Incomplete):
            MODULE._conditioned_test_inventories(altered, role)

    def test_measured_gap_controls_are_code_owned_and_cannot_be_omitted(self):
        required = {
            "lifecycle_error_families_keep_bounded_redacted_diagnostics_and_typed_causes",
            "settled_cursor_constructor_refusals_preserve_custody_and_first_reference_credit",
            "constructor_deadline_overflow_has_independent_exact_clock_neighbors",
            "terminal_cursor_construction_cannot_restore_unknown_or_exhausted_authority",
            "issue_deadlines_and_no_effect_permission_preserve_original_operation_budget",
            "aggregate_cap_refusal_has_an_exact_cap_neighbor_without_hidden_issuance",
            "reference_input::rate_delivery::phase_plan::tests::no_effect_latches_each_deadline_before_later_issue_can_mask_it",
            "independent_target_tag_mismatches_preserve_cursor_and_source_credit",
            "genuine_phase_envelope_has_adjacent_declared_cap_neighbors",
            "actual_rate_cap_and_zero_transient_keep_both_debt_signs",
            "negative_debt_direction_and_zero_residual_have_typed_neighbors",
            "zero_phase_baseline_only_issuance_accepts_both_actual_rate_signs",
            "postsettlement_phase_cap_distinguishes_equal_and_rounded_excess",
            "late_known_delivery_reports_each_expiry_before_transfer",
            "fresh_transfer_baseline_cap_has_signed_adjacent_neighbors",
            "terminal_current_clocks_contain_delayed_known_delivery_independently",
            "regression::chart::roundoff::phase_plan::tests::numeric_error_families_redact_and_delegate_typed_sources",
            "regression::chart::roundoff::phase_plan::tests::checked_debt_and_rates_debug_are_opaque_over_distinct_values",
            "regression::chart::roundoff::phase_plan::tests::transformed_scalar_faults_fail_raw_and_checked_rate_admission",
            "regression::chart::roundoff::phase_plan::tests::transformed_branch_faults_fail_raw_and_checked_rate_admission",
            "regression::chart::roundoff::phase_plan::tests::settlement_raw_rules_reject_before_zero_duration_shortcuts",
            "regression::chart::roundoff::phase_plan::tests::zero_delivery_and_bound_exhaustion_preserve_prior_debt",
            "regression::chart::roundoff::phase_plan::tests::noninteger_horizon_error_includes_denominator_conversion",
            "regression::chart::roundoff::phase_plan::tests::settlement_rounding_order_preserves_noninteger_duration",
            "regression::chart::roundoff::phase_plan::tests::proposal_input_rules_preserve_caps_and_baseline_boundaries",
            "regression::chart::roundoff::phase_plan::tests::proposal_error_encloses_inherited_debt_interval",
            "regression::chart::roundoff::phase_plan::tests::delivery_error_preserves_zero_factor_identity",
        }
        self.assertTrue(required <= SOURCE_CASES)
        private = "crates/time-core/tests/private/phase_plan_lifecycle.rs"
        self.assertIn(private, MODULE._phase_plan_role().bound_inputs)
        self.assertIn('#[path = "../../../tests/private/phase_plan_lifecycle.rs"]\nmod tests;',
                      (ROOT / MODULE.PHASE_PLAN_FILE).read_text())
        self.assertIn("#[test]\nfn no_effect_latches_each_deadline_before_later_issue_can_mask_it()",
                      (ROOT / private).read_text())
        policy = self.policy()
        docs = [name for name in policy["test_inventories"]["baseline"] if " (line " in name]
        policy["test_inventories"] = {
            "coverage": sorted(SOURCE_CASES), "baseline": sorted([*SOURCE_CASES, *docs])}
        with patch.object(MODULE, "PHASE_PLAN_REQUIRED_TESTS", SOURCE_CASES):
            role = MODULE._phase_plan_role()
            self.assertEqual(MODULE._conditioned_test_inventories(policy, role), policy["test_inventories"])
            for missing in required:
                altered = copy.deepcopy(policy)
                for names in altered["test_inventories"].values():
                    names.remove(missing)
                with self.subTest(missing=missing), self.assertRaises(MODULE.Incomplete):
                    MODULE._conditioned_test_inventories(altered, role)

    def test_current_candidate_helper_pins_and_source_only_owners_are_enforced(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            helper = "scripts/analysis-controls.py"
            bound = (helper, *MODULE.PHASE_PLAN_SUBJECTS.values())
            for filename in bound:
                path = root / filename
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("// synthetic checked owner\n" if filename.endswith(".rs") else "# helper\n")
            (root / "policy").mkdir()
            manifest = root / "policy/verification-manifest.toml"
            manifest.write_text("critical_files = " + json.dumps(list(MODULE.PHASE_PLAN_SUBJECTS.values()))
                + "\nsecurity_modules = [{package = \"time-core\", files = "
                + json.dumps(list(MODULE.PHASE_PLAN_SUBJECTS.values())) + "}]\n")
            identity = {"candidate.rs": "c" * 64}
            policy = self.policy()
            policy.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                          inputs_sha256={name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                                         for name in bound})
            target = root / MODULE.PHASE_PLAN_POLICY
            with patch.object(MODULE, "PHASE_PLAN_BOUND_INPUTS", bound), \
                 patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                target.write_text(json.dumps(policy))
                self.assertEqual(MODULE.read_phase_plan_policy(root), policy)
                for change in ("candidate", "helper", "helper_drift", "owner", "manifest", "equivalence", "workers", "debug_info"):
                    altered = copy.deepcopy(policy)
                    if change == "candidate":
                        altered["candidate_sha256"] = "d" * 64
                    elif change == "helper":
                        altered["inputs_sha256"].pop(helper)
                    elif change == "helper_drift":
                        (root / helper).write_text("# changed after admission\n")
                    elif change == "owner":
                        filename = MODULE.PHASE_PLAN_SUBJECTS["phase_plan_numeric"]
                        (root / filename).write_text("#[test]\nfn hidden() {}\n")
                        altered["inputs_sha256"][filename] = hashlib.sha256((root / filename).read_bytes()).hexdigest()
                    elif change == "manifest":
                        manifest.write_text("critical_files = []\nsecurity_modules = []\n")
                    elif change == "workers":
                        altered["mutation_jobs"] = 8
                    elif change == "debug_info":
                        altered["mutation_debug_info"] = False
                    else:
                        altered["mutation_equivalences"] = [{"historical": "not admitted"}]
                    target.write_text(json.dumps(altered))
                    with self.subTest(change=change):
                        if change in {"helper", "helper_drift"}:
                            with self.assertRaisesRegex(MODULE.Incomplete, "protected source/helper bindings changed"):
                                MODULE.read_phase_plan_policy(root)
                        else:
                            with self.assertRaises(MODULE.Incomplete):
                                MODULE.read_phase_plan_policy(root)
                    for filename in MODULE.PHASE_PLAN_SUBJECTS.values():
                        (root / filename).write_text("// synthetic checked owner\n")
                    (root / helper).write_text("# helper\n")
                    manifest.write_text("critical_files = " + json.dumps(list(MODULE.PHASE_PLAN_SUBJECTS.values()))
                        + "\nsecurity_modules = [{package = \"time-core\", files = "
                        + json.dumps(list(MODULE.PHASE_PLAN_SUBJECTS.values())) + "}]\n")

    def test_ten_worker_disk_diagnostic_preserves_historical_eight_worker_wording(self):
        args = SimpleNamespace(subject="phase_plan", lane="mutation", tool_admission_profile="full",
                               output_dir=Path("/owned/output"))
        roles = [(MODULE._phase_plan_role(), "ten"),
                 (MODULE._conditioned_window_role(), "eight"),
                 (MODULE._conditioned_fit_role(), "eight")]
        for role, word in roles:
            policy = {**copy.deepcopy(role.fixed), "mutation_inventories": {
                name: {"count": 1, "sha256": "b" * 64} for name in role.scope.select("all")}}
            selected = SimpleNamespace(**{**vars(args), "subject": role.selector})
            with self.subTest(role=role.selector), \
                 patch.object(MODULE.COMMON, "_require_private_output"), \
                 patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5 * 1024 ** 3 - 1)), \
                 patch.object(MODULE.COMMON, "Session") as session, \
                 self.assertRaisesRegex(MODULE.Incomplete, word + "-worker mutation requires at least five GiB"):
                MODULE._execute_conditioned(selected, {}, role, read_policy=lambda _root: policy)
            session.assert_not_called()

    def test_each_file_gate_and_final_custody_failure_prevent_success(self):
        for failure in (None, "phase_plan", "phase_plan_numeric", "mutation", "finish"):
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                role, policy, report, events = MODULE._phase_plan_role(), self.policy(), {}, []
                identity = {"candidate.rs": "c" * 64}
                policy.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                              inputs_sha256={name: "d" * 64 for name in role.bound_inputs})
                def run(label, command, timeout, **_options):
                    events.append(label)
                    if label == "baseline":
                        return native_log(policy["test_inventories"]["baseline"])
                    self.assertEqual(label, "coverage")
                    self.assertIn("--lib", command)
                    self.assertEqual([command[index + 1] for index, value in enumerate(command) if value == "--test"],
                                     ["phase_plan"])
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename,
                         "summary": {"lines": {"count": 10, "covered": 8 if owner == failure else 9}}}
                        for owner, filename in role.scope.select("all").items()]}]}))
                    return native_log(policy["test_inventories"]["coverage"])
                def finish():
                    events.append("finish")
                    if failure == "finish":
                        raise MODULE.Incomplete("final source custody changed")
                def mutation(scope, subjects, admitted, actual_work, tools, execute, actual_report, lane):
                    self.assertEqual((scope, subjects, lane), (role.scope, role.scope.select("all"), "mutation"))
                    self.assertEqual(set(actual_report["coverage"]), {"phase_plan", "phase_plan_numeric"})
                    self.assertEqual(events[-1], "configure-mutation")
                    self.assertEqual(events[:3], ["baseline", "coverage", "configure-mutation"])
                    events.append("mutation")
                    if failure == "mutation":
                        raise MODULE.Incomplete("one owner failed its mutation gate")
                session = mutation_session(work, events)
                session.__dict__.update(product_identity=identity, source=work, work=work,
                    tools=work / "tools", run=run, finish=finish)
                args = SimpleNamespace(subject="phase_plan", lane="all", tool_admission_profile="full",
                                       output_dir=work / "output")
                with patch.object(MODULE, "read_phase_plan_policy", return_value=policy), \
                     patch.object(MODULE, "inventory_document", return_value=policy), \
                     patch.object(MODULE.PRODUCT, "sha256", return_value="d" * 64), \
                     patch.object(MODULE.COMMON, "_require_private_output"), \
                     patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=6 * 1024 ** 3)), \
                     patch.object(MODULE.COMMON, "Session", return_value=session), \
                     patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=mutation):
                    if failure is None:
                        MODULE.execute_phase_plan(args, report)
                        self.assertTrue(report["qualification_passed"])
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute_phase_plan(args, report)
                        self.assertFalse(report["qualification_passed"])
                    self.assertEqual(events[-1], "finish")

    def test_mutation_debug_profile_changes_only_sealed_symbol_information(self):
        with tempfile.TemporaryDirectory() as temporary:
            session = mutation_session(Path(temporary), [])
            original = session.config.read_bytes()
            report = {}
            MODULE.configure_phase_plan_mutation(session, self.policy(), report, MODULE._phase_plan_role())
            record = report["mutation_debug_configuration"]
            self.assertEqual(record["before_text"].encode(), original)
            self.assertEqual(record["before_sha256"], hashlib.sha256(original).hexdigest())
            self.assertEqual(record["after_text"].encode(), session.config.read_bytes())
            self.assertEqual(record["after_sha256"], hashlib.sha256(session.config.read_bytes()).hexdigest())
            expected = tomllib.loads(original.decode())
            expected["profile"] = {"dev": {"debug": 0}, "test": {"debug": 0}}
            self.assertEqual(tomllib.loads(session.config.read_text()), expected)
            self.assertFalse(record["other_profile_settings_changed"])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.configure_phase_plan_mutation(session, self.policy(), {}, MODULE._phase_plan_role())

    def test_mutation_debug_profile_rejects_wrong_type_vendor_or_other_settings(self):
        failures = ["missing", False, True, 1, -1, "0", "vendor", "registry", "build", "profile",
                    "test-profile", "seal", "window-role", "fit-role"]
        for fault in failures:
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                events, report = [], {}
                session = mutation_session(Path(temporary), events)
                policy, role = self.policy(), MODULE._phase_plan_role()
                if fault == "missing":
                    policy.pop("mutation_debug_info")
                elif type(fault) is not str or fault == "0":
                    policy["mutation_debug_info"] = fault
                elif fault == "window-role":
                    role = MODULE._conditioned_window_role()
                elif fault == "fit-role":
                    role = MODULE._conditioned_fit_role()
                else:
                    changed = session.config.read_text()
                    if fault == "vendor":
                        changed = changed.replace(str(session.vendor), "/foreign/vendor")
                    elif fault == "registry":
                        changed = changed.replace('replace-with="reviewed"', 'replace-with="foreign"')
                    elif fault == "build":
                        changed += "\n[build]\njobs = 1\n"
                    elif fault == "profile":
                        changed += "\n[profile.dev]\nopt-level = 3\n"
                    elif fault == "test-profile":
                        changed += "\n[profile.test]\ndebug = 0\n"
                    else:
                        changed += "\n# changed after seal\n"
                    session.config.write_text(changed)
                    if fault != "seal":
                        session.config_sha256 = hashlib.sha256(session.config.read_bytes()).hexdigest()
                before = session.config.read_bytes()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.configure_phase_plan_mutation(session, policy, report, role)
                self.assertEqual(session.config.read_bytes(), before)
                self.assertEqual(events, [])
                self.assertNotIn("mutation_debug_configuration", report)

    def test_mutation_debug_profile_rejects_unsealed_configure_result(self):
        with tempfile.TemporaryDirectory() as temporary:
            session = mutation_session(Path(temporary), [])
            session.configure = lambda _text: None
            report = {}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.configure_phase_plan_mutation(session, self.policy(), report, MODULE._phase_plan_role())
            self.assertNotIn("mutation_debug_configuration", report)

    def test_baseline_coverage_and_enumeration_keep_the_original_config(self):
        for lane in ("baseline", "coverage", "enumerate"):
            with self.subTest(lane=lane), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                events, report = [], {}
                session = mutation_session(work, events)
                before = session.config.read_bytes()
                policy, role = self.policy(), MODULE._phase_plan_role()
                identity = {"candidate.rs": "c" * 64}
                policy.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                              inputs_sha256={name: "d" * 64 for name in role.bound_inputs})
                def run(label, _command, _timeout, **_options):
                    if label == "baseline":
                        return native_log(policy["test_inventories"]["baseline"])
                    self.assertEqual(label, "coverage")
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename,
                         "summary": {"lines": {"count": 10, "covered": 9}}}
                        for filename in role.scope.select("all").values()]}]}))
                    return native_log(policy["test_inventories"]["coverage"])
                session.__dict__.update(product_identity=identity, source=work, work=work,
                    tools=work / "tools", run=run, finish=lambda: None)
                args = SimpleNamespace(subject="phase_plan", lane=lane, tool_admission_profile="full",
                                       output_dir=work / "output")
                with patch.object(MODULE, "read_phase_plan_policy", return_value=policy), \
                     patch.object(MODULE, "inventory_document", return_value=policy), \
                     patch.object(MODULE.PRODUCT, "sha256", return_value="d" * 64), \
                     patch.object(MODULE.COMMON, "Session", return_value=session), \
                     patch.object(MODULE, "_enumerate_conditioned"), \
                     patch.object(MODULE, "configure_phase_plan_mutation") as configure:
                    MODULE.execute_phase_plan(args, report)
                    configure.assert_not_called()
                self.assertEqual(session.config.read_bytes(), before)
                self.assertEqual(events, [])

    def test_external_registry_binds_actual_fields_borrows_and_legal_neighbors(self):
        examples = MODULE.phase_plan_contract_examples()
        self.assertEqual(sum(row["code"] is None for row in examples.values()), 1)
        self.assertEqual(sum(row["code"] is not None for row in examples.values()), 7)
        required = {"phase_plan_producer_lease": ("E0502", "producer"),
                    "phase_plan_live_proposal": ("E0505", "owner"),
                    "phase_plan_double_transfer": ("E0499", "retired"),
                    "phase_plan_retirement_escape": ("E0515", "retired"),
                    "phase_plan_baseline_literal": ("E0451", "rate"),
                    "phase_plan_profile_mutation": ("E0616", "phase_cap"),
                    "phase_plan_cursor_retarget": ("E0616", "cursor")}
        for name, (code, token) in required.items():
            row = examples[name]
            self.assertEqual(row["code"], code)
            self.assertIn(token, row["tokens"])
            self.assertIn(token, row["source"].splitlines()[row["line"] - 1])
            self.assertEqual(row["external_legal_neighbor"]["file"], "crates/time-core/tests/phase_plan.rs")
            self.assertTrue(row["external_legal_neighbor"]["test"])

    def test_contract_finalization_runs_after_actual_consumer_failure(self):
        policy, role, report, events = self.policy(), MODULE._phase_plan_role(), {}, []
        identity = {"candidate.rs": "c" * 64}
        policy.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                      inputs_sha256={name: "d" * 64 for name in role.bound_inputs})
        session = SimpleNamespace(product_identity=identity, source=Path("/owned/source"),
                                  finish=lambda: events.append("finish"))
        args = SimpleNamespace(subject="phase_plan", lane="contracts", tool_admission_profile="full")
        with patch.object(MODULE, "read_phase_plan_policy", return_value=policy), \
             patch.object(MODULE, "inventory_document", return_value=policy), \
             patch.object(MODULE.PRODUCT, "sha256", return_value="d" * 64), \
             patch.object(MODULE.COMMON, "Session", return_value=session), \
             patch.object(MODULE, "phase_plan_contracts", side_effect=MODULE.Incomplete("intended cause missing")), \
             patch.object(MODULE, "configure_phase_plan_mutation") as configure:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.execute_phase_plan(args, report)
            self.assertFalse(report["qualification_passed"])
            self.assertEqual(events, ["finish"])
            configure.assert_not_called()


if __name__ == "__main__":
    unittest.main()
