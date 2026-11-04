"""Bounded S158/S1.4a source reservations, distinct from executed test outcomes.

No I/O, repository import, process launch or native admission occurs here.
Frozen sparse recurrences preserve the separately admitted owners' existing
capacity64/16 ceilings. The coordinator injects their actual pure functions;
each result must dominate every frozen named ceiling. Source identities,
ordinary outcomes and prospective reservations are separate obligations.
"""
from __future__ import annotations

import hashlib
import re
from typing import Callable, Literal, NamedTuple, TypeAlias, TypedDict, TypeGuard

class Incomplete(ValueError):
    """A frozen source recipe, owner projection or observation is incomplete."""

WorkFunction: TypeAlias = Callable[[dict[str, int]], dict[str, int]]

class WorkOwners(NamedTuple):
    """Injected source-admitted pure functions, normalized to named integer inputs."""
    adaptive_work: WorkFunction
    projection_work: WorkFunction
    adapter_work: WorkFunction
    enclosure_work: WorkFunction
    selection_work: WorkFunction
    chart_owner_work: WorkFunction
    tracking_work: WorkFunction
    chart_roundoff_work: WorkFunction
    chart_batch_work: WorkFunction
    legacy_batch_work: WorkFunction
    descriptor_work: WorkFunction
    principal_receiver_work: WorkFunction
    lifecycle_work: WorkFunction

STATUS = "prepared-source-recipe-not-native-admission"
INPUTS = ("scripts/core-chart-combination-accounting.py", "scripts/tests/test_core_chart_combination_accounting.py")
MAX_UNITS = (1 << 63) - 1
BUDGET_FIELDS = 486
BASE_BUDGET_FIELDS = 2214
FULL_BUDGET_FIELDS = 2700
PREFIX = "chart_combination."
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
REFERENCE_FIELDS = ("state_constructions", "readiness_reads", "admissions", "feedback",
                    "token_constructions", "combined_reads", "round_reads", "error_displays",
                    "error_debug", "state_token_debug")

SOURCE_SHA256 = {'crates/time-core/src/combination.rs': 'd33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326',
 'crates/time-core/src/combination/admission.rs': '7519a3ffabcbe05e7b4adc24baff0bcbbdcc1be31768454b49708ab190350bc0',
 'crates/time-core/src/combination/lifecycle.rs': '6e415208ccfd93ebe976c09c887c8c42090f85bfec313a17c19594a891dc5081',
 'crates/time-core/src/regression/chart.rs': 'ed2c7d54b7a9a346327199b3b4d3b32fc8ed14f8d7c5a6bb61b867b820f9aed0',
 'crates/time-core/src/regression/chart/roundoff.rs': '99bb94b460a66401ea683440b3991ae75a11f3ca1de75dc4fceb71f7109e9169',
 'crates/time-core/src/combination/chart/roundoff.rs': '1001dffe6cac911207879962f39b3aeff5a0668916cf53b5e42986523ff1c5eb',
 'crates/time-core/src/combination/chart/receiving.rs': '4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553',
 'crates/time-core/src/combination/chart/reduction.rs': '3a9e98e64224ca0ad5f4c016a5c370dd305e52aef6dc8d38b4f2cc2db7eb74eb',
 'crates/time-core/src/combination/lifecycle/chart.rs': 'b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7',
 'crates/time-core/src/combination/chart.rs': '71520d44b91d321caea3bceaff043a42ae5ad6d3e65a23218ad072387a2f0eed',
 'crates/time-core/src/lib.rs': 'fa86c7a1aef3a91e8e711bc99a88bff28e376daa11bcf15fe4356e7c585918d5',
 'crates/time-core/tests/private/chart_combination_roundoff.rs': 'aa2fa8275c1199f2595c0f2528ea4ed3cd2d1177514dc79266cebde5f3efc579',
 'crates/time-core/src/combination/chart/receiving_tests.rs': 'b4234295d21a3b5a0df40c99803ff7625f21143b44b9db87149eb2518919f132',
 'crates/time-core/tests/chart_combination_lifecycle.rs': 'eaf350e238bcdc881024ada0fd4915b7f2dcf1f2d97b4c0d8e0227b18a183772',
 'crates/time-core/tests/chart_combination_edges.rs': 'ed0055ab15c9665c56b737c9958abf1f74bd7a718db7a4552c6207431421d360',
 'crates/time-core/tests/private/chart_combination_lifecycle_guards.rs': '42bd34d892968439bd5852857e2c56ae980a42f9ad4594a4fb7a688e99ff3e45',
 'crates/time-core/src/reference_input.rs': 'cb5202acfd4bc5d6b34b8ac160f8b17e99232539ffa95f22c6c96dfad848254d',
 'crates/time-core/tests/private/reference_input_readiness.rs': 'b4b6e456d3dd0828aed831823f5ccafe66815b244015ec42f4d3eda61519270e'}

GROUP_FIELDS: dict[str, tuple[str, ...]] = {'fixture': ('histories',
             'observations',
             'appends',
             'labels_upper',
             'contexts_upper',
             'monotonic_upper',
             'coordinates',
             'exchanges',
             'frames_upper',
             'domains_upper',
             'charts',
             'prepares',
             'known_prepares_upper',
             'terms',
             'anchor_projections',
             'tracking_projections',
             'chart_enclosures',
             'raw_enclosures',
             'chart_batches',
             'raw_batches',
             'minima',
             'principal_parameters',
             'strata',
             'principal_seeds',
             'principal_advances',
             'chart_principal_advances',
             'combination_parameters',
             'combination_seeds',
             'combination_advances',
             'reachability_updates',
             'reachability_records',
             'state_equalities',
             'state_clones',
             'chart_reductions',
             'singleton_copies',
             'getters_upper'),
 'adaptive_call': ('retained_points',
                   'retained_inputs',
                   'retained_selections',
                   'adaptive_observations',
                   'adaptive_appends',
                   'adaptive_histories',
                   'raw_parameter_validations',
                   'raw_ordinal_validations',
                   'raw_event_validations',
                   'external_normalizations'),
 'projection_call': ('histories',
                     'appends',
                     'observations',
                     'contexts',
                     'monotonic',
                     'coordinates',
                     'exchanges',
                     'getters',
                     'terms',
                     'projections'),
 'enclosure_call': ('histories',
                    'appends',
                    'observations',
                    'contexts',
                    'monotonic',
                    'coordinates',
                    'exchanges',
                    'getters',
                    'terms',
                    'projections',
                    'enclosures'),
 'adaptive_work': ('charged_fit_work_upper',
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
                   'raw_event_source_lookups_upper'),
 'projection_work': ('charged_fit_work_upper',
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
                     'projection_arithmetic_primitive_calls_upper'),
 'adapter_work': ('projection_capsule_and_kernel_entries_upper',
                  'projection_adaptive_extra_internal_getters_upper'),
 'enclosure_work': ('charged_fit_work_upper',
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
                    'enclosure_interval_rule_calls_upper'),
 'selection_work': ('selection_endpoint_collection_writes_upper',
                    'selection_endpoint_sort_comparisons_upper',
                    'selection_endpoint_dedup_comparisons_upper',
                    'selection_cell_visits_upper',
                    'selection_membership_probes_upper',
                    'selection_component_visits_upper',
                    'selection_witness_probes_upper',
                    'selection_label_sort_comparisons_upper',
                    'selection_intersection_reductions_upper',
                    'selection_batch_copy_adapter_attempts_upper',
                    'selection_batch_label_comparisons_upper'),
 'chart_owner_work': ('frame_garde_validations_upper',
                      'frame_rule_callbacks_upper',
                      'frame_relationship_predicates_upper',
                      'domain_garde_validations_upper',
                      'domain_rule_callbacks_upper',
                      'domain_relationship_predicates_upper',
                      'domain_integer_getters_upper',
                      'affine_garde_validations_upper',
                      'affine_rule_callbacks_upper',
                      'affine_scalar_predicates_upper',
                      'history_garde_validations_upper',
                      'history_rule_callbacks_upper',
                      'history_relationship_passes_upper',
                      'history_row_visits_upper',
                      'history_domain_comparisons_upper',
                      'history_getters_upper',
                      'projection_garde_validations_upper',
                      'projection_rule_callbacks_upper',
                      'projection_scalar_predicates_upper',
                      'tracking_garde_validations_upper',
                      'tracking_rule_callbacks_upper',
                      'tracking_scalar_predicates_upper',
                      'project_dispatch_entries_upper',
                      'target_context_constructions_upper',
                      'project_parent_getters_upper',
                      'query_domain_comparisons_upper',
                      'original_projection_retention_upper',
                      'tracking_payload_retention_upper',
                      'enclosure_outer_entries_upper',
                      'enclosure_radius_bridge_entries_upper',
                      'enclosure_context_extra_getters_upper',
                      'enclosure_context_constructions_upper',
                      'enclosure_projection_copies_upper'),
 'tracking_work': ('standard_error_requests_upper',
                   'standard_error_binary_primitives_upper',
                   'standard_error_square_roots_upper',
                   'standard_error_scalar_constructions_upper',
                   'standard_error_garde_validations_upper',
                   'standard_error_rule_callbacks_upper',
                   'standard_error_predicates_upper',
                   'reference_duration_operations_upper',
                   'reference_duration_getters_upper',
                   'reference_integer_conversions_upper',
                   'reference_conversion_shared_primitives_upper',
                   'numeric_getters_upper',
                   'tracking_binary_primitives_upper',
                   'skew_validations_upper',
                   'final_validations_upper',
                   'rule_callbacks_upper',
                   'scalar_predicates_upper'),
 'chart_roundoff_work': ('transform_requests_upper',
                         'elapsed_requests_upper',
                         'monotonic_getters_upper',
                         'original_projection_getters_upper',
                         'integer_conversion_requests_upper',
                         'integer_shared_primitives_upper',
                         'value_primitives_upper',
                         'total_shared_value_primitives_upper',
                         'step_operation_mappings_upper',
                         'bound_add_requests_upper',
                         'bound_multiply_requests_upper',
                         'bound_divide_requests_upper',
                         'bound_requests_upper',
                         'bound_binary_primitives_upper',
                         'bound_next_up_requests_upper',
                         'bound_operand_and_result_predicates_upper'),
 'chart_batch_work': ('constructor_attempts_upper',
                      'initial_capacity_checks_upper',
                      'expected_descriptor_getters_upper',
                      'expected_float_bit_queries_upper',
                      'candidate_constructions_upper',
                      'garde_validations_upper',
                      'garde_callbacks_upper',
                      'relationship_passes_upper',
                      'relationship_capacity_checks_upper',
                      'header_domain_comparisons_upper',
                      'header_domain_getters_upper',
                      'entry_visits_upper',
                      'entry_group_comparisons_upper',
                      'entry_scalar_comparisons_upper',
                      'entry_product_getters_upper',
                      'entry_float_bit_queries_upper',
                      'scratch_initialization_writes_upper',
                      'scratch_interval_copy_writes_upper',
                      'scratch_interval_getters_upper',
                      'scratch_checked_prefixes_upper',
                      'legacy_batch_attempts_upper',
                      'legacy_batch_staging_initializations_upper',
                      'legacy_batch_checked_slots_upper',
                      'legacy_batch_label_getters_upper',
                      'selection_dispatches_upper',
                      'selection_result_assemblies_upper',
                      'minimum_constructions_upper',
                      'minimum_garde_validations_upper',
                      'minimum_rule_callbacks_upper',
                      'minimum_range_comparisons_upper'),
 'legacy_batch_work': ('constructor_attempts_upper',
                       'authored_helper_entries_upper',
                       'garde_validations_upper',
                       'garde_callbacks_upper',
                       'entry_read_attempts_upper',
                       'product_getters_upper',
                       'scratch_copy_assignments_upper'),
 'descriptor_work': ('initial_metadata_capacity_checks_upper',
                     'candidate_constructions_upper',
                     'garde_validations_upper',
                     'garde_callbacks_upper',
                     'relationship_passes_upper',
                     'descriptor_getters_upper',
                     'frame_label_comparisons_upper',
                     'domain_instant_comparisons_upper',
                     'scalar_bit_queries_upper',
                     'scalar_bit_comparisons_upper',
                     'batch_header_and_source_frame_getters_upper'),
 'principal_receiver_work': ('parameter_constructor_attempts_upper',
                             'parameter_garde_validations_upper',
                             'parameter_garde_callbacks_upper',
                             'parameter_scalar_rule_calls_upper',
                             'parameter_scalar_predicates_upper',
                             'stratum_constructor_attempts_upper',
                             'stratum_garde_validations_upper',
                             'stratum_garde_callbacks_upper',
                             'stratum_range_predicates_upper',
                             'roster_garde_validations_upper',
                             'roster_garde_callbacks_upper',
                             'roster_relationship_passes_upper',
                             'roster_duplicate_comparisons_upper',
                             'seed_slot_initializations_upper',
                             'seed_source_record_writes_upper',
                             'receiver_garde_validations_upper',
                             'receiver_garde_callbacks_upper',
                             'receiver_relationship_passes_upper',
                             'receiver_key_comparisons_upper',
                             'receiver_control_predicates_upper',
                             'receiver_original_projection_visits_upper',
                             'legacy_projection_adapter_getters_upper',
                             'chart_projection_adapter_getters_upper',
                             'receiver_original_product_getters_upper',
                             'source_lookup_slot_visits_upper',
                             'stratum_lookup_slot_visits_upper',
                             'staged_source_field_writes_upper',
                             'staged_state_clone_cells_upper',
                             'staged_source_fields_copied_upper',
                             'staged_fixed_fields_copied_upper',
                             'witness_slot_initializations_upper',
                             'witness_membership_visits_upper',
                             'ranking_slot_visits_upper',
                             'checked_scalar_primitives_upper',
                             'scalar_supported_predicates_upper',
                             'credit_slot_visits_upper',
                             'legacy_selection_dispatches_upper',
                             'selection_association_entry_visits_upper',
                             'selection_association_supporter_probes_upper',
                             'decision_assemblies_upper',
                             'completed_state_commit_attempts_upper'),
 'lifecycle_work': ('lifecycle_parameter_constructor_attempts_upper',
                    'lifecycle_parameter_garde_callbacks_upper',
                    'lifecycle_parameter_predicate_calls_upper',
                    'lifecycle_raw_ordinal_constructor_attempts_upper',
                    'lifecycle_garde_validate_calls_upper',
                    'lifecycle_source_counter_constructions_upper',
                    'lifecycle_source_counter_predicate_calls_upper',
                    'lifecycle_event_admission_predicate_calls_upper',
                    'lifecycle_checked_next_calls_upper',
                    'lifecycle_compatibility_checked_binary_calls_upper',
                    'lifecycle_compatibility_source_visits_upper',
                    'lifecycle_source_index_label_probes_upper',
                    'lifecycle_witness_membership_label_probes_upper',
                    'lifecycle_output_contributor_label_probes_upper',
                    'lifecycle_output_predicate_calls_upper',
                    'lifecycle_source_slot_writes_upper',
                    'lifecycle_staged_source_copy_cells_upper',
                    'lifecycle_state_equality_source_cells_upper',
                    'lifecycle_state_equality_fixed_fields_upper',
                    'lifecycle_internal_product_getter_requests_upper',
                    'bridge_admission_entry_attempts_upper',
                    'bridge_complete_receiving_passes_upper',
                    'bridge_receiving_pointer_comparisons_upper',
                    'bridge_receiving_label_comparisons_upper',
                    'bridge_mask_admission_slot_visits_upper',
                    'bridge_direct_garde_validate_calls_upper',
                    'bridge_reduction_entry_attempts_upper',
                    'bridge_reduction_checked_binary_calls_upper',
                    'bridge_reduction_checked_root_calls_upper',
                    'bridge_reduction_positive_denominator_guards_upper',
                    'bridge_reduction_supporter_visits_upper',
                    'bridge_record_lookup_visits_upper',
                    'bridge_internal_product_getter_requests_upper',
                    'uncertainty_constructor_attempts_upper',
                    'uncertainty_garde_validation_calls_upper',
                    'uncertainty_garde_rule_callbacks_upper',
                    'uncertainty_predicate_calls_upper',
                    'kernel_final_uncertainty_predicate_calls_upper'),
 'point_work': ('kernel_checked_binary_calls_upper',
                'kernel_checked_square_root_calls_upper',
                'kernel_positive_denominator_guards_upper',
                'kernel_supporter_reduction_visits_upper',
                'kernel_record_lookup_visits_upper',
                'kernel_metadata_pointer_comparisons_upper',
                'kernel_metadata_label_comparisons_upper',
                'kernel_internal_product_getter_requests_upper',
                'kernel_final_uncertainty_predicate_calls_upper'),
 'prior_adapter_work': ('legacy_principal_seeds_upper',
                        'legacy_principal_advances_upper',
                        'legacy_direct_completions_upper',
                        'original_cache_slot_initializations_upper',
                        'original_cache_materialization_getters_upper',
                        'supporter_values_extractions_upper',
                        'transaction_adapter_entries_upper',
                        'decision_outcome_adapter_getters_upper',
                        'bound_bridge_calls_upper'),
 'companion_work': ('input_constructions_upper',
                    'result_constructions_upper',
                    'exact_constructions_upper',
                    'add_requests_upper',
                    'subtract_requests_upper',
                    'multiply_requests_upper',
                    'divide_requests_upper',
                    'lower_margin_requests_upper',
                    'point_primitives_upper',
                    'bound_primitive_requests_upper',
                    'bound_operand_result_predicates_upper',
                    'bound_successor_requests_upper',
                    'point_predicates_upper',
                    'error_predicates_upper',
                    'garde_validations_upper',
                    'garde_callbacks_upper'),
 'receiving_work': ('holder_requests_upper',
                    'holder_slot_initializations_upper',
                    'entry_lookup_visits_upper',
                    'admission_requests_upper',
                    'garde_validations_upper',
                    'receiving_predicate_passes_upper',
                    'context_comparisons_upper',
                    'actual_wrapper_comparisons_upper',
                    'duplicate_label_comparisons_upper',
                    'witness_label_comparisons_upper',
                    'attachment_pointer_comparisons_upper',
                    'padding_slot_visits_upper',
                    'cache_slot_initializations_upper',
                    'cache_record_materializations_upper'),
 'reduction_work': ('driver_attempts_upper',
                    'direct_record_attempts_upper',
                    'finish_attempts_upper',
                    'singleton_attempts_upper',
                    'mask_garde_validations_upper',
                    'mask_predicate_passes_upper',
                    'mask_slot_visits_upper',
                    'contributor_visits_upper',
                    'canonical_label_writes_upper',
                    'point_consistency_comparisons_upper'),
 'output_work': ('completion_attempts_upper',
                 'garde_validations_upper',
                 'garde_callbacks_upper',
                 'postcondition_passes_upper',
                 'descriptor_comparisons_upper',
                 'batch_pointer_comparisons_upper',
                 'principal_pointer_comparisons_upper',
                 'source_context_comparisons_upper',
                 'target_context_comparisons_upper',
                 'point_scalar_predicates_upper',
                 'mean_bound_predicates_upper',
                 'canonical_label_comparisons_upper',
                 'contributor_membership_probes_upper'),
 'observer_work': ('ordinary_outcomes_upper',
                   'assertion_predicates_upper',
                   'getter_requests_upper',
                   'integer_oracle_operations_upper',
                   'diagnostic_requests_upper',
                   'outcome_receiving_rows_upper'),
 'doctest': ('compilation_cases', 'compile_fail_cases', 'executed_examples', 'fit_or_point_work')}

