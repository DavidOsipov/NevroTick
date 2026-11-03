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
    " batches=1 minima=1 selections=1 batch_getters=22\n"
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
            self.assertEqual(sum(value.values()), 77)
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
        actual = {name: hashlib.sha256((SCRIPTS.parent / name).read_bytes()).hexdigest()
                  for name in MODULE.BATCH_SOURCE_INPUTS}
        self.assertEqual(MODULE.admit_chart_batch_source_identity(actual), actual)
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


if __name__ == "__main__":
    unittest.main()
