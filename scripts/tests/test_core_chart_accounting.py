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
SELECTION = load("core-selection-accounting")
CONTRACTS = load("core-chart-contracts")
RATE = load("core-rate-delivery-accounting")

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
    " terms=1 tracked_projections=1 enclosures=1 getters=22"
    " batches=1 minima=1 selections=1 batch_getters=22"
    " raw_enclosures=1 raw_batches=1 principal_parameters=1 principal_strata=1"
    " principal_seeds=1 principal_advances=1 chart_principal_advances=1 principal_getters=27\n"
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


BATCH_NAMES = (
    "chart_batch_cases::chart_batch_selects_genuine_majority_at_common_time",
    "chart_batch_cases::chart_batch_source_loss_does_not_lower_minimum",
    "chart_batch_cases::chart_batch_empty_and_capacity_boundaries_use_real_owners",
    "chart_batch_cases::chart_batch_compares_every_chart_frame_field",
    "chart_batch_cases::chart_batch_orders_domain_scalar_time_and_entry_mismatches",
    "chart_batch_cases::chart_batch_preserves_real_duplicate_cause_and_redaction",
    "chart_batch_cases::chart_batch_equal_descriptor_values_preserve_pointers_and_zero_bits",
    "chart_batch::tests::raw_chart_batch_header_checks_inclusive_domain",
)
BATCH_COMPLETE = "".join("test " + name + " ... ok\n" for name in BATCH_NAMES)


def batch_budget(**options):
    """Invoke the actual four injected source-only owners, never the coordinator."""
    return MODULE.chart_batch_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE, SELECTION, **options)


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

    def test_source_identity_exact_historical_map_and_current_rejection(self):
        actual = dict(MODULE.SOURCE_SHA256)
        self.assertEqual(MODULE.admit_source_identity(actual), actual)
        current = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest() for name in actual}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_source_identity(current)
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
            self.assertEqual(sum(value.values()), 111)
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


