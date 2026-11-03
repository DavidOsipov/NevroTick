"""Independent literal chart recipe, custody and native-text controls."""
import ast
import hashlib
import importlib.util
import json
from pathlib import Path
from types import SimpleNamespace
import unittest
from unittest import mock


SCRIPTS = Path(__file__).resolve().parents[1]


def load(name):
    """Load one explicitly named pure helper; never import the coordinator."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), SCRIPTS / (name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


MODULE = load("core-chart-accounting")
ADAPTIVE = load("core-adaptive-accounting")
PROJECTION = load("core-projection-accounting")
ENCLOSURE = load("core-enclosure-accounting")
CONTRACTS = load("core-chart-contracts")

# Native Cargo/libtest names fixed independently of the implementation's tuple.
NAMES = (
    "s144_chart_reexpresses_without_mutating_genuine_history",
    "weighted_tracking_uses_one_common_chart_reference_and_genuine_owner",
    "post_prune_auxiliary_row_is_required_and_selected_fit_is_preserved",
    "refusal_unknown_and_context_mismatch_leave_history_unchanged",
    "producer_errors_precede_query_domain_and_preserve_source",
    "full_capacity_and_inclusive_domain_preserve_all_actual_rows",
    "invalid_scalar_and_overflow_leave_real_source_unchanged",
    "chart_descriptor_identity_and_integer_domain_boundaries",
    "negative_elapsed_and_rate_reach_actual_chart_enclosure",
    "regression::chart::roundoff::tests::upper_operators_preserve_zero_and_underflow_neighbors",
    "regression::chart::roundoff::tests::upper_operators_reject_invalid_and_exhausted_bounds",
    "regression::chart::roundoff::tests::value_primitives_preserve_steps_and_reject_underflow",
    "regression::chart::roundoff::tests::signed_elapsed_preserves_exact_integer_limits",
    "regression::chart::chart_guards::chart_raw_frame_and_domain_carriers",
    "regression::chart::chart_guards::chart_raw_affine_carrier_checks_both_signed_fields",
    "regression::chart::chart_guards::chart_raw_projection_candidate_roles_and_precedence",
    "regression::chart::chart_guards::chart_raw_tracking_candidate_roles_and_precedence",
    "regression::chart::chart_guards::chart_steps_preserve_actual_operation_mapping",
    "regression::chart::chart_guards::chart_errors_preserve_closed_messages_and_typed_sources",
)
COMPLETE = "".join("test " + name + " ... ok\n" for name in NAMES)
LEGAL_ROW = (
    "CORE_CHART_CONTRACT_PROFILE labels=5 contexts=2 monotonic=4 coordinates=1 exchanges=1"
    " observations=3 histories=1 appends=3 frames=1 domains=1 charts=1 prepares=4"
    " terms=1 tracked_projections=1 enclosures=1 getters=22\n"
)
LEGAL_SUCCESS = "legal-chart-contract\n"


def budget(**options):
    """Exercise actual adopted producer recurrences with fixed source input."""
    return MODULE.chart_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE, **options)


def adaptive_changed(change):
    """Alter one receiving result from a genuine pure dependency for a control."""
    def work(calls, **options):
        value = ADAPTIVE.source_work(calls, **options)
        change(value)
        return value
    return SimpleNamespace(FIELDS=ADAPTIVE.FIELDS, source_work=work)


def old_vector():
    """Synthetic complete old-width vector with explicit aggregate neighbors."""
    return {
        **{"old.owner." + str(index): index for index in range(1208)},
        "charged_fit_work_upper": 29339,
        "point_admissions_upper": 1086518,
        "standard_errors.geometry.report_copies_upper": 29339,
        "standard_errors.geometry.logical_bytes_upper": 938848,
    }


class ChartAccountingTests(unittest.TestCase):
    """Source upper bounds stay distinct from source identity and observed calls."""

    def test_package_actual_dependencies_and_independent_numeric_upper(self):
        value = budget()
        self.assertEqual(len(value), 296)
        self.assertEqual(MODULE.chart_numeric_work(value), {
            "charged_fit_work_upper": 5952, "point_admissions_upper": 205536})
        self.assertEqual(value["chart.adaptive_call.adaptive_appends"], 96)
        self.assertEqual(value["chart.adaptive_work.selector_entries_upper"], 96)
        self.assertEqual(value["chart.adaptive_work.adaptive_generated_points_upper"], 6144)
        self.assertEqual(value["chart.adaptive_work.selector_point_copy_upper"], 199392)
        self.assertEqual(value["chart.projection_call.projections"], 17)
        self.assertEqual(value["chart.enclosure_call.enclosures"], 7)
        self.assertEqual(value["chart.projection_work.projection_arithmetic_primitive_calls_upper"], 85)
        self.assertEqual(value["chart.tracking_work.standard_error_requests_upper"], 5)
        self.assertEqual(value["chart.roundoff_work.total_shared_value_primitives_upper"], 124)
        self.assertEqual(value["chart.roundoff_work.bound_requests_upper"], 608)
        for prefix in ("projection_work", "enclosure_work"):
            self.assertEqual(value["chart." + prefix + ".charged_fit_work_upper"], 0)
            self.assertEqual(value["chart." + prefix + ".point_admissions_upper"], 0)
        self.assertEqual(value["chart.guard_private.monotonic_constructors"], 18)
        self.assertEqual(value["chart.guard_private.monotonic_getters_upper"], 24)
        self.assertEqual(value["chart.guard_private.garde_validations_upper"], 60)
        self.assertEqual(value["chart.guard_private.garde_callbacks_upper"], 162)
        self.assertEqual(value["chart.guard_private.predicate_requests_upper"], 252)
        self.assertEqual(value["chart.fixture.debug_format_requests_upper"], 7)

    def test_legal_recipe_is_separate_and_has_no_package_private_or_docs_work(self):
        value = budget(external_legal=True)
        self.assertEqual(len(value), 296)
        self.assertEqual(MODULE.chart_numeric_work(value), {
            "charged_fit_work_upper": 186, "point_admissions_upper": 6423})
        for name, amount in value.items():
            if name.startswith(("chart.guard_private.", "chart.roundoff_private.", "chart.doctest.")):
                self.assertEqual(amount, 0, name)
        self.assertEqual(value["chart.fixture.prepares"], 4)
        self.assertEqual(value["chart.fixture.known_prepares_upper"], 1)
        self.assertEqual(value["chart.legal_observer.counter_charge_attempts"], 52)
        self.assertEqual(value["chart.owner_work.history_relationship_passes_upper"], 2)
        self.assertEqual(value["chart.tracking_work.standard_error_requests_upper"], 1)
        self.assertEqual(value["chart.fixture.debug_format_requests_upper"], 0)

    def test_doctests_and_private_scope_change_only_their_disjoint_groups(self):
        complete, reduced = budget(), budget(include_doctests=False, include_guard_tests=False)
        self.assertEqual(set(complete), set(reduced))
        for key in complete:
            self.assertEqual(reduced[key], 0 if key.startswith(("chart.doctest.", "chart.guard_private."))
                             else complete[key], key)
        self.assertEqual(complete["chart.doctest.compilation_cases"], 6)
        self.assertEqual(complete["chart.doctest.compile_fail_cases"], 4)
        self.assertEqual(complete["chart.doctest.fit_or_point_work"], 0)

    def test_disjoint_extension_preserves_old_names_and_nonaggregate_values(self):
        old, new = old_vector(), budget()
        frozen_old = dict(old)
        merged = MODULE.preserve_and_extend(old, new)
        self.assertEqual(len(merged), 1508)
        self.assertEqual(old, frozen_old)
        self.assertEqual(merged["charged_fit_work_upper"], 35291)
        self.assertEqual(merged["point_admissions_upper"], 1292054)
        self.assertEqual(merged["standard_errors.geometry.report_copies_upper"], 35291)
        self.assertEqual(merged["standard_errors.geometry.logical_bytes_upper"], 1129312)
        for name in old:
            if name.startswith("old."):
                self.assertEqual(merged[name], old[name])
        self.assertEqual({key: merged[key] for key in new}, new)
        new["chart.extra"] = 0
        with self.assertRaises(MODULE.Incomplete):
            MODULE.preserve_and_extend(old, new)

    def test_extension_rejects_namespace_collision_missing_aggregate_and_overflow(self):
        for kind in ("collision", "aggregate", "overflow", "bool"):
            with self.subTest(kind=kind):
                old, new = old_vector(), budget()
                if kind == "collision":
                    old[next(iter(new))] = old.pop("old.owner.0")
                elif kind == "aggregate":
                    old["wrong.aggregate"] = old.pop("charged_fit_work_upper")
                elif kind == "overflow":
                    old["charged_fit_work_upper"] = (1 << 63) - 1
                else:
                    old["old.owner.0"] = False
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.preserve_and_extend(old, new)

    def test_exact_scope_flags_and_owner_fields_reject_aliases(self):
        for key in ("external_legal", "include_doctests", "include_guard_tests"):
            for invalid in (None, 0, 1, "true"):
                with self.subTest(key=key, value=invalid), self.assertRaises(MODULE.Incomplete):
                    budget(**{key: invalid})
        for fields in (None, list(ADAPTIVE.FIELDS), ADAPTIVE.FIELDS[:-1], ADAPTIVE.FIELDS[::-1]):
            owner = SimpleNamespace(FIELDS=fields, source_work=ADAPTIVE.source_work)
            with self.subTest(fields=fields), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_budget_vector(owner, PROJECTION, ENCLOSURE)

    def test_dependency_key_shape_numeric_types_bounds_and_wrong_recurrence_reject(self):
        changes = (
            lambda work: work.pop("selector_entries_upper"),
            lambda work: work.update(unknown=work.pop("selector_entries_upper")),
            lambda work: work.update(selector_entries_upper=False),
            lambda work: work.update(selector_entries_upper=-1),
            lambda work: work.update(selector_entries_upper=1 << 63),
            lambda work: work.update(charged_fit_work_upper=5951),
            lambda work: work.update(point_admissions_upper=205535),
        )
        for index, change in enumerate(changes):
            with self.subTest(index=index), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_budget_vector(adaptive_changed(change), PROJECTION, ENCLOSURE)

    def test_missing_protocol_field_rejects_before_accounting_or_legal_parsing(self):
        unexpected = mock.Mock(side_effect=AssertionError("Malformed owner must stop before work"))
        missing = SimpleNamespace(source_work=unexpected, source_work_uppers=unexpected,
                                  projection_adapter_work=unexpected,
                                  legal_contract_source=unexpected, contract_counts=unexpected)
        for owners in ((missing, PROJECTION, ENCLOSURE), (ADAPTIVE, missing, ENCLOSURE),
                       (ADAPTIVE, PROJECTION, missing)):
            with self.subTest(owners=owners), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_budget_vector(*owners)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_legal_observation(LEGAL_ROW + LEGAL_SUCCESS, missing)
        unexpected.assert_not_called()

    def test_old_projection_cannot_double_fund_new_adaptive_setup(self):
        def changed(calls):
            value = PROJECTION.source_work_uppers(calls)
            value["charged_fit_work_upper"] = 1
            return value
        owner = SimpleNamespace(FIELDS=PROJECTION.FIELDS, source_work_uppers=changed,
                                projection_adapter_work=PROJECTION.projection_adapter_work)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_budget_vector(ADAPTIVE, owner, ENCLOSURE)

    def test_source_identity_exact_complete_map_and_current_bytes(self):
        actual = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest()
                  for name in MODULE.SOURCE_SHA256}
        self.assertEqual(MODULE.admit_source_identity(actual), actual)
        for kind in ("missing", "extra", "stale", "invalid", "bool", "not-map"):
            bad = dict(actual)
            name = next(iter(bad))
            if kind == "missing":
                del bad[name]
            elif kind == "extra":
                bad["unbound.rs"] = "0" * 64
            elif kind == "stale":
                bad[name] = "0" * 64
            elif kind == "invalid":
                bad[name] = "a" * 63
            elif kind == "bool":
                bad[name] = True
            else:
                bad = list(bad.items())
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_source_identity(bad)

    def test_literal_all_nineteen_outcomes_have_no_measured_operation_counts(self):
        self.assertEqual(set(NAMES), set(MODULE.CURRENT_ORDINARY_TESTS))
        for text in (COMPLETE, COMPLETE.replace("\n", "\r\n"),
                     COMPLETE.replace("test s144_", "test chart::s144_")):
            value = MODULE.chart_test_observation(text, require_complete=True)
            self.assertFalse(value["chart_source_only_pending"])
            self.assertEqual(set(value["chart_test_statuses"].values()), {"ok"})
            self.assertEqual(value["chart_recorded_source_work_upper"], {
                "charged_fit_work_upper": 5952, "point_admissions_upper": 205536})
            self.assertIsNone(value["chart_observed_operation_counts"])

    def test_empty_partial_failed_and_unterminated_outcomes_remain_pending(self):
        for text in ("", COMPLETE.splitlines(keepends=True)[0], COMPLETE[:-1],
                     COMPLETE.replace(" ... ok\n", " ... FAILED\n", 1),
                     COMPLETE.replace(" ... ok\n", " ... ignored\n", 1),
                     COMPLETE.replace(" ... ok\n", " ... ok extra\n", 1)):
            with self.subTest(text=text[:90]):
                value = MODULE.chart_test_observation(text)
                self.assertTrue(value["chart_source_only_pending"])
                self.assertIsNone(value["chart_recorded_source_work_upper"])
                self.assertIsNone(value["chart_observed_operation_counts"])
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.chart_test_observation(text, require_complete=True)
        self.assertEqual(MODULE.chart_numeric_work(budget())["charged_fit_work_upper"], 5952)

    def test_replayed_outcomes_reject_and_json_or_nextest_rows_do_not_supply_evidence(self):
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_test_observation(COMPLETE + COMPLETE.splitlines(keepends=True)[0])
        for text in (json.dumps({"message": COMPLETE}) + "\n",
                     "".join("PASS [0.1s] time-core::chart " + name + "\n" for name in NAMES)):
            value = MODULE.chart_test_observation(text)
            self.assertTrue(value["chart_source_only_pending"])
            self.assertEqual(set(value["chart_test_statuses"].values()), {"missing"})

    def test_outcome_type_boolean_and_character_bounds_reject(self):
        for text in (None, b"", 0, " " * (16 * 1024 * 1024 + 1)):
            with self.subTest(kind=type(text)), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_test_observation(text)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_test_observation(COMPLETE, require_complete=1)

    def test_actual_legal_consumer_metadata_and_raw_marker_are_separate(self):
        for text in (LEGAL_ROW + LEGAL_SUCCESS, LEGAL_SUCCESS + LEGAL_ROW,
                     (LEGAL_ROW + LEGAL_SUCCESS).replace("\n", "\r\n")):
            value = MODULE.chart_legal_observation(text, CONTRACTS)
            self.assertEqual(sum(value.values()), 52)
            self.assertEqual(value["appends"], 3)
            self.assertEqual(value["getters"], 22)
        for text in (COMPLETE, LEGAL_ROW, LEGAL_SUCCESS, LEGAL_ROW + LEGAL_SUCCESS.rstrip(),
                     LEGAL_ROW.replace("getters=22", "getters=21") + LEGAL_SUCCESS,
                     LEGAL_ROW + LEGAL_SUCCESS + "extra\n"):
            with self.subTest(text=text[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_legal_observation(text, CONTRACTS)

    def test_legal_fields_calls_and_emitted_source_hash_reject_drift(self):
        source = CONTRACTS.legal_contract_source()
        sha = hashlib.sha256(source.encode()).hexdigest()
        expected = CONTRACTS.legal_expected()
        self.assertEqual(MODULE.admit_legal_metadata(CONTRACTS.FIELDS, expected, sha), expected)
        cases = (
            (CONTRACTS.FIELDS[::-1], expected, sha),
            (CONTRACTS.FIELDS, {**expected, "histories": True}, sha),
            (CONTRACTS.FIELDS, {**expected, "getters": 21}, sha),
            (CONTRACTS.FIELDS, expected, "0" * 64),
        )
        for fields, calls, digest in cases:
            with self.subTest(fields=fields, digest=digest), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_legal_metadata(fields, calls, digest)
        parsed = mock.Mock(side_effect=AssertionError("Parser must not run after stale source"))
        stale = SimpleNamespace(FIELDS=CONTRACTS.FIELDS, legal_expected=CONTRACTS.legal_expected,
                                legal_contract_source=lambda: source + "\n", contract_counts=parsed)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_legal_observation(LEGAL_ROW + LEGAL_SUCCESS, stale)
        parsed.assert_not_called()

    def test_pure_import_and_source_functions_have_no_io_or_process(self):
        source = (SCRIPTS / "core-chart-accounting.py").read_text()
        parsed = ast.parse(source)
        imports = {node.module for node in ast.walk(parsed) if isinstance(node, ast.ImportFrom)}
        imports.update(alias.name for node in ast.walk(parsed) if isinstance(node, ast.Import)
                       for alias in node.names)
        self.assertEqual(imports, {"__future__", "hashlib", "re", "typing"})
        compiled = compile(source, "pure_chart_import", "exec")
        with mock.patch("builtins.open", side_effect=AssertionError("I/O")), \
                mock.patch.object(Path, "open", side_effect=AssertionError("Path I/O")), \
                mock.patch("subprocess.Popen", side_effect=AssertionError("process")):
            fresh = {}
            exec(compiled, fresh)
            value = fresh["chart_budget_vector"](ADAPTIVE, PROJECTION, ENCLOSURE)
            self.assertEqual(len(value), 296)


if __name__ == "__main__":
    unittest.main()
