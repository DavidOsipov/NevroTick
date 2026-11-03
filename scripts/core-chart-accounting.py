"""Pure chart source reservations and separately bound outcome observations.

No I/O, process launch or repository imports occur here. The coordinator supplies
admitted pure accounting and contract owners. These fixed source ceilings are not
measured operations, physical resource claims or a native qualification verdict.
Preserve the earlier 1212-key namespace and add the disjoint chart fields.
"""
from __future__ import annotations

import hashlib
import re
from typing import Protocol, TypeGuard


class Incomplete(ValueError):
    """A source recipe, dependency result or native outcome is not admitted."""


class AdaptiveOwner(Protocol):
    """Injected, separately source-admitted adaptive work owner."""
    FIELDS: tuple[str, ...]

    def source_work(self, calls: object, *, include_raw_minimum: bool = False,
                    event_fixture_reached: bool = False) -> dict[str, int]: ...


class ProjectionOwner(Protocol):
    """Injected, separately source-admitted original projection work owner."""
    FIELDS: tuple[str, ...]

    def source_work_uppers(self, calls: dict[str, int]) -> dict[str, int]: ...

    def projection_adapter_work(self, fixed_requests: int,
                                adaptive_requests: int) -> dict[str, int]: ...


class EnclosureOwner(Protocol):
    """Injected, separately source-admitted exact enclosure work owner."""
    FIELDS: tuple[str, ...]

    def source_work_uppers(self, calls: dict[str, int]) -> dict[str, int]: ...


class ChartContractOwner(Protocol):
    """The single owner of emitted legal source and its exact raw grammar."""

    FIELDS: tuple[str, ...]

    def legal_expected(self) -> dict[str, int]: ...

    def legal_contract_source(self) -> str: ...

    def contract_counts(self, output: object) -> dict[str, int]: ...


class SelectionOwner(Protocol):
    """Injected source-admitted interval batch and majority-loop recurrence."""

    def source_work_uppers(self, evaluator_attempts: int,
                          batch_attempts: int) -> dict[str, int]: ...


STATUS = "prepared-source-recipe-cohort-not-admitted"
INPUTS = ("scripts/core-chart-accounting.py", "scripts/tests/test_core_chart_accounting.py")
MAX_UNITS = (1 << 63) - 1
BUDGET_FIELDS = 296
HISTORICAL_LEGAL_FIELDS = (
    "labels", "contexts", "monotonic", "coordinates", "exchanges",
    "observations", "histories", "appends", "frames", "domains", "charts",
    "prepares", "terms", "tracked_projections", "enclosures", "getters",
)
HISTORICAL_LEGAL_EXPECTED = (5, 2, 4, 1, 1, 3, 1, 3, 1, 1, 1, 4, 1, 1, 1, 22)
HISTORICAL_LEGAL_SOURCE_SHA256 = "1ec1bb10009df768bc670384db93e23cbf0e7da83f6feb9d8b2499d9c2a3ef5d"
# Current legal metadata is bound only after the independent consumer freezes.
LEGAL_FIELDS = (*HISTORICAL_LEGAL_FIELDS, "batches", "minima", "selections", "batch_getters")
LEGAL_EXPECTED = (*HISTORICAL_LEGAL_EXPECTED, 1, 1, 1, 22)
LEGAL_SOURCE_SHA256 = "6f1d16eeaa436ef04e6d513ecb1de1f95e69d75ed66151228d8ddc24196d1bce"
SOURCE_SHA256 = {
    "crates/time-core/tests/private/chart_guards.rs": "6b04885fd9b96baeb7516581d04919cad828fe5d3e429cff364fbddd023034cf",
    "crates/time-core/src/regression/chart.rs": "77fccbecf2b0e020c92b636b2db1478f8bc434526a1cbe85eaa2d9e229798bf8",
    "crates/time-core/src/regression/chart/roundoff.rs": "c79fc05485c0b1f32f3ff80992f45ef8a13c632dc1971c3b60aa96b4f721426f",
    "crates/time-core/src/chart_enclosure.rs": "6785e1caa3274d9f2561735ad611128f1ceded20c739337dd8f70de4e863c0c2",
    "crates/time-core/src/projection_enclosure.rs": "6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023",
    "crates/time-core/tests/chart.rs": "aebcdb16afd6c67184ca8b07e0f59c9feb819713d40f0b0c3cb6a57ab5aa171c",
    "crates/time-core/tests/private/chart_roundoff.rs": "cc2e0cd8a5003fd1d65cb0156f7c26bea33ebc8e182bac00d44e8c65b83de68e",
    "scripts/core-adaptive-accounting.py": "87f135190282b252ebb0059cb67cdc4affa0016b10ab72c3821744bf36fb3c49",
    "scripts/core-projection-accounting.py": "8edac190cc28913f4dbadefd80ac31af1a762537ef38d084fd69bb6ac7d17300",
    "scripts/core-enclosure-accounting.py": "6727ce5b193bb6d2e951221a73cfe393f26a921519841586d461175dc60491ad",
}
PUBLIC_TESTS = (
    "s144_chart_reexpresses_without_mutating_genuine_history",
    "weighted_tracking_uses_one_common_chart_reference_and_genuine_owner",
    "post_prune_auxiliary_row_is_required_and_selected_fit_is_preserved",
    "refusal_unknown_and_context_mismatch_leave_history_unchanged",
    "producer_errors_precede_query_domain_and_preserve_source",
    "full_capacity_and_inclusive_domain_preserve_all_actual_rows",
    "invalid_scalar_and_overflow_leave_real_source_unchanged",
    "chart_descriptor_identity_and_integer_domain_boundaries",
    "negative_elapsed_and_rate_reach_actual_chart_enclosure",
)
ROUNDOFF_TESTS = tuple("regression::chart::roundoff::tests::" + name for name in (
    "upper_operators_preserve_zero_and_underflow_neighbors",
    "upper_operators_reject_invalid_and_exhausted_bounds",
    "value_primitives_preserve_steps_and_reject_underflow",
    "signed_elapsed_preserves_exact_integer_limits",
))
# The actual new cfg(test) hook fixes these six private source identities.
GUARD_TEST_SUFFIXES = (
    "chart_raw_frame_and_domain_carriers",
    "chart_raw_affine_carrier_checks_both_signed_fields",
    "chart_raw_projection_candidate_roles_and_precedence",
    "chart_raw_tracking_candidate_roles_and_precedence",
    "chart_steps_preserve_actual_operation_mapping",
    "chart_errors_preserve_closed_messages_and_typed_sources",
)
GUARD_TESTS = tuple("regression::chart::chart_guards::" + name for name in GUARD_TEST_SUFFIXES)
CURRENT_ORDINARY_TESTS = (*PUBLIC_TESTS, *ROUNDOFF_TESTS, *GUARD_TESTS)
EXPECTED_NUMERIC_WORK = {"charged_fit_work_upper": 5952, "point_admissions_upper": 205536}
EXPECTED_LEGAL_NUMERIC_WORK = {"charged_fit_work_upper": 186, "point_admissions_upper": 6423}