DELEGATE_INPUTS: dict[str, tuple[str, ...]] = {'adaptive_work': ('adaptive_histories', 'adaptive_observations', 'adaptive_appends'),
 'projection_work': ('histories',
                     'appends',
                     'observations',
                     'contexts',
                     'monotonic',
                     'coordinates',
                     'exchanges',
                     'getters',
                     'terms',
                     'projections'),
 'adapter_work': ('fixed', 'adaptive'),
 'enclosure_work': ('histories',
                    'appends',
                    'observations',
                    'contexts',
                    'monotonic',
                    'coordinates',
                    'exchanges',
                    'getters',
                    'terms',
                    'projections',
                    'enclosures'),
 'selection_work': ('evaluations', 'batches'),
 'chart_owner_work': ('histories',
                      'observations',
                      'appends',
                      'contexts_upper',
                      'labels_upper',
                      'monotonic_upper',
                      'coordinates',
                      'exchanges',
                      'frames_upper',
                      'domains_upper',
                      'charts',
                      'prepares',
                      'known_prepares_upper',
                      'terms',
                      'anchor_projections',
                      'tracking_projections',
                      'enclosures',
                      'snapshots',
                      'snapshot_rows_upper',
                      'snapshot_getters_upper',
                      'other_getters_upper',
                      'exact_units_calls',
                      'subtraction_oracle_calls',
                      'nanosecond_oracle_calls',
                      'near_comparisons',
                      'debug_format_requests_upper'),
 'tracking_work': ('tracked',),
 'chart_roundoff_work': ('histories',
                         'observations',
                         'appends',
                         'contexts_upper',
                         'labels_upper',
                         'monotonic_upper',
                         'coordinates',
                         'exchanges',
                         'frames_upper',
                         'domains_upper',
                         'charts',
                         'prepares',
                         'known_prepares_upper',
                         'terms',
                         'anchor_projections',
                         'tracking_projections',
                         'enclosures',
                         'snapshots',
                         'snapshot_rows_upper',
                         'snapshot_getters_upper',
                         'other_getters_upper',
                         'exact_units_calls',
                         'subtraction_oracle_calls',
                         'nanosecond_oracle_calls',
                         'near_comparisons',
                         'debug_format_requests_upper'),
 'chart_batch_work': ('batches', 'selections', 'minima'),
 'legacy_batch_work': ('batches',),
 'descriptor_work': ('advances',),
 'principal_receiver_work': ('principal_seeds',
                             'legacy_advances',
                             'chart_advances',
                             'principal_parameters',
                             'principal_strata'),
 'lifecycle_work': ('combination_parameters',
                    'seeds',
                    'advances',
                    'equalities',
                    'reachability_updates',
                    'reachability_records')}

