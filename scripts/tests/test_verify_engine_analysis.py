"""Independent engine analysis admission controls; no compiler/analyzer execution."""
from __future__ import annotations

import copy
from contextlib import contextmanager
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import re
import shutil
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("engine_analysis_test", ROOT / "scripts/verify-engine-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def independent_digest(value):
    """Bind independently declared fixture values using the documented JSON encoding."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def literal_execution(*, doctests):
    """Literal native libtest records, independent of the runner's required-name set."""
    text = """running 59 tests
test processing_guard_tests::processing_uses_fresh_deadline_and_current_control ... ok
test processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct ... ok
test processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries ... ok
test processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays ... ok
test processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap ... ok
test processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections ... ok
test processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions ... ok
test processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor ... ok
test processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes ... ok
test processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port ... ok
test processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct ... ok
test processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history ... ok
test processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four ... ok
test processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize ... ok
test processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor ... ok
test processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error ... ok
test processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice ... ok
test processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection ... ok
test processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates ... ok
test processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once ... ok
test measurement::tests::exact_dyadic_results_ignore_the_wire_token ... ok
test measurement::tests::peer_shift_changes_sign_without_changing_rtt ... ok
test measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved ... ok
test measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values ... ok
test measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals ... ok
test measurement::tests::absent_profile_is_a_distinct_transport_only_reason ... ok
test measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor ... ok
test measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors ... ok
test measurement::tests::reversed_brackets_and_processing_are_not_ages ... ok
test measurement::tests::out_of_scope_local_coordinate_is_not_rebased ... ok
test measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints ... ok
test measurement::tests::era_zero_and_multiple_candidates_remain_distinct ... ok
test measurement::tests::inert_header_does_not_bypass_the_time_reply_guard ... ok
test measurement::tests::profile_bounds_and_maximum_allowance_are_checked ... ok
test measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic ... ok
test measurement::tests::refusal_codes_and_display_remain_literal_and_redacted ... ok
test measurement::tests::software_window_retains_each_local_and_peer_extremum ... ok
test measurement::tests::coherence_helper_rejects_each_reversed_bracket ... ok
test measurement::tests::decimal_remainders_change_the_exact_nominal_phase ... ok
test measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values ... ok
test measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association ... ok
test measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result ... ok
test measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds ... ok
test measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection ... ok
test measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt ... ok
test measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint ... ok
test processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals ... ok
test measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks ... ok
test measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding ... ok
test measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign ... ok
test relative::tests::midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints ... ok
test relative::tests::midpoint_publication_enforces_raw_association_and_garde_rule ... ok
test relative::tests::duration_nanoseconds_admits_full_u64_and_rejects_next_tick ... ok
test relative::tests::reference_instants_check_order_before_duration_conversion ... ok
test relative::tests::relative_reference_differences_keep_direction_and_half_ticks ... ok
test relative::tests::relative_phase_raw_i128_endpoints_keep_u128_magnitude ... ok
test relative::tests::shared_origin_distinguishes_equal_values_from_run_identity ... ok
test relative::tests::relative_seconds_scale_requires_nonzero_with_healthy_neighbors ... ok
test relative::tests::relative_diagnostics_preserve_fixed_text_and_original_causes ... ok
test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
running 4 tests
test argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure ... ok
test argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows ... ok
test argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles ... ok
test argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 5 tests
test prepare_outside_runtime_owns_and_releases_standard_sockets ... ok
test prepare_rejects_entered_runtime_before_native_startup ... ok
test execute_rejects_entered_runtime_without_nested_runtime_creation ... ok
test cli_worker_binary_preserves_literal_transport_contracts ... ok
test cli_worker_binary_rejects_arguments_and_observes_sink_failures ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s
"""
    if doctests:
        text += """   Doc-tests nevrotick_engine
running 33 tests
test crates/nevrotick-engine/src/lib.rs - Config::new (line 168) ... ok
test crates/nevrotick-engine/src/lib.rs - ControlSchedule::new (line 1238) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 91) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 101) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 114) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 123) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalProfile (line 128) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 213) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 219) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 226) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 234) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::ExperimentalMeasurement (line 239) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 600) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 610) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 620) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 630) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 640) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 660) ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 670) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 680) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 690) - compile fail ... ok
test crates/nevrotick-engine/src/measurement.rs - measurement::NominalRoundTripNs (line 700) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 90) ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 101) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 106) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 111) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalReferenceNs (line 116) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 155) ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 169) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 185) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 192) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 197) - compile fail ... ok
test crates/nevrotick-engine/src/relative.rs - relative::ExperimentalRelativeMeasurement (line 204) - compile fail ... ok
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"""
    return text


def literal_inventories():
    """Synthetic listing controls are not actual product mutant observations."""
    result = {}
    for subject, filename in [
        ("library", "crates/nevrotick-engine/src/lib.rs"),
        ("cli", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs"),
        ("measurement", "crates/nevrotick-engine/src/measurement.rs"),
        ("relative", "crates/nevrotick-engine/src/relative.rs"),
    ]:
        name = subject + " synthetic fault"
        result[subject] = {name: {
            "name": name, "package": "nevrotick-engine", "file": filename,
            "function": {"function_name": "fixture_only", "return_type": "bool",
                         "span": {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 1}}},
            "span": {"start": {"line": 1, "column": 2}, "end": {"line": 1, "column": 4}},
            "replacement": "!=", "genre": "BinaryOperator",
        }}
    return result


def contract_artifact_messages(session, libraries):
    """Declare synthetic exact local units, independent of collector discovery."""
    session.source = session.work / "source"
    session.source.mkdir(exist_ok=True)
    session.vendor = session.work / "vendor"; session.vendor.mkdir(exist_ok=True)
    lock = "version=4\n"; records = []
    for name, artifact in libraries.items():
        manifest = session.source / "crates" / name / "Cargo.toml"
        manifest.parent.mkdir(parents=True, exist_ok=True)
        manifest.write_text('[package]\nname="' + name + '"\nversion="0.0.0"\n')
        lock += '[[package]]\nname="' + name + '"\nversion="0.0.0"\n'
        virtual = "/work/source/crates/" + name
        metadata_root = session.work / "target/debug/deps"; metadata_root.mkdir(parents=True, exist_ok=True)
        meta = metadata_root / ("lib" + name.replace("-", "_") + "-" + "1" * 16 + ".rmeta")
        meta.write_bytes(b"independent metadata fixture")
        records.append({"reason": "compiler-artifact", "manifest_path": virtual + "/Cargo.toml",
            "package_id": "path+file://" + virtual + "#0.0.0",
            "target": {"name": name.replace("-", "_"), "kind": ["lib"], "crate_types": ["lib"], "src_path": virtual + "/src/lib.rs"},
            "profile": {"test": False, "opt_level": "0", "debug_assertions": True, "overflow_checks": True},
            "filenames": ["/work/" + str(item.relative_to(session.work)) for item in (artifact, meta)]})
    (session.source / "Cargo.lock").write_text(lock)
    return "\n".join(map(json.dumps, records))


class EngineAnalysisTests(unittest.TestCase):
    """Healthy neighbors discriminate scope, candidate, listing and baseline gates."""

    def test_nominal_type_doctests_cannot_be_omitted_with_consistent_summaries(self):
        healthy = literal_execution(doctests=True)
        for owner in ("NominalPhaseNs", "NominalRoundTripNs"):
            line = next(row for row in healthy.splitlines()
                        if "measurement::" + owner + " " in row and "compile fail" in row)
            bad = healthy.replace(line + "\n", "", 1).replace("ok. 33 passed;", "ok. 32 passed;")
            with self.subTest(owner=owner), self.assertRaisesRegex(MODULE.Incomplete, "construction/privacy"):
                MODULE.validate_test_execution(bad, doctests=True)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 33)

    def test_four_source_owners_have_exact_selections_and_command_targets(self):
        expected = {"library": "crates/nevrotick-engine/src/lib.rs",
                    "cli": "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
                    "measurement": "crates/nevrotick-engine/src/measurement.rs",
                    "relative": "crates/nevrotick-engine/src/relative.rs"}
        self.assertEqual(MODULE.SCOPE.select("all"), expected)
        for subject, filename in expected.items():
            self.assertEqual(MODULE.SCOPE.select(subject), {subject: filename})
            command = MODULE.mutation_prefix(Path("/owned/tools"), subject, MODULE.FIXED_POLICY)
            self.assertEqual(command[command.index("--file") + 1], filename)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.SCOPE.select("foreign")
        self.assertEqual(MODULE.CONTRACT_SCOPE.select("all"), {"library": expected["library"]})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.CONTRACT_SCOPE.select("measurement")

    def fixture(self, root):
        for name in (MODULE.POLICY, *MODULE.BOUND_INPUTS):
            destination = root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        identity = {"owned-fixture-input.rs": "a" * 64}
        policy = json.loads((root / MODULE.POLICY).read_text())
        policy["candidate_sha256"] = independent_digest(identity)
        policy["mutation_inventories"] = {}  # This literal fixture models the held state.
        policy["mutation_equivalences"] = []
        policy["inputs_sha256"] = {
            name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in MODULE.BOUND_INPUTS}
        self.write_policy(root, policy)
        return policy, identity

    def write_policy(self, root, policy):
        (root / MODULE.POLICY).write_text(json.dumps(policy))

    def admit_fixture(self, root, identity):
        with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
            return MODULE.read_policy(root)

    def test_new_analysis_files_are_outside_the_current_product_census(self):
        _, identity = MODULE.PRODUCT.admit_product(ROOT)
        # This test checks census membership. Exact cohort counts belong to each
        # profile's admission, which must refuse unrelated concurrent source drift.
        self.assertNotIn(MODULE.QUALITY_POLICY, identity)
        self.assertIn("scripts/tests/test_verify_vendor_admission.py", identity)
        self.assertIn("crates/time-core/src/window_variance.rs", identity)
        self.assertIn("crates/time-core/tests/window_variance.rs", identity)
        self.assertIn("crates/nevrotick-engine/src/measurement.rs", identity)
        self.assertIn("crates/nevrotick-engine/src/relative.rs", identity)
        self.assertIn("crates/nevrotick-engine/tests/private/relative.rs", identity)
        self.assertIn("crates/nevrotick-engine/src/series.rs", identity)
        self.assertIn("crates/nevrotick-engine/tests/private/series.rs", identity)
        self.assertIn("crates/nevrotick-engine/tests/series.rs", identity)
        for name in ["scripts/verify-engine-analysis.py", "scripts/tests/test_verify_engine_analysis.py",
                     "policy/engine-analysis.json", MODULE.SERIES_POLICY, MODULE.SERIES_TESTS]:
            self.assertNotIn(name, identity)

    def test_healthy_policy_is_full_profile_and_campaign_initially_held(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); _, identity = self.fixture(root)
            policy = self.admit_fixture(root, identity)
        self.assertEqual(policy["critical_files"], [
            "crates/nevrotick-engine/src/lib.rs", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
            "crates/nevrotick-engine/src/measurement.rs", "crates/nevrotick-engine/src/relative.rs"])
        self.assertEqual(policy["mutation_inventories"], {})
        self.assertEqual(policy["mutation_equivalences"], [])
        self.assertEqual(policy["tool_admission_profile"], "full")
        self.assertEqual(policy["mutation_jobs"], 10)

    def equivalence_fixture(self, root):
        """Use real product/source admission with synthetic, explicitly scoped E controls."""
        _, current = MODULE.PRODUCT.admit_product(ROOT)
        for name in sorted(set(current) | set(MODULE.BOUND_INPUTS) | {MODULE.POLICY}):
            target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        _, identity = MODULE.PRODUCT.admit_product(root)
        policy = json.loads((root / MODULE.POLICY).read_text())
        policy["candidate_sha256"] = independent_digest(identity)
        policy["inputs_sha256"] = {
            name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in MODULE.BOUND_INPUTS}
        policy["mutation_inventories"] = {}
        filename = "crates/nevrotick-engine/src/measurement.rs"
        lines = (root / filename).read_text().splitlines()
        start = next(index for index, line in enumerate(lines, 1) if line.startswith("fn coherence_residual("))
        end = next(index for index, line in enumerate(lines, 1) if index > start and line == "}")
        cases = [("ENGINE-COHERENCE-LOWER-EQUALITY", "<", "<=", "wall_delta < lower"),
                 ("ENGINE-COHERENCE-UPPER-EQUALITY", ">", ">=", "wall_delta > upper")]
        proof = root / "tasks/engine-analysis-equivalences.md"; proof.parent.mkdir(exist_ok=True)
        proof.write_text("# Synthetic admission controls, not product qualification\n" + "".join(
            "\n## " + identifier + "\nSynthetic review-section fixture.\n" for identifier, *_ in cases))
        entries = []
        for identifier, original, replacement, expression in cases:
            line = next(index for index, text in enumerate(lines, 1) if expression in text)
            column = lines[line - 1].index(original) + 1
            mutant = {
                "name": f"{filename}:{line}:{column}: replace {original} with {replacement} in coherence_residual",
                "package": "nevrotick-engine", "file": filename,
                "function": {"function_name": "coherence_residual", "return_type": "-> Result<u64, MeasurementRefusal>",
                             "span": {"start": {"line": start, "column": 1}, "end": {"line": end, "column": 2}}},
                "span": {"start": {"line": line, "column": column}, "end": {"line": line, "column": column + 1}},
                "replacement": replacement, "genre": "BinaryOperator"}
            entries.append({"id": identifier, "candidate_sha256": policy["candidate_sha256"],
                            "source_sha256": identity[filename], "mutant": mutant,
                            "requirement": "CHECK-7/8/25", "rationale": "Synthetic exact-custody admission control.",
                            "proof": {"path": "tasks/engine-analysis-equivalences.md",
                                      "sha256": hashlib.sha256(proof.read_bytes()).hexdigest(),
                                      "section": identifier, "reviewer": "independent synthetic control",
                                      "verdict": "independently-justified-equivalent"}})
        policy["mutation_equivalences"] = entries
        self.write_policy(root, policy)
        return policy, proof

    def test_exact_engine_equivalences_bind_real_product_source_and_shared_review_custody(self):
        """No product or equivalence-admission mocks stand in for the healthy control."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, _ = self.equivalence_fixture(root)
            self.assertEqual(MODULE.PRODUCT.admit_product(root)[1],
                             MODULE.PRODUCT.admit_product(ROOT)[1])
            self.assertEqual(MODULE.read_policy(root), policy)
            self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SCOPE, policy),
                             policy["mutation_equivalences"])
            policy["mutation_equivalences"] = []
            self.write_policy(root, policy)
            self.assertEqual(MODULE.read_policy(root), policy)

    def test_engine_equivalences_reject_partial_foreign_duplicate_stale_and_malformed_reviews(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, _ = self.equivalence_fixture(root)
            for change in ("missing", "foreign", "duplicate", "owner", "function", "path", "operator", "genre",
                           "candidate", "source", "proof", "section", "reviewer", "verdict", "descriptor",
                           "unknown", "id_type", "list_type", "missing_key"):
                bad = copy.deepcopy(healthy); entries = bad["mutation_equivalences"]; case = entries[0]
                if change == "missing": entries.pop()
                if change == "foreign": case["id"] = "FOREIGN-EQUALITY"
                if change == "duplicate": entries[1] = copy.deepcopy(case)
                if change == "owner": case["mutant"]["file"] = "crates/nevrotick-engine/src/lib.rs"
                if change == "function": case["mutant"]["function"]["function_name"] = "measure"
                if change == "path": case["proof"]["path"] = "tasks/foreign.md"
                if change == "operator": case["mutant"]["replacement"] = ">="
                if change == "genre": case["mutant"]["genre"] = "FnValue"
                if change == "candidate": case["candidate_sha256"] = "0" * 64
                if change == "source": case["source_sha256"] = "0" * 64
                if change == "proof": case["proof"]["sha256"] = "0" * 64
                if change == "section": case["proof"]["section"] = "UNREVIEWED"
                if change == "reviewer": case["proof"]["reviewer"] = ""
                if change == "verdict": case["proof"]["verdict"] = "proposed"
                if change == "descriptor": case["mutant"].pop("span")
                if change == "unknown": case["skip_function"] = "coherence_residual"
                if change == "id_type": case["id"] = []
                if change == "list_type": bad["mutation_equivalences"] = {}
                if change == "missing_key": bad.pop("mutation_equivalences")
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root)
            self.write_policy(root, healthy)
            self.assertEqual(MODULE.read_policy(root), healthy)

    def test_zero_sign_equivalence_scope_is_an_exact_optional_third_case(self):
        """Synthetic scope controls; actual source/review custody is checked separately."""
        entries = [
            {"id": identifier, "mutant": {
                "file": "crates/nevrotick-engine/src/measurement.rs",
                "function": {"function_name": owner}, "genre": "BinaryOperator",
                "replacement": replacement},
             "proof": {"path": "tasks/engine-analysis-equivalences.md"}}
            for identifier, owner, replacement in [
                ("ENGINE-COHERENCE-LOWER-EQUALITY", "coherence_residual", "<="),
                ("ENGINE-COHERENCE-UPPER-EQUALITY", "coherence_residual", ">="),
                ("ENGINE-NOMINAL-ZERO-SIGN", "nominal_seconds", "<="),
            ]
        ]
        MODULE.validate_equivalence_scope([])
        MODULE.validate_equivalence_scope(entries[:2])
        MODULE.validate_equivalence_scope(entries)
        for change in ("zero_only", "missing_lower", "missing_upper", "duplicate", "foreign_id",
                       "function", "file", "operator", "genre", "proof_path"):
            bad = copy.deepcopy(entries)
            if change == "zero_only": bad = bad[2:]
            if change == "missing_lower": bad.pop(0)
            if change == "missing_upper": bad.pop(1)
            if change == "duplicate": bad.append(copy.deepcopy(bad[2]))
            if change == "foreign_id": bad[2]["id"] = "ENGINE-UNREVIEWED-ZERO"
            if change == "function": bad[2]["mutant"]["function"]["function_name"] = "coherence_residual"
            if change == "file": bad[2]["mutant"]["file"] = "crates/nevrotick-engine/src/lib.rs"
            if change == "operator": bad[2]["mutant"]["replacement"] = ">="
            if change == "genre": bad[2]["mutant"]["genre"] = "FnValue"
            if change == "proof_path": bad[2]["proof"]["path"] = "tasks/foreign.md"
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_equivalence_scope(bad)

    @staticmethod
    def commit_fallback_entries():
        """Literal native descriptor identities; not a semantic-equivalence oracle."""
        filename = "crates/nevrotick-engine/src/lib.rs"
        entries = []
        for identifier, replacement in (
            ("ENGINE-COMMIT-DURATION-FALLBACK-SUB", "-="),
            ("ENGINE-COMMIT-DURATION-FALLBACK-MUL", "*="),
        ):
            entries.append({"id": identifier,
                "source_sha256": "6f3c6eb2f491de7f27a89f8fba84556e818874a9c875643b08081206f5dd4d46",
                "mutant": {
                "name": f"{filename}:1335:23: replace += with {replacement} in commit_reply",
                "package": "nevrotick-engine", "file": filename,
                "function": {"function_name": "commit_reply", "return_type": "",
                             "span": {"start": {"line": 1315, "column": 1},
                                      "end": {"line": 1364, "column": 2}}},
                "span": {"start": {"line": 1335, "column": 23},
                         "end": {"line": 1335, "column": 25}},
                "replacement": replacement, "genre": "BinaryOperator"},
                "proof": {"path": "tasks/engine-analysis-equivalences.md"}})
        return entries

    def test_commit_fallback_scope_requires_complete_exact_native_pair(self):
        entries = self.commit_fallback_entries()
        failure = None
        try:
            MODULE.validate_equivalence_scope(entries)
        except MODULE.Incomplete as error:
            failure = str(error)
        self.assertIsNone(failure, "Both exact reviewed fallback descriptors must be admitted.")
        for change in ("single", "duplicate", "foreign_id", "function", "file", "operator",
                       "genre", "proof_path", "reachable_counter", "function_span", "source", "mixed"):
            bad = copy.deepcopy(entries)
            if change == "single": bad.pop()
            elif change == "duplicate": bad.append(copy.deepcopy(bad[0]))
            elif change == "foreign_id": bad[0]["id"] = "ENGINE-UNREVIEWED-FALLBACK"
            elif change == "function": bad[0]["mutant"]["function"]["function_name"] = "stage_reply"
            elif change == "file": bad[0]["mutant"]["file"] = "crates/nevrotick-engine/src/measurement.rs"
            elif change == "operator": bad[0]["mutant"]["replacement"] = "+="
            elif change == "genre": bad[0]["mutant"]["genre"] = "FnValue"
            elif change == "proof_path": bad[0]["proof"]["path"] = "tasks/foreign.md"
            elif change == "reachable_counter":
                mutant = bad[0]["mutant"]
                mutant["name"] = mutant["name"].replace(":1335:", ":1331:")
                mutant["span"]["start"]["line"] = 1331
                mutant["span"]["end"]["line"] = 1331
            elif change == "function_span": bad[0]["mutant"]["function"]["span"]["end"]["line"] += 1
            elif change == "source": bad[0]["source_sha256"] = "a" * 64
            else:
                bad.append({"id": "ENGINE-COHERENCE-LOWER-EQUALITY"})
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_equivalence_scope(bad)

    def test_commit_fallback_review_uses_actual_product_and_shared_custody(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy, proof = self.equivalence_fixture(root)
            entries = self.commit_fallback_entries()
            proof.write_text("# Synthetic fallback admission controls, not product qualification\n" +
                             "".join("\n## " + item["id"] + "\nSynthetic review fixture.\n" for item in entries))
            for entry in entries:
                entry.update(candidate_sha256=policy["candidate_sha256"],
                             source_sha256=MODULE.PRODUCT.sha256(root / entry["mutant"]["file"]),
                             requirement="CHECK-6/25/27",
                             rationale="Synthetic exact native-descriptor custody control.")
                entry["proof"].update(sha256=MODULE.PRODUCT.sha256(proof), section=entry["id"],
                                      reviewer="independent synthetic control",
                                      verdict="independently-justified-equivalent")
            policy["mutation_equivalences"] = entries
            self.write_policy(root, policy)
            historical_source = self.commit_fallback_entries()[0]["source_sha256"]
            if entries[0]["source_sha256"] == historical_source:
                self.assertEqual(MODULE.read_policy(root), policy)
                self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SCOPE, policy), entries)
            else:
                # A copied current tree is not the historical reviewed library.
                # Relabelling its source/candidate cannot reuse old native spans.
                with self.assertRaisesRegex(MODULE.Incomplete, "exact source and native descriptor"):
                    MODULE.read_policy(root)

    def test_engine_equivalence_review_bytes_and_missing_sections_cannot_drift(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, proof = self.equivalence_fixture(root)
            healthy = copy.deepcopy(policy)
            original = proof.read_text()
            proof.write_text(original + "Changed after independent review.\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                MODULE.read_policy(root)
            proof.write_text("# No reviewed sections\n")
            for entry in policy["mutation_equivalences"]:
                entry["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
            self.write_policy(root, policy)
            with self.assertRaisesRegex(MODULE.Incomplete, "exact section"):
                MODULE.read_policy(root)
            proof.unlink()
            with self.assertRaises(FileNotFoundError):
                MODULE.read_policy(root)
            proof.write_text(original)
            self.write_policy(root, healthy)
            self.assertEqual(MODULE.read_policy(root), healthy)

    def test_wrong_scope_threshold_budget_type_profile_and_equivalence_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            changes = [("package", "time-core"), ("critical_files", healthy["critical_files"][:1]),
                       ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                       ("maximum_mutants", 1), ("jobs", True), ("coverage_test_threads", 12),
                       ("tool_admission_profile", "stable-core"), ("mutation_jobs", 1),
                       ("mutation_equivalences", [{}]), ("unknown", True)]
            for key, value in changes:
                bad = copy.deepcopy(healthy); bad[key] = value; self.write_policy(root, bad)
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    self.admit_fixture(root, identity)

    def test_measurement_and_relative_scope_reject_missing_extra_and_redirected_selection(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            for filename in ("crates/nevrotick-engine/src/measurement.rs",
                             "crates/nevrotick-engine/src/relative.rs"):
                for change in ("missing", "extra", "redirected"):
                    bad = copy.deepcopy(healthy)
                    if change == "missing": bad["critical_files"].remove(filename)
                    elif change == "extra": bad["critical_files"].append("crates/nevrotick-engine/src/foreign.rs")
                    else: bad["critical_files"][bad["critical_files"].index(filename)] = "crates/nevrotick-engine/src/foreign.rs"
                    self.write_policy(root, bad)
                    with self.subTest(filename=filename, change=change), self.assertRaisesRegex(MODULE.Incomplete, "scope"):
                        self.admit_fixture(root, identity)

    def test_candidate_drift_and_refreshed_local_pins_do_not_evade_whole_candidate(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            with self.assertRaisesRegex(MODULE.Incomplete, "complete current product candidate"):
                self.admit_fixture(root, {**identity, "new-input.rs": "b" * 64})
            bad = copy.deepcopy(healthy); bad["candidate_sha256"] = "0" * 64; self.write_policy(root, bad)
            with self.assertRaisesRegex(MODULE.Incomplete, "complete current product candidate"):
                self.admit_fixture(root, identity)

    def test_protected_input_missing_extra_and_actual_content_drift_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            for change in ["missing", "extra", "content"]:
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["inputs_sha256"].pop("Cargo.lock")
                elif change == "extra": bad["inputs_sha256"]["not-admitted"] = "0" * 64
                else: (root / "Cargo.lock").write_text("changed fixture input")
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "bindings changed"):
                    self.admit_fixture(root, identity)

    def test_critical_owner_removal_rejects_even_with_refreshed_local_pin(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); policy, identity = self.fixture(root)
            path = root / "policy/verification-manifest.toml"
            path.write_text(path.read_text().replace('package = "nevrotick-engine"', 'package = "wrong"'))
            policy["inputs_sha256"]["policy/verification-manifest.toml"] = hashlib.sha256(path.read_bytes()).hexdigest()
            self.write_policy(root, policy)
            with self.assertRaisesRegex(MODULE.Incomplete, "matching critical/security owner"):
                self.admit_fixture(root, identity)

    def test_relative_owner_requires_one_critical_and_security_registration_after_repinning(self):
        """A fresh manifest pin cannot authorize an absent or ambiguous owner."""
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            self.assertEqual(self.admit_fixture(root, identity), healthy)
            path = root / "policy/verification-manifest.toml"
            original = path.read_text()
            critical = '    "crates/nevrotick-engine/src/relative.rs",\n'
            security = next(line + "\n" for line in original.splitlines()
                            if 'name = "engine-experimental-relative-measurement"' in line)
            self.assertEqual(original.count(critical), 1)
            for change in ("critical_missing", "critical_duplicate", "critical_redirected",
                           "security_missing", "security_duplicate", "security_redirected", "security_package"):
                if change == "critical_missing": changed = original.replace(critical, "", 1)
                elif change == "critical_duplicate": changed = original.replace(critical, critical * 2, 1)
                elif change == "critical_redirected": changed = original.replace(critical, critical.replace("relative.rs", "foreign.rs"), 1)
                elif change == "security_missing": changed = original.replace(security, "", 1)
                elif change == "security_duplicate": changed = original.replace(security, security * 2, 1)
                elif change == "security_redirected": changed = original.replace(security, security.replace("relative.rs", "foreign.rs"), 1)
                else: changed = original.replace(security, security.replace('package = "nevrotick-engine"', 'package = "wrong"'), 1)
                path.write_text(changed)
                bad = copy.deepcopy(healthy)
                bad["inputs_sha256"]["policy/verification-manifest.toml"] = hashlib.sha256(path.read_bytes()).hexdigest()
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "matching critical/security owner"):
                    self.admit_fixture(root, identity)
            path.write_text(original)
            self.write_policy(root, healthy)
            self.assertEqual(self.admit_fixture(root, identity), healthy)

    def test_relative_private_tests_require_exact_protected_bytes_and_path(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy, identity = self.fixture(root)
            name = "crates/nevrotick-engine/tests/private/relative.rs"
            self.assertIn(name, healthy["inputs_sha256"])
            path = root / name; original = path.read_bytes()
            for change in ("missing", "redirected", "content"):
                bad = copy.deepcopy(healthy)
                if change == "missing": bad["inputs_sha256"].pop(name)
                elif change == "redirected": bad["inputs_sha256"][name.replace("relative.rs", "foreign.rs")] = bad["inputs_sha256"].pop(name)
                else: path.write_bytes(original + b"\n// changed protected test bytes\n")
                self.write_policy(root, bad)
                with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "bindings changed"):
                    self.admit_fixture(root, identity)
                path.write_bytes(original)
            self.write_policy(root, healthy)
            self.assertEqual(self.admit_fixture(root, identity), healthy)

    def test_json_rejects_duplicates_nonfinite_overflow_link_and_size(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); path = root / "input.json"
            for text in ['{"x":1,"x":2}', '{"x":{"x":1,"x":2}}', '{"x":NaN}', '{"x":1e999}']:
                path.write_text(text)
                with self.subTest(text=text), self.assertRaises(MODULE.Incomplete): MODULE.read_json(path)
            path.write_text('{"duration":0.25}')
            self.assertEqual(MODULE.read_json(path), {"duration": 0.25})
            with self.assertRaises(MODULE.Incomplete): MODULE.read_json(path, maximum=2)
            linked = root / "linked.json"; linked.symlink_to(path)
            with self.assertRaises(MODULE.Incomplete): MODULE.read_json(linked)

    def test_campaign_bindings_must_cover_four_nonempty_exact_inventories(self):
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        for bindings in [{}, {"library": {"count": 1, "sha256": "a" * 64}},
                         {name: {"count": count, "sha256": "a" * 64}
                          for name, count in [("library", 1), ("cli", 1), ("measurement", 0), ("relative", 1)]},
                         {name: {"count": True, "sha256": "a" * 64}
                          for name in ["library", "cli", "measurement", "relative"]},
                         {name: {"count": 1, "sha256": "not-a-hash"}
                          for name in ["library", "cli", "measurement", "relative"]}]:
            policy["mutation_inventories"] = bindings
            with self.subTest(bindings=bindings), self.assertRaises(MODULE.Incomplete):
                MODULE.inventory_bindings(policy, required=True)
        healthy = {name: {"count": 1, "sha256": "a" * 64} for name in ["library", "cli", "measurement", "relative"]}
        policy["mutation_inventories"] = healthy
        self.assertEqual(len(MODULE.inventory_bindings(policy, required=True)), 4)
        for change in ("missing", "missing_relative", "extra", "redirected"):
            bad = copy.deepcopy(healthy)
            if change == "missing": bad.pop("measurement")
            elif change == "missing_relative": bad.pop("relative")
            elif change == "extra": bad["foreign"] = {"count": 1, "sha256": "a" * 64}
            else: bad["redirected"] = bad.pop("measurement")
            policy["mutation_inventories"] = bad
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.inventory_bindings(policy, required=True)

    def test_fresh_descriptor_metadata_must_match_review_while_rendered_diff_is_ignored(self):
        observed = literal_inventories()
        policy = copy.deepcopy(MODULE.FIXED_POLICY)
        policy["mutation_inventories"] = {
            name: {"count": 1, "sha256": independent_digest(items)} for name, items in observed.items()}
        self.assertEqual(set(MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, observed)),
                         {"library", "cli", "measurement", "relative"})
        changed = copy.deepcopy(observed); changed["library"]["library synthetic fault"]["diff"] = "rendering only"
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, changed)
        changed["library"]["library synthetic fault"]["function"]["return_type"] = "u8"
        with self.assertRaisesRegex(MODULE.Incomplete, "differs from its reviewed"):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.SCOPE, policy, changed)

    def test_empty_and_foreign_actual_listing_are_rejected(self):
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_inventory(MODULE.SCOPE, [], 4096, MODULE.SUBJECTS["library"])
        item = next(iter(literal_inventories()["cli"].values()))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_inventory(MODULE.SCOPE, [item], 4096, MODULE.SUBJECTS["library"])

    def test_baseline_has_healthy_coverage_and_full_cargo_neighbors(self):
        self.assertEqual(MODULE.validate_test_execution(literal_execution(doctests=False), doctests=False)
                         ["tests_executed"], 68)
        full = MODULE.validate_test_execution(literal_execution(doctests=True), doctests=True)
        self.assertEqual((full["tests_executed"], full["doctests"]), (101, 33))

    def test_relative_bridge_cases_reject_individual_omissions_with_consistent_counts(self):
        """Independent literal cases require the real bridge and each numeric boundary."""
        healthy = literal_execution(doctests=False)
        required = (
            "processing_guard_tests::public_execute_relative_view_retains_actual_common_run_and_refusals",
            "measurement::tests::relative_phase_direction_preserves_checked_nominal_ticks",
            "measurement::tests::relative_phase_anchoring_precedes_large_absolute_rounding",
            "measurement::tests::relative_phase_difference_exceeds_i128_without_losing_sign",
            "relative::tests::midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints",
            "relative::tests::midpoint_publication_enforces_raw_association_and_garde_rule",
            "relative::tests::duration_nanoseconds_admits_full_u64_and_rejects_next_tick",
            "relative::tests::reference_instants_check_order_before_duration_conversion",
            "relative::tests::relative_reference_differences_keep_direction_and_half_ticks",
            "relative::tests::relative_phase_raw_i128_endpoints_keep_u128_magnitude",
            "relative::tests::shared_origin_distinguishes_equal_values_from_run_identity",
            "relative::tests::relative_seconds_scale_requires_nonzero_with_healthy_neighbors",
            "relative::tests::relative_diagnostics_preserve_fixed_text_and_original_causes",
        )
        self.assertEqual(len(required), 13)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
        for name in required:
            record = "test " + name + " ... ok\n"
            self.assertEqual(healthy.count(record), 1)
            bad = healthy.replace(record, "", 1).replace("running 59 tests", "running 58 tests", 1)
            bad = bad.replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaisesRegex(MODULE.Incomplete, "required actual"):
                MODULE.validate_test_execution(bad, doctests=False)

    def test_relative_doctests_require_both_legal_views_and_all_nine_privacy_failures(self):
        """Missing cases still fail when the native doctest counts agree with the log."""
        healthy = literal_execution(doctests=True)
        docs = [line for line in healthy.splitlines()
                if line.startswith("test crates/nevrotick-engine/src/relative.rs - ")]
        self.assertEqual(len(docs), 11)
        self.assertEqual(sum("relative::ExperimentalReferenceNs " in line and "compile fail" not in line
                             for line in docs), 1)
        self.assertEqual(sum("relative::ExperimentalReferenceNs " in line and "compile fail" in line
                             for line in docs), 4)
        self.assertEqual(sum("relative::ExperimentalRelativeMeasurement " in line and "compile fail" not in line
                             for line in docs), 1)
        self.assertEqual(sum("relative::ExperimentalRelativeMeasurement " in line and "compile fail" in line
                             for line in docs), 5)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 33)
        for line in docs:
            bad = healthy.replace(line + "\n", "", 1).replace("running 33 tests", "running 32 tests", 1)
            bad = bad.replace("33 passed;", "32 passed;", 1)
            with self.subTest(omitted=line), self.assertRaisesRegex(MODULE.Incomplete, "construction/privacy"):
                MODULE.validate_test_execution(bad, doctests=True)
        for change, before, after in (
            ("file", "src/relative.rs - relative::ExperimentalReferenceNs",
             "src/lib.rs - relative::ExperimentalReferenceNs"),
            ("owner", "relative::ExperimentalRelativeMeasurement (line 204)",
             "relative::ExperimentalReferenceNs (line 204)"),
            ("compile_failure", "relative::ExperimentalRelativeMeasurement (line 204) - compile fail",
             "relative::ExperimentalRelativeMeasurement (line 204)"),
        ):
            self.assertEqual(healthy.count(before), 1 if change != "file" else 5)
            bad = healthy.replace(before, after, 1)
            with self.subTest(change=change), self.assertRaisesRegex(MODULE.Incomplete, "construction/privacy"):
                MODULE.validate_test_execution(bad, doctests=True)

    def test_constructor_and_guard_alternatives_cannot_be_omitted_from_baseline(self):
        """A healthy full transcript discriminates the earlier seven-case baseline."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
        added = (
            "processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries",
            "processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays",
            "processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap",
            "processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections",
            "processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions",
            "processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor",
        )
        for name in added:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_argument_budget_cases_cannot_be_omitted_from_baseline(self):
        """Checked resource owners require their actual native unit records."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
        for name in (
            "argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure",
            "argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows",
            "argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles",
            "argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries",
        ):
            text = healthy.replace("test " + name + " ... ok\n", "").replace("4 passed;", "3 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_measurement_and_owner_cases_cannot_be_omitted_from_baseline(self):
        """A counted healthy transcript rejects every new semantic/ownership omission."""
        healthy = literal_execution(doctests=False)
        required = (
            "processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice",
            "processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection",
            "processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates",
            "processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once",
            "measurement::tests::exact_dyadic_results_ignore_the_wire_token",
            "measurement::tests::peer_shift_changes_sign_without_changing_rtt",
            "measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved",
            "measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values",
            "measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals",
            "measurement::tests::absent_profile_is_a_distinct_transport_only_reason",
            "measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor",
            "measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors",
            "measurement::tests::reversed_brackets_and_processing_are_not_ages",
            "measurement::tests::out_of_scope_local_coordinate_is_not_rebased",
            "measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints",
            "measurement::tests::era_zero_and_multiple_candidates_remain_distinct",
            "measurement::tests::inert_header_does_not_bypass_the_time_reply_guard",
            "measurement::tests::profile_bounds_and_maximum_allowance_are_checked",
            "measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic",
            "measurement::tests::refusal_codes_and_display_remain_literal_and_redacted",
            "measurement::tests::software_window_retains_each_local_and_peer_extremum",
            "measurement::tests::coherence_helper_rejects_each_reversed_bracket",
            "measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt",
            "measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint",
        )
        for name in required:
            text = healthy.replace("test " + name + " ... ok\n", "").replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaisesRegex(MODULE.Incomplete, "required actual"):
                MODULE.validate_test_execution(text, doctests=False)

    def test_measurement_doctests_require_success_and_all_private_construction_failures(self):
        healthy = literal_execution(doctests=True)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=True)["doctests"], 33)
        docs = [line for line in healthy.splitlines() if line.startswith("test crates/")]
        for line in docs:
            text = healthy.replace(line + "\n", "").replace("33 passed;", "32 passed;", 1)
            with self.subTest(omitted=line), self.assertRaisesRegex(MODULE.Incomplete, "doctest"):
                MODULE.validate_test_execution(text, doctests=True)
        for text in (
            healthy.replace("measurement::ExperimentalProfile (line 101) - compile fail",
                            "measurement::ExperimentalProfile (line 101)"),
            healthy.replace("src/measurement.rs - measurement::ExperimentalMeasurement",
                            "src/lib.rs - measurement::ExperimentalMeasurement", 1),
            healthy.replace("measurement::ExperimentalProfile (line 128)", "unrelated::Example (line 128)"),
        ):
            with self.subTest(text=text[-500:]), self.assertRaisesRegex(MODULE.Incomplete, "doctest"):
                MODULE.validate_test_execution(text, doctests=True)
        with self.assertRaisesRegex(MODULE.Incomplete, "unexpectedly includes doctests"):
            MODULE.validate_test_execution(healthy, doctests=False)

    def test_library_native_result_alternatives_cannot_be_omitted(self):
        """Every newly required admission/completion owner has a native record."""
        healthy = literal_execution(doctests=False)
        self.assertEqual(MODULE.validate_test_execution(healthy, doctests=False)["tests_executed"], 68)
        for name in (
            "processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes",
            "processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port",
            "processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct",
            "processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history",
            "processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four",
            "processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize",
            "processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor",
            "processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error",
        ):
            text = healthy.replace("test " + name + " ... ok\n", "").replace("59 passed;", "58 passed;", 1)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_old_runtime_only_baseline_and_printed_names_cannot_replace_execution(self):
        old = """running 3 tests
test prepare_outside_runtime_owns_and_releases_standard_sockets ... ok
test prepare_rejects_entered_runtime_before_native_startup ... ok
test execute_rejects_entered_runtime_without_nested_runtime_creation ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"""
        for text in [old, old + "\n".join(MODULE.REQUIRED_TESTS),
                     literal_execution(doctests=False).replace("59 passed;", "0 passed;", 1)]:
            with self.subTest(text=text[:40]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=False)

    def test_failed_skipped_filtered_or_missing_doctest_baselines_reject(self):
        healthy = literal_execution(doctests=True)
        for text in [healthy.replace(" ... ok", " ... ignored", 1),
                     healthy.replace("0 failed;", "1 failed;", 1),
                     healthy.replace("0 filtered out;", "1 filtered out;", 1),
                     literal_execution(doctests=False)]:
            with self.subTest(text=text[:40]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_test_execution(text, doctests=True)

    def test_actual_baseline_log_and_finite_native_timings_are_read(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); directory = work / "mutation/library/mutants.out"
            (directory / "log").mkdir(parents=True)
            (directory / "log/baseline.log").write_text(literal_execution(doctests=True))
            path = directory / "outcomes.json"
            path.write_text(json.dumps({"outcomes": [{"scenario": "Baseline", "summary": "Success",
                                                    "duration": 0.25, "log_path": "log/baseline.log"}]}))
            self.assertEqual(MODULE.validate_baseline(work, "library")["tests_executed"], 101)
            bad = json.loads(path.read_text()); bad["outcomes"][0]["log_path"] = "../escape.log"
            path.write_text(json.dumps(bad))
            with self.assertRaisesRegex(MODULE.Incomplete, "escapes"):
                MODULE.validate_baseline(work, "library")

    def test_baseline_hash_uses_retained_bytes_without_reopening_the_log(self):
        """The baseline digest and validated transcript identify one snapshot."""
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); directory = work / "mutation/library/mutants.out"
            (directory / "log").mkdir(parents=True)
            raw = literal_execution(doctests=True).encode("utf-8")
            baseline = directory / "log/baseline.log"
            baseline.write_bytes(raw)
            (directory / "outcomes.json").write_text(json.dumps({"outcomes": [
                {"scenario": "Baseline", "summary": "Success", "duration": 0.25,
                 "log_path": "log/baseline.log"}]}))
            resolved = []
            original_resolve = Path.resolve
            def observe_resolve(path, *arguments, **options):
                resolved.append(path)
                return original_resolve(path, *arguments, **options)
            with patch.object(MODULE.PRODUCT, "sha256", wraps=MODULE.PRODUCT.sha256) as hashing, \
                    patch.object(Path, "resolve", observe_resolve):
                result = MODULE.validate_baseline(work, "library")
            self.assertEqual(result["tests_executed"], 101)
            self.assertEqual(result["log_sha256"], hashlib.sha256(raw).hexdigest())
            self.assertFalse(any(call.args and call.args[0] == baseline
                                 for call in hashing.call_args_list),
                             "The admitted baseline bytes must not be reopened for hashing.")
            self.assertNotIn(baseline, resolved,
                             "Generated paths must not follow links before bounded admission.")

    def test_mutation_without_review_fails_before_any_session_or_command(self):
        policy = copy.deepcopy(MODULE.FIXED_POLICY); policy["mutation_inventories"] = {}
        args = SimpleNamespace(lane="mutation", subject="all")
        with patch.object(MODULE, "read_policy", return_value=policy), patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "requires reviewed nonempty"):
                MODULE.execute(args, {})
            session.assert_not_called()

    def test_direct_unknown_lane_cannot_manufacture_a_zero_command_pass(self):
        with patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "Unknown engine analysis lane"):
                MODULE.execute(SimpleNamespace(lane="not-a-lane", subject="all"), {})
            session.assert_not_called()

    def test_enumerate_only_launches_lists_and_never_claims_campaign_or_qualification(self):
        observed = literal_inventories(); calls = []; finished = []

        class FakeSession:
            def __init__(self, root, args, report, scope, chosen_policy, extra):
                self.tools = Path("/owned/tools")
                self.work = Path("/owned/work")
                self.selected_lane = args.lane
                self.source = source
                self.product_identity = identity

            def run(self, label, command, timeout):
                calls.append((label, command, timeout))
                subject = label.removeprefix("mutation-enumeration:")
                return json.dumps(list(observed[subject].values()))

            def finish(self):
                finished.append(True)

        args = SimpleNamespace(lane="enumerate", subject="all")
        report = {}
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp); policy, identity = self.fixture(source)
            with patch.object(MODULE, "read_policy", return_value=policy), \
                    patch.object(MODULE.COMMON, "Session", FakeSession), \
                    patch.object(MODULE.COMMON, "run_analysis_lanes") as campaign:
                MODULE.execute(args, report)
                campaign.assert_not_called()
        self.assertEqual(len(calls), 4)
        self.assertTrue(all(command[-2:] == ["--list", "--json"] and "--baseline" not in command
                            for _, command, _ in calls))
        self.assertEqual(report["listed_total"], 4)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        self.assertEqual(finished, [True])

    def test_combined_lane_keeps_coverage_first_and_one_final_custody_boundary(self):
        """Control-plane checks only: mocked commands supply no native qualification."""
        for failure_at in (None, "coverage", "finish", "inventory"):
            with self.subTest(failure_at=failure_at), tempfile.TemporaryDirectory() as temp:
                source = Path(temp); policy, identity = self.fixture(source)
                if failure_at != "inventory":
                    policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64}
                                                       for name in MODULE.SUBJECTS}
                self.write_policy(source, policy)
                report, calls, finished, sessions = {}, [], [], []

                class FakeSession:
                    def __init__(self, root, args, received_report, scope, chosen_policy, extra):
                        sessions.append(args.lane)
                        self.source, self.product_identity = source, identity
                        self.work, self.tools = Path("/owned/work"), Path("/owned/tools")
                    def run(self, *args, **options):
                        raise AssertionError("Synthetic control must not execute a command")
                    def finish(self):
                        finished.append(True)
                        if failure_at == "finish":
                            raise MODULE.Incomplete("final custody failure")

                def selected(scope, subjects, chosen, work, tools, run, received_report, lane):
                    calls.append(lane)
                    if lane == failure_at:
                        raise MODULE.Incomplete("selected coverage failure")

                with patch.object(MODULE, "read_policy", return_value=policy), \
                        patch.object(MODULE.COMMON, "Session", FakeSession), \
                        patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=selected):
                    if failure_at is None:
                        MODULE.execute(SimpleNamespace(lane="all", subject="all"), report)
                        self.assertTrue(report["qualification_passed"])
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute(SimpleNamespace(lane="all", subject="all"), report)
                        self.assertFalse(report.get("qualification_passed", False))
                self.assertEqual(sessions, [] if failure_at == "inventory" else ["all"])
                self.assertEqual(finished, [] if failure_at == "inventory" else [True])
                self.assertEqual(calls, [] if failure_at == "inventory" else
                                 ["coverage"] if failure_at == "coverage" else ["coverage", "mutation"])

    def test_failed_final_custody_cannot_leave_a_qualification_claim(self):
        report = {}
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp); policy, identity = self.fixture(source)
            fake = SimpleNamespace(source=source, product_identity=identity,
                                   work=Path("/owned/work"), tools=Path("/owned/tools"),
                                   run=lambda *args, **kwargs: "", finish=lambda: (_ for _ in ()).throw(
                                       MODULE.Incomplete("custody changed")))
            with patch.object(MODULE, "read_policy", return_value=policy), \
                    patch.object(MODULE.COMMON, "Session", return_value=fake), \
                    patch.object(MODULE.COMMON, "run_analysis_lanes"):
                with self.assertRaisesRegex(MODULE.Incomplete, "custody changed"):
                    MODULE.execute(SimpleNamespace(lane="coverage", subject="all"), report)
        self.assertFalse(report["qualification_passed"])

    def snapshot_control(self, change):
        """Exercise actual execute with a separately captured synthetic Session."""
        commands = []; finished = []; report = {}; failure = None
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp); policy, identity = self.fixture(source)

            class FakeSession:
                def __init__(self, root, args, received_report, scope, chosen_policy, extra):
                    self.source = source
                    self.work = Path("/owned/work")
                    self.tools = Path("/owned/tools")
                    self.product_identity = dict(identity)
                    if change == "candidate":
                        self.product_identity["new-session-input.rs"] = "b" * 64
                    elif change == "pin":
                        (source / "Cargo.lock").write_text("different copied lock bytes")
                    elif change == "policy":
                        copied = copy.deepcopy(policy); copied["scope"] = "different copied policy"
                        (source / MODULE.POLICY).write_text(json.dumps(copied))

                def run(self, label, command, timeout, **options):
                    commands.append((label, command))
                    return literal_execution(doctests=False)

                def finish(self):
                    finished.append(True)

            def selected_lane(scope, subjects, chosen_policy, work, tools, run, received_report, lane):
                run("coverage", ["fixture-only-coverage-command"], 1)

            with patch.object(MODULE, "read_policy", return_value=policy), \
                    patch.object(MODULE.COMMON, "Session", FakeSession), \
                    patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=selected_lane):
                try:
                    MODULE.execute(SimpleNamespace(lane="coverage", subject="all"), report)
                except MODULE.Incomplete as error:
                    failure = error
        return report, failure, commands, finished

    def test_execute_rejects_session_candidate_change_before_any_command(self):
        report, failure, commands, finished = self.snapshot_control("candidate")
        self.assertEqual(finished, [True])
        self.assertEqual(commands, [])
        self.assertIsInstance(failure, MODULE.Incomplete)
        self.assertFalse(report["qualification_passed"])

    def test_execute_rejects_copied_input_pin_drift_before_any_command(self):
        report, failure, commands, finished = self.snapshot_control("pin")
        self.assertEqual(finished, [True])
        self.assertEqual(commands, [])
        self.assertIsInstance(failure, MODULE.Incomplete)
        self.assertFalse(report["qualification_passed"])

    def test_execute_rejects_copied_policy_drift_before_any_command(self):
        report, failure, commands, finished = self.snapshot_control("policy")
        self.assertEqual(finished, [True])
        self.assertEqual(commands, [])
        self.assertIsInstance(failure, MODULE.Incomplete)
        self.assertFalse(report["qualification_passed"])

    def test_execute_healthy_captured_session_is_admitted_with_final_custody(self):
        report, failure, commands, finished = self.snapshot_control("healthy")
        self.assertIsNone(failure)
        self.assertEqual(commands, [("coverage", ["fixture-only-coverage-command"])])
        self.assertEqual(finished, [True])
        self.assertTrue(report["qualification_passed"])

    def test_readiness_is_a_finite_unexecuted_single_worker_proposal(self):
        proposal = MODULE.readiness_proposal()
        self.assertEqual((proposal["kind"], proposal["executed"], proposal["outer_workers"]),
                         ("proposal", False, 1))
        self.assertEqual((proposal["build_seconds"], proposal["test_seconds"]), (90, 60))
        self.assertIn("not assessed", proposal["ten_worker_readiness"])

    def test_enumeration_cli_status_remains_distinct_from_passed_qualification(self):
        observed = []

        def complete_listing(args, report):
            observed.append(report)
            report.update(qualification_passed=False, campaign_admission=False)

        with patch.object(MODULE, "execute", side_effect=complete_listing), patch("sys.stdout", new_callable=io.StringIO):
            status = MODULE.main(["--lane", "enumerate", "--vendor-dir", "/admitted/vendor",
                                  "--output-dir", "/new/owned/output"])
        self.assertEqual(status, 0)
        self.assertEqual(observed[0]["status"], "enumerated")
        self.assertFalse(observed[0]["qualification_passed"])

    def test_cli_selectors_route_measurement_and_relative_without_changing_profile_or_lane(self):
        for subject in ("measurement", "relative"):
            def selected(args, report):
                self.assertEqual((args.lane, args.subject, args.tool_admission_profile),
                                 ("coverage", subject, "full"))
            with self.subTest(subject=subject), patch.object(MODULE, "execute", side_effect=selected), \
                    patch("sys.stdout", new_callable=io.StringIO):
                self.assertEqual(MODULE.main(["--lane", "coverage", "--subject", subject,
                                              "--vendor-dir", "/admitted/vendor", "--output-dir", "/owned/output"]), 0)


class EngineExternalContractTests(unittest.TestCase):
    """Current inert external contracts never inherit the historical campaigns."""

    def setUp(self):
        # Synthetic reader input, not current-product admission or evidence.
        # Both captured and dynamically consulted role bindings must agree;
        # expected identity comes from this declaration, never the validator.
        # One input also makes True numerically equal to the legal count, so
        # the boolean refusal cannot rely only on a different numeric value.
        self.fixture_identity = {"synthetic/engine.rs": "a" * 64}
        self.fixture_binding = {"source_file_count": len(self.fixture_identity),
                                "candidate_sha256": independent_digest(self.fixture_identity)}
        self.contract_policy = {**copy.deepcopy(MODULE.FIXED_CONTRACT_POLICY), **self.fixture_binding}
        for scoped in (
                patch.object(MODULE, "FIXED_CONTRACT_POLICY", self.contract_policy),
                patch.object(MODULE.COMMON, "CONTRACT_PRODUCT_BINDING", self.fixture_binding),
                patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, self.fixture_identity))):
            scoped.start(); self.addCleanup(scoped.stop)

    def fixture(self, root):
        for name in (MODULE.POLICY,"policy/verification-manifest.toml"):
            target=root/name;target.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(ROOT/name,target)
        parent=json.loads((root/MODULE.POLICY).read_text())
        parent["mutation_equivalences"]=[]
        parent["external_contracts"]=copy.deepcopy(self.contract_policy)
        (root/MODULE.POLICY).write_text(json.dumps(parent))
        return parent

    def test_configured_contract_cli_has_a_distinct_scoped_result(self):
        reports=[]
        def completed(args, report):
            self.assertEqual(args.tool_admission_profile,"stable-core")
            reports.append(report)
            report.update(contracts_qualification_passed=True,qualification_passed=False)
        with patch.object(MODULE,"execute",side_effect=completed),patch("sys.stdout",new_callable=io.StringIO):
            status=MODULE.main(["--lane","contracts","--tool-admission-profile","stable-core",
                                "--vendor-dir","/admitted/vendor","--output-dir","/new/owned/output"])
        self.assertEqual(status,0)
        self.assertEqual(reports[0]["status"],"passed")
        self.assertFalse(reports[0]["qualification_passed"])
        self.assertTrue(reports[0]["contracts_qualification_passed"])

    def test_current_contract_role_is_independent_of_stale_historical_candidate(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);parent=self.fixture(root)
            for name in MODULE.BOUND_INPUTS:
                target=root/name;target.parent.mkdir(parents=True,exist_ok=True)
                shutil.copyfile(ROOT/name,target)
            identity=self.fixture_identity
            parent["candidate_sha256"]=independent_digest(identity)
            parent["mutation_inventories"]={}
            parent["mutation_equivalences"]=[]
            parent["inputs_sha256"]={name:hashlib.sha256((root/name).read_bytes()).hexdigest()
                                      for name in MODULE.BOUND_INPUTS}
            path=root/MODULE.POLICY
            path.write_text(json.dumps(parent))
            with patch.object(MODULE.PRODUCT,"admit_product",return_value=({},identity)):
                self.assertEqual(MODULE.read_policy(root),parent)
                policy=MODULE.read_contract_policy(root)
                stale=copy.deepcopy(parent);stale["candidate_sha256"]="0"*64
                path.write_text(json.dumps(stale))
                original=path.read_bytes()
                self.assertEqual(MODULE.read_contract_policy(root),policy)
                with self.assertRaisesRegex(MODULE.Incomplete,"complete current product"):
                    MODULE.read_policy(root)
                self.assertEqual(path.read_bytes(),original)
                path.write_text(json.dumps(parent))
                self.assertEqual(MODULE.read_policy(root),parent)
                self.assertEqual(MODULE.read_contract_policy(root),policy)
        self.assertEqual(policy["source_file_count"],len(self.fixture_identity))
        self.assertEqual(policy["candidate_sha256"],independent_digest(self.fixture_identity))
        self.assertEqual(policy["critical_files"],["crates/nevrotick-engine/src/lib.rs"])
        self.assertEqual(policy["jobs"],6)
        self.assertEqual(policy["test_threads"],1)
        self.assertEqual(policy["tool_admission_profile"],"stable-core")
        self.assertEqual(MODULE.CONTRACT_SCOPE.subjects,
                         (("library","crates/nevrotick-engine/src/lib.rs"),))
        self.assertEqual(MODULE.COMMON.admission_profile(
            SimpleNamespace(lane="contracts",tool_admission_profile="stable-core"),
            MODULE.CONTRACT_SCOPE,policy),"stable-core")

    def test_child_role_rejects_poison_fields_without_rebinding_parent(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);parent=self.fixture(root)
            historical={key:copy.deepcopy(value) for key,value in parent.items() if key!="external_contracts"}
            healthy=MODULE.read_contract_policy(root)
            count=len(self.fixture_identity)
            for key,value in (("schema_version",2),("admission_status","passed"),
                              ("package","time-domain"),("source_file_count",True),
                              ("source_file_count",False),("source_file_count",float(count)),
                              ("source_file_count",count-1),("source_file_count",count+1),
                              ("candidate_sha256","0"*64),("jobs",12),("test_threads",12),
                              ("tool_admission_profile","full"),("critical_files",list(MODULE.SUBJECTS.values())),
                              ("mutation_equivalences",[{}]),("mutation_error_values",{"library":["panic!()"]}),
                              ("contracts_wall_seconds",0),("unexpected",1)):
                bad=copy.deepcopy(parent);bad["external_contracts"][key]=value
                path=root/MODULE.POLICY
                path.write_text(json.dumps(bad));original=path.read_bytes()
                with self.subTest(key=key,value=value),self.assertRaises(MODULE.Incomplete):
                    MODULE.read_contract_policy(root)
                self.assertEqual(path.read_bytes(),original)
            (root/MODULE.POLICY).write_text(json.dumps(parent))
            self.assertEqual(MODULE.read_contract_policy(root),healthy)
            self.assertEqual({key:value for key,value in MODULE.read_json(root/MODULE.POLICY).items()
                              if key!="external_contracts"},historical)

    def test_contract_candidate_requires_declared_fixture_paths_and_bytes(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);self.fixture(root)
            path=root/MODULE.POLICY;original=path.read_bytes()
            healthy=MODULE.read_contract_policy(root)
            altered_bytes={**self.fixture_identity,"synthetic/engine.rs":"c"*64}
            altered_path={"synthetic/other.rs":self.fixture_identity["synthetic/engine.rs"]}
            for kind,identity in (("bytes",altered_bytes),("path",altered_path)):
                self.assertEqual(len(identity),len(self.fixture_identity))
                with self.subTest(kind=kind), \
                     patch.object(MODULE.PRODUCT,"admit_product",return_value=({},identity)):
                    with self.assertRaisesRegex(MODULE.Incomplete,"complete contract candidate path/byte digest"):
                        MODULE.read_contract_policy(root)
                self.assertEqual(path.read_bytes(),original)
            self.assertEqual(MODULE.read_contract_policy(root),healthy)

    def test_privacy_trait_and_raw_api_fixtures_have_owned_causal_lines(self):
        cases=MODULE.contract_examples()
        self.assertEqual(len(cases),13)
        self.assertIsNone(cases["legal"]["code"])
        expected={
            "config_construction":"E0451","schedule_construction":"E0451",
            "config_peers_mutation":"E0616","config_timeout_mutation":"E0616",
            "schedule_stop_mutation":"E0616","schedule_invalidate_mutation":"E0616",
            "config_default":"E0277","schedule_default":"E0277",
            "config_from":"E0277","schedule_from":"E0277",
            "raw_config_prepare":"E0308","raw_schedule_execute":"E0308"}
        for name,code in expected.items():
            case=cases[name]
            with self.subTest(case=name):
                self.assertEqual(case["code"],code)
                self.assertEqual(case["line"],4)
                self.assertEqual(len(case["source"].splitlines()),6)
                self.assertNotIn("\\\\n",case["source"])
                self.assertTrue(case["tokens"])
        legal=cases["legal"]["source"]
        self.assertIn("Config::new",legal)
        self.assertIn("ControlSchedule::new",legal)
        self.assertIn("PilotError::DuplicatePeer",legal)
        self.assertIn("PilotError::InvalidTimeout",legal)
        self.assertNotIn("prepare(",legal)
        self.assertNotIn("execute(",legal)
        self.assertNotIn("Runtime",legal)
        self.assertIn("fn rejected(pilot: PreparedPilot)",cases["raw_schedule_execute"]["source"])
        self.assertTrue({"peers","timeout","experimental_profile"} <= set(cases["config_construction"]["tokens"]))
        self.assertEqual(cases["config_construction"]["source"].splitlines()[3],
                         "let _ = Config { peers: [peers[0], peers[0]], timeout: Duration::ZERO, "
                         "experimental_profile: None };")
        self.assertTrue({"stop_after","invalidate_after"} <= set(cases["schedule_construction"]["tokens"]))
        self.assertTrue(all("let _ = &value;" in cases[name]["source"].splitlines()[3]
                            for name in expected if name.endswith("_mutation")))

    def test_contract_library_copy_and_command_dispatch_preserve_real_artifact(self):
        cases=MODULE.contract_examples()
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary);self.fixture(work)
            deps=work/"target/debug/deps";deps.mkdir(parents=True)
            artifact=work/"target/debug/libnevrotick_engine.rlib";artifact.write_bytes(b"actual emitted library")
            dependency=deps/("libtokio-"+"1"*16+".rlib"); dependency.write_bytes(b"actual dependency")
            calls=[]
            def run(label,command,timeout,**kwargs):
                calls.append((label,command,kwargs.get("specimen")))
                if label=="contract-library":
                    self.assertEqual(command[command.index("--jobs")+1],"6")
                    return messages
                if label=="external-legal-execution":
                    return cases["legal"]["success_marker"]
                return ""
            session=SimpleNamespace(work=work,stable=work/"stable",run=run,protect=lambda _:None)
            messages=contract_artifact_messages(session,{"nevrotick-engine":artifact,"tokio":dependency})
            report={}
            with patch.object(MODULE.COMMON,"library_artifact",
                              return_value="/work/target/debug/libnevrotick_engine.rlib"):
                MODULE.contracts(session,MODULE.read_contract_policy(work),report)
            self.assertEqual(report["contracts"]["legal_consumers_compiled_and_executed"],1)
            self.assertEqual(len([case for _,_,case in calls if case is not None]),12)
            copied=work/"contract-deps/libnevrotick_engine.rlib"
            self.assertEqual(copied.read_bytes(),artifact.read_bytes())
            self.assertNotEqual(copied.stat().st_ino,artifact.stat().st_ino)
            self.assertEqual(copied.stat().st_mode&0o777,0o400)
            self.assertTrue(all("--error-format=json" in command for label,command,_ in calls
                                if label.startswith("external-consumer-")))
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary);self.fixture(work)
            (work/"target/debug/deps").mkdir(parents=True)
            (work/"target/debug/libnevrotick_engine.rlib").write_bytes(b"emitted library")
            def missing_marker(label,command,timeout,**kwargs):
                return "not the expected actual consumer marker" if label=="external-legal-execution" else messages if label=="contract-library" else ""
            session=SimpleNamespace(work=work,stable=work/"stable",run=missing_marker,protect=lambda _:None)
            messages=contract_artifact_messages(session,{"nevrotick-engine":work/"target/debug/libnevrotick_engine.rlib"})
            report={}
            with patch.object(MODULE.COMMON,"library_artifact",
                              return_value="/work/target/debug/libnevrotick_engine.rlib"):
                with self.assertRaisesRegex(MODULE.Incomplete,"Legal engine consumer"):
                    MODULE.contracts(session,MODULE.read_contract_policy(work),report)
            self.assertNotIn("contracts",report)

    def test_contracts_reject_wrong_profile_or_subject_before_session(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);self.fixture(root)
            cases=(("all","full","Analysis admission profile differs from the current policy."),
                   ("cli","stable-core","External engine contracts belong only to the library owner."),
                   ("measurement","stable-core","External engine contracts belong only to the library owner."))
            with patch.object(MODULE,"ROOT",root):
                for subject,profile,diagnostic in cases:
                    with self.subTest(subject=subject,profile=profile), \
                         patch.object(MODULE.COMMON,"Session") as session:
                        with self.assertRaises(MODULE.Incomplete) as refused:
                            MODULE.execute(SimpleNamespace(lane="contracts",subject=subject,
                                           tool_admission_profile=profile),{})
                        self.assertEqual(str(refused.exception),diagnostic)
                        session.assert_not_called()

    def test_failed_current_custody_never_publishes_contract_qualification(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)/"original";source=Path(temporary)/"captured"
            self.fixture(root);self.fixture(source)
            finished=[]
            def finish():
                finished.append(True)
                raise MODULE.Incomplete("final custody failure")
            fake=SimpleNamespace(source=source,product_identity=dict(self.fixture_identity),finish=finish)
            report={}
            with patch.object(MODULE,"ROOT",root), \
                 patch.object(MODULE.COMMON,"Session",return_value=fake) as session, \
                 patch.object(MODULE,"contracts") as contracts:
                with self.assertRaisesRegex(MODULE.Incomplete,"final custody failure"):
                    MODULE.execute(SimpleNamespace(lane="contracts",subject="all",
                                   tool_admission_profile="stable-core"),report)
                session.assert_called_once()
                contracts.assert_called_once_with(fake,self.contract_policy,report)
            self.assertEqual(finished,[True])
            self.assertEqual(report["candidate_sha256"],independent_digest(self.fixture_identity))
            self.assertFalse(report["qualification_passed"])
            self.assertFalse(report["contracts_qualification_passed"])


class QualityAnalysisTests(unittest.TestCase):
    """Q1 registration controls use synthetic native logs; no analyzer is invoked."""

    def setUp(self):
        """Keep reader identities independent of the host home and private evidence."""
        self.fixture_receipt = Path("/synthetic/q1-reader/ordinary.json")
        self.fixture_receipt_sha256 = hashlib.sha256(b"synthetic Q1 ordinary identity\n").hexdigest()
        names = ("quality_ordinary_v2.py", "quality_ordinary_v1.py", "relative_bridge_ordinary.py",
                 "conditioned_fit_ordinary_v2.py", "series_ordinary_v1.py", "series_ordinary_v3.py")
        self.fixture_recipe_pins = {name: hashlib.sha256(
            ("# Synthetic Q1 registration recipe: " + name + "\n").encode()).hexdigest()
            for name in names}
        for context in (
                patch.object(MODULE, "QUALITY_ORDINARY_RECEIPT", self.fixture_receipt),
                patch.object(MODULE, "QUALITY_ORDINARY_SHA256", self.fixture_receipt_sha256),
                patch.object(MODULE, "QUALITY_ORDINARY_RECIPES", self.fixture_recipe_pins)):
            context.start()
            self.addCleanup(context.stop)

    @staticmethod
    def assertion_sources():
        """Small parser fixtures, not historical Rust source or executed product tests."""
        private = """fn exact() { assert_eq!(17, 17); }
fn facts() { assert_eq!(41, 41); }
fn computed_policy_boundaries_preserve_strict_and_inclusive_comparisons() { assert_eq!(1, 1); }
fn declared_getters_and_constructor_precedence_preserve_original_bits() { assert_eq!(1, 1); }
fn dyadic_calculation_borrows_original_facts_and_assumptions() { facts(); exact(); assert!(true); }
fn evaluator_failure_precedence_preserves_original_observation() { assert_eq!(1, 1); }
fn finite_input_overflow_is_refused_at_the_first_computed_stage() { assert_eq!(1, 1); }
fn generated_garde_dispatch_covers_nested_and_computed_fields() { assert_eq!(1, 1); }
fn gradual_underflow_does_not_claim_exact_zero_uncertainty() { assert_eq!(1, 1); }
fn mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields() { facts(); assert!(true); }
fn negative_local_wall_interval_grows_declared_frequency_dispersion() { assert_eq!(1, 1); }
fn negative_server_interval_processing_limit_uses_absolute_magnitude() { assert_eq!(1, 1); }
fn original_measurement_refusal_and_redacted_diagnostics_are_preserved() { assert_eq!(1, 1); }
fn precision_clamps_have_exact_boundary_and_neighbor_values() { assert_eq!(1, 1); }
fn raw_round_trip_is_converted_once_before_cancellation() { assert_eq!(1, 1); }
fn raw_scalar_and_interval_helpers_preserve_full_admitted_domains() { assert_eq!(1, 1); }
fn refusal_families_keep_bounded_diagnostics_and_typed_sources() { assert_eq!(1, 1); }
fn signed_server_interval_and_unsigned_root_words_are_retained() { assert_eq!(1, 1); }
fn zero_raw_rtt_uses_combined_precision_floor() { assert_eq!(1, 1); }
"""
        external = """fn check(_value: bool, _fact: &str) {}
fn facts() { assert_eq!(41, 41); }
fn run_series() { assert_eq!(43, 43); }
fn check_report() { assert_eq!(47, 47); }
fn diagnostic() { assert_eq!(53, 53); }
fn serve() { assert_eq!(59, 59); }
fn check_ratio() { assert_eq!(2, 2); }
fn check_near() { check(true, "fixture quality arithmetic differs"); }
fn check_quality() { check_ratio(); check_near(); }
fn absent_measurement_refuses_quality_with_actual_valid_neighbor() { check_quality(); }
fn actual_quality_limits_refuse_without_rewriting_completion_or_age() { check_quality(); }
fn actual_series_quality_retains_signed_intervals_source_and_original_age() {
    facts(); run_series(); check_report(); diagnostic(); serve(); check_quality();
    check(true, "conditional quality changed literal peer claims");
}
"""
        return {
            "crates/nevrotick-engine/src/quality/tests.rs": private.encode(),
            "crates/nevrotick-engine/tests/quality.rs": external.encode(),
        }

    def semantic_policy(self):
        """Bind only synthetic assertion inputs at this unit boundary, never production policy."""
        policy = self.policy()
        pins = {name: hashlib.sha256(data).hexdigest()
                for name, data in self.assertion_sources().items()}
        policy["inputs_sha256"].update(pins)
        policy["candidate_sha256"] = independent_digest(pins)
        return policy

    def write_assertion_sources(self, root):
        """Materialize the two fixed parser inputs without reading private evidence."""
        for filename, data in self.assertion_sources().items():
            target = root / filename
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)

    @contextmanager
    def ordinary_fixture(self):
        """Temporary synthetic receipt/recipes exercise the real reader, not product evidence."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            recipes = root / "recipes"
            recipes.mkdir()
            recipe_names = (
                "quality_ordinary_v2.py", "quality_ordinary_v1.py", "relative_bridge_ordinary.py",
                "conditioned_fit_ordinary_v2.py", "series_ordinary_v1.py", "series_ordinary_v3.py",
            )
            recipe_pins = {}
            for name in recipe_names:
                data = ("# Synthetic reader fixture only: " + name + "\n").encode()
                (recipes / name).write_bytes(data)
                recipe_pins[name] = hashlib.sha256(data).hexdigest()
            policy = self.policy()
            source = {f"synthetic/source-{index:03}.rs": f"{index:064x}" for index in range(421)}
            policy["candidate_sha256"] = independent_digest(source)
            inventory = policy["test_inventory"]
            receipt_data = {
                "scope": "Synthetic receipt-reader fixture; no Rust execution or qualification.",
                "status": "passed-q1-quality-all", "bookends_unchanged": True,
                "candidate_sha256": policy["candidate_sha256"], "source_sha256": source,
                "recipes_sha256": {str(recipes / name): digest for name, digest in recipe_pins.items()},
                "lanes": [{"lane": lane, "tests": {"executed": len(inventory[key]),
                    "test_names": inventory[key]}} for lane, key in (
                        ("engine-tests", "ordinary"), ("engine-optimized-tests", "ordinary"),
                        ("engine-doctests", "doctests"))],
            }
            receipt = root / "receipt.json"
            raw = json.dumps(receipt_data, sort_keys=True, separators=(",", ":")).encode()
            receipt.write_bytes(raw)
            policy["ordinary_evidence"] = {"receipt": str(receipt),
                "sha256": hashlib.sha256(raw).hexdigest(), "recipes_sha256": recipe_pins}
            with patch.object(MODULE, "QUALITY_EVIDENCE_ROOT", recipes), \
                 patch.object(MODULE, "QUALITY_ORDINARY_RECEIPT", receipt), \
                 patch.object(MODULE, "QUALITY_ORDINARY_SHA256", hashlib.sha256(raw).hexdigest()), \
                 patch.object(MODULE, "QUALITY_ORDINARY_RECIPES", recipe_pins):
                yield policy, receipt, recipes

    def test_current_roles_keep_distinct_source_scopes_and_bound_input_sets(self):
        """Current package identities do not merge the two qualification owners."""
        policy = self.policy()
        MODULE.validate_quality_policy(policy)
        ordinary, docs = MODULE.quality_test_inventory(policy)
        self.assertEqual((len(ordinary), len(docs)), (113, 50))
        self.assertEqual(len(MODULE.SERIES_REQUIRED_TESTS), 23)
        self.assertTrue(MODULE.SERIES_CHECKPOINT_REQUIRED_TESTS <= set(ordinary))
        self.assertNotIn(MODULE.SERIES_PROCESS_SOURCE, MODULE.QUALITY_BOUND_INPUTS)
        self.assertIn(MODULE.SERIES_PROCESS_SOURCE, MODULE.SERIES_CURRENT_BOUND_INPUTS)
        self.assertEqual(MODULE.QUALITY_FIXED_POLICY["source_file_count"], 421)

    def policy(self):
        """Reuse fixed historical role fields with separately declared reader identities."""
        policy = copy.deepcopy(json.loads((ROOT / MODULE.QUALITY_POLICY).read_text()))
        policy["ordinary_evidence"] = {
            "receipt": str(self.fixture_receipt), "sha256": self.fixture_receipt_sha256,
            "recipes_sha256": dict(self.fixture_recipe_pins),
        }
        return policy

    def execution(self, *, doctests=True):
        """Render declared parser-fixture identities, never claim product execution."""
        inventory = self.policy()["test_inventory"]
        names = inventory["ordinary"] + (inventory["doctests"] if doctests else [])
        return ("\n".join("test " + name + " ... ok" for name in names)
                + f"\ntest result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def phases(self, summary):
        """Independent literal native phase shape for accounting fixtures."""
        phases = ["Build"] if summary == "Unviable" else ["Build", "Test"]
        return [{"phase": phase, "duration": 0.1,
            "argv": ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                     "--verbose", "--package=nevrotick-engine@0.0.0", "--message-format=json", "--frozen"],
            "process_status": {"Failure": 101} if summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test") else "Success"}
            for phase in phases]

    def descriptor(self, name="quality synthetic fault"):
        """Complete synthetic fault descriptor, not actual cargo-mutants enumeration."""
        return {"name": name, "package": "nevrotick-engine", "file": MODULE.QUALITY_SUBJECTS["quality"],
            "function": {"function_name": "fixture_only", "return_type": "bool",
                         "span": {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 1}}},
            "span": {"start": {"line": 1, "column": 2}, "end": {"line": 1, "column": 4}},
            "replacement": "!=", "genre": "BinaryOperator"}

    def baseline_fixture(self, root):
        self.write_assertion_sources(root / "source")
        directory = root / "mutation/quality/mutants.out"
        directory.mkdir(parents=True)
        (directory / "baseline.log").write_text(self.execution())
        report = {"outcomes": [{"scenario": "Baseline", "summary": "Success",
            "phase_results": self.phases("Success"), "log_path": "baseline.log"}]}
        (directory / "outcomes.json").write_text(json.dumps(report))
        return directory, report

    def semantic_sites(self):
        """Use fixed synthetic parser bytes; no historical product-source claim follows."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.write_assertion_sources(root)
            return MODULE.quality_semantic_assertion_sites(root, self.semantic_policy())

    @staticmethod
    def failed_execution(failures):
        """Literal completed libtest failure records; no product execution claimed."""
        noun = "test" if len(failures) == 1 else "tests"
        return (f"running {len(failures)} {noun}\n"
                + "".join(f"test {test} ... FAILED\n" for test, _ in failures)
                + "\nfailures:\n\n"
                + "".join(f"---- {test} stdout ----\n{body}\n\n" for test, body in failures)
                + "failures:\n\n" + "".join(f"    {test}\n" for test, _ in failures)
                + f"\ntest result: FAILED. 0 passed; {len(failures)} failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s\n")

    def caught_fixture(self, directory):
        test = "quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        site=next(site for site in self.semantic_sites()["tests"][test] if site["owner"]=="exact")
        body = (f"thread '{test}' panicked at "
                f"{site['source']}:{site['line']}:{site['column']}:\n"
                f"{site['marker']}\n  left: 17\n right: 19")
        text = self.failed_execution([(test, body)])
        (directory / "caught.log").write_text(text)
        row = {"scenario": {"Mutant": self.descriptor()}, "summary": "CaughtMutant",
               "phase_results": self.phases("CaughtMutant"), "log_path": "caught.log"}
        return text, {"outcomes": [row]}

    def test_quality_role_selects_only_one_owner_with_unchanged_finite_gates(self):
        policy = self.policy()
        self.assertEqual(MODULE.validate_quality_policy(policy), policy)
        self.assertEqual(MODULE.QUALITY_SCOPE.select("all"),
                         {"quality": "crates/nevrotick-engine/src/quality.rs"})
        for subject in ["library", "series", "relative", "measurement", "cli"]:
            with self.subTest(subject=subject), self.assertRaises(MODULE.Incomplete):
                MODULE.QUALITY_SCOPE.select(subject)
        for key, value in {"coverage_minimum_line_percent":90, "mutation_minimum_viable_killed_percent":95,
            "mutation_jobs":8, "jobs":6, "test_threads":1, "minimum_free_bytes":5*1024**3,
            "tool_admission_profile":"full"}.items():
            self.assertEqual(policy[key], value)
        self.assertEqual([entry["id"] for entry in policy["mutation_equivalences"]],
                         ["ENGINE-QUALITY-INTERVAL-ZERO-SIGN"])
        self.assertEqual(policy["mutation_inventories"], {"quality":{"count":167,
            "sha256":"52851ec7a7408ed86034de072648f0e3071e3f4b314142f9ee529fb9b90971f5"}})
        self.assertEqual(policy["source_file_count"],421)
        self.assertEqual(policy["test_inventory"]["baseline_count"],163)
        self.assertEqual(len([name for name in MODULE.QUALITY_REQUIRED_TESTS if name.startswith("quality::tests::")]),17)
        self.assertEqual(len(MODULE.QUALITY_REQUIRED_TESTS),20)
        self.assertEqual(len(policy["ordinary_evidence"]["recipes_sha256"]),6)
        self.assertIn("quality_ordinary_v2.py",policy["ordinary_evidence"]["recipes_sha256"])
        readiness={"declared_getters_and_constructor_precedence_preserve_original_bits",
            "evaluator_failure_precedence_preserves_original_observation",
            "generated_garde_dispatch_covers_nested_and_computed_fields",
            "negative_local_wall_interval_grows_declared_frequency_dispersion",
            "negative_server_interval_processing_limit_uses_absolute_magnitude",
            "raw_scalar_and_interval_helpers_preserve_full_admitted_domains",
            "refusal_families_keep_bounded_diagnostics_and_typed_sources",
            "zero_raw_rtt_uses_combined_precision_floor"}
        self.assertTrue({"quality::tests::"+name for name in readiness}<=MODULE.QUALITY_REQUIRED_TESTS)

    def test_quality_policy_rejects_changed_source_recipe_count_and_exclusions(self):
        changes = [lambda p:p.update(source_file_count=411), lambda p:p.update(mutation_jobs=10),
            lambda p:p.update(tool_admission_profile="stable-core"),
            lambda p:p.update(mutation_equivalences=[{"inherited":"series"}]),
            lambda p:p["inputs_sha256"].update({MODULE.QUALITY_SUBJECTS["quality"]:"a"*64}),
            lambda p:p["ordinary_evidence"]["recipes_sha256"].update({"quality_ordinary_v1.py":"a"*64}),
            lambda p:p["ordinary_evidence"]["recipes_sha256"].update({"quality_ordinary_v2.py":"a"*64}),
            lambda p:p.update(candidate_sha256="a"*64),
            lambda p:p["test_inventory"].update(baseline_count=True)]
        for change in changes:
            policy = self.policy(); change(policy)
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_policy(policy)

    @staticmethod
    def zero_sign_entry():
        """Independent literal reviewed fault; this fixture supplies no native outcome."""
        identifier = "ENGINE-QUALITY-INTERVAL-ZERO-SIGN"
        return {
            "id": identifier,
            "candidate_sha256": "e8f6ead34f41429033a8d6c0ebd823bc693dae8caa645d53a8eafedf20e8727d",
            "source_sha256": "f3abb4b00c769b3a4fb655cb923df7626c4209cb792594a55f065449051e2135",
            "mutant": {
                "file": "crates/nevrotick-engine/src/quality.rs", "package": "nevrotick-engine",
                "name": "crates/nevrotick-engine/src/quality.rs:524:29: replace < with <= in interval_seconds",
                "function": {"function_name": "interval_seconds",
                    "return_type": "-> Result<RoundedRationalSeconds, QualityRefusal>",
                    "span": {"start": {"line": 514, "column": 1}, "end": {"line": 531, "column": 2}}},
                "span": {"start": {"line": 524, "column": 29}, "end": {"line": 524, "column": 30}},
                "replacement": "<=", "genre": "BinaryOperator"},
            "requirement": "REQ-8/BND-1; CHECK-25/27; canonical rational zero",
            "rationale": "Exact reviewed full typed-domain constructor canonicalization; synthetic admission control only.",
            "proof": {"path": "tasks/engine-analysis-equivalences.md",
                "sha256": hashlib.sha256((ROOT / "tasks/engine-analysis-equivalences.md").read_bytes()).hexdigest(),
                "section": identifier, "reviewer": "independent zero-sign source review; observer fixture",
                "verdict": "independently-justified-equivalent"}}

    def test_quality_equivalence_opt_in_is_only_the_exact_zero_sign_fault(self):
        entry = self.zero_sign_entry()
        self.assertEqual(independent_digest(entry["mutant"]),
                         "ab870f10eb8cde026ababd980eb91338a2026e753395d4cd178bd50b8b6393ec")
        MODULE.validate_quality_equivalence_scope([])
        MODULE.validate_quality_equivalence_scope([entry])
        for change in ("list", "duplicate", "foreign", "source", "line", "span", "replacement",
                       "function", "return_type", "genre", "package", "file", "proof_path", "proof_section",
                       "proof_hash", "verdict", "reviewer", "candidate", "rationale", "extra"):
            bad = copy.deepcopy(entry)
            if change == "list": bad = None
            elif change == "foreign": bad["id"] = "ENGINE-SERIES-UNAVAILABLE-ADMITTED-ZERO"
            elif change == "source": bad["source_sha256"] = "a" * 64
            elif change == "line": bad["mutant"]["span"]["start"]["line"] = 525
            elif change == "span": bad["mutant"]["function"]["span"]["end"]["line"] = 532
            elif change == "replacement": bad["mutant"]["replacement"] = ">="
            elif change == "function": bad["mutant"]["function"]["function_name"] = "exact_intervals"
            elif change == "return_type": bad["mutant"]["function"]["return_type"] = "-> f64"
            elif change == "genre": bad["mutant"]["genre"] = "FnValue"
            elif change == "package": bad["mutant"]["package"] = "time-domain"
            elif change == "file": bad["mutant"]["file"] = "crates/time-domain/src/rational_seconds.rs"
            elif change == "proof_path": bad["proof"]["path"] = "tasks/foreign.md"
            elif change == "proof_section": bad["proof"]["section"] = "ENGINE-NOMINAL-ZERO-SIGN"
            elif change == "proof_hash": bad["proof"]["sha256"] = "bad"
            elif change == "verdict": bad["proof"]["verdict"] = "unreviewed"
            elif change == "reviewer": bad["proof"]["reviewer"] = " "
            elif change == "candidate": bad["candidate_sha256"] = False
            elif change == "rationale": bad["rationale"] = " "
            elif change == "extra": bad["unknown"] = True
            cohort = None if change == "list" else [bad, copy.deepcopy(bad)] if change == "duplicate" else [bad]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_equivalence_scope(cohort)
        policy = self.policy(); policy["mutation_equivalences"] = []
        MODULE.validate_quality_policy(policy)
        policy["mutation_equivalences"] = [entry]
        MODULE.validate_quality_policy(policy)
        policy["mutation_equivalences"][0]["candidate_sha256"] = "a" * 64
        with self.assertRaisesRegex(MODULE.Incomplete, "complete policy candidate"):
            MODULE.validate_quality_policy(policy)

    def test_quality_converter_and_dossier_are_explicit_protected_inputs(self):
        converter, dossier = "crates/time-domain/src/rational_seconds.rs", "tasks/engine-analysis-equivalences.md"
        policy = self.policy()
        self.assertIn(converter, MODULE.QUALITY_BOUND_INPUTS)
        self.assertIn(dossier, MODULE.QUALITY_BOUND_INPUTS)
        self.assertEqual(policy["inputs_sha256"][converter],
                         "2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438")
        self.assertEqual(policy["inputs_sha256"][dossier], policy["mutation_equivalences"][0]["proof"]["sha256"])
        policy["inputs_sha256"][converter] = "a" * 64
        with self.assertRaisesRegex(MODULE.Incomplete, "bindings are invalid"):
            MODULE.validate_quality_policy(policy)

    def test_quality_equivalent_native_miss_is_excluded_without_kill_credit(self):
        entry = self.zero_sign_entry(); equivalent = entry["mutant"]
        caught = self.descriptor("quality synthetic caught companion")
        expected = {row["name"]: row for row in (caught, equivalent)}
        report = {"cargo_mutants_version": "27.1.0", "total_mutants": 2,
            "caught": 1, "missed": 1, "unviable": 0, "timeout": 0, "success": 0,
            "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": self.phases("Success")},
                {"scenario": {"Mutant": caught}, "summary": "CaughtMutant", "phase_results": self.phases("CaughtMutant")},
                {"scenario": {"Mutant": equivalent}, "summary": "MissedMutant", "phase_results": self.phases("MissedMutant")}]}
        def observe(data, native=2):
            return MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE, data, expected,
                "crates/nevrotick-engine/src/quality.rs", native_exit_code=native,
                equivalences=[entry], defer_gate=True)
        result = observe(report)
        self.assertEqual((result["killed"], result["survived"], result["excluded"],
                          result["raw_denominator"], result["adjusted_denominator"]), (1, 1, 1, 2, 1))
        with self.assertRaisesRegex(MODULE.Incomplete, "native exit"): observe(report, native=0)
        bad = copy.deepcopy(report); bad["outcomes"][-1].update(summary="CaughtMutant", phase_results=self.phases("CaughtMutant"))
        bad.update(caught=2, missed=0)
        with self.assertRaisesRegex(MODULE.Incomplete, "not an actual missed mutant"):
            observe(bad, native=0)

    def test_quality_ordinary_evidence_rejects_changed_actual_receipt_and_recipe_bytes(self):
        with self.ordinary_fixture() as (policy, receipt, recipes):
            admitted = MODULE.admit_quality_ordinary(policy)
            self.assertFalse(admitted["fresh_execution"])
            original = receipt.read_bytes()
            receipt.write_bytes(b"changed synthetic receipt")
            with self.assertRaisesRegex(MODULE.Incomplete,"receipt bytes"):
                MODULE.admit_quality_ordinary(policy)
            receipt.write_bytes(original)
            recipe = recipes / "quality_ordinary_v2.py"
            original_recipe = recipe.read_bytes()
            recipe.write_bytes(b"# changed synthetic recipe\n")
            with self.assertRaisesRegex(MODULE.Incomplete,"recipes"):
                MODULE.admit_quality_ordinary(policy)
            recipe.write_bytes(original_recipe)
            self.assertFalse(MODULE.admit_quality_ordinary(policy)["fresh_execution"])
            with self.assertRaisesRegex(MODULE.Incomplete, "Unreviewed quality"):
                MODULE.validate_quality_policy(policy)
        policy = self.policy()
        self.assertEqual(MODULE.validate_quality_policy(policy), policy)
        with patch.object(MODULE,"read_json",return_value=policy), \
             patch.object(MODULE.PRODUCT,"admit_product",return_value=({}, {"new-source.rs":"a"*64})):
            with self.assertRaisesRegex(MODULE.Incomplete,"complete current product"):
                MODULE.read_quality_policy(ROOT)

    def test_quality_ordinary_parses_only_the_exact_hashed_raw_byte_observation(self):
        with self.ordinary_fixture() as (policy, receipt, _recipes):
            original=receipt.read_bytes()
            read=MODULE.COMMON.read_analysis_bytes
            def replaced_after_read(path,**kwargs):
                data=read(path,**kwargs)
                receipt.write_text('{"status":"replaced-after-hash"}')
                return data
            with patch.object(MODULE.COMMON,"read_analysis_bytes",side_effect=replaced_after_read), \
                 patch.object(MODULE,"read_json",side_effect=AssertionError("receipt was reopened")):
                admitted=MODULE.admit_quality_ordinary(policy)
            self.assertEqual(admitted["sha256"],hashlib.sha256(original).hexdigest())
            self.assertEqual(json.loads(receipt.read_text())["status"],"replaced-after-hash")
        for raw in [b'{"a":1,"a":2}',b'{"n":NaN}',b'{"n":1e9999}',b'[]',b'\xff']:
            bad=self.policy();bad["ordinary_evidence"]["sha256"]=hashlib.sha256(raw).hexdigest()
            with patch.object(MODULE.COMMON,"read_analysis_bytes",return_value=raw):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_quality_ordinary(bad)

    def test_quality_inventory_starts_empty_and_requires_exact_complete_nonempty_campaign(self):
        policy = self.policy()
        policy["mutation_inventories"] = {}
        self.assertEqual(MODULE.quality_inventory_bindings(policy), {})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.quality_inventory_bindings(policy, required=True)
        descriptor = self.descriptor()
        inventory = {descriptor["name"]:descriptor}
        binding = {"quality":{"count":1,"sha256":independent_digest(inventory)}}
        policy["mutation_inventories"] = binding
        self.assertEqual(MODULE.quality_inventory_bindings(policy, required=True), binding)
        admitted = MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.QUALITY_SCOPE, policy, {"quality":inventory})
        self.assertEqual(admitted, binding)
        for invalid in [{"series":binding["quality"]}, {"quality":{"count":0,"sha256":"a"*64}},
                        {"quality":{"count":True,"sha256":"a"*64}}, {"quality":{"count":513,"sha256":"a"*64}}]:
            policy["mutation_inventories"] = invalid
            with self.assertRaises(MODULE.Incomplete):
                MODULE.quality_inventory_bindings(policy, required=True)
        policy["mutation_inventories"] = {"quality":{"count":2,"sha256":independent_digest(inventory)}}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.QUALITY_SCOPE, policy, {"quality":inventory})

    def test_quality_execution_requires_complete_unique_actual_native_records(self):
        policy = self.policy()
        healthy = self.execution()
        self.assertEqual(MODULE.validate_quality_test_execution(healthy, policy, doctests=True)["tests_executed"],
                         policy["test_inventory"]["baseline_count"])
        ordinary = self.execution(doctests=False)
        self.assertEqual(MODULE.validate_quality_test_execution(ordinary, policy, doctests=False)["ordinary_tests"],
                         len(policy["test_inventory"]["ordinary"]))
        first = healthy.splitlines()[0]
        for invalid in ["", healthy.replace(first+"\n", "", 1), first+"\n"+healthy,
                        healthy.replace(" ... ok", " ... FAILED", 1),
                        healthy.replace("0 ignored", "1 ignored"), healthy.replace("0 filtered out", "1 filtered out")]:
            with self.subTest(invalid=invalid[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_test_execution(invalid, policy, doctests=True)

    def test_quality_baseline_requires_selected_successful_full_native_execution(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory, report = self.baseline_fixture(root)
            result = MODULE.validate_quality_baseline(root, "quality", self.semantic_policy())
            self.assertEqual(result["tests_executed"], self.policy()["test_inventory"]["baseline_count"])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_baseline(root, "library", self.semantic_policy())
            for rows in [[], report["outcomes"]*2, [{**report["outcomes"][0],"summary":"CaughtMutant"}]]:
                (directory/"outcomes.json").write_text(json.dumps({"outcomes":rows}))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_baseline(root, "quality", self.semantic_policy())
            (directory/"outcomes.json").write_text(json.dumps(report))
            (directory/"baseline.log").write_text("test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\n")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_baseline(root, "quality", self.semantic_policy())

    def test_quality_caught_causes_require_owned_selected_behavioral_assertions(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            healthy, outcomes = self.caught_fixture(directory)
            result = MODULE.validate_quality_caught_causes(directory, outcomes, self.semantic_sites())
            self.assertEqual(len(result["checked_caught_logs"]), 1)
            for text in [healthy.replace("src/quality/tests.rs", "src/series.rs"),
                         healthy.replace("assertion `left == right` failed", "Error: fixture setup failed"),
                         healthy+"Q1_HARNESS_INCOMPLETE\n", healthy+"N3_HARNESS_INCOMPLETE\n",
                         healthy+"error[E0308]: mismatched types\n",
                         healthy+'Error: Custom { kind: Other, error: "unrelated peer failure" }\n',
                         healthy.replace("quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions", "processing_guard_tests::fixture")]:
                (directory/"caught.log").write_text(text)
                with self.subTest(text=text[:60]), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory, outcomes, self.semantic_sites())
            (directory/"caught.log").write_text(healthy)
            for change in [lambda p:p["outcomes"].append(copy.deepcopy(p["outcomes"][0])),
                lambda p:p["outcomes"][0].update(log_path="../outside.log"),
                lambda p:p["outcomes"][0]["scenario"]["Mutant"].update(file="crates/nevrotick-engine/src/lib.rs")]:
                bad = copy.deepcopy(outcomes); change(bad)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory, bad, self.semantic_sites())

    def test_quality_caught_causes_require_complete_native_failure_records(self):
        mapping = self.semantic_sites()
        test = "quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            healthy, outcomes = self.caught_fixture(directory)
            prior = ("running 1 test\ntest fixture::healthy ... ok\n"
                     "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")
            for complete in (healthy, prior + healthy):
                (directory / "caught.log").write_text(complete)
                result = MODULE.validate_quality_caught_causes(directory, outcomes, mapping)
                self.assertEqual(len(next(iter(result["checked_caught_logs"].values()))["causes"]), 1)
            section = healthy[healthy.index("---- "):healthy.rindex("\nfailures:\n")]
            foreign = healthy.replace("running 1 test\n", "running 2 tests\n").replace(
                f"test {test} ... FAILED\n", f"test {test} ... FAILED\ntest fixture::peer ... FAILED\n"
            ).replace("0 passed; 1 failed", "0 passed; 2 failed").replace(
                "\n\ntest result:", "\n    fixture::peer\n\ntest result:")
            bad = {
                "section only": section,
                "truncated after assertion": healthy[:healthy.rindex("\nfailures:\n")],
                "truncated before summary": healthy[:healthy.rindex("test result:")],
                "signal": healthy + "error: process did not exit successfully (signal: 9, SIGKILL: kill)\n",
                "abort": healthy + "fatal runtime error: thread stack overflow, aborting\n",
                "duplicate failed row": healthy + f"test {test} ... FAILED\n",
                "duplicate truncated run": healthy + f"running 1 test\ntest {test} ... FAILED\n",
                "truncated later passing block": healthy + "running 1 test\ntest fixture::healthy ... ok\n",
                "inconsistent summary": healthy.replace("0 passed; 1 failed", "1 passed; 1 failed"),
                "wrong running count": healthy.replace("running 1 test\n", "running 2 tests\n"),
                "foreign failed row without section": foreign,
            }
            for label, text in bad.items():
                (directory / "caught.log").write_text(text)
                with self.subTest(label=label), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory, outcomes, mapping)

    def test_quality_semantic_sites_are_bound_to_source_and_reachable_helpers(self):
        mapping=self.semantic_sites()
        test="quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        self.assertTrue(any(site["owner"]=="exact" for site in mapping["tests"][test]))
        constructor="quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields"
        self.assertFalse(any(site["owner"]=="exact" for site in mapping["tests"][constructor]))
        self.assertTrue(all(site["owner"] not in {"facts","run_series","check_report","diagnostic","serve"}
            for sites in mapping["tests"].values() for site in sites))
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            self.write_assertion_sources(root)
            self.assertEqual(MODULE.quality_semantic_assertion_sites(root,self.semantic_policy()),mapping)
            private=root/"crates/nevrotick-engine/src/quality/tests.rs"
            private.write_text("\n"+private.read_text())
            with self.assertRaisesRegex(MODULE.Incomplete,"admitted bytes"):
                MODULE.quality_semantic_assertion_sites(root,self.semantic_policy())

    def test_quality_correct_filename_requires_exact_named_site_and_assertion_kind(self):
        mapping=self.semantic_sites()
        test="quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
        exact=next(site for site in mapping["tests"][test] if site["owner"]=="exact")
        foreign=next(site for site in mapping["tests"][
            "quality::tests::mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields"]
            if site["kind"]=="panic_assertion")
        with tempfile.TemporaryDirectory() as temporary:
            directory=Path(temporary);healthy,outcomes=self.caught_fixture(directory)
            location=f"{exact['source']}:{exact['line']}:{exact['column']}"
            for replacement in [f"{exact['source']}:0:{exact['column']}",
                f"{exact['source']}:999999:{exact['column']}",f"{exact['source']}:{exact['line']}:0",
                f"{exact['source']}:{exact['line']}:999999",
                f"{foreign['source']}:{foreign['line']}:{foreign['column']}"]:
                (directory/"caught.log").write_text(healthy.replace(location,replacement))
                with self.subTest(location=replacement), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
            for changed in [healthy.replace(exact["marker"],"assertion `left != right` failed"),
                            healthy.replace("thread '"+test+"'","thread 'unrelated-worker'")]:
                (directory/"caught.log").write_text(changed)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)

    def test_quality_primary_assertion_allows_only_closed_secondary_product_refusals(self):
        mapping=self.semantic_sites()
        secondary="quality::tests::precision_clamps_have_exact_boundary_and_neighbor_values"
        with tempfile.TemporaryDirectory() as temporary:
            directory=Path(temporary);healthy,outcomes=self.caught_fixture(directory)
            primary="quality::tests::dyadic_calculation_borrows_original_facts_and_assumptions"
            body=healthy.split(f"---- {primary} stdout ----\n",1)[1].rsplit("\nfailures:\n",1)[0].rstrip("\n")
            def mixed(error):
                return self.failed_execution([(primary,body),(secondary,error)])
            for error in ["Error: InvalidAssumption(HistoricalQuantum)","Error: NonFinite(FrequencyWidth)"]:
                (directory/"caught.log").write_text(mixed(error))
                result=MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
                row=next(iter(result["checked_caught_logs"].values()))
                self.assertEqual(len(row["causes"]),1)
                self.assertEqual(len(row["secondary_product_refusals_without_credit"]),1)
            for error in ['Error: Custom { kind: Other, error: "peer fixture failed" }',
                'Error: Custom { kind: TimedOut, error: "peer timed out" }',"Error: Unavailable",
                "Error: InvalidAssumption(UnknownField)","no cause recorded"]:
                (directory/"caught.log").write_text(mixed(error))
                with self.subTest(error=error), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
            (directory/"caught.log").write_text(self.failed_execution([
                (secondary,"Error: InvalidAssumption(HistoricalQuantum)")]))
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_quality_caught_causes(directory,outcomes,mapping)

    def test_quality_external_static_fact_is_source_bound_and_peer_failure_is_rejected(self):
        mapping=self.semantic_sites()
        test="actual_series_quality_retains_signed_intervals_source_and_original_age"
        site=next(site for site in mapping["tests"][test] if site["owner"]=="check_near")
        with tempfile.TemporaryDirectory() as temporary:
            directory=Path(temporary);_,outcomes=self.caught_fixture(directory)
            healthy=self.failed_execution([(test,site["marker"])])
            (directory/"caught.log").write_text(healthy)
            result=MODULE.validate_quality_caught_causes(directory,outcomes,mapping)
            cause=next(iter(result["checked_caught_logs"].values()))["causes"][0]
            self.assertEqual((cause["source"],cause["line"]),(site["source"],site["line"]))
            for marker in [site["marker"].replace("arithmetic","unregistered arithmetic"),
                'Error: Custom { kind: Other, error: "conditional quality changed literal peer claims" }']:
                (directory/"caught.log").write_text(healthy.replace(site["marker"],marker))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_quality_caught_causes(directory,outcomes,mapping)

    def test_quality_score_cannot_count_compiler_harness_timeout_or_missing_inventory(self):
        expected = {}
        rows = [{"scenario":"Baseline","summary":"Success","phase_results":self.phases("Success")}]
        for number in range(20):
            descriptor = self.descriptor("synthetic quality fault "+str(number))
            expected[descriptor["name"]] = descriptor
            rows.append({"scenario":{"Mutant":descriptor},"summary":"CaughtMutant",
                         "phase_results":self.phases("CaughtMutant")})
        outcomes = {"cargo_mutants_version":"27.1.0","outcomes":rows,"total_mutants":20,
                    "caught":20,"missed":0,"unviable":0,"timeout":0,"success":0}
        result = MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE, outcomes, expected,
            MODULE.QUALITY_SUBJECTS["quality"], native_exit_code=0)
        self.assertEqual(result["viable_killed_percent"], 100)
        for change in [lambda p:p.update(caught=19), lambda p:p["outcomes"].pop(),
                       lambda p:p["outcomes"][1].update(summary="Timeout"),
                       lambda p:p["outcomes"][1]["phase_results"][0].update(process_status={"Failure":101})]:
            bad=copy.deepcopy(outcomes); change(bad)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE,bad,expected,
                    MODULE.QUALITY_SUBJECTS["quality"],native_exit_code=0)
        bad=copy.deepcopy(outcomes)
        bad.update(caught=18,missed=2)
        for row in bad["outcomes"][-2:]:
            row.update(summary="MissedMutant",phase_results=self.phases("MissedMutant"))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_counts(MODULE.QUALITY_SCOPE,bad,expected,
                MODULE.QUALITY_SUBJECTS["quality"],native_exit_code=2)

    def test_quality_external_probes_and_constructor_omissions_keep_real_boundaries(self):
        examples = MODULE.quality_contract_examples()
        self.assertEqual(sum(item["code"] is None for item in examples.values()),1)
        self.assertTrue({"E0451","E0616","E0308","E0277","E0515"} <= {item["code"] for item in examples.values()})
        self.assertEqual(examples["quality_assumptions_substitution"]["code"],"E0616")
        self.assertEqual(examples["quality_assumptions_substitution"]["tokens"],["assumptions","private"])
        for owner in ("facts","assumptions"):
            case=examples["quality_"+owner+"_owner_escape"]
            self.assertEqual(case["code"],"E0515")
            self.assertEqual(case["tokens"],["return value","function parameter",owner])
        self.assertIn("facts: TransportFacts, assumptions: &'a DeclaredQualityAssumptions",
                      examples["quality_facts_owner_escape"]["source"])
        self.assertIn("facts: &'a TransportFacts, assumptions: DeclaredQualityAssumptions",
                      examples["quality_assumptions_owner_escape"]["source"])
        self.assertIn("fn retained<'a>",examples["quality_legal"]["source"])
        self.assertIn("evaluate_conditional_quality(facts, assumptions)",examples["quality_legal"]["source"])
        for specimen in examples.values():
            self.assertEqual(specimen["line"],4)
            self.assertTrue(specimen["source"].splitlines()[3])
            self.assertNotIn("execute_series(",specimen["source"])
        source=(ROOT/MODULE.QUALITY_SUBJECTS["quality"]).read_text()
        variants=MODULE.quality_validation_variants(source)
        self.assertEqual(set(variants),{"frequency","limits","assumptions"})
        for body in variants.values():
            self.assertEqual(source.count("candidate.validate()")-body.count("candidate.validate()"),1)
        with self.assertRaises((MODULE.Incomplete,ValueError)):
            MODULE.quality_validation_variants(source.replace("candidate.validate()","candidate.check()"))

    def test_quality_contract_phases_share_the_original_cumulative_wall_budget(self):
        examples=MODULE.quality_contract_examples()
        # Twenty seconds have already elapsed since the caller's original epoch.
        now=[25.0];deadline=305.0;observed=[];calls=[]
        def remaining(actual,maximum):
            observed.append(actual)
            self.assertEqual(actual,deadline)
            value=int(actual-now[0])
            if value<=0:
                raise MODULE.Incomplete("shared contract deadline exhausted")
            return min(maximum,value)
        def run(label,command,timeout,**kwargs):
            calls.append((label,timeout))
            now[0]+=min(timeout,5 if label.startswith("external-") else 80)
            if label=="external-legal-execution":
                return examples["quality_legal"]["success_marker"]
            if label=="quality-validation-execute:healthy":
                return "Q1_VALIDATION_CONTROL:healthy"
            return ""
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary)
            deps=work/"target/debug/deps";deps.mkdir(parents=True)
            (work/"target/debug/libnevrotick_engine.rlib").write_bytes(b"synthetic emitted library")
            (deps/"libgarde-control.rlib").write_bytes(b"synthetic derive dependency")
            (deps/"libtime_domain-control.rlib").write_bytes(b"synthetic rational dependency")
            session=SimpleNamespace(work=work,source=ROOT,stable=work/"stable",run=run,protect=lambda _:None)
            report={}
            with patch.object(MODULE.time,"monotonic",side_effect=lambda:now[0]), \
                 patch.object(MODULE.COMMON,"remaining_timeout",side_effect=remaining), \
                 patch.object(MODULE,"copy_contract_dependencies",side_effect=lambda *args, **kwargs: shutil.copytree(deps, work/"contract-deps")), \
                 patch.object(MODULE.COMMON,"library_artifact",return_value="/work/target/debug/libnevrotick_engine.rlib"):
                MODULE.contracts(session,self.policy(),report,examples=examples,deadline=deadline)
                with self.assertRaisesRegex(MODULE.Incomplete,"shared contract deadline"):
                    MODULE.quality_validation_contracts(session,report,deadline=deadline)
            self.assertTrue(any(label.startswith("quality-validation-") for label,_ in calls))
            self.assertTrue(observed)
            self.assertEqual(set(observed),{deadline})
            self.assertEqual(now[0],deadline)
            self.assertNotIn("quality_validation_controls",report)

    def test_quality_preflight_and_unknown_subject_cannot_launch_a_native_session(self):
        with tempfile.TemporaryDirectory() as temporary:
            args=SimpleNamespace(profile="quality",lane="enumerate",subject="quality",
                output_dir=Path(temporary)/"output",tool_admission_profile="full")
            with patch.object(MODULE,"read_quality_policy",return_value=self.policy()), \
                 patch.object(MODULE.shutil,"disk_usage",return_value=SimpleNamespace(free=5*1024**3-1)), \
                 patch.object(MODULE.COMMON,"Session") as session:
                with self.assertRaisesRegex(MODULE.Incomplete,"five GiB"):
                    MODULE.execute(args,{})
                session.assert_not_called()
                args.subject="library"
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute(args,{})
                session.assert_not_called()



class EngineKeAnalysisRoleTests(unittest.TestCase):
    """Synthetic admission controls only; no product, compiler or analyzer runs."""

    HOST = {
        "ke_core": "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs",
        "ke_association": "crates/nevrotick-engine/src/nts_ke_experiment/association.rs",
        "ke_process": "crates/nevrotick-engine/src/nts_ke_experiment/process.rs",
        "ke_handoff": "crates/nevrotick-engine/src/nts_ke_experiment/handoff.rs",
        "ke_request": "crates/nevrotick-engine/src/nts_ke_experiment/request.rs",
        "ke_fault": "crates/nevrotick-engine/src/nts_ke_experiment/transport_fault.rs",
        "ke_ledger": "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs",
        "ke_retirement": "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/retirement.rs",
        "ke_response": "crates/nevrotick-engine/src/nts_ke_experiment/response.rs",
        "ke_datagram": "crates/nevrotick-engine/src/nts_ke_experiment/datagram.rs",
        "ke_recovery": "crates/nevrotick-engine/src/nts_ke_experiment/recovery.rs",
        "ke_job": "crates/nevrotick-engine/src/nts_ke_experiment/job.rs",
        "ke_preview_target": "crates/nevrotick-engine/src/ke_preview_target.rs",
    }
    PROBE_CALLER = "tools/crypto-cng-lifecycle/src/bin/nts-ke-probe.rs"
    MANAGED_PROTECTED_CASES = (
        "nts_ke_experiment::datagram::tests::managed_protected_attempts_keep_floors_after_failure_and_stop",
        "nts_ke_experiment::datagram::tests::managed_protected_rechecks_definition_after_readiness",
        "nts_ke_experiment::datagram::tests::managed_protected_reply_distinguishes_wire_peer_and_policy_epochs",
        "nts_ke_experiment::datagram::tests::managed_protected_send_uses_the_exact_shared_floor",
        "nts_ke_experiment::managed_preview_tests::managed_protected_abnormal_join_preserves_sealed_configuration",
        "nts_ke_experiment::managed_preview_tests::managed_protected_refuses_preview_downgrade_before_charging",
        "nts_ke_experiment::managed_preview_tests::managed_protected_worker_preserves_committed_definition_through_join",
        "nts_ke_experiment::managed_preview_tests::managed_protected_worker_refuses_replacement_at_both_effects",
    )

    MANAGED_SUCCESSOR_CASES = (
        "nts_ke_experiment::datagram::tests::managed_retired_first_request_allows_one_same_association_successor",
        "nts_ke_experiment::datagram::tests::managed_retired_authentication_refuses_uncommitted_and_invalidated_history",
    )

    MANAGED_DENY_CASES = (
        "nts_ke_experiment::datagram::tests::managed_protected_deny_retires_only_the_exact_authenticated_successor",
        "nts_ke_experiment::datagram::tests::managed_protected_ordinary_follows_rejected_controls_without_renewal",
        "nts_ke_experiment::datagram::tests::managed_retired_deny_commits_only_its_source_and_exact_endpoint",
        "nts_ke_experiment::datagram::tests::managed_mixed_receive_admits_deny_on_final_frame",
        "nts_ke_experiment::datagram::tests::managed_mixed_receive_exhausts_on_invalid_final_frame",
    )

    MANAGED_WORKER_BOUNDARY_CASES = (
        "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::managed_deny_worker_retains_exact_history_without_numerical_projection",
        "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::managed_deny_worker_post_record_stop_preserves_only_once_joined_history",
        "nts_ke_experiment::datagram::reached::bound_endpoint_check_refuses_each_invalid_coordinate",
    )

    MANAGED_WAIT_AND_RATE_CASES = (
        "nts_ke_experiment::datagram::tests::managed_retired_rate_preserves_independent_intervals_and_exact_next_send",
        "nts_ke_experiment::datagram::tests::managed_protected_policy_wait_checks_out_cookie_only_when_due",
        "nts_ke_experiment::datagram::tests::managed_protected_policy_wait_refuses_before_cookie_checkout",
        "nts_ke_experiment::datagram::tests::managed_protected_policy_wait_last_credit_refuses_before_cookie_checkout",
    )

    FINAL_REPLY_CUTOFF_CASES = (
        "processing_guard_tests::pilot_final_guard_time_cutoff_rejects_selected_reply_once",
        "series::tests::reply_final_guard_time_cutoff_keeps_its_actual_receiving_checkpoint",
    )
    PLAIN_CAPTURE_CASES = (
        "processing_guard_tests::unmanaged_send_cutoff_uses_both_clock_readings",
        "source_policy::tests::managed_send_cutoff_uses_both_clock_readings",
        "source_policy::tests::managed_inverted_pre_send_capture_refuses_before_reservation",
        "session::tests::mixed_series_capture_refusal_precedes_retained_healthy_facts_if_profiled",
    )

    RECEIVE_CAPTURE_CASES = (
        "source_policy::tests::managed_plain_receive_cutoff_charges_once_before_policy_effect",
        "source_policy::tests::managed_plain_reversed_receive_before_cutoff_keeps_header_history",
        "nts_ke_experiment::datagram::tests::managed_retired_deny_refuses_receive_capture_at_cutoff",
    )

    RECEIVE_CHRONOLOGY_CASES = (
        "source_policy::tests::managed_plain_reversed_receive_leaves_control_pending_until_ordered_reply",
        "nts_ke_experiment::datagram::tests::managed_retired_deny_refuses_reversed_receive_capture",
        "nts_ke_experiment::datagram::tests::managed_retired_deny_refuses_future_receive_tail",
    )

    ORDINARY_PROTECTED_RECEIVE_CASE = (
        "nts_ke_experiment::datagram::tests::ordinary_protected_reversed_receive_bracket_cannot_commit")
    HEADER_AGE_CASE = (
        "source_policy::tests::managed_header_writes_receive_to_commit_age_only_for_ordered_capture")
    KNOWN_RECEIVE_DEBIT_CASE = (
        "nts_ke_experiment::datagram::tests::known_protected_datagram_with_invalid_capture_charges_frame_and_bytes")

    PROTECTED_RECEIVE_EFFECT_CASES = (
        "nts_ke_experiment::datagram::tests::retired_completion_commit_respects_acceptance_and_expiry",
        "nts_ke_experiment::datagram::tests::recovery_tests::reversed_final_zero_cookie_capture_cannot_schedule_replacement",
        "nts_ke_experiment::datagram::tests::recovery_tests::final_zero_cookie_capture_at_cutoff_cannot_schedule_replacement",
    )

    class SessionReached(Exception):
        """A healthy admission reached Session; stop before any execution."""

    def policy_fixture(self):
        """Independent limits; synthetic 560/276 ordinary and 90 annotated Rustdoc names."""
        critical = ["crates/nevrotick-engine/src/lib.rs", *self.HOST.values(),
            "crates/nevrotick-engine/src/nts_ke_experiment/profile.rs",
            "crates/nevrotick-engine/src/nts_ke_experiment/transport.rs",
            self.PROBE_CALLER,
            "crates/nevrotick-engine/src/nts_ke_experiment/managed_preview.rs",
            "crates/nevrotick-engine/src/source_policy.rs"]
        names = sorted([f"ordinary_fixture_{index:03}" for index in range(274)] +
                       [f"nts_ke_experiment::fixture_case_{index:02}" for index in range(245)] +
                       ["nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
                        "numerics_tests::facade_protected_numerics_send_after_wall_step_is_not_omitted",
                        *self.MANAGED_PROTECTED_CASES, *self.MANAGED_SUCCESSOR_CASES,
                        *self.MANAGED_DENY_CASES, *self.MANAGED_WORKER_BOUNDARY_CASES,
                        *self.MANAGED_WAIT_AND_RATE_CASES, *self.FINAL_REPLY_CUTOFF_CASES,
                        *self.PLAIN_CAPTURE_CASES, *self.RECEIVE_CAPTURE_CASES, *self.RECEIVE_CHRONOLOGY_CASES,
                        self.KNOWN_RECEIVE_DEBIT_CASE, self.HEADER_AGE_CASE, self.ORDINARY_PROTECTED_RECEIVE_CASE,
                        *self.PROTECTED_RECEIVE_EFFECT_CASES])
        docs = sorted([f"src/lib.rs - Fixture::entered_{index:02} (line 100)" for index in range(87)]
                      + ["src/lib.rs - Example::checked (line 10)",
                         "src/lib.rs - Example::private (line 20) - compile fail",
                         "src/lib.rs - Example::prepared (line 30) - compile"])
        return {
            "schema_version": 1, "package": "nevrotick-engine",
            "admission_status": "admitted-engine-ke-source-analysis",
            "subjects": self.HOST.copy(), "critical_files": critical,
            "features": ["ke-association-prototype"], "test_workspace": False,
            "tool_admission_profile": "stable-core",
            "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
            "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 3072,
            "baseline_wall_seconds": 300, "coverage_wall_seconds": 300,
            "enumeration_wall_seconds": 240, "mutation_wall_seconds": 1200,
            "mutation_build_seconds": 90, "mutation_test_seconds": 60,
            "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
            "mutation_jobs": 6, "mutation_jobserver_tasks": 0,
            "mutation_process_limit": 512, "mutation_message_format": "json",
            "minimum_free_bytes": 7 * 1024**3, "stop_free_bytes": 4 * 1024**3,
            "owned_output_maximum_allocated_bytes": 3 * 1024**3,
            "owned_output_maximum_entries": 32768, "mutation_equivalences": [],
            "campaigns_admitted": False, "mutation_inventories": {},
            "binding": {"candidate_sha256": "a" * 64, "source_file_count": 2,
                "inputs_sha256": {name: "b" * 64 for name in MODULE.ENGINE_KE_BOUND_INPUTS},
                "test_inventory": names, "doctest_inventory": docs, "source_context_sha256": "c" * 64,
                "ordinary_evidence": {"path": "/synthetic/ordinary.json", "sha256": "d" * 64},
                "root_graph_evidence": {"path": "/synthetic/graph.json", "sha256": "e" * 64}},
        }

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / "source"
        self.root.mkdir(mode=0o700)
        self.policy = self.policy_fixture()
        for name in MODULE.ENGINE_KE_BOUND_INPUTS:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("Synthetic admitted source marker; never compiled.\n")
        critical = self.policy["critical_files"]
        engine = [name for name in critical if name != self.PROBE_CALLER]
        manifest = ("critical_files = " + json.dumps(critical) + "\nsecurity_modules = [\n" +
            '{package="nevrotick-engine",files=' + json.dumps(engine) + '},\n' +
            '{package="nevrotick-crypto-cng-lifecycle",files=' + json.dumps([self.PROBE_CALLER]) + '},\n]\n')
        (self.root / "policy/verification-manifest.toml").write_text(manifest)
        self.identity = {name: MODULE.PRODUCT.sha256(self.root / name)
                         for name in ("Cargo.lock", "crates/nevrotick-engine/Cargo.toml")}
        binding = self.policy["binding"]
        binding["candidate_sha256"] = independent_digest(self.identity)
        binding["inputs_sha256"] = {name: MODULE.PRODUCT.sha256(self.root / name)
                                    for name in MODULE.ENGINE_KE_BOUND_INPUTS}
        graph = self.base / "root_graph_evidence.json"
        graph.write_text('{"scope":"synthetic graph; no product evidence"}\n')
        binding["root_graph_evidence"] = {"path": str(graph), "sha256": MODULE.PRODUCT.sha256(graph)}
        self.write_ep1_reference()
        self.args = SimpleNamespace(profile="ke_association", lane="baseline", subject="all",
            tool_admission_profile="stable-core", output_dir=self.base / "new-output", vendor_dir=self.base / "vendor")

    def write_ep1_reference(self):
        """Provide independently named, explicitly synthetic successful lane documents."""
        binding = self.policy["binding"]
        path = self.base / "ordinary_evidence.json"
        prefix = ["cargo", "test", "--package", "nevrotick-engine", "--frozen", "--jobs", "6",
                  "--features", "ke-association-prototype"]
        lanes = []
        for lane, selectors, names in (
            ("ep1-ordinary", ["--lib", "--tests"], binding["test_inventory"]),
            ("ep1-doctests", ["--doc"], binding["doctest_inventory"]),
        ):
            raw = path.with_name(lane + ".json")
            output = self.execution(names)
            cases = names
            if lane == "ep1-ordinary":
                binary = "unittests src/lib.rs"
                output = f"     Running {binary} (/work/target/debug/synthetic-engine)\n" + output
                cases = [{"package": "nevrotick-engine", "binary": binary, "name": name} for name in names]
            raw.write_text(json.dumps({"command": [*prefix, *selectors, "--", "--test-threads=1"],
                                       "exit_code": 0, "failure": None, "output": output}))
            lanes.append({"lane": lane, "sha256": MODULE.PRODUCT.sha256(raw),
                          "exit_code": 0, "failure": None, "tests_executed": len(names), "cases": cases})
        receipt = {"status": "passed", "mode": "combined", "inputs": self.identity,
                   "candidate_sha256": independent_digest(self.identity), "lanes": lanes}
        path.write_text(json.dumps(receipt))
        binding["ordinary_evidence"] = {"path": str(path), "sha256": MODULE.PRODUCT.sha256(path)}

    def invoke(self, *, accepted=False, free=7 * 1024**3):
        """Exercise real pre-Session guards with assumed prior product admission."""
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        dependency = {"engine_ke_association": {"rows": {}}}
        with patch.object(MODULE, "ROOT", self.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=(dependency, self.identity)), \
                patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=free)), \
                patch.object(MODULE.COMMON, "Session", side_effect=self.SessionReached) as session:
            with self.assertRaises(self.SessionReached if accepted else MODULE.Incomplete):
                MODULE.execute_engine_ke_profile(self.args, {})
            if accepted:
                session.assert_called_once()
            else:
                session.assert_not_called()

    def test_closed_scope_features_and_gates_keep_healthy_neighbor(self):
        self.assertEqual(MODULE.ENGINE_KE_SCOPE.select("all"), self.HOST)
        MODULE.validate_engine_ke_policy(self.policy)
        for field, value in (("features", []), ("features", ["foreign"]),
            ("features", ["ke-association-prototype", "foreign"]), ("test_workspace", True),
            ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
            ("jobs", 12), ("mutation_jobs", 8), ("mutation_jobs", 10),
            ("mutation_jobs", True), ("critical_files", self.policy["critical_files"][:-1])):
            bad = copy.deepcopy(self.policy); bad[field] = value
            with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_policy(bad)
        for scope in (MODULE.COMMON.Scope("foreign", tuple(self.HOST.items()), MODULE.ENGINE_KE_POLICY),
                      MODULE.COMMON.Scope("nevrotick-engine", tuple(self.HOST.items()), "policy/foreign.json"),
                      MODULE.COMMON.Scope("nevrotick-engine", tuple(self.HOST.items())[:-1], MODULE.ENGINE_KE_POLICY)):
            with self.subTest(scope=scope), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.require_engine_ke_role(scope, self.policy)
        self.invoke(accepted=True)

    def test_source_helper_and_candidate_pin_drift_refuse_before_session(self):
        cutoff_sources = ("crates/nevrotick-engine/tests/private/processing_guards.rs",
                          "crates/nevrotick-engine/tests/private/series.rs")
        for name in ("scripts/portable-analysis.py", "scripts/analysis-controls.py",
                     "scripts/check-analysis.py",
                     "crates/nevrotick-engine/src/nts_ke_experiment/association.rs",
                     "crates/nevrotick-engine/src/nts_ke_experiment/recovery/numerics_tests.rs",
                     *cutoff_sources):
            path = self.root / name; original = path.read_bytes()
            path.write_bytes(original + b"changed bytes\n")
            with self.subTest(source=name): self.invoke()
            path.write_bytes(original)
        for name in cutoff_sources:
            bad = copy.deepcopy(self.policy)
            del bad["binding"]["inputs_sha256"][name]
            with self.subTest(missing=name), self.assertRaisesRegex(MODULE.Incomplete, "source/helper inputs differ"):
                MODULE.validate_engine_ke_policy(bad)
        original = self.policy["binding"]["candidate_sha256"]
        self.policy["binding"]["candidate_sha256"] = "f" * 64
        self.invoke()
        self.policy["binding"]["candidate_sha256"] = original
        self.invoke(accepted=True)

    def test_reference_byte_drift_refuses_before_session(self):
        for field in ("ordinary_evidence", "root_graph_evidence"):
            path = Path(self.policy["binding"][field]["path"]); original = path.read_bytes()
            path.write_bytes(original + b" ")
            with self.subTest(reference=field): self.invoke()
            path.write_bytes(original)
        self.invoke(accepted=True)

    def test_seven_gib_startup_boundary_precedes_product_and_evidence_admission(self):
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        dependency = {"engine_ke_association": {"rows": {}}}
        for free, accepted in ((7516192767, False), (7516192768, True)):
            with self.subTest(free=free), patch.object(MODULE, "ROOT", self.root), \
                    patch.object(MODULE.PRODUCT, "admit_product", return_value=(dependency, self.identity)) as product, \
                    patch.object(MODULE, "admit_engine_ke_evidence", wraps=MODULE.admit_engine_ke_evidence) as evidence, \
                    patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=free)), \
                    patch.object(MODULE.COMMON, "Session", side_effect=self.SessionReached) as session:
                if accepted:
                    with self.assertRaises(self.SessionReached):
                        MODULE.execute_engine_ke_profile(self.args, {})
                    product.assert_called_once()
                    evidence.assert_called_once()
                    session.assert_called_once()
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, "seven GiB"):
                        MODULE.execute_engine_ke_profile(self.args, {})
                    product.assert_not_called()
                    evidence.assert_not_called()
                    session.assert_not_called()

    def test_six_worker_fallback_refuses_eight_before_session(self):
        self.args.lane = "mutation"
        self.args.subject = "ke_core"
        self.policy["campaigns_admitted"] = True
        self.policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64} for name in self.HOST}
        self.policy["mutation_jobs"] = 6
        self.invoke(accepted=True)
        for workers in (8, 10, True, 6.0):
            self.policy["mutation_jobs"] = workers
            with self.subTest(workers=workers):
                self.invoke()
        self.policy["mutation_jobs"] = 6
        self.invoke(accepted=True)

    def test_ke_zero_extra_tokens_are_required_before_session(self):
        self.invoke(accepted=True)
        for value in (6, True, False, 0.0, "0", None, -1):
            self.policy["mutation_jobserver_tasks"] = value
            with self.subTest(tokens=value):
                self.invoke()
        self.policy.pop("mutation_jobserver_tasks")
        self.invoke()
        self.policy["mutation_jobserver_tasks"] = 0
        self.invoke(accepted=True)

    def test_current_ke_policy_selects_zero_extra_tokens_and_keeps_campaign_closed(self):
        policy = json.loads((ROOT / MODULE.ENGINE_KE_POLICY).read_text())
        self.assertIs(type(policy.get("mutation_jobserver_tasks")), int)
        self.assertEqual(policy["mutation_jobserver_tasks"], 0)
        self.assertEqual(policy["mutation_jobs"], 6)
        self.assertIs(type(policy["minimum_free_bytes"]), int)
        self.assertEqual(policy["minimum_free_bytes"], 7516192768)
        self.assertIs(type(policy["owned_output_maximum_allocated_bytes"]), int)
        self.assertEqual(policy["owned_output_maximum_allocated_bytes"], 3221225472)
        self.assertEqual(policy["stop_free_bytes"], 4294967296)
        self.assertEqual(policy["mutation_inventories"], {})
        self.assertIs(policy["campaigns_admitted"], False)
        self.assertEqual(policy["mutation_equivalences"], [])
        self.assertTrue(MODULE.COMMON.engine_ke_equivalences_valid(policy))
        self.assertEqual(
            MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.ENGINE_KE_SCOPE, policy),
            [],
        )
        # The historical E9 fixture retains its original binding and proof.
        historical = {'binding': {'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d'},
         'mutation_equivalences': [{'id': 'KE-RETURN-POOL-CAPACITY-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:774:17: '
                                                       "replace || with && in AssociationOwner<'a>::return_unused",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::return_unused",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 737, 'column': 5},
                                                                     'end': {'line': 787, 'column': 6}}},
                                               'span': {'start': {'line': 774, 'column': 17},
                                                        'end': {'line': 774, 'column': 19}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'The complete admitted return domain includes genuine Reserved, '
                                                 'Preparing and Prepared tickets, failed preparation, exact '
                                                 'pool-byte equality, cancellation and expiry; it does not assume '
                                                 'successful encoding. After the unchanged guards, reservation has '
                                                 'removed one exact pool entry and no admitted writer can shrink or '
                                                 'replace that pool before return. Its remaining length is at most '
                                                 'seven and its capacity still exceeds that length; its '
                                                 'reserved-plus-available byte sum remains within 131072. The '
                                                 'changed connector combines only false predicates. Both forms '
                                                 'preserve the same returned allocation, any retired UID, spent '
                                                 'attempts, original deadline and error paths.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-RETURN-POOL-CAPACITY-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-RETURN-POOL-BYTES-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:773:17: '
                                                       "replace || with && in AssociationOwner<'a>::return_unused",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::return_unused",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 737, 'column': 5},
                                                                     'end': {'line': 787, 'column': 6}}},
                                               'span': {'start': {'line': 773, 'column': 17},
                                                        'end': {'line': 773, 'column': 19}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'After the unchanged ticket, Published-control and pre-invocation '
                                                 'phase checks, the exact reserved cookie was removed from a checked '
                                                 'pool of at most eight entries. No admitted operation changes that '
                                                 'pool while the slot remains reserved. Available count is therefore '
                                                 'at most seven, available-plus-reserved payload remains at most '
                                                 '131072 bytes, and removal preserves an allocated slot beyond the '
                                                 'current length. All three rejection predicates are false, '
                                                 'including strict byte excess at exact equality. Both operator '
                                                 'forms move back the same cookie without copying, exposing it, '
                                                 'refunding work or renewing its original expiry. Earlier refusal '
                                                 'paths are unchanged.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-RETURN-POOL-BYTES-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-SEND-PREDECESSOR-VALUE-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:854:17: '
                                                       "replace || with && in AssociationOwner<'a>::invoke_sender",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::invoke_sender",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 813, 'column': 5},
                                                                     'end': {'line': 907, 'column': 6}}},
                                               'span': {'start': {'line': 854, 'column': 17},
                                                        'end': {'line': 854, 'column': 19}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'The stored predecessor set is filtered before reservation selects '
                                                 'the exact cookie. It is immutable thereafter; install_previous '
                                                 'requires the untouched reservation allowance and cannot replace it '
                                                 'once any ticket has been issued. Consequently the selected '
                                                 'Prepared cookie is not a member of the stored predecessor, whether '
                                                 'that set is empty, arbitrary or complete. Local count, capacity, '
                                                 'encoded-byte and local-membership predicates are also false. '
                                                 'Replacing the final connector with AND therefore changes no error, '
                                                 'archive move, callback, no-reuse or lifecycle outcome. This '
                                                 'argument does not certify completeness or provenance of the '
                                                 'predecessor set; actual consuming retirement and checked '
                                                 'projection enforce that separate producer contract.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-SEND-PREDECESSOR-VALUE-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-SEND-ARCHIVE-VALUE-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:853:17: '
                                                       "replace || with && in AssociationOwner<'a>::invoke_sender",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::invoke_sender",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 813, 'column': 5},
                                                                     'end': {'line': 907, 'column': 6}}},
                                               'span': {'start': {'line': 853, 'column': 17},
                                                        'end': {'line': 853, 'column': 19}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'Reservation filters every value in the current local used archive '
                                                 'before moving its selected cookie into the single occupied slot. '
                                                 'No other invocation can append a local archive entry while that '
                                                 'ticket owns the slot, and pre-send return consumes the ticket '
                                                 'without refunding its checkout. Thus the local membership '
                                                 'predicate is false at the unchanged Prepared/currentness guard. '
                                                 'Count, capacity, encoded-byte and predecessor predicates are also '
                                                 'false in that domain. Changing this connector only changes pure '
                                                 'bounded reads; both forms archive the same cookie before callback '
                                                 'exposure and preserve all errors, receipt absence, budgets and '
                                                 'completion custody.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-SEND-ARCHIVE-VALUE-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-SEND-ARCHIVE-CAPACITY-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:852:17: '
                                                       "replace || with && in AssociationOwner<'a>::invoke_sender",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::invoke_sender",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 813, 'column': 5},
                                                                     'end': {'line': 907, 'column': 6}}},
                                               'span': {'start': {'line': 852, 'column': 17},
                                                        'end': {'line': 852, 'column': 19}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'On the whole admitted Prepared-ticket domain, including the last '
                                                 'checkout, local archive capacity is at least 32 and pre-push '
                                                 'length is at most 31. Return and failure never refund a '
                                                 'reservation or shrink this archive. The encoded-byte ceiling makes '
                                                 'excess false; reservation and immutable predecessor installation '
                                                 'make both membership predicates false. The changed connector '
                                                 'therefore joins false conditions only. Neither form changes the '
                                                 'moved cookie, Invoked transition, callback arguments, '
                                                 'unknown-completion custody or subsequent original-control checks. '
                                                 'This is a local-context argument: predecessor history remains a '
                                                 'separate bounded owner, not extra entries in the local Vec.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-SEND-ARCHIVE-CAPACITY-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-SEND-ARCHIVE-BYTES-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:851:17: '
                                                       "replace || with && in AssociationOwner<'a>::invoke_sender",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::invoke_sender",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 813, 'column': 5},
                                                                     'end': {'line': 907, 'column': 6}}},
                                               'span': {'start': {'line': 851, 'column': 17},
                                                        'end': {'line': 851, 'column': 19}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'Only a checked same-owner Prepared packet reaches the archive '
                                                 'predicate. The current checkout is one of at most 32 irreversible '
                                                 'local reservations and is not archived yet, so existing local '
                                                 'length is at most 31 and capacity is at least 32. Every sendable '
                                                 'cookie fits the 4096-byte algorithm-15 packet after at least 128 '
                                                 'bytes of header, UID, cookie header and authenticator overhead, '
                                                 'hence each is at most 3968 bytes and the post-push local total is '
                                                 'at most 126976, strictly below 131072. Reservation excludes both '
                                                 'local and stored-predecessor members. All five local/predecessor '
                                                 'rejection predicates are false. Both variants archive before the '
                                                 'same callback and preserve every completion, failure, cancellation '
                                                 'and no-reuse disposition.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-SEND-ARCHIVE-BYTES-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-SEND-ARCHIVE-BYTES-EQUALITY',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:851:26: '
                                                       "replace > with == in AssociationOwner<'a>::invoke_sender",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::invoke_sender",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 813, 'column': 5},
                                                                     'end': {'line': 907, 'column': 6}}},
                                               'span': {'start': {'line': 851, 'column': 26},
                                                        'end': {'line': 851, 'column': 27}},
                                               'replacement': '==',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'The unchanged checked packet formatter admits at most 3968 cookie '
                                                 'bytes per actual prepared send and at most 32 such cookies enter '
                                                 'one local archive. Its checked post-push sum is at most 126976, so '
                                                 'both sum > 131072 and sum == 131072 are false without overflow. '
                                                 'Earlier ticket, phase, expiry and preparation refusals remain '
                                                 'unchanged. The mutation supplies no permission to narrow the raw '
                                                 'history-bounds checker or return-pool domain: those independently '
                                                 'admit exact 131072-byte data and retain their own negative and '
                                                 'healthy controls.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-SEND-ARCHIVE-BYTES-EQUALITY',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-SEND-ARCHIVE-BYTES-INCLUSIVE',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:851:26: '
                                                       "replace > with >= in AssociationOwner<'a>::invoke_sender",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::invoke_sender",
                                                            'return_type': '-> Result<(), Failure>',
                                                            'span': {'start': {'line': 813, 'column': 5},
                                                                     'end': {'line': 907, 'column': 6}}},
                                               'span': {'start': {'line': 851, 'column': 26},
                                                        'end': {'line': 851, 'column': 27}},
                                               'replacement': '>=',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'For every admitted Prepared invocation, including the 32nd local '
                                                 'reservation, the packet/attempt bounds give a post-push local '
                                                 'archive maximum of 126976 bytes. Both sum > 131072 and sum >= '
                                                 '131072 are therefore false. The same cookie and packet reach the '
                                                 'same callback after one archive move; no error, budget, expiry, '
                                                 'cancellation or completion disposition differs. The bound concerns '
                                                 'only this formatter-produced local archive, not arbitrary bounded '
                                                 'data, total predecessor-plus-current memory, TLS traffic, native '
                                                 'allocations or a wider future profile.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-SEND-ARCHIVE-BYTES-INCLUSIVE',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}},
                                   {'id': 'KE-RECEIVE-STABLE-SLOT-OR',
                                    'candidate_sha256': '0b7be101405f7cceae21f6ded612699bbe4bab3e9772da02adc90740cc1dbf8d',
                                    'source_sha256': 'e6625bb2f5e4acefd21f2e488ea7160e03087cb6b42fe130fb062258bb33d08a',
                                    'mutant': {'name': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:1084:40: '
                                                       "replace || with && in AssociationOwner<'a>::receive_reply",
                                               'package': 'nevrotick-engine',
                                               'file': 'crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs',
                                               'function': {'function_name': "AssociationOwner<'a>::receive_reply",
                                                            'return_type': '-> Result<Receipt, Failure>',
                                                            'span': {'start': {'line': 1003, 'column': 5},
                                                                     'end': {'line': 1107, 'column': 6}}},
                                               'span': {'start': {'line': 1084, 'column': 40},
                                                        'end': {'line': 1084, 'column': 42}},
                                               'replacement': '&&',
                                               'genre': 'BinaryOperator'},
                                    'requirement': 'REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27',
                                    'rationale': 'The complete receive_reply input domain is guarded at the actual '
                                                 'charged receiver. Its initial Published guard requires Pending and '
                                                 'captures that active serial, UID, wire token, allowance and send '
                                                 'observation before charging. Successful stage_current_reply and '
                                                 'stage_replacement only immutably borrow the owner, use its own '
                                                 'keys and checked issued-UID prefix, and expose no effectful '
                                                 'publisher, await or owner-mutating callback. Staging errors may '
                                                 'retire work but exit before the final comparison. The final fresh '
                                                 'Published guard rejects stop or expiry before entering the inline '
                                                 'publication closure. Whenever the changed expression executes, the '
                                                 'actual serial and Pending phase still match the captured values, '
                                                 'so both mismatch predicates are false. Both variants atomically '
                                                 'publish the same cookies/receipt once; physical timing and native '
                                                 'completion are not established.',
                                    'proof': {'path': 'tasks/engine-ke-association-equivalences.md',
                                              'sha256': '6163769960cf33bf366e78fc16b5888f6a301224a11fa05f93f0eaaad6a10409',
                                              'section': 'KE-RECEIVE-STABLE-SLOT-OR',
                                              'reviewer': '/root; /root/kod_owner_source_design; '
                                                          '/root/v10_execution_admission_review',
                                              'verdict': 'independently-justified-equivalent'}}]}
        self.assertTrue(MODULE.COMMON.engine_ke_equivalences_valid(historical))
        legacy = copy.deepcopy(historical)
        legacy_candidate = "964a9573143e5668a63d264bb44906fa103d5b4c3ddf1fc82cecea33012c5468"
        legacy["binding"]["candidate_sha256"] = legacy_candidate
        for entry in legacy["mutation_equivalences"]:
            entry["candidate_sha256"] = legacy_candidate
        self.assertFalse(MODULE.COMMON.engine_ke_equivalences_valid(legacy))

    def test_storage_usage_charges_actual_work_tmp_allocated_blocks(self):
        output = self.base / "scratch-output"
        temporary = output / "work/tmp"
        temporary.mkdir(mode=0o700, parents=True)
        before = MODULE.address_storage_usage(output)
        payload = temporary / "worker-target"
        payload.write_bytes(b"x" * 65536)
        allocated = payload.stat().st_blocks * 512
        self.assertGreater(allocated, 0)
        after = MODULE.address_storage_usage(output)
        self.assertEqual(after["entries"], before["entries"] + 1)
        self.assertGreaterEqual(after["allocated_bytes"] - before["allocated_bytes"], allocated)

    def test_ke_storage_checkpoint_accepts_below_three_gib_and_refuses_at_or_above(self):
        output = self.base / "checkpoint-output"
        output.mkdir(mode=0o700)
        session = SimpleNamespace(output=output)
        for allocated, accepted in ((3221225471, True), (3221225472, False), (3221225473, False)):
            report = {}
            with self.subTest(allocated=allocated), \
                    patch.object(MODULE, "address_storage_usage", return_value={"allocated_bytes": allocated, "entries": 1}), \
                    patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=7516192768)):
                if accepted:
                    MODULE.address_storage_checkpoint(session, self.policy, report, "ke-mutation", "after")
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, "allocated cap"):
                        MODULE.address_storage_checkpoint(session, self.policy, report, "ke-mutation", "after")
            expected = {"lane": "ke-mutation", "phase": "after", "allocated_bytes": allocated,
                        "entries": 1, "available_bytes": 7516192768}
            self.assertEqual(report["storage_samples"], [expected])
            self.assertEqual(json.loads((output / "progress.json").read_text())["storage_samples"], [expected])

    def test_address_storage_neighbor_preserves_five_gib_and_768_mib_limits(self):
        policy = MODULE.ADDRESS_FIXED_POLICY
        self.assertEqual(policy["minimum_free_bytes"], 5368709120)
        self.assertEqual(policy["stop_free_bytes"], 4294967296)
        self.assertEqual(policy["owned_output_maximum_allocated_bytes"], 805306368)
        self.assertEqual(policy["owned_output_maximum_entries"], 32768)
        output = self.base / "address-checkpoint-output"
        output.mkdir(mode=0o700)
        session = SimpleNamespace(output=output)
        for allocated, accepted in ((805306367, True), (805306368, False)):
            report = {}
            with self.subTest(allocated=allocated), \
                    patch.object(MODULE, "address_storage_usage", return_value={"allocated_bytes": allocated, "entries": 1}), \
                    patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5368709120)):
                if accepted:
                    MODULE.address_storage_checkpoint(session, policy, report, "address", "after")
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, "allocated cap"):
                        MODULE.address_storage_checkpoint(session, policy, report, "address", "after")
            self.assertEqual(report["storage_samples"][-1]["allocated_bytes"], allocated)

    def test_owned_contract_lane_requires_exact_profile_subject_and_stable_role_before_session(self):
        self.args.lane = "contracts"
        for subject in ("all", "ke_core", [], True):
            self.args.subject = subject
            with self.subTest(subject=subject): self.invoke()
        self.args.subject = "ke_process"
        self.args.tool_admission_profile = "full"
        self.invoke()
        self.args.tool_admission_profile = "stable-core"
        self.args.profile = "legacy"
        self.invoke()
        self.args.profile = "ke_association"
        self.invoke(accepted=True)
        self.args.subject = "ke_job"
        self.invoke(accepted=True)

    def test_provider_contract_lane_finishes_custody_without_claiming_broader_qualification(self):
        self.args.lane = "contracts"
        self.args.subject = "ke_process"
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        support = self
        finished = []
        report = {}
        class Session:
            def __init__(self, root, *_arguments):
                self.source, self.product_identity = root, support.identity
            def finish(self):
                support.assertFalse(report["provider_claim_contracts_passed"])
                finished.append(True)
        with patch.object(MODULE, "ROOT", self.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=7 * 1024**3)), \
                patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE, "engine_ke_provider_contracts") as contracts:
            MODULE.execute_engine_ke_profile(self.args, report)
        contracts.assert_called_once()
        self.assertEqual(finished, [True])
        self.assertTrue(report["provider_claim_contracts_passed"])
        self.assertTrue(report["selected_lane_passed"])
        for key in ("qualification_passed", "contracts_qualification_passed",
                    "selected_host_metrics_passed", "selected_host_qualification_passed"):
            self.assertFalse(report[key], key)

    def test_history_contract_lane_publishes_only_after_finish_and_reference_bookends(self):
        self.args.lane, self.args.subject = "contracts", "ke_job"
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        support = self
        for failure in (None, "finish", "reference"):
            report, finished = {}, []
            class Session:
                def __init__(self, root, *_arguments):
                    support.assertFalse(report["provider_claim_contracts_passed"])
                    support.assertFalse(report["protected_history_contracts_passed"])
                    self.source, self.product_identity = root, support.identity
                def finish(self):
                    support.assertFalse(report["protected_history_contracts_passed"])
                    finished.append(True)
                    if failure == "finish":
                        raise MODULE.Incomplete("synthetic final custody refusal")
            evidence = [{}, {"changed": True}] if failure == "reference" else [{}, {}]
            with self.subTest(failure=failure), patch.object(MODULE, "ROOT", self.root), \
                    patch.object(MODULE.PRODUCT, "admit_product", return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                    patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=7 * 1024**3)), \
                    patch.object(MODULE.COMMON, "Session", Session), \
                    patch.object(MODULE, "admit_engine_ke_evidence", side_effect=evidence), \
                    patch.object(MODULE, "engine_ke_provider_contracts") as provider, \
                    patch.object(MODULE, "engine_ke_history_contracts") as history:
                if failure is None:
                    MODULE.execute_engine_ke_profile(self.args, report)
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.execute_engine_ke_profile(self.args, report)
            history.assert_called_once()
            provider.assert_not_called()
            self.assertEqual(finished, [True])
            self.assertEqual(report["protected_history_contracts_passed"], failure is None)
            self.assertFalse(report["provider_claim_contracts_passed"])
            for key in ("qualification_passed", "contracts_qualification_passed", "selected_host_qualification_passed"):
                self.assertFalse(report.get(key, False), key)

    def test_premature_campaign_and_empty_review_refuse_before_session(self):
        for lane in ("mutation", "all"):
            self.args.lane = lane
            with self.subTest(lane=lane): self.invoke()
        self.policy["campaigns_admitted"] = True
        self.invoke()
        self.args.lane = "baseline"; self.policy["campaigns_admitted"] = False
        self.invoke(accepted=True)

    def test_malformed_complete_inventory_cannot_admit_campaign(self):
        self.policy["campaigns_admitted"] = True
        for record in ({}, {"count": 0, "sha256": "a" * 64},
                       {"count": True, "sha256": "a" * 64}, {"count": 1, "sha256": "bad"}):
            self.policy["mutation_inventories"] = {name: copy.deepcopy(record) for name in self.HOST}
            with self.subTest(record=record), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_policy(self.policy, campaign=True)

    def phases(self, *, feature=True):
        """Pinned native Cargo argument format; synthetic phase observations only."""
        return {"summary": "Success", "phase_results": [
            {"phase": phase, "argv": ["cargo", "test", *(["--no-run"] if phase == "Build" else []),
                "--verbose", "--package=nevrotick-engine@0.0.0",
                *(["--features=ke-association-prototype"] if feature else []),
                "--message-format=json", "--frozen"], "duration": 0.01, "process_status": "Success"}
            for phase in ("Build", "Test")]}

    def test_emitted_feature_argv_requires_exact_pinned_order_and_legacy_stays_closed(self):
        healthy = self.phases()
        MODULE.COMMON.mutation_phases(MODULE.ENGINE_KE_SCOPE, healthy)
        MODULE.COMMON.mutation_phases(MODULE.SCOPE, self.phases(feature=False))
        for change in ("omitted", "foreign", "all", "order"):
            bad = copy.deepcopy(healthy)
            for phase in bad["phase_results"]:
                args = phase["argv"]; index = args.index("--features=ke-association-prototype")
                if change == "omitted": del args[index]
                elif change == "foreign": args[index] = "--features=foreign"
                elif change == "all": args[index] = "--all-features"
                else: args.append(args.pop(index))
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_phases(MODULE.ENGINE_KE_SCOPE, bad)
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.mutation_phases(MODULE.SCOPE, healthy)

    def test_campaign_effect_boundary_refuses_before_any_execution(self):
        tools = Path("/owned/synthetic-tools")
        command = [str(tools / "cargo-mutants"), "mutants", "--package", "nevrotick-engine",
            "--file", self.HOST["ke_association"], "--test-workspace", "false", "--cap-lints", "false",
            "--jobs", "6", "--jobserver", "true", "--jobserver-tasks", "0",
            "--cargo-arg=--message-format=json", "--features", "ke-association-prototype",
            "--baseline", "run", "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/ke_association"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", command,
                expected_jobs=6, expected_message_format="json", policy=self.policy)
        self.policy["campaigns_admitted"] = True
        self.policy["mutation_inventories"] = {name: {"count": 1, "sha256": "a" * 64} for name in self.HOST}
        MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", command,
            expected_jobs=6, expected_message_format="json", policy=self.policy)
        extra_tokens = command.copy()
        extra_tokens[extra_tokens.index("--jobserver-tasks") + 1] = "6"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", extra_tokens,
                expected_jobs=6, expected_message_format="json", policy=self.policy)
        # A coherent caller/command pair must still obey the actual role policy.
        for foreign_jobs in (8, 10):
            substituted = command.copy()
            for flag in ("--jobs", "--jobserver-tasks"):
                substituted[substituted.index(flag) + 1] = str(foreign_jobs)
            with self.subTest(foreign_jobs=foreign_jobs), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_campaign_command(
                    MODULE.ENGINE_KE_SCOPE, tools, "ke_association", substituted,
                    expected_jobs=foreign_jobs, expected_message_format="json", policy=self.policy)
        human = command.copy(); human.remove("--cargo-arg=--message-format=json")
        with self.subTest(foreign_message_format="human"), self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_campaign_command(
                MODULE.ENGINE_KE_SCOPE, tools, "ke_association", human,
                expected_jobs=6, expected_message_format="human", policy=self.policy)
        bad = command.copy(); bad[bad.index("--test-workspace") + 1] = "true"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_campaign_command(MODULE.ENGINE_KE_SCOPE, tools, "ke_association", bad,
                expected_jobs=6, expected_message_format="json", policy=self.policy)

    def execution(self, names):
        """Build synthetic complete entered libtest output, never product evidence."""
        return (f"running {len(names)} tests\n" + "".join(f"test {name} ... ok\n" for name in names) +
                f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def test_entered_blocks_require_all_current_cases_and_ke_neighbors(self):
        binding = self.policy["binding"]; names = binding["test_inventory"]
        healthy = self.execution(names)
        self.assertEqual(MODULE.validate_engine_ke_execution(healthy, binding)["tests_executed"], 560)
        self.assertEqual(MODULE.validate_engine_ke_execution(healthy, binding)["ke_tests_executed"], 276)
        missing = [name for name in names if name != "nts_ke_experiment::fixture_case_00"]
        old = [name for name in names if not name.startswith("nts_ke_experiment::")]
        variants = [self.execution([]), self.execution(old), self.execution(missing),
            self.execution([*names, names[0]]), healthy.replace(" ... ok", " ... FAILED", 1),
            healthy.replace("0 filtered out", "1 filtered out"), healthy.replace("running 560", "running 559"),
            healthy.replace("test result: ok. 560 passed", "test result: ok. 559 passed"),
            "\n".join(line for line in healthy.splitlines() if not line.startswith("running "))]
        for text in variants:
            with self.subTest(sample=text[-120:]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_execution(text, binding)
        altered = copy.deepcopy(binding); altered["test_inventory"][0] = "nts_ke_experiment::unexecuted_other_case"
        with self.assertRaises(MODULE.Incomplete): MODULE.validate_engine_ke_execution(healthy, altered)

    def stale_engine_ke_inventories(self):
        """Historical and substituted synthetic rosters, independently constructed."""
        required = ("nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::"
                    "numerics_tests::facade_protected_numerics_send_after_wall_step_is_not_omitted")
        current = self.policy["binding"]["test_inventory"]
        before_effect = [name for name in current if name not in self.PROTECTED_RECEIVE_EFFECT_CASES]
        before_order = [name for name in before_effect if name != self.ORDINARY_PROTECTED_RECEIVE_CASE]
        before_age = [name for name in before_order if name != self.HEADER_AGE_CASE]
        before_debit = [name for name in before_age if name != self.KNOWN_RECEIVE_DEBIT_CASE]
        before_chronology = [name for name in before_debit if name not in self.RECEIVE_CHRONOLOGY_CASES]
        before_receive_capture = [name for name in before_chronology if name not in self.RECEIVE_CAPTURE_CASES]
        before_capture = [name for name in before_receive_capture if name not in self.PLAIN_CAPTURE_CASES]
        before_wait_and_rate = [name for name in before_capture if name not in self.MANAGED_WAIT_AND_RATE_CASES]
        before_reply_cutoff = [name for name in before_wait_and_rate if name not in self.FINAL_REPLY_CUTOFF_CASES]
        return {
            "historical_400_222": sorted(
                [f"ordinary_fixture_{index:03}" for index in range(178)] +
                [f"nts_ke_experiment::fixture_case_{index:02}" for index in range(222)]),
            "historical_410_229": sorted(
                [f"ordinary_fixture_{index:03}" for index in range(181)] +
                [f"nts_ke_experiment::fixture_case_{index:02}" for index in range(228)] + [required]),
            "historical_409_228": sorted(
                [f"ordinary_fixture_{index:03}" for index in range(181)] +
                [f"nts_ke_experiment::fixture_case_{index:02}" for index in range(228)]),
            "historical_479_234": sorted(
                [f"ordinary_fixture_{index:03}" for index in range(245)] +
                [f"nts_ke_experiment::fixture_case_{index:02}" for index in range(233)] + [required]),
            "historical_520_246": [name for name in before_reply_cutoff if name not in (
                *self.MANAGED_PROTECTED_CASES, *self.MANAGED_SUCCESSOR_CASES, *self.MANAGED_DENY_CASES,
                *self.MANAGED_WORKER_BOUNDARY_CASES)],
            "historical_528_254": [name for name in before_reply_cutoff if name not in (
                *self.MANAGED_SUCCESSOR_CASES, *self.MANAGED_DENY_CASES, *self.MANAGED_WORKER_BOUNDARY_CASES)],
            "historical_530_256": [name for name in before_reply_cutoff if name not in (
                *self.MANAGED_DENY_CASES, *self.MANAGED_WORKER_BOUNDARY_CASES)],
            "historical_535_261": [name for name in before_reply_cutoff if name not in self.MANAGED_WORKER_BOUNDARY_CASES],
            "historical_538_264": before_reply_cutoff,
            "historical_540_264": before_wait_and_rate,
            "historical_544_268": before_capture,
            "historical_548_268": before_receive_capture,
            "historical_551_269": before_chronology,
            "historical_554_271": before_debit,
            "historical_555_272": before_age,
            "historical_556_272": before_order,
            "historical_557_273": before_effect,
            "same_counts_missing_protected_receive": sorted(
                [name for name in current if name != self.ORDINARY_PROTECTED_RECEIVE_CASE]
                + ["nts_ke_experiment::other_order_case"]),
            "same_counts_missing_header_age": sorted(
                [name for name in current if name != self.HEADER_AGE_CASE] + ["ordinary_other_header_age"]),
            **{f"same_counts_missing_{omitted.rsplit('::', 1)[-1]}": sorted(
                [name for name in current if name != omitted] + [
                    "nts_ke_experiment::other_receive_case" if omitted.startswith("nts_ke_experiment::")
                    else "ordinary_other_receive_case"])
                for omitted in (*self.RECEIVE_CAPTURE_CASES, *self.RECEIVE_CHRONOLOGY_CASES,
                                self.KNOWN_RECEIVE_DEBIT_CASE, *self.PROTECTED_RECEIVE_EFFECT_CASES)},
            "current_missing_forwarding": [name for name in current if name != required],
            "same_counts_missing_forwarding": sorted(
                [name for name in current if name != required] + ["nts_ke_experiment::other_case"]),
            **{f"same_counts_missing_{omitted.rsplit('::', 1)[-1]}": sorted(
                [name for name in current if name != omitted] + ["nts_ke_experiment::other_case"])
                for omitted in (*self.MANAGED_PROTECTED_CASES, *self.MANAGED_SUCCESSOR_CASES,
                                *self.MANAGED_DENY_CASES, *self.MANAGED_WORKER_BOUNDARY_CASES,
                                *self.MANAGED_WAIT_AND_RATE_CASES)},
            **{f"same_counts_missing_{omitted.rsplit('::', 1)[-1]}": sorted(
                [name for name in current if name != omitted] + ["ordinary_other_case"])
                for omitted in (*self.FINAL_REPLY_CUTOFF_CASES, *self.PLAIN_CAPTURE_CASES)},
        }

    def test_old_or_substituted_roster_refuses_policy_and_execution(self):
        MODULE.validate_engine_ke_policy(self.policy)
        for label, names in self.stale_engine_ke_inventories().items():
            bad = copy.deepcopy(self.policy); bad["binding"]["test_inventory"] = names
            with self.subTest(roster=label, owner="policy"), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_policy(bad)
            with self.subTest(roster=label, owner="execution"), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_execution(self.execution(names), bad["binding"])

    def test_fixed_inventory_rejects_malformed_names_and_ke_mix(self):
        names = self.policy["binding"]["test_inventory"]
        self.assertEqual(MODULE.validate_engine_ke_inventory(names), names)
        duplicate = [*names[:-1], names[0]]
        wrong_mix = sorted([*names[:-1], "nts_ke_experiment::extra_ke_case"])
        for value in (None, True, tuple(names), names[1:], names[::-1], duplicate, wrong_mix,
                      [*names[:-1], ""], [*names[:-1], "a" * 4097], [*names[:-1], []]):
            with self.subTest(kind=type(value).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_inventory(value)

    def test_missing_numerics_helper_pin_refuses_admission(self):
        path = "crates/nevrotick-engine/src/nts_ke_experiment/recovery/numerics_tests.rs"
        self.assertIn(path, self.policy["binding"]["inputs_sha256"])
        bad = copy.deepcopy(self.policy); del bad["binding"]["inputs_sha256"][path]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)

    def test_receiving_supplier_requires_one_source_owner_and_its_helper_pin(self):
        path = "crates/nevrotick-engine/src/nts_ke_experiment/recovery/receiving_tests.rs"
        self.assertEqual(MODULE.ENGINE_KE_TEST_SOURCES.count(path), 1)
        self.assertEqual(MODULE.ENGINE_KE_BOUND_INPUTS.count(path), 1)
        self.assertIn(path, self.policy["binding"]["inputs_sha256"])
        MODULE.validate_engine_ke_policy(self.policy)
        bad = copy.deepcopy(self.policy); del bad["binding"]["inputs_sha256"][path]
        with self.assertRaisesRegex(MODULE.Incomplete, "source/helper inputs differ"):
            MODULE.validate_engine_ke_policy(bad)

    def test_plain_capture_case_sources_require_exact_role_pins(self):
        for path in ("crates/nevrotick-engine/tests/private/session.rs",
                     "crates/nevrotick-engine/tests/private/source_policy.rs"):
            with self.subTest(path=path):
                self.assertEqual(MODULE.ENGINE_KE_TEST_SOURCES.count(path), 1)
                self.assertEqual(MODULE.ENGINE_KE_BOUND_INPUTS.count(path), 1)
                bad = copy.deepcopy(self.policy)
                del bad["binding"]["inputs_sha256"][path]
                with self.assertRaisesRegex(MODULE.Incomplete, "source/helper inputs differ"):
                    MODULE.validate_engine_ke_policy(bad)

    def test_loaded_analysis_controls_is_in_exact_helper_closure(self):
        path = "scripts/analysis-controls.py"
        self.assertIn(path, MODULE.ENGINE_KE_BOUND_INPUTS)
        MODULE.validate_engine_ke_policy(self.policy)
        bad = copy.deepcopy(self.policy); del bad["binding"]["inputs_sha256"][path]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)

    def test_complete_annotated_doctests_are_exact_in_both_execution_lanes(self):
        binding = self.policy["binding"]
        names = binding["test_inventory"]
        docs = binding["doctest_inventory"]
        ordinary = self.execution(names)
        rustdoc = self.execution(docs)
        self.assertEqual(MODULE.validate_engine_ke_current_doctests(rustdoc, binding)["test_names"], docs)
        combined = MODULE.validate_engine_ke_execution(ordinary + rustdoc, binding, include_doctests=True)
        self.assertEqual(combined["additional_doctests"], docs)
        self.assertEqual(combined["tests_executed"], 650)
        self.assertEqual(combined["ke_tests_executed"], 276)
        self.assertEqual(MODULE.validate_engine_ke_execution(ordinary, binding)["test_names"], names)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_execution(ordinary + rustdoc, binding)

        compile_fail = next(name for name in docs if name.endswith(" - compile fail"))
        suffix = [name.removesuffix(" - compile fail") if name == compile_fail else name for name in docs]
        wrong_line = [name.replace("line 20", "line 21") if name == compile_fail else name for name in docs]
        problems = {
            "zero": self.execution([]),
            "partial": self.execution(docs[1:]),
            "same-count-name": self.execution([*docs[:-1], "src/lib.rs - Substituted::checked (line 10)"]),
            "same-count-suffix": self.execution(suffix),
            "same-count-line": self.execution(wrong_line),
            "duplicate": rustdoc + self.execution([docs[0]]),
            "failed": rustdoc.replace(" ... ok", " ... FAILED", 1),
            "ignored": rustdoc.replace(" ... ok", " ... ignored", 1),
            "filtered": rustdoc.replace("0 filtered out", "1 filtered out"),
            "truncated": rustdoc.rpartition("test result:")[0],
        }
        for name, text in problems.items():
            with self.subTest(problem=name, lane="combined"), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_execution(ordinary + text, binding, include_doctests=True)
            with self.subTest(problem=name, lane="standalone"), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_current_doctests(text, binding)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_execution(self.execution(names[1:]) + rustdoc, binding, include_doctests=True)

    def test_live_policy_rejects_missing_or_malformed_doctest_inventory(self):
        valid = self.policy["binding"]["doctest_inventory"]
        for bad_roster in (None, (), [], valid[1:], valid[::-1], [*valid[:-1], valid[0]], [*valid[:-1], True],
                           [*valid[:-1], "src/lib.rs - Stub (line 0)"], [*valid[:-1], valid[-1] + " - run"]):
            policy = copy.deepcopy(self.policy)
            policy["binding"]["doctest_inventory"] = bad_roster
            with self.subTest(roster=repr(bad_roster)[:90]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_policy(policy)
        policy = copy.deepcopy(self.policy)
        policy["binding"].pop("doctest_inventory")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(policy)
        self.invoke(accepted=True)

    def test_ep1_evidence_rejects_candidate_lane_and_physical_log_drift_before_session(self):
        binding = self.policy["binding"]
        receipt = Path(binding["ordinary_evidence"]["path"])
        ordinary = receipt.with_name("ep1-ordinary.json")
        rustdoc = receipt.with_name("ep1-doctests.json")
        self.invoke(accepted=True)
        for case in ("candidate", "identity", "receipt-mode", "row-duplicate", "row-failed", "row-cases",
                     "ordinary-bytes", "rustdoc-bytes", "wrong-command", "same-count-raw",
                     "wrong-package", "wrong-binary"):
            with self.subTest(case=case):
                self.write_ep1_reference()
                report = json.loads(receipt.read_bytes())
                doc_row = next(row for row in report["lanes"] if row["lane"] == "ep1-doctests")
                if case == "candidate":
                    report["candidate_sha256"] = "f" * 64
                elif case == "identity":
                    report["inputs"]["Cargo.lock"] = "f" * 64
                elif case == "receipt-mode":
                    report["mode"] = "compile"
                elif case == "row-duplicate":
                    report["lanes"].append(doc_row.copy())
                elif case == "row-failed":
                    doc_row["exit_code"] = 1
                elif case == "row-cases":
                    doc_row["cases"].pop()
                elif case in {"wrong-package", "wrong-binary"}:
                    first = next(row for row in report["lanes"] if row["lane"] == "ep1-ordinary")["cases"][0]
                    first["package" if case == "wrong-package" else "binary"] = "foreign"
                elif case in {"ordinary-bytes", "rustdoc-bytes"}:
                    raw = ordinary if case == "ordinary-bytes" else rustdoc
                    raw.write_bytes(raw.read_bytes() + b" ")
                else:
                    raw = json.loads(rustdoc.read_bytes())
                    if case == "wrong-command":
                        raw["command"].remove("ke-association-prototype")
                    else:
                        raw["output"] = raw["output"].replace(" - compile fail", " - compile", 1)
                        doc_row["cases"] = [name.removesuffix(" fail") if name.endswith(" - compile fail") else name
                                            for name in doc_row["cases"]]
                    rustdoc.write_text(json.dumps(raw))
                    doc_row["sha256"] = MODULE.PRODUCT.sha256(rustdoc)
                receipt.write_text(json.dumps(report))
                binding["ordinary_evidence"]["sha256"] = MODULE.PRODUCT.sha256(receipt)
                self.invoke()
        self.write_ep1_reference()
        previous = binding["doctest_inventory"]
        changed = [name.removesuffix(" fail") if name.endswith(" - compile fail") else name for name in previous]
        binding["doctest_inventory"] = sorted(changed)
        self.invoke()
        binding["doctest_inventory"] = previous
        self.invoke(accepted=True)

    def test_ep1_typed_native_observations_refuse_before_session(self):
        """Numeric equality cannot replace literal native status/count types."""
        self.invoke(accepted=True)
        binding = self.policy["binding"]
        receipt = Path(binding["ordinary_evidence"]["path"])
        faults = ("row-bool-exit", "row-float-exit", "row-float-count",
                  "raw-bool-exit", "raw-float-exit", "missing-raw-failure")
        for lane in ("ep1-ordinary", "ep1-doctests"):
            for fault in faults:
                with self.subTest(lane=lane, fault=fault):
                    self.write_ep1_reference()
                    report = json.loads(receipt.read_bytes())
                    row = next(item for item in report["lanes"] if item["lane"] == lane)
                    raw_path = receipt.with_name(lane + ".json")
                    raw = json.loads(raw_path.read_bytes())
                    if fault == "row-bool-exit":
                        row["exit_code"] = False
                    elif fault == "row-float-exit":
                        row["exit_code"] = 0.0
                    elif fault == "row-float-count":
                        row["tests_executed"] = float(row["tests_executed"])
                    elif fault == "raw-bool-exit":
                        raw["exit_code"] = False
                    elif fault == "raw-float-exit":
                        raw["exit_code"] = 0.0
                    else:
                        raw.pop("failure")
                    raw_path.write_text(json.dumps(raw))
                    row["sha256"] = MODULE.PRODUCT.sha256(raw_path)
                    receipt.write_text(json.dumps(report))
                    binding["ordinary_evidence"]["sha256"] = MODULE.PRODUCT.sha256(receipt)
                    before = (receipt.read_bytes(), raw_path.read_bytes())
                    reason = ("EP1 lane lacks complete successful execution identity"
                              if fault.startswith("row-")
                              else "raw lane differs from its fixed successful EP1 command")
                    with self.assertRaisesRegex(MODULE.Incomplete, reason):
                        MODULE.admit_engine_ke_evidence(binding)
                    self.invoke()
                    self.assertFalse(self.args.output_dir.exists())
                    self.assertEqual((receipt.read_bytes(), raw_path.read_bytes()), before)
        self.write_ep1_reference()
        self.invoke(accepted=True)

    def test_ep1_evidence_refuses_linked_copies_even_with_exact_bytes(self):
        path = self.base / "ep1-doctests.json"
        contents = path.read_bytes()
        other = self.base / "other-current-docs.json"
        other.write_bytes(contents)
        for link in (path.symlink_to, path.hardlink_to):
            path.unlink()
            link(other)
            self.invoke()
        path.unlink()
        path.write_bytes(contents)
        self.invoke(accepted=True)

    def test_physical_doc_log_drift_after_session_finish_refuses_qualification(self):
        self.args.lane, self.args.subject = "contracts", "ke_job"
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        support = self
        report = {}
        finished = []

        class Session:
            def __init__(self, root, *_arguments):
                self.source, self.product_identity = root, support.identity

            def finish(self):
                path = support.base / "ep1-doctests.json"
                path.write_bytes(path.read_bytes() + b" ")
                finished.append(True)

        with patch.object(MODULE, "ROOT", self.root), \
                patch.object(MODULE.PRODUCT, "admit_product",
                             return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=7 * 1024**3)), \
                patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE, "engine_ke_history_contracts") as history, \
                self.assertRaisesRegex(MODULE.Incomplete, "exact binding"):
            MODULE.execute_engine_ke_profile(self.args, report)
        history.assert_called_once()
        self.assertEqual(finished, [True])
        self.assertFalse(report["protected_history_contracts_passed"])
        self.assertFalse(report.get("selected_lane_passed", False))

    def test_reviewed_descriptors_must_match_actual_listing_not_only_count(self):
        inventories = {name: {name + " synthetic fault": {"name": name + " synthetic fault",
            "file": path, "package": "nevrotick-engine", "replacement": "false"}}
            for name, path in self.HOST.items()}
        self.policy["mutation_inventories"] = {name: {"count": 1, "sha256": independent_digest(items)}
            for name, items in inventories.items()}
        MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.ENGINE_KE_SCOPE, self.policy, inventories)
        bad = copy.deepcopy(inventories); bad["ke_association"]["ke_association synthetic fault"]["replacement"] = "true"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.admit_reviewed_mutation_inventories(MODULE.ENGINE_KE_SCOPE, self.policy, bad)

    def test_historical_six_owner_scope_cannot_qualify_new_owners(self):
        self.assertIn("ke_ledger", MODULE.ENGINE_KE_HOST_SUBJECTS)
        self.assertIn("ke_response", MODULE.ENGINE_KE_HOST_SUBJECTS)
        self.assertIn("ke_datagram", MODULE.ENGINE_KE_HOST_SUBJECTS)
        bad = copy.deepcopy(self.policy)
        for name in ("ke_ledger", "ke_response"):
            del bad["subjects"][name]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)
        bad = copy.deepcopy(self.policy)
        del bad["subjects"]["ke_datagram"]
        bad["critical_files"].remove(self.HOST["ke_datagram"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)
        bad = copy.deepcopy(self.policy)
        bad["binding"]["test_inventory"] = bad["binding"]["test_inventory"][:233]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_policy(bad)

    def test_doctest_lane_preserves_compilation_only_meaning(self):
        names = ["src/lib.rs - Example::new (line 1)", "src/lib.rs - Example::no_run (line 2) - compile",
                 "src/lib.rs - Example::private (line 3) - compile fail"]
        result = MODULE.validate_engine_ke_doctests(self.execution(names))
        self.assertEqual(result["tests_executed"], 3)
        self.assertEqual(result["compilation_only"], names[1:])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_doctests(self.execution(["ordinary_case"]))

    def test_baseline_keeps_full_release_docs_and_windows_source_checks_separate(self):
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        calls = []
        outer = self
        class Session:
            def __init__(self, root, _args, report, *_arguments):
                self.product_identity = outer.identity
                self.source = root
                self.output = outer.args.output_dir
                self.output.mkdir(mode=0o700)
                self.work = self.output / "work"
                self.work.mkdir(mode=0o700)
                (self.work / "target").mkdir(mode=0o700)
                self.release = self.work / "target/release"
                self.release.mkdir(mode=0o700)
                self.debug = self.work / "target/debug"
                self.debug.mkdir(mode=0o700)
                (self.debug / "completed-test-cache").write_text("completed synthetic debug artifact\n")
                self.keep = [self.work / "target/x86_64-pc-windows-msvc/debug/keep", self.work / "target/unrelated/keep",
                             self.output / "raw.log", self.source / "source-sentinel"]
                for path in self.keep:
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_text("retained synthetic sentinel\n")
                self.report = report
                report["lanes"] = []
            def run(self, label, argv, timeout):
                calls.append((label, argv, timeout))
                self.report["lanes"].append({"lane": label, "exit_code": 0, "failure": None})
                if label == "ke-release":
                    outer.assertFalse(self.debug.exists(), "validated debug target must retire before release")
                    outer.assertTrue(self.report["debug_target_retirement"]["removed"])
                    for path in self.keep:
                        outer.assertEqual(path.read_text(), "retained synthetic sentinel\n")
                    (self.release / "completed-artifact").write_bytes(b"x" * 65536)
                if label == "ke-doctests":
                    outer.assertFalse(self.release.exists(), "completed release target must retire before doctests")
                    self.debug.mkdir(mode=0o700)
                    (self.debug / "rebuilt-for-docs").write_text("fresh synthetic debug metadata\n")
                    for path in self.keep:
                        outer.assertEqual(path.read_text(), "retained synthetic sentinel\n")
                if label in {"ke-baseline", "ke-release"}:
                    return outer.execution(outer.policy["binding"]["test_inventory"])
                if label == "ke-doctests":
                    return outer.execution(outer.policy["binding"]["doctest_inventory"])
                return "Synthetic compiler result only; no product execution.\n"
            def finish(self):
                calls.append(("finish", [], 0))
        report = {}
        with patch.object(MODULE, "ROOT", self.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=7 * 1024**3)), \
                patch.object(MODULE.COMMON, "Session", Session):
            MODULE.execute_engine_ke_profile(self.args, report)
        self.assertEqual([row[0] for row in calls], ["ke-format", "ke-windows-check", "ke-baseline", "ke-clippy",
            "ke-release", "ke-doctests", "ke-windows-clippy", "finish"])
        self.assertIn("--release", calls[4][1])
        self.assertIn("--message-format=json", calls[1][1])
        for label, argv, _timeout in (calls[1], calls[6]):
            self.assertEqual(argv[argv.index("--target") + 1], "x86_64-pc-windows-msvc")
            self.assertIn("--all-targets", argv)
            self.assertNotIn("build", argv)
            self.assertNotIn("run", argv)
        self.assertEqual(report["baseline_execution"]["tests_executed"], 560)
        self.assertEqual(report["release_execution"]["ke_tests_executed"], 276)
        self.assertEqual(report["doctest_execution"]["compilation_only"],
                         ["src/lib.rs - Example::prepared (line 30) - compile",
                          "src/lib.rs - Example::private (line 20) - compile fail"])
        retired = report["release_target_retirement"]
        self.assertEqual((retired["path"], retired["completed_lane"], retired["removed"]),
                         ("work/target/release", "ke-release", True))
        self.assertGreaterEqual(retired["before"]["allocated_bytes"], 65536)
        self.assertEqual([row["phase"] for row in report["storage_samples"]
                          if row["lane"] == "ke-release-target-retirement"], ["before", "after"])
        self.assertFalse(report["qualification_passed"])
        self.assertEqual((report["debug_target_retirement"]["path"],
                          report["debug_target_retirement"]["completed_lane"],
                          report["debug_target_retirement"]["removed"]),
                         ("work/target/debug", "ke-clippy", True))

    def debug_retirement_fixture(self):
        """Real owned cache/sentinels; explicitly synthetic successful child records."""
        temporary = tempfile.TemporaryDirectory(prefix="debug-retirement-", dir=self.base)
        self.addCleanup(temporary.cleanup)
        output = Path(temporary.name)
        work = output / "work"; work.mkdir(mode=0o700)
        target = work / "target"; target.mkdir(mode=0o700)
        debug = target / "debug"; debug.mkdir(mode=0o700)
        (debug / "completed").write_bytes(b"d" * 65536)
        kept = [target / "x86_64-pc-windows-msvc/debug/metadata", target / "release/not-debug",
                output / "source/input.rs", output / "tools/admitted", output / "raw.log"]
        for path in kept:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"unchanged synthetic sentinel")
        session = SimpleNamespace(output=output, work=work)
        report = {"lanes": [{"lane": name, "exit_code": 0, "failure": None}
                            for name in ("ke-baseline", "ke-clippy")],
                  "baseline_execution": MODULE.validate_engine_ke_execution(
                      self.execution(self.policy["binding"]["test_inventory"]), self.policy["binding"])}
        MODULE.address_storage_checkpoint(session, self.policy, report, "ke-baseline", "after")
        MODULE.address_storage_checkpoint(session, self.policy, report, "ke-clippy", "after")
        return session, report, debug, kept

    def test_debug_retirement_has_closed_profiles_and_preserves_all_other_outputs(self):
        session, report, debug, kept = self.debug_retirement_fixture()
        for profile in ("other", "../source", "debug/..", None, True):
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE._retire_engine_ke_target(session, self.policy, report, profile)
            self.assertTrue(debug.is_dir())
        MODULE.retire_engine_ke_debug_target(session, self.policy, report)
        self.assertFalse(debug.exists())
        for path in kept:
            self.assertEqual(path.read_bytes(), b"unchanged synthetic sentinel")
        retired = report["debug_target_retirement"]
        self.assertEqual((retired["path"], retired["completed_lane"], retired["removed"]),
                         ("work/target/debug", "ke-clippy", True))
        self.assertGreaterEqual(retired["before"]["allocated_bytes"], 65536)
        self.assertEqual([row["phase"] for row in report["storage_samples"]
                          if row["lane"] == "ke-debug-target-retirement"], ["before", "after"])

    def test_debug_retirement_rejects_old_or_substituted_roster_without_deleting_cache(self):
        for label, names in self.stale_engine_ke_inventories().items():
            session, report, debug, kept = self.debug_retirement_fixture()
            bad = copy.deepcopy(self.policy); bad["binding"]["test_inventory"] = names
            report["baseline_execution"] = {
                "test_names": names, "tests_executed": len(names),
                "ke_tests_executed": sum(name.startswith("nts_ke_experiment::") for name in names),
                "inventory_sha256": independent_digest(names),
            }
            with self.subTest(roster=label), self.assertRaises(MODULE.Incomplete):
                MODULE.retire_engine_ke_debug_target(session, bad, report)
            self.assertTrue(debug.is_dir())
            for path in kept:
                self.assertEqual(path.read_bytes(), b"unchanged synthetic sentinel")

    def test_debug_retirement_requires_complete_ordered_successful_roster_and_storage(self):
        cases = ("missing-baseline", "duplicate-baseline", "order", "failed-baseline", "failed-clippy",
                 "bool-exit", "child-failure", "missing-execution", "partial-roster", "count", "ke-count",
                 "digest", "missing-sample", "stale-sample", "cap", "baseline-cap", "reserve")
        for case in cases:
            with self.subTest(case=case):
                session, report, debug, kept = self.debug_retirement_fixture()
                if case == "missing-baseline": report["lanes"].pop(0)
                elif case == "duplicate-baseline": report["lanes"].insert(0, report["lanes"][0].copy())
                elif case == "order": report["lanes"].reverse()
                elif case == "failed-baseline": report["lanes"][0]["exit_code"] = 1
                elif case == "failed-clippy": report["lanes"][-1]["exit_code"] = 1
                elif case == "bool-exit": report["lanes"][0]["exit_code"] = False
                elif case == "child-failure": report["lanes"][-1]["failure"] = "synthetic child failure"
                elif case == "missing-execution": report.pop("baseline_execution")
                elif case == "partial-roster": report["baseline_execution"]["test_names"].pop()
                elif case == "count": report["baseline_execution"]["tests_executed"] -= 1
                elif case == "ke-count": report["baseline_execution"]["ke_tests_executed"] -= 1
                elif case == "digest": report["baseline_execution"]["inventory_sha256"] = "f" * 64
                elif case == "missing-sample": report["storage_samples"].pop(0)
                elif case == "stale-sample": report["storage_samples"].append({"lane": "other", "phase": "after"})
                elif case == "cap": report["storage_samples"][-1]["allocated_bytes"] = self.policy["owned_output_maximum_allocated_bytes"]
                elif case == "baseline-cap": report["storage_samples"][0]["allocated_bytes"] = self.policy["owned_output_maximum_allocated_bytes"]
                else: report["storage_samples"][-1]["available_bytes"] = self.policy["stop_free_bytes"] - 1
                with patch.object(MODULE.shutil, "rmtree") as removal:
                    removal.avoids_symlink_attacks = True
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.retire_engine_ke_debug_target(session, self.policy, report)
                    removal.assert_not_called()
                self.assertTrue(debug.is_dir())
                self.assertNotIn("debug_target_retirement", report)
                for path in kept:
                    self.assertEqual(path.read_bytes(), b"unchanged synthetic sentinel")

    def test_debug_deletion_core_cannot_bypass_admission(self):
        session, report, debug, _kept = self.debug_retirement_fixture()
        report.pop("baseline_execution")
        with self.assertRaises(MODULE.Incomplete):
            MODULE._retire_engine_ke_target(session, self.policy, report, "debug")
        self.assertTrue(debug.is_dir())

    def test_debug_retirement_refuses_unsafe_paths_ownership_and_missing_safe_removal(self):
        for case in ("debug-link", "child-link", "fifo", "target-link", "noncanonical", "public", "unowned", "unsafe-removal"):
            with self.subTest(case=case):
                session, report, debug, kept = self.debug_retirement_fixture()
                outside = self.base / ("debug-external-" + case); outside.mkdir(mode=0o700)
                sentinel = outside / "keep"; sentinel.write_bytes(b"outside ownership")
                if case == "debug-link":
                    shutil.rmtree(debug); debug.symlink_to(outside, target_is_directory=True)
                elif case == "child-link": (debug / "linked").symlink_to(outside, target_is_directory=True)
                elif case == "fifo": os.mkfifo(debug / "pipe", mode=0o600)
                elif case == "target-link":
                    debug.parent.rename(session.work / "saved-target")
                    (session.work / "target").symlink_to(outside, target_is_directory=True)
                elif case == "noncanonical":
                    (session.output / "alias").mkdir(mode=0o700)
                    session.work = session.output / "alias/../work"
                elif case == "public": session.work.chmod(0o755)
                if case == "unowned":
                    with patch.object(MODULE.os, "geteuid", return_value=os.geteuid() + 1), self.assertRaises(MODULE.Incomplete):
                        MODULE.retire_engine_ke_debug_target(session, self.policy, report)
                elif case == "unsafe-removal":
                    with patch.object(MODULE.shutil.rmtree, "avoids_symlink_attacks", False), self.assertRaises(MODULE.Incomplete):
                        MODULE.retire_engine_ke_debug_target(session, self.policy, report)
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.retire_engine_ke_debug_target(session, self.policy, report)
                self.assertEqual(sentinel.read_bytes(), b"outside ownership")
                self.assertEqual((session.output / "raw.log").read_bytes(), b"unchanged synthetic sentinel")
                self.assertNotIn("debug_target_retirement", report)

    def test_debug_deletion_failure_keeps_raw_data_and_fails_closed(self):
        session, report, debug, kept = self.debug_retirement_fixture()
        with patch.object(MODULE.shutil, "rmtree", side_effect=OSError("synthetic removal failure")) as removal:
            removal.avoids_symlink_attacks = True
            with self.assertRaises(MODULE.Incomplete):
                MODULE.retire_engine_ke_debug_target(session, self.policy, report)
            removal.assert_called_once_with(debug)
        self.assertFalse(report["debug_target_retirement"]["removed"])
        self.assertTrue(debug.is_dir())
        for path in kept:
            self.assertEqual(path.read_bytes(), b"unchanged synthetic sentinel")

    def release_retirement_fixture(self):
        """Real private files plus explicitly synthetic completed-child observations."""
        temporary = tempfile.TemporaryDirectory(prefix="retirement-", dir=self.base)
        self.addCleanup(temporary.cleanup)
        output = Path(temporary.name)
        work = output / "work"; work.mkdir(mode=0o700)
        target = work / "target"; target.mkdir(mode=0o700)
        release = target / "release"; release.mkdir(mode=0o700)
        (release / "artifact").write_bytes(b"owned release artifact")
        debug = target / "debug"; debug.mkdir(mode=0o700)
        (debug / "keep").write_bytes(b"retained debug sentinel")
        (output / "raw.log").write_text("retained raw child output")
        session = SimpleNamespace(output=output, work=work)
        binding = self.policy["binding"]
        report = {"lanes": [{"lane": "ke-release", "exit_code": 0, "failure": None}],
                  "release_execution": MODULE.validate_engine_ke_execution(
                      self.execution(binding["test_inventory"]), binding)}
        MODULE.address_storage_checkpoint(session, self.policy, report, "ke-release", "after")
        return session, report, release

    def test_release_retirement_removes_only_exact_validated_cache(self):
        session, report, release = self.release_retirement_fixture()
        MODULE.retire_engine_ke_release_target(session, self.policy, report)
        self.assertFalse(release.exists())
        self.assertTrue(report["release_target_retirement"]["removed"])
        self.assertEqual((session.output / "raw.log").read_text(), "retained raw child output")
        self.assertEqual((release.parent / "debug/keep").read_bytes(), b"retained debug sentinel")

    def test_release_retirement_rejects_unverified_execution_without_deleting_cache(self):
        for case in ("missing-execution", "counts-only", "wrong-type", "partial-roster",
                     "same-count-substitution", "wrong-count", "wrong-ke-count", "wrong-digest"):
            with self.subTest(case=case):
                session, report, release = self.release_retirement_fixture()
                execution = report["release_execution"]
                if case == "missing-execution":
                    report.pop("release_execution")
                elif case == "counts-only":
                    execution.pop("test_names")
                    execution.pop("inventory_sha256")
                elif case == "wrong-type":
                    report["release_execution"] = True
                elif case == "partial-roster":
                    execution["test_names"].pop()
                elif case == "same-count-substitution":
                    execution["test_names"][0] = "ordinary_unexecuted_substitute"
                    execution["inventory_sha256"] = independent_digest(sorted(execution["test_names"]))
                elif case == "wrong-count":
                    execution["tests_executed"] -= 1
                elif case == "wrong-ke-count":
                    execution["ke_tests_executed"] -= 1
                else:
                    execution["inventory_sha256"] = "f" * 64

                refused = False
                try:
                    MODULE.retire_engine_ke_release_target(session, self.policy, report)
                except MODULE.Incomplete:
                    refused = True
                self.assertEqual((session.output / "raw.log").read_text(), "retained raw child output")
                self.assertEqual((release.parent / "debug/keep").read_bytes(), b"retained debug sentinel")
                self.assertEqual((refused, release.is_dir(), "release_target_retirement" in report),
                                 (True, True, False))
                self.assertEqual((release / "artifact").read_bytes(), b"owned release artifact")

    def test_release_retirement_refuses_links_special_and_noncanonical_or_public_parents(self):
        for case in ("release-link", "child-link", "fifo", "target-link", "noncanonical", "public-work"):
            with self.subTest(case=case):
                session, report, release = self.release_retirement_fixture()
                outside = session.output.parent / (session.output.name + "-outside")
                outside.mkdir(mode=0o700)
                self.addCleanup(shutil.rmtree, outside)
                sentinel = outside / "release/keep"; sentinel.parent.mkdir()
                sentinel.write_text("unrelated external sentinel")
                if case == "release-link":
                    shutil.rmtree(release)
                    release.symlink_to(outside / "release", target_is_directory=True)
                elif case == "child-link":
                    (release / "linked").symlink_to(outside, target_is_directory=True)
                elif case == "fifo":
                    os.mkfifo(release / "pipe", mode=0o600)
                elif case == "target-link":
                    release.parent.rename(session.work / "saved-target")
                    (session.work / "target").symlink_to(outside, target_is_directory=True)
                elif case == "noncanonical":
                    (session.output / "alias").mkdir(mode=0o700)
                    session.work = session.output / "alias/../work"
                else:
                    session.work.chmod(0o755)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.retire_engine_ke_release_target(session, self.policy, report)
                self.assertEqual(sentinel.read_text(), "unrelated external sentinel")
                self.assertEqual((session.output / "raw.log").read_text(), "retained raw child output")
                self.assertNotIn("release_target_retirement", report)

    def test_failed_release_validation_or_postlane_cap_prevents_retirement_and_later_lanes(self):
        (self.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(self.policy))
        for mode in ("command", "validation", "storage"):
            with self.subTest(mode=mode):
                self.args.output_dir = self.base / ("negative-release-" + mode)
                calls = []; report = {}; outer = self
                artifact = self.args.output_dir / "work/target/release/artifact"
                class Session:
                    def __init__(self, root, _args, report, *_arguments):
                        self.product_identity = outer.identity; self.source = root
                        self.output = outer.args.output_dir; self.output.mkdir(mode=0o700)
                        self.work = self.output / "work"; self.work.mkdir(mode=0o700)
                        (self.work / "target").mkdir(mode=0o700)
                        (self.work / "target/debug").mkdir(mode=0o700)
                        (self.work / "target/debug/complete").write_text("validated synthetic debug cache")
                        artifact.parent.mkdir(mode=0o700)
                        self.report = report; report["lanes"] = []
                    def run(self, label, argv, timeout):
                        calls.append(label)
                        self.report["lanes"].append({"lane": label, "exit_code": 1 if label == "ke-release" and mode == "command" else 0,
                                                     "failure": None})
                        if label == "ke-release":
                            artifact.write_text("completed or failed release evidence")
                            if mode == "command":
                                raise MODULE.Incomplete("Synthetic release child failure")
                            names = outer.policy["binding"]["test_inventory"]
                            return outer.execution(names[:-1] if mode == "validation" else names)
                        if label == "ke-baseline":
                            return outer.execution(outer.policy["binding"]["test_inventory"])
                        return "Synthetic compiler result only; no product execution.\n"
                    def finish(self):
                        calls.append("finish")
                original_usage = MODULE.address_storage_usage
                def measured_usage(directory):
                    usage = original_usage(directory)
                    if mode == "storage" and artifact.exists():
                        return {**usage, "allocated_bytes": 3221225472}
                    return usage
                with patch.object(MODULE, "ROOT", self.root), \
                        patch.object(MODULE.PRODUCT, "admit_product", return_value=({"engine_ke_association": {"rows": {}}}, self.identity)), \
                        patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=7 * 1024**3)), \
                        patch.object(MODULE.COMMON, "Session", Session), \
                        patch.object(MODULE, "address_storage_usage", side_effect=measured_usage):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.execute_engine_ke_profile(self.args, report)
                self.assertEqual(calls, ["ke-format", "ke-windows-check", "ke-baseline", "ke-clippy", "ke-release", "finish"])
                self.assertEqual(artifact.read_text(), "completed or failed release evidence")
                self.assertNotIn("release_target_retirement", report)
                if mode == "storage":
                    self.assertEqual((report["storage_samples"][-1]["lane"], report["storage_samples"][-1]["phase"],
                                      report["storage_samples"][-1]["allocated_bytes"]), ("ke-release", "after", 3221225472))

class EngineKeProviderCompilerContractTests(unittest.TestCase):
    """Synthetic Cargo and custody controls; never execute a compiler or product."""

    FILENAME = "crates/nevrotick-engine/src/nts_ke_experiment/process.rs"
    MANIFEST = "/work/provider-contract-source/crates/nevrotick-engine/Cargo.toml"
    EXPLANATION = "For more information about this error, try `rustc --explain E0382`."
    SPECIMEN = {"code": "E0382", "tokens": ["claim", "moved"], "line": 42}

    def diagnostic(self, *, line=42):
        """Literal rustc moved-value record with an independently selected span."""
        return {"$message_type": "diagnostic", "message": "use of moved value: `claim`",
            "code": {"code": "E0382", "explanation": None}, "level": "error", "children": [], "rendered": None,
            "spans": [{"file_name": self.FILENAME, "byte_start": 20, "byte_end": 25,
                "line_start": line, "line_end": line, "column_start": 1, "column_end": 6,
                "is_primary": True, "text": [{"text": "claim", "highlight_start": 1, "highlight_end": 6}],
                "label": "value used here after move", "suggested_replacement": None,
                "suggestion_applicability": None, "expansion": None}]}

    def envelope(self, diagnostic):
        return {"reason": "compiler-message",
            "package_id": "path+file:///work/provider-contract-source/crates/nevrotick-engine#0.0.0",
            "manifest_path": self.MANIFEST,
            "target": {"name": "nevrotick_engine", "kind": ["lib"], "crate_types": ["lib"],
                "src_path": "/work/provider-contract-source/crates/nevrotick-engine/src/lib.rs",
                "edition": "2024", "doc": True, "doctest": True, "test": True}, "message": diagnostic}

    def records(self, *, line=42, rendered=None):
        note = {"$message_type": "diagnostic", "message": self.EXPLANATION, "code": None,
            "level": "failure-note", "spans": [], "children": [], "rendered": rendered}
        return [self.envelope(self.diagnostic(line=line)), self.envelope(note),
                {"reason": "build-finished", "success": False}]

    def result(self, records=None, *, errors=1):
        text = "\n".join(json.dumps(item) for item in (self.records() if records is None else records))
        return {"exit_code": 101, "failure": None, "output": text +
            f"\nerror: could not compile `nevrotick-engine` (lib test) due to {errors} previous error" +
            ("s" if errors != 1 else "") + "\n"}

    def test_examples_use_only_actual_consuming_constructors_and_genuine_claims(self):
        original = b"// unchanged private owner\n// second unchanged line\n"
        examples = MODULE.engine_ke_provider_contract_examples(original)
        self.assertEqual([(row["target"], row["case"]) for row in examples],
            [(target, case) for target in ("gnu", "msvc") for case in ("once", "overlap", "after-drop")])
        for row in examples:
            source = row["source"].decode()
            constructor = "component" if row["target"] == "gnu" else "new"
            self.assertEqual(row["constructor"], "ProviderScope::" + constructor)
            self.assertEqual(row["source"], original + row["appended_source"].encode())
            self.assertIn("#[cfg(all(test, " + ("not(windows)" if row["target"] == "gnu" else "windows") + "))]", source)
            self.assertIn("admission.claim()?" if row["target"] == "gnu" else "PROCESS_ADMISSION.claim()?", source)
            self.assertNotIn("struct ProcessClaim", source)
            self.assertNotIn("ProcessClaim {", source)
            self.assertNotIn("#[allow", source)
            self.assertIn(f"let first = ProviderScope::{constructor}(claim);", source)
            if row["case"] == "once":
                self.assertIsNone(row["diagnostic"])
                self.assertNotIn("let second", source)
            else:
                self.assertEqual(row["diagnostic"]["code"], "E0382")
                self.assertEqual(source.splitlines()[row["diagnostic"]["line"] - 1],
                    f"        let second = ProviderScope::{constructor}(claim);")
                self.assertEqual(source.index("drop(first);") < source.index("let second")
                    if row["case"] == "after-drop" else "drop((first, second));" in source, True)
        for invalid in (b"", b"missing final newline", b"\xff\n", b"x" * (256 * 1024) + b"\n",
                        original + b"mod provider_claim_contract_probe {}\n", original.decode()):
            with self.subTest(invalid=repr(invalid)[:60]), self.assertRaises(MODULE.Incomplete):
                MODULE.engine_ke_provider_contract_examples(invalid)

    def test_exact_cargo_tail_is_checked_and_only_the_actual_error_reaches_common(self):
        for rendered in (None, self.EXPLANATION + "\n"):
            result = self.result(self.records(rendered=rendered))
            raw = copy.deepcopy(result)
            with patch.object(MODULE.COMMON, "compiler_cause", wraps=MODULE.COMMON.compiler_cause) as matcher:
                MODULE.validate_engine_ke_provider_cause(result, self.SPECIMEN, self.MANIFEST)
            normalized, specimen, filename = matcher.call_args.args
            self.assertEqual(normalized["exit_code"], 101)
            self.assertEqual([json.loads(line)["level"] for line in normalized["output"].splitlines()], ["error"])
            self.assertNotIn("aborting due", normalized["output"])
            self.assertEqual(specimen, self.SPECIMEN)
            self.assertEqual(filename, self.FILENAME)
            self.assertEqual(result, raw, "normalization must preserve the retained raw Cargo result")

    def test_foreign_attribution_wrong_semantics_and_infrastructure_cannot_supply_a_cause(self):
        changes = [lambda rows: rows[0].update(package_id="registry+https://example#nevrotick-engine@0.0.0"),
            lambda rows: rows[0].update(manifest_path="/work/foreign/Cargo.toml"),
            lambda rows: rows[0]["target"].update(name="foreign"),
            lambda rows: rows[0]["target"].update(kind=["test"]),
            lambda rows: rows[0]["target"].update(src_path="/work/foreign/lib.rs"),
            lambda rows: rows[0]["message"]["code"].update(code="E0308"),
            lambda rows: rows[0]["message"]["spans"][0].update(line_start=43, line_end=43),
            lambda rows: rows[0]["message"]["spans"][0].update(file_name="foreign.rs"),
            lambda rows: rows[0]["message"].update(message="unrelated mismatched types", children=[]),
            lambda rows: rows[0]["message"].update(message="use of moved claim: failed to spawn compiler"),
            lambda rows: rows[0].update(unexpected="unqualified envelope field")]
        for change in changes:
            rows = self.records()
            change(rows)
            if "unrelated" in rows[0]["message"]["message"]:
                rows[0]["message"]["spans"][0]["label"] = None
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_provider_cause(self.result(rows), self.SPECIMEN, self.MANIFEST)
        rows = self.records()
        rows.insert(1, self.envelope(self.diagnostic()))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_provider_cause(self.result(rows, errors=2), self.SPECIMEN, self.MANIFEST)

    def test_terminal_note_cannot_hide_extra_diagnostics_or_supply_the_cause(self):
        changes = [lambda rows: rows[1]["message"].update(message="foreign explanation"),
            lambda rows: rows[1]["message"].update(rendered="hidden rendered contamination"),
            lambda rows: rows[1]["message"].update(children=[{}]),
            lambda rows: rows[1]["message"].update(spans=[{}]),
            lambda rows: rows[1]["message"].update(code={"code": "E0382", "explanation": None}),
            lambda rows: rows[1]["message"].update(extra=True),
            lambda rows: rows.insert(2, copy.deepcopy(rows[0])),
            lambda rows: rows.insert(2, copy.deepcopy(rows[1])),
            lambda rows: rows.pop(1), lambda rows: rows.pop(0),
            lambda rows: rows.reverse()]
        for change in changes:
            rows = self.records()
            change(rows)
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_provider_cause(self.result(rows), self.SPECIMEN, self.MANIFEST)

    def test_malformed_stdout_unknown_records_and_noncompiler_outcomes_fail_closed(self):
        healthy = self.result()
        bad_outputs = [healthy["output"] + "unexpected stdout\n", healthy["output"] + "{}\n",
            healthy["output"].replace('"reason": "compiler-message"', '"reason": "compiler-message", "reason": "compiler-message"', 1),
            healthy["output"].replace('"rendered": null', '"rendered": NaN', 1),
            '{"reason":' + "[" * 1100 + "0" + "]" * 1100 + "}",
            "x" * (4 * 1024 * 1024 + 1)]
        for records in ([{"reason": "foreign-record"}, *self.records()], self.records()[:-1],
                        [*self.records()[:-1], {"reason": "build-finished", "success": True}],
                        [*self.records(), {"reason": "build-finished", "success": False}],
                        [{"reason": "compiler-artifact"}, *self.records()],
                        [{"reason": "build-script-executed"}, *self.records()]):
            bad_outputs.append(self.result(records)["output"])
        bad = [{**healthy, "output": output} for output in bad_outputs]
        bad += [{**healthy, "exit_code": code} for code in (0, 1, -9, True, None)]
        bad += [{**healthy, "failure": "wall timeout"}, {**healthy, "output": b"not text"},
                {key: value for key, value in healthy.items() if key != "failure"}]
        for result in bad:
            with self.subTest(sample=str(result)[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_provider_cause(result, self.SPECIMEN, self.MANIFEST)
        MODULE.validate_engine_ke_provider_cause(healthy, self.SPECIMEN, self.MANIFEST)

    def session_fixture(self):
        """Use real Session.check over inert files; substitute only candidate admission."""
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        base = Path(temporary.name)
        root = base / "checkout"
        original = root / self.FILENAME
        original.parent.mkdir(parents=True)
        original.write_bytes(b"// synthetic unchanged private source; never compiled\n")
        (root / "crates/nevrotick-engine/Cargo.toml").write_text("# inert manifest; never compiled\n")
        work = base / "work"
        work.mkdir(mode=0o700)
        source = work / "source"
        shutil.copytree(root, source)
        session = MODULE.COMMON.Session.__new__(MODULE.COMMON.Session)
        session.root, session.work, session.source = root, work, source
        session.identity = MODULE.PRODUCT.inventory(root)
        session.product_identity = dict(session.identity)
        session.source_inventory = MODULE.PRODUCT.inventory(source)
        session.protected_tools, session.protected_islands = {}, {}
        session.config = work / "cargo-config.toml"
        session.config.write_text("# immutable inert configuration\n")
        session.config_sha256 = MODULE.PRODUCT.sha256(session.config)
        session.protect = lambda _path: self.fail("controlled deltas must never bless a fresh current inventory")
        return session, {"lanes": []}, {"baseline_wall_seconds": 300}

    def fake_run(self, session, report, calls):
        """Supply literal synthetic child results after checking the sealed islands."""
        def run(label, command, timeout, **options):
            session.check()
            private = session.work / "provider-contract-source"
            self.assertEqual(options["cwd"], private)
            self.assertIn(private, session.protected_islands)
            source = (private / self.FILENAME).read_text()
            calls.append((label, command, timeout, options, source))
            negative = options.get("negative")
            if negative:
                self.assertEqual(negative, '"E0382"')
                second = next(index + 1 for index, line in enumerate(source.splitlines()) if "let second =" in line)
                result = self.result(self.records(line=second))
            else:
                result = {"exit_code": 0, "failure": None,
                    "output": '{"reason":"build-finished","success":true}\n'}
            # Session retains the actual namespace/prlimit argv, including its final '--'.
            result.update(lane=label, command=["synthetic-bwrap", "--", "synthetic-prlimit", "--", *command])
            report["lanes"].append(result)
            session.check()
            return result["output"] if negative else result["output"].strip()
        return run

    def test_six_metadata_commands_share_one_deadline_copy_and_exact_restoration(self):
        session, report, policy = self.session_fixture()
        original = (session.source / self.FILENAME).read_bytes()
        calls = []
        with patch.object(MODULE.COMMON, "require_current_candidate"), \
                patch.object(MODULE.COMMON, "remaining_timeout", wraps=MODULE.COMMON.remaining_timeout) as remaining:
            MODULE.engine_ke_provider_contracts(session, policy, report, self.fake_run(session, report, calls))
            session.check()
        self.assertEqual(len(calls), 6)
        self.assertEqual(len({row.args[0] for row in remaining.call_args_list}), 1)
        for index, (label, command, timeout, options, source) in enumerate(calls):
            self.assertEqual(command[:4], ["cargo", "check", "--package", "nevrotick-engine"])
            for token in ("--all-targets", "--frozen", "--message-format=json", "--quiet"):
                self.assertIn(token, command)
            self.assertEqual(command[command.index("--jobs") + 1], "6")
            self.assertEqual(command[command.index("--features") + 1], "ke-association-prototype")
            self.assertEqual(options.get("negative") is None, index in (0, 3))
            self.assertGreater(timeout, 0)
            self.assertLessEqual(timeout, 300)
            self.assertNotIn("run", command)
            self.assertNotIn("test", command)
            if index >= 3:
                self.assertEqual(command[command.index("--target") + 1], "x86_64-pc-windows-msvc")
            else:
                self.assertNotIn("--target", command)
        private = session.work / "provider-contract-source"
        self.assertEqual(MODULE.PRODUCT.inventory(private), session.source_inventory)
        self.assertEqual((session.source / self.FILENAME).read_bytes(), original)
        self.assertEqual((session.root / self.FILENAME).read_bytes(), original)
        self.assertEqual(len(report["provider_claim_contracts"]["probes"]), 6)
        self.assertNotIn("qualification_passed", report)
        self.assertNotIn("contracts_qualification_passed", report)

    def test_compiler_failure_and_retained_command_mismatch_restore_the_only_changed_source(self):
        for failure in ("compiler", "lane", "command", "output"):
            session, report, policy = self.session_fixture()
            calls = []
            ordinary = self.fake_run(session, report, calls)
            def run(label, command, timeout, **options):
                if failure == "compiler":
                    raise MODULE.Incomplete("synthetic compiler/infrastructure refusal")
                result = ordinary(label, command, timeout, **options)
                if options.get("negative"):
                    key = "output" if failure == "output" else failure
                    report["lanes"][-1][key] = "foreign retained result"
                return result
            with self.subTest(failure=failure), patch.object(MODULE.COMMON, "require_current_candidate"):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.engine_ke_provider_contracts(session, policy, report, run)
                session.check()
            private = session.work / "provider-contract-source"
            self.assertEqual(MODULE.PRODUCT.inventory(private), session.source_inventory)
            self.assertNotIn("provider_claim_contracts_passed", report)

    def test_expired_deadline_restores_the_probe_before_any_child_can_run(self):
        session, report, policy = self.session_fixture()
        with patch.object(MODULE.COMMON, "require_current_candidate"), \
                patch.object(MODULE.COMMON, "remaining_timeout", side_effect=MODULE.Incomplete("shared deadline exhausted")), \
                patch.object(session, "run", create=True) as run:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.engine_ke_provider_contracts(session, policy, report, run)
            run.assert_not_called()
            session.check()
        self.assertEqual(MODULE.PRODUCT.inventory(session.work / "provider-contract-source"), session.source_inventory)

    def test_unexpected_initial_copy_bytes_refuse_before_the_first_command(self):
        session, report, policy = self.session_fixture()
        actual_copy = MODULE.shutil.copytree
        def changed_copy(source, destination, *arguments, **options):
            result = actual_copy(source, destination, *arguments, **options)
            if Path(destination) == session.work / "provider-contract-source":
                (Path(destination) / "unexpected-input").write_text("unexplained initial copy drift\n")
            return result
        with patch.object(MODULE.COMMON, "require_current_candidate"), \
                patch.object(MODULE.shutil, "copytree", side_effect=changed_copy), \
                patch.object(session, "run", create=True) as run:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.engine_ke_provider_contracts(session, policy, report, run)
            run.assert_not_called()
            session.check()
        self.assertEqual(session.protected_islands, {})

    def test_unexpected_copy_drift_cannot_be_blessed_by_rebinding_the_island(self):
        session, report, policy = self.session_fixture()
        calls = []
        ordinary = self.fake_run(session, report, calls)
        def run(label, command, timeout, **options):
            result = ordinary(label, command, timeout, **options)
            private = session.work / "provider-contract-source"
            (private / "unexpected-input").write_text("unexplained copy drift\n")
            session.protected_islands[private] = MODULE.PRODUCT.inventory(private)
            return result
        with patch.object(MODULE.COMMON, "require_current_candidate"), self.assertRaises(MODULE.Incomplete):
            MODULE.engine_ke_provider_contracts(session, policy, report, run)
        private = session.work / "provider-contract-source"
        self.assertEqual((private / self.FILENAME).read_bytes(), (session.source / self.FILENAME).read_bytes())
        self.assertTrue((private / "unexpected-input").is_file(), "unexplained evidence must remain visible")
        self.assertEqual(report["provider_claim_contracts"]["probes"], [])


class EngineKeHistoryCompilerContractTests(unittest.TestCase):
    """Independent literal compiler envelopes and real custody checks; no Cargo execution."""

    FILENAME = "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs"
    MANIFEST = "/work/history-contract-source/crates/nevrotick-engine/Cargo.toml"
    CASES = {
        "copy-completion": ("E0277", ["InvocationCompletion", "Copy"],
            "the trait bound `InvocationCompletion: Copy` is not satisfied", "require_copy::<InvocationCompletion>();"),
        "clone-completion": ("E0277", ["InvocationCompletion", "Clone"],
            "the trait bound `InvocationCompletion: Clone` is not satisfied", "let duplicate = <InvocationCompletion as Clone>"),
        "copy-history": ("E0277", ["ProtectedHistory", "Copy"],
            "the trait bound `ProtectedHistory: Copy` is not satisfied", "require_copy::<datagram::ProtectedHistory>();"),
        "clone-history": ("E0277", ["ProtectedHistory", "Clone"],
            "the trait bound `ProtectedHistory: Clone` is not satisfied", "let duplicate = <datagram::ProtectedHistory as Clone>"),
        "second-consume-completion": ("E0382", ["completion", "moved"],
            "use of moved value: `completion`", "let second = completion;"),
        "raw-history": ("E0451", ["ProtectedHistory", "summary", "receipt", "population", "send_before",
            "send_after", "read_return", "local", "remote", "identity", "run_context", "private"],
            "fields summary, receipt, population, send_before, send_after, read_return, local, remote, identity, run_context of struct ProtectedHistory are private",
            "let duplicate = datagram::ProtectedHistory {"),
        "raw-receiving": ("E0451", ["joined", "private"],
            "field `joined` of struct `ReceivingOutcome` is private",
            "let receiving = job::ReceivingOutcome { joined };"),
        "mutate-receiving": ("E0616", ["joined", "private"],
            "field `joined` of struct `ReceivingOutcome` is private",
            "receiving.joined.retired = true;"),
        "clone-receiving": ("E0277", ["Clone"],
            "the trait bound `ReceivingOutcome: Clone` is not satisfied", "require_clone(&receiving);"),
        "second-consume-receiving": ("E0382", ["receiving", "moved"],
            "use of moved value: `receiving`", "let second = receiving.into_normal();"),
        "read-received-history": ("E0616", ["history", "private"],
            "field `history` of struct `ReceivedProtectedNumerics` is private", "let _ = &received.history;"),
        "read-received-measurement": ("E0616", ["measurement", "private"],
            "field `measurement` of struct `ReceivedProtectedNumerics` is private", "let _ = &received.measurement;"),
        "mutate-received-profile": ("E0616", ["profile", "private"],
            "field `profile` of struct `ReceivedProtectedNumerics` is private", "received.profile = None;"),
        "clone-received-numerics": ("E0277", ["Clone"],
            "the trait bound `ReceivedProtectedNumerics: Clone` is not satisfied", "require_clone(received);"),
    }
    LEGAL_CASES = ("legal", "legal-receiving", "legal-received")
    ORDERED_CASES = (
        "legal", "copy-completion", "clone-completion", "copy-history", "clone-history",
        "second-consume-completion", "raw-history",
        "legal-receiving", "raw-receiving", "mutate-receiving", "clone-receiving",
        "second-consume-receiving",
        "legal-received", "read-received-history", "read-received-measurement",
        "mutate-received-profile", "clone-received-numerics",
    )
    session_fixture = EngineKeProviderCompilerContractTests.session_fixture

    def cargo_result(self, case, *, line=42):
        """Use independently declared semantic text, actual library attribution and exact tails."""
        code, _tokens, message, _needle = self.CASES[case]
        diagnostic = EngineKeProviderCompilerContractTests.diagnostic(self, line=line)
        diagnostic.update(message=message, code={"code": code, "explanation": None})
        diagnostic["spans"][0].update(label="actual ownership refusal", text=[{
            "text": "actual consumer", "highlight_start": 1, "highlight_end": 7}])
        parent = str(Path(self.MANIFEST).parent)
        def envelope(value):
            return {"reason": "compiler-message", "package_id": "path+file://" + parent + "#0.0.0",
                "manifest_path": self.MANIFEST,
                "target": {"name": "nevrotick_engine", "kind": ["lib"], "crate_types": ["lib"],
                    "src_path": parent + "/src/lib.rs", "edition": "2024", "doc": True, "doctest": True, "test": True},
                "message": value}
        note = {"$message_type": "diagnostic", "message": f"For more information about this error, try `rustc --explain {code}`.",
            "code": None, "level": "failure-note", "spans": [], "children": [], "rendered": None}
        records = [envelope(diagnostic), envelope(note), {"reason": "build-finished", "success": False}]
        return {"exit_code": 101, "failure": None, "output": "\n".join(json.dumps(row) for row in records) +
            "\nerror: could not compile `nevrotick-engine` (lib test) due to 1 previous error\n"}

    def specimen(self, case, *, line=42):
        """The test's fixed matrix is independent of the production generator."""
        code, tokens, _message, _needle = self.CASES[case]
        return {"code": code, "tokens": list(tokens), "line": line}

    def test_examples_preserve_old_owners_and_reach_new_private_receiving_rules(self):
        original = b"// actual unchanged owner\n// source prefix\n"
        all_examples = MODULE.engine_ke_history_contract_examples(original)
        self.assertEqual(len(all_examples), 43)
        examples = [row for row in all_examples if row["case"] in self.ORDERED_CASES]
        self.assertEqual([(row["target"], row["case"]) for row in examples],
            [(target, case) for target in ("gnu", "msvc") for case in self.ORDERED_CASES])
        self.assertEqual(len(examples), 34)
        for row in examples:
            source, probe = row["source"].decode(), row["appended_source"]
            self.assertEqual(row["source"], original + probe.encode())
            self.assertIn("#[cfg(all(test, " + ("not(windows)" if row["target"] == "gnu" else "windows") + "))]", probe)
            old_owner = row["case"] in self.ORDERED_CASES[:7]
            if old_owner:
                self.assertIn("const _: fn(InvocationCompletion, datagram::ProtectedHistory)", probe)
                for method in ("summary", "receipt", "population", "captures", "endpoints", "identity", "run_context"):
                    self.assertIn("history." + method + "()", probe)
            for forbidden in ("struct ProtectedHistory", "enum InvocationCompletion", "mod reached", "ProviderScope",
                              "PROCESS_ADMISSION", "zeroed", "#[allow", "#[test]"):
                self.assertNotIn(forbidden, probe)
            if row["case"] in self.LEGAL_CASES:
                self.assertIsNone(row["diagnostic"])
                if row["case"] == "legal":
                    self.assertIn("(completion, history)", probe)
                elif row["case"] == "legal-receiving":
                    for statement in ("slot.collect().map_err(RunFailed::Collection)?;",
                            "slot.take_for_receiving().map_err(RunFailed::Collection)?;",
                            "datagram::receive_joined(receiving, profile)",
                            "receiving.into_normal()"):
                        self.assertIn(statement, probe)
                    self.assertNotIn("JoinedOutcome {", probe)
                else:
                    for method in ("history", "profile", "measurement"):
                        self.assertIn("received." + method + "()", probe)
                    self.assertIn("if let Some(received) = completion.protected()", probe)
                continue
            expected = self.specimen(row["case"], line=row["diagnostic"]["line"])
            self.assertEqual(row["diagnostic"], expected)
            actual_line = source.splitlines()[expected["line"] - 1]
            self.assertIn(self.CASES[row["case"]][3], actual_line)
            self.assertGreater(expected["line"], len(original.splitlines()))
            if row["case"] == "second-consume-completion":
                self.assertIn("let first = completion;", probe)
                self.assertIn("(first, history)", probe)
                self.assertNotIn("(completion, history)\n", probe)
            if row["case"] == "raw-history":
                for field in ("summary:", "receipt:", "population:", "send_before", "send_after", "read_return",
                              "local", "remote", "identity:", "run_context:"):
                    self.assertIn(field, actual_line)
                self.assertNotIn("..", actual_line)
                self.assertIn("Arc::clone(history.run_context())", actual_line)
                self.assertNotIn("std::sync::Arc", actual_line,
                    "the actual parent already imports Arc; unrelated lint failures cannot qualify privacy")
            if row["case"] == "second-consume-receiving":
                self.assertIn("let first = receiving.into_normal();", probe)
                self.assertIn("drop((first, second));", probe)
            if row["case"].startswith(("read-received-", "mutate-received-", "clone-received-")):
                self.assertIn("if let Some(received) = completion.protected()", probe)
                self.assertNotIn("ReceivedCompletion::Protected", probe)
                self.assertNotIn("datagram::ReceivedProtectedNumerics", probe,
                    "the carrier type is inferred through its real readonly accessor")
            if row["case"] == "clone-received-numerics":
                self.assertIn("require_clone(received);", probe)
                self.assertNotIn("require_clone(&received);", probe,
                    "a reference's Clone must not masquerade as the payload's Clone")
            if row["case"] == "mutate-receiving":
                self.assertIn("fn(&mut job::ReceivingOutcome)", probe)
            if row["case"] == "mutate-received-profile":
                self.assertIn("fn(&datagram::ReceivedCompletion)", probe)
                self.assertNotIn("protected_mut", probe)
            self.assertNotIn("|mut ", probe, "privacy refusal must not add an unrelated unused-mut lint")
        for invalid in (b"", b"no newline", b"\xff\n", b"x" * (256 * 1024) + b"\n",
                        original + b"// protected_history_contract_probe\n", original.decode()):
            with self.subTest(invalid=repr(invalid)[:60]), self.assertRaises(MODULE.Incomplete):
                MODULE.engine_ke_history_contract_examples(invalid)

    def test_each_closed_cause_reaches_unchanged_matcher_without_broadening_provider(self):
        for case in self.CASES:
            result, specimen = self.cargo_result(case), self.specimen(case)
            retained = copy.deepcopy(result)
            with self.subTest(case=case), patch.object(MODULE.COMMON, "compiler_cause", wraps=MODULE.COMMON.compiler_cause) as matcher:
                MODULE.validate_engine_ke_history_cause(result, specimen, self.MANIFEST)
            self.assertEqual(matcher.call_args.args[1:], (specimen, self.FILENAME))
            self.assertEqual(result, retained)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_provider_cause(result, specimen, self.MANIFEST)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_history_cause(self.cargo_result("second-consume-completion"),
                {"code": "E0382", "tokens": ["claim", "moved"], "line": 42}, self.MANIFEST)

    def test_new_privacy_causes_require_the_field_not_an_incidental_display_type(self):
        for case in ("raw-receiving", "mutate-receiving", "read-received-history",
                     "read-received-measurement", "mutate-received-profile"):
            healthy = self.cargo_result(case)
            records = [json.loads(line) for line in healthy["output"].splitlines() if line.startswith("{")]
            message = records[0]["message"]
            message["message"] = message["message"].replace("ReceivingOutcome", "display::OpaqueJoin") \
                .replace("ReceivedProtectedNumerics", "display::OpaqueCalculation")
            def result(rows):
                return {**healthy, "output": "\n".join(json.dumps(row) for row in rows) +
                    "\nerror: could not compile `nevrotick-engine` (lib test) due to 1 previous error\n"}
            with self.subTest(case=case):
                MODULE.validate_engine_ke_history_cause(result(records), self.specimen(case), self.MANIFEST)
                for missing in (self.CASES[case][1][0], "private"):
                    wrong = copy.deepcopy(records)
                    wrong[0]["message"]["message"] = wrong[0]["message"]["message"].replace(missing, "unrelated")
                    with self.subTest(missing=missing), self.assertRaises(MODULE.Incomplete):
                        MODULE.validate_engine_ke_history_cause(result(wrong), self.specimen(case), self.MANIFEST)

    def test_every_new_negative_has_its_real_owner_healthy_companion(self):
        examples = MODULE.engine_ke_history_contract_examples(b"// actual unchanged owner\n")
        companion = {
            "raw-receiving": "legal-receiving", "mutate-receiving": "legal-receiving",
            "clone-receiving": "legal-receiving", "second-consume-receiving": "legal-receiving",
            "read-received-history": "legal-received", "read-received-measurement": "legal-received",
            "mutate-received-profile": "legal-received", "clone-received-numerics": "legal-received",
        }
        for target in ("gnu", "msvc"):
            selected = [row for row in examples if row["target"] == target]
            for case, healthy_case in companion.items():
                with self.subTest(target=target, case=case):
                    negative = next(row for row in selected if row["case"] == case)
                    healthy = next(row for row in selected if row["case"] == healthy_case)
                    self.assertEqual(negative["owner"], healthy["owner"])
                    self.assertIsNone(healthy["diagnostic"])
                    self.assertIsNotNone(negative["diagnostic"])
                    self.assertLess(selected.index(healthy), selected.index(negative))

    def test_wrong_cause_attribution_extra_errors_and_split_private_spans_refuse(self):
        for case in self.CASES:
            healthy = self.cargo_result(case)
            records = [json.loads(line) for line in healthy["output"].splitlines() if line.startswith("{")]
            changes = [lambda rows: rows[0].update(manifest_path="/work/foreign/Cargo.toml"),
                lambda rows: rows[0]["message"].update(message="unrelated type or trait refusal"),
                lambda rows: rows[0]["message"]["code"].update(code="E0603"),
                lambda rows: rows[0]["message"]["spans"][0].update(file_name="foreign.rs"),
                lambda rows: rows[0]["message"]["spans"][0].update(line_start=43, line_end=43),
                lambda rows: rows[0]["message"]["spans"].append({**rows[0]["message"]["spans"][0], "line_start": 43, "line_end": 43}),
                lambda rows: rows.insert(1, copy.deepcopy(rows[0])),
                lambda rows: rows[1]["message"].update(message="unrelated explanation"),
                lambda rows: rows[1]["message"].update(children=[{}]),
                lambda rows: rows[-1].update(success=True),
                lambda rows: rows[0]["message"].update(message=rows[0]["message"]["message"] + "; failed to spawn compiler")]
            for change in changes:
                changed = copy.deepcopy(records)
                change(changed)
                result = {**healthy, "output": "\n".join(json.dumps(row) for row in changed) +
                    "\nerror: could not compile `nevrotick-engine` (lib test) due to 1 previous error\n"}
                with self.subTest(case=case, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_engine_ke_history_cause(result, self.specimen(case), self.MANIFEST)
        for specimen in ({"code": "E0603", "tokens": ["private"], "line": 42},
                         {"code": "E0277", "tokens": ["ProtectedHistory", "Default"], "line": 42},
                         {"code": "E0277", "tokens": ["InvocationCompletion", "Copy"], "line": True}):
            with self.subTest(specimen=specimen), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_history_cause(self.cargo_result("copy-completion"), specimen, self.MANIFEST)

    def fake_run(self, session, report, calls):
        """Return synthetic literal outcomes while real Session.check protects both sources."""
        def run(label, command, timeout, **options):
            session.check()
            private = session.work / "history-contract-source"
            self.assertEqual(options["cwd"], private)
            source = (private / self.FILENAME).read_text()
            target, case = label.removeprefix("ke-history-contract-").split("-", 1)
            calls.append((target, case, command, timeout, options, source))
            projection = case in EngineKeProjectionCompilerContractTests.ORDERED_CASES
            fixture = EngineKeProjectionCompilerContractTests() if projection else self
            if case in fixture.LEGAL_CASES:
                self.assertNotIn("negative", options)
                result = {"exit_code": 0, "failure": None, "output": '{"reason":"build-finished","success":true}\n'}
            else:
                code, _tokens, _message, needle = fixture.CASES[case]
                self.assertEqual(options["negative"], json.dumps(code))
                line = next(index + 1 for index, value in enumerate(source.splitlines()) if needle in value)
                result = fixture.cargo_result(case, line=line)
            result.update(lane=label, command=["synthetic-bwrap", "--", "synthetic-prlimit", "--", *command])
            report["lanes"].append(result)
            session.check()
            return result["output"]
        return run

    def test_metadata_commands_share_deadline_and_restore_each_single_delta(self):
        session, report, policy = self.session_fixture()
        calls = []
        with patch.object(MODULE.COMMON, "require_current_candidate"), \
                patch.object(MODULE.COMMON, "remaining_timeout", wraps=MODULE.COMMON.remaining_timeout) as remaining:
            MODULE.engine_ke_history_contracts(session, policy, report, self.fake_run(session, report, calls))
            session.check()
        self.assertEqual([(row[0], row[1]) for row in calls],
            [(target, case) for target in ("gnu", "msvc") for case in self.ORDERED_CASES]
            + [("msvc", case) for case in EngineKeProjectionCompilerContractTests.ORDERED_CASES])
        self.assertEqual(len({row.args[0] for row in remaining.call_args_list}), 1)
        for target, case, command, timeout, _options, _source in calls:
            self.assertEqual(command[:4], ["cargo", "check", "--package", "nevrotick-engine"])
            for token in ("--frozen", "--message-format=json", "--quiet"):
                self.assertIn(token, command)
            production = case in EngineKeProjectionCompilerContractTests.ORDERED_CASES
            self.assertIn("--lib" if production else "--all-targets", command)
            self.assertNotIn("--all-targets" if production else "--lib", command)
            self.assertEqual(command[command.index("--jobs") + 1], "6")
            self.assertEqual(command[command.index("--features") + 1], "ke-association-prototype")
            self.assertGreater(timeout, 0)
            self.assertLessEqual(timeout, 300)
            self.assertNotIn("test", command)
            if target == "msvc":
                self.assertEqual(command[command.index("--target") + 1], "x86_64-pc-windows-msvc")
            else:
                self.assertNotIn("--target", command)
        self.assertEqual(MODULE.PRODUCT.inventory(session.work / "history-contract-source"), session.source_inventory)
        self.assertEqual(MODULE.PRODUCT.inventory(session.source), session.source_inventory)
        self.assertEqual(MODULE.PRODUCT.inventory(session.root), session.identity)
        self.assertEqual(len(report["protected_history_contracts"]["probes"]), 43)
        production = [row for row in report["protected_history_contracts"]["probes"] if "compile_scope" in row]
        self.assertEqual(len(production), 9)
        self.assertTrue(all(row["compile_scope"] == "Windows production library, no cfg(test)" for row in production))
        self.assertNotIn("protected_history_contracts_passed", report)
        self.assertNotIn("provider_claim_contracts", report)

    def test_compiler_retained_result_and_deadline_failures_restore_history_source(self):
        for failure in ("compiler", "lane", "command", "output", "deadline", "healthy-lane", "healthy-command",
                        "healthy-output", "healthy-exit", "healthy-failure", "healthy-missing-failure"):
            session, report, policy = self.session_fixture()
            calls = []
            ordinary = self.fake_run(session, report, calls)
            def run(label, command, timeout, **options):
                if failure == "compiler":
                    raise MODULE.Incomplete("synthetic child failure")
                raw = ordinary(label, command, timeout, **options)
                if options.get("negative"):
                    report["lanes"][-1][failure] = "foreign retained result"
                elif failure.startswith("healthy-"):
                    key = failure.removeprefix("healthy-")
                    row = report["lanes"][-1]
                    if key == "exit":
                        row["exit_code"] = 1
                    elif key == "missing-failure":
                        row.pop("failure")
                    else:
                        row[key] = "foreign retained healthy result"
                return raw
            timeout = MODULE.Incomplete("shared deadline exhausted") if failure == "deadline" else None
            with self.subTest(failure=failure), patch.object(MODULE.COMMON, "require_current_candidate"), \
                    patch.object(MODULE.COMMON, "remaining_timeout", side_effect=timeout,
                        wraps=None if timeout else MODULE.COMMON.remaining_timeout):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.engine_ke_history_contracts(session, policy, report, run)
                session.check()
            self.assertEqual(MODULE.PRODUCT.inventory(session.work / "history-contract-source"), session.source_inventory)
            self.assertNotIn("protected_history_contracts_passed", report)

    def test_history_copy_drift_and_incomplete_roster_refuse_without_blessing(self):
        session, report, policy = self.session_fixture()
        calls = []
        ordinary = self.fake_run(session, report, calls)
        def drift(label, command, timeout, **options):
            raw = ordinary(label, command, timeout, **options)
            private = session.work / "history-contract-source"
            (private / "unexpected-input").write_text("unexplained drift\n")
            session.protected_islands[private] = MODULE.PRODUCT.inventory(private)
            return raw
        with patch.object(MODULE.COMMON, "require_current_candidate"), self.assertRaises(MODULE.Incomplete):
            MODULE.engine_ke_history_contracts(session, policy, report, drift)
        private = session.work / "history-contract-source"
        self.assertEqual((private / self.FILENAME).read_bytes(), (session.source / self.FILENAME).read_bytes())
        self.assertTrue((private / "unexpected-input").is_file())
        self.assertEqual(report["protected_history_contracts"]["probes"], [])
        session, report, policy = self.session_fixture()
        examples = MODULE.engine_ke_history_contract_examples((session.source / self.FILENAME).read_bytes())[:-1]
        with patch.object(MODULE.COMMON, "require_current_candidate"), \
                patch.object(MODULE, "engine_ke_history_contract_examples", return_value=examples), \
                patch.object(session, "run", create=True) as run, self.assertRaises(MODULE.Incomplete):
            MODULE.engine_ke_history_contracts(session, policy, report, run)
        run.assert_not_called()
        self.assertFalse((session.work / "history-contract-source").exists())


class EngineKeProjectionCompilerContractTests(unittest.TestCase):
    """Independent production parent/sibling consumers and strict compiler envelopes."""

    FILENAME = "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs"
    MANIFEST = "/work/history-contract-source/crates/nevrotick-engine/Cargo.toml"
    CASES = {
        "projection-parent-preview": ("E0599", ["ReceivedCompletion", "Preview"],
            "no associated item named `Preview` found for struct `ReceivedCompletion`", "datagram::ReceivedCompletion::Preview(summary)"),
        "projection-parent-inner-mut": ("E0616", ["inner", "private"],
            "field `inner` of struct `ReceivedCompletion` is private", "let _inner = &mut completion.inner;"),
        "projection-parent-literal": ("E0451", ["completion", "private"],
            "field `completion` of struct `CompletionProjection` is private", "datagram::CompletionProjection { completion }"),
        "projection-parent-field": ("E0616", ["completion", "private"],
            "field `completion` of struct `CompletionProjection` is private", "let _ = &projection.completion;"),
        "projection-parent-clone": ("E0277", ["CompletionProjection", "Clone"],
            "the trait bound `CompletionProjection: Clone` is not satisfied", "require_clone(&projection);"),
        "projection-parent-escape": ("E0515", ["owned", "return"],
            "cannot return value referencing function parameter `owned`", "owned.projection()"),
        "projection-sibling-preview": ("E0599", ["ReceivedCompletion", "Preview"],
            "no associated item named `Preview` found for struct `ReceivedCompletion`", "super::datagram::ReceivedCompletion::Preview(summary)"),
    }
    LEGAL_CASES = ("projection-parent-legal", "projection-sibling-legal")
    ORDERED_CASES = (
        "projection-parent-legal", "projection-parent-preview", "projection-parent-inner-mut",
        "projection-parent-literal", "projection-parent-field", "projection-parent-clone",
        "projection-parent-escape", "projection-sibling-legal", "projection-sibling-preview",
    )
    specimen = EngineKeHistoryCompilerContractTests.specimen

    def cargo_result(self, case, *, line=42):
        """Keep the same exact Cargo library envelope but refuse test-build attribution."""
        result = EngineKeHistoryCompilerContractTests.cargo_result(self, case, line=line)
        return {**result, "output": result["output"].replace("(lib test)", "(lib)")}

    def test_nine_production_consumers_have_real_accessible_namespaces_and_legal_neighbors(self):
        original = b"// actual unchanged private parent\n"
        rows = [row for row in MODULE.engine_ke_history_contract_examples(original)
                if row["case"] in self.ORDERED_CASES]
        self.assertEqual([row["case"] for row in rows], list(self.ORDERED_CASES))
        self.assertEqual((sum(row["diagnostic"] is None for row in rows), len(rows)), (2, 9))
        for row in rows:
            probe = row["appended_source"]
            self.assertEqual(row["target"], "msvc")
            self.assertEqual(row["source"], original + probe.encode())
            self.assertIn("#[cfg(windows)]", probe)
            self.assertNotIn("cfg(all(test", probe)
            self.assertNotIn("crate::nts_ke_experiment", probe,
                "an inaccessible outer import cannot qualify an actual parent/sibling guarantee")
            sibling = row["case"].startswith("projection-sibling-")
            self.assertEqual("mod joined_projection_contract_probe_sibling" in probe, sibling)
            if row["case"] in self.LEGAL_CASES:
                for expected in ("slot.collect()", "slot.take_for_receiving()", "receive_joined(",
                                 "completion.projection()", "projection.summary()", "projection.numerical()",
                                 "projection.numerical_reason()"):
                    self.assertIn(expected, probe)
                self.assertIsNone(row["diagnostic"])
            else:
                code, tokens, _message, needle = self.CASES[row["case"]]
                self.assertEqual(row["diagnostic"]["code"], code)
                self.assertEqual(row["diagnostic"]["tokens"], tokens)
                actual_line = row["source"].decode().splitlines()[row["diagnostic"]["line"] - 1]
                self.assertIn(needle, actual_line)
                healthy_case = "projection-sibling-legal" if sibling else "projection-parent-legal"
                healthy = next(value for value in rows if value["case"] == healthy_case)
                self.assertLess(rows.index(healthy), rows.index(row))
                self.assertEqual(healthy["owner"], row["owner"])
        escape = next(row for row in rows if row["case"] == "projection-parent-escape")["appended_source"]
        self.assertIn("fn(&'a datagram::ReceivedCompletion, datagram::ReceivedCompletion)", escape,
            "the rejected return lifetime is tied to a real borrowed owner, avoiding an unrelated E0581")
        self.assertIn("|_borrowed, owned|", escape)

    def test_production_refusals_reach_exact_matcher_and_reject_cfg_test_summary(self):
        for case in self.CASES:
            result, specimen = self.cargo_result(case), self.specimen(case)
            with self.subTest(case=case), patch.object(MODULE.COMMON, "compiler_cause", wraps=MODULE.COMMON.compiler_cause) as matcher:
                MODULE.validate_engine_ke_projection_cause(result, specimen, self.MANIFEST)
            self.assertEqual(matcher.call_args.args[1:], (specimen, self.FILENAME))
            with self.subTest(case=case, wrong="test attribution"), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_projection_cause(
                    {**result, "output": result["output"].replace("(lib)", "(lib test)")}, specimen, self.MANIFEST)
            for wrong in ({**specimen, "code": "E0603"}, {**specimen, "tokens": ["private"]},
                          {**specimen, "line": 43}):
                with self.subTest(case=case, wrong=wrong), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_engine_ke_projection_cause(result, wrong, self.MANIFEST)

    def test_production_matrix_cannot_borrow_legacy_or_provider_diagnostics(self):
        old = EngineKeHistoryCompilerContractTests()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_engine_ke_projection_cause(old.cargo_result("copy-history"), old.specimen("copy-history"), self.MANIFEST)
        for case in ("projection-parent-preview", "projection-parent-clone", "projection-parent-escape"):
            with self.subTest(case=case), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_provider_cause(self.cargo_result(case), self.specimen(case), self.MANIFEST)


class EngineKeFixtureLaneTests(unittest.TestCase):
    """One public generator, never synthetic Engine qualification evidence."""

    NAMES = ("HEALTHY ZERO_RECEIVE ZERO_TRANSMIT ZERO_TIMESTAMPS ZERO_COOKIES EMPTY_COOKIE TWO_COOKIES EIGHT_COOKIES NINE_COOKIES "
        "MISSING_UID SHORT_UID DUPLICATE_UID CLEAR_COOKIE CLEAR_PLACEHOLDER INNER_UID INNER_AUTH "
        "INNER_PLACEHOLDER UNKNOWN_FIELDS_AND_SUFFIX SUFFIX_DUPLICATE_UID_AUTH MALFORMED_INNER "
        "MAX_INNER_FIELDS TOO_MANY_INNER MAX_OUTER_FIELDS TOO_MANY_OUTER MAX_PACKET_BYTES "
        "TOO_MANY_PACKET_BYTES WRONG_UID WRONG_ORIGIN WRONG_VERSION WRONG_MODE UNSYNCHRONIZED "
        "STRATUM_ZERO STRATUM_SIXTEEN NONCE_ZERO NONCE_ONE NONCE_TWELVE").split()

    def output(self, names=None, body="    0x00, 0xff,\n"):
        """Clearly synthetic public literals inside a complete captured libtest block."""
        source = ("// SPDX-License-Identifier: GPL-2.0-only\n"
            "//! Public component fixtures generated through the existing maintained AES-SIV API.\n"
            "//! No TLS/exporter/native/entropy or production authentication provenance is claimed.\n")
        for name in self.NAMES if names is None else names:
            source += (f"/// Public independently generated valid-tag fixture: {name}.\n"
                f"pub(in crate::nts_ke_experiment) const {name}: &[u8] = &[\n{body}];\n\n")
        return ("running 1 test\ntest generate_ep2_response_fixtures ... ok\n\nsuccesses:\n\n"
            "---- generate_ep2_response_fixtures stdout ----\n" + source +
            "\nsuccesses:\n    generate_ep2_response_fixtures\n\n"
            "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def test_exact_public_roster_and_complete_actual_case(self):
        source, facts = MODULE.validate_engine_ke_fixture_output(self.output())
        self.assertEqual(set(facts["fixtures"]), set(self.NAMES))
        self.assertTrue(facts["generated"])
        self.assertEqual(facts["tests_executed"], 1)
        self.assertEqual(facts["payload_bytes"], 72)
        self.assertTrue(source.startswith("// SPDX-License-Identifier:"))
        self.assertNotIn("successes:", source)
        self.assertNotIn("FIXTURES_GENERATED", source)
        self.assertEqual(facts["fixtures"]["HEALTHY"],
            {"bytes": 2, "sha256": hashlib.sha256(bytes([0, 255])).hexdigest()})

    def test_missing_extra_duplicate_malformed_and_incomplete_refuse(self):
        healthy = self.output()
        bad = [self.output(self.NAMES[:-1]), self.output([*self.NAMES, "FOREIGN"]),
            self.output([*self.NAMES, self.NAMES[0]]), self.output(body=""),
            self.output(body="    0xgg,\n"), self.output(body="    255,\n"),
            self.output(body="    0x00,,\n"), self.output(body="    include_bytes!(\"secret\"),\n"),
            healthy.replace("running 1 test\n", ""), healthy.replace(" ... ok", " ... FAILED"),
            healthy.replace("0 filtered out", "1 filtered out"),
            healthy.replace("fixture: HEALTHY.", "fixture: FOREIGN.")]
        for text in bad:
            with self.subTest(tail=text[-100:]), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_fixture_output(text)

    def test_finite_byte_and_source_bounds_keep_healthy_neighbor(self):
        MODULE.validate_engine_ke_fixture_output(self.output().replace(
            "    0x00, 0xff,\n", "    " + "0x00, " * 4100 + "\n", 1))
        for text in (self.output().replace("    0x00, 0xff,\n", "    " + "0x00, " * 4107 + "\n", 1),
                     self.output(body="    " + "0x00, " * 4100 + "\n")):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_engine_ke_fixture_output(text)

    def fixture_context(self):
        """Reuse admission fixtures, deliberately leaving the ordinary binding stale."""
        support = EngineKeAnalysisRoleTests()
        support.setUp()
        self.addCleanup(support.doCleanups)
        support.args.lane = "fixtures"
        support.policy["binding"] = {"deliberately_stale": True}
        generator = support.root / MODULE.ENGINE_KE_FIXTURE_SOURCE
        generator.parent.mkdir(parents=True, exist_ok=True)
        generator.write_text("Synthetic public generator, never compiled.\n")
        support.identity[MODULE.ENGINE_KE_FIXTURE_SOURCE] = MODULE.PRODUCT.sha256(generator)
        lock = ('version = 4\n[[package]]\nname="nevrotick-s115-aes-siv-fixture"\nversion="0.0.0"\n'
            '[[package]]\nname="ntp-wire"\nversion="0.0.0"\n[[package]]\nname="aes-siv"\nversion="0.8.0"\n'
            'source="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="' + "a" * 64 + '"\n')
        (support.root / "Cargo.lock").write_text(lock)
        (support.root / MODULE.ENGINE_KE_FIXTURE_LOCK).write_text(lock)
        support.identity["Cargo.lock"] = MODULE.PRODUCT.sha256(support.root / "Cargo.lock")
        support.identity[MODULE.ENGINE_KE_FIXTURE_LOCK] = MODULE.PRODUCT.sha256(support.root / MODULE.ENGINE_KE_FIXTURE_LOCK)
        (support.root / MODULE.ENGINE_KE_FIXTURE_MANIFEST).write_text("Synthetic manifest marker; never compiled.\n")
        support.fixture_dependency = {"engine_ke_association": {"rows": {}},
            "packages": [{"name": "aes-siv", "version": "0.8.0", "checksum": "a" * 64}]}
        return support

    def test_fixed_commands_source_bookends_and_no_engine_credit(self):
        support = self.fixture_context()
        (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
        calls = []
        outer = self
        class Session:
            def __init__(self, root, args, report, scope, policy, inputs):
                outer.assertEqual(args.lane, "baseline")
                outer.assertIn(MODULE.ENGINE_KE_FIXTURE_SOURCE, inputs)
                self.product_identity = support.identity
                self.source = root
                self.output = support.args.output_dir
                self.output.mkdir(mode=0o700)
            def run(self, label, argv, timeout):
                calls.append((label, argv, timeout))
                return "nevrotick-s115-aes-siv-fixture v0.0.0\n" if label == "fixture-tree" else outer.output()
            def finish(self):
                calls.append(("finish",))
        report = {}
        with patch.object(MODULE, "ROOT", support.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=(support.fixture_dependency, support.identity)), \
                patch.object(MODULE.COMMON, "Session", Session), \
                patch.object(MODULE, "validate_engine_ke_policy", side_effect=AssertionError("stale ordinary binding used")):
            MODULE.execute_engine_ke_fixtures(support.args, report)
        self.assertEqual(calls, [("fixture-tree", ["cargo", "tree", "--package", "nevrotick-s115-aes-siv-fixture",
            "--manifest-path", "/work/source/tools/crypto-aes-siv-witness/Cargo.toml",
            "--target", "x86_64-unknown-linux-gnu", "--frozen", "--edges", "normal,build,features"], 120),
            ("fixture-generator", ["cargo", "test", "--package", "nevrotick-s115-aes-siv-fixture",
                "--manifest-path", "/work/source/tools/crypto-aes-siv-witness/Cargo.toml", "--test",
                "ep2_response_fixtures", "--frozen", "--jobs", "1", "--", "--exact",
                "generate_ep2_response_fixtures", "--show-output", "--test-threads=1"], 120), ("finish",)])
        self.assertTrue(report["fixture_generation"]["generated"])
        self.assertFalse(report["qualification_passed"])
        self.assertFalse(report["selected_host_metrics_passed"])
        artifact = support.args.output_dir / "ep2-response-fixtures.rs"
        self.assertEqual(MODULE.PRODUCT.sha256(artifact), report["fixture_generation"]["source_sha256"])

    def test_wrong_role_lane_subject_or_open_campaign_refuses_before_session(self):
        for change in (lambda s: setattr(s.args, "profile", "legacy"), lambda s: setattr(s.args, "lane", "baseline"),
                lambda s: setattr(s.args, "subject", "ke_core"), lambda s: setattr(s.args, "tool_admission_profile", "full"),
                lambda s: s.policy.update(campaigns_admitted=True), lambda s: s.policy.update(jobs=1),
                lambda s: s.policy.update(features=["foreign"]),
                lambda s: s.policy.update(mutation_inventories={name: {"count": 1, "sha256": "a" * 64}
                    for name in EngineKeAnalysisRoleTests.HOST})):
            support = self.fixture_context()
            change(support)
            (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
            with patch.object(MODULE, "ROOT", support.root), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute_engine_ke_fixtures(support.args, {})
                session.assert_not_called()

    def test_cli_cannot_borrow_fixture_lane_for_other_profile_or_subject(self):
        for extra in (["--profile", "legacy"], ["--profile", "ke_association", "--subject", "ke_core"]):
            with patch.object(MODULE.COMMON, "Session") as session, patch.object(MODULE.sys, "stderr", io.StringIO()):
                with self.assertRaises(SystemExit) as error:
                    MODULE.main(["--lane", "fixtures", "--output-dir", "/unused/synthetic", *extra])
                self.assertEqual(error.exception.code, 2)
                session.assert_not_called()

    def test_fixture_seven_gib_startup_boundary_precedes_product_and_lock_admission(self):
        support = self.fixture_context()
        (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
        for free, accepted in ((7516192767, False), (7516192768, True)):
            with self.subTest(free=free), patch.object(MODULE, "ROOT", support.root), \
                    patch.object(MODULE.PRODUCT, "admit_product", return_value=(support.fixture_dependency, support.identity)) as product, \
                    patch.object(MODULE, "admit_engine_ke_fixture_lock", wraps=MODULE.admit_engine_ke_fixture_lock) as lock, \
                    patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=free)), \
                    patch.object(MODULE.COMMON, "Session", side_effect=EngineKeAnalysisRoleTests.SessionReached) as session:
                if accepted:
                    with self.assertRaises(EngineKeAnalysisRoleTests.SessionReached):
                        MODULE.execute_engine_ke_fixtures(support.args, {})
                    product.assert_called_once()
                    lock.assert_called_once()
                    session.assert_called_once()
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, "seven GiB"):
                        MODULE.execute_engine_ke_fixtures(support.args, {})
                    product.assert_not_called()
                    lock.assert_not_called()
                    session.assert_not_called()

    def test_standalone_lock_requires_exact_selected_root_identities(self):
        support = self.fixture_context()
        facts = MODULE.admit_engine_ke_fixture_lock(support.root, support.fixture_dependency)
        self.assertEqual(facts["registry_packages"], ["aes-siv@0.8.0"])
        lock = support.root / MODULE.ENGINE_KE_FIXTURE_LOCK
        original = lock.read_text()
        for changed in (original.replace('version="0.8.0"', 'version="0.7.0"'),
                        original.replace("a" * 64, "b" * 64),
                        original.replace('name="ntp-wire"', 'name="foreign-owner"'),
                        original + '[[package]]\nname="ntp-wire"\nversion="0.0.0"\n'):
            lock.write_text(changed)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_engine_ke_fixture_lock(support.root, support.fixture_dependency)
        lock.write_text(original)
        dependency = copy.deepcopy(support.fixture_dependency)
        dependency["packages"] = []
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_engine_ke_fixture_lock(support.root, dependency)

    def test_invalid_capture_writes_no_artifact_and_finishes_custody(self):
        support = self.fixture_context()
        (support.root / MODULE.ENGINE_KE_POLICY).write_text(json.dumps(support.policy))
        finished = []
        class Session:
            def __init__(self, root, *_arguments):
                self.product_identity = support.identity
                self.source = root
                self.output = support.args.output_dir
                self.output.mkdir(mode=0o700)
            def run(self, *_arguments):
                return "Invalid synthetic output; never a product result.\n"
            def finish(self):
                finished.append(True)
        report = {}
        with patch.object(MODULE, "ROOT", support.root), \
                patch.object(MODULE.PRODUCT, "admit_product", return_value=(support.fixture_dependency, support.identity)), \
                patch.object(MODULE.COMMON, "Session", Session):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.execute_engine_ke_fixtures(support.args, report)
        self.assertEqual(finished, [True])
        self.assertFalse(report["fixture_generation"]["generated"])
        self.assertFalse((support.args.output_dir / "ep2-response-fixtures.rs").exists())


class EngineSourcePolicyAnalysisRoleTests(unittest.TestCase):
    """Declared synthetic reader/dispatch controls; no compiler or product executes."""

    OWNERS = {
        "source_policy": "crates/nevrotick-engine/src/source_policy.rs",
        "library": "crates/nevrotick-engine/src/lib.rs",
        "session": "crates/nevrotick-engine/src/session.rs",
        "series": "crates/nevrotick-engine/src/series.rs",
        "address_round": "crates/nevrotick-engine/src/address_round.rs",
    }
    SNAPSHOT_TESTS = [
        "source_policy::tests::policy_snapshot_canonical_aliases_and_unknown_reads_do_not_allocate",
        "source_policy::tests::policy_snapshot_capped_rate_and_wait_are_historical",
        "source_policy::tests::policy_snapshot_distinguishes_source_endpoint_and_selective_reenable",
        "source_policy::tests::policy_snapshot_does_not_publish_changes_or_wake_a_future_floor",
        "source_policy::tests::policy_snapshot_keeps_unavailable_records_as_data",
        "source_policy::tests::policy_snapshot_preserves_independent_rate_intervals",
        "source_policy::tests::policy_snapshot_preserves_poison_and_the_healthy_owner",
        "source_policy::tests::policy_snapshot_public_sessions_observe_real_restriction_and_unmanaged_absence",
        "source_policy::tests::policy_snapshot_status_is_bounded_payload_free_and_preserves_sink_errors",
    ]
    PROVENANCE_TESTS = [
        "source_policy::tests::managed_source_provenance_follows_actual_owner_across_sessions_and_endpoints",
        "series::tests::managed_source_provenance_receiving_rejects_absent_and_foreign_owners",
        "series::address_round::tests::managed_source_provenance_address_transfer_binds_the_original_candidate",
    ]
    CONFIGURATION_TESTS = [
        "source_policy::tests::configuration::source_configuration_address_set_bounds_and_canonical_membership",
        "source_policy::tests::configuration::source_configuration_install_and_expected_snapshot_replacement_are_atomic",
        "source_policy::tests::configuration::source_configuration_parallel_public_sessions_and_reordered_round_share_revision",
        "source_policy::tests::configuration::source_configuration_outside_peers_cannot_publish_endpoint_capacity",
        "source_policy::tests::configuration::source_configuration_first_install_and_equal_replace_retire_old_prepared_plans",
        "source_policy::tests::configuration::source_configuration_replacement_preserves_kod_records_floors_and_targets",
        "source_policy::tests::configuration::source_configuration_revision_exhaustion_is_sticky_and_source_local",
        "series::tests::source_configuration_final_reply_replacement_preserves_report",
        "series::tests::source_configuration_final_send_replacement_prevents_send",
        "series::tests::source_configuration_postcommit_replacement_preserves_history",
    ]
    CONFIGURATION_DOCS = [
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::configuration::SourceAddressSet (line 4001)",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::configuration::SourceAddressSet (line 4002) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::configuration::SourceAddressSet (line 4003) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::configuration::SourceConfigurationSnapshot (line 4004)",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::configuration::SourceConfigurationSnapshot (line 4005) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::configuration::SourceConfigurationSnapshot (line 4006) - compile fail",
    ]
    PROVENANCE_DOCS = [
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::SourceIdentity (line 1431) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::SourceIdentity (line 1437) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::SourceIdentity (line 1442) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::SourceIdentity (line 1447) - compile fail",
        "crates/nevrotick-engine/src/source_policy.rs - source_policy::SourceIdentity (line 1452)",
    ]

    @staticmethod
    def execution(names):
        return (f"running {len(names)} tests\n"
                + "".join(f"test {name} ... ok\n" for name in names)
                + f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")

    def write_manifest(self, critical=None, modules=None):
        critical = list(self.OWNERS.values()) if critical is None else critical
        modules = ([{"package": "nevrotick-engine", "files": [name]} for name in self.OWNERS.values()]
                   if modules is None else modules)
        text = "critical_files = " + json.dumps(critical) + "\nsecurity_modules = [\n"
        for row in modules:
            text += "{ package = " + json.dumps(row["package"]) + ", files = " + json.dumps(row["files"]) + " },\n"
        path = self.root / "policy/verification-manifest.toml"
        path.write_text(text + "]\n")
        if hasattr(self, "policy"):
            self.policy["binding"]["inputs_sha256"]["policy/verification-manifest.toml"] = hashlib.sha256(path.read_bytes()).hexdigest()

    def write_policy(self):
        (self.root / MODULE.SOURCE_POLICY_POLICY).write_text(json.dumps(self.policy))

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / "source"
        self.root.mkdir()
        for name in MODULE.SOURCE_POLICY_BOUND_INPUTS:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("synthetic source-policy fixture: " + name + "\n")
        self.write_manifest()
        # This five-path identity is declared independently of checkout growth.
        self.identity = {name: hashlib.sha256((self.root / name).read_bytes()).hexdigest()
                         for name in self.OWNERS.values()}
        self.ordinary = sorted([f"ordinary_fixture_{index:03}" for index in range(194)]
                               + [f"source_policy::tests::synthetic_case_{index:02}" for index in range(49)]
                               + ["source_policy::tests::public_policy_diagnostics_are_literal_bounded_and_redacted",
                                  "source_policy::tests::public_managed_setup_exhausts_64_stale_hints_without_a_send_or_refund"]
                               + self.SNAPSHOT_TESTS + self.PROVENANCE_TESTS + self.CONFIGURATION_TESTS)
        # Independently declared full optional-profile identity. The reader under
        # test supplies neither these names nor their expected digest.
        self.ep1 = sorted(self.ordinary
                          + [f"nts_ke_experiment::synthetic_ep1_case_{index:03}"
                             for index in range(234)])
        self.docs = sorted([f"crates/nevrotick-engine/src/lib.rs - synthetic_doc_{index:02} (line {index + 1})"
                            for index in range(71)]
                           + [f"crates/nevrotick-engine/src/source_policy.rs - synthetic_doc (line {index}) - compile fail"
                              for index in range(1, 6)]
                           + [f"crates/nevrotick-engine/src/source_policy.rs - synthetic_doc (line {index})"
                              for index in (6, 7)] + self.PROVENANCE_DOCS + self.CONFIGURATION_DOCS)
        self.policy = {
            "schema_version": 1, "package": "nevrotick-engine",
            "admission_status": "admitted-engine-source-policy-analysis",
            "subjects": self.OWNERS.copy(), "critical_files": list(self.OWNERS.values()),
            "features": [], "test_workspace": False, "tool_admission_profile": "stable-core",
            "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
            "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 2560,
            "baseline_wall_seconds": 300, "coverage_wall_seconds": 300,
            "enumeration_wall_seconds": 240, "mutation_wall_seconds": 1200,
            "mutation_build_seconds": 90, "mutation_test_seconds": 60,
            "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
            "mutation_jobs": 6, "mutation_jobserver_tasks": 0,
            "mutation_process_limit": 512, "mutation_message_format": "json",
            "minimum_free_bytes": 7 * 1024**3, "stop_free_bytes": 4 * 1024**3,
            "owned_output_maximum_allocated_bytes": 3 * 1024**3,
            "owned_output_maximum_entries": 32768, "mutation_equivalences": [],
            "campaigns_admitted": False, "mutation_inventories": {},
            "binding": {"candidate_sha256": independent_digest(self.identity), "source_file_count": 5,
                "inputs_sha256": {name: hashlib.sha256((self.root / name).read_bytes()).hexdigest()
                                  for name in MODULE.SOURCE_POLICY_BOUND_INPUTS},
                "test_inventory": self.ordinary.copy(), "doctest_inventory": self.docs.copy(),
                "development_evidence": {"path": str(self.base / "reference.json"), "sha256": "a" * 64}},
        }
        for context in (
                patch.object(MODULE, "ROOT", self.root),
                patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, self.identity)),
                patch.object(MODULE, "SOURCE_POLICY_ORDINARY_DIGEST", independent_digest(self.ordinary)),
                patch.object(MODULE, "SOURCE_POLICY_EP1_DIGEST", independent_digest(self.ep1), create=True),
                patch.object(MODULE, "SOURCE_POLICY_DOCTEST_DIGEST", independent_digest(self.docs)),
                patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=8 * 1024**3))):
            context.start()
            self.addCleanup(context.stop)
        self.write_policy()
        self.args = SimpleNamespace(profile="source_policy", lane="baseline", subject="all",
            tool_admission_profile="stable-core", vendor_dir=self.base / "vendor",
            output_dir=self.base / "output")

    @contextmanager
    def synthetic_source_policy_equivalence(self):
        """Declare a synthetic review without weakening either actual reader."""
        identifier = "KOD-SERIES-UNAVAILABLE-ADMITTED-ZERO"
        filename = self.OWNERS["series"]
        descriptor = {"name": "independent synthetic zero-result review", "package": "nevrotick-engine",
            "file": filename, "function": {"function_name": "independent_synthetic_zero", "return_type": "-> usize",
                "span": {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 2}}},
            "span": {"start": {"line": 1, "column": 1}, "end": {"line": 1, "column": 2}},
            "replacement": "0", "genre": "FnValue"}
        path = self.root / "tasks/engine-source-policy-equivalences.md"
        original = copy.deepcopy(self.policy)
        original_proof = path.read_bytes() if path.exists() else None
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("Synthetic independent review only.\n## " + identifier + "\n")
        proof_sha = hashlib.sha256(path.read_bytes()).hexdigest()
        reviewer = "synthetic independent reader control"
        rationale = "Independent synthetic closed-domain zero result"
        entry = {"id": identifier, "candidate_sha256": independent_digest(self.identity),
            "source_sha256": self.identity[filename], "mutant": descriptor,
            "requirement": "CHECK-10/CHECK-19/CHECK-21/CHECK-25/CHECK-27", "rationale": rationale,
            "proof": {"path": "tasks/engine-source-policy-equivalences.md", "sha256": proof_sha,
                "section": identifier, "reviewer": reviewer, "verdict": "independently-justified-equivalent"}}
        dependency_names = ("Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml", "rust-toolchain.toml",
            ".cargo/config.toml", ".cargo/mutants.toml", ".config/nextest.toml", "policy/analysis-tools.json",
            "policy/tool-manifest.json", "policy/verification-manifest.toml")
        dependencies = {name: self.policy["binding"]["inputs_sha256"][name] for name in dependency_names}
        stable = self.base / "synthetic-stable"
        standard_sources = {}
        for name in ("core/src/time.rs", "std/src/sys/pal/unix/time.rs", "std/src/sys/time/unix.rs",
                     "std/src/sys/time/windows.rs", "std/src/time.rs"):
            source = stable / "lib/rustlib/src/rust/library" / name
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_text("Independent synthetic standard source: " + name)
            standard_sources[name] = hashlib.sha256(source.read_bytes()).hexdigest()
        tool_inventory = [{"name": "stable", "sha256": "d" * 64, "entries": 5, "root": "stable"}]
        self.policy["mutation_equivalences"] = [entry]
        self.policy["binding"]["inputs_sha256"]["tasks/engine-source-policy-equivalences.md"] = proof_sha
        self.write_policy()
        with patch.multiple(MODULE.COMMON,
                ENGINE_SOURCE_POLICY_EQUIVALENCE_CANDIDATE=independent_digest(self.identity),
                ENGINE_SOURCE_POLICY_EQUIVALENCE_PROOF_SHA256=proof_sha,
                ENGINE_SOURCE_POLICY_EQUIVALENCE_REVIEWERS=reviewer,
                ENGINE_SOURCE_POLICY_EQUIVALENCE_REVIEWS={identifier: (self.identity[filename],
                    independent_digest(descriptor), hashlib.sha256(b"synthetic rendered diff").hexdigest(), rationale)},
                ENGINE_SOURCE_POLICY_EQUIVALENCE_DEPENDENCIES=dependencies,
                ENGINE_SOURCE_POLICY_EQUIVALENCE_TOOL_INVENTORY_SHA256=independent_digest(tool_inventory),
                ENGINE_SOURCE_POLICY_EQUIVALENCE_STANDARD_SOURCES=standard_sources), \
                patch.object(MODULE.COMMON.ANALYSIS, "admit", return_value={"stable": stable,
                    "inventories": tool_inventory}):
            try:
                yield entry
            finally:
                if original_proof is None:
                    path.unlink()
                else:
                    path.write_bytes(original_proof)
                self.policy = original
                self.write_policy()

    def test_source_policy_reader_retains_exact_equivalence_and_actual_refusals(self):
        with self.synthetic_source_policy_equivalence() as entry:
            policy, _binding, _identity = MODULE.read_source_policy_policy(self.root)
            self.assertEqual(policy["mutation_equivalences"], [entry])
            self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(
                self.root, MODULE.SOURCE_POLICY_SCOPE, policy), [entry])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admit_mutation_equivalences(self.root, MODULE.SCOPE, policy)
            for key, value in (("id", "UNREVIEWED"), ("source_sha256", "f" * 64),
                               ("rationale", "policy-approved itself")):
                bad = copy.deepcopy(entry)
                bad[key] = value
                self.policy["mutation_equivalences"] = [bad]
                self.write_policy()
                before = (self.root / MODULE.SOURCE_POLICY_POLICY).read_bytes()
                with self.subTest(field=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_source_policy_policy(self.root)
                self.assertEqual((self.root / MODULE.SOURCE_POLICY_POLICY).read_bytes(), before)

    def test_new_catalogue_does_not_change_historical_engine_fallback_scope(self):
        """The old exact receiving validator still accepts its literal pair."""
        entries = []
        for suffix, replacement in (("SUB", "-="), ("MUL", "*=")):
            entries.append({"id": "ENGINE-COMMIT-DURATION-FALLBACK-" + suffix,
                "source_sha256": "6f3c6eb2f491de7f27a89f8fba84556e818874a9c875643b08081206f5dd4d46",
                "mutant": {"name": f"crates/nevrotick-engine/src/lib.rs:1335:23: replace += with {replacement} in commit_reply",
                    "package": "nevrotick-engine", "file": "crates/nevrotick-engine/src/lib.rs",
                    "function": {"function_name": "commit_reply", "return_type": "",
                        "span": {"start": {"line": 1315, "column": 1}, "end": {"line": 1364, "column": 2}}},
                    "span": {"start": {"line": 1335, "column": 23}, "end": {"line": 1335, "column": 25}},
                    "replacement": replacement, "genre": "BinaryOperator"},
                "proof": {"path": "tasks/engine-analysis-equivalences.md"}})
        MODULE.validate_equivalence_scope(entries)
        self.assertFalse(MODULE.COMMON._engine_source_policy_equivalence_role(MODULE.SCOPE,
            {"mutation_equivalences": entries}))
        for scope in (MODULE.SERIES_SCOPE, MODULE.SESSION_SCOPE):
            self.assertFalse(MODULE.COMMON._engine_source_policy_equivalence_role(scope,
                {"mutation_equivalences": entries}))
        with self.synthetic_source_policy_equivalence() as entry, self.assertRaises(MODULE.Incomplete):
            MODULE.validate_equivalence_scope([entry])

    def reference_fixture(self, *, ep1_outputs=None, independent_monitor=False):
        """Create physical hash-bound synthetic observations, without claiming executions."""
        directory = self.base / "development/revision-8" if independent_monitor else self.base
        directory.mkdir(parents=True, exist_ok=True)
        lanes = []
        for name in ("boundary-control", "format", "gnu-check", "gnu-clippy", "msvc-clippy", "msvc-check", "gnu-rustdoc",
                     "default-ordinary", "default-optimized", "default-doctests",
                     "ep1-ordinary", "ep1-optimized", "ep1-doctests"):
            output = (self.execution(self.docs) if name.endswith("doctests")
                      else self.execution(self.ep1) if name.startswith("ep1-")
                      else self.execution(self.ordinary) if "ordinary" in name or "optimized" in name
                      else "synthetic fixture only\n")
            if ep1_outputs is not None and name in ep1_outputs:
                output = ep1_outputs[name]
            path = directory / (name + ".json")
            path.write_text(json.dumps({"exit_code": 0, "failure": None, "output": output}))
            lanes.append({"lane": name, "path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                          "exit_code": 0, "failure": None})
        path = directory / "reference.json"
        reference = {"status": "workload-passed", "custody_unchanged": True,
                     "source_sha256": self.identity, "lanes": lanes}
        if independent_monitor:
            reference.update(self.independent_monitor_fixture(directory))
        else:
            reference.update(resource_observer_joined=True, terminal_resource_failures=[])
            # Actual accepted legacy receipts also contain this diagnostic, but
            # none of the three current-format provenance markers.
            reference["independent_monitor"] = {"ok": True, "errors": [], "joined": True,
                "exit_code": 0, "result": {"status": "stopped", "role": "attempt-2"},
                "result_sha256": "b" * 64, "resource_samples_sha256": "c" * 64}
        path.write_text(json.dumps(reference))
        self.policy["binding"]["development_evidence"]["path"] = str(path)
        self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
        self.write_policy()

    def independent_monitor_fixture(self, directory):
        """Declare durable monitor bytes and expectations independently of the reader."""
        source = directory.parent / "monitor.py"
        source.write_bytes(b"# independently declared synthetic sampler source\n")
        source_sha = hashlib.sha256(source.read_bytes()).hexdigest()
        result = {"status": "stopped", "parent_pid": 11, "parent_start": 12, "observer_pid": 13,
                  "role": "attempt-8", "recipe_sha256": source_sha, "samples": 2,
                  "maximum_gap_seconds": 1.0, "finished_monotonic": 101.5,
                  "recipe_final_sha256": source_sha}
        samples = [{"monotonic_seconds": 100.0, "gap_seconds": 0.25,
                    "available_bytes": 2 * 1024**3, "free_bytes": 8 * 1024**3,
                    "memory_bytes": 1024**3, "tasks": 2},
                   {"monotonic_seconds": 101.0, "gap_seconds": 1.0,
                    "available_bytes": 2 * 1024**3, "free_bytes": 8 * 1024**3,
                    "memory_bytes": 1024**3, "tasks": 2}]
        observer = directory / "observer"
        observer.mkdir(exist_ok=True)
        terminal = observer / "monitor-result.json"
        terminal.write_text(json.dumps(result))
        sampled = observer / "resources.jsonl"
        sampled.write_text("".join(json.dumps(row) + "\n" for row in samples))
        controls = {}
        for role in ("healthy", "gap"):
            copied_directory = directory / "monitor-controls" / role
            copied_directory.mkdir(parents=True, exist_ok=True)
            # Declared prior-control artifacts, not executed controls. The gap
            # result intentionally reports failure; it is never runtime GREEN.
            gap = role == "gap"
            observed = dict(result, role=role)
            observed_samples = copy.deepcopy(samples)
            if gap:
                observed.update(status="failed", maximum_gap_seconds=4.0, finished_monotonic=104.5,
                                error="Resource observer freshness exceeded two seconds.",
                                parent_notification="pidfd-SIGTERM")
                observed_samples[-1].update(monotonic_seconds=104.0, gap_seconds=4.0)
            terminal_copy = copied_directory / "monitor-result.json"
            terminal_copy.write_text(json.dumps(observed))
            samples_copy = copied_directory / "resources.jsonl"
            samples_copy.write_text("".join(json.dumps(row) + "\n" for row in observed_samples))
            control = {"expected_control_observed": True, "notifications": [104.6] if gap else [],
                       "monitor": {"ok": not gap, "errors": ["Monitor failed or stopped unexpectedly."] if gap else [],
                           "result": observed, "exit_code": 2 if gap else 0, "joined": True,
                           "result_sha256": hashlib.sha256(terminal_copy.read_bytes()).hexdigest(),
                           "resource_samples_sha256": hashlib.sha256(samples_copy.read_bytes()).hexdigest()}}
            (copied_directory / "control.json").write_text(json.dumps(control))
            for leaf in ("control.json", "monitor-result.json", "resources.jsonl"):
                copied = copied_directory / leaf
                controls[f"/dev/shm/synthetic-source-policy-observer/{role}/{leaf}"] = hashlib.sha256(copied.read_bytes()).hexdigest()
        return {"source_revision": 8, "monitor_sha256": source_sha, "monitor_control_inputs": controls,
                "independent_monitor": {"ok": True, "errors": [], "result": result, "exit_code": 0,
                    "joined": True, "result_sha256": hashlib.sha256(terminal.read_bytes()).hexdigest(),
                    "resource_samples_sha256": hashlib.sha256(sampled.read_bytes()).hexdigest()}}

    def fake_session(self, *, finish_failure=False, coverage=None):
        support = self

        class FakeSession:
            def __init__(self, root, args, report, scope, policy, inputs):
                support.assertEqual(scope.subjects, tuple(support.OWNERS.items()))
                support.assertEqual(set(inputs), set(MODULE.SOURCE_POLICY_BOUND_INPUTS))
                self.source = root
                self.product_identity = support.identity.copy()
                self.output = args.output_dir
                self.output.mkdir()
                self.work = self.output / "work"
                self.work.mkdir()
                self.tools = self.output / "tools"
                self.tools.mkdir()
                support.commands = []
                support.finished = False

            def run(self, label, command, timeout, **options):
                support.commands.append((label, command, timeout, options))
                if label == "source-policy-baseline":
                    return support.execution(support.ordinary)
                if label == "source-policy-doctests":
                    return support.execution(support.docs)
                if label == "coverage" and coverage is not None:
                    (self.work / "coverage.json").write_text(json.dumps(coverage))
                    return support.execution(support.ordinary)
                raise AssertionError("Unadmitted synthetic command: " + label)

            def finish(self):
                support.finished = True
                if finish_failure:
                    raise MODULE.Incomplete("synthetic late current custody failure")

        return FakeSession

    @contextmanager
    def managed_reference_fixture(self):
        """Physical fixed606/786 fixture; all producer lineage is declared synthetic."""
        self.reference_fixture(independent_monitor=True)
        old_path = Path(self.policy["binding"]["development_evidence"]["path"])
        old = json.loads(old_path.read_text())
        base = self.base / "managed-development"
        directory = base / "full-4"
        extension = base / "extension-v1"
        directory.mkdir(parents=True)
        extension.mkdir()
        identity = {name: hashlib.sha256((self.root / name).read_bytes()).hexdigest()
                    for name in self.OWNERS.values()}
        graph = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml",
                 ".cargo/mutants.toml", ".config/nextest.toml", "clippy.toml",
                 "crates/nevrotick-engine/Cargo.toml")
        for name in graph:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            if not path.exists():
                path.write_text("Independent synthetic graph input: " + name + "\n")
            identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        for index in range(606 - len(identity)):
            name = f"synthetic/product_{index:03}.rs"
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(f"// Fixed independent product specimen {index}.\n")
            identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        captured = dict(identity)
        for name in MODULE.SOURCE_POLICY_BOUND_INPUTS:
            captured[name] = hashlib.sha256((self.root / name).read_bytes()).hexdigest()
        for index in range(786 - len(captured)):
            name = f"synthetic/helper_{index:03}.py"
            path = self.root / name
            path.write_text(f"# Fixed independent capture specimen {index}.\n")
            captured[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        self.assertEqual((len(identity), len(captured)), (606, 786))
        for name in captured:
            destination = directory / "source" / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes((self.root / name).read_bytes())
        lanes = copy.deepcopy(old["lanes"])
        for lane in lanes:
            original = Path(lane["path"])
            destination = directory / original.name
            destination.write_bytes(original.read_bytes())
            lane.update(path=str(destination), sha256=hashlib.sha256(destination.read_bytes()).hexdigest())
        original_recipe = base / "development.py"
        original_monitor = base / "monitor.py"
        original_recipe.write_text("# Independent inert original development recipe.\n")
        original_monitor.write_text("# Independent inert original monitor recipe.\n")
        (extension / "monitor.py").write_bytes((old_path.parent.parent / "monitor.py").read_bytes())
        (extension / "development.py").write_text("# Independent inert extension recipe.\n")
        digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
        monitor_sha = digest(extension / "monitor.py")
        recipe_sha = digest(extension / "development.py")
        source_budget = {"admitted": True, "scope": "Synthetic source authority only.",
            "baseline": "e" * 40, "preparation_start_unix": 1000.0,
            "conservative_origin_includes_entire_gap_and_all_current_planning": True,
            "budget_seconds": 3600, "work_seconds": 3300, "cleanup_seconds": 300,
            "max_focused_attempts": 3, "max_full_development_attempts": 3,
            "no_mutations_admitted": True, "public_campaigns_admitted": False,
            "all_prior_source_qualification_clocks_and_charges_retained": True,
            "dependency_or_toolchain_changes_allowed": False}
        source_budget_path = base / "source-development-budget.json"
        source_budget_path.write_text(json.dumps(source_budget))
        source_budget_sha = digest(source_budget_path)
        original_budget = {"admitted": True, "budget_seconds": 3600, "reserve_seconds": 300,
            "maximum_attempts_per_phase": 3, "mutations_admitted": False,
            "recipe_sha256": digest(original_recipe), "source_budget_sha256": source_budget_sha,
            "new_test_names": self.CONFIGURATION_TESTS.copy(), "baseline": "e" * 40,
            "index_sha256": "f" * 64, "start_unix": 1000.0, "preparation_started_unix": 1000.0,
            "source_files": {self.OWNERS["library"]: identity[self.OWNERS["library"]]},
            "maximum_new_Rust_result_instances": 5172}
        original_budget_path = base / "development-budget.json"
        original_budget_path.write_text(json.dumps(original_budget))
        replan = {"completion": "completed", "verdict": "READY", "findings": [],
            "reviewed_extension": {"original_start_unix": 1000.0, "original_total_seconds": 3600,
                "original_work_seconds": 3300, "extension_seconds": 900,
                "cumulative_total_seconds": 4500, "cumulative_work_seconds": 4200,
                "cleanup_seconds": 300, "maximum_focused_attempts": 3, "maximum_full_attempts": 4,
                "additional_attempt": "full 4 only", "original_result_upper_charge": 5172,
                "additional_result_upper_charge": 1714, "cumulative_result_upper_charge": 6886,
                "candidate_sha256": independent_digest(identity), "product_inputs": 606,
                "complete_lanes": 13, "default_cases_per_mode": 267, "ep1_cases_per_mode": 501,
                "doctests_per_graph": 89}}
        replan_path = extension / "resource-replan-review.json"
        replan_path.write_text(json.dumps(replan))
        budget = {**original_budget, "budget_seconds": 4500, "work_seconds": 4200,
            "maximum_attempts_per_phase": 4, "maximum_focused_attempts": 3,
            "maximum_full_attempts": 4, "additional_full_attempts": 1, "extension_seconds": 900,
            "maximum_new_Rust_result_instances": 6886, "recipe_sha256": recipe_sha,
            "prior_development_budget_sha256": digest(original_budget_path),
            "prior_recipe_sha256": digest(original_recipe), "prior_monitor_sha256": digest(original_monitor),
            "coordinator_review_sha256": digest(replan_path),
            "original_budget_disposition": "incomplete; no original failure or clock reset",
            "candidate_sha256": independent_digest(identity)}
        (extension / "development-budget.json").write_text(json.dumps(budget))
        budget_sha = digest(extension / "development-budget.json")
        control_base = base / "qualification-admission"
        control_base.mkdir()
        (control_base / "monitor.py").write_bytes((extension / "monitor.py").read_bytes())
        control_budget = {"admitted": True, "start_unix": 4100.0,
            "preparation_started_unix": 4100.0, "budget_seconds": 4500, "reserve_seconds": 300,
            "monitor_sha256": monitor_sha, "resource_profile": {
                "memory_max": 6 * 1024**3, "swap_max": 512 * 1024**2, "tasks_max": 512,
                "startup_free_disk": 7 * 1024**3, "startup_available_ram": 15 * 1024**3 // 2,
                "running_free_disk": 4 * 1024**3, "running_available_ram": 1024**3,
                "observation_max_gap_seconds": 2}}
        (control_base / "development-budget.json").write_text(json.dumps(control_budget))
        control_pins = {"sha256": digest(control_base / "development-budget.json"), "start_unix": 4100.0}
        observer = directory / "resource-monitor"
        observer.mkdir()
        monitor = copy.deepcopy(old["independent_monitor"])
        monitor["result"]["role"] = "development"
        (observer / "monitor-result.json").write_text(json.dumps(monitor["result"]))
        samples = [json.loads(row) for row in (old_path.parent / "observer/resources.jsonl").read_text().splitlines()]
        for row, elapsed in zip(samples, (3000.0, 3001.0), strict=True):
            row["elapsed_seconds"] = elapsed
        (observer / "resources.jsonl").write_text("".join(json.dumps(row) + "\n" for row in samples))
        monitor["result_sha256"] = digest(observer / "monitor-result.json")
        monitor["resource_samples_sha256"] = digest(observer / "resources.jsonl")
        controls = {}
        for role in ("healthy", "gap"):
            for leaf in ("control.json", "monitor-result.json", "resources.jsonl"):
                destination = control_base / "monitor-controls" / role / leaf
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes((old_path.parent / "monitor-controls" / role / leaf).read_bytes())
                controls[role + "/" + leaf] = digest(destination)
        pins = {"candidate_sha256": independent_digest(identity),
                "captured_inputs_sha256": independent_digest(captured),
                "recipe_sha256": recipe_sha, "budget_sha256": budget_sha,
                "monitor_sha256": monitor_sha, "source_budget_sha256": source_budget_sha}
        reference = {"status": "workload-passed", "phase": "full", "attempt": 4, "lanes": lanes,
            "scope": "Synthetic receiving control only; no Rust/monitor execution claimed.",
            "generated_mutation_credit": False, "budget_sha256": budget_sha, "recipe_sha256": recipe_sha,
            "source_budget_sha256": source_budget_sha,
            "HEAD": budget["baseline"], "index_sha256": budget["index_sha256"],
            "source_sha256": identity, "captured_inputs": captured,
            "candidate_sha256": pins["candidate_sha256"],
            "tools": [{"name": "stable", "sha256": "d" * 64, "entries": 1, "root": "synthetic"}],
            "compiler": str(self.base / "synthetic-toolchain"), "vendor_identity": {"synthetic": True},
            "cargo_home_config_sha256": "b" * 64, "custody_unchanged": True,
            "durable_monitor": str(observer), "terminal_resource_failures": [],
            "independent_monitor": monitor, "elapsed_seconds": 1.5}
        path = directory / "verification.json"
        path.write_text(json.dumps(reference))
        closeout = {"status": "source-development-passed-qualification-not-admitted",
            "baseline": budget["baseline"], "product": pins["candidate_sha256"],
            "product_inputs": 606, "captured_inputs": 786, "source_origin_unix": 1000.0,
            "closed_unix": 4100.0, "cumulative_elapsed_seconds": 3100.0,
            "original3600_disposition": "incomplete; original first failures retained",
            "extension4500_disposition": "all13 development lanes passed within4200work; observer joined",
            "verification_sha256": digest(path), "implementation_review_sha256": "a" * 64,
            "reviewed_source_unchanged": True, "executed_named_results": 1714,
            "prior_focused_results": 10, "actual_result_total": 1724,
            "cumulative_result_upper_charge": 6886, "metric_admission": False,
            "public_campaigns": False, "native_qualification": False}
        closeout_path = base / "source-development-closeout-coherent.json"
        closeout_path.write_text(json.dumps(closeout))
        lineage_pins = {"original_development_budget_sha256": digest(original_budget_path),
            "original_recipe_sha256": digest(original_recipe), "original_monitor_sha256": digest(original_monitor),
            "replan_review_sha256": digest(replan_path), "closeout_sha256": digest(closeout_path)}
        binding = copy.deepcopy(self.policy["binding"])
        binding.update(candidate_sha256=pins["candidate_sha256"], source_file_count=606)
        fixture = SimpleNamespace(base=base, extension=extension, path=path, reference=reference,
            binding=binding, identity=identity, pins=pins, controls=controls, control_pins=control_pins,
            lineage_pins=lineage_pins, closeout=closeout, closeout_path=closeout_path)
        self.write_managed_reference(fixture, reference)
        # Real readers/refusal mechanisms remain untouched; only independently
        # declared fixture identities replace machine-local historical pins.
        with patch.object(MODULE, "SOURCE_POLICY_MANAGED_DEVELOPMENT", pins), \
             patch.object(MODULE, "SOURCE_POLICY_CONFIGURATION_LINEAGE", lineage_pins), \
             patch.object(MODULE, "SOURCE_POLICY_MANAGED_MONITOR_CONTROLS", controls), \
             patch.object(MODULE, "SOURCE_POLICY_MANAGED_CONTROL_BUDGET", control_pins):
            yield fixture

    def write_managed_reference(self, fixture, reference):
        """Rebind only the physical reference after a deliberate semantic challenge."""
        fixture.path.write_text(json.dumps(reference))
        reference_sha = hashlib.sha256(fixture.path.read_bytes()).hexdigest()
        fixture.binding["development_evidence"] = {"path": str(fixture.path), "sha256": reference_sha}
        if hasattr(fixture, "closeout"):
            fixture.closeout["verification_sha256"] = reference_sha
            fixture.closeout_path.write_text(json.dumps(fixture.closeout))
            fixture.lineage_pins["closeout_sha256"] = hashlib.sha256(fixture.closeout_path.read_bytes()).hexdigest()

    def rebind_managed_lineage(self, fixture):
        """Reconcile authored fixture bytes so a real semantic lineage guard decides."""
        digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
        source_path = fixture.base / "source-development-budget.json"
        original_path = fixture.base / "development-budget.json"
        review_path = fixture.extension / "resource-replan-review.json"
        fixture.pins["source_budget_sha256"] = digest(source_path)
        original = json.loads(original_path.read_text())
        original["source_budget_sha256"] = fixture.pins["source_budget_sha256"]
        original_path.write_text(json.dumps(original))
        fixture.lineage_pins["original_development_budget_sha256"] = digest(original_path)
        fixture.lineage_pins["replan_review_sha256"] = digest(review_path)
        budget_path = fixture.extension / "development-budget.json"
        budget = json.loads(budget_path.read_text())
        budget.update(source_budget_sha256=fixture.pins["source_budget_sha256"],
            prior_development_budget_sha256=fixture.lineage_pins["original_development_budget_sha256"],
            coordinator_review_sha256=fixture.lineage_pins["replan_review_sha256"])
        budget_path.write_text(json.dumps(budget))
        fixture.pins["budget_sha256"] = digest(budget_path)
        reference = {**fixture.reference, "budget_sha256": fixture.pins["budget_sha256"],
                     "source_budget_sha256": fixture.pins["source_budget_sha256"]}
        self.write_managed_reference(fixture, reference)

    def test_managed_configuration_lineage_rejects_restarted_or_weakened_allowances(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            source_path = fixture.base / "source-development-budget.json"
            original_path = fixture.base / "development-budget.json"
            review_path = fixture.extension / "resource-replan-review.json"
            budget_path = fixture.extension / "development-budget.json"
            originals = {path: path.read_bytes() for path in
                         (source_path, original_path, review_path, budget_path)}
            pins, lineage = fixture.pins.copy(), fixture.lineage_pins.copy()
            cases = [
                (source_path, ("admitted",), False),
                (source_path, ("budget_seconds",), 3601),
                (source_path, ("work_seconds",), True),
                (source_path, ("cleanup_seconds",), 299),
                (source_path, ("max_focused_attempts",), True),
                (source_path, ("preparation_start_unix",), 1001.0),
                (source_path, ("no_mutations_admitted",), False),
                (source_path, ("public_campaigns_admitted",), True),
                (source_path, ("all_prior_source_qualification_clocks_and_charges_retained",), False),
                (source_path, ("dependency_or_toolchain_changes_allowed",), True),
                (original_path, ("admitted",), False),
                (original_path, ("budget_seconds",), 4500),
                (original_path, ("maximum_attempts_per_phase",), True),
                (original_path, ("start_unix",), 1001.0),
                (original_path, ("maximum_new_Rust_result_instances",), 5171),
                (original_path, ("recipe_sha256",), "a" * 64),
                (review_path, ("verdict",), "NOT_READY"),
                (review_path, ("completion",), "incomplete"),
                (review_path, ("reviewed_extension", "maximum_full_attempts"), 5),
                (review_path, ("reviewed_extension", "original_start_unix"), True),
                (review_path, ("reviewed_extension", "cumulative_result_upper_charge"), 6885),
            ]
            for path, keys, value in cases:
                with self.subTest(path=path.relative_to(fixture.base), field=keys):
                    try:
                        changed = json.loads(originals[path])
                        target = changed
                        for key in keys[:-1]:
                            target = target[key]
                        target[keys[-1]] = value
                        path.write_text(json.dumps(changed))
                        self.rebind_managed_lineage(fixture)
                        with self.assertRaisesRegex(MODULE.Incomplete, "retained extension lineage|original-clock review"):
                            MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
                    finally:
                        for stored_path, raw in originals.items():
                            stored_path.write_bytes(raw)
                        fixture.pins.clear()
                        fixture.pins.update(pins)
                        fixture.lineage_pins.clear()
                        fixture.lineage_pins.update(lineage)
                        self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_configuration_closeout_requires_real_join_and_original_work_clock(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            proposals = [
                {"status": "workload-passed"}, {"baseline": "a" * 40},
                {"product": "a" * 64}, {"product_inputs": True}, {"captured_inputs": 785},
                {"source_origin_unix": 1001.0}, {"closed_unix": True},
                {"cumulative_elapsed_seconds": 3100.001},
                {"closed_unix": 5200.0, "cumulative_elapsed_seconds": 4200.0},
                {"closed_unix": 4001.499, "cumulative_elapsed_seconds": 4001.499 - 1000.0},
                {"verification_sha256": "a" * 64}, {"implementation_review_sha256": "invalid"},
                {"original3600_disposition": "passed"},
                {"extension4500_disposition": "passed"},
                {"reviewed_source_unchanged": False}, {"executed_named_results": True},
                {"prior_focused_results": 0}, {"actual_result_total": 1714},
                {"cumulative_result_upper_charge": 6885}, {"metric_admission": True},
                {"public_campaigns": True}, {"native_qualification": True}, {"extra_poison": True},
            ]
            for replacement in proposals:
                with self.subTest(replacement=replacement):
                    fixture.closeout_path.write_text(json.dumps({**fixture.closeout, **replacement}))
                    fixture.lineage_pins["closeout_sha256"] = hashlib.sha256(fixture.closeout_path.read_bytes()).hexdigest()
                    with self.assertRaisesRegex(MODULE.Incomplete, "closeout"):
                        MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            # The terminal is at source-origin1000 + elapsed3001 + join-gap0.5.
            # Equality supplies custody; the just-before proposal above does not.
            fixture.closeout_path.write_text(json.dumps({**fixture.closeout,
                "closed_unix": 4001.5, "cumulative_elapsed_seconds": 3001.5}))
            fixture.lineage_pins["closeout_sha256"] = hashlib.sha256(fixture.closeout_path.read_bytes()).hexdigest()
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_receiver_guard_actual_candidate_cannot_downgrade_valid_historical_receipt(self):
        """Select the current contract from actual bytes even with old valid metadata."""
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            actual = copy.deepcopy(fixture.identity)
            name = next(name for name in actual if name.endswith(".rs"))
            source = self.root / name
            original = source.read_bytes()
            try:
                source.write_bytes(original + b"// Independent current dispatch specimen.\n")
                actual[name] = hashlib.sha256(source.read_bytes()).hexdigest()
                candidate = hashlib.sha256(json.dumps(actual, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
                current = {**MODULE.SOURCE_POLICY_RECEIVER_DEVELOPMENT, "candidate_sha256": candidate,
                           "reference_sha256": None}
                binding = {**fixture.binding, "candidate_sha256": candidate}
                with patch.object(MODULE, "SOURCE_POLICY_RECEIVER_DEVELOPMENT", current):
                    with self.assertRaisesRegex(MODULE.Incomplete, "Receiver-guard development pins remain unbound"):
                        MODULE.admit_source_policy_development(binding, actual)
                    # Current markers are absent from the independently valid
                    # historical receipt; that cannot select legacy acceptance.
            finally:
                source.write_bytes(original)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_receiver_guard_claimed_and_actual_candidate_mismatch_cannot_fall_back(self):
        """Either actual or claimed current identity selects the strict receiving gate."""
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            actual = copy.deepcopy(fixture.identity)
            name = next(name for name in actual if name.endswith(".rs"))
            source = self.root / name
            original = source.read_bytes()
            changed = original + b"// Independent alternate admitted-byte identity.\n"
            actual[name] = hashlib.sha256(changed).hexdigest()
            candidate = hashlib.sha256(json.dumps(actual, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
            current = {**MODULE.SOURCE_POLICY_RECEIVER_DEVELOPMENT, "candidate_sha256": candidate}
            try:
                with patch.object(MODULE, "SOURCE_POLICY_RECEIVER_DEVELOPMENT", current):
                    proposals = (({**fixture.binding, "candidate_sha256": candidate}, fixture.identity, original),
                                 (fixture.binding, actual, changed))
                    for binding, identity, content in proposals:
                        source.write_bytes(content)
                        self.assertEqual(hashlib.sha256(source.read_bytes()).hexdigest(), identity[name])
                        with self.subTest(claim=binding["candidate_sha256"], actual=identity == actual):
                            with self.assertRaisesRegex(MODULE.Incomplete, "receiver development candidate binding differs"):
                                MODULE.admit_source_policy_development(binding, identity)
            finally:
                source.write_bytes(original)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_receipt_admits_complete_physical_capture(self):
        with self.managed_reference_fixture() as fixture:
            admitted = MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            self.assertEqual(len(admitted), 819)
            self.assertEqual(admitted, {name: hashlib.sha256(Path(name).read_bytes()).hexdigest()
                                        for name in admitted})
            self.assertEqual(len(fixture.reference["source_sha256"]), 606)
            self.assertEqual(len(fixture.reference["captured_inputs"]), 786)
            self.assertEqual((len(self.ordinary), len(self.ep1), len(self.docs)), (267, 501, 89))
            self.assertEqual(sum(name.startswith("source_policy::tests::") for name in self.ordinary), 68)
            self.assertEqual(sum(name.startswith("crates/nevrotick-engine/src/source_policy.rs - ")
                                 for name in self.docs), 18)
            self.assertEqual(sum(name.startswith("crates/nevrotick-engine/src/source_policy.rs - ")
                                 and name.endswith(" - compile fail") for name in self.docs), 13)
            with patch.object(MODULE, "SOURCE_POLICY_MANAGED_MONITOR_CONTROLS", {}):
                with self.assertRaisesRegex(MODULE.Incomplete, "not been independently pinned"):
                    MODULE.admit_source_policy_development(fixture.binding, fixture.identity)

    def test_current_configuration_cannot_downgrade_to_legacy_reference(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            # Keep the actual current synthetic identity and all thirteen healthy
            # raw lanes. Only the reference format/custody declaration changes;
            # no managed discriminator or independent-monitor marker remains.
            legacy = {key: copy.deepcopy(fixture.reference[key]) for key in
                      ("status", "custody_unchanged", "source_sha256", "lanes")}
            legacy.update(resource_observer_joined=True, terminal_resource_failures=[])
            self.assertFalse({"phase", "attempt", "captured_inputs", "durable_monitor",
                              "source_revision", "monitor_sha256", "monitor_control_inputs"} & set(legacy))
            path = fixture.path.parent / "legacy-reference.json"
            path.write_text(json.dumps(legacy))
            binding = copy.deepcopy(fixture.binding)
            binding["development_evidence"] = {"path": str(path),
                "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
            with self.assertRaisesRegex(MODULE.Incomplete, "managed|producer"):
                MODULE.admit_source_policy_development(binding, fixture.identity)
            foreign_identity = {**fixture.identity, self.OWNERS["library"]: "b" * 64}
            foreign_binding = copy.deepcopy(binding)
            foreign_binding["candidate_sha256"] = independent_digest(foreign_identity)
            for label, selected_binding, actual in (
                    ("current-source-foreign-binding", foreign_binding, fixture.identity),
                    ("foreign-source-current-binding", binding, foreign_identity)):
                with self.subTest(candidate_pair=label):
                    with self.assertRaisesRegex(MODULE.Incomplete, "current development candidate binding differs"):
                        MODULE.admit_source_policy_development(selected_binding, actual)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_receipt_refuses_missing_or_changed_physical_inputs(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            paths = [fixture.base / name for name in ("monitor.py", "development.py", "development-budget.json")]
            paths += [fixture.extension / name for name in
                      ("monitor.py", "development.py", "development-budget.json", "resource-replan-review.json")]
            paths += [fixture.base / name for name in
                      ("source-development-budget.json", "source-development-closeout-coherent.json")]
            paths += [fixture.path.parent / "resource-monitor" / name
                      for name in ("monitor-result.json", "resources.jsonl")]
            paths += [fixture.base / "qualification-admission/monitor-controls/gap/control.json",
                      fixture.base / "qualification-admission/development-budget.json",
                      fixture.base / "qualification-admission/monitor.py",
                      fixture.path.parent / "source" / self.OWNERS["library"]]
            for path in paths:
                original = path.read_bytes()
                for missing in (False, True):
                    with self.subTest(path=path.relative_to(fixture.base), missing=missing):
                        try:
                            if missing:
                                path.unlink()
                            else:
                                path.write_bytes(original + b"\n# independently changed bytes\n")
                            with self.assertRaises((MODULE.Incomplete, OSError)):
                                MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
                        finally:
                            path.write_bytes(original)
                self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_receipt_refuses_semantic_custody_and_stale_gap(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            faults = ((("phase",), "focused"), (("attempt",), True),
                      (("custody_unchanged",), False), (("terminal_resource_failures",), ["resource failure"]),
                      (("generated_mutation_credit",), True), (("elapsed_seconds",), 4200),
                      (("attempt",), 1), (("elapsed_seconds",), True),
                      (("elapsed_seconds",), 0), (("elapsed_seconds",), 4500),
                      (("source_budget_sha256",), "a" * 64),
                      (("HEAD",), "a" * 40), (("durable_monitor",), str(fixture.base / "wrong")),
                      (("independent_monitor", "joined"), False),
                      (("independent_monitor", "exit_code"), 1),
                      (("independent_monitor", "result", "role"), "attempt-8"),
                      (("independent_monitor", "result", "maximum_gap_seconds"), 2.01),
                      (("independent_monitor", "result", "finished_monotonic"), 104.5))
            for keys, value in faults:
                with self.subTest(field=keys, value=value):
                    changed = copy.deepcopy(fixture.reference)
                    target = changed
                    for key in keys[:-1]:
                        target = target[key]
                    target[keys[-1]] = value
                    terminal = fixture.path.parent / "resource-monitor/monitor-result.json"
                    terminal.write_text(json.dumps(changed["independent_monitor"]["result"]))
                    changed["independent_monitor"]["result_sha256"] = hashlib.sha256(terminal.read_bytes()).hexdigest()
                    self.write_managed_reference(fixture, changed)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            terminal.write_text(json.dumps(fixture.reference["independent_monitor"]["result"]))
            self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            for removed in ("phase", "attempt", "captured_inputs", "durable_monitor", "source_budget_sha256"):
                with self.subTest(missing_discriminator=removed):
                    changed = copy.deepcopy(fixture.reference)
                    del changed[removed]
                    self.write_managed_reference(fixture, changed)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            budget_path = fixture.extension / "development-budget.json"
            original_budget = budget_path.read_bytes()
            budget = json.loads(original_budget)
            for key, value in (("admitted", False), ("reserve_seconds", 299),
                               ("budget_seconds", 4499), ("budget_seconds", 4501),
                               ("budget_seconds", True), ("maximum_attempts_per_phase", True),
                               ("work_seconds", True), ("work_seconds", 4199),
                               ("maximum_focused_attempts", 4), ("maximum_full_attempts", 5),
                               ("additional_full_attempts", True), ("additional_full_attempts", 2),
                               ("extension_seconds", 901), ("maximum_new_Rust_result_instances", 6885),
                               ("start_unix", True), ("preparation_started_unix", 1001.0),
                               ("source_files", {}),
                               ("source_files", {self.OWNERS["library"]: "a" * 64}),
                               ("mutations_admitted", True), ("new_test_names", self.CONFIGURATION_TESTS[:-1])):
                with self.subTest(budget_field=key):
                    changed_budget = {**budget, key: value}
                    budget_path.write_text(json.dumps(changed_budget))
                    fixture.pins["budget_sha256"] = hashlib.sha256(budget_path.read_bytes()).hexdigest()
                    changed = copy.deepcopy(fixture.reference)
                    changed["budget_sha256"] = fixture.pins["budget_sha256"]
                    self.write_managed_reference(fixture, changed)
                    with self.assertRaisesRegex(MODULE.Incomplete, "allowance or original custody"):
                        MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            budget_path.write_bytes(original_budget)
            fixture.pins["budget_sha256"] = hashlib.sha256(original_budget).hexdigest()
            self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_work_and_fresh_control_budgets_keep_distinct_clocks(self):
        with self.managed_reference_fixture() as fixture:
            changed = {**fixture.reference, "elapsed_seconds": 2099.999}
            self.write_managed_reference(fixture, changed)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            self.write_managed_reference(fixture, fixture.reference)
            path = fixture.base / "qualification-admission/development-budget.json"
            original = path.read_bytes()
            budget = json.loads(original)
            original_pins = fixture.control_pins.copy()
            proposals = [({"admitted": False}, None), ({"budget_seconds": 4499}, None),
                         ({"budget_seconds": 4501}, None), ({"budget_seconds": True}, None),
                         ({"reserve_seconds": 299}, None), ({"monitor_sha256": "a" * 64}, None),
                         ({"preparation_started_unix": 4101.0}, None),
                         ({"extra_allowance": 4500}, None),
                         ({"start_unix": 1000.0, "preparation_started_unix": 1000.0}, 1000.0),
                         ({"start_unix": 4099.999, "preparation_started_unix": 4099.999}, 4099.999)]
            for name, value in budget["resource_profile"].items():
                proposals.append(({"resource_profile": {**budget["resource_profile"], name: value + 1}}, None))
            proposals.append(({"resource_profile": {**budget["resource_profile"], "tasks_max": True}}, None))
            for replacement, clock_pin in proposals:
                with self.subTest(replacement=replacement):
                    path.write_text(json.dumps({**budget, **replacement}))
                    fixture.control_pins.update(sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                        start_unix=original_pins["start_unix"] if clock_pin is None else clock_pin)
                    with self.assertRaisesRegex(MODULE.Incomplete, "cannot replace or restart"):
                        MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            # Actual source closure is sufficient before allocated allowance
            # expiry5500; one tick before closure4100 was refused above.
            for clock in (4100.0, 4100.001):
                path.write_text(json.dumps({**budget, "start_unix": clock, "preparation_started_unix": clock}))
                fixture.control_pins.update(sha256=hashlib.sha256(path.read_bytes()).hexdigest(), start_unix=clock)
                self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            path.write_bytes(original)
            fixture.control_pins.update(original_pins)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_receipt_retains_source_and_ep1_receiving_guards(self):
        # The separate historical format still reaches its runtime-map guard;
        # the current managed source now has a stronger candidate-pair guard.
        self.reference_fixture()
        self.assertEqual(len(MODULE.admit_source_policy_development(self.policy["binding"], self.identity)), 14)
        historical_other = {**self.identity, self.OWNERS["library"]: "b" * 64}
        with self.assertRaisesRegex(MODULE.Incomplete, "runtime/graph/configuration differs"):
            MODULE.admit_source_policy_development(self.policy["binding"], historical_other)
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            altered = copy.deepcopy(fixture.reference)
            name = self.OWNERS["library"]
            altered["captured_inputs"][name] = "a" * 64
            fixture.pins["captured_inputs_sha256"] = independent_digest(altered["captured_inputs"])
            self.write_managed_reference(fixture, altered)
            with self.assertRaisesRegex(MODULE.Incomplete, "complete reviewed closure"):
                MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
            fixture.pins["captured_inputs_sha256"] = independent_digest(fixture.reference["captured_inputs"])
            self.write_managed_reference(fixture, fixture.reference)
            other_identity = {**fixture.identity, name: "b" * 64}
            with self.assertRaisesRegex(MODULE.Incomplete, "current development candidate binding differs"):
                MODULE.admit_source_policy_development(fixture.binding, other_identity)
            for lane, output, exit_code, reason in (
                ("ep1-ordinary", self.execution([]), 0, "native block declaration or summary"),
                ("ep1-optimized", self.execution(self.ordinary), 0, "Source-policy.*EP1"),
                ("ep1-ordinary", self.execution(self.ep1), 101, "raw lane is failed or changed"),
            ):
                with self.subTest(lane=lane, exit_code=exit_code):
                    changed = copy.deepcopy(fixture.reference)
                    row = next(row for row in changed["lanes"] if row["lane"] == lane)
                    raw_path = Path(row["path"])
                    original = raw_path.read_bytes()
                    try:
                        raw_path.write_text(json.dumps({"exit_code": exit_code, "failure": None, "output": output}))
                        row.update(sha256=hashlib.sha256(raw_path.read_bytes()).hexdigest(), exit_code=exit_code)
                        self.write_managed_reference(fixture, changed)
                        with self.assertRaisesRegex(MODULE.Incomplete, reason):
                            MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
                    finally:
                        raw_path.write_bytes(original)
            self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_requires_successful_rustdoc_lane(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            for fault in ("missing", "duplicate", "failed", "boolean-exit", "failure"):
                with self.subTest(fault=fault):
                    changed = copy.deepcopy(fixture.reference)
                    row = next(row for row in changed["lanes"] if row["lane"] == "gnu-rustdoc")
                    path = Path(row["path"])
                    original = path.read_bytes()
                    try:
                        if fault == "missing":
                            changed["lanes"].remove(row)
                        elif fault == "duplicate":
                            changed["lanes"].append(copy.deepcopy(row))
                        else:
                            raw = json.loads(original)
                            if fault in ("failed", "boolean-exit"):
                                raw["exit_code"] = row["exit_code"] = 1 if fault == "failed" else True
                            else:
                                raw["failure"] = row["failure"] = "synthetic rustdoc failure"
                            path.write_text(json.dumps(raw))
                            row["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                        self.write_managed_reference(fixture, changed)
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
                    finally:
                        path.write_bytes(original)
            self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_managed_development_rejects_each_missing_or_substituted_provenance_control(self):
        with self.managed_reference_fixture() as fixture:
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)
            for lane, expected, mandatory in (
                ("default-ordinary", self.ordinary, self.PROVENANCE_TESTS + self.CONFIGURATION_TESTS),
                ("default-optimized", self.ordinary, self.PROVENANCE_TESTS + self.CONFIGURATION_TESTS),
                ("ep1-ordinary", self.ep1, self.PROVENANCE_TESTS + self.CONFIGURATION_TESTS),
                ("ep1-optimized", self.ep1, self.PROVENANCE_TESTS + self.CONFIGURATION_TESTS),
                ("default-doctests", self.docs, self.PROVENANCE_DOCS + self.CONFIGURATION_DOCS),
                ("ep1-doctests", self.docs, self.PROVENANCE_DOCS + self.CONFIGURATION_DOCS),
            ):
                for name in mandatory:
                    for alteration in ("omit", "same-count"):
                        with self.subTest(lane=lane, name=name, alteration=alteration):
                            names = [item for item in expected if item != name]
                            self.assertEqual(len(names), len(expected) - 1)
                            if alteration == "same-count":
                                replacement = (name.replace("source_policy::", "synthetic_source_policy::", 1)
                                    if lane.endswith("doctests")
                                    else "synthetic_independent_same_count_provenance_substitute")
                                names.append(replacement)
                                self.assertEqual(len(names), len(expected))
                            changed = copy.deepcopy(fixture.reference)
                            row = next(row for row in changed["lanes"] if row["lane"] == lane)
                            path = Path(row["path"])
                            original = path.read_bytes()
                            try:
                                path.write_text(json.dumps({"exit_code": 0, "failure": None,
                                    "output": self.execution(sorted(names))}))
                                row["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                                self.write_managed_reference(fixture, changed)
                                reason = ("Source-policy development compiler examples differ"
                                    if lane.endswith("doctests") else "Source-policy.*(omitted|substituted)")
                                with self.assertRaisesRegex(MODULE.Incomplete, reason):
                                    MODULE.admit_source_policy_development(fixture.binding, fixture.identity)
                            finally:
                                path.write_bytes(original)
            self.write_managed_reference(fixture, fixture.reference)
            self.assertEqual(len(MODULE.admit_source_policy_development(fixture.binding, fixture.identity)), 819)

    def test_real_reader_admits_declared_fixture_without_live_checkout_identity(self):
        policy, binding, identity = MODULE.read_source_policy_policy(self.root)
        self.assertEqual(identity, self.identity)
        self.assertEqual(binding["candidate_sha256"], independent_digest(self.identity))
        self.assertEqual(policy["critical_files"], list(self.OWNERS.values()))
        (self.root / "unrelated-future-source.txt").write_text("not declared product evidence\n")
        self.assertEqual(MODULE.read_source_policy_policy(self.root)[2], self.identity)

    def test_real_reader_rejects_poison_and_incomplete_role_before_lower_admission(self):
        for field, value in (("features", ["ke-association-prototype"]), ("jobs", True),
                             ("schema_version", True), ("coverage_minimum_line_percent", 89),
                             ("mutation_minimum_viable_killed_percent", 94),
                             ("mutation_jobserver_tasks", 1), ("extra_poison", True)):
            with self.subTest(field=field):
                bad = json.loads(json.dumps(self.policy))
                bad[field] = value
                with patch.object(MODULE.PRODUCT, "admit_product") as lower:
                    (self.root / MODULE.SOURCE_POLICY_POLICY).write_text(json.dumps(bad))
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.read_source_policy_policy(self.root)
                    lower.assert_not_called()
        self.write_policy()
        self.assertEqual(MODULE.read_source_policy_policy(self.root)[2], self.identity)

    def test_count_types_and_altered_source_or_helper_identity_refuse(self):
        for count in (True, False, 0, 4, 6, "5"):
            with self.subTest(count=count):
                original = self.policy["binding"]["source_file_count"]
                self.policy["binding"]["source_file_count"] = count
                self.write_policy()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_source_policy_policy(self.root)
                self.policy["binding"]["source_file_count"] = original
        self.write_policy()
        changed = dict(self.identity, **{self.OWNERS["source_policy"]: "f" * 64})
        with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, changed)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_source_policy_policy(self.root)
        helper = self.root / "scripts/analysis-runtime.py"
        original = helper.read_bytes()
        helper.write_bytes(original + b"changed")
        with self.assertRaises(MODULE.Incomplete):
            MODULE.read_source_policy_policy(self.root)
        helper.write_bytes(original)
        self.assertEqual(MODULE.read_source_policy_policy(self.root)[2], self.identity)

    def test_incomplete_and_foreign_protected_closures_refuse(self):
        for name in ("scripts/analysis-runtime.py", "/absolute/foreign.rs", "../outside.rs"):
            with self.subTest(name=name):
                bad = json.loads(json.dumps(self.policy))
                if name.startswith("scripts/"):
                    del bad["binding"]["inputs_sha256"][name]
                else:
                    bad["binding"]["inputs_sha256"][name] = "e" * 64
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_source_policy_policy(bad)

    def test_duplicate_missing_or_foreign_security_registrations_refuse(self):
        healthy = [{"package": "nevrotick-engine", "files": [name]} for name in self.OWNERS.values()]
        path = self.OWNERS["source_policy"]
        cases = [([], healthy), (list(self.OWNERS.values()) + [path], healthy),
                 (list(self.OWNERS.values()), healthy[1:]),
                 (list(self.OWNERS.values()), healthy + [healthy[0]]),
                 (list(self.OWNERS.values()), [{"package": "nevrotick-engine", "files": [path, path]}, *healthy[1:]]),
                 (list(self.OWNERS.values()), [{"package": "time-core", "files": [path]}, *healthy[1:]])]
        for index, (critical, modules) in enumerate(cases):
            with self.subTest(case=index):
                self.write_manifest(critical, modules)
                self.write_policy()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_source_policy_policy(self.root)
        self.write_manifest()
        self.write_policy()
        self.assertEqual(MODULE.read_source_policy_policy(self.root)[2], self.identity)

    def test_roster_removal_same_count_substitution_and_missing_compiler_examples_refuse(self):
        for key in ("test_inventory", "doctest_inventory"):
            for alteration in ("remove", "substitute", "duplicate"):
                with self.subTest(key=key, alteration=alteration):
                    bad = json.loads(json.dumps(self.policy))
                    names = bad["binding"][key]
                    if alteration == "remove":
                        names.pop()
                    else:
                        names[-1] = names[0] if alteration == "duplicate" else "same_count_foreign_case"
                        names.sort()
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.validate_source_policy_policy(bad)
        MODULE.validate_source_policy_policy(self.policy)
        missing_diagnostics = json.loads(json.dumps(self.policy))
        names = missing_diagnostics["binding"]["test_inventory"]
        names.remove("source_policy::tests::public_policy_diagnostics_are_literal_bounded_and_redacted")
        names.append("source_policy::tests::synthetic_diagnostics_substitute")
        names.sort()
        # Admit this independently declared synthetic identity so only omission
        # of the mandatory case, rather than a prior hash/count check, refuses it.
        with patch.object(MODULE, "SOURCE_POLICY_ORDINARY_DIGEST", independent_digest(names)):
            with self.assertRaisesRegex(MODULE.Incomplete, "public diagnostics"):
                MODULE.validate_source_policy_policy(missing_diagnostics)

    def test_same_count_budget_substitution_reaches_mandatory_case_refusal(self):
        MODULE.validate_source_policy_policy(self.policy)
        missing_budget = json.loads(json.dumps(self.policy))
        names = missing_budget["binding"]["test_inventory"]
        names.remove("source_policy::tests::public_managed_setup_exhausts_64_stale_hints_without_a_send_or_refund")
        names.append("source_policy::tests::synthetic_budget_substitute")
        names.sort()
        self.assertEqual(len(names), 267)
        self.assertEqual(sum(name.startswith("source_policy::tests::") for name in names), 68)
        # Reconcile this independent synthetic digest so the real named-case
        # guard supplies refusal, rather than an earlier count or stale hash.
        with patch.object(MODULE, "SOURCE_POLICY_ORDINARY_DIGEST", independent_digest(names)):
            with self.assertRaisesRegex(MODULE.Incomplete, "setup budget"):
                MODULE.validate_source_policy_policy(missing_budget)
        MODULE.validate_source_policy_policy(self.policy)

    def test_each_same_count_provenance_substitution_reaches_the_mandatory_guard(self):
        MODULE.validate_source_policy_policy(self.policy)
        for name in self.PROVENANCE_TESTS:
            with self.subTest(name=name):
                bad = copy.deepcopy(self.policy)
                names = bad["binding"]["test_inventory"]
                names.remove(name)
                names.append(name.rsplit("::", 1)[0] + "::synthetic_same_count_source_substitute")
                names.sort()
                self.assertEqual(len(names), 267)
                self.assertEqual(sum(item.startswith("source_policy::tests::") for item in names), 68)
                # The independent identity and counts are consistent. The named
                # requirement, rather than stale bytes, must reject the change.
                with patch.object(MODULE, "SOURCE_POLICY_ORDINARY_DIGEST", independent_digest(names)):
                    with self.assertRaisesRegex(MODULE.Incomplete, "provenance"):
                        MODULE.validate_source_policy_policy(bad)
        MODULE.validate_source_policy_policy(self.policy)

    def test_each_same_count_configuration_substitution_reaches_the_mandatory_guard(self):
        MODULE.validate_source_policy_policy(self.policy)
        for name in self.CONFIGURATION_TESTS:
            with self.subTest(name=name):
                bad = copy.deepcopy(self.policy)
                names = bad["binding"]["test_inventory"]
                names.remove(name)
                names.append(name.rsplit("::", 1)[0] + "::synthetic_same_count_configuration_substitute")
                names.sort()
                self.assertEqual(len(names), 267)
                self.assertEqual(sum(item.startswith("source_policy::tests::") for item in names), 68)
                # Counts and independently calculated identity are healthy; the
                # actual named configuration requirement must supply refusal.
                with patch.object(MODULE, "SOURCE_POLICY_ORDINARY_DIGEST", independent_digest(names)):
                    with self.assertRaisesRegex(MODULE.Incomplete, "configuration"):
                        MODULE.validate_source_policy_policy(bad)
        MODULE.validate_source_policy_policy(self.policy)

    def test_closed_campaign_refuses_before_session_or_output(self):
        self.args.lane = "mutation"
        with patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.execute_source_policy_profile(self.args, {})
            session.assert_not_called()
        self.assertFalse(self.args.output_dir.exists())

    def test_full_blocks_are_required_and_doctests_remain_a_separate_union(self):
        binding = self.policy["binding"]
        self.assertEqual(MODULE.source_policy_execution(self.execution(self.ordinary), binding)["tests_executed"], 267)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.source_policy_execution(self.execution(self.docs), binding, doctests=True)
        both = self.execution(self.ordinary) + self.execution(self.docs)
        self.assertEqual(MODULE.source_policy_execution(both, binding, doctests=True)["tests_executed"], 356)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.source_policy_execution(self.execution(self.ordinary).replace("0 filtered out", "1 filtered out"), binding)

    def test_actual_dispatch_runs_default_baseline_and_separate_docs_then_finishes(self):
        self.reference_fixture()
        report = {}
        with patch.object(MODULE.COMMON, "Session", self.fake_session()):
            MODULE.execute(self.args, report)
        self.assertTrue(self.finished)
        self.assertEqual([row[0] for row in self.commands], ["source-policy-baseline", "source-policy-doctests"])
        self.assertTrue(all("--features" not in row[1] and 0 < row[2] <= 300 for row in self.commands))
        self.assertEqual(report["baseline_execution"]["tests_executed"], 267)
        self.assertEqual(report["doctest_execution"]["tests_executed"], 89)
        self.assertTrue(report["selected_lane_passed"])
        self.assertFalse(report["qualification_passed"])

    def test_failed_final_custody_never_publishes_a_baseline_pass(self):
        self.reference_fixture()
        report = {}
        with patch.object(MODULE.COMMON, "Session", self.fake_session(finish_failure=True)):
            with self.assertRaisesRegex(MODULE.Incomplete, "late current custody"):
                MODULE.execute_source_policy_profile(self.args, report)
        self.assertTrue(self.finished)
        self.assertEqual(len(self.commands), 2)
        self.assertFalse(report["selected_lane_passed"])
        self.assertFalse(report["qualification_passed"])

    def coverage_fixture(self):
        """Independently declared LLVM rows: each selected owner has exactly90/100 lines."""
        return {"data": [{"files": [{"filename": "/work/source/" + name,
            "summary": {"lines": {"count": 100, "covered": 90}}}
            for name in self.OWNERS.values()]}]}

    def test_actual_coverage_dispatch_attributes_exactly_five_owners(self):
        unselected = "crates/nevrotick-engine/src/unselected_fixture.rs"
        path = self.root / unselected
        path.write_text("synthetic unselected Rust input\n")
        self.identity[unselected] = hashlib.sha256(path.read_bytes()).hexdigest()
        self.policy["binding"]["source_file_count"] = len(self.identity)
        self.policy["binding"]["candidate_sha256"] = independent_digest(self.identity)
        self.reference_fixture()
        self.args.lane = "coverage"
        report = {}
        with patch.object(MODULE.COMMON, "Session", self.fake_session(coverage=self.coverage_fixture())):
            MODULE.execute_source_policy_profile(self.args, report)
        self.assertTrue(self.finished)
        self.assertEqual(set(report["coverage"]), set(self.OWNERS))
        self.assertTrue(all(row["covered_lines"] == 90 and row["executable_lines"] == 100
                            and row["line_percent"] == 90 for row in report["coverage"].values()))
        self.assertEqual(report["coverage_execution"]["tests_executed"], 267)
        label, command, timeout, options = self.commands[0]
        self.assertEqual(label, "coverage")
        self.assertNotIn("--features", command)
        self.assertEqual(command[command.index("--fail-under-file-lines") + 1], "90")
        expression = command[command.index("--ignore-filename-regex") + 1]
        self.assertTrue(re.search(expression, "/work/source/" + unselected))
        self.assertTrue(all(re.search(expression, "/work/source/" + name) is None
                            for name in self.OWNERS.values()))
        self.assertTrue(report["selected_lane_passed"])
        self.assertFalse(report["qualification_passed"])

    def test_coverage_omission_foreign_duplicate_and_low_owner_refuse_after_finish(self):
        self.reference_fixture()
        self.args.lane = "coverage"
        for change in ("omit", "foreign", "duplicate", "low", "empty"):
            with self.subTest(change=change):
                coverage = self.coverage_fixture()
                rows = coverage["data"][0]["files"]
                if change == "omit":
                    rows.pop()
                elif change == "foreign":
                    rows[-1]["filename"] = "/work/source/crates/foreign.rs"
                elif change == "duplicate":
                    rows.append(json.loads(json.dumps(rows[0])))
                else:
                    rows[-1]["summary"]["lines"] = ({"count": 100, "covered": 89}
                        if change == "low" else {"count": 0, "covered": 0})
                self.args.output_dir = self.base / ("coverage-" + change)
                report = {}
                with patch.object(MODULE.COMMON, "Session", self.fake_session(coverage=coverage)):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.execute_source_policy_profile(self.args, report)
                self.assertTrue(self.finished)
                self.assertFalse(report["selected_lane_passed"])
                self.assertFalse(report["qualification_passed"])

    def test_all_raw_gates_cannot_publish_independent_qualification(self):
        self.policy["campaigns_admitted"] = True
        self.policy["mutation_inventories"] = {name: {"count": 1, "sha256": "e" * 64}
                                               for name in self.OWNERS}
        self.reference_fixture()
        self.args.lane = "all"
        report = {}
        with patch.object(MODULE.COMMON, "Session", self.fake_session(coverage=self.coverage_fixture())), \
                patch.object(MODULE.COMMON, "run_analysis_lanes") as mutation:
            MODULE.execute_source_policy_profile(self.args, report)
        mutation.assert_called_once()
        self.assertEqual(mutation.call_args.args[-1], "mutation")
        self.assertTrue(self.finished)
        self.assertTrue(report["selected_lane_passed"])
        self.assertFalse(report["qualification_passed"])

    def test_changed_reference_or_lane_bytes_refuse_before_session(self):
        self.reference_fixture()
        for filename in ("reference.json", "default-ordinary.json"):
            path = self.base / filename
            original = path.read_bytes()
            path.write_bytes(original + b" ")
            with self.subTest(filename=filename), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute_source_policy_profile(self.args, {})
                session.assert_not_called()
            path.write_bytes(original)
        self.assertEqual(len(MODULE.admit_source_policy_development(self.policy["binding"], self.identity)), 14)

    def test_development_complete_ep1_modes_have_healthy_reader_and_custody(self):
        self.assertEqual(len(self.ep1), len(self.ordinary) + 234)
        self.assertTrue(set(self.ordinary) <= set(self.ep1))
        self.assertEqual(sum(name.startswith("nts_ke_experiment::") for name in self.ep1), 234)
        self.reference_fixture()
        self.assertEqual(len(MODULE.admit_source_policy_development(
            self.policy["binding"], self.identity)), 14)
        report = {}
        with patch.object(MODULE.COMMON, "Session", self.fake_session()):
            MODULE.execute_source_policy_profile(self.args, report)
        self.assertTrue(self.finished)
        self.assertTrue(report["selected_lane_passed"])
        self.assertFalse(report["qualification_passed"])

    def test_capture_development_requires_independent_ordinary_and_rustdoc_graphs(self):
        default = sorted([*self.ordinary, "source_policy::tests::capture_refused",
                          "source_policy::tests::capture_preserved"])
        ep1 = sorted([*default, "ke_preview_target::tests::typed_peer",
                      "source_policy::tests::configuration::withdrawing_ke_preview"])
        ep1_docs = sorted([*self.docs, "crates/nevrotick-engine/src/ke_preview_target.rs - "
                           "ke_preview_target::KePreviewTarget (line 33) - compile fail"])
        specs = {key: (len(names), independent_digest(names)) for key, names in (
            ("default-ordinary", default), ("ep1-ordinary", ep1),
            ("default-doctests", self.docs), ("ep1-doctests", ep1_docs))}
        binding = {**self.policy["binding"], "candidate_sha256": MODULE.SOURCE_POLICY_CAPTURE_CANDIDATE,
                   "test_inventory": default}
        with patch.object(MODULE, "SOURCE_POLICY_CAPTURE_ROSTER", specs):
            self.assertEqual(MODULE.source_policy_execution(self.execution(default), binding)["test_names"], default)
            self.assertEqual(MODULE.source_policy_ep1_execution(self.execution(ep1), binding)["test_names"], ep1)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_ep1_execution(
                    self.execution([name.replace("typed_peer", "foreign_peer") for name in ep1]), binding)
            for lane, names in (("default-doctests", self.docs), ("ep1-doctests", ep1_docs)):
                facts = MODULE.source_policy_development_doctests(self.execution(names), binding, lane)
                self.assertEqual(facts["test_names"], names)
            for lane, names in (("default-doctests", ep1_docs), ("ep1-doctests", self.docs),
                                ("ep1-doctests", [name.replace("(line 33)", "(line 34)")
                                                  if "ke_preview_target.rs" in name else name for name in ep1_docs])):
                with self.subTest(lane=lane, count=len(names)), self.assertRaises(MODULE.Incomplete):
                    MODULE.source_policy_development_doctests(self.execution(names), binding, lane)

    def test_capture_development_requires_effective_sandbox_and_selected_core_block(self):
        expected = {"label": "core-library-ordinary", "kind": "core",
                    "command": ["cargo", "test", "--package", "time-core", "--lib", "--", "--exact", "known"],
                    "expected": [["unittests src/lib.rs", "known"]]}
        prefix = ["/usr/bin/bwrap", "--unshare-all"]
        output = ("     Running unittests src/lib.rs (/work/target/debug/core-tests)\n"
                  + self.execution(["known"]).replace("0 filtered out", "249 filtered out"))
        raw = {"command": expected["command"], "sandbox_command": [*prefix, "--", *expected["command"]],
               "output": output, "exit_code": 0, "failure": None}
        row = {"exit_code": 0, "failure": None, "cases": ["known"], "tests_executed": 1}
        binding = self.policy["binding"]
        MODULE.validate_source_policy_capture_lane(row, expected, raw, binding, prefix)
        for candidate, message in (
                ({**raw, "sandbox_command": []}, "effective sandbox"),
                ({**raw, "sandbox_command": ["--", *expected["command"]]}, "effective sandbox"),
                ({**raw, "sandbox_command": [*prefix, "--", "cargo", "test", "--features",
                                              "ke-association-prototype"]}, "effective sandbox"),
                ({**raw, "output": ""}, "missing, truncated or extra native blocks"),
                ({**raw, "output": output.replace("test known", "test foreign")}, "Core identities"),
                ({**raw, "output": output.replace("unittests src/lib.rs", "tests/phase_plan.rs")}, "Core identities"),
                ({**raw, "output": output.replace("running 1 tests", "running 2 tests")}, "Core identities")):
            with self.subTest(message=message), self.assertRaisesRegex(MODULE.Incomplete, message):
                MODULE.validate_source_policy_capture_lane(row, expected, candidate, binding, prefix)
        with self.assertRaisesRegex(MODULE.Incomplete, "Core identities"):
            MODULE.validate_source_policy_capture_lane({**row, "cases": []}, expected, raw, binding, prefix)

        boundary = {"label": "boundary-control", "kind": "boundary",
                    "command": ["/usr/bin/python3", "-c", "<isolated boundary probe>"]}
        command = ["/usr/bin/python3", "-c", "captured test-namespace probe"]
        raw = {"command": command, "sandbox_command": [*prefix, "--", *command],
               "output": "ANALYSIS_BOUNDARY_OK\n", "exit_code": 0, "failure": None}
        digest = independent_digest(raw)
        row = {"sha256": digest, "exit_code": 0, "failure": None}
        with patch.object(MODULE, "SOURCE_POLICY_CAPTURE_BOUNDARY", digest):
            MODULE.validate_source_policy_capture_lane(row, boundary, raw, binding, prefix)
            altered = [*command[:2], 'print("ANALYSIS_BOUNDARY_OK")']
            raw = {**raw, "command": altered, "sandbox_command": [*prefix, "--", *altered]}
            with self.assertRaisesRegex(MODULE.Incomplete, "namespace boundary control"):
                MODULE.validate_source_policy_capture_lane(
                    {**row, "sha256": independent_digest(raw)}, boundary, raw, binding, prefix)

    def test_capture_development_refuses_stale_binding_and_no_legacy_receipt_fallback(self):
        self.reference_fixture()
        binding = self.policy["binding"]
        candidate = binding["candidate_sha256"]
        legacy = Path(binding["development_evidence"]["path"]).read_bytes()
        directory = self.base / "capture" / "reports" / "run-01"
        directory.mkdir(parents=True)
        raw_manifest = json.dumps({"candidate_sha256": candidate}).encode()
        (directory.parents[1] / "candidate.json").write_bytes(raw_manifest)
        path = directory / "verification.json"
        path.write_bytes(legacy)
        current = {**binding, "development_evidence": {
            "path": str(path), "sha256": hashlib.sha256(legacy).hexdigest()}}
        with patch.multiple(MODULE, SOURCE_POLICY_CAPTURE_CANDIDATE=candidate,
                            SOURCE_POLICY_CAPTURE_MANIFEST=hashlib.sha256(raw_manifest).hexdigest()), \
                patch.object(MODULE, "admit_source_policy_observer") as legacy_reader:
            with self.assertRaisesRegex(MODULE.Incomplete, "candidate binding differs"):
                MODULE.admit_source_policy_development(current, {})
            with self.assertRaisesRegex(MODULE.Incomplete, "passing terminal custody"):
                MODULE.admit_source_policy_development(current, self.identity)
        legacy_reader.assert_not_called()

    def test_receive_combined_roster_preserves_the_distinct_historical_graph(self):
        old = sorted([*self.ordinary, "source_policy::tests::historical_one",
                      "source_policy::tests::historical_two"])
        new = sorted([*old, "source_policy::tests::receive_cutoff", "source_policy::tests::receive_order"])
        ep1 = sorted([*new, "nts_ke_experiment::datagram::tests::receive_cutoff"])
        chronology = sorted([*new, "source_policy::tests::chronology"])
        chronology_ep1 = sorted([*ep1, "source_policy::tests::chronology",
                                 "nts_ke_experiment::datagram::tests::chronology_reversed",
                                 "nts_ke_experiment::datagram::tests::chronology_future"])
        debit_case = "nts_ke_experiment::datagram::tests::known_protected_datagram_with_invalid_capture_charges_frame_and_bytes"
        debit_ep1 = sorted([*chronology_ep1, debit_case])
        docs = sorted([*self.docs, "crates/nevrotick-engine/src/ke_preview_target.rs - "
                       "ke_preview_target::KePreviewTarget (line 33) - compile fail"])
        def specs(default, optional):
            return {key: (len(names), independent_digest(names)) for key, names in (
                ("default-ordinary", default), ("ep1-ordinary", optional),
                ("default-doctests", self.docs), ("ep1-doctests", docs))}

        historical = {**self.policy["binding"], "candidate_sha256": MODULE.SOURCE_POLICY_CAPTURE_CANDIDATE,
                      "test_inventory": old}
        current = {**historical, "candidate_sha256": MODULE.SOURCE_POLICY_RECEIVE_CANDIDATE,
                   "test_inventory": new}
        next_binding = {**current, "candidate_sha256": MODULE.SOURCE_POLICY_CHRONOLOGY_CANDIDATE,
                        "test_inventory": chronology}
        debit_binding = {**next_binding, "candidate_sha256": MODULE.SOURCE_POLICY_DEBIT_CANDIDATE}
        with patch.multiple(MODULE, SOURCE_POLICY_CAPTURE_ROSTER=specs(old, old),
                            SOURCE_POLICY_RECEIVE_ROSTER=specs(new, ep1),
                            SOURCE_POLICY_CHRONOLOGY_ROSTER=specs(chronology, chronology_ep1),
                            SOURCE_POLICY_DEBIT_ROSTER=specs(chronology, debit_ep1)):
            MODULE.source_policy_execution(self.execution(old), historical)
            MODULE.source_policy_execution(self.execution(new), current)
            MODULE.source_policy_ep1_execution(self.execution(ep1), current)
            MODULE.source_policy_development_doctests(self.execution(docs), current, "ep1-doctests")
            MODULE.source_policy_execution(self.execution(chronology), next_binding)
            MODULE.source_policy_ep1_execution(self.execution(chronology_ep1), next_binding)
            MODULE.source_policy_execution(self.execution(chronology), debit_binding)
            MODULE.source_policy_ep1_execution(self.execution(debit_ep1), debit_binding)
            for incompatible in (chronology_ep1, [name if name != debit_case else
                                                   "nts_ke_experiment::wrong_debit" for name in debit_ep1]):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.source_policy_ep1_execution(self.execution(incompatible), debit_binding)
            for binding, roster in ((current, old), (historical, new), (next_binding, new), (current, chronology)):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.source_policy_execution(self.execution(roster), binding)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_ep1_execution(self.execution(new), current)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_ep1_execution(
                    self.execution([name.replace("::receive_cutoff", "::foreign_cutoff") for name in ep1]), current)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_development_doctests(
                    self.execution([name.replace("(line 33)", "(line 34)") for name in docs]), current, "ep1-doctests")

    def test_header_age_combined_requires_both_rosters_and_current_rustdoc_annotation(self):
        old_default = sorted([*self.ordinary] + [
            f"source_policy::tests::additional_age_neighbor_{index}" for index in range(5)])
        case = "source_policy::tests::managed_header_writes_receive_to_commit_age_only_for_ordered_capture"
        default = sorted([*old_default, case])
        extra = "nts_ke_experiment::datagram::tests::age_role_ke_neighbor"
        ep1 = sorted([*default, extra])
        old_anchor = "crates/nevrotick-engine/src/lib.rs - ControlSchedule::new (line 1905)"
        new_anchor = old_anchor.replace("1905", "1922")
        old_docs = sorted([*self.docs, old_anchor])
        docs = sorted([*self.docs, new_anchor])
        ep1_docs = sorted([*docs, "crates/nevrotick-engine/src/ke_preview_target.rs - "
                           "ke_preview_target::KePreviewTarget (line 33) - compile fail"])
        binding = {**self.policy["binding"], "candidate_sha256": MODULE.SOURCE_POLICY_HEADER_AGE_CANDIDATE,
                   "test_inventory": default, "doctest_inventory": docs}
        names = {"default-ordinary": default, "ep1-ordinary": ep1,
                 "default-doctests": docs, "ep1-doctests": ep1_docs}
        specs = {lane: (len(roster), independent_digest(roster)) for lane, roster in names.items()}
        with patch.object(MODULE, "SOURCE_POLICY_HEADER_AGE_ROSTER", specs):
            MODULE.source_policy_execution(self.execution(default), binding)
            MODULE.source_policy_ep1_execution(self.execution(ep1), binding)
            MODULE.source_policy_development_doctests(self.execution(docs), binding, "default-doctests")
            MODULE.source_policy_development_doctests(self.execution(ep1_docs), binding, "ep1-doctests")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_execution(self.execution(old_default), binding)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_ep1_execution(self.execution(sorted([*old_default, extra])), binding)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_development_doctests(self.execution(old_docs), binding, "default-doctests")

    def test_protected_order_ke_addition_keeps_the_current_default_and_compiler_examples(self):
        ordinary = sorted([*self.ordinary] + [
            f"source_policy::tests::age_neighbor_{index}" for index in range(6)])
        case = "nts_ke_experiment::datagram::tests::ordinary_protected_reversed_receive_bracket_cannot_commit"
        ep1 = sorted([*ordinary, "nts_ke_experiment::previous_case", case])
        docs = sorted([*self.docs])
        binding = {**self.policy["binding"], "candidate_sha256": MODULE.SOURCE_POLICY_PROTECTED_ORDER_CANDIDATE,
                   "test_inventory": ordinary, "doctest_inventory": docs}
        specs = {name: (len(names), independent_digest(names)) for name, names in (
            ("default-ordinary", ordinary), ("default-doctests", docs), ("ep1-ordinary", ep1),
            ("ep1-doctests", docs))}
        with patch.object(MODULE, "SOURCE_POLICY_PROTECTED_ORDER_ROSTER", specs):
            MODULE.source_policy_execution(self.execution(ordinary), binding)
            MODULE.source_policy_ep1_execution(self.execution(ep1), binding)
            MODULE.source_policy_development_doctests(self.execution(docs), binding, "default-doctests")
            for wrong in ([name for name in ep1 if name != case],
                          sorted(name if name != case else "nts_ke_experiment::other_order_case" for name in ep1)):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.source_policy_ep1_execution(self.execution(wrong), binding)

    def test_application_placement_keeps_ordinary_names_and_checks_both_shifted_doc_graphs(self):
        ordinary = sorted([*self.ordinary] + [
            f"source_policy::tests::placement_neighbor_{index}" for index in range(6)])
        ep1 = sorted([*ordinary, "nts_ke_experiment::placement_neighbor"])
        before = "crates/nevrotick-engine/src/measurement.rs - measurement::NominalPhaseNs (line 702) - compile fail"
        after = before.replace("702", "719")
        docs = sorted([*self.docs, after])
        ep1_docs = sorted([*docs, "crates/nevrotick-engine/src/ke_preview_target.rs - "
                           "ke_preview_target::KePreviewTarget (line 33) - compile fail"])
        binding = {**self.policy["binding"], "candidate_sha256": MODULE.SOURCE_POLICY_APPLICATION_PLACEMENT_CANDIDATE,
                   "test_inventory": ordinary, "doctest_inventory": docs}
        specs = {name: (len(names), independent_digest(names)) for name, names in (
            ("default-ordinary", ordinary), ("ep1-ordinary", ep1),
            ("default-doctests", docs), ("ep1-doctests", ep1_docs))}
        with patch.object(MODULE, "SOURCE_POLICY_APPLICATION_PLACEMENT_ROSTER", specs):
            MODULE.source_policy_execution(self.execution(ordinary), binding)
            MODULE.source_policy_ep1_execution(self.execution(ep1), binding)
            for graph, names in (("default-doctests", docs), ("ep1-doctests", ep1_docs)):
                MODULE.source_policy_development_doctests(self.execution(names), binding, graph)
                stale = sorted(before if name == after else name for name in names)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.source_policy_development_doctests(self.execution(stale), binding, graph)

    def test_receive_effect_guards_requires_each_current_ep1_case(self):
        ordinary = sorted([*self.ordinary] + [
            f"source_policy::tests::guard_neighbor_{index}" for index in range(6)])
        required = EngineKeAnalysisRoleTests.PROTECTED_RECEIVE_EFFECT_CASES
        ep1 = sorted([*ordinary, *required])
        binding = {**self.policy["binding"],
                   "candidate_sha256": MODULE.SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_CANDIDATE,
                   "test_inventory": ordinary}
        docs = binding["doctest_inventory"]
        old = MODULE.SOURCE_POLICY_APPLICATION_PLACEMENT_ROSTER
        current = MODULE.SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_ROSTER
        for graph in ("default-ordinary", "default-doctests", "ep1-doctests"):
            self.assertEqual(current[graph], old[graph])
        specs = {"default-ordinary": (len(ordinary), independent_digest(ordinary)),
                 "default-doctests": (len(docs), independent_digest(docs)),
                 "ep1-ordinary": (len(ep1), independent_digest(ep1))}
        with patch.object(MODULE, "SOURCE_POLICY_RECEIVE_EFFECT_GUARDS_ROSTER", specs):
            MODULE.source_policy_execution(self.execution(ordinary), binding)
            MODULE.source_policy_ep1_execution(self.execution(ep1), binding)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.source_policy_ep1_execution(self.execution(ordinary), binding)
            for omitted in required:
                wrong = sorted("nts_ke_experiment::other_guard_case" if case == omitted else case
                               for case in ep1)
                with self.subTest(omitted=omitted), self.assertRaises(MODULE.Incomplete):
                    MODULE.source_policy_ep1_execution(self.execution(wrong), binding)

    def test_chronology_focused_lineage_rehashes_every_pinned_evidence_file(self):
        files = {}
        for index in range(74):
            path = self.base / f"chronology-{index}.json"
            path.write_text(str(index), encoding="utf-8")
            files[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
        digest = independent_digest(files)
        reference = {"focused_evidence": files, "focused_evidence_index_sha256": digest,
                     "invocation_id": "1" * 32}
        with patch.object(MODULE, "SOURCE_POLICY_CHRONOLOGY_FOCUSED", digest):
            self.assertEqual(MODULE.admit_source_policy_chronology_focus(reference), files)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_source_policy_chronology_focus({**reference, "invocation_id": "not-hex"})
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_source_policy_chronology_focus(
                    {**reference, "focused_evidence_index_sha256": "2" * 64})
            path.write_text("changed", encoding="utf-8")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_source_policy_chronology_focus(reference)

    def test_receive_focused_lineage_requires_all_physical_raw_receipts(self):
        def write(path, data):
            raw = json.dumps(data).encode()
            path.write_bytes(raw)
            return {"path": str(path), "sha256": hashlib.sha256(raw).hexdigest()}

        capture_path = self.base / "candidate.json"
        fixture = {"prior_plain_failed_development": [], "combined_development_reference": {"sha256": "b" * 64}}
        raw_index = {"plain": [], "protected": []}
        expected = {}
        for owner, index, status, mode, key in (
                ("plain", 0, "incomplete", "green", "expected_plain_candidate_sha256"),
                ("plain", 1, "passed", "green", "expected_plain_candidate_sha256"),
                ("protected", 0, "passed", "red", "expected_protected_red_candidate_sha256"),
                ("protected", 1, "passed", "green", "expected_protected_candidate_sha256")):
            directory = self.base / f"{owner}-{index}"
            directory.mkdir()
            candidate = fixture.setdefault(key, independent_digest(key))
            lane = write(directory / "cutoff.json", {"exit_code": 101 if mode == "red" else 0, "failure": None})
            report = write(directory / "verification.json", {
                "status": status, "mode": mode, "candidate_sha256": candidate,
                "baseline_report_sha256": "b" * 64, "temporary_work_removed": True,
                "lanes": [{"lane": "cutoff", "sha256": lane["sha256"],
                           "exit_code": 101 if mode == "red" else 0, "failure": None}],
            })
            raw_index[owner].append(report)
            expected.update({ref["path"]: ref["sha256"] for ref in (report, lane)})
        fixture["plain_red_focused"] = write(self.base / "plain-red.json", {"status": "passed"})
        index = write(self.base / "focused-index.json", raw_index)
        for ref in (fixture["plain_red_focused"], index):
            expected[ref["path"]] = ref["sha256"]
        reference = {"invocation_id": "a" * 32, "focused_evidence_index_sha256": index["sha256"],
                     "focused_evidence": expected}
        missing = str(self.base / "protected-1/cutoff.json")
        with patch.object(MODULE, "SOURCE_POLICY_RECEIVE_FOCUSED", index["sha256"]):
            self.assertEqual(MODULE.admit_source_policy_receive_focus(capture_path, fixture, reference), expected)
            for tampered in ({**reference, "focused_evidence_index_sha256": "1" * 64},
                             {**reference, "invocation_id": True},
                             {**reference, "focused_evidence": {p: h for p, h in expected.items() if p != missing}}):
                with self.assertRaisesRegex(MODULE.Incomplete, "focused"):
                    MODULE.admit_source_policy_receive_focus(capture_path, fixture, tampered)
            Path(missing).write_bytes(b"{}")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_source_policy_receive_focus(capture_path, fixture, reference)

    def test_each_development_monitor_format_requires_rustdoc_before_session(self):
        for current in (False, True):
            for fault in ("missing", "failed"):
                with self.subTest(independent_monitor=current, fault=fault):
                    self.reference_fixture(independent_monitor=current)
                    path = Path(self.policy["binding"]["development_evidence"]["path"])
                    reference = json.loads(path.read_text())
                    row = next(row for row in reference["lanes"] if row["lane"] == "gnu-rustdoc")
                    if fault == "missing":
                        reference["lanes"].remove(row)
                    else:
                        raw_path = Path(row["path"])
                        raw = json.loads(raw_path.read_text())
                        raw["exit_code"] = row["exit_code"] = 1
                        raw_path.write_text(json.dumps(raw))
                        row["sha256"] = hashlib.sha256(raw_path.read_bytes()).hexdigest()
                    path.write_text(json.dumps(reference))
                    self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                    self.write_policy()
                    report = {}
                    with patch.object(MODULE.COMMON, "Session", side_effect=self.SessionReached) as session:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute_source_policy_profile(self.args, report)
                        session.assert_not_called()
                    self.assertFalse(self.args.output_dir.exists())
                    self.assertFalse(report["selected_lane_passed"])
                    self.assertFalse(report["qualification_passed"])

    def test_development_monitor_formats_bind_their_actual_physical_inputs(self):
        for current, expected_count in ((False, 14), (True, 23)):
            with self.subTest(independent_monitor=current):
                self.reference_fixture(independent_monitor=current)
                admitted = MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
                self.assertEqual(len(admitted), expected_count)
                self.assertEqual(admitted, {name: hashlib.sha256(Path(name).read_bytes()).hexdigest()
                                            for name in admitted})
                self.assertTrue(all(not name.startswith("/dev/shm/") for name in admitted))
        for marker, value in (("source_revision", 8), ("monitor_sha256", "d" * 64),
                              ("monitor_control_inputs", {})):
            with self.subTest(partial_current_marker=marker):
                self.reference_fixture()
                path = Path(self.policy["binding"]["development_evidence"]["path"])
                reference = json.loads(path.read_text())
                reference[marker] = value
                path.write_text(json.dumps(reference))
                self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        self.reference_fixture()
        path = Path(self.policy["binding"]["development_evidence"]["path"])
        reference = json.loads(path.read_text())
        reference["resource_observer_joined"] = False
        path.write_text(json.dumps(reference))
        self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_source_policy_development(self.policy["binding"], self.identity)

    def test_development_monitor_controls_preserve_owned_negative_disposition(self):
        faults = (
            ("healthy", ("monitor", "ok"), False),
            ("healthy", ("monitor", "result", "observer_pid"), 11),
            ("healthy", ("notifications",), [100.0]),
            ("gap", ("expected_control_observed",), 1),
            ("gap", ("monitor", "ok"), True),
            ("gap", ("monitor", "joined"), False),
            ("gap", ("monitor", "result", "observer_pid"), 11),
            ("gap", ("monitor", "exit_code"), False),
            ("gap", ("monitor", "result", "maximum_gap_seconds"), 1.5),
            ("gap", ("monitor", "result", "error"), "unrelated failure"),
            ("gap", ("monitor", "result", "parent_notification"), "unbound-SIGTERM"),
            ("gap", ("notifications",), []),
        )
        for role, keys, value in faults:
            with self.subTest(role=role, field=keys, value=value):
                self.reference_fixture(independent_monitor=True)
                path = Path(self.policy["binding"]["development_evidence"]["path"])
                reference = json.loads(path.read_text())
                directory = path.parent / "monitor-controls" / role
                control_path = directory / "control.json"
                control = json.loads(control_path.read_text())
                target = control
                for key in keys[:-1]:
                    target = target[key]
                target[keys[-1]] = value
                terminal = directory / "monitor-result.json"
                terminal.write_text(json.dumps(control["monitor"]["result"]))
                control["monitor"]["result_sha256"] = hashlib.sha256(terminal.read_bytes()).hexdigest()
                control_path.write_text(json.dumps(control))
                for leaf in ("control.json", "monitor-result.json"):
                    declared = f"/dev/shm/synthetic-source-policy-observer/{role}/{leaf}"
                    reference["monitor_control_inputs"][declared] = hashlib.sha256((directory / leaf).read_bytes()).hexdigest()
                path.write_text(json.dumps(reference))
                self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        self.reference_fixture(independent_monitor=True)
        self.assertEqual(len(MODULE.admit_source_policy_development(self.policy["binding"], self.identity)), 23)

    def test_development_current_monitor_refuses_unproven_custody_and_physical_drift(self):
        for removed in ("source_revision", "monitor_sha256", "monitor_control_inputs", "independent_monitor"):
            with self.subTest(missing_current_marker=removed):
                self.reference_fixture(independent_monitor=True)
                path = Path(self.policy["binding"]["development_evidence"]["path"])
                reference = json.loads(path.read_text())
                del reference[removed]
                path.write_text(json.dumps(reference))
                self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        faults = (
            (("independent_monitor", "ok"), 1),
            (("independent_monitor", "joined"), 1),
            (("independent_monitor", "joined"), False),
            (("independent_monitor", "exit_code"), False),
            (("independent_monitor", "exit_code"), 2),
            (("independent_monitor", "errors"), ["sample failed"]),
            (("independent_monitor", "result", "status"), "failed"),
            (("independent_monitor", "result", "samples"), True),
            (("independent_monitor", "result", "samples"), 3),
            (("independent_monitor", "result", "observer_pid"), 11),
            (("independent_monitor", "result", "maximum_gap_seconds"), float("nan")),
            (("independent_monitor", "result", "maximum_gap_seconds"), float("inf")),
            (("independent_monitor", "result", "maximum_gap_seconds"), 2.01),
            (("independent_monitor", "result", "maximum_gap_seconds"), 0.5),
            (("independent_monitor", "result", "finished_monotonic"), 104.0),
            (("independent_monitor", "result", "finished_monotonic"), float("nan")),
            (("independent_monitor", "result", "recipe_final_sha256"), "f" * 64),
            (("source_revision",), True),
            (("resource_observer_joined",), True),
        )
        for keys, value in faults:
            with self.subTest(field=keys, value=value):
                self.reference_fixture(independent_monitor=True)
                path = Path(self.policy["binding"]["development_evidence"]["path"])
                reference = json.loads(path.read_text())
                target = reference
                for key in keys[:-1]:
                    target = target[key]
                target[keys[-1]] = value
                # Keep physical terminal custody coherent so semantic refusal is
                # not accidentally supplied by an unrelated stale terminal hash.
                terminal = path.parent / "observer/monitor-result.json"
                terminal.write_text(json.dumps(reference["independent_monitor"]["result"]))
                reference["independent_monitor"]["result_sha256"] = hashlib.sha256(terminal.read_bytes()).hexdigest()
                path.write_text(json.dumps(reference))
                self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        for key, value in (("monotonic_seconds", True), ("monotonic_seconds", float("inf")),
                           ("gap_seconds", 0.5), ("tasks", True), ("free_bytes", 4 * 1024**3 - 1)):
            with self.subTest(sample_field=key, value=value):
                self.reference_fixture(independent_monitor=True)
                path = Path(self.policy["binding"]["development_evidence"]["path"])
                reference = json.loads(path.read_text())
                sampled = path.parent / "observer/resources.jsonl"
                rows = [json.loads(line) for line in sampled.read_text().splitlines()]
                rows[-1][key] = value
                sampled.write_text("".join(json.dumps(row) + "\n" for row in rows))
                reference["independent_monitor"]["resource_samples_sha256"] = hashlib.sha256(sampled.read_bytes()).hexdigest()
                path.write_text(json.dumps(reference))
                self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        for replacement in ("/dev/shm/other-parent/healthy/control.json",
                            "/dev/shm/synthetic-source-policy-observer/healthy/../control.json",
                            "/tmp/synthetic-source-policy-observer/healthy/control.json"):
            with self.subTest(declared_control=replacement):
                self.reference_fixture(independent_monitor=True)
                path = Path(self.policy["binding"]["development_evidence"]["path"])
                reference = json.loads(path.read_text())
                controls = reference["monitor_control_inputs"]
                controls[replacement] = controls.pop("/dev/shm/synthetic-source-policy-observer/healthy/control.json")
                path.write_text(json.dumps(reference))
                self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        self.reference_fixture(independent_monitor=True)
        path = Path(self.policy["binding"]["development_evidence"]["path"])
        physical = [path.parent.parent / "monitor.py", path.parent / "observer/monitor-result.json",
                    path.parent / "observer/resources.jsonl"]
        physical += [path.parent / "monitor-controls" / role / leaf for role in ("healthy", "gap")
                     for leaf in ("control.json", "monitor-result.json", "resources.jsonl")]
        for filename in physical:
            for missing in (False, True):
                with self.subTest(path=filename.relative_to(self.base), missing=missing):
                    self.reference_fixture(independent_monitor=True)
                    if missing:
                        filename.unlink()
                    else:
                        filename.write_bytes(filename.read_bytes() + b"changed\n")
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        self.reference_fixture(independent_monitor=True)
        self.assertEqual(len(MODULE.admit_source_policy_development(self.policy["binding"], self.identity)), 23)

    def test_development_ep1_modes_reject_hash_bound_incomplete_or_substituted_rosters(self):
        mandatory = "source_policy::tests::public_policy_diagnostics_are_literal_bounded_and_redacted"
        missing_kod = [name for name in self.ep1 if name != mandatory]
        substituted = self.ep1.copy()
        substituted[substituted.index("nts_ke_experiment::synthetic_ep1_case_000")] = (
            "nts_ke_experiment::unadmitted_same_count_case")
        duplicated = self.ep1.copy()
        duplicated[duplicated.index("nts_ke_experiment::synthetic_ep1_case_001")] = (
            "nts_ke_experiment::synthetic_ep1_case_000")
        alternate_mode = self.ep1.copy()
        alternate_mode[alternate_mode.index(mandatory)] = (
            "source_policy::tests::unadmitted_other_mode_case")
        proposals = {
            "zero": [],
            "default-only": self.ordinary,
            "missing-required-kod": missing_kod,
            "same-count-ke-substitution": sorted(substituted),
            "duplicate": sorted(duplicated),
            "different-mode-roster": sorted(alternate_mode),
        }
        for lane in ("ep1-ordinary", "ep1-optimized"):
            for defect, names in proposals.items():
                with self.subTest(lane=lane, defect=defect):
                    # Rebuild both physical hashes so an earlier custody
                    # mismatch cannot stand in for actual roster rejection.
                    # The other mode retains the full independent healthy set.
                    self.reference_fixture(ep1_outputs={lane: self.execution(names)})
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_source_policy_development(
                            self.policy["binding"], self.identity)
                    with patch.object(MODULE.COMMON, "Session") as session:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.execute_source_policy_profile(self.args, {})
                        session.assert_not_called()

    class SessionReached(Exception):
        pass

    def replace_reference_lane_output(self, lane, output):
        """Keep physical integrity valid while changing only the selected observation."""
        reference_path = self.base / "reference.json"
        reference = json.loads(reference_path.read_text())
        row = next(row for row in reference["lanes"] if row["lane"] == lane)
        raw_path = Path(row["path"])
        raw = json.loads(raw_path.read_text())
        raw["output"] = output
        raw_path.write_text(json.dumps(raw))
        row["sha256"] = hashlib.sha256(raw_path.read_bytes()).hexdigest()
        reference_path.write_text(json.dumps(reference))
        self.policy["binding"]["development_evidence"]["sha256"] = hashlib.sha256(reference_path.read_bytes()).hexdigest()
        self.write_policy()

    def test_development_ep1_admits_distinct_complete_graphs_before_session(self):
        self.assertEqual(len(self.ordinary), 267)
        self.assertEqual(len(self.ep1), 501)
        self.assertEqual(len(set(self.ep1) - set(self.ordinary)), 234)
        self.reference_fixture()
        admitted = MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        self.assertEqual(admitted, {str(path): hashlib.sha256(path.read_bytes()).hexdigest()
                                   for path in self.base.glob("*.json")})
        self.assertEqual(len(admitted), 14)
        # Ordering and sub-binary boundaries are not part of the roster contract.
        for lane in ("ep1-ordinary", "ep1-optimized"):
            self.replace_reference_lane_output(lane,
                self.execution(list(reversed(self.ep1[:234])))
                + self.execution(list(reversed(self.ep1[234:]))))
        with patch.object(MODULE.COMMON, "Session", side_effect=self.SessionReached) as session:
            with self.assertRaises(self.SessionReached):
                MODULE.execute_source_policy_profile(self.args, {})
            session.assert_called_once()
        self.assertFalse(self.args.output_dir.exists())

    def test_development_ep1_rejects_hash_valid_execution_faults(self):
        missing = [name for name in self.ep1
                   if name != "nts_ke_experiment::synthetic_ep1_case_000"]
        substituted = [name if name != "nts_ke_experiment::synthetic_ep1_case_000"
                       else "nts_ke_experiment::foreign_same_count_case" for name in self.ep1]
        self.assertEqual(len(missing), 500)
        self.assertEqual(len(substituted), 501)
        self.assertEqual(sum(name.startswith("nts_ke_experiment::") for name in substituted), 234)
        cases = (
            ("empty", "", "native block declaration or summary"),
            ("zero-tests", self.execution([]), "native block declaration or summary"),
            ("missing-ke", self.execution(missing), "Source-policy.*EP1"),
            ("duplicate-ke", self.execution(self.ep1 + ["nts_ke_experiment::synthetic_ep1_case_000"]),
             "native records are duplicated"),
            ("same-count-substitute", self.execution(substituted), "Source-policy.*EP1"),
            ("default-graph-swap", self.execution(self.ordinary), "Source-policy.*EP1"),
            ("filtered", self.execution(self.ep1).replace("0 filtered out", "1 filtered out"),
             "native block declaration or summary"),
            ("unfinished-block", self.execution(self.ep1) + "running 1 test\n",
             "native block declaration or summary"),
        )
        for lane in ("ep1-ordinary", "ep1-optimized"):
            for fault, output, reason in cases:
                with self.subTest(lane=lane, fault=fault):
                    self.reference_fixture()
                    self.replace_reference_lane_output(lane, output)
                    with self.assertRaisesRegex(MODULE.Incomplete, reason):
                        MODULE.admit_source_policy_development(self.policy["binding"], self.identity)
        self.reference_fixture()
        self.assertEqual(len(MODULE.admit_source_policy_development(self.policy["binding"], self.identity)), 14)

    def test_development_ep1_missing_case_refuses_before_session(self):
        missing = [name for name in self.ep1
                   if name != "nts_ke_experiment::synthetic_ep1_case_000"]
        for lane in ("ep1-ordinary", "ep1-optimized"):
            with self.subTest(lane=lane):
                self.reference_fixture()
                self.replace_reference_lane_output(lane, self.execution(missing))
                report = {}
                with patch.object(MODULE.COMMON, "Session", side_effect=self.SessionReached) as session:
                    with self.assertRaisesRegex(MODULE.Incomplete, "Source-policy.*EP1"):
                        MODULE.execute_source_policy_profile(self.args, report)
                    session.assert_not_called()
                self.assertFalse(self.args.output_dir.exists())
                self.assertFalse(report["selected_lane_passed"])
                self.assertFalse(report["qualification_passed"])


    def test_cli_subject_guards_preserve_existing_profile_ownership(self):
        valid = [("source_policy", name) for name in (*self.OWNERS, "all")]
        valid += [("series", "series"), ("session", "session"), ("address_round", "address_round")]
        for profile, subject in valid:
            with self.subTest(profile=profile, subject=subject), patch.object(MODULE, "execute") as execute:
                execute.side_effect = MODULE.Incomplete("synthetic dispatch reached")
                self.assertEqual(MODULE.main(["--profile", profile, "--subject", subject,
                    "--lane", "coverage", "--vendor-dir", str(self.base / "vendor"),
                    "--output-dir", str(self.base / "unused")]), 1)
                execute.assert_called_once()
        for profile, subject in (("source_policy", "quality"), ("source_policy", "address_progress"),
                                 ("legacy", "source_policy"), ("series", "source_policy"),
                                 ("legacy", "session"), ("legacy", "address_round")):
            with self.subTest(profile=profile, subject=subject), patch.object(MODULE, "execute") as execute:
                with self.assertRaises(SystemExit):
                    MODULE.main(["--profile", profile, "--subject", subject, "--lane", "coverage",
                        "--output-dir", str(self.base / "unused")])
                execute.assert_not_called()


if __name__ == "__main__":
    unittest.main()