# Upper ceilings, not attempt observations. Frames/domain <=24 deliberately
# cover all public constructor attempts, including their invalid neighbors.
# Snapshot getter allowance:20*(7*64+10)=9160. Other public getter/bit accessor
# requests are conservatively <=1024:46 exact-unit calls,13 subtraction-oracle
# calls,one drift oracle,18 near comparisons and <=17 result inspections.
# S154c adds43 bounded getter/bit inspections inside this unchanged1024 ceiling;
# it adds no constructor, projection, enclosure or bound-arithmetic request.
FIXTURE_CALLS = {
    "histories": 10, "observations": 96, "appends": 96,
    "contexts_upper": 64, "labels_upper": 256, "monotonic_upper": 256,
    "coordinates": 192, "exchanges": 96, "frames_upper": 24,
    "domains_upper": 24, "charts": 24, "prepares": 19,
    "known_prepares_upper": 19, "terms": 17,
    "anchor_projections": 12, "tracking_projections": 5,
    "enclosures": 7, "snapshots": 20, "snapshot_rows_upper": 1280,
    "snapshot_getters_upper": 9160, "other_getters_upper": 1024,
    "exact_units_calls": 46, "subtraction_oracle_calls": 13,
    "nanosecond_oracle_calls": 1, "near_comparisons": 18,
    "debug_format_requests_upper": 7,
}

# Current four private arithmetic cases:10 zero/positive-neighbor bound calls,
# 29 invalid/exhausted bound calls,plus2 value-versus-bound neighbors=41.
# Their six elapsed attempts include four accepted integer conversions and two
# range rejections; retained fixed-source work, not an arbitrary-mutant bound.
ROUNDOFF_PRIVATE_WORK = {
    "bound_requests_upper": 41,
    "bound_add_requests_upper": 13,
    "bound_multiply_requests_upper": 14,
    "bound_divide_requests_upper": 14,
    "bound_binary_primitives_upper": 12,
    "bound_next_up_requests_upper": 4,
    "direct_value_requests_upper": 9,
    "elapsed_requests_upper": 6,
    "integer_conversion_requests_upper": 6,
    "integer_shared_primitives_upper": 8,
    "elapsed_value_divides_upper": 4,
    "total_shared_value_primitives_upper": 21,
    "monotonic_constructions_upper": 12,
    "monotonic_getters_upper": 12,
    "step_operation_mappings_upper": 13,
    "bound_operand_and_result_predicates_upper": 492,
}

# New tests are raw candidates/public error values, not forged checked
# histories or projections. They add zero observation, append, fit or point work.
GUARD_WORK = {
    "labels": 4, "monotonic_constructors": 18, "monotonic_getters_upper": 24,
    "frame_candidates": 4, "domain_candidates": 6, "affine_candidates": 6,
    "projection_candidates": 13, "tracking_candidates": 9,
    "garde_validations_upper": 60, "garde_callbacks_upper": 162,
    "predicate_requests_upper": 252, "step_mappings": 6,
    "display_requests": 12, "error_source_requests": 12,
    "debug_requests": 8, "typed_downcasts": 3,
}

# The source-bound prospective external consumer's own literal call vector.
# Only its single Known preparation reaches the chart relationship validator.
LEGAL_FIXTURE_CALLS = {
    "histories": 1, "observations": 3, "appends": 3,
    "contexts_upper": 2, "labels_upper": 5, "monotonic_upper": 4,
    "coordinates": 1, "exchanges": 1, "frames_upper": 1,
    "domains_upper": 1, "charts": 1, "prepares": 4,
    "known_prepares_upper": 1, "terms": 1,
    "anchor_projections": 0, "tracking_projections": 1,
    "enclosures": 1, "snapshots": 0, "snapshot_rows_upper": 0,
    "snapshot_getters_upper": 0, "other_getters_upper": 22,
    "exact_units_calls": 0, "subtraction_oracle_calls": 0,
    "nanosecond_oracle_calls": 0, "near_comparisons": 0,
    "debug_format_requests_upper": 0,
}



# Exact interfaces of the independently pinned injected accounting owners.
ADAPTIVE_FIELDS = ('retained_points',
 'retained_inputs',
 'retained_selections',
 'adaptive_observations',
 'adaptive_appends',
 'adaptive_histories',
 'raw_parameter_validations',
 'raw_ordinal_validations',
 'raw_event_validations',
 'external_normalizations')
ADAPTIVE_WORK_FIELDS = ('charged_fit_work_upper',
 'point_admissions_upper',
 'retained_direct_points_upper',
 'retained_direct_inputs_upper',
 'selector_entries_upper',
 'selector_fit_upper',
 'selector_point_copy_upper',
 'selector_input_slots_upper',
 'selector_residual_slots_upper',
 'selector_decision_residuals_upper',
 'selector_checked_float_upper',
 'adaptive_normalizations_upper',
 'adaptive_generated_points_upper',
 'adaptive_duration_ops_upper',
 'adaptive_fit_inputs_upper',
 'adaptive_normalized_input_slots_upper',
 'retained_direct_input_slots_upper',
 'direct_and_generated_point_validations_upper',
 'direct_and_generated_point_rule_callbacks_upper',
 'adaptive_stage_init_slots_upper',
 'adaptive_stage_retained_copies_upper',
 'adaptive_compaction_copies_upper',
 'adaptive_array_transfer_slots_upper',
 'adaptive_complete_context_checks_upper',
 'adaptive_complete_order_checks_upper',
 'adaptive_observation_validations_upper',
 'adaptive_observation_rule_callbacks_upper',
 'adaptive_minimum_validations_upper',
 'adaptive_minimum_rule_callbacks_upper',
 'adaptive_minimum_predicates_upper',
 'adaptive_error_display_requests_upper',
 'adaptive_error_nested_display_calls_upper',
 'adaptive_error_source_requests_upper',
 'adaptive_error_source_downcasts_upper',
 'adaptive_private_prefix_calls_upper',
 'adaptive_private_final_validator_calls_upper',
 'adaptive_private_context_constructions_upper',
 'adaptive_private_observation_constructions_upper',
 'adaptive_private_append_calls_upper',
 'adaptive_private_history_constructions_upper',
 'adaptive_private_monotonic_constructions_upper',
 'adaptive_private_utc_constructions_upper',
 'adaptive_private_exchange_calls_upper',
 'adaptive_private_getter_calls_upper',
 'adaptive_private_context_checks_upper',
 'adaptive_private_order_checks_upper',
 'adaptive_private_array_init_slots_upper',
 'adaptive_private_array_row_writes_upper',
 'adaptive_private_array_transfer_slots_upper',
 'raw_lifecycle_validations_upper',
 'raw_lifecycle_rule_callbacks_upper',
 'raw_event_fixture_entries_upper',
 'raw_event_projected_batches_upper',
 'raw_event_principal_parameters_upper',
 'raw_event_combination_parameters_upper',
 'raw_event_minima_upper',
 'raw_event_seeds_upper',
 'raw_event_update_constructions_upper',
 'raw_event_record_calls_upper',
 'raw_event_source_state_constructions_upper',
 'raw_event_owner_admission_predicate_calls_upper',
 'raw_event_lifecycle_garde_validations_upper',
 'raw_event_lifecycle_garde_callbacks_upper',
 'raw_event_state_clones_upper',
 'raw_event_state_equalities_upper',
 'raw_event_source_lookups_upper')