DELEGATE_FLOORS: dict[str, dict[str, dict[str, int]]] = {'adaptive_work': {'adaptive_minimum_validations_upper': {'adaptive_histories': 1},
                   'adaptive_minimum_rule_callbacks_upper': {'adaptive_histories': 1},
                   'adaptive_minimum_predicates_upper': {'adaptive_histories': 2},
                   'adaptive_observation_validations_upper': {'adaptive_observations': 1},
                   'adaptive_observation_rule_callbacks_upper': {'adaptive_observations': 2},
                   'charged_fit_work_upper': {'adaptive_appends': 62},
                   'point_admissions_upper': {'adaptive_appends': 2141},
                   'selector_entries_upper': {'adaptive_appends': 1},
                   'selector_fit_upper': {'adaptive_appends': 62},
                   'selector_point_copy_upper': {'adaptive_appends': 2077},
                   'selector_input_slots_upper': {'adaptive_appends': 3968},
                   'selector_residual_slots_upper': {'adaptive_appends': 3968},
                   'selector_decision_residuals_upper': {'adaptive_appends': 3098},
                   'selector_checked_float_upper': {'adaptive_appends': 53540},
                   'adaptive_normalizations_upper': {'adaptive_appends': 1},
                   'adaptive_generated_points_upper': {'adaptive_appends': 64},
                   'adaptive_duration_ops_upper': {'adaptive_appends': 64},
                   'adaptive_fit_inputs_upper': {'adaptive_appends': 1},
                   'adaptive_normalized_input_slots_upper': {'adaptive_appends': 64},
                   'direct_and_generated_point_validations_upper': {'adaptive_appends': 64},
                   'direct_and_generated_point_rule_callbacks_upper': {'adaptive_appends': 192},
                   'adaptive_stage_init_slots_upper': {'adaptive_appends': 64},
                   'adaptive_stage_retained_copies_upper': {'adaptive_appends': 63},
                   'adaptive_compaction_copies_upper': {'adaptive_appends': 63},
                   'adaptive_array_transfer_slots_upper': {'adaptive_appends': 256},
                   'adaptive_complete_context_checks_upper': {'adaptive_appends': 64},
                   'adaptive_complete_order_checks_upper': {'adaptive_appends': 63}},
 'projection_work': {'charged_fit_work_upper': {'appends': 1},
                     'point_admissions_upper': {'appends': 64},
                     'history_normalization_entries_upper': {'appends': 1},
                     'history_statistics_entries_upper': {'appends': 1},
                     'history_statistics_inner_fit_upper': {'appends': 1},
                     'history_duration_operations_upper': {'appends': 64},
                     'history_generated_point_admissions_upper': {'appends': 64},
                     'history_fit_input_constructions_upper': {'appends': 1},
                     'history_staging_Copy_assignments_upper': {'appends': 127},
                     'projection_raw_candidate_constructions_upper': {'terms': 1},
                     'projection_candidate_validation_calls_upper': {'terms': 1},
                     'projection_first_error_calls_upper': {'terms': 1},
                     'projection_term_garde_rule_callbacks_upper': {'terms': 3},
                     'projection_term_predicate_calls_upper': {'terms': 6},
                     'projection_duration_operations_upper': {'projections': 2},
                     'projection_internal_product_getter_requests_upper': {'projections': 12},
                     'projection_checked_integer_helper_calls_upper': {'projections': 2},
                     'projection_normalization_primitive_calls_upper': {'projections': 6},
                     'projection_arithmetic_primitive_calls_upper': {'projections': 5}},
 'adapter_work': {'projection_capsule_and_kernel_entries_upper': {'fixed': 2, 'adaptive': 2},
                  'projection_adaptive_extra_internal_getters_upper': {'adaptive': 1}},
 'enclosure_work': {'charged_fit_work_upper': {'appends': 1},
                    'point_admissions_upper': {'appends': 64},
                    'history_normalization_entries_upper': {'appends': 1},
                    'history_statistics_entries_upper': {'appends': 1},
                    'history_statistics_inner_fit_upper': {'appends': 1},
                    'history_duration_operations_upper': {'appends': 64},
                    'history_generated_point_admissions_upper': {'appends': 64},
                    'history_fit_input_constructions_upper': {'appends': 1},
                    'history_staging_Copy_assignments_upper': {'appends': 127},
                    'projection_raw_candidate_constructions_upper': {'terms': 1},
                    'projection_candidate_validation_calls_upper': {'terms': 1},
                    'projection_first_error_calls_upper': {'terms': 1},
                    'projection_term_garde_rule_callbacks_upper': {'terms': 3},
                    'projection_term_predicate_calls_upper': {'terms': 6},
                    'projection_duration_operations_upper': {'projections': 2},
                    'projection_internal_product_getter_requests_upper': {'projections': 12},
                    'projection_checked_integer_helper_calls_upper': {'projections': 2},
                    'projection_normalization_primitive_calls_upper': {'projections': 6},
                    'projection_arithmetic_primitive_calls_upper': {'projections': 5},
                    'enclosure_bridge_entry_calls_upper': {'enclosures': 1},
                    'enclosure_wrapper_debug_format_calls_upper': {'enclosures': 1},
                    'enclosure_projection_getter_chain_calls_upper': {'enclosures': 5},
                    'enclosure_raw_helper_entries_upper': {'enclosures': 28},
                    'enclosure_raw_garde_validate_calls_upper': {'enclosures': 1},
                    'enclosure_raw_garde_rule_callbacks_upper': {'enclosures': 2},
                    'enclosure_checked_scalar_primitive_calls_upper': {'enclosures': 20},
                    'enclosure_interval_constructor_attempts_upper': {'enclosures': 1},
                    'enclosure_interval_garde_validate_calls_upper': {'enclosures': 2},
                    'enclosure_interval_rule_calls_upper': {'enclosures': 1}},
 'selection_work': {'selection_endpoint_collection_writes_upper': {'evaluations': 32},
                    'selection_endpoint_sort_comparisons_upper': {'evaluations': 496},
                    'selection_endpoint_dedup_comparisons_upper': {'evaluations': 31},
                    'selection_cell_visits_upper': {'evaluations': 63},
                    'selection_membership_probes_upper': {'evaluations': 1008},
                    'selection_component_visits_upper': {'evaluations': 63},
                    'selection_witness_probes_upper': {'evaluations': 16},
                    'selection_label_sort_comparisons_upper': {'evaluations': 120},
                    'selection_intersection_reductions_upper': {'evaluations': 30},
                    'selection_batch_copy_adapter_attempts_upper': {'batches': 16},
                    'selection_batch_label_comparisons_upper': {'batches': 120}},
 'chart_owner_work': {'frame_garde_validations_upper': {'frames_upper': 1},
                      'frame_rule_callbacks_upper': {'frames_upper': 1},
                      'frame_relationship_predicates_upper': {'frames_upper': 2},
                      'domain_garde_validations_upper': {'domains_upper': 1},
                      'domain_rule_callbacks_upper': {'domains_upper': 1},
                      'domain_relationship_predicates_upper': {'domains_upper': 2},
                      'domain_integer_getters_upper': {'domains_upper': 4},
                      'affine_garde_validations_upper': {'charts': 1},
                      'affine_rule_callbacks_upper': {'charts': 2},
                      'affine_scalar_predicates_upper': {'charts': 4},
                      'history_getters_upper': {'prepares': 1, 'known_prepares_upper': 270},
                      'history_garde_validations_upper': {'known_prepares_upper': 1},
                      'history_rule_callbacks_upper': {'known_prepares_upper': 1},
                      'history_relationship_passes_upper': {'known_prepares_upper': 2},
                      'history_row_visits_upper': {'known_prepares_upper': 128},
                      'history_domain_comparisons_upper': {'known_prepares_upper': 260},
                      'projection_garde_validations_upper': {'anchor_projections': 1,
                                                             'tracking_projections': 1},
                      'projection_rule_callbacks_upper': {'anchor_projections': 4, 'tracking_projections': 4},
                      'projection_scalar_predicates_upper': {'anchor_projections': 8,
                                                             'tracking_projections': 8},
                      'project_dispatch_entries_upper': {'anchor_projections': 1, 'tracking_projections': 1},
                      'target_context_constructions_upper': {'anchor_projections': 1,
                                                             'tracking_projections': 1},
                      'project_parent_getters_upper': {'anchor_projections': 6, 'tracking_projections': 6},
                      'query_domain_comparisons_upper': {'anchor_projections': 4, 'tracking_projections': 4},
                      'original_projection_retention_upper': {'anchor_projections': 1,
                                                              'tracking_projections': 1},
                      'tracking_garde_validations_upper': {'tracking_projections': 1},
                      'tracking_rule_callbacks_upper': {'tracking_projections': 2},
                      'tracking_scalar_predicates_upper': {'tracking_projections': 4},
                      'tracking_payload_retention_upper': {'tracking_projections': 1},
                      'enclosure_outer_entries_upper': {'enclosures': 1},
                      'enclosure_radius_bridge_entries_upper': {'enclosures': 1},
                      'enclosure_context_extra_getters_upper': {'enclosures': 2},
                      'enclosure_context_constructions_upper': {'enclosures': 1},
                      'enclosure_projection_copies_upper': {'enclosures': 1}},
 'tracking_work': {'standard_error_requests_upper': {'tracked': 1},
                   'standard_error_binary_primitives_upper': {'tracked': 8},
                   'standard_error_square_roots_upper': {'tracked': 2},
                   'standard_error_scalar_constructions_upper': {'tracked': 2},
                   'standard_error_garde_validations_upper': {'tracked': 2},
                   'standard_error_rule_callbacks_upper': {'tracked': 2},
                   'standard_error_predicates_upper': {'tracked': 4},
                   'reference_duration_operations_upper': {'tracked': 1},
                   'reference_duration_getters_upper': {'tracked': 1},
                   'reference_integer_conversions_upper': {'tracked': 1},
                   'reference_conversion_shared_primitives_upper': {'tracked': 3},
                   'numeric_getters_upper': {'tracked': 4},
                   'tracking_binary_primitives_upper': {'tracked': 4},
                   'skew_validations_upper': {'tracked': 1},
                   'final_validations_upper': {'tracked': 1},
                   'rule_callbacks_upper': {'tracked': 5},
                   'scalar_predicates_upper': {'tracked': 10}},
 'chart_roundoff_work': {'transform_requests_upper': {'anchor_projections': 1, 'tracking_projections': 1},
                         'elapsed_requests_upper': {'anchor_projections': 1, 'tracking_projections': 1},
                         'monotonic_getters_upper': {'anchor_projections': 2, 'tracking_projections': 2},
                         'original_projection_getters_upper': {'anchor_projections': 4,
                                                               'tracking_projections': 5},
                         'integer_conversion_requests_upper': {'anchor_projections': 1,
                                                               'tracking_projections': 1},
                         'integer_shared_primitives_upper': {'anchor_projections': 2,
                                                             'tracking_projections': 2},
                         'value_primitives_upper': {'anchor_projections': 5, 'tracking_projections': 6},
                         'total_shared_value_primitives_upper': {'anchor_projections': 7,
                                                                 'tracking_projections': 8},
                         'step_operation_mappings_upper': {'anchor_projections': 5,
                                                           'tracking_projections': 6},
                         'bound_add_requests_upper': {'anchor_projections': 22,
                                                      'tracking_projections': 28,
                                                      'enclosures': 1},
                         'bound_multiply_requests_upper': {'anchor_projections': 10,
                                                           'tracking_projections': 12},
                         'bound_divide_requests_upper': {'anchor_projections': 1, 'tracking_projections': 1},
                         'bound_requests_upper': {'anchor_projections': 33,
                                                  'tracking_projections': 41,
                                                  'enclosures': 1},
                         'bound_binary_primitives_upper': {'anchor_projections': 33,
                                                           'tracking_projections': 41,
                                                           'enclosures': 1},
                         'bound_next_up_requests_upper': {'anchor_projections': 33,
                                                          'tracking_projections': 41,
                                                          'enclosures': 1},
                         'bound_operand_and_result_predicates_upper': {'anchor_projections': 396,
                                                                       'tracking_projections': 492,
                                                                       'enclosures': 12}},
 'chart_batch_work': {'constructor_attempts_upper': {'batches': 1},
                      'initial_capacity_checks_upper': {'batches': 1},
                      'expected_descriptor_getters_upper': {'batches': 4},
                      'expected_float_bit_queries_upper': {'batches': 2},
                      'candidate_constructions_upper': {'batches': 1},
                      'garde_validations_upper': {'batches': 1},
                      'garde_callbacks_upper': {'batches': 1},
                      'relationship_passes_upper': {'batches': 2},
                      'relationship_capacity_checks_upper': {'batches': 2},
                      'header_domain_comparisons_upper': {'batches': 4},
                      'header_domain_getters_upper': {'batches': 4},
                      'entry_visits_upper': {'batches': 32},
                      'entry_group_comparisons_upper': {'batches': 160},
                      'entry_scalar_comparisons_upper': {'batches': 320},
                      'entry_product_getters_upper': {'batches': 256},
                      'entry_float_bit_queries_upper': {'batches': 64},
                      'scratch_initialization_writes_upper': {'batches': 16},
                      'scratch_interval_copy_writes_upper': {'batches': 16},
                      'scratch_interval_getters_upper': {'batches': 17},
                      'scratch_checked_prefixes_upper': {'batches': 1},
                      'legacy_batch_attempts_upper': {'batches': 1},
                      'legacy_batch_staging_initializations_upper': {'batches': 16},
                      'legacy_batch_checked_slots_upper': {'batches': 16},
                      'legacy_batch_label_getters_upper': {'batches': 240},
                      'selection_dispatches_upper': {'selections': 1},
                      'selection_result_assemblies_upper': {'selections': 1},
                      'minimum_constructions_upper': {'minima': 1},
                      'minimum_garde_validations_upper': {'minima': 1},
                      'minimum_rule_callbacks_upper': {'minima': 1},
                      'minimum_range_comparisons_upper': {'minima': 2}},
 'legacy_batch_work': {'constructor_attempts_upper': {'batches': 1},
                       'authored_helper_entries_upper': {'batches': 57},
                       'garde_validations_upper': {'batches': 1},
                       'garde_callbacks_upper': {'batches': 1},
                       'entry_read_attempts_upper': {'batches': 48},
                       'product_getters_upper': {'batches': 176},
                       'scratch_copy_assignments_upper': {'batches': 31}},
 'descriptor_work': {'initial_metadata_capacity_checks_upper': {'advances': 1},
                     'candidate_constructions_upper': {'advances': 1},
                     'garde_validations_upper': {'advances': 1},
                     'garde_callbacks_upper': {'advances': 1},
                     'relationship_passes_upper': {'advances': 2},
                     'descriptor_getters_upper': {'advances': 16},
                     'frame_label_comparisons_upper': {'advances': 8},
                     'domain_instant_comparisons_upper': {'advances': 6},
                     'scalar_bit_queries_upper': {'advances': 8},
                     'scalar_bit_comparisons_upper': {'advances': 4},
                     'batch_header_and_source_frame_getters_upper': {'advances': 8}},
 'principal_receiver_work': {'roster_garde_validations_upper': {'principal_seeds': 1},
                             'roster_garde_callbacks_upper': {'principal_seeds': 1},
                             'roster_relationship_passes_upper': {'principal_seeds': 2},
                             'roster_duplicate_comparisons_upper': {'principal_seeds': 240},
                             'seed_slot_initializations_upper': {'principal_seeds': 16},
                             'seed_source_record_writes_upper': {'principal_seeds': 16},
                             'receiver_garde_validations_upper': {'principal_seeds': 1,
                                                                  'legacy_advances': 1,
                                                                  'chart_advances': 1},
                             'receiver_garde_callbacks_upper': {'principal_seeds': 1,
                                                                'legacy_advances': 1,
                                                                'chart_advances': 1},
                             'receiver_relationship_passes_upper': {'principal_seeds': 2,
                                                                    'legacy_advances': 2,
                                                                    'chart_advances': 2},
                             'receiver_key_comparisons_upper': {'principal_seeds': 4096,
                                                                'legacy_advances': 4096,
                                                                'chart_advances': 4096},
                             'receiver_control_predicates_upper': {'principal_seeds': 1024,
                                                                   'legacy_advances': 1024,
                                                                   'chart_advances': 1024},
                             'receiver_original_projection_visits_upper': {'principal_seeds': 128,
                                                                           'legacy_advances': 128,
                                                                           'chart_advances': 128},
                             'legacy_projection_adapter_getters_upper': {'principal_seeds': 128,
                                                                         'legacy_advances': 128},
                             'receiver_original_product_getters_upper': {'principal_seeds': 768,
                                                                         'legacy_advances': 768,
                                                                         'chart_advances': 768},
                             'source_lookup_slot_visits_upper': {'principal_seeds': 4096,
                                                                 'legacy_advances': 4096,
                                                                 'chart_advances': 4096},
                             'stratum_lookup_slot_visits_upper': {'principal_seeds': 1024,
                                                                  'legacy_advances': 1024,
                                                                  'chart_advances': 1024},
                             'staged_source_field_writes_upper': {'principal_seeds': 256,
                                                                  'legacy_advances': 256,
                                                                  'chart_advances': 256},
                             'staged_state_clone_cells_upper': {'legacy_advances': 16, 'chart_advances': 16},
                             'staged_source_fields_copied_upper': {'legacy_advances': 96,
                                                                   'chart_advances': 96},
                             'staged_fixed_fields_copied_upper': {'legacy_advances': 16,
                                                                  'chart_advances': 16},
                             'witness_slot_initializations_upper': {'legacy_advances': 16,
                                                                    'chart_advances': 16},
                             'witness_membership_visits_upper': {'legacy_advances': 16, 'chart_advances': 16},
                             'ranking_slot_visits_upper': {'legacy_advances': 16, 'chart_advances': 16},
                             'checked_scalar_primitives_upper': {'legacy_advances': 82, 'chart_advances': 82},
                             'scalar_supported_predicates_upper': {'legacy_advances': 492,
                                                                   'chart_advances': 492},
                             'credit_slot_visits_upper': {'legacy_advances': 32, 'chart_advances': 32},
                             'legacy_selection_dispatches_upper': {'legacy_advances': 1},
                             'selection_association_entry_visits_upper': {'legacy_advances': 16,
                                                                          'chart_advances': 16},
                             'selection_association_supporter_probes_upper': {'legacy_advances': 16,
                                                                              'chart_advances': 16},
                             'decision_assemblies_upper': {'legacy_advances': 1, 'chart_advances': 1},
                             'completed_state_commit_attempts_upper': {'legacy_advances': 1,
                                                                       'chart_advances': 1},
                             'chart_projection_adapter_getters_upper': {'chart_advances': 256},
                             'parameter_constructor_attempts_upper': {'principal_parameters': 1},
                             'parameter_garde_validations_upper': {'principal_parameters': 1},
                             'parameter_garde_callbacks_upper': {'principal_parameters': 2},
                             'parameter_scalar_rule_calls_upper': {'principal_parameters': 4},
                             'parameter_scalar_predicates_upper': {'principal_parameters': 16},
                             'stratum_constructor_attempts_upper': {'principal_strata': 1},
                             'stratum_garde_validations_upper': {'principal_strata': 1},
                             'stratum_garde_callbacks_upper': {'principal_strata': 1},
                             'stratum_range_predicates_upper': {'principal_strata': 2}},
 'lifecycle_work': {'lifecycle_parameter_constructor_attempts_upper': {'combination_parameters': 1},
                    'lifecycle_parameter_garde_callbacks_upper': {'combination_parameters': 2},
                    'lifecycle_parameter_predicate_calls_upper': {'combination_parameters': 4},
                    'lifecycle_garde_validate_calls_upper': {'combination_parameters': 1,
                                                             'seeds': 16,
                                                             'advances': 49,
                                                             'reachability_updates': 1,
                                                             'reachability_records': 2},
                    'lifecycle_source_counter_constructions_upper': {'seeds': 16,
                                                                     'advances': 48,
                                                                     'reachability_records': 1},
                    'lifecycle_source_counter_predicate_calls_upper': {'seeds': 32,
                                                                       'advances': 96,
                                                                       'reachability_records': 2},
                    'lifecycle_source_slot_writes_upper': {'seeds': 16,
                                                           'advances': 48,
                                                           'reachability_records': 1},
                    'lifecycle_internal_product_getter_requests_upper': {'seeds': 16,
                                                                         'advances': 8192,
                                                                         'reachability_records': 64},
                    'lifecycle_checked_next_calls_upper': {'advances': 1, 'reachability_records': 2},
                    'lifecycle_compatibility_checked_binary_calls_upper': {'advances': 62},
                    'lifecycle_compatibility_source_visits_upper': {'advances': 16},
                    'lifecycle_source_index_label_probes_upper': {'advances': 256,
                                                                  'reachability_records': 32},
                    'lifecycle_witness_membership_label_probes_upper': {'advances': 768},
                    'lifecycle_output_contributor_label_probes_upper': {'advances': 512},
                    'lifecycle_output_predicate_calls_upper': {'advances': 2},
                    'lifecycle_staged_source_copy_cells_upper': {'advances': 16},
                    'bridge_admission_entry_attempts_upper': {'advances': 1},
                    'bridge_complete_receiving_passes_upper': {'advances': 2},
                    'bridge_receiving_pointer_comparisons_upper': {'advances': 512},
                    'bridge_receiving_label_comparisons_upper': {'advances': 786},
                    'bridge_mask_admission_slot_visits_upper': {'advances': 32},
                    'bridge_reduction_entry_attempts_upper': {'advances': 1},
                    'bridge_reduction_checked_binary_calls_upper': {'advances': 293},
                    'bridge_reduction_checked_root_calls_upper': {'advances': 3},
                    'bridge_reduction_positive_denominator_guards_upper': {'advances': 48},
                    'bridge_reduction_supporter_visits_upper': {'advances': 16},
                    'bridge_record_lookup_visits_upper': {'advances': 768},
                    'bridge_internal_product_getter_requests_upper': {'advances': 12288},
                    'kernel_final_uncertainty_predicate_calls_upper': {'advances': 3},
                    'lifecycle_state_equality_source_cells_upper': {'equalities': 16},
                    'lifecycle_state_equality_fixed_fields_upper': {'equalities': 4},
                    'lifecycle_raw_ordinal_constructor_attempts_upper': {'reachability_updates': 1},
                    'lifecycle_event_admission_predicate_calls_upper': {'reachability_records': 2}}}