class ChartBatchAccountingTests(unittest.TestCase):
    """Independent S155 arithmetic, preservation and hostile receiving controls."""

    def test_disjoint_real_producer_recurrences_and_complete_batch_work(self):
        value = batch_budget()
        self.assertEqual(len(value), 315)
        self.assertEqual(set(value), set(MODULE.BATCH_BUDGET_KEYS))
        self.assertEqual(MODULE.chart_batch_numeric_work(value), {
            "charged_fit_work_upper": 5952, "point_admissions_upper": 205536})
        expected = {
            "fixture.histories": 32, "fixture.appends": 96,
            "fixture.tracking_projections": 46, "fixture.snapshots": 64,
            "fixture.snapshot_rows_upper": 192, "fixture.snapshot_getters_upper": 1984,
            "fixture.other_getters_upper": 4096,
            "adaptive_work.selector_entries_upper": 96,
            "projection_work.projection_arithmetic_primitive_calls_upper": 230,
            "tracking_work.standard_error_binary_primitives_upper": 368,
            "tracking_work.standard_error_square_roots_upper": 92,
            "roundoff_work.total_shared_value_primitives_upper": 368,
            "roundoff_work.bound_requests_upper": 1932,
            "owner_work.history_row_visits_upper": 5888,
            "batch_work.constructor_attempts_upper": 46,
            "batch_work.garde_validations_upper": 51,
            "batch_work.garde_callbacks_upper": 51,
            "batch_work.relationship_passes_upper": 97,
            "batch_work.entry_visits_upper": 1408,
            "batch_work.entry_scalar_comparisons_upper": 14080,
            "batch_work.scratch_initialization_writes_upper": 704,
            "batch_work.scratch_interval_copy_writes_upper": 704,
            "batch_work.legacy_batch_attempts_upper": 44,
            "batch_work.minimum_constructions_upper": 8,
            "batch_work.minimum_garde_validations_upper": 8,
            "batch_work.minimum_rule_callbacks_upper": 8,
            "selection_work.selection_endpoint_collection_writes_upper": 256,
            "selection_work.selection_endpoint_sort_comparisons_upper": 3968,
            "selection_work.selection_membership_probes_upper": 8064,
            "selection_work.selection_batch_label_comparisons_upper": 5280,
            "raw_header.labels": 1, "raw_header.monotonic_constructors_upper": 10,
            "raw_header.descriptor_garde_validations_upper": 3,
            "raw_header.descriptor_garde_callbacks_upper": 4,
            "raw_header.direct_descriptor_getters_upper": 20,
            "raw_header.direct_float_bit_queries_upper": 10,
            "raw_header.raw_array_visits": 5,
        }
        for name, amount in expected.items():
            self.assertEqual(value["chart_batch." + name], amount, name)
        for group in ("projection_work", "enclosure_work"):
            for key in ("charged_fit_work_upper", "point_admissions_upper"):
                self.assertEqual(value["chart_batch." + group + "." + key], 0)

    def test_preserves_all_old_fields_and_adds_numeric_geometry_once(self):
        old = MODULE.preserve_and_extend(old_vector(), budget())
        frozen = dict(old)
        supplement = batch_budget()
        combined = MODULE.preserve_chart_batch_extension(old, supplement)
        self.assertEqual(len(combined), 1823)
        self.assertEqual(old, frozen)
        aggregates = {
            "charged_fit_work_upper": 41243, "point_admissions_upper": 1497590,
            "standard_errors.geometry.report_copies_upper": 41243,
            "standard_errors.geometry.logical_bytes_upper": 1319776,
        }
        for key in old:
            self.assertEqual(combined[key], aggregates.get(key, old[key]), key)
        self.assertEqual({key: combined[key] for key in supplement}, supplement)
        for kind in ("collision", "missing", "overflow", "bool", "replacement"):
            left, right = dict(old), dict(supplement)
            if kind == "collision":
                left[next(iter(right))] = left.pop("old.owner.0")
            elif kind == "missing":
                left["wrong.aggregate"] = left.pop("charged_fit_work_upper")
            elif kind == "overflow":
                left["standard_errors.geometry.logical_bytes_upper"] = (1 << 63) - 1
            elif kind == "bool":
                right[next(iter(right))] = True
            else:
                right["chart_batch.unknown"] = right.pop(next(iter(right)))
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                MODULE.preserve_chart_batch_extension(left, right)

    def test_adaptive_partition_preserves_existing_per_call_ceiling_and_no_fixed_charges(self):
        called = []
        def work(calls, **options):
            called.append((dict(calls), dict(options)))
            return ADAPTIVE.source_work(calls, **options)
        owner = SimpleNamespace(FIELDS=ADAPTIVE.FIELDS, source_work=work)
        value = MODULE.chart_batch_budget_vector(owner, PROJECTION, ENCLOSURE, SELECTION)
        self.assertEqual(len(called), 4)
        expected = dict.fromkeys(ADAPTIVE.FIELDS, 0)
        expected.update(adaptive_histories=8, adaptive_observations=24, adaptive_appends=24)
        for calls, options in called:
            self.assertEqual(calls, expected)
            self.assertEqual(options, {"include_raw_minimum": False, "event_fixture_reached": False})
        for field in ADAPTIVE.FIELDS:
            self.assertEqual(sum(calls[field] for calls, _ in called),
                             value["chart_batch.adaptive_call." + field])
        self.assertEqual(value["chart_batch.adaptive_work.adaptive_minimum_validations_upper"], 32)
        self.assertEqual(value["chart_batch.adaptive_work.raw_event_fixture_entries_upper"], 0)
        self.assertEqual(value["chart_batch.adaptive_work.adaptive_private_append_calls_upper"], 0)

    def test_legal_delta_has_no_repeated_fit_projection_or_history_work(self):
        value = batch_budget(external_legal=True)
        self.assertEqual(len(value), 315)
        self.assertEqual(MODULE.chart_batch_numeric_work(value), {
            "charged_fit_work_upper": 0, "point_admissions_upper": 0})
        for name, amount in value.items():
            if name.startswith(tuple("chart_batch." + key for key in (
                    "adaptive_call.", "adaptive_work.", "projection_call.", "projection_work.",
                    "enclosure_call.", "enclosure_work.", "adapter_work.", "owner_work.",
                    "tracking_work.", "roundoff_work.", "raw_header.", "doctest."))):
                self.assertEqual(amount, 0, name)
        self.assertEqual(value["chart_batch.batch_work.constructor_attempts_upper"], 1)
        self.assertEqual(value["chart_batch.batch_work.selection_dispatches_upper"], 1)
        self.assertEqual(value["chart_batch.batch_work.minimum_constructions_upper"], 1)
        self.assertEqual(value["chart_batch.batch_work.garde_validations_upper"], 1)
        self.assertEqual(value["chart_batch.observer.record_integer_fields"], 4)
        self.assertEqual(value["chart_batch.observer.record_rows"], 0)
        self.assertEqual(value["chart_batch.observer.iterator_requests_upper"], 2)
        self.assertEqual(value["chart_batch.fixture.other_getters_upper"], 20)
        self.assertEqual(value["chart_batch.observer.counter_charge_attempts"], 25)
        old = budget(external_legal=True)
        self.assertEqual(old["chart.legal_observer.counter_charge_attempts"], 52)
        self.assertEqual(old["chart.fixture.other_getters_upper"], 22)

    def test_five_doc_cases_have_no_executed_producers_or_guard_scope_changes(self):
        full, reduced = batch_budget(), batch_budget(include_doctests=False)
        self.assertEqual(set(full), set(reduced))
        for key in full:
            self.assertEqual(reduced[key], 0 if key.startswith("chart_batch.doctest.") else full[key], key)
        self.assertEqual(full["chart_batch.doctest.compilation_cases"], 5)
        self.assertEqual(full["chart_batch.doctest.compile_fail_cases"], 4)
        self.assertEqual(full["chart_batch.doctest.uncalled_legal_helpers"], 1)
        self.assertEqual(full["chart_batch.doctest.executed_producers"], 0)

    def test_bad_scope_injected_schema_bounds_and_double_funding_reject(self):
        for field in ("external_legal", "include_doctests"):
            for wrong in (0, 1, None, "false"):
                with self.subTest(field=field, wrong=wrong), self.assertRaises(MODULE.Incomplete):
                    batch_budget(**{field: wrong})
        for field in ("charged_fit_work_upper", "point_admissions_upper"):
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_batch_budget_vector(adaptive_changed(lambda work: work.update({field: 0})),
                                                 PROJECTION, ENCLOSURE, SELECTION)
        for kind in ("missing", "replacement", "bool", "negative", "overflow"):
            def malformed(selections, batches):
                value = SELECTION.source_work_uppers(selections, batches)
                key = "selection_membership_probes_upper"
                if kind == "missing":
                    value.pop(key)
                elif kind == "replacement":
                    value["wrong"] = value.pop(key)
                else:
                    value[key] = {"bool": False, "negative": -1, "overflow": 1 << 63}[kind]
                return value
            owner = SimpleNamespace(source_work_uppers=malformed)
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_batch_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE, owner)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_batch_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE, SimpleNamespace())
        def repeated(calls):
            value = PROJECTION.source_work_uppers(calls)
            value["point_admissions_upper"] = 1
            return value
        owner = SimpleNamespace(FIELDS=PROJECTION.FIELDS, source_work_uppers=repeated,
                                projection_adapter_work=PROJECTION.projection_adapter_work)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_batch_budget_vector(ADAPTIVE, owner, ENCLOSURE, SELECTION)

    def test_source_binding_requires_reviewed_complete_map_and_rejects_drift(self):
        actual = dict(MODULE.BATCH_SOURCE_SHA256)
        self.assertEqual(MODULE.admit_chart_batch_source_identity(actual), actual)
        current = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest() for name in actual}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_chart_batch_source_identity(current)
        synthetic = {name: "a" * 64 for name in MODULE.BATCH_SOURCE_INPUTS}
        with mock.patch.object(MODULE, "BATCH_SOURCE_SHA256", {}):
            for value in ({}, synthetic):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_chart_batch_source_identity(value)
        with mock.patch.object(MODULE, "BATCH_SOURCE_SHA256", synthetic):
            self.assertEqual(MODULE.admit_chart_batch_source_identity(synthetic), synthetic)
            for kind in ("missing", "extra", "stale", "bool", "invalid", "not-map"):
                bad = dict(synthetic)
                name = next(iter(bad))
                if kind == "missing":
                    bad.pop(name)
                elif kind == "extra":
                    bad["not-bound"] = "a" * 64
                elif kind == "stale":
                    bad[name] = "b" * 64
                elif kind == "bool":
                    bad[name] = True
                elif kind == "invalid":
                    bad[name] = "a" * 63
                else:
                    bad = list(bad.items())
                with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_chart_batch_source_identity(bad)

    def test_exact_eight_names_include_raw_header_without_operation_observation(self):
        self.assertEqual(BATCH_NAMES, MODULE.BATCH_ORDINARY_TESTS)
        for text in (BATCH_COMPLETE, BATCH_COMPLETE.replace("\n", "\r\n"),
                     BATCH_COMPLETE.replace("test chart_batch_cases::", "test chart::chart_batch_cases::")):
            observed = MODULE.chart_batch_test_observation(text, require_complete=True)
            self.assertFalse(observed["chart_batch_source_only_pending"])
            self.assertEqual(set(observed["chart_batch_test_statuses"].values()), {"ok"})
            self.assertEqual(observed["chart_batch_recorded_source_work_upper"], {
                "charged_fit_work_upper": 5952, "point_admissions_upper": 205536})
            self.assertIsNone(observed["chart_batch_observed_operation_counts"])

    def test_partial_failed_replayed_or_embedded_names_cannot_supply_completion(self):
        for text in ("", "".join(BATCH_COMPLETE.splitlines(keepends=True)[:-1]), BATCH_COMPLETE[:-1],
                     BATCH_COMPLETE.replace(" ... ok\n", " ... FAILED\n", 1),
                     BATCH_COMPLETE.replace(" ... ok\n", " ... ignored\n", 1),
                     BATCH_COMPLETE.replace(" ... ok\n", " ... ok extra\n", 1),
                     json.dumps({"message": BATCH_COMPLETE}) + "\n",
                     "".join("PASS [0.1s] time-core::chart " + name + "\n" for name in BATCH_NAMES)):
            with self.subTest(text=text[:80]):
                observed = MODULE.chart_batch_test_observation(text)
                self.assertTrue(observed["chart_batch_source_only_pending"])
                self.assertIsNone(observed["chart_batch_recorded_source_work_upper"])
                self.assertIsNone(observed["chart_batch_observed_operation_counts"])
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.chart_batch_test_observation(text, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_batch_test_observation(BATCH_COMPLETE + BATCH_COMPLETE.splitlines(keepends=True)[0])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_batch_test_observation(BATCH_COMPLETE, require_complete=1)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_batch_test_observation(" " * (16 * 1024 * 1024 + 1))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_batch_test_observation(b"")


PRINCIPAL_NAMES = (
    "chart_principal_cases::retained_owner_preserves_credit_and_actual_chart_provenance",
    "chart_principal_cases::genuine_new_observation_survives_chart_receiving_once",
    "chart_principal_cases::unexpected_chart_rejects_atomically_and_equal_neighbor_recovers",
    "chart_principal_cases::source_origin_and_evaluation_guards_remain_unavoidable",
    "chart_principal_cases::expected_frame_bits_and_metadata_capacity_reject_before_commit",
    "chart_principal_cases::chart_event_rejections_preserve_credit_for_valid_retry",
    "chart_principal_cases::chart_principal_exact_metadata_capacity_and_overflow_rollback",
    "chart_principal_pending::chart_replacement_consumes_preexisting_pending_credit_once",
)
PRINCIPAL_COMPLETE = "".join("test " + name + " ... ok\n" for name in PRINCIPAL_NAMES)


def principal_budget(**options):
    """Reserve the S157 source delta through the genuine injected recurrences."""
    return MODULE.chart_principal_budget_vector(
        ADAPTIVE, PROJECTION, ENCLOSURE, SELECTION, **options)


class ChartPrincipalAccountingTests(unittest.TestCase):
    """Independent S157 census, prior-field preservation and hostile outcomes."""

    def test_complete_source_delta_has_separately_funded_owners(self):
        value = principal_budget()
        self.assertEqual(set(value), set(MODULE.PRINCIPAL_BUDGET_KEYS))
        self.assertEqual(len(value), MODULE.PRINCIPAL_BUDGET_FIELDS)
        self.assertEqual(MODULE.chart_principal_numeric_work(value), {
            "charged_fit_work_upper": 7564, "point_admissions_upper": 261202})
        expected = {
            "fixture.histories": 39, "fixture.appends": 122,
            "fixture.observations": 122, "fixture.prepares": 52,
            "fixture.tracking_projections": 52, "fixture.enclosures": 52,
            "fixture.raw_enclosures": 34, "fixture.charts": 24,
            "fixture.chart_batches": 16, "fixture.legacy_batches": 9,
            "fixture.principal_seeds": 8, "fixture.legacy_advances": 8,
            "fixture.chart_advances": 36, "fixture.principal_strata": 99,
            "fixture.principal_parameters": 9, "fixture.state_clones": 11,
            "fixture.value_equalities": 31,
            "adaptive_call.adaptive_histories": 39,
            "adaptive_work.selector_entries_upper": 122,
            "projection_call.projections": 52, "enclosure_call.enclosures": 86,
            "owner_work.history_row_visits_upper": 6656,
            "roundoff_work.bound_requests_upper": 2184,
            "chart_batch_work.constructor_attempts_upper": 16,
            "chart_batch_work.selection_dispatches_upper": 36,
            "legacy_batch_work.constructor_attempts_upper": 9,
            "descriptor_work.garde_validations_upper": 36,
            "descriptor_work.relationship_passes_upper": 72,
            "receiver_work.receiver_garde_validations_upper": 52,
            "receiver_work.receiver_relationship_passes_upper": 104,
            "receiver_work.checked_scalar_primitives_upper": 3608,
            "receiver_work.staged_state_clone_cells_upper": 704,
            "observer.state_clone_cells_upper": 176,
            "observer.value_equality_slot_visits_upper": 496,
            "selection_work.selection_endpoint_collection_writes_upper": 1408,
            "selection_work.selection_batch_copy_adapter_attempts_upper": 400,
        }
        for key, amount in expected.items():
            self.assertEqual(value["chart_principal." + key], amount, key)
        for group in ("projection_work", "enclosure_work"):
            for field in ("charged_fit_work_upper", "point_admissions_upper"):
                self.assertEqual(value["chart_principal." + group + "." + field], 0)

    def test_complete_adaptive_partition_preserves_all_observations(self):
        called = []
        def work(calls, **options):
            called.append((dict(calls), dict(options)))
            return ADAPTIVE.source_work(calls, **options)
        owner = SimpleNamespace(FIELDS=ADAPTIVE.FIELDS, source_work=work)
        value = MODULE.chart_principal_budget_vector(owner, PROJECTION, ENCLOSURE, SELECTION)
        self.assertEqual(len(called), 4)
        for index, (calls, options) in enumerate(called):
            expected = dict.fromkeys(ADAPTIVE.FIELDS, 0)
            expected.update(adaptive_histories=(12, 11, 8, 8)[index],
                            adaptive_observations=(37, 37, 24, 24)[index],
                            adaptive_appends=(37, 37, 24, 24)[index])
            self.assertEqual(calls, expected)
            self.assertEqual(options, {"include_raw_minimum": False, "event_fixture_reached": False})
        for field in ADAPTIVE.FIELDS:
            self.assertEqual(sum(calls[field] for calls, _ in called),
                             value["chart_principal.adaptive_call." + field])
        self.assertEqual(value["chart_principal.adaptive_work.adaptive_minimum_validations_upper"], 39)
        self.assertEqual(value["chart_principal.adaptive_work.raw_event_fixture_entries_upper"], 0)

    def test_preserves_every_1823_field_and_adds_numeric_geometry_once(self):
        old = MODULE.preserve_chart_batch_extension(
            MODULE.preserve_and_extend(old_vector(), budget()), batch_budget())
        frozen, supplement = dict(old), principal_budget()
        combined = MODULE.preserve_chart_principal_extension(old, supplement)
        self.assertEqual(len(old), 1823)
        self.assertEqual(old, frozen)
        self.assertEqual(len(combined), 1823 + MODULE.PRINCIPAL_BUDGET_FIELDS)
        aggregates = {
            "charged_fit_work_upper": 48807, "point_admissions_upper": 1758792,
            "standard_errors.geometry.report_copies_upper": 48807,
            "standard_errors.geometry.logical_bytes_upper": 1561824,
        }
        for key in old:
            self.assertEqual(combined[key], aggregates.get(key, old[key]), key)
        self.assertEqual({key: combined[key] for key in supplement}, supplement)
        for kind in ("collision", "aggregate", "bool", "replacement", "overflow"):
            left, right = dict(old), dict(supplement)
            if kind == "collision":
                left[next(iter(right))] = left.pop("old.owner.0")
            elif kind == "aggregate":
                left["wrong"] = left.pop("charged_fit_work_upper")
            elif kind == "bool":
                right[next(iter(right))] = True
            elif kind == "replacement":
                right["chart_principal.wrong"] = right.pop(next(iter(right)))
            else:
                left["point_admissions_upper"] = (1 << 63) - 1
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                MODULE.preserve_chart_principal_extension(left, right)

    def test_legal_delta_reuses_all_history_and_chart_producers(self):
        value = principal_budget(external_legal=True)
        self.assertEqual(MODULE.chart_principal_numeric_work(value), {
            "charged_fit_work_upper": 0, "point_admissions_upper": 0})
        for key, amount in value.items():
            if key.startswith(tuple("chart_principal." + group for group in (
                    "adaptive_call.", "adaptive_work.", "projection_call.", "projection_work.",
                    "adapter_work.", "owner_work.", "tracking_work.", "roundoff_work.", "doctest."))):
                self.assertEqual(amount, 0, key)
        self.assertEqual(value["chart_principal.enclosure_call.enclosures"], 1)
        self.assertEqual(value["chart_principal.chart_batch_work.constructor_attempts_upper"], 0)
        self.assertEqual(value["chart_principal.chart_batch_work.selection_dispatches_upper"], 1)
        self.assertEqual(value["chart_principal.legacy_batch_work.constructor_attempts_upper"], 1)
        self.assertEqual(value["chart_principal.receiver_work.receiver_garde_validations_upper"], 3)
        self.assertEqual(value["chart_principal.observer.counter_charge_attempts"], 34)
        self.assertEqual(value["chart_principal.observer.direct_product_getters_upper"], 27)
        self.assertEqual(value["chart_principal.observer.record_integer_fields"], 8)
        self.assertEqual(budget(external_legal=True)["chart.legal_observer.counter_charge_attempts"], 52)
        self.assertEqual(batch_budget(external_legal=True)["chart_batch.observer.counter_charge_attempts"], 25)

    def test_three_doctests_have_no_executed_producers(self):
        full, reduced = principal_budget(), principal_budget(include_doctests=False)
        self.assertEqual(MODULE.PRINCIPAL_DOCTEST_LINES, (123, 138, 150))
        for key in full:
            self.assertEqual(reduced[key], 0 if key.startswith("chart_principal.doctest.") else full[key], key)
        self.assertEqual(full["chart_principal.doctest.compilation_cases"], 3)
        self.assertEqual(full["chart_principal.doctest.compile_fail_cases"], 2)
        self.assertEqual(full["chart_principal.doctest.executed_producers"], 0)

    def test_injected_shapes_types_bounds_and_double_funding_reject(self):
        for field in ("external_legal", "include_doctests"):
            for wrong in (None, 0, 1, "true"):
                with self.subTest(field=field, wrong=wrong), self.assertRaises(MODULE.Incomplete):
                    principal_budget(**{field: wrong})
        for change in (lambda value: value.pop("selector_entries_upper"),
                       lambda value: value.update(charged_fit_work_upper=4587),
                       lambda value: value.update(point_admissions_upper=158433),
                       lambda value: value.update(selector_entries_upper=True)):
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_principal_budget_vector(adaptive_changed(change), PROJECTION,
                                                     ENCLOSURE, SELECTION)
        for kind in ("missing", "replacement", "bool", "overflow"):
            def malformed(selections, batches):
                work = SELECTION.source_work_uppers(selections, batches)
                key = "selection_cell_visits_upper"
                if kind == "missing":
                    work.pop(key)
                elif kind == "replacement":
                    work["wrong"] = work.pop(key)
                else:
                    work[key] = True if kind == "bool" else 1 << 63
                return work
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_principal_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE,
                                                     SimpleNamespace(source_work_uppers=malformed))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_principal_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE, SimpleNamespace())
        def double_funded(calls):
            result = ENCLOSURE.source_work_uppers(calls)
            result["charged_fit_work_upper"] = 1
            return result
        owner = SimpleNamespace(FIELDS=ENCLOSURE.FIELDS, source_work_uppers=double_funded)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_principal_budget_vector(ADAPTIVE, PROJECTION, owner, SELECTION)

    def test_source_binding_requires_exact_historical_map_and_current_rejection(self):
        actual = dict(MODULE.PRINCIPAL_SOURCE_SHA256)
        self.assertEqual(MODULE.admit_chart_principal_source_identity(actual), actual)
        current = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest() for name in actual}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_chart_principal_source_identity(current)
        for kind in ("missing", "extra", "stale", "bool", "invalid", "not-map"):
            value = dict(actual)
            key = next(iter(value))
            if kind == "missing":
                value.pop(key)
            elif kind == "extra":
                value["wrong"] = "a" * 64
            elif kind == "stale":
                value[key] = "0" * 64
            elif kind == "bool":
                value[key] = True
            elif kind == "invalid":
                value[key] = "a" * 63
            else:
                value = list(value.items())
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_chart_principal_source_identity(value)
        with mock.patch.object(MODULE, "PRINCIPAL_SOURCE_SHA256", {}), self.assertRaises(MODULE.Incomplete):
            MODULE.admit_chart_principal_source_identity(actual)

    def test_exact_eight_names_receive_only_standalone_complete_success(self):
        self.assertEqual(MODULE.PRINCIPAL_ORDINARY_TESTS, PRINCIPAL_NAMES)
        for text in (PRINCIPAL_COMPLETE, PRINCIPAL_COMPLETE.replace("\n", "\r\n"),
                     PRINCIPAL_COMPLETE.replace("test chart_principal_", "test chart::chart_principal_")):
            actual = MODULE.chart_principal_test_observation(text, require_complete=True)
            self.assertFalse(actual["chart_principal_source_only_pending"])
            self.assertEqual(set(actual["chart_principal_test_statuses"].values()), {"ok"})
            self.assertEqual(actual["chart_principal_recorded_source_work_upper"], {
                "charged_fit_work_upper": 7564, "point_admissions_upper": 261202})
            self.assertIsNone(actual["chart_principal_observed_operation_counts"])

    def test_missing_failed_ignored_malformed_embedded_and_replayed_are_incomplete(self):
        for text in ("", PRINCIPAL_COMPLETE.splitlines(keepends=True)[0], PRINCIPAL_COMPLETE[:-1],
                     PRINCIPAL_COMPLETE.replace(" ... ok\n", " ... FAILED\n", 1),
                     PRINCIPAL_COMPLETE.replace(" ... ok\n", " ... ignored\n", 1),
                     PRINCIPAL_COMPLETE.replace(" ... ok\n", " ... ok extra\n", 1),
                     PRINCIPAL_COMPLETE.replace("test ", "test unrelated::", 1),
                     json.dumps({"message": PRINCIPAL_COMPLETE}) + "\n",
                     "".join("PASS [0.1s] time-core::chart " + name + "\n" for name in PRINCIPAL_NAMES)):
            with self.subTest(text=text[:80]):
                actual = MODULE.chart_principal_test_observation(text)
                self.assertTrue(actual["chart_principal_source_only_pending"])
                self.assertIsNone(actual["chart_principal_recorded_source_work_upper"])
                self.assertIsNone(actual["chart_principal_observed_operation_counts"])
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.chart_principal_test_observation(text, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_principal_test_observation(
                PRINCIPAL_COMPLETE + PRINCIPAL_COMPLETE.splitlines(keepends=True)[0])
        for text in (b"", None, 0, " " * (16 * 1024 * 1024 + 1)):
            with self.subTest(kind=type(text)), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_principal_test_observation(text)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_principal_test_observation(PRINCIPAL_COMPLETE, require_complete=1)


class PriorReceiverAdapterReservationTests(unittest.TestCase):
    """Independent old-call census for newly shared adapter work; no Rust run."""

    def test_older_package_callers_have_named_adapter_allowances(self):
        vector = MODULE.chart_principal_budget_vector(ADAPTIVE, PROJECTION, ENCLOSURE, SELECTION)
        # Source-reviewed roots: principal, combination, public lifecycle,
        # lifecycle guard/bridge, tracking, and empty adaptive-event fixture.
        seeds = 27 + 14 + 31 + 11 + 1 + 15 + 1
        advances = 95 + 15 + 167 + 12 + 1 + 14
        self.assertEqual((seeds, advances), (100, 304))
        expected = {
            'seeds_source_upper': 100, 'advances_source_upper': 304,
            'direct_completions_source_upper': 7,
            'get_original_dispatches_upper': 51824, 'indexed_slice_reads_upper': 51824,
            'projection_getter_requests_upper': 51824, 'len_dispatches_upper': 12935,
            'shared_helper_entries_upper': 919, 'variant_constructions_upper': 715,
        }
        for field, value in expected.items():
            with self.subTest(field=field):
                self.assertEqual(vector.get('chart_principal.prior_adapter.' + field), value)

    def test_older_legal_consumers_fund_four_operations_without_producers(self):
        vector = MODULE.chart_principal_budget_vector(
            ADAPTIVE, PROJECTION, ENCLOSURE, SELECTION, external_legal=True)
        expected = {
            'seeds_source_upper': 2, 'advances_source_upper': 2,
            'direct_completions_source_upper': 0,
            'get_original_dispatches_upper': 512, 'indexed_slice_reads_upper': 512,
            'projection_getter_requests_upper': 512, 'len_dispatches_upper': 128,
            'shared_helper_entries_upper': 6, 'variant_constructions_upper': 6,
        }
        for field, value in expected.items():
            with self.subTest(field=field):
                self.assertEqual(vector.get('chart_principal.prior_adapter.' + field), value)
        self.assertEqual(MODULE.chart_principal_numeric_work(vector), {
            'charged_fit_work_upper': 0, 'point_admissions_upper': 0})


class CompositionReservationTests(unittest.TestCase):
    """Independent current supplement beside explicitly historical source work."""

    @staticmethod
    def basis():
        fixture = load("tests/test_core_chart_combination_accounting")
        combination = fixture.MODULE
        prior = combination.preserve_and_extend(fixture.prior_budget(), combination.full_package_reservation(
            fixture.owners(), prior_budget=fixture.prior_budget()))
        delta = combination.fixture_delta_reservation(fixture.owners(),
            combination.composition_package_fixture_recipe(), ordinary_outcomes=4)
        return prior, delta

    def test_current_width_numeric_and_geometry_increment_only_once(self):
        prior, delta = self.basis()
        supplement = MODULE.composition_package_reservation(prior, delta)
        self.assertEqual(len(supplement), 40)
        self.assertEqual(set(supplement), set(MODULE.COMPOSITION_FIELDS))
        value = MODULE.preserve_composition_extension(prior, delta, supplement)
        self.assertEqual(len(value), 2740)
        self.assertEqual(value["charged_fit_work_upper"], 62571)
        self.assertEqual(value["point_admissions_upper"], 2234094)
        self.assertEqual(value["standard_errors.geometry.report_copies_upper"], 62571)
        self.assertEqual(value["standard_errors.geometry.logical_bytes_upper"], 62571 * 32)
        self.assertEqual(supplement["chart_composition.composition_attempts"], 38)
        self.assertEqual(supplement["chart_composition.composition_point_additions"], 76)
        self.assertEqual(supplement["chart_composition.composition_outward_requests"], 532)
        self.assertEqual(supplement["chart_composition.raw_affine_validations"], 21)
        self.assertEqual(supplement["chart_composition.raw_affine_admissions"], 11)
        aggregate = {"charged_fit_work_upper", "point_admissions_upper",
                     "standard_errors.geometry.report_copies_upper", "standard_errors.geometry.logical_bytes_upper"}
        for key in prior:
            if key not in aggregate:
                self.assertEqual(value[key], prior[key] + delta.get(key, 0))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.preserve_composition_extension(value, delta, supplement)

    def test_all_transforms_reserve_mutated_inherited_error_prefix(self):
        prior, delta = self.basis()
        vector = MODULE.composition_package_reservation(prior, delta)
        transforms = prior["chart_combination.chart_roundoff_work.transform_requests_upper"] + 17
        self.assertEqual(vector["chart_composition.inherited_projection_attempts"], transforms)
        self.assertEqual(vector["chart_composition.inherited_projection_outward_requests"], 8 * transforms)
        self.assertEqual(vector["chart_composition.inherited_projection_condition_checks"], 3 * transforms)
        self.assertEqual(vector["chart_composition.raw_affine_garde_callbacks"], 84)
        self.assertEqual(vector["chart_composition.raw_affine_scalar_predicates"], 128)

    def test_four_outcomes_are_strict_and_do_not_claim_calls(self):
        names = (
            "chart_composition::composition_preserves_dyadic_values_and_rounding_provenance",
            "chart_composition::composition_retains_both_operand_errors_through_signed_elapsed",
            "chart_composition::composition_rejects_relationships_and_ordered_arithmetic_failures",
            "chart_composition_lifecycle::coefficient_error_identity_precedes_batch_principal_and_combination_updates")
        self.assertEqual(MODULE.COMPOSITION_CURRENT_ORDINARY_TESTS, names)
        text = "".join("test " + name + " ... ok\n" for name in names)
        self.assertTrue(MODULE.composition_test_observation(text, require_complete=True)["complete"])
        self.assertTrue(MODULE.composition_test_observation(text.replace("\n", "\r\n"), require_complete=True)["complete"])
        self.assertTrue(MODULE.composition_test_observation(text + "unowned footer without LF", require_complete=True)["complete"])
        for changed in ("", text.replace(" ... ok", " ... FAILED", 1), text.replace(" ... ok", " ... ignored", 1)):
            self.assertFalse(MODULE.composition_test_observation(changed)["complete"])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.composition_test_observation(changed, require_complete=True)
        for changed in (text + text.splitlines()[0] + "\n", text + "test chart_composition::foreign ... ok\n",
                        text + "CORE_CHART_COMPOSITION_CONTRACT_PROFILE labels=10\n", text[:-1]):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.composition_test_observation(changed)

    def test_composition_source_union_stays_historical_and_rate_owns_current_bytes(self):
        expected = MODULE.COMPOSITION_SOURCE_SHA256
        self.assertTrue(set(MODULE.SOURCE_SHA256) | set(MODULE.BATCH_SOURCE_SHA256)
                        | set(MODULE.PRINCIPAL_SOURCE_SHA256) <= set(expected))
        self.assertEqual(MODULE.admit_composition_sources(dict(expected)), expected)
        actual = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest() for name in expected}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_composition_sources(actual)
        current_rate = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest()
                        for name in RATE.SOURCE_SHA256}
        self.assertEqual(RATE.admit_sources(current_rate), current_rate)
        self.assertEqual(actual, {name: RATE.SOURCE_SHA256.get(name, digest)
                                  for name, digest in expected.items()})
        for changed in ({}, {**expected, "foreign": "0" * 64}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_composition_sources(changed)
        key = "crates/time-core/src/regression/chart.rs"
        self.assertNotEqual(expected[key], MODULE.SOURCE_SHA256[key])
        for invalid in (True, "0" * 64):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_composition_sources({**expected, key: invalid})

    def test_four_field_and_two_bit_adapters_are_additive(self):
        prior, delta = self.basis()
        additions = {
            "chart.owner_work.affine_garde_validations_upper": 2,
            "chart_batch.owner_work.affine_garde_validations_upper": 3,
            "chart_principal.owner_work.affine_garde_validations_upper": 4,
            "chart.doctest.charts": 1,
            "chart.guard_private.affine_candidates": 6,
            "chart_batch.raw_header.descriptor_garde_validations_upper": 3,
            "chart_batch.batch_work.constructor_attempts_upper": 1,
            "chart_principal.chart_batch_work.constructor_attempts_upper": 3,
            "chart_batch.batch_work.entry_visits_upper": 2,
            "chart_principal.chart_batch_work.entry_visits_upper": 4,
            "chart_principal.descriptor_work.relationship_passes_upper": 4,
        }
        for key, amount in additions.items():
            prior.pop(next(name for name in prior if name.startswith("old.owner.")))
            prior[key] = amount
        vector = MODULE.composition_package_reservation(prior, delta)
        #32 earlier helper charts+54new, nine other owner charts, one doc and
        #one raw-header descriptor. Six earlier raw literals only validate.
        self.assertEqual(vector["chart_composition.old_affine_additional_callbacks"], 2 * (32 + 54 + 9 + 1 + 1 + 6))
        self.assertEqual(vector["chart_composition.old_affine_additional_scalar_predicates"], 4 * (32 + 54 + 9 + 1 + 1) + 2 * 6)
        self.assertEqual(vector["chart_composition.batch_expected_additional_getters"], 2 * (29 + 4 + 1 + 3))
        self.assertEqual(vector["chart_composition.batch_entries_additional_bit_queries"], 2 * (928 + 128 + 2 + 4))
        self.assertEqual(vector["chart_composition.receiver_additional_scalar_comparisons"], 2 * (82 + 1056 + 24 + 48 + 4))

    def test_raw_header_expectations_add_both_error_getters_and_bits(self):
        prior, delta = self.basis()
        raw = MODULE._batch_raw_header_work(True)
        self.assertEqual(raw["raw_array_visits"], 5)
        # Historical counters retain four descriptor getters/two bit queries
        # per expected(at); the new two coefficient fields belong to the delta.
        self.assertEqual(raw["direct_descriptor_getters_upper"], 20)
        self.assertEqual(raw["direct_float_bit_queries_upper"], 10)
        prior.pop(next(name for name in prior if name.startswith("old.owner.")))
        key = "chart_batch.raw_header.raw_array_visits"
        prior[key] = raw["raw_array_visits"]
        active = MODULE.composition_package_reservation(prior, delta)
        legal_neighbor = dict(prior)
        legal_neighbor[key] = MODULE._batch_raw_header_work(False)["raw_array_visits"]
        inactive = MODULE.composition_package_reservation(legal_neighbor, delta)
        for field in ("batch_expected_additional_getters", "batch_expected_additional_bit_queries"):
            name = "chart_composition." + field
            self.assertEqual(active[name], 2 * (29 + 4 + 5))
            self.assertEqual(inactive[name], 2 * (29 + 4))
            self.assertEqual(active[name] - inactive[name], 10)

    def test_closed_units_overflow_and_prior_namespace(self):
        prior, delta = self.basis()
        for field in MODULE.COMPOSITION_CALL_FIELDS:
            for invalid in (True, -1, MODULE.MAX_UNITS):
                calls = {**MODULE.COMPOSITION_CALLS, field: invalid}
                # The direct observer/read axis has no derived multiplication,
                # so MAX_UNITS itself remains a valid finite bound there.
                if invalid == MODULE.MAX_UNITS and field in {"observer_reads", "private_step_additions"}:
                    continue
                with self.subTest(field=field, invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.composition_mechanism_reservation(prior, delta, calls)
        for changed in ({}, {**MODULE.COMPOSITION_CALLS, "foreign": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.composition_mechanism_reservation(prior, delta, changed)
        for changed in ({}, {**delta, "foreign": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.composition_package_reservation(prior, changed)

    def test_descriptor_only_external_is_exactly_source_bound(self):
        fixture = load("tests/test_core_chart_combination_accounting")
        prior, _ = self.basis()
        delta = fixture.MODULE.fixture_delta_reservation(fixture.owners(),
            fixture.MODULE.composition_external_fixture_recipe(), ordinary_outcomes=1)
        contract = MODULE.CompositionContractOwner(CONTRACTS.COMPOSITION_FIELDS,
            CONTRACTS.composition_legal_expected, CONTRACTS.composition_legal_source)
        vector = MODULE.composition_external_reservation(prior, delta, contract)
        self.assertEqual(sum(CONTRACTS.composition_legal_expected().values()), 55)
        self.assertEqual(vector["chart_composition.composition_point_additions"], 6)
        self.assertEqual(vector["chart_composition.composition_outward_requests"], 42)
        whole = MODULE.preserve_composition_extension(prior, delta, vector)
        self.assertEqual(whole["charged_fit_work_upper"], prior["charged_fit_work_upper"])
        self.assertEqual(whole["point_admissions_upper"], prior["point_admissions_upper"])
        for invalid in (lambda: {**CONTRACTS.composition_legal_expected(), "charts": True},
                        lambda: {**CONTRACTS.composition_legal_expected(), "compositions": 2}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.composition_external_reservation(prior, delta,
                    MODULE.CompositionContractOwner(contract.fields, invalid, contract.source))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.composition_external_reservation(prior, delta, MODULE.CompositionContractOwner(
                contract.fields, contract.expected, lambda: contract.source() + "\n"))

    def test_equal_width_scopes_keep_distinct_numerical_bounds(self):
        fixture = load("tests/test_core_chart_combination_accounting")
        prior, delta = self.basis()
        coverage = dict(prior)
        coverage["point_admissions_upper"] -= 3
        value = MODULE.preserve_composition_extension(coverage, delta,
            MODULE.composition_package_reservation(coverage, delta))
        self.assertEqual((len(value), value["charged_fit_work_upper"], value["point_admissions_upper"]),
                         (2740, 62571, 2234091))
        legal = dict(prior)
        legal.update(charged_fit_work_upper=685, point_admissions_upper=23679)
        legal["standard_errors.geometry.report_copies_upper"] = 685
        legal["standard_errors.geometry.logical_bytes_upper"] = 32 * 685
        descriptor = fixture.MODULE.fixture_delta_reservation(fixture.owners(),
            fixture.MODULE.composition_external_fixture_recipe(), ordinary_outcomes=1)
        contract = MODULE.CompositionContractOwner(CONTRACTS.COMPOSITION_FIELDS,
            CONTRACTS.composition_legal_expected, CONTRACTS.composition_legal_source)
        value = MODULE.preserve_composition_extension(legal, descriptor,
            MODULE.composition_external_reservation(legal, descriptor, contract))
        self.assertEqual((len(value), value["charged_fit_work_upper"], value["point_admissions_upper"]),
                         (2740, 685, 23679))
        recipe = fixture.MODULE._recipe(fixture.MODULE._fixture(
            charts=3, frames_upper=3, domains_upper=1, labels_upper=6, monotonic_upper=3, getters_upper=70))
        observer = fixture.MODULE.fixture_delta_reservation(fixture.owners(), recipe, ordinary_outcomes=1)
        calls = dict(zip(MODULE.COMPOSITION_CALL_FIELDS, (2, 0, 0, 0, 0, 70), strict=True))
        supplement = MODULE.composition_mechanism_reservation(prior, observer, calls)
        value = MODULE.preserve_composition_extension(prior, observer, supplement)
        self.assertEqual((len(value), value["charged_fit_work_upper"], value["point_admissions_upper"]),
                         (2740, 61827, 2208402))
        self.assertEqual(supplement["chart_composition.composition_point_additions"], 4)
        self.assertEqual(supplement["chart_composition.composition_outward_requests"], 28)


class AchievedPhaseReservationTests(unittest.TestCase):
    """Independent additive consumer, constructor and unchanged-candidate controls."""

    @staticmethod
    def basis():
        fixture = load("tests/test_core_chart_combination_accounting")
        combination = fixture.MODULE
        prior, old_delta = CompositionReservationTests.basis()
        qualified = MODULE.preserve_composition_extension(
            prior, old_delta, MODULE.composition_package_reservation(prior, old_delta))
        delta = combination.fixture_delta_reservation(fixture.owners(),
            combination.achieved_phase_package_fixture_recipe(), ordinary_outcomes=5)
        return qualified, delta

    def test_receiver_prefix_funds_failed_admission_and_exact_six_documentation_cases(self):
        vector = MODULE.achieved_phase_package_reservation()
        self.assertEqual(len(vector), 25)
        prefix = MODULE.ACHIEVED_PHASE_PREFIX
        expected = {"receiver_attempts": 38, "garde_validations": 38, "garde_callbacks": 38,
                    "relationship_passes": 76, "readiness_checks": 76, "chart_identity_checks": 76,
                    "mapping_label_comparisons": 152, "domain_field_comparisons": 228,
                    "internal_product_getter_requests_upper": 760,
                    "effect_chart_constructions_upper": 76, "endpoint_frame_constructions_upper": 76,
                    "coordinate_changed_commits_upper": 38, "composition_attempts_upper": 38,
                    "completion_constructions_upper": 38, "replacement_reads": 1,
                    "replacement_moves": 21, "doctest_executed_examples": 0}
        self.assertEqual({key: vector[prefix + key] for key in expected}, expected)
        docs = MODULE.achieved_phase_package_reservation(include_doctests=True)
        changed = {key for key in docs if docs[key] != vector[key]}
        self.assertEqual(changed, {prefix + "additional_doctest_compilations",
                                  prefix + "additional_doctest_compile_fail_cases",
                                  prefix + "additional_uncalled_legal_helpers"})
        self.assertEqual(docs[prefix + "additional_doctest_compilations"], 3)
        self.assertEqual(docs[prefix + "additional_doctest_compile_fail_cases"], 2)
        self.assertEqual(docs[prefix + "additional_uncalled_legal_helpers"], 1)
        # A canonical admission failure may use no constructor or composition;
        # its prospective source recipe keeps the entire weakened-guard suffix.
        calls = dict.fromkeys(MODULE.ACHIEVED_PHASE_CALL_FIELDS, 0)
        calls["receiver_attempts"] = 1
        rejected = MODULE.achieved_phase_mechanism_reservation(calls)
        self.assertEqual(rejected[prefix + "effect_chart_constructions_upper"], 2)
        self.assertEqual(rejected[prefix + "endpoint_frame_constructions_upper"], 2)
        self.assertEqual(rejected[prefix + "composition_attempts_upper"], 1)

    def test_five_fixture_delta_increases_old_composition_and_numeric_totals_once(self):
        prior, delta = self.basis()
        scalar = MODULE.achieved_phase_composition_reservation(prior, delta)
        self.assertEqual(set(scalar), set(MODULE.COMPOSITION_FIELDS))
        self.assertEqual(scalar["chart_composition.composition_attempts"], 38)
        self.assertEqual(scalar["chart_composition.composition_point_additions"], 76)
        self.assertEqual(scalar["chart_composition.composition_outward_requests"], 532)
        self.assertEqual(scalar["chart_composition.old_affine_additional_callbacks"], 168)
        self.assertEqual(scalar["chart_composition.old_affine_additional_scalar_predicates"], 336)
        self.assertEqual(scalar["chart_composition.inherited_projection_attempts"], 9)
        value = MODULE.preserve_achieved_phase_extension(
            prior, delta, scalar, MODULE.achieved_phase_package_reservation())
        self.assertEqual(len(value), 2765)
        self.assertEqual(value["charged_fit_work_upper"], 63129)
        self.assertEqual(value["point_admissions_upper"], 2253363)
        self.assertEqual(value["standard_errors.geometry.report_copies_upper"], 63129)
        self.assertEqual(value["standard_errors.geometry.logical_bytes_upper"], 32 * 63129)
        aggregates = {"charged_fit_work_upper", "point_admissions_upper",
                      "standard_errors.geometry.report_copies_upper", "standard_errors.geometry.logical_bytes_upper"}
        for key in prior:
            if key not in aggregates:
                self.assertEqual(value[key], prior[key] + delta.get(key, 0) + scalar.get(key, 0))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.preserve_achieved_phase_extension(value, delta, scalar, MODULE.achieved_phase_package_reservation())

    def test_missing_axes_boolean_negative_overflow_and_every_positive_undercharge_reject(self):
        full = MODULE.achieved_phase_package_reservation(include_doctests=True)
        self.assertEqual(MODULE.admit_achieved_phase_allowances(full, include_doctests=True), full)
        for field, value in full.items():
            for invalid in (True, -1, value - 1) if value else (True, -1):
                bad = {**full, field: invalid}
                with self.subTest(field=field, invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_achieved_phase_allowances(bad, include_doctests=True)
        for bad in ({}, {**full, "foreign": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admit_achieved_phase_allowances(bad)
        for field in MODULE.ACHIEVED_PHASE_CALL_FIELDS:
            for invalid in (True, -1, 1 << 63):
                with self.subTest(field=field, invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.achieved_phase_mechanism_reservation({**MODULE.ACHIEVED_PHASE_CALLS, field: invalid})
        for bad in ({}, {**MODULE.ACHIEVED_PHASE_CALLS, "foreign": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_mechanism_reservation(bad)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.achieved_phase_mechanism_reservation({**MODULE.ACHIEVED_PHASE_CALLS,
                                                        "receiver_attempts": MODULE.MAX_UNITS})
        for flag in (0, 1, None, "true"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_package_reservation(include_doctests=flag)

    def test_discounted_prior_composition_and_changed_delta_cannot_become_new_baseline(self):
        prior, delta = self.basis()
        scalar = MODULE.achieved_phase_composition_reservation(prior, delta)
        receiver = MODULE.achieved_phase_package_reservation()
        for field, value in scalar.items():
            if value:
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.preserve_achieved_phase_extension(prior, delta, {**scalar, field: value - 1}, receiver)
        for field in MODULE.COMPOSITION_FIELDS:
            if prior[field]:
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.achieved_phase_composition_reservation({**prior, field: prior[field] - 1}, delta)
        for changed in ({}, {**delta, "foreign": 0}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_composition_reservation(prior, changed)
        for changed in ({}, {**receiver, "foreign": 0}, {**receiver, "achieved_phase.receiver_attempts": 37}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.preserve_achieved_phase_extension(prior, delta, scalar, changed)


if __name__ == "__main__":
    unittest.main()