PROJECTION_FIELDS = ('histories',
 'appends',
 'observations',
 'contexts',
 'monotonic',
 'coordinates',
 'exchanges',
 'getters',
 'terms',
 'projections')
PROJECTION_WORK_FIELDS = ('charged_fit_work_upper',
 'point_admissions_upper',
 'history_normalization_entries_upper',
 'history_statistics_entries_upper',
 'history_statistics_inner_fit_upper',
 'history_duration_operations_upper',
 'history_generated_point_admissions_upper',
 'history_fit_input_constructions_upper',
 'history_staging_Copy_assignments_upper',
 'projection_duration_operations_upper',
 'projection_internal_product_getter_requests_upper',
 'projection_raw_candidate_constructions_upper',
 'projection_candidate_validation_calls_upper',
 'projection_first_error_calls_upper',
 'projection_term_garde_rule_callbacks_upper',
 'projection_term_predicate_calls_upper',
 'projection_checked_integer_helper_calls_upper',
 'projection_normalization_primitive_calls_upper',
 'projection_arithmetic_primitive_calls_upper')
ENCLOSURE_FIELDS = ('histories',
 'appends',
 'observations',
 'contexts',
 'monotonic',
 'coordinates',
 'exchanges',
 'getters',
 'terms',
 'projections',
 'enclosures')
ENCLOSURE_WORK_FIELDS = ('charged_fit_work_upper',
 'point_admissions_upper',
 'history_normalization_entries_upper',
 'history_statistics_entries_upper',
 'history_statistics_inner_fit_upper',
 'history_duration_operations_upper',
 'history_generated_point_admissions_upper',
 'history_fit_input_constructions_upper',
 'history_staging_Copy_assignments_upper',
 'projection_duration_operations_upper',
 'projection_internal_product_getter_requests_upper',
 'projection_raw_candidate_constructions_upper',
 'projection_candidate_validation_calls_upper',
 'projection_first_error_calls_upper',
 'projection_term_garde_rule_callbacks_upper',
 'projection_term_predicate_calls_upper',
 'projection_checked_integer_helper_calls_upper',
 'projection_normalization_primitive_calls_upper',
 'projection_arithmetic_primitive_calls_upper',
 'enclosure_bridge_entry_calls_upper',
 'enclosure_wrapper_debug_format_calls_upper',
 'enclosure_projection_getter_chain_calls_upper',
 'enclosure_raw_helper_entries_upper',
 'enclosure_raw_garde_validate_calls_upper',
 'enclosure_raw_garde_rule_callbacks_upper',
 'enclosure_checked_scalar_primitive_calls_upper',
 'enclosure_interval_constructor_attempts_upper',
 'enclosure_interval_garde_validate_calls_upper',
 'enclosure_interval_rule_calls_upper')

def _checked(values: object, width: int) -> dict[str, int]:
    """Require exact finite unit count; never infer missing work as observed zero."""
    if (type(values) is not dict or len(values) != width
            or any(type(key) is not str or type(value) is not int
                   or not 0 <= value <= MAX_UNITS for key, value in values.items())):
        raise Incomplete("Chart source-work interface changed or exceeded its finite bound.")
    return dict(values)


def _fields(actual: object, expected: tuple[str, ...]) -> None:
    """Reject a reordered, missing, duplicate or changed injected call schema."""
    if type(actual) is not tuple or actual != expected:
        raise Incomplete("Chart injected owner fields differ from the frozen recipe.")


def _owner_fields(owner: AdaptiveOwner | ProjectionOwner | EnclosureOwner | ChartContractOwner,
                  expected: tuple[str, ...]) -> None:
    """Read a declared Protocol field while rejecting an absent runtime interface."""
    try:
        actual: object = owner.FIELDS
    except AttributeError as error:
        raise Incomplete("Chart injected owner fields differ from the frozen recipe.") from error
    _fields(actual, expected)


def _is_exact_dict(value: object) -> TypeGuard[dict[object, object]]:
    """Contain unknown dictionary keys and values as objects before admission."""
    return type(value) is dict


def _work(actual: object, expected: tuple[str, ...]) -> dict[str, int]:
    """Require every exact nonnegative source-work key, with no replacement key."""
    checked = _checked(actual, len(expected))
    if set(checked) != set(expected):
        raise Incomplete("Chart injected source-work keys differ from the frozen recipe.")
    return checked


def admit_source_identity(actual: object) -> dict[str, str]:
    """Check a caller-supplied exact digest map without reading a filesystem.

    Complete product/tool membership and hashing remain the coordinator's duty.
    This narrower map cannot substitute for that admission or authorize effects.
    """
    if not _is_exact_dict(actual) or set(actual) != set(SOURCE_SHA256):
        raise Incomplete("Chart source identity is missing, stale or outside its exact scope.")
    result: dict[str, str] = {}
    for name, digest in actual.items():
        if (type(name) is not str or type(digest) is not str
                or re.fullmatch(r"[0-9a-f]{64}", digest) is None
                or digest != SOURCE_SHA256[name]):
            raise Incomplete("Chart source identity is missing, stale or outside its exact scope.")
        result[name] = digest
    return result


def admit_legal_metadata(fields: object, calls: object, source_sha256: object) -> dict[str, int]:
    """Bind the actual consumer's exact fields, literal calls and emitted bytes."""
    _fields(fields, LEGAL_FIELDS)
    values = _work(calls, LEGAL_FIELDS)
    if (values != dict(zip(LEGAL_FIELDS, LEGAL_EXPECTED, strict=True))
            or type(source_sha256) is not str
            or re.fullmatch(r"[0-9a-f]{64}", source_sha256) is None
            or source_sha256 != LEGAL_SOURCE_SHA256):
        raise Incomplete("Chart legal source or literal call recipe differs from its bound bytes.")
    return values


def chart_legal_observation(output: object, contracts: ChartContractOwner) -> dict[str, int]:
    """Delegate raw legal parsing once after exact source and metadata admission.

    Only returned direct attempts are observations. The source-work vector is a
    separate reservation; ordinary named outcomes never manufacture this row.
    """
    _owner_fields(contracts, LEGAL_FIELDS)
    source = contracts.legal_contract_source()
    if type(source) is not str or not 0 < len(source) <= 65536:
        raise Incomplete("Chart emitted legal source is not bounded text.")
    expected = admit_legal_metadata(
        contracts.FIELDS, contracts.legal_expected(), hashlib.sha256(source.encode()).hexdigest())
    try:
        observed = contracts.contract_counts(output)
    except ValueError as error:
        raise Incomplete("Chart legal raw record was rejected by its contract owner.") from error
    if _work(observed, LEGAL_FIELDS) != expected:
        raise Incomplete("Chart legal observed calls do not match the exact completed consumer.")
    return dict(observed)