POINT_CEILINGS: dict[str, int] = {'kernel_checked_binary_calls_upper': 293,
 'kernel_checked_square_root_calls_upper': 3,
 'kernel_positive_denominator_guards_upper': 48,
 'kernel_supporter_reduction_visits_upper': 16,
 'kernel_record_lookup_visits_upper': 256,
 'kernel_metadata_pointer_comparisons_upper': 256,
 'kernel_metadata_label_comparisons_upper': 393,
 'kernel_internal_product_getter_requests_upper': 4096,
 'kernel_final_uncertainty_predicate_calls_upper': 3}

GROUP_FIELDS["reference_input_work"] = (*REFERENCE_FIELDS, "relationship_passes_upper",
    "garde_validations_upper", "garde_callbacks_upper", "chart_identity_checks_upper",
    "upstream_getter_requests_upper", "doctest_compilation_cases", "doctest_compile_fail_cases",
    "doctest_executed_examples")
FIELDS = tuple(PREFIX + group + "." + field for group, fields in GROUP_FIELDS.items() for field in fields)

SELECTED_OWNERS = {'combination': 'crates/time-core/src/combination.rs',
 'combination_admission': 'crates/time-core/src/combination/admission.rs',
 'combination_lifecycle': 'crates/time-core/src/combination/lifecycle.rs',
 'chart': 'crates/time-core/src/regression/chart.rs',
 'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
 'chart_combination_roundoff': 'crates/time-core/src/combination/chart/roundoff.rs',
 'chart_combination_receiving': 'crates/time-core/src/combination/chart/receiving.rs',
 'chart_combination_reduction': 'crates/time-core/src/combination/chart/reduction.rs',
 'chart_combination_lifecycle': 'crates/time-core/src/combination/lifecycle/chart.rs'}

TEST_GROUPS = (('combination::chart::roundoff::tests::companion_construction_preserves_supported_bits_and_rejects_invalid_fields',
  'combination::chart::roundoff::tests::positive_margin_keeps_ties_and_exact_subnormal_differences',
  'combination::chart::roundoff::tests::positive_division_consumes_the_exact_smallest_subnormal_margin',
  'combination::chart::roundoff::tests::companion_operations_contain_independent_interval_endpoints',
  'combination::chart::roundoff::tests::exact_input_ties_and_cancellation_retain_separate_rounding',
  'combination::chart::roundoff::tests::sixteen_stored_tenths_include_new_rounded_reduction_error',
  'combination::chart::roundoff::tests::reciprocal_rate_weight_retains_the_sd_square_error',
  'combination::chart::roundoff::tests::companion_failures_preserve_actual_point_and_bound_owners',
  'combination::chart::roundoff::tests::companion_diagnostics_are_bounded_and_keep_real_source_chains'),
 ('combination::admission::chart_receiving::tests::chart_tracking_receiver_keeps_actual_slots_and_complete_witness',
  'combination::admission::chart_receiving::tests::chart_tracking_receiver_rejects_wrong_relationships',
  'combination::admission::chart_receiving::tests::chart_tracking_receiver_requires_excluded_and_retained_tracking',
  'combination::admission::chart_receiving::tests::chart_tracking_receiver_absence_requires_empty_metadata',
  'combination::admission::chart_receiving::tests::chart_tracking_reduction_contains_exact_stored_input_means',
  'combination::admission::chart_receiving::tests::chart_tracking_reduction_failure_preserves_prior_sums',
  'combination::admission::chart_receiving::tests::chart_tracking_singleton_copies_zero_denominator_attachment_bits'),
 ('chart_combination_retains_real_provenance_and_observation_credit',
  'chart_combination_failure_rolls_back_before_same_event_retry'),
 ('unequal_original_weights_contain_exact_stored_input_means',
  'sixteen_contributors_bound_new_reduction_roundoff',
  'original_population_controls_singleton_and_filtered_one_retry',
  'bound_only_failure_rolls_back_and_allows_same_observation_retry'),
 ('combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_preserve_complete_relationships',
  'combination::lifecycle::chart::guard_tests::chart_raw_count_seventeen_cannot_reuse_sixteen_slots'))
S158_ORDINARY_TESTS = tuple(name for group in TEST_GROUPS for name in group)
REFERENCE_ORDINARY_TESTS = (
    "reference_input_readiness::genuine_ready_input_is_readonly_and_replay_has_no_new_credit",
    "reference_input_readiness::unavailable_and_waiting_outcomes_reject_without_reviving_readiness",
    "reference_input_readiness::foreign_chart_instances_reject_beside_the_bound_original",
    "reference_input_readiness::feedback_pairs_and_repeats_never_restore_an_invalidated_input",
)
CURRENT_ORDINARY_TESTS = (*S158_ORDINARY_TESTS, *REFERENCE_ORDINARY_TESTS)

DOCTEST_LINES = (163,)
REFERENCE_DOCTEST_LINES = (229, 248, 266)
REFERENCE_COMPILE_FAIL_LINES = (229, 248)

class Recipe(TypedDict):
    """Frozen fixture attempts and direct private entries, never observed counters."""
    fixture: dict[str, int]
    companion: tuple[int, ...]
    receiving: tuple[int, ...]
    reduction: tuple[int, ...]
    guards: dict[str, int]
    direct_selections: int
    reference_input: dict[str, int]


class ContractOwner(NamedTuple):
    """Frozen legal-source provider; the caller separately admits module bytes."""
    fields: tuple[str, ...]
    expected: Callable[[], dict[str, int]]
    source: Callable[[], str]


class OutcomeSummary(TypedDict):
    """Actual assertion outcomes without conversion to operation observations."""
    status: Literal["complete", "incomplete"]
    complete: bool
    outcomes: dict[str, str]
    missing: list[str]
    failed: list[str]
    expected: int
    scope: str


def is_exact_dict(value: object) -> TypeGuard[dict[str, object]]:
    """Narrow external mappings without introducing dynamic Any."""
    return type(value) is dict


def _integer(value: object) -> int:
    """Admit exact nonnegative bounded accounting units, excluding bool."""
    if type(value) is not int or not 0 <= value <= MAX_UNITS:
        raise Incomplete("Source work requires exact nonnegative bounded integers.")
    return value


def _add(*values: int) -> int:
    """Reject aggregate overflow before publishing a prospective reservation."""
    return _integer(sum(_integer(value) for value in values))


def _mul(left: int, right: int) -> int:
    """Keep source multiplication inside the same finite accounting envelope."""
    return _integer(_integer(left) * _integer(right))


def _mapping(value: object, fields: tuple[str, ...]) -> dict[str, int]:
    """Require a closed exact field set before any accounting projection."""
    if not is_exact_dict(value) or set(value) != set(fields):
        raise Incomplete("Source work has unknown or missing fields.")
    return {field: _integer(value[field]) for field in fields}


def _fixture(**provided: int) -> dict[str, int]:
    """Build immutable source ceilings from the fixed fixture census."""
    fields = GROUP_FIELDS["fixture"]
    if set(provided) - set(fields):
        raise Incomplete("Fixture recipe has an unknown field.")
    result = dict.fromkeys(fields, 0)
    result.update(provided)
    appends = result["appends"]
    result["observations"] = provided.get("observations", appends)
    result["coordinates"] = provided.get("coordinates", _mul(2, appends))
    result["exchanges"] = provided.get("exchanges", appends)
    result["frames_upper"] = provided.get("frames_upper", result["charts"])
    result["domains_upper"] = provided.get("domains_upper", result["charts"])
    projections = _add(result["anchor_projections"], result["tracking_projections"])
    result["prepares"] = provided.get("prepares", projections)
    result["known_prepares_upper"] = provided.get("known_prepares_upper", result["prepares"])
    result["terms"] = provided.get("terms", projections)
    result["chart_enclosures"] = provided.get("chart_enclosures", projections)
    seeds = _add(result["principal_seeds"], result["combination_seeds"])
    result["principal_parameters"] = provided.get("principal_parameters", seeds)
    result["combination_parameters"] = provided.get("combination_parameters", result["combination_seeds"])
    result["minima"] = provided.get("minima", seeds)
    result["chart_reductions"] = provided.get("chart_reductions", result["combination_advances"])
    result["singleton_copies"] = provided.get("singleton_copies", result["combination_advances"])
    if result["histories"] or result["charts"] or seeds:
        result["labels_upper"] = provided.get("labels_upper", _add(
            _mul(8, result["histories"]), _mul(4, result["charts"]), _mul(8, seeds), result["getters_upper"]))
        result["contexts_upper"] = provided.get("contexts_upper", _add(result["histories"], _mul(2, projections)))
        result["monotonic_upper"] = provided.get("monotonic_upper", _add(
            result["observations"], _mul(3, result["charts"]), _mul(4, projections),
            _mul(4, _add(result["raw_batches"], result["chart_batches"])), _mul(8, seeds)))
    return _mapping(result, fields)


GUARD_FIELDS = ("quantity_predicates", "estimate_predicates", "principal_matches",
                "contributor_predicates", "contributor_garde", "outcome_predicates", "outcome_garde")