def _owner_work(fixture: dict[str, int]) -> dict[str, int]:
    """Bound six mandatory chart carriers and actual immutable owner traversal."""
    frames, domains, charts = (fixture[key] for key in ("frames_upper", "domains_upper", "charts"))
    prepares = fixture["prepares"]
    known = fixture["known_prepares_upper"]
    tracked = fixture["tracking_projections"]
    projections = fixture["anchor_projections"] + tracked
    enclosures = fixture["enclosures"]
    return {
        "frame_garde_validations_upper": frames,
        "frame_rule_callbacks_upper": frames,
        "frame_relationship_predicates_upper": 2 * frames,
        "domain_garde_validations_upper": domains,
        "domain_rule_callbacks_upper": domains,
        "domain_relationship_predicates_upper": 2 * domains,
        "domain_integer_getters_upper": 4 * domains,
        "affine_garde_validations_upper": charts,
        "affine_rule_callbacks_upper": 2 * charts,
        "affine_scalar_predicates_upper": 4 * charts,
        "history_garde_validations_upper": known,
        "history_rule_callbacks_upper": known,
        "history_relationship_passes_upper": 2 * known,
        "history_row_visits_upper": 128 * known,
        "history_domain_comparisons_upper": 260 * known,
        "history_getters_upper": prepares + 270 * known,
        "projection_garde_validations_upper": projections,
        "projection_rule_callbacks_upper": 4 * projections,
        "projection_scalar_predicates_upper": 8 * projections,
        "tracking_garde_validations_upper": tracked,
        "tracking_rule_callbacks_upper": 2 * tracked,
        "tracking_scalar_predicates_upper": 4 * tracked,
        "project_dispatch_entries_upper": projections,
        "target_context_constructions_upper": projections,
        "project_parent_getters_upper": 6 * projections,
        "query_domain_comparisons_upper": 4 * projections,
        "original_projection_retention_upper": projections,
        "tracking_payload_retention_upper": tracked,
        "enclosure_outer_entries_upper": enclosures,
        "enclosure_radius_bridge_entries_upper": enclosures,
        "enclosure_context_extra_getters_upper": 2 * enclosures,
        "enclosure_context_constructions_upper": enclosures,
        "enclosure_projection_copies_upper": enclosures,
    }


def _tracking_work(tracked: int) -> dict[str, int]:
    """Fund actual old fitted tracking beyond its shared anchor producer once."""
    return {
        "standard_error_requests_upper": tracked,
        "standard_error_binary_primitives_upper": 8 * tracked,
        "standard_error_square_roots_upper": 2 * tracked,
        "standard_error_scalar_constructions_upper": 2 * tracked,
        "standard_error_garde_validations_upper": 2 * tracked,
        "standard_error_rule_callbacks_upper": 2 * tracked,
        "standard_error_predicates_upper": 4 * tracked,
        "reference_duration_operations_upper": tracked,
        "reference_duration_getters_upper": tracked,
        "reference_integer_conversions_upper": tracked,
        "reference_conversion_shared_primitives_upper": 3 * tracked,
        "numeric_getters_upper": 4 * tracked,
        "tracking_binary_primitives_upper": 4 * tracked,
        "skew_validations_upper": tracked,
        "final_validations_upper": tracked,
        "rule_callbacks_upper": 5 * tracked,
        "scalar_predicates_upper": 10 * tracked,
    }


def _roundoff_work(fixture: dict[str, int]) -> dict[str, int]:
    """Bound child primitives; each outward operation has at most one next_up."""
    anchor, tracked = fixture["anchor_projections"], fixture["tracking_projections"]
    projections = anchor + tracked
    # Each transform: displacement12A/7M/1D; each center6A/2M;
    # rate4A/1M. Tracking adds one more center. Enclosure adds one bound Add.
    add = 22 * anchor + 28 * tracked + fixture["enclosures"]
    multiply = 10 * anchor + 12 * tracked
    divide = projections
    bound = add + multiply + divide
    return {
        "transform_requests_upper": projections,
        "elapsed_requests_upper": projections,
        "monotonic_getters_upper": 2 * projections,
        "original_projection_getters_upper": 4 * anchor + 5 * tracked,
        "integer_conversion_requests_upper": projections,
        "integer_shared_primitives_upper": 2 * projections,
        "value_primitives_upper": 5 * anchor + 6 * tracked,
        "total_shared_value_primitives_upper": 7 * anchor + 8 * tracked,
        "step_operation_mappings_upper": 5 * anchor + 6 * tracked,
        "bound_add_requests_upper": add,
        "bound_multiply_requests_upper": multiply,
        "bound_divide_requests_upper": divide,
        "bound_requests_upper": bound,
        "bound_binary_primitives_upper": bound,
        "bound_next_up_requests_upper": bound,
        "bound_operand_and_result_predicates_upper": 12 * bound,
    }


def _doctest_work(enabled: bool) -> dict[str, int]:
    """Six new compile cases, but only one executes three checked constructors."""
    factor = int(enabled)
    return {key: factor * value for key, value in {
        "compilation_cases": 6, "compile_fail_cases": 4,
        "executed_descriptor_examples": 1, "uncalled_legal_helpers": 1,
        "labels": 4, "monotonic": 3, "frames": 1, "domains": 1, "charts": 1,
        "getters": 2, "garde_validations_upper": 3,
        "garde_callbacks_upper": 4, "relationship_predicates_upper": 4,
        "domain_integer_getters_upper": 4, "scalar_predicates_upper": 4,
        "fit_or_point_work": 0,
    }.items()}


def _legal_observer_work(enabled: bool) -> dict[str, int]:
    """Fund the prospective exact52-call observer separately from product work."""
    factor = int(enabled)
    return {key: factor * value for key, value in {
        "counter_charge_attempts": 52, "indexed_counter_accesses_upper": 104,
        "checked_additions_upper": 52, "cap_comparisons_upper": 52,
        "counter_slot_writes_upper": 52, "finish_equality_elements_upper": 16,
        "record_rows": 2, "record_integer_fields": 16,
        "assertion_predicates_upper": 64,
    }.items()}