def _guards(**provided: int) -> dict[str, int]:
    """Close direct raw-guard attempts independently of real lifecycle advances."""
    if set(provided) - set(GUARD_FIELDS):
        raise Incomplete("A direct guard recipe has an unknown field.")
    return _mapping({**dict.fromkeys(GUARD_FIELDS, 0), **provided}, GUARD_FIELDS)


def _reference_input(**provided: int) -> dict[str, int]:
    """Close source attempts and healthy success facts, without claiming execution."""
    if set(provided) - set(REFERENCE_FIELDS):
        raise Incomplete("A reference-input recipe has an unknown field.")
    return _mapping({**dict.fromkeys(REFERENCE_FIELDS, 0), **provided}, REFERENCE_FIELDS)


def _recipe(fixture: dict[str, int], *, companion: tuple[int, ...] = (0, 0, 0, 0, 0, 0),
            receiving: tuple[int, ...] = (0, 0, 0, 0),
            reduction: tuple[int, ...] = (0, 0, 0), guards: dict[str, int] | None = None,
            direct_selections: int = 0, reference_input: dict[str, int] | None = None) -> Recipe:
    """Keep direct entries separate from ordinary source fixture attempts."""
    if (any(type(values) is not tuple for values in (companion, receiving, reduction))
            or len(companion) != 6 or len(receiving) != 4 or len(reduction) != 3):
        raise Incomplete("A direct source recipe has the wrong fixed shape.")
    return {"fixture": _mapping(fixture, GROUP_FIELDS["fixture"]),
            "companion": tuple(_integer(value) for value in companion),
            "receiving": tuple(_integer(value) for value in receiving),
            "reduction": tuple(_integer(value) for value in reduction),
            "guards": _guards() if guards is None else _mapping(guards, GUARD_FIELDS),
            "direct_selections": _integer(direct_selections),
            "reference_input": (_reference_input() if reference_input is None
                                else _mapping(reference_input, REFERENCE_FIELDS))}


# Companion tuple: input construction, Add, Subtract, Multiply, Divide, direct margin.
# Receiving tuple: holder, admission, direct typed predicate, direct Garde.
# Reduction tuple: direct with_record, finish, singleton. Every rejected prefix
# keeps its complete reservation. Getter limits bound test-side access/bit/oracle
# sites; delegated owner getters have their independent, larger named ceilings.
_COMPANION_RECIPES = (
    (34, 0, 0, 0, 0, 0), (0, 0, 0, 0, 0, 16), (3, 0, 0, 0, 2, 0),
    (4, 2, 2, 2, 2, 0), (4, 1, 1, 1, 0, 0), (4, 32, 0, 16, 1, 0),
    (2, 0, 0, 1, 1, 0), (6, 1, 0, 1, 1, 0), (1, 0, 0, 0, 0, 0),
)


def s158_case_recipes() -> dict[str, Recipe]:
    """Return a fresh complete census of the twenty-four frozen new outcomes."""
    rows: list[Recipe] = [_recipe(_fixture(getters_upper=128), companion=row)
                          for row in _COMPANION_RECIPES]
    receiver = (
        _recipe(_fixture(histories=2, appends=6, charts=2, tracking_projections=2, raw_enclosures=2,
                         raw_batches=1, chart_batches=1, principal_seeds=1, strata=4,
                         chart_principal_advances=2, getters_upper=1024), receiving=(2, 2, 0, 0)),
        # A same-label copied principal reuses the genuine entries/holder. Its
        # one raw Garde and one typed admission add12 direct getter requests,
        # two complete receiving-pass uppers and no new producer or lifecycle.
        _recipe(_fixture(histories=6, appends=18, charts=4, tracking_projections=6, raw_enclosures=3,
                         raw_batches=1, chart_batches=1, principal_seeds=1, strata=6,
                         chart_principal_advances=1, getters_upper=2060), receiving=(1, 6, 10, 4)),
        _recipe(_fixture(histories=2, appends=6, charts=1, tracking_projections=1, anchor_projections=1,
                         raw_enclosures=2, raw_batches=1, chart_batches=1, principal_seeds=1,
                         strata=4, chart_principal_advances=2, getters_upper=512), receiving=(2, 0, 0, 0)),
        _recipe(_fixture(histories=2, appends=6, charts=1, tracking_projections=1, anchor_projections=2,
                         raw_enclosures=2, raw_batches=1, chart_batches=1, principal_seeds=1,
                         strata=2, chart_principal_advances=1, getters_upper=512), receiving=(1, 3, 0, 0)),
        _recipe(_fixture(histories=2, appends=6, charts=1, tracking_projections=2, raw_enclosures=2,
                         raw_batches=1, chart_batches=1, principal_seeds=1, strata=4,
                         chart_principal_advances=1, getters_upper=2048), receiving=(1, 1, 0, 0), reduction=(2, 1, 1)),
        _recipe(_fixture(histories=2, appends=6, charts=1, tracking_projections=2, raw_enclosures=2,
                         raw_batches=1, chart_batches=1, principal_seeds=1, strata=4,
                         chart_principal_advances=1, getters_upper=512), receiving=(1, 1, 0, 0), reduction=(2, 2, 0)),
        _recipe(_fixture(histories=1, appends=3, charts=1, tracking_projections=1, raw_enclosures=1,
                         raw_batches=1, chart_batches=1, principal_seeds=1, strata=2,
                         chart_principal_advances=1, getters_upper=512), receiving=(1, 1, 0, 0), reduction=(0, 0, 1)),
    )
    rows.extend(receiver)
    rows.extend((
        _recipe(_fixture(histories=2, appends=7, charts=1, tracking_projections=6, raw_enclosures=2,
                         raw_batches=1, chart_batches=3, combination_seeds=1, strata=2,
                         combination_advances=4, getters_upper=2048)),
        _recipe(_fixture(histories=2, appends=7, charts=1, tracking_projections=5, anchor_projections=1,
                         raw_enclosures=4, raw_batches=2, chart_batches=3, combination_seeds=2,
                         strata=2, combination_advances=6, state_equalities=4, getters_upper=2048)),
        _recipe(_fixture(histories=2, appends=6, charts=1, tracking_projections=2, raw_enclosures=2,
                         raw_batches=1, chart_batches=1, combination_seeds=1, strata=2,
                         combination_advances=1, getters_upper=2048)),
        _recipe(_fixture(histories=16, appends=48, charts=1, tracking_projections=16, raw_enclosures=16,
                         raw_batches=1, chart_batches=1, combination_seeds=1, strata=16,
                         combination_advances=1, getters_upper=4096)),
        _recipe(_fixture(histories=2, appends=6, charts=1, tracking_projections=3, raw_enclosures=5,
                         raw_batches=3, chart_batches=3, combination_seeds=3, strata=5,
                         combination_advances=5, state_equalities=4, getters_upper=2048)),
        _recipe(_fixture(histories=2, appends=6, charts=2, tracking_projections=4, raw_enclosures=4,
                         raw_batches=2, chart_batches=2, combination_seeds=2, strata=4, minima=3,
                         combination_advances=4, state_equalities=4, getters_upper=2048), direct_selections=1),
        _recipe(_fixture(histories=2, appends=6, charts=1, prepares=2, terms=2, tracking_projections=2,
                         raw_enclosures=2, raw_batches=1, chart_batches=2, combination_seeds=1,
                         strata=2, combination_advances=2, chart_reductions=3, singleton_copies=0,
                         labels_upper=14, contexts_upper=2, monotonic_upper=16, getters_upper=512),
                receiving=(1, 1, 0, 0),
                guards=_guards(quantity_predicates=49, estimate_predicates=49, principal_matches=2,
                               contributor_predicates=3, contributor_garde=3,
                               outcome_predicates=49, outcome_garde=49)),
        # Sixteen real histories and one checked principal advance support an
        # unpublished raw17-count estimate. No combination transition/reduction
        # or mathematically validated output is claimed for the literal values.
        _recipe(_fixture(histories=16, appends=48, charts=1, prepares=16, terms=16, tracking_projections=16,
                         raw_enclosures=16, raw_batches=1, chart_batches=1, combination_seeds=1,
                         strata=16, chart_principal_advances=1, combination_advances=0,
                         chart_reductions=0, singleton_copies=0, labels_upper=70, contexts_upper=16,
                         monotonic_upper=85, getters_upper=512),
                guards=_guards(quantity_predicates=2, estimate_predicates=2)),
    ))
    if len(rows) != len(S158_ORDINARY_TESTS):
        raise Incomplete("The complete new test census and source recipes differ.")
    return dict(zip(S158_ORDINARY_TESTS, rows, strict=True))


def case_recipes() -> dict[str, Recipe]:
    """Append four genuine reference-input fixtures to unchanged S158 recurrences.

    Each test starts with two real three-row histories and one actual seed.
    The first appends once more and projects both sources again for round two.
    Foreign descriptors add eight chart constructors, never another fit.
    Readiness/admission counts are source bounds, not callback observations.
    """
    rows = s158_case_recipes()
    fixtures = ((7, 1, 1, 1, 4, 2, 3), (6, 1, 1, 1, 2, 2, 3),
                (6, 9, 6, 3, 2, 1, 1), (6, 1, 1, 1, 2, 1, 1))
    direct = ((1, 7, 6, 0, 5, 6, 6, 6, 1, 2), (1, 4, 5, 1, 1, 1, 1, 0, 0, 0),
              (9, 40, 32, 8, 16, 16, 16, 0, 0, 0), (9, 63, 27, 45, 5, 5, 5, 0, 0, 0))
    for name, values, calls in zip(REFERENCE_ORDINARY_TESTS, fixtures, direct, strict=True):
        appends, charts, frames, domains, projections, batches, advances = values
        rows[name] = _recipe(_fixture(
            histories=2, appends=appends, charts=charts, frames_upper=frames, domains_upper=domains,
            tracking_projections=projections, raw_enclosures=2, raw_batches=1, chart_batches=batches,
            combination_seeds=1, strata=2, combination_advances=advances, getters_upper=1024),
            reference_input=dict(zip(REFERENCE_FIELDS, calls, strict=True)))
    return rows


def case_ceilings() -> dict[str, Recipe]:
    """The fixed source attempts are upper ceilings, never native call counts."""
    return case_recipes()


def _merge_recipes(recipes: list[Recipe]) -> Recipe:
    """Sum disjoint source recipes without refunding failed/missing outcomes."""
    fixture = dict.fromkeys(GROUP_FIELDS["fixture"], 0)
    direct = [[0] * 6, [0] * 4, [0] * 3]
    guards = _guards()
    reference = _reference_input()
    selections = 0
    for row in recipes:
        for key, value in row["fixture"].items():
            fixture[key] = _add(fixture[key], value)
        for index, values in enumerate((row["companion"], row["receiving"], row["reduction"])):
            for column, value in enumerate(values):
                direct[index][column] = _add(direct[index][column], value)
        for field, amount in _mapping(row["guards"], GUARD_FIELDS).items():
            guards[field] = _add(guards[field], amount)
        for field, amount in _mapping(row["reference_input"], REFERENCE_FIELDS).items():
            reference[field] = _add(reference[field], amount)
        selections = _add(selections, row["direct_selections"])
    return _recipe(fixture, companion=tuple(direct[0]), receiving=tuple(direct[1]),
                   reduction=tuple(direct[2]), guards=guards, direct_selections=selections,
                   reference_input=reference)


def _delegate_inputs(recipe: Recipe) -> dict[str, dict[str, int]]:
    """Project each actual callee's input schema once, without duplicate producers."""
    c = recipe["fixture"]
    projections = _add(c["anchor_projections"], c["tracking_projections"])
    seeds = _add(c["principal_seeds"], c["combination_seeds"])
    chart_advances = _add(c["chart_principal_advances"], c["combination_advances"])
    selections = _add(chart_advances, recipe["direct_selections"])
    adaptive = dict.fromkeys(GROUP_FIELDS["adaptive_call"], 0)
    adaptive.update(adaptive_histories=c["histories"], adaptive_observations=c["observations"], adaptive_appends=c["appends"])
    projection = dict.fromkeys(GROUP_FIELDS["projection_call"], 0)
    projection.update(terms=c["terms"], projections=projections)
    enclosure = dict.fromkeys(GROUP_FIELDS["enclosure_call"], 0)
    enclosure["enclosures"] = _add(c["chart_enclosures"], c["raw_enclosures"])
    chart = dict.fromkeys(DELEGATE_INPUTS["chart_owner_work"], 0)
    chart.update(frames_upper=c["frames_upper"], domains_upper=c["domains_upper"], charts=c["charts"],
                 prepares=c["prepares"], known_prepares_upper=c["known_prepares_upper"],
                 anchor_projections=c["anchor_projections"], tracking_projections=c["tracking_projections"], enclosures=c["chart_enclosures"])
    return {"adaptive_work": adaptive, "projection_work": projection,
            "adapter_work": {"fixed": 0, "adaptive": projections}, "enclosure_work": enclosure,
            "selection_work": {"evaluations": selections, "batches": _add(c["chart_batches"], c["raw_batches"])},
            "chart_owner_work": chart, "tracking_work": {"tracked": c["tracking_projections"]},
            "chart_roundoff_work": dict(chart),
            "chart_batch_work": {"batches": c["chart_batches"], "selections": selections, "minima": c["minima"]},
            "legacy_batch_work": {"batches": c["raw_batches"]}, "descriptor_work": {"advances": chart_advances},
            "principal_receiver_work": {"principal_seeds": seeds, "legacy_advances": c["principal_advances"],
                                         "chart_advances": chart_advances, "principal_parameters": c["principal_parameters"], "principal_strata": c["strata"]},
            "lifecycle_work": {"combination_parameters": c["combination_parameters"], "seeds": c["combination_seeds"],
                               "advances": c["combination_advances"], "equalities": c["state_equalities"],
                               "reachability_updates": c["reachability_updates"], "reachability_records": c["reachability_records"]}}