def chart_budget_vector(adaptive: AdaptiveOwner, projection: ProjectionOwner,
                        enclosure: EnclosureOwner, *, external_legal: bool = False,
                        include_doctests: bool = True,
                        include_guard_tests: bool = True) -> dict[str, int]:
    """Build disjoint source reservations, never native-call observations.

    External execution uses its literal separate recipe, no package guards/docs.
    Neither the package nor legal reservation admits a native campaign.
    The coordinator must admit source identity before native effects.
    """
    if any(type(flag) is not bool for flag in (external_legal, include_doctests, include_guard_tests)):
        raise Incomplete("Chart recipe requires exact Boolean scope selectors.")
    _owner_fields(adaptive, ADAPTIVE_FIELDS)
    _owner_fields(projection, PROJECTION_FIELDS)
    _owner_fields(enclosure, ENCLOSURE_FIELDS)
    fixture = dict(LEGAL_FIXTURE_CALLS if external_legal else FIXTURE_CALLS)
    adaptive_calls = dict.fromkeys(adaptive.FIELDS, 0)
    adaptive_calls.update(adaptive_histories=fixture["histories"],
                          adaptive_observations=fixture["observations"],
                          adaptive_appends=fixture["appends"])
    adaptive_work = _work(adaptive.source_work(adaptive_calls, include_raw_minimum=False),
                          ADAPTIVE_WORK_FIELDS)
    expected = EXPECTED_LEGAL_NUMERIC_WORK if external_legal else EXPECTED_NUMERIC_WORK
    if {key: adaptive_work[key] for key in expected} != expected:
        raise Incomplete("The real adaptive recurrence differs from the reviewed numerical upper.")
    projection_calls = dict.fromkeys(projection.FIELDS, 0)
    projection_calls.update(terms=fixture["terms"],
                            projections=fixture["anchor_projections"] + fixture["tracking_projections"])
    projection_work = _work(projection.source_work_uppers(projection_calls), PROJECTION_WORK_FIELDS)
    enclosure_calls = dict.fromkeys(enclosure.FIELDS, 0)
    enclosure_calls.update(enclosures=fixture["enclosures"])
    enclosure_work = _work(enclosure.source_work_uppers(enclosure_calls), ENCLOSURE_WORK_FIELDS)
    if any(work[key] for work in (projection_work, enclosure_work) for key in expected):
        raise Incomplete("Legacy projection/enclosure recipe double-funds adaptive fitting.")
    adapter = _work(projection.projection_adapter_work(0, projection_calls["projections"]), (
        "projection_capsule_and_kernel_entries_upper", "projection_adaptive_extra_internal_getters_upper"))
    private = {key: 0 if external_legal else value for key, value in ROUNDOFF_PRIVATE_WORK.items()}
    guards = {key: value if include_guard_tests and not external_legal else 0
              for key, value in GUARD_WORK.items()}
    groups = (
        ("fixture.", fixture), ("adaptive_call.", adaptive_calls),
        ("adaptive_work.", adaptive_work), ("projection_call.", projection_calls),
        ("projection_work.", projection_work), ("adapter_work.", adapter),
        ("enclosure_call.", enclosure_calls), ("enclosure_work.", enclosure_work),
        ("owner_work.", _owner_work(fixture)),
        ("tracking_work.", _tracking_work(fixture["tracking_projections"])),
        ("roundoff_work.", _roundoff_work(fixture)),
        ("roundoff_private.", private), ("guard_private.", guards),
        ("doctest.", _doctest_work(include_doctests and not external_legal)),
        ("legal_observer.", _legal_observer_work(external_legal)),
    )
    result: dict[str, int] = {}
    for prefix, values in groups:
        for key, value in _checked(values, len(values)).items():
            name = "chart." + prefix + key
            if name in result:
                raise Incomplete("Chart source namespaces overlap.")
            result[name] = value
    return _checked(result, BUDGET_FIELDS)


def chart_numeric_work(vector: dict[str, int]) -> dict[str, int]:
    """Select the one funded producer contribution for existing global scalars."""
    _checked(vector, BUDGET_FIELDS)
    if any("chart.adaptive_work." + key not in vector for key in EXPECTED_NUMERIC_WORK):
        raise Incomplete("Chart numeric source keys are missing.")
    return {key: vector["chart.adaptive_work." + key] for key in EXPECTED_NUMERIC_WORK}


def preserve_and_extend(old: dict[str, int], chart: dict[str, int]) -> dict[str, int]:
    """Keep old namespaces and add the new numeric work once to scalar totals.

    Geometry-copy fields are existing per-report ownership, so increase those
    by the new charged-fit ceiling, rather than add a second chart copy owner.
    Integration may instead let the existing standard_error_budget_vector
    derive these two aggregate fields from the new package_charged_upper.
    """
    _checked(old, 1212)
    _checked(chart, BUDGET_FIELDS)
    aggregate = (*EXPECTED_NUMERIC_WORK, "standard_errors.geometry.report_copies_upper",
                 "standard_errors.geometry.logical_bytes_upper")
    if (set(old) & set(chart) or any(not key.startswith("chart.") for key in chart)
            or any(key not in old for key in aggregate)):
        raise Incomplete("Chart extension requires the earlier 1212-key namespace.")
    result = {**old, **chart}
    extra = chart_numeric_work(chart)
    for key, amount in extra.items():
        result[key] += amount
    result["standard_errors.geometry.report_copies_upper"] += extra["charged_fit_work_upper"]
    result["standard_errors.geometry.logical_bytes_upper"] += 32 * extra["charged_fit_work_upper"]
    return _checked(result, 1212 + BUDGET_FIELDS)


def chart_test_observation(output: str, *, require_complete: bool = False) -> dict[str, object]:
    """Separate exact named outcomes from source upper bounds; no attempt marker.

    Original six doctest identities must be added by the coordinator after final
    source freeze, using actual baseline names. Mutants may move source line
    numbers; require full doctest totals there, not stale original line labels.
    Caught prefixes retain full reservation and pending—not zero—observation.
    """
    if type(output) is not str or len(output) > 16 * 1024 * 1024 or type(require_complete) is not bool:
        raise Incomplete("Chart observation has an invalid type or output bound.")
    names = CURRENT_ORDINARY_TESTS
    observations: dict[str, list[str]] = {name: [] for name in names}
    for line in output.splitlines(keepends=True):
        if not line.startswith("test "):
            continue
        for name in names:
            if name not in line:
                continue
            prefix = r"(?:chart::)?" if name in PUBLIC_TESTS else ""
            matched = re.fullmatch(r"test " + prefix + re.escape(name) + r" \.\.\. (ok|FAILED|ignored)\r?\n", line)
            observations[name].append(matched.group(1) if matched else "malformed")
    if any(len(values) > 1 for values in observations.values()):
        raise Incomplete("Chart named outcome is duplicated or replayed.")
    statuses = {name: values[0] if values else "missing" for name, values in observations.items()}
    complete = all(value == "ok" for value in statuses.values())
    if require_complete and not complete:
        raise Incomplete("Complete package lacks all source-bound chart test outcomes.")
    return {"chart_test_statuses": statuses, "chart_source_only_pending": not complete,
            "chart_recorded_source_work_upper": dict(EXPECTED_NUMERIC_WORK) if complete else None,
            "chart_observed_operation_counts": None}