def _delegated(owners: WorkOwners, recipe: Recipe) -> dict[str, dict[str, int]]:
    """Require actual injected results to preserve every frozen named ceiling."""
    calls = _delegate_inputs(recipe)
    result: dict[str, dict[str, int]] = {}
    functions: dict[str, WorkFunction] = {
        "adaptive_work": owners.adaptive_work, "projection_work": owners.projection_work,
        "adapter_work": owners.adapter_work, "enclosure_work": owners.enclosure_work,
        "selection_work": owners.selection_work, "chart_owner_work": owners.chart_owner_work,
        "tracking_work": owners.tracking_work, "chart_roundoff_work": owners.chart_roundoff_work,
        "chart_batch_work": owners.chart_batch_work, "legacy_batch_work": owners.legacy_batch_work,
        "descriptor_work": owners.descriptor_work, "principal_receiver_work": owners.principal_receiver_work,
        "lifecycle_work": owners.lifecycle_work,
    }
    for group, inputs in calls.items():
        if group == "adaptive_work":
            # Preserve the adopted helper's13-history/200-row input admission.
            # Disjoint partitions add its unchanged linear named source bounds;
            # extending a caller census never widens that owner's old ceiling.
            actual = dict.fromkeys(GROUP_FIELDS[group], 0)
            caps = {"adaptive_histories": 13, "adaptive_observations": 200, "adaptive_appends": 200}
            boundaries = {0}
            for name, cap in caps.items():
                count, remainder = divmod(inputs[name], cap)
                boundaries.update((count, count + int(remainder != 0)))
            ordered = sorted(boundaries)
            # At most six intervals, even for hostile near-envelope integers.
            # Each interval represents identical complete, partial or zero
            # disjoint partitions, so only its multiplicity is multiplied.
            for start, stop in zip(ordered, ordered[1:]):
                part = dict.fromkeys(GROUP_FIELDS["adaptive_call"], 0)
                for name, cap in caps.items():
                    part[name] = min(max(inputs[name] - start * cap, 0), cap)
                projected = _mapping(functions[group](part), GROUP_FIELDS[group])
                for field in GROUP_FIELDS[group]:
                    actual[field] = _add(actual[field], _mul(stop - start, projected[field]))
            if not any(inputs.values()):
                actual = _mapping(functions[group](dict(inputs)), GROUP_FIELDS[group])
        else:
            actual = _mapping(functions[group](dict(inputs)), GROUP_FIELDS[group])
        for field in GROUP_FIELDS[group]:
            floor = 0
            for source, coefficient in DELEGATE_FLOORS[group].get(field, {}).items():
                floor = _add(floor, _mul(inputs[source], coefficient))
            if actual[field] < floor:
                raise Incomplete("A delegated owner discounted a prior named source ceiling.")
        result[group] = actual
    fits, points = result["adaptive_work"]["charged_fit_work_upper"], result["adaptive_work"]["point_admissions_upper"]
    if fits < _mul(62, recipe["fixture"]["appends"]) or points < _mul(2141, recipe["fixture"]["appends"]):
        raise Incomplete("Adaptive source fitting was favorably discounted.")
    if any(result[group][key] for group in ("projection_work", "enclosure_work")
           for key in ("charged_fit_work_upper", "point_admissions_upper")):
        raise Incomplete("A delegated nonfitting projection duplicated producer charges.")
    return result


AGGREGATE_FIELDS: tuple[str, ...] = ("charged_fit_work_upper", "point_admissions_upper",
                    "standard_errors.geometry.report_copies_upper", "standard_errors.geometry.logical_bytes_upper")
PRIOR_ADAPTER_KEYS = (
    "chart_principal.prior_adapter.seeds_source_upper",
    "chart_principal.prior_adapter.advances_source_upper",
    "chart_principal.prior_adapter.direct_completions_source_upper",
    "combination.call.combinations", "lifecycle.work.bridge_admission_entry_attempts_upper")
PRIOR_BOUND_KEYS = ("chart.roundoff_private.bound_requests_upper", "chart.roundoff_work.bound_requests_upper",
                    "chart_batch.roundoff_work.bound_requests_upper", "chart_principal.roundoff_work.bound_requests_upper")
PRIOR_LIFECYCLE_KEYS = ("lifecycle.work.lifecycle_output_predicate_calls_upper",
                        "lifecycle.work.lifecycle_internal_product_getter_requests_upper",
                        "lifecycle.work.bridge_reduction_entry_attempts_upper")
LEGAL_FIELDS = ("labels", "contexts", "monotonic", "coordinates", "exchanges", "observations", "histories", "appends",
                "frames", "domains", "charts", "prepares", "terms", "tracked_projections", "chart_enclosures",
                "raw_enclosures", "raw_batches", "principal_parameters", "combination_parameters", "minima", "strata",
                "combination_seeds", "chart_batches", "chart_advances", "bootstrap_getters", "outcome_getters",
                "estimate_getters", "iterator_next", "state_equalities")
LEGAL_EXPECTED = (5, 2, 4, 1, 1, 3, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 5, 17, 21, 2, 2)
LEGAL_SOURCE_SHA256 = "28d8e269bffaf9bb808d826ff31607966c0d4171ab4d60293e00417bb6fff50f"
REFERENCE_LEGAL_FIELDS = (*LEGAL_FIELDS, "reference_states", "reference_readiness", "reference_admissions",
                          "reference_feedback", "reference_combined", "reference_rounds", "reference_estimate_getters")
REFERENCE_LEGAL_EXPECTED = (*LEGAL_EXPECTED, 1, 2, 4, 4, 1, 2, 2)
REFERENCE_LEGAL_SOURCE_SHA256 = "c7ea8535d63efe3003ba6c83c63f38bb853643f6a7fb988674ea12460d49b1df"


def _old_vector(value: object) -> dict[str, int]:
    """Keep the old complete namespace closed against collisions and bad units.

    The existing coordinator separately admits the complete prior2214-key schema;
    this extension owns its width, integer envelope, named dependencies and all
    disjoint new keys. It does not replace that prior source/schema admission.
    """
    if not is_exact_dict(value) or len(value) != BASE_BUDGET_FIELDS:
        raise Incomplete("The complete independently admitted prior2214-field vector is required.")
    if any(type(key) is not str or key.startswith(PREFIX) for key in value):
        raise Incomplete("Prior and current source namespaces must be disjoint.")
    if any(key not in value for key in (*AGGREGATE_FIELDS, *PRIOR_ADAPTER_KEYS, *PRIOR_BOUND_KEYS, *PRIOR_LIFECYCLE_KEYS)):
        raise Incomplete("A prior shared-adapter source dependency is absent.")
    return {key: _integer(amount) for key, amount in value.items()}


def _prior_work(prior: dict[str, int], *, external_legal: bool, chart_advances: int) -> dict[str, int]:
    """Fund new shared production work on every older admitted source recipe.

    Old package100seeds/304advances/7directcompletions and60kernel calls are
    independently preserved S157 source ceilings, not a file-level score credit.
    Old legal source is2seeds/2advances/one kernel. Every potential admission
    funds a new16-slot cache,64 materialization getters, and full scalar value
    extraction. DecisionFacts/shared predicates/transaction adapters reserve128
    getter visits and three entries per attempt. Direct older raw-output and
    estimate controls additionally retain the complete prior lifecycle getter
    and output-predicate ceilings for their new facts/view dispatch. Direct old
    reductions additionally fund sixteen scalar extractions each. Older ceilings
    remain separately present. The chart evaluate_up bridge adds one entry for
    every formerly reserved bound call, with no new fitting or numerical work.
    New chart advances additionally enter the same three transaction/preparation/
    emission adapters and fund128 facts/view/struct-materialization requests per
    attempt. Their complete staging, predicates and Garde stay in lifecycle_work
    and output_work; those existing units are not duplicated here.
    """
    floors = (2, 2, 0, 1) if external_legal else (100, 304, 7, 60)
    seeds, advances, completions, combinations, bridges = (prior[key] for key in PRIOR_ADAPTER_KEYS)
    if any(value < floor for value, floor in zip((seeds, advances, completions, combinations), floors, strict=True)):
        raise Incomplete("A prior whole-package or external-legal recipe was favorably discounted.")
    operations = _add(seeds, advances, completions)
    raw_outputs, lifecycle_getters, reductions = (prior[key] for key in PRIOR_LIFECYCLE_KEYS)
    admissions = _add(operations, bridges, combinations)
    return dict(zip(GROUP_FIELDS["prior_adapter_work"], (
        seeds, advances, completions, _mul(16, admissions), _mul(64, admissions),
        _mul(16, _add(admissions, reductions)), _add(_mul(3, _add(advances, completions, chart_advances)), raw_outputs),
        _add(_mul(128, _add(operations, chart_advances)), lifecycle_getters),
        _add(*(prior[key] for key in PRIOR_BOUND_KEYS))), strict=True))


def _companion_work(recipe: Recipe, records: int, finishes: int, empties: int) -> dict[str, int]:
    """Reserve all separately rounded point and outward-bound primitives.

    Each ChartSums record uses four Adds, three Multiplies and two Divides;
    finish uses two Divides. Four zero companions initialize a reduction and
    five admitted/exact inputs per record reserve eight constructors. Failure
    prefixes reserve the whole request, never a measured successful prefix.
    Add/Sub have five bound primitives, Multiply nine, Divide seven. Every bound
    primitive retains the older12-predicate ceiling plus one successor. Margin
    guards separately reserve their finite/positive checks and predecessor.
    """
    inputs, adds, subtracts, multiplies, divides, margins = recipe["companion"]
    inputs = _add(inputs, _mul(8, records), _mul(4, empties))
    adds = _add(adds, _mul(4, records))
    multiplies = _add(multiplies, _mul(3, records))
    divides = _add(divides, _mul(2, records), _mul(2, finishes))
    results = _add(adds, subtracts, multiplies, divides)
    constructions = _add(inputs, results)
    margins = _add(margins, divides)
    bounds = _add(_mul(5, _add(adds, subtracts)), _mul(9, multiplies), _mul(7, divides))
    return dict(zip(GROUP_FIELDS["companion_work"], (
        inputs, results, inputs, adds, subtracts, multiplies, divides, margins,
        _add(results, margins), bounds, _add(_mul(12, bounds), _mul(8, margins)),
        _add(bounds, margins), _mul(2, constructions), _mul(2, constructions),
        constructions, _mul(2, constructions)), strict=True))


def _new_groups(recipe: Recipe, *, ordinary: int, docs: bool) -> dict[str, dict[str, int]]:
    """Reserve capacity16 on every driver, including retained and failed attempts.

    Receiver admission invokes two complete validation passes. Each pass may
    compare16wrappers against16witnesses,120duplicate pairs and16attachments;
    each record separately compares raw frame, generation and projected_at.
    Each holder initialization funds16 defaults plus16 occupied-record fills.
    Each cache initialization funds32 array defaults plus16 occupied-label
    assignments; the16 original-record materializations have their own field.
    Each reduction funds16 label defaults plus16 contributor-prefix writes.
    Reduction drivers reserve the existing293binary/3root/48denominator kernel
    ceiling even for singleton, failure or retained decisions; scalar extraction
    uses the old4096getter ceiling. Complete output validates twice, including
    descriptor, actual pointers, raw and target contexts, scalar/bound fields and
    capacity16 membership. Observer upper scope reserves4096integer oracle ops
    and4096assertion/getter/diagnostic operations per outcome, beyond fixture
    getters and all separately delegated product getters. A direct quantity,
    estimate or principal-association probe reserves one full chart-output
    envelope, including nested raw subpredicates, without claiming an advance
    or completed output. Direct mask Garde and typed predicates each reserve
    one complete sixteen-slot pass. None of these bounds are measurements.
    """
    c = recipe["fixture"]
    holders, admissions, raw_passes, raw_garde = recipe["receiving"]
    holders, admissions = _add(holders, c["combination_advances"]), _add(admissions, c["combination_advances"])
    passes = _add(_mul(2, admissions), raw_passes, raw_garde)
    records, finishes, singletons = recipe["reduction"]
    drivers = c["chart_reductions"]
    records = _add(records, _mul(16, drivers))
    finishes = _add(finishes, drivers)
    singletons = _add(singletons, c["singleton_copies"])
    complete = c["combination_advances"]
    guards = _mapping(recipe["guards"], GUARD_FIELDS)
    direct_output = _add(*(guards[field] for field in (
        "quantity_predicates", "estimate_predicates", "principal_matches",
        "outcome_predicates", "outcome_garde")))
    direct_masks = _add(guards["contributor_predicates"], guards["contributor_garde"])
    post = _add(_mul(2, complete), direct_output)
    point_attempts = _add(drivers, recipe["reduction"][0], recipe["reduction"][1], recipe["reduction"][2])
    return {
        "point_work": {key: _mul(value, point_attempts) for key, value in POINT_CEILINGS.items()},
        "companion_work": _companion_work(recipe, records, finishes, _add(drivers, recipe["reduction"][1], recipe["reduction"][2])),
        "receiving_work": dict(zip(GROUP_FIELDS["receiving_work"], (
            holders, _mul(32, holders), _mul(256, holders), admissions, _add(admissions, raw_garde), passes,
            _mul(48, passes), _mul(256, passes), _mul(120, passes), _mul(256, passes), _mul(16, passes),
            _mul(16, passes), _mul(48, admissions), _mul(16, admissions)), strict=True)),
        "reduction_work": dict(zip(GROUP_FIELDS["reduction_work"], (
            drivers, records, finishes, singletons, _add(drivers, guards["contributor_garde"]),
            _add(_mul(2, drivers), direct_masks), _add(_mul(32, drivers), _mul(16, direct_masks)),
            _mul(16, drivers), _mul(32, drivers), _mul(2, finishes)), strict=True)),
        "output_work": dict(zip(GROUP_FIELDS["output_work"], (
            complete, _add(complete, guards["outcome_garde"]), _add(complete, guards["outcome_garde"]),
            post, _mul(4, post), post, post, _mul(2, post),
            _mul(2, post), _mul(8, post), _mul(4, post), _mul(256, post), _mul(256, post)), strict=True)),
        "observer_work": dict(zip(GROUP_FIELDS["observer_work"], (
            ordinary, _mul(4096, ordinary), _add(c["getters_upper"], _mul(4096, ordinary)),
            _mul(4096, ordinary), _mul(4096, ordinary), ordinary), strict=True)),
        "doctest": dict(zip(GROUP_FIELDS["doctest"], (int(docs), int(docs), 0, 0), strict=True)),
        "reference_input_work": _reference_input_work(recipe["reference_input"], docs=docs),
    }