# S155 is additive: none of the preceding 296 chart fields or the earlier
# 1212 fields changes meaning. The coordinator retains old raw receipts and
# independently binds the new complete product/tool candidate before execution.
BATCH_STATUS = "prepared-source-recipe-cohort-not-admitted"
BATCH_BUDGET_FIELDS = 315
BATCH_SOURCE_INPUTS = (
    *SOURCE_SHA256,
    "crates/time-core/src/chart_batch.rs",
    "crates/time-core/tests/private/chart_batch_cases.rs",
    "crates/time-core/tests/private/chart_batch_guards.rs",
    "scripts/core-selection-accounting.py",
)
BATCH_SOURCE_SHA256: dict[str, str] = {
    "crates/time-core/tests/private/chart_guards.rs": "6b04885fd9b96baeb7516581d04919cad828fe5d3e429cff364fbddd023034cf",
    "crates/time-core/src/regression/chart.rs": "77fccbecf2b0e020c92b636b2db1478f8bc434526a1cbe85eaa2d9e229798bf8",
    "crates/time-core/src/regression/chart/roundoff.rs": "c79fc05485c0b1f32f3ff80992f45ef8a13c632dc1971c3b60aa96b4f721426f",
    "crates/time-core/src/chart_enclosure.rs": "6785e1caa3274d9f2561735ad611128f1ceded20c739337dd8f70de4e863c0c2",
    "crates/time-core/src/projection_enclosure.rs": "6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023",
    "crates/time-core/tests/chart.rs": "aebcdb16afd6c67184ca8b07e0f59c9feb819713d40f0b0c3cb6a57ab5aa171c",
    "crates/time-core/tests/private/chart_roundoff.rs": "cc2e0cd8a5003fd1d65cb0156f7c26bea33ebc8e182bac00d44e8c65b83de68e",
    "scripts/core-adaptive-accounting.py": "87f135190282b252ebb0059cb67cdc4affa0016b10ab72c3821744bf36fb3c49",
    "scripts/core-projection-accounting.py": "8edac190cc28913f4dbadefd80ac31af1a762537ef38d084fd69bb6ac7d17300",
    "scripts/core-enclosure-accounting.py": "6727ce5b193bb6d2e951221a73cfe393f26a921519841586d461175dc60491ad",
    "crates/time-core/src/chart_batch.rs": "ee6d9d5d63c87573cb1f8b55169f59dd92ac67438a26a3b565580e7c22160873",
    "crates/time-core/tests/private/chart_batch_cases.rs": "6d7638728ee09df57522771ec76465603a9a2bb46f44d754ba35f6f0d4c905d1",
    "crates/time-core/tests/private/chart_batch_guards.rs": "8c16f05d4155a2653100f6cc9392ff78bc0f931fe2eb12d6710cbd9ff4e1591f",
    "scripts/core-selection-accounting.py": "d9cba324bcc60102e93ade48fdba0a2810e5f3bfd8cda034159a3debe7b2fa83"
}
BATCH_PUBLIC_TESTS = tuple("chart_batch_cases::" + name for name in (
    "chart_batch_selects_genuine_majority_at_common_time",
    "chart_batch_source_loss_does_not_lower_minimum",
    "chart_batch_empty_and_capacity_boundaries_use_real_owners",
    "chart_batch_compares_every_chart_frame_field",
    "chart_batch_orders_domain_scalar_time_and_entry_mismatches",
    "chart_batch_preserves_real_duplicate_cause_and_redaction",
    "chart_batch_equal_descriptor_values_preserve_pointers_and_zero_bits",
))
# Final raw-case spelling is reconciled with its actual frozen owner, not a
# synthetic profile row. The original five doctest names belong to the baseline
# receiver; mutated line numbers never become mandatory original positions.
BATCH_RAW_TESTS = ("chart_batch::tests::raw_chart_batch_header_checks_inclusive_domain",)
BATCH_ORDINARY_TESTS = (*BATCH_PUBLIC_TESTS, *BATCH_RAW_TESTS)
BATCH_EXPECTED_NUMERIC_WORK = {"charged_fit_work_upper": 5952, "point_admissions_upper": 205536}
BATCH_SELECTION_WORK_FIELDS = (
    "selection_endpoint_collection_writes_upper",
    "selection_endpoint_sort_comparisons_upper",
    "selection_endpoint_dedup_comparisons_upper",
    "selection_cell_visits_upper", "selection_membership_probes_upper",
    "selection_component_visits_upper", "selection_witness_probes_upper",
    "selection_label_sort_comparisons_upper", "selection_intersection_reductions_upper",
    "selection_batch_copy_adapter_attempts_upper", "selection_batch_label_comparisons_upper",
)

# Every public interval() takes the tracking path. All snapshots retain three
# genuine rows:64*(7*3+10) getter requests. The separate4096 ceiling covers direct
# observations, not hidden history traversal. Context128 >=32 owners+46 targets;
# labels256 >=3*32+4*22; monotonic512 covers96 rows+66 map instants+46 queries+
# 44 batch headers and all explicit observer neighbors. These are source upper
# bounds, not counters; failures retain the full prospective reservation.
BATCH_FIXTURE_CALLS = {
    "histories": 32, "observations": 96, "appends": 96,
    "contexts_upper": 128, "labels_upper": 256, "monotonic_upper": 512,
    "coordinates": 192, "exchanges": 96, "frames_upper": 22,
    "domains_upper": 22, "charts": 22, "prepares": 46,
    "known_prepares_upper": 46, "terms": 46,
    "anchor_projections": 0, "tracking_projections": 46,
    "enclosures": 46, "snapshots": 64, "snapshot_rows_upper": 192,
    "snapshot_getters_upper": 1984, "other_getters_upper": 4096,
    "exact_units_calls": 0, "subtraction_oracle_calls": 0,
    "nanosecond_oracle_calls": 0, "near_comparisons": 0,
    "debug_format_requests_upper": 64,
}


def admit_chart_batch_source_identity(actual: object) -> dict[str, str]:
    """Admit exactly the final reviewed source map, without I/O or re-pinning.

    An empty/unbound reviewed map always rejects. The coordinator supplies final
    hashes only after writer release and separately verifies complete membership.
    """
    if (set(BATCH_SOURCE_SHA256) != set(BATCH_SOURCE_INPUTS)
            or not _is_exact_dict(actual) or set(actual) != set(BATCH_SOURCE_INPUTS)):
        raise Incomplete("Chart-batch final source binding is pending or incomplete.")
    result: dict[str, str] = {}
    for name, digest in actual.items():
        if (type(name) is not str or type(digest) is not str
                or re.fullmatch(r"[0-9a-f]{64}", digest) is None
                or digest != BATCH_SOURCE_SHA256[name]):
            raise Incomplete("Chart-batch source identity is stale or outside its exact scope.")
        result[name] = digest
    return result


def _batch_work(batches: int, selections: int, minima: int, *, raw_header: bool) -> dict[str, int]:
    """Bound actual two-pass admission and existing selector receiving work.

    Two extra raw-test public constructors have empty, out-of-domain inputs.
    Their relationship checks are funded; they cannot reach downstream admission.
    The five direct raw validations also have empty slices. Other attempts use
    the public16-entry capacity envelope, including rejected-prefix cases.
    """
    raw = int(raw_header)
    constructors = batches + 2 * raw
    passes = 2 * constructors + 5 * raw
    visits = 2 * 16 * batches
    return {
        "constructor_attempts_upper": constructors,
        "initial_capacity_checks_upper": constructors,
        "expected_descriptor_getters_upper": 4 * constructors,
        "expected_float_bit_queries_upper": 2 * constructors,
        "candidate_constructions_upper": constructors + 5 * raw,
        "garde_validations_upper": constructors + 5 * raw,
        "garde_callbacks_upper": constructors + 5 * raw,
        "relationship_passes_upper": passes,
        "relationship_capacity_checks_upper": passes,
        "header_domain_comparisons_upper": 2 * passes,
        "header_domain_getters_upper": 2 * passes,
        "entry_visits_upper": visits,
        "entry_group_comparisons_upper": 5 * visits,
        "entry_scalar_comparisons_upper": 10 * visits,
        "entry_product_getters_upper": 8 * visits,
        "entry_float_bit_queries_upper": 2 * visits,
        "scratch_initialization_writes_upper": 16 * batches,
        "scratch_interval_copy_writes_upper": 16 * batches,
        "scratch_interval_getters_upper": 17 * batches,
        "scratch_checked_prefixes_upper": batches,
        "legacy_batch_attempts_upper": batches,
        "legacy_batch_staging_initializations_upper": 16 * batches,
        "legacy_batch_checked_slots_upper": 16 * batches,
        "legacy_batch_label_getters_upper": 240 * batches,
        "selection_dispatches_upper": selections,
        "selection_result_assemblies_upper": selections,
        "minimum_constructions_upper": minima,
        "minimum_garde_validations_upper": minima,
        "minimum_rule_callbacks_upper": minima,
        "minimum_range_comparisons_upper": 2 * minima,
    }


def _batch_raw_header_work(enabled: bool) -> dict[str, int]:
    """Fund only genuine descriptor setup for the source-only raw header case.

    Raw/public carrier calls are already in _batch_work. This group owns their
    checked descriptor producers and direct assertions; no history or fit exists.
    """
    factor = int(enabled)
    return {key: factor * value for key, value in {
        "labels": 1, "monotonic_constructors_upper": 10,
        "frame_constructions": 1, "domain_constructions": 1, "chart_constructions": 1,
        "descriptor_garde_validations_upper": 3,
        "descriptor_garde_callbacks_upper": 4,
        "descriptor_relationship_predicates_upper": 4,
        "descriptor_scalar_predicates_upper": 4,
        "descriptor_domain_integer_getters_upper": 4,
        "direct_descriptor_getters_upper": 20,
        "direct_float_bit_queries_upper": 10,
        "raw_array_visits": 5,
        "direct_assertion_predicates_upper": 16,
        "direct_error_trait_requests_upper": 0,
    }.items()}


def _batch_observer_work(external_legal: bool) -> dict[str, int]:
    """Keep test observers separate from the source work of product owners.

    Public fixture headers bound formatting/error/iterator requests by64 in
    aggregate; charging each category64 is a conservative disjoint ceiling.
    Each selected witness has at most16 entries. Snapshot equality upper bounds
    count both observations of each actual pair; no checked history is cloned.
    The legal branch charges only extensions to the preserved52-call consumer.
    """
    if external_legal:
        getters = LEGAL_EXPECTED[-1]
        additions = 3 + getters
        return {
            "error_source_requests_upper": 0, "typed_downcasts_upper": 0,
            "iterator_requests_upper": 2, "iterator_item_visits_upper": 2,
            "snapshot_row_equalities_upper": 0, "snapshot_scalar_equalities_upper": 0,
            "fixture_entry_copy_slots_upper": 1, "assertion_predicates_upper": 64,
            "counter_charge_attempts": additions,
            "indexed_counter_accesses_upper": 2 * additions,
            "checked_additions_upper": additions, "cap_comparisons_upper": additions,
            "counter_slot_writes_upper": additions, "finish_equality_elements_upper": 4,
            "record_integer_fields": 4, "record_rows": 0,
        }
    return {
        "error_source_requests_upper": 64, "typed_downcasts_upper": 64,
        "iterator_requests_upper": 64, "iterator_item_visits_upper": 1024,
        "snapshot_row_equalities_upper": 192, "snapshot_scalar_equalities_upper": 896,
        "fixture_entry_copy_slots_upper": 1440, "assertion_predicates_upper": 4096,
        "counter_charge_attempts": 0, "indexed_counter_accesses_upper": 0,
        "checked_additions_upper": 0, "cap_comparisons_upper": 0,
        "counter_slot_writes_upper": 0, "finish_equality_elements_upper": 0,
        "record_integer_fields": 0, "record_rows": 0,
    }