def _reference_input_work(calls: dict[str, int], *, docs: bool) -> dict[str, int]:
    """Reserve both shared passes and every potentially constructed token.

    Healthy successes remain frozen in Recipe, while weakened guards can
    construct an unexpected token before the rejection helper aborts. Every
    admission therefore funds one possible token. Each relationship pass can
    call principal/reference_ready/combined/round/chart/batch-chart once.
    Three public doctests only define functions; none invokes a producer or
    reference API. Their compilation outcomes carry no runtime source work.
    """
    direct = _mapping(calls, REFERENCE_FIELDS)
    admissions = direct["admissions"]
    direct["token_constructions"] = max(direct["token_constructions"], admissions)
    result = {**direct, "relationship_passes_upper": _mul(2, admissions),
              "garde_validations_upper": admissions, "garde_callbacks_upper": admissions,
              "chart_identity_checks_upper": _mul(2, admissions),
              "upstream_getter_requests_upper": _mul(12, admissions),
              "doctest_compilation_cases": _mul(3, int(docs)),
              "doctest_compile_fail_cases": _mul(2, int(docs)),
              "doctest_executed_examples": 0}
    return _mapping(result, GROUP_FIELDS["reference_input_work"])


def _guard_lifecycle_work(work: dict[str, int], guards: dict[str, int]) -> dict[str, int]:
    """Retain shared-parent work entered by direct chart raw/subpredicate probes.

    Only direct whole-output Garde/typed checks can enter shared_outcome_valid;
    each retains its full256-label membership/contributor scan. Every direct
    chart-output or mask envelope keeps the existing8192 internal-getter upper
    for nested source/target/selection/view dispatch, even on early rejection.
    This adds no seed, advance, completion, clone, reduction or numerical work
    to the old2214 fields or to the direct fixture census.
    """
    checked = _mapping(guards, GUARD_FIELDS)
    whole = _add(checked["outcome_predicates"], checked["outcome_garde"])
    envelopes = _add(*checked.values())
    additions = {
        "lifecycle_garde_validate_calls_upper": checked["outcome_garde"],
        "lifecycle_output_predicate_calls_upper": whole,
        "lifecycle_witness_membership_label_probes_upper": _mul(256, whole),
        "lifecycle_output_contributor_label_probes_upper": _mul(256, whole),
        "lifecycle_internal_product_getter_requests_upper": _mul(8192, envelopes),
    }
    result = _mapping(work, GROUP_FIELDS["lifecycle_work"])
    for field, amount in additions.items():
        result[field] = _add(result[field], amount)
    return result


def _reservation(owners: WorkOwners, recipe: Recipe, prior_budget: dict[str, int], *,
                 ordinary: int, docs: bool, external_legal: bool) -> dict[str, int]:
    """Compose existing468 fields and the disjoint18-field reference-input group."""
    prior = _old_vector(prior_budget)
    groups = _delegated(owners, recipe)
    calls = _delegate_inputs(recipe)
    groups.update(fixture=recipe["fixture"], adaptive_call=calls["adaptive_work"],
                  projection_call=calls["projection_work"], enclosure_call=calls["enclosure_work"],
                  prior_adapter_work=_prior_work(prior, external_legal=external_legal,
                                                 chart_advances=recipe["fixture"]["combination_advances"]))
    groups.update(_new_groups(recipe, ordinary=_integer(ordinary), docs=docs))
    groups["lifecycle_work"] = _guard_lifecycle_work(groups["lifecycle_work"], recipe["guards"])
    result: dict[str, int] = {}
    for group, fields in GROUP_FIELDS.items():
        for field, amount in _mapping(groups[group], fields).items():
            result[PREFIX + group + "." + field] = amount
    return _mapping(result, FIELDS)


def full_package_reservation(owners: WorkOwners, *, prior_budget: dict[str, int],
                             include_doctests: bool = True) -> dict[str, int]:
    """Reserve all28 authored outcomes and four uninvoked documentation cases.

    This source recipe is reserved once per whole package command/baseline or
    mutant, independently of whether an outcome is missing, failed or reached.
    Coverage uses the same recipe with include_doctests=False. No observed test
    success, source admission or mutation score is established by this function.
    """
    if type(include_doctests) is not bool:
        raise Incomplete("Doctest recipe selection must be an exact Boolean.")
    return _reservation(owners, _merge_recipes(list(case_recipes().values())), prior_budget,
                        ordinary=len(CURRENT_ORDINARY_TESTS), docs=include_doctests, external_legal=False)


def numeric_work(vector: dict[str, int]) -> dict[str, int]:
    """Extract only new adaptive producer charges; projections do not refit."""
    checked = _mapping(vector, FIELDS)
    return {key: checked[PREFIX + "adaptive_work." + key] for key in AGGREGATE_FIELDS[:2]}


def preserve_and_extend(prior_budget: dict[str, int], vector: dict[str, int]) -> dict[str, int]:
    """Add486 disjoint fields and only the existing four numerical aggregates."""
    prior, checked = _old_vector(prior_budget), _mapping(vector, FIELDS)
    result = {**prior, **checked}
    extra = numeric_work(checked)
    for key, amount in extra.items():
        result[key] = _add(prior[key], amount)
    fits = extra["charged_fit_work_upper"]
    result[AGGREGATE_FIELDS[2]] = _add(prior[AGGREGATE_FIELDS[2]], fits)
    result[AGGREGATE_FIELDS[3]] = _add(prior[AGGREGATE_FIELDS[3]], _mul(32, fits))
    if len(result) != FULL_BUDGET_FIELDS:
        raise Incomplete("The combined source family is not exactly2700 fields.")
    return result


def admit_sources(source_sha256: dict[str, str]) -> dict[str, str]:
    """Check caller-supplied hashes for the15 actual source dependencies.

    Source hashing and complete154-input candidate custody belong to the caller;
    this pure comparison establishes neither native execution nor product scope.
    """
    if type(source_sha256) is not dict or source_sha256 != SOURCE_SHA256:
        raise Incomplete("A frozen source dependency changed or is missing.")
    return dict(SOURCE_SHA256)


def external_legal_reservation(owners: WorkOwners, contracts: ContractOwner, *,
                               prior_budget: dict[str, int], reference_input: bool = False) -> dict[str, int]:
    """Consume the actual strict legal owner's frozen source and literal census.

    Negative compiler specimens have no executed producer work. Each selected
    source profile covers one legal specimen, with two independent
    seeds, three chart advances and two whole-state equalities; no public Clone.
    Earlier legal consumers' new shared-adapter work is separately funded here.
    The explicit reference profile reuses that producer and adds the checked
    token/feedback calls; the default retains the exact historical source.
    """
    if type(reference_input) is not bool:
        raise Incomplete("The external reference-input selector must be an exact Boolean.")
    fields = REFERENCE_LEGAL_FIELDS if reference_input else LEGAL_FIELDS
    values = REFERENCE_LEGAL_EXPECTED if reference_input else LEGAL_EXPECTED
    expected_source = REFERENCE_LEGAL_SOURCE_SHA256 if reference_input else LEGAL_SOURCE_SHA256
    expected = dict(zip(fields, values, strict=True))
    actual = _mapping(contracts.expected(), fields)
    source = contracts.source()
    if contracts.fields != fields or actual != expected or type(source) is not str:
        raise Incomplete("The external legal source census is not the frozen strict owner.")
    if hashlib.sha256(source.encode("utf-8")).hexdigest() != expected_source:
        raise Incomplete("The external legal source changed after its census.")
    fixture = _fixture(histories=actual["histories"], observations=actual["observations"], appends=actual["appends"],
                       labels_upper=actual["labels"], contexts_upper=actual["contexts"], monotonic_upper=actual["monotonic"],
                       coordinates=actual["coordinates"], exchanges=actual["exchanges"], charts=actual["charts"],
                       frames_upper=actual["frames"], domains_upper=actual["domains"], prepares=actual["prepares"],
                       terms=actual["terms"], tracking_projections=actual["tracked_projections"],
                       chart_enclosures=actual["chart_enclosures"], raw_enclosures=actual["raw_enclosures"],
                       raw_batches=actual["raw_batches"], chart_batches=actual["chart_batches"],
                       principal_parameters=actual["principal_parameters"], combination_parameters=actual["combination_parameters"],
                       minima=actual["minima"], strata=actual["strata"], combination_seeds=actual["combination_seeds"],
                       combination_advances=actual["chart_advances"], state_equalities=actual["state_equalities"],
                       getters_upper=_add(*(actual[key] for key in ("bootstrap_getters", "outcome_getters", "estimate_getters", "iterator_next")),
                                          _add(actual["reference_estimate_getters"], 2) if reference_input else 0))
    reference = (_reference_input(state_constructions=actual["reference_states"],
        readiness_reads=actual["reference_readiness"], admissions=actual["reference_admissions"],
        feedback=actual["reference_feedback"], token_constructions=2,
        combined_reads=actual["reference_combined"], round_reads=actual["reference_rounds"])
        if reference_input else _reference_input())
    return _reservation(owners, _recipe(fixture, reference_input=reference), prior_budget,
                        ordinary=1, docs=False, external_legal=True)


def mechanism_reservation(owners: WorkOwners, recipe: Recipe, *, prior_budget: dict[str, int],
                          ordinary_outcomes: int) -> dict[str, int]:
    """Project an explicitly reviewed observer recipe; admit no observer here.

    The caller must freeze observer/recipe/candidate hashes, complete original
    recipe inputs, substituted-package work and dominance before execution. It
    must reserve each healthy/forced command separately. This API does not infer
    source authenticity or treat a partial/failed mechanism run as cheaper work.
    """
    if type(recipe) is not dict or set(recipe) != {"fixture", "companion", "receiving", "reduction", "guards", "direct_selections", "reference_input"}:
        raise Incomplete("A complete closed mechanism source recipe is required.")
    fixture = _mapping(recipe["fixture"], GROUP_FIELDS["fixture"])
    checked = _recipe(fixture, companion=recipe["companion"], receiving=recipe["receiving"],
                      reduction=recipe["reduction"], guards=recipe["guards"], direct_selections=recipe["direct_selections"],
                      reference_input=recipe["reference_input"])
    return _reservation(owners, checked, prior_budget, ordinary=_integer(ordinary_outcomes), docs=False, external_legal=False)