def chart_batch_budget_vector(adaptive: AdaptiveOwner, projection: ProjectionOwner,
                              enclosure: EnclosureOwner, selection: SelectionOwner, *,
                              external_legal: bool = False,
                              include_doctests: bool = True) -> dict[str, int]:
    """Reserve S155 only; keep every previous chart budget field unchanged.

    Ordinary setup reuses the actual source-admitted adaptive/projection/exact-
    enclosure recurrences. Legal setup is already funded by the historical chart
    contribution, so its new vector funds only batch/selection and observer work.
    Source identity, native names and complete candidate admission are separate.
    """
    if any(type(flag) is not bool for flag in (external_legal, include_doctests)):
        raise Incomplete("Chart-batch recipe requires exact Boolean scope selectors.")
    _owner_fields(adaptive, ADAPTIVE_FIELDS)
    _owner_fields(projection, PROJECTION_FIELDS)
    _owner_fields(enclosure, ENCLOSURE_FIELDS)
    if external_legal:
        admit_legal_metadata(LEGAL_FIELDS, dict(zip(LEGAL_FIELDS, LEGAL_EXPECTED, strict=True)),
                             LEGAL_SOURCE_SHA256)
    fixture = {key: 0 if external_legal else value for key, value in BATCH_FIXTURE_CALLS.items()}
    if external_legal:
        # The marker's22 requests contain20 product getters and two iterator
        # next calls; the latter are funded only by the observer group.
        fixture["other_getters_upper"] = LEGAL_EXPECTED[-1] - 2
    adaptive_calls = dict.fromkeys(ADAPTIVE_FIELDS, 0)
    adaptive_calls.update(adaptive_histories=fixture["histories"],
                          adaptive_observations=fixture["observations"],
                          adaptive_appends=fixture["appends"])
    # Keep the helper's original13-history per-call ceiling. Four disjoint
    # (8 histories,24 observations,24 appends) blocks exactly partition this
    # fixed fixture. All source_work terms are additive for these fields with
    # raw/private/event scopes false; no per-fixture event or raw charge repeats.
    blocks = 1 if external_legal else 4
    if any(value % blocks for value in adaptive_calls.values()):
        raise Incomplete("Chart-batch source partition is not an exact disjoint sum.")
    partition = {key: value // blocks for key, value in adaptive_calls.items()}
    adaptive_work = dict.fromkeys(ADAPTIVE_WORK_FIELDS, 0)
    for _ in range(blocks):
        part = _work(adaptive.source_work(partition, include_raw_minimum=False,
                                         event_fixture_reached=False), ADAPTIVE_WORK_FIELDS)
        for key, value in part.items():
            adaptive_work[key] += value
    adaptive_work = _work(adaptive_work, ADAPTIVE_WORK_FIELDS)
    expected = {key: 0 if external_legal else value
                for key, value in BATCH_EXPECTED_NUMERIC_WORK.items()}
    if {key: adaptive_work[key] for key in expected} != expected:
        raise Incomplete("Chart-batch adaptive recurrence differs from its reviewed upper.")
    projection_calls = dict.fromkeys(PROJECTION_FIELDS, 0)
    projection_calls.update(terms=fixture["terms"], projections=fixture["tracking_projections"])
    projection_work = _work(projection.source_work_uppers(projection_calls), PROJECTION_WORK_FIELDS)
    enclosure_calls = dict.fromkeys(ENCLOSURE_FIELDS, 0)
    enclosure_calls.update(enclosures=fixture["enclosures"])
    enclosure_work = _work(enclosure.source_work_uppers(enclosure_calls), ENCLOSURE_WORK_FIELDS)
    if any(work[key] for work in (projection_work, enclosure_work) for key in expected):
        raise Incomplete("Chart-batch delegated work double-funds its adaptive setup.")
    adapter = _work(projection.projection_adapter_work(0, projection_calls["projections"]), (
        "projection_capsule_and_kernel_entries_upper", "projection_adaptive_extra_internal_getters_upper"))
    batches, minima, selections = (1, 1, 1) if external_legal else (44, 8, 8)
    batch_work = _batch_work(batches, selections, minima, raw_header=not external_legal)
    try:
        selected = selection.source_work_uppers(selections, batch_work["legacy_batch_attempts_upper"])
    except AttributeError as error:
        raise Incomplete("Chart-batch selection accounting interface is missing.") from error
    selection_work = _work(selected, BATCH_SELECTION_WORK_FIELDS)
    docs = int(include_doctests and not external_legal)
    groups = (
        ("fixture.", fixture), ("adaptive_call.", adaptive_calls),
        ("adaptive_work.", adaptive_work), ("projection_call.", projection_calls),
        ("projection_work.", projection_work), ("adapter_work.", adapter),
        ("enclosure_call.", enclosure_calls), ("enclosure_work.", enclosure_work),
        ("owner_work.", _owner_work(fixture)),
        ("tracking_work.", _tracking_work(fixture["tracking_projections"])),
        ("roundoff_work.", _roundoff_work(fixture)),
        ("batch_work.", batch_work), ("selection_work.", selection_work),
        ("raw_header.", _batch_raw_header_work(not external_legal)),
        ("observer.", _batch_observer_work(external_legal)),
        ("doctest.", {"compilation_cases": 5 * docs, "compile_fail_cases": 4 * docs,
                      "uncalled_legal_helpers": docs, "executed_producers": 0}),
    )
    result: dict[str, int] = {}
    for prefix, values in groups:
        for key, value in _checked(values, len(values)).items():
            name = "chart_batch." + prefix + key
            if name in result:
                raise Incomplete("Chart-batch source namespaces overlap.")
            result[name] = value
    return _checked_batch_vector(result)


def chart_batch_numeric_work(vector: dict[str, int]) -> dict[str, int]:
    """Extract only the supplemental adaptive contribution to global scalars."""
    checked = _checked_batch_vector(vector)
    names = {key: "chart_batch.adaptive_work." + key for key in BATCH_EXPECTED_NUMERIC_WORK}
    if any(name not in checked for name in names.values()):
        raise Incomplete("Chart-batch numeric source keys are missing.")
    return {key: checked[name] for key, name in names.items()}


def preserve_chart_batch_extension(old: dict[str, int], batch: dict[str, int]) -> dict[str, int]:
    """Preserve1508 fields and add fit/point and existing Geometry totals once."""
    _checked(old, 1508)
    _checked_batch_vector(batch)
    aggregate = (*BATCH_EXPECTED_NUMERIC_WORK, "standard_errors.geometry.report_copies_upper",
                 "standard_errors.geometry.logical_bytes_upper")
    if (set(old) & set(batch) or any(not key.startswith("chart_batch.") for key in batch)
            or any(key not in old for key in aggregate)):
        raise Incomplete("Chart-batch extension requires the unchanged1508-field namespace.")
    result = {**old, **batch}
    extra = chart_batch_numeric_work(batch)
    for key, amount in extra.items():
        result[key] += amount
    result["standard_errors.geometry.report_copies_upper"] += extra["charged_fit_work_upper"]
    result["standard_errors.geometry.logical_bytes_upper"] += 32 * extra["charged_fit_work_upper"]
    return _checked(result, 1508 + BATCH_BUDGET_FIELDS)


def chart_batch_test_observation(output: str, *, require_complete: bool = False) -> dict[str, object]:
    """Receive exact standalone ordinary outcomes without inventing call counts.

    Cargo JSON/nextest summaries supply no named libtest observation. Partial or
    caught prefixes remain pending and keep the full source reservation. The raw
    header case must be observed independently of the seven public tests.
    """
    if type(output) is not str or len(output) > 16 * 1024 * 1024 or type(require_complete) is not bool:
        raise Incomplete("Chart-batch observation has an invalid type or output bound.")
    observations: dict[str, list[str]] = {name: [] for name in BATCH_ORDINARY_TESTS}
    for line in output.splitlines(keepends=True):
        if not line.startswith("test "):
            continue
        for name in BATCH_ORDINARY_TESTS:
            if name not in line:
                continue
            prefix = r"(?:chart::)?" if name in BATCH_PUBLIC_TESTS else ""
            matched = re.fullmatch(r"test " + prefix + re.escape(name) + r" \.\.\. (ok|FAILED|ignored)\r?\n", line)
            observations[name].append(matched.group(1) if matched else "malformed")
    if any(len(values) > 1 for values in observations.values()):
        raise Incomplete("Chart-batch named outcome is duplicated or replayed.")
    statuses = {name: values[0] if values else "missing" for name, values in observations.items()}
    complete = all(value == "ok" for value in statuses.values())
    if require_complete and not complete:
        raise Incomplete("Complete package lacks all source-bound chart-batch outcomes.")
    return {"chart_batch_test_statuses": statuses, "chart_batch_source_only_pending": not complete,
            "chart_batch_recorded_source_work_upper": dict(BATCH_EXPECTED_NUMERIC_WORK) if complete else None,
            "chart_batch_observed_operation_counts": None}


# Closed namespace schema derives only key names from local pure functions and
# already literal interface tuples. No injected owner or native work runs here.
BATCH_GROUP_FIELDS: dict[str, tuple[str, ...]] = {
    "fixture.": tuple(BATCH_FIXTURE_CALLS),
    "adaptive_call.": ADAPTIVE_FIELDS, "adaptive_work.": ADAPTIVE_WORK_FIELDS,
    "projection_call.": PROJECTION_FIELDS, "projection_work.": PROJECTION_WORK_FIELDS,
    "adapter_work.": ("projection_capsule_and_kernel_entries_upper",
                      "projection_adaptive_extra_internal_getters_upper"),
    "enclosure_call.": ENCLOSURE_FIELDS, "enclosure_work.": ENCLOSURE_WORK_FIELDS,
    "owner_work.": tuple(_owner_work(BATCH_FIXTURE_CALLS)),
    "tracking_work.": tuple(_tracking_work(0)),
    "roundoff_work.": tuple(_roundoff_work(BATCH_FIXTURE_CALLS)),
    "batch_work.": tuple(_batch_work(0, 0, 0, raw_header=False)),
    "selection_work.": BATCH_SELECTION_WORK_FIELDS,
    "raw_header.": tuple(_batch_raw_header_work(False)),
    "observer.": tuple(_batch_observer_work(False)),
    "doctest.": ("compilation_cases", "compile_fail_cases", "uncalled_legal_helpers", "executed_producers"),
}
BATCH_BUDGET_KEYS = tuple("chart_batch." + prefix + field
                          for prefix, fields in BATCH_GROUP_FIELDS.items() for field in fields)


def _checked_batch_vector(values: object) -> dict[str, int]:
    """Reject replacement keys as well as wrong types, widths and overflow."""
    checked = _checked(values, BATCH_BUDGET_FIELDS)
    if (len(BATCH_BUDGET_KEYS) != BATCH_BUDGET_FIELDS
            or len(set(BATCH_BUDGET_KEYS)) != BATCH_BUDGET_FIELDS
            or set(checked) != set(BATCH_BUDGET_KEYS)):
        raise Incomplete("Chart-batch vector differs from its complete closed namespace.")
    return checked