def source_test_outcomes(output: str, *, require_complete: bool = False) -> OutcomeSummary:
    """Receive anchored native outcomes; missing/failed/ignored stay incomplete.

    Unrelated older package rows are outside this new source census. Duplicate
    current rows and unknown names in these private/current namespaces are
    rejected. Outcome observations never change the prospective source budget.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Native outcome input is not bounded exact text.")
    outcomes: dict[str, str] = {}
    expected = set(CURRENT_ORDINARY_TESTS)
    private = ("combination::chart::roundoff::tests::", "combination::admission::chart_receiving::tests::",
               "combination::lifecycle::chart::guard_tests::", "reference_input_readiness::")
    for line in output.splitlines():
        if not line.startswith("test "):
            continue
        match = re.fullmatch(r"test ([A-Za-z0-9_:]+) \.\.\. (ok|FAILED|ignored)", line)
        name = line[5:].split(" ", 1)[0]
        # The continuation owns exactly these five outcomes and receives their
        # complete lines independently. All other unknown private names remain
        # rejected by the historical namespace guard below.
        if name in ACHIEVED_PHASE_ORDINARY_TESTS:
            continue
        relevant = name in expected or name.startswith(private)
        if not relevant:
            continue
        if match is None or name not in expected or name in outcomes:
            raise Incomplete("A current source outcome is malformed, duplicated or unknown.")
        outcomes[name] = match.group(2)
    missing = [name for name in CURRENT_ORDINARY_TESTS if name not in outcomes]
    failed = [name for name in CURRENT_ORDINARY_TESTS if name in outcomes and outcomes[name] != "ok"]
    complete = not missing and not failed
    if require_complete and not complete:
        raise Incomplete("The actual new source outcomes remain incomplete.")
    return {"status": "complete" if complete else "incomplete", "complete": complete,
            "outcomes": outcomes, "missing": missing, "failed": failed,
            "expected": len(CURRENT_ORDINARY_TESTS), "scope": "28 source outcomes only; no work counters or native admission"}


# S1.4b uses a separate source profile: historical receipts retain the mappings
# above, and a live composition candidate must explicitly admit these bytes.
COMPOSITION_SOURCE_SHA256 = {
    **SOURCE_SHA256,
    'crates/time-core/src/regression/chart.rs': '8c1eaae20f287b96f0859b79a4522cf3176ead59f5f46a35c5cfa797aded2221',
    'crates/time-core/src/regression/chart/roundoff.rs': '1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc',
    'crates/time-core/src/chart_batch.rs': 'b25ac73756f94bd2be9692130d8dee201bd8c2b0a9c0adf63c7eba929740c79c',
    'crates/time-core/tests/chart.rs': '6c556bdd92917803389b5867af2190fbd7055eb20120ac4c4dc14d1decc9aaac',
    'crates/time-core/tests/private/chart_guards.rs': 'af9fb05f3b179956da0b33874b8db8369fe68f9b22364985dc1eb78907b9b82f',
    'crates/time-core/tests/chart_combination_lifecycle.rs': '3d0f1e5864787fd5f437fec5b4237e0bedcc43efedfa92a7defc99d0e32a0398',
    'crates/time-core/tests/private/chart_composition.rs': '1d52638ab10f66c37b55bae2edac3e3b23898479531bb641b16d741181e4ba1a',
    'crates/time-core/tests/private/chart_composition_lifecycle.rs': 'da7b4f2f627ae92c1b3d829d7b2aeb25e41b8dde1cc43ddc4b07f221775fa84d',
}
COMPOSITION_ORDINARY_TESTS = (
    "chart_composition::composition_preserves_dyadic_values_and_rounding_provenance",
    "chart_composition::composition_retains_both_operand_errors_through_signed_elapsed",
    "chart_composition::composition_rejects_relationships_and_ordered_arithmetic_failures",
    "chart_composition_lifecycle::coefficient_error_identity_precedes_batch_principal_and_combination_updates",
)


def admit_composition_sources(actual: dict[str, str]) -> dict[str, str]:
    """Admit only the new explicit profile, without transferring historical pins."""
    if type(actual) is not dict or actual != COMPOSITION_SOURCE_SHA256:
        raise Incomplete("Composition source pins differ from the reviewed candidate.")
    return dict(actual)


def composition_case_recipes() -> dict[str, Recipe]:
    """Four new tests, separate from the frozen twenty-eight earlier recipes.

    Failed composition and descriptor admissions retain their full possible
    prefixes. V2 uses four independently genuine seeds, not an unavailable
    public combination clone. Four explicit principal clones are copied
    outside the six principal and six nested combination advances.
    """
    rows = (
        _recipe(_fixture(histories=1, appends=3, charts=8, frames_upper=8, domains_upper=1,
                         tracking_projections=3, chart_enclosures=3, getters_upper=8192)),
        _recipe(_fixture(histories=1, appends=3, charts=25, frames_upper=25, domains_upper=1,
                         tracking_projections=10, chart_enclosures=8, getters_upper=8192)),
        _recipe(_fixture(charts=16, frames_upper=16, domains_upper=4, getters_upper=8192)),
        _recipe(_fixture(histories=2, appends=6, charts=5, frames_upper=5, domains_upper=1,
                         tracking_projections=4, chart_enclosures=4, raw_enclosures=8, raw_batches=4,
                         chart_batches=4, combination_seeds=4, strata=2, chart_principal_advances=6,
                         combination_advances=6, state_equalities=6, state_clones=4, getters_upper=8192)),
    )
    return dict(zip(COMPOSITION_ORDINARY_TESTS, rows, strict=True))


def composition_package_fixture_recipe() -> Recipe:
    """Merge only the four new fixtures; no earlier package is projected twice."""
    return _merge_recipes(list(composition_case_recipes().values()))


def composition_external_fixture_recipe() -> Recipe:
    """Descriptor-only legal caller: five direct charts and no numerical producer."""
    return _recipe(_fixture(charts=5, frames_upper=5, domains_upper=1, labels_upper=10,
                           monotonic_upper=3, getters_upper=28))


def fixture_delta_reservation(owners: WorkOwners, recipe: Recipe, *, ordinary_outcomes: int) -> dict[str, int]:
    """Project only newly supplied fixtures into the existing486 source units.

    This is an additive delta, never a whole-package replacement. Existing
    whole-package APIs retain their historical floors unchanged. Only actual
    new chart-combination attempts add shared transaction/materialization work;
    no historical seed, advance, bound bridge or direct kernel is invented.
    """
    if type(recipe) is not dict or set(recipe) != {"fixture", "companion", "receiving", "reduction", "guards", "direct_selections", "reference_input"}:
        raise Incomplete("A complete closed fixture-delta recipe is required.")
    checked = _recipe(recipe["fixture"], companion=recipe["companion"], receiving=recipe["receiving"],
                      reduction=recipe["reduction"], guards=recipe["guards"],
                      direct_selections=recipe["direct_selections"], reference_input=recipe["reference_input"])
    groups = _delegated(owners, checked)
    calls = _delegate_inputs(checked)
    prior = dict.fromkeys(GROUP_FIELDS["prior_adapter_work"], 0)
    advances = checked["fixture"]["combination_advances"]
    prior["transaction_adapter_entries_upper"] = _mul(3, advances)
    prior["decision_outcome_adapter_getters_upper"] = _mul(128, advances)
    groups.update(fixture=checked["fixture"], adaptive_call=calls["adaptive_work"],
                  projection_call=calls["projection_work"], enclosure_call=calls["enclosure_work"],
                  prior_adapter_work=prior)
    groups.update(_new_groups(checked, ordinary=_integer(ordinary_outcomes), docs=False))
    groups["lifecycle_work"] = _guard_lifecycle_work(groups["lifecycle_work"], checked["guards"])
    clones = checked["fixture"]["state_clones"]
    for field, per_clone in (("staged_state_clone_cells_upper", 16),
                             ("staged_source_fields_copied_upper", 96),
                             ("staged_fixed_fields_copied_upper", 16)):
        groups["principal_receiver_work"][field] = _add(
            groups["principal_receiver_work"][field], _mul(per_clone, clones))
    return _mapping({PREFIX + group + "." + field: amount
                     for group, fields in GROUP_FIELDS.items()
                     for field, amount in _mapping(groups[group], fields).items()}, FIELDS)


def asymmetric_settled_transform_fixture_recipe() -> Recipe:
    """Fund only the new body of an already registered private guard test.

    Both anchor projections borrow the existing three-row retained source and
    cached report: no history, append, fit, point, tracking or enclosure is new.
    The one exact comparator chart belongs here; the separately counted settled
    effect and its four-scalar admission belong to the rate overlay. Reusing the
    checked frame/domain constructs neither again. Generic owner bounds retain
    both whole-history validation passes and every anchor transform primitive.
    Earlier case recipes and their source profiles remain unchanged.
    """
    return _recipe(_fixture(
        charts=1, frames_upper=0, domains_upper=0, prepares=2, known_prepares_upper=2,
        terms=2, anchor_projections=2, chart_enclosures=0, labels_upper=0,
        contexts_upper=0, monotonic_upper=4, getters_upper=64))


ACHIEVED_PHASE_ORDINARY_TESTS = tuple("reference_input_readiness::" + name for name in (
    "achieved_phase_dyadics_change_coordinates_and_preserve_rate",
    "achieved_phase_context_and_scalar_rejections_preserve_ready_owner",
    "achieved_phase_noeffect_unknown_and_repetition_are_monotone",
    "achieved_phase_arithmetic_failure_is_admitted_and_invalidates_old_owner",
    "achieved_phase_remapped_histories_retain_without_new_estimate",
))
ACHIEVED_PHASE_RECEIVER_ATTEMPTS = (5, 26, 4, 2, 1)
ACHIEVED_PHASE_ADMITTED = (5, 13, 1, 1, 1)
ACHIEVED_PHASE_DOCTEST_LINES = (106, 436, 455, 473, 495, 517)
ACHIEVED_PHASE_COMPILE_FAIL_LINES = (106, 436, 455, 473)
ACHIEVED_PHASE_SOURCE_SHA256 = {
    **COMPOSITION_SOURCE_SHA256,
    'crates/time-core/src/reference_input.rs': 'be4a4142672606c3feca43ce36b307eeb2d1b9a56866be203889412f9aa30209',
    'crates/time-core/src/lib.rs': 'af227606c8f560aa1bdf4d3c99c3e7df1429967b64f276f23562353892f3a629',
    'crates/time-core/tests/private/reference_input_readiness.rs': '8dc34c4ac39c09302cd18d0633c0edc2a0aefb7d66cbf157110fbf0c17e3b58c',
}


def admit_achieved_phase_sources(actual: dict[str, str]) -> dict[str, str]:
    """Bind the complete current source profile, retaining all historical pins."""
    if type(actual) is not dict or actual != ACHIEVED_PHASE_SOURCE_SHA256:
        raise Incomplete("Achieved-phase source pins differ from the reviewed candidate.")
    return dict(actual)


def achieved_phase_case_recipes() -> dict[str, Recipe]:
    """Five disjoint genuine fixtures, including every possible receiver prefix.

    Each receiver attempt reserves two effect-chart constructors and two
    endpoint-frame constructors, even where the canonical helper rejects early.
    Only the actual three histories/nine appends produce numerical fit work.
    Composed result frames/charts are funded by the existing forty-field
    composition recurrence, not invented again in this fixture census.
    Getter uppers include the finite source snapshots and independent assertions;
    none is an observed operation count. The final retained journey copies its
    public principal state and source counters, never a combination credit owner.
    """
    fixtures = (
        _fixture(histories=1, appends=3, charts=2, frames_upper=2, domains_upper=1,
                 anchor_projections=5, chart_enclosures=5, getters_upper=8192),
        _fixture(charts=2, frames_upper=9, domains_upper=5, getters_upper=8192),
        _fixture(charts=1, frames_upper=2, domains_upper=1, getters_upper=8192),
        _fixture(charts=2, frames_upper=2, domains_upper=1, getters_upper=8192),
        _fixture(histories=2, appends=6, charts=1, frames_upper=2, domains_upper=1,
                 tracking_projections=4, chart_enclosures=4, raw_enclosures=2, raw_batches=1,
                 chart_batches=2, combination_seeds=1, strata=2, combination_advances=2,
                 state_equalities=1, state_clones=1, getters_upper=8192),
    )
    references = (
        _reference_input(state_constructions=5, readiness_reads=4),
        _reference_input(state_constructions=13, readiness_reads=26),
        _reference_input(state_constructions=1, readiness_reads=1, feedback=6),
        _reference_input(state_constructions=1, readiness_reads=1),
        _reference_input(state_constructions=2, admissions=3, token_constructions=1,
                         combined_reads=1, round_reads=4),
    )
    rows: dict[str, Recipe] = {}
    for name, fixture, reference, attempts in zip(
            ACHIEVED_PHASE_ORDINARY_TESTS, fixtures, references, ACHIEVED_PHASE_RECEIVER_ATTEMPTS, strict=True):
        # Increase only constructor entries: reusing checked frame/domain values
        # inside Chart::new constructs no new domains or numerical observations.
        fixture["charts"] = _add(fixture["charts"], _mul(2, attempts))
        fixture["frames_upper"] = _add(fixture["frames_upper"], _mul(2, attempts))
        rows[name] = _recipe(fixture, reference_input=reference)
    return rows


def achieved_phase_package_fixture_recipe() -> Recipe:
    """Merge only the five continuation fixtures; earlier package recipes stay frozen."""
    return _merge_recipes(list(achieved_phase_case_recipes().values()))


def achieved_phase_test_observation(output: str, *, require_complete: bool = False) -> OutcomeSummary:
    """Require exact complete named lines without turning outcomes into call counts."""
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Achieved-phase outcome input is not bounded exact text.")
    outcomes: dict[str, str] = {}
    for raw in output.splitlines(keepends=True):
        line = raw[:-2] if raw.endswith("\r\n") else raw[:-1] if raw.endswith("\n") else raw
        if not line.startswith("test "):
            continue
        name = line[5:].split(" ", 1)[0]
        if not name.startswith("reference_input_readiness::") or name in REFERENCE_ORDINARY_TESTS:
            continue
        if not raw.endswith("\n"):
            raise Incomplete("An achieved-phase outcome lacks its complete line delimiter.")
        match = re.fullmatch(r"test ([A-Za-z0-9_:]+) \.\.\. (ok|FAILED|ignored)", line)
        if match is None or name not in ACHIEVED_PHASE_ORDINARY_TESTS or name in outcomes:
            raise Incomplete("An achieved-phase outcome is malformed, duplicated or unknown.")
        outcomes[name] = match.group(2)
    missing = [name for name in ACHIEVED_PHASE_ORDINARY_TESTS if name not in outcomes]
    failed = [name for name in ACHIEVED_PHASE_ORDINARY_TESTS if name in outcomes and outcomes[name] != "ok"]
    complete = not missing and not failed
    if require_complete and not complete:
        raise Incomplete("The five actual achieved-phase outcomes remain incomplete.")
    return {"status": "complete" if complete else "incomplete", "complete": complete,
            "outcomes": outcomes, "missing": missing, "failed": failed, "expected": 5,
            "scope": "Five source outcomes only; no operation counters or native admission."}
