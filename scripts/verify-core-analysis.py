#!/usr/bin/env python3
"""Bounded four-owner combination checkpoint; execution needs current source binding.

GPL-2.0-only, David Osipov with AI assistance, 29 October 2026.
Adapts existing C/D wrappers; portable-analysis owns execution/custody. Admission
bindings are coordinator-filled only after source freeze, never invented here.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import math
from pathlib import Path
import re
import shutil
import struct
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("core_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
DOMAIN = COMMON.load_module("core_cohort_helpers", ROOT / "scripts/verify-time-domain-analysis.py")
SELECTION = COMMON.load_module("core_selection_accounting", ROOT / "scripts/core-selection-accounting.py")
SELECTION_CONTRACTS = COMMON.load_module("core_selection_contracts", ROOT / "scripts/core-selection-contracts.py")
SELECTION_INPUTS = ("scripts/core-selection-accounting.py", "scripts/core-selection-contracts.py")
PROJECTION = COMMON.load_module("core_projection_accounting", ROOT / "scripts/core-projection-accounting.py")
COHORTS = COMMON.load_module("core_analysis_cohorts", ROOT / "scripts/core-analysis-cohorts.py")
PROJECTION_INPUTS = ("scripts/core-projection-accounting.py", "scripts/core-analysis-cohorts.py")
ENCLOSURE = COMMON.load_module("core_enclosure_accounting", ROOT / "scripts/core-enclosure-accounting.py")
ENCLOSURE_INPUTS = ("scripts/core-enclosure-accounting.py",)
PROJECTED_BATCH = COMMON.load_module("core_projected_batch_accounting", ROOT / "scripts/core-projected-batch-accounting.py")
PROJECTED_BATCH_INPUTS = ("scripts/core-projected-batch-accounting.py",)
COMBINATION = COMMON.load_module("core_combination_accounting", ROOT / "scripts/core-combination-accounting.py")
COMBINATION_INPUTS = ("scripts/core-combination-accounting.py",)
COMBINATION_CONTRACT_MARKER = 'CORE_COMBINATION_CONTRACT_PROFILE'
PRODUCT, ANALYSIS, PREPARE = COMMON.PRODUCT, COMMON.ANALYSIS, COMMON.PREPARE
Incomplete = COMMON.Incomplete
REPORT_FAILURES = (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError,
                   KeyError, TypeError, StopIteration, RuntimeError)
HISTORICAL_SUBJECTS = {"regression": "crates/time-core/src/regression.rs",
            "residuals": "crates/time-core/src/regression/residuals.rs",
            "normalization": "crates/time-core/src/regression/normalization.rs",
            "history": "crates/time-core/src/regression/history.rs",
            "runs": "crates/time-core/src/regression/runs.rs",
            "selection": "crates/time-core/src/selection.rs",
            "projection": "crates/time-core/src/regression/projection.rs",
            "enclosure": "crates/time-core/src/projection_enclosure.rs",
            "projected_batch": "crates/time-core/src/projected_batch.rs"}
SUBJECTS = {"combination": "crates/time-core/src/combination.rs",
            "principal": "crates/time-core/src/principal.rs",
            "regression": HISTORICAL_SUBJECTS['regression'],
            "projection": HISTORICAL_SUBJECTS['projection']}
POLICY = "policy/time-core-analysis.json"
SCOPE = COMMON.Scope("time-core", tuple(SUBJECTS.items()), POLICY)
BOUND_INPUTS = (
    '.cargo/audit.toml',
    '.cargo/config.toml',
    '.cargo/mutants.toml',
    '.config/nextest.toml',
    'Cargo.lock',
    'Cargo.toml',
    'clippy.toml',
    'crates/nevrotick-ui/Cargo.toml',
    'crates/nevrotick-ui/src/lib.rs',
    'crates/nevrotick-ui/src/main.rs',
    'crates/nevrotick-ui/src/model.rs',
    'crates/nevrotick-ui/src/native.rs',
    'crates/nevrotick-ui/src/native/controls.rs',
    'crates/nevrotick-ui/src/native/view.rs',
    'crates/nevrotick-ui/tests/preview.rs',
    'crates/ntp-exchange/COPYING',
    'crates/ntp-exchange/Cargo.toml',
    'crates/ntp-exchange/PROVENANCE.md',
    'crates/ntp-exchange/clippy.toml',
    'crates/ntp-exchange/src/association.rs',
    'crates/ntp-exchange/src/context.rs',
    'crates/ntp-exchange/src/lib.rs',
    'crates/ntp-exchange/tests/association.rs',
    'crates/ntp-exchange/tests/support/scenario_driver.rs',
    'crates/ntp-exchange/tests/support/scenario_expectations.rs',
    'crates/ntp-exchange/tests/support/vectors.rs',
    'crates/ntp-wire/COPYING',
    'crates/ntp-wire/Cargo.toml',
    'crates/ntp-wire/PROVENANCE.md',
    'crates/ntp-wire/src/datagram.rs',
    'crates/ntp-wire/src/header.rs',
    'crates/ntp-wire/src/lib.rs',
    'crates/ntp-wire/tests/datagram.rs',
    'crates/ntp-wire/tests/header.rs',
    'crates/ntp-wire/tests/support/vectors.rs',
    'crates/time-core/COPYING',
    'crates/time-core/Cargo.toml',
    'crates/time-core/PROVENANCE.md',
    'crates/time-core/clippy.toml',
    'crates/time-core/src/combination.rs',
    'crates/time-core/src/lib.rs',
    'crates/time-core/src/principal.rs',
    'crates/time-core/src/projection_enclosure.rs',
    'crates/time-core/src/projected_batch.rs',
    'crates/time-core/src/regression.rs',
    'crates/time-core/src/regression/residuals.rs',
    'crates/time-core/src/regression/normalization.rs',
    'crates/time-core/src/regression/history.rs',
    'crates/time-core/src/regression/projection.rs',
    'crates/time-core/src/regression/runs.rs',
    'crates/time-core/src/selection.rs',
    'crates/time-core/tests/private/residual_guards.rs',
    'crates/time-core/tests/private/normalization_guards.rs',
    'crates/time-core/tests/private/unit_guards.rs',
    'crates/time-core/tests/private/runs_guards.rs',
    'crates/time-core/tests/private/runs_sign_observer.rs',
    'crates/time-core/tests/private/selection_guards.rs',
    'crates/time-core/tests/private/projection_guards.rs',
    'crates/time-core/tests/private/enclosure_guards.rs',
    'crates/time-core/tests/private/projected_batch_guards.rs',
    'crates/time-core/tests/regression.rs',
    'crates/time-core/tests/residuals.rs',
    'crates/time-core/tests/normalization.rs',
    'crates/time-core/tests/history.rs',
    'crates/time-core/tests/runs.rs',
    'crates/time-core/tests/selection.rs',
    'crates/time-core/tests/projection.rs',
    'crates/time-core/tests/projection_enclosure.rs',
    'crates/time-core/tests/projected_batch.rs',
    'crates/time-core/tests/principal.rs',
    'crates/time-core/tests/support/principal_cases.rs',
    'crates/time-core/tests/private/principal_guards.rs',
    'crates/time-core/tests/private/combination_cases.rs',
    'crates/time-core/tests/support/residual_facts.rs',
    'crates/time-core/tests/support/normalization_facts.rs',
    'crates/time-core/tests/support/history_facts.rs',
    'crates/time-domain/COPYING',
    'crates/time-domain/Cargo.toml',
    'crates/time-domain/PROVENANCE.md',
    'crates/time-domain/clippy.toml',
    'crates/time-domain/src/clock.rs',
    'crates/time-domain/src/coordinate.rs',
    'crates/time-domain/src/era.rs',
    'crates/time-domain/src/exchange.rs',
    'crates/time-domain/src/lib.rs',
    'crates/time-domain/src/rounding.rs',
    'crates/time-domain/tests/clock.rs',
    'crates/time-domain/tests/coordinate.rs',
    'crates/time-domain/tests/era.rs',
    'crates/time-domain/tests/exchange.rs',
    'crates/time-domain/tests/rounding.rs',
    'deny.toml',
    'policy/rust-workspace.toml',
    'policy/tool-manifest.json',
    'policy/ui-m0-dependencies.json',
    'policy/verification-manifest.toml',
    'rust-toolchain.toml',
    'rustfmt.toml',
    'scripts/check-environment.py',
    'scripts/check-strict.py',
    'scripts/verify.py',
)
SOURCE_FILE_COUNT = 101
# Discovery remains separately admitted and never grants a campaign.
INVENTORY_ADMISSION = "tasks/s1.3-combination-inventory-admission.json"
INVENTORY_SUBJECTS = dict(SUBJECTS)
INVENTORY_BOUND_INPUTS = BOUND_INPUTS
INVENTORY_SOURCE_FILE_COUNT = SOURCE_FILE_COUNT
INVENTORY_FIXED = {
    "schema_version": 4, "admission_status": "admitted-four-owner-inventory-only",
    "package": "time-core", "source_file_count": INVENTORY_SOURCE_FILE_COUNT,
    "subjects": INVENTORY_SUBJECTS, "maximum_mutants": 512,
    "listing_maximum_per_file": 512, "listing_maximum_total": 3072,
    "inventory_wall_seconds": 600, "mutation_jobs": 10,
    "mutation_process_limit": 512, "listing_process_limit": 128,
    "mutation_message_format": "json", "mutation_equivalences": [],
    "native_test_invocations": 0, "campaigns_admitted": False,
}
INVENTORY_REQUIRED_TOOLS = frozenset({
    "scripts/verify-core-analysis.py", "scripts/portable-analysis.py",
    "scripts/verify-time-domain-analysis.py", "scripts/analysis-runtime.py",
    "scripts/core-selection-accounting.py", "scripts/core-selection-contracts.py",
    *PROJECTION_INPUTS,
    *ENCLOSURE_INPUTS,
    *PROJECTED_BATCH_INPUTS,
    *COMBINATION_INPUTS,
    "scripts/check-analysis.py", "scripts/analysis-controls.py",
    "scripts/prepare-product.py", "scripts/verify.py",
    "scripts/check-environment.py", "scripts/check-strict.py",
    "policy/analysis-tools.json",
})
# Exact reviewed single-mutant set; source changes require fresh admission.
HISTORICAL_MUTATION_INVENTORIES = {'history': {'count': 20, 'sha256': '1acac35ade9b5595000b8e9300bb528e1c6de81abb549c8f86b48d84e63b1013'},
 'normalization': {'count': 75, 'sha256': '65a08b84c5fb51dc26448eb062085b7091ad545fee1533b4461c29a7f948d605'},
 'regression': {'count': 110, 'sha256': '27b659a1c75c63bdc1ad6411185fc52efd3321bc5aad3851ae59eaf8bf353d29'},
 'residuals': {'count': 39, 'sha256': '8ec028d0ef3e363138b3fe0ec8779062f716e566efe3a044a68d14f482fd29bb'},
 'runs': {'count': 116, 'sha256': '74d6761a97caea2bc46114d2e6b80443c60b47a5a8d770906bcff915eaca7ed4'},
 'selection': {'count': 141, 'sha256': 'c09b9fd94a4efd1a3f15f3a54cfe8ce10beea213e13bd42cd742e24b1cb186a5'}}
HISTORICAL_PROJECTION_MUTATION_INVENTORIES = {
  "regression": {
    "count": 110,
    "sha256": "6d956a70848df7db2dd1ec7ceb15c52f967719630252eca4dbcb39274fc6eafc"
  },
  "residuals": {
    "count": 39,
    "sha256": "8ec028d0ef3e363138b3fe0ec8779062f716e566efe3a044a68d14f482fd29bb"
  },
  "normalization": {
    "count": 75,
    "sha256": "65a08b84c5fb51dc26448eb062085b7091ad545fee1533b4461c29a7f948d605"
  },
  "history": {
    "count": 20,
    "sha256": "1acac35ade9b5595000b8e9300bb528e1c6de81abb549c8f86b48d84e63b1013"
  },
  "runs": {
    "count": 116,
    "sha256": "74d6761a97caea2bc46114d2e6b80443c60b47a5a8d770906bcff915eaca7ed4"
  },
  "selection": {
    "count": 141,
    "sha256": "c09b9fd94a4efd1a3f15f3a54cfe8ce10beea213e13bd42cd742e24b1cb186a5"
  },
  "projection": {
    "count": 56,
    "sha256": "7ab2064572ec2eff0d0871352a4be48bee47b15a25015f9019af7ae4bfd4e665"
  }
}
MUTATION_INVENTORIES = {'enclosure': {'count': 131,
               'sha256': '4bb51f97bf78702db283ce281a5d395f76ca4c679ac22c900d0b94d00a92558f'},
 'history': {'count': 20,
             'sha256': '1acac35ade9b5595000b8e9300bb528e1c6de81abb549c8f86b48d84e63b1013'},
 'normalization': {'count': 75,
                   'sha256': '65a08b84c5fb51dc26448eb062085b7091ad545fee1533b4461c29a7f948d605'},
 'projected_batch': {'count': 29,
                     'sha256': 'cbc90c38a55e2f2fa8933b6dcaff59a8ec46dda2df99f69947b8c7559d7a2731'},
 'projection': {'count': 56,
                'sha256': '7ab2064572ec2eff0d0871352a4be48bee47b15a25015f9019af7ae4bfd4e665'},
 'regression': {'count': 110,
                'sha256': '6d956a70848df7db2dd1ec7ceb15c52f967719630252eca4dbcb39274fc6eafc'},
 'residuals': {'count': 39,
               'sha256': '8ec028d0ef3e363138b3fe0ec8779062f716e566efe3a044a68d14f482fd29bb'},
 'runs': {'count': 116,
          'sha256': '74d6761a97caea2bc46114d2e6b80443c60b47a5a8d770906bcff915eaca7ed4'},
 'selection': {'count': 141,
               'sha256': 'c09b9fd94a4efd1a3f15f3a54cfe8ce10beea213e13bd42cd742e24b1cb186a5'}}
HISTORICAL_MUTATION_INVENTORIES = MUTATION_INVENTORIES
HISTORICAL_PRINCIPAL_MUTATION_INVENTORIES = {
    'principal': {'count': 217, 'sha256': '433c59b4b164904c8160163df5902992faa39e558c04a96545e55bdba65a8d0d'},
    'regression': {'count': 110, 'sha256': '7573a1e8b84a1609ad19a3d70de89d85169affbc58c786b08d4dd949da4d6a5b'},
}
MUTATION_INVENTORIES = {}
GRAPH_STATUS = 'pending-four-owner-inventory-and-exact-cohort-binding'
# Historical root-owned discovery; never evidence for the changed candidate.
HISTORICAL_PROJECTION_TEST_COUNTS = {
  "ordinary": 107,
  "doctests": 5,
  "baseline": 112
}
FINAL_PACKAGE_TEST_COUNTS = {'ordinary': 163, 'doctests': 5, 'baseline': 168}
COORDINATOR_COHORTS = {"status": 'pending-exact-source-and-four-owner-cohorts', "schema_version": 4,
                      "maximum_variants_per_cohort": 512, "listing_maximum_total": 3072,
                      "partition": "four-disjoint-changed-or-coupled-owner-cohorts",
                      "plan_required_before_mutation": True,
                      "duplicate_receipts_or_retries_admitted": False}
PROJECTION_GLOBAL_LIMITS = {'projection.call.appends': 67611,
 'projection.call.contexts': 7997,
 'projection.call.coordinates': 270444,
 'projection.call.exchanges': 67611,
 'projection.call.getters': 2460168,
 'projection.call.histories': 8724,
 'projection.call.monotonic': 87967,
 'projection.call.observations': 67611,
 'projection.call.projections': 34896,
 'projection.call.terms': 49436,
 'projection.doctest.projection_term_garde_rule_callbacks_upper': 2178,
 'projection.doctest.projection_term_predicate_calls_upper': 4356,
 'projection.doctest.projection_terms_constructor_attempts_upper': 726,
 'projection.doctest.projection_terms_getter_requests_upper': 1452,
 'projection.private.candidate_validation_calls_upper': 17448,
 'projection.private.error_format_calls_upper': 68338,
 'projection.private.error_source_calls_upper': 34169,
 'projection.private.error_wrapper_constructions_upper': 34169,
 'projection.private.expected_arithmetic_error_calls_upper': 26172,
 'projection.private.first_error_calls_upper': 17448,
 'projection.private.projection_arithmetic_calls_upper': 40712,
 'projection.private.raw_candidate_constructions_upper': 17448,
 'projection.private.shared_arithmetic_calls_upper': 40712,
 'projection.private.step_operation_calls_upper': 44347,
 'projection.private.supported_result_calls_upper': 22537,
 'projection.private.term_predicate_calls_upper': 107596,
 'projection.private.term_rule_calls_upper': 61068,
 'projection.private.typed_error_downcast_attempts_upper': 23264,
 'projection.work.charged_fit_work_upper': 67611,
 'projection.work.history_duration_operations_upper': 4327104,
 'projection.work.history_fit_input_constructions_upper': 67611,
 'projection.work.history_generated_point_admissions_upper': 4327104,
 'projection.work.history_normalization_entries_upper': 67611,
 'projection.work.history_staging_Copy_assignments_upper': 8586597,
 'projection.work.history_statistics_entries_upper': 67611,
 'projection.work.history_statistics_inner_fit_upper': 67611,
 'projection.work.point_admissions_upper': 4327104,
 'projection.work.projection_arithmetic_primitive_calls_upper': 174480,
 'projection.work.projection_candidate_validation_calls_upper': 49436,
 'projection.work.projection_checked_integer_helper_calls_upper': 69792,
 'projection.work.projection_duration_operations_upper': 69792,
 'projection.work.projection_first_error_calls_upper': 49436,
 'projection.work.projection_internal_product_getter_requests_upper': 418752,
 'projection.work.projection_normalization_primitive_calls_upper': 209376,
 'projection.work.projection_raw_candidate_constructions_upper': 49436,
 'projection.work.projection_term_garde_rule_callbacks_upper': 148308,
 'projection.work.projection_term_predicate_calls_upper': 296616}
PROJECTION_TELEMETRY = {
    "status": "admitted-source-owned-observer", "counter_family": PROJECTION.MARKER,
    "observed_fields": list(PROJECTION.FIELDS),
    "case_recipes": {case: dict(zip(PROJECTION.FIELDS, counts, strict=True))
                     for case, counts in PROJECTION.SOURCE_RECIPES},
    "source_owned_private_work_per_package": PROJECTION.SOURCE_PRIVATE_WORK,
    "global_limits": PROJECTION_GLOBAL_LIMITS,
}
ENCLOSURE_GLOBAL_LIMITS = {'enclosure.call.appends': 32715,
 'enclosure.call.contexts': 10905,
 'enclosure.call.coordinates': 130860,
 'enclosure.call.enclosures': 11632,
 'enclosure.call.exchanges': 32715,
 'enclosure.call.getters': 2303136,
 'enclosure.call.histories': 10905,
 'enclosure.call.monotonic': 55979,
 'enclosure.call.observations': 32715,
 'enclosure.call.projections': 11632,
 'enclosure.call.terms': 11632,
 'enclosure.private.checked_scalar_primitive_calls_upper': 23002280,
 'enclosure.private.raw_garde_rule_callbacks_upper': 130860,
 'enclosure.private.raw_garde_validate_calls_upper': 55252,
 'enclosure.private.raw_predicate_calls_upper': 151216,
 'enclosure.private.source_helper_entries_upper': 1150114,
 'enclosure.work.charged_fit_work_upper': 32715,
 'enclosure.work.enclosure_bridge_entry_calls_upper': 11632,
 'enclosure.work.enclosure_checked_scalar_primitive_calls_upper': 232640,
 'enclosure.work.enclosure_interval_constructor_attempts_upper': 11632,
 'enclosure.work.enclosure_interval_garde_validate_calls_upper': 23264,
 'enclosure.work.enclosure_interval_rule_calls_upper': 11632,
 'enclosure.work.enclosure_projection_getter_chain_calls_upper': 58160,
 'enclosure.work.enclosure_raw_garde_rule_callbacks_upper': 23264,
 'enclosure.work.enclosure_raw_garde_validate_calls_upper': 11632,
 'enclosure.work.enclosure_raw_helper_entries_upper': 325696,
 'enclosure.work.enclosure_wrapper_debug_format_calls_upper': 11632,
 'enclosure.work.history_duration_operations_upper': 2093760,
 'enclosure.work.history_fit_input_constructions_upper': 32715,
 'enclosure.work.history_generated_point_admissions_upper': 2093760,
 'enclosure.work.history_normalization_entries_upper': 32715,
 'enclosure.work.history_staging_Copy_assignments_upper': 4154805,
 'enclosure.work.history_statistics_entries_upper': 32715,
 'enclosure.work.history_statistics_inner_fit_upper': 32715,
 'enclosure.work.point_admissions_upper': 2093760,
 'enclosure.work.projection_arithmetic_primitive_calls_upper': 58160,
 'enclosure.work.projection_candidate_validation_calls_upper': 11632,
 'enclosure.work.projection_checked_integer_helper_calls_upper': 23264,
 'enclosure.work.projection_duration_operations_upper': 23264,
 'enclosure.work.projection_first_error_calls_upper': 11632,
 'enclosure.work.projection_internal_product_getter_requests_upper': 139584,
 'enclosure.work.projection_normalization_primitive_calls_upper': 69792,
 'enclosure.work.projection_raw_candidate_constructions_upper': 11632,
 'enclosure.work.projection_term_garde_rule_callbacks_upper': 34896,
 'enclosure.work.projection_term_predicate_calls_upper': 69792}
ENCLOSURE_TELEMETRY = {
    "status": "admitted-source-owned-observer",
    "counter_family": "CORE_ENCLOSURE_PROFILE",
    "observed_fields": list(ENCLOSURE.FIELDS),
    "case_recipes": ENCLOSURE.case_recipes(),
    "source_owned_private_work_per_package": ENCLOSURE.SOURCE_PRIVATE_WORK,
    "source_owned_per_enclosure_upper": ENCLOSURE.ENCLOSURE_WORK_PER_CALL,
    "literal_doctests_added": ENCLOSURE.LITERAL_DOCTESTS_ADDED,
    "global_limits": ENCLOSURE_GLOBAL_LIMITS,
}
PROJECTED_BATCH_GLOBAL_LIMITS = {'projected_batch.call.appends': 93783,
 'projected_batch.call.batches': 23264,
 'projected_batch.call.contexts': 31261,
 'projected_batch.call.coordinates': 375132,
 'projected_batch.call.enclosures': 31261,
 'projected_batch.call.exchanges': 93783,
 'projected_batch.call.getters': 3223518,
 'projected_batch.call.histories': 31261,
 'projected_batch.call.minima': 5816,
 'projected_batch.call.monotonic': 160667,
 'projected_batch.call.observations': 93783,
 'projected_batch.call.projections': 31261,
 'projected_batch.call.selections': 11632,
 'projected_batch.call.terms': 31261,
 'projected_batch.private.direct_raw_authored_helper_entries_upper': 66157,
 'projected_batch.private.direct_raw_entry_read_attempts_upper': 15994,
 'projected_batch.private.direct_raw_garde_rule_callbacks_upper': 7270,
 'projected_batch.private.direct_raw_garde_validate_calls_upper': 7270,
 'projected_batch.private.direct_raw_internal_product_getter_requests_upper': 72700,
 'projected_batch.work.charged_fit_work_upper': 93783,
 'projected_batch.work.constructor_authored_helper_entries_upper': 1326048,
 'projected_batch.work.delegated_batch_attempts_upper': 23264,
 'projected_batch.work.enclosure_bridge_entry_calls_upper': 31261,
 'projected_batch.work.enclosure_checked_scalar_primitive_calls_upper': 625220,
 'projected_batch.work.enclosure_interval_constructor_attempts_upper': 31261,
 'projected_batch.work.enclosure_interval_garde_validate_calls_upper': 62522,
 'projected_batch.work.enclosure_interval_rule_calls_upper': 31261,
 'projected_batch.work.enclosure_projection_getter_chain_calls_upper': 156305,
 'projected_batch.work.enclosure_raw_garde_rule_callbacks_upper': 62522,
 'projected_batch.work.enclosure_raw_garde_validate_calls_upper': 31261,
 'projected_batch.work.enclosure_raw_helper_entries_upper': 875308,
 'projected_batch.work.enclosure_wrapper_debug_format_calls_upper': 31261,
 'projected_batch.work.entry_read_attempts_upper': 1116672,
 'projected_batch.work.history_duration_operations_upper': 6002112,
 'projected_batch.work.history_fit_input_constructions_upper': 93783,
 'projected_batch.work.history_generated_point_admissions_upper': 6002112,
 'projected_batch.work.history_normalization_entries_upper': 93783,
 'projected_batch.work.history_staging_Copy_assignments_upper': 11910441,
 'projected_batch.work.history_statistics_entries_upper': 93783,
 'projected_batch.work.history_statistics_inner_fit_upper': 93783,
 'projected_batch.work.internal_product_getter_requests_upper': 4094464,
 'projected_batch.work.minimum_garde_validate_calls_upper': 5816,
 'projected_batch.work.point_admissions_upper': 6002112,
 'projected_batch.work.projection_arithmetic_primitive_calls_upper': 156305,
 'projected_batch.work.projection_candidate_validation_calls_upper': 31261,
 'projected_batch.work.projection_checked_integer_helper_calls_upper': 62522,
 'projected_batch.work.projection_duration_operations_upper': 62522,
 'projected_batch.work.projection_first_error_calls_upper': 31261,
 'projected_batch.work.projection_internal_product_getter_requests_upper': 375132,
 'projected_batch.work.projection_normalization_primitive_calls_upper': 187566,
 'projected_batch.work.projection_raw_candidate_constructions_upper': 31261,
 'projected_batch.work.projection_term_garde_rule_callbacks_upper': 93783,
 'projected_batch.work.projection_term_predicate_calls_upper': 187566,
 'projected_batch.work.raw_garde_rule_callbacks_upper': 23264,
 'projected_batch.work.raw_garde_validate_calls_upper': 23264,
 'projected_batch.work.scratch_Copy_assignments_upper': 721184,
 'projected_batch.work.selection_batch_copy_adapter_attempts_upper': 372224,
 'projected_batch.work.selection_batch_label_comparisons_upper': 2791680,
 'projected_batch.work.selection_cell_visits_upper': 732816,
 'projected_batch.work.selection_component_visits_upper': 732816,
 'projected_batch.work.selection_endpoint_collection_writes_upper': 372224,
 'projected_batch.work.selection_endpoint_dedup_comparisons_upper': 360592,
 'projected_batch.work.selection_endpoint_sort_comparisons_upper': 5769472,
 'projected_batch.work.selection_intersection_reductions_upper': 348960,
 'projected_batch.work.selection_label_sort_comparisons_upper': 1395840,
 'projected_batch.work.selection_membership_probes_upper': 11725056,
 'projected_batch.work.selection_witness_probes_upper': 186112,
 'projected_batch.work.wrapper_debug_format_calls_upper': 3223518}
PROJECTED_BATCH_TELEMETRY = {
    "status": 'admitted-source-owned-observer',
    "counter_family": PROJECTED_BATCH.MARKER,
    "guard_counter_family": PROJECTED_BATCH.GUARD_MARKER,
    "observed_fields": list(PROJECTED_BATCH.FIELDS),
    "guard_observed_fields": list(PROJECTED_BATCH.GUARD_FIELDS),
    "case_recipes": PROJECTED_BATCH.case_recipes(),
    "guard_recipes": PROJECTED_BATCH.guard_recipes(),
    "source_owned_work_per_batch": PROJECTED_BATCH.SOURCE_WORK_PER_BATCH,
    "source_owned_work_per_getter": PROJECTED_BATCH.SOURCE_WORK_PER_GETTER,
    "source_owned_private_work_per_package": PROJECTED_BATCH.SOURCE_PRIVATE_WORK,
    "literal_doctests_added": PROJECTED_BATCH.LITERAL_DOCTESTS_ADDED,
    "global_limits": PROJECTED_BATCH_GLOBAL_LIMITS,
}
CORE_EQUIVALENCE_SOURCE = 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e'
CORE_EQUIVALENCE_PROOF = 'tasks/s1.3-selection-equivalences.md'
CORE_EQUIVALENCE_IDENTITIES = {
    'TC-S13-E-001': 'crates/time-core/src/selection.rs:694:26: replace || with && in maximum_components',
    'TC-S13-E-002': 'crates/time-core/src/selection.rs:756:52: replace > with < in common_witness',
    'TC-S13-E-003': 'crates/time-core/src/selection.rs:756:62: replace || with && in common_witness',
}
ENCLOSURE_EQUIVALENCE_SOURCE = '219142ff1ce58475b4707306e5f85909eb03918d9b3e21d0f5181e2bee49fe36'
ENCLOSURE_EQUIVALENCE_PROOF = 'tasks/s1.3-enclosure-equivalences.md'
ENCLOSURE_EQUIVALENCE_IDENTITIES = {
    'TC-S13-E-004': 'crates/time-core/src/projection_enclosure.rs:155:15: replace >> with << in center_admissible',
    'TC-S13-E-005': 'crates/time-core/src/projection_enclosure.rs:166:15: replace >> with << in distance_admissible',
    'TC-S13-E-006': 'crates/time-core/src/projection_enclosure.rs:251:31: replace < with <= in Fraction::compare',
    'TC-S13-E-007': 'crates/time-core/src/projection_enclosure.rs:314:28: replace | with ^ in ScaledValue::from_bits',
}
CORE_EQUIVALENCE_REPLACEMENTS = {
    'TC-S13-E-001': '&&', 'TC-S13-E-002': '<', 'TC-S13-E-003': '&&',
    'TC-S13-E-004': '<<', 'TC-S13-E-005': '<<', 'TC-S13-E-006': '<=', 'TC-S13-E-007': '^',
}
FUZZ_PATHS = {"manifest": "policy/time-core-fuzz-manifest.toml", "lockfile": "policy/time-core-fuzz.lock",
              "target_source": "policy/time-core-fuzz-target.rs"}
FUZZ_INPUTS = (*FUZZ_PATHS.values(), "policy/time-domain-analysis.json", "policy/time-domain-fuzz.lock",
               "scripts/verify-time-domain-analysis.py", "policy/ntp-wire-analysis.json",
               "policy/dependency-notices/libfuzzer-sys-0.4.13-LLVM-LICENSE.txt")
LEGACY_PROFILE_LIMITS = {'charged_fit_work_upper': 640, 'point_admissions_upper': 65536}
PROFILE_LIMITS = {'charged_fit_work_upper': 1888, 'point_admissions_upper': 108000}
# Fail closed until the coordinator admits exact runs receipts and campaign budgets.
# This is an admission hold, not a new policy approval or executed observation.
RUNS_TELEMETRY_STATUS = 'admitted-reviewed-runs-telemetry-and-campaign-budget'
# Registration draft only: actual parser/control/body admission is still gated.
HISTORY_TELEMETRY = {
    'status': 'admitted-source-owned-observer',
    'counter_family': 'CORE_HISTORY_PROFILE_COUNTS',
    'observed_fields': ['histories', 'appends', 'observations', 'contexts', 'monotonic',
                        'coordinates', 'exchanges', 'getters', 'normalizations', 'stats', 'fits', 'points'],
    'profile_limits': {'histories': 32, 'appends': 160, 'observations': 256, 'contexts': 64,
                       'monotonic': 256, 'coordinates': 1024, 'exchanges': 256, 'getters': 65536,
                       'normalizations': 0, 'stats': 0, 'fits': 0, 'points': 0},
    'source_owned_per_append_upper': {'normalizer': 1, 'statistics': 1, 'inner_fit': 1,
                                    'durations': 64, 'generated_points': 64, 'FitInput': 1,
                                    'staging_Copy_assignments': 127},
}
LITERAL_DOCTEST_LIMITS = {
    'total_source_doctests_maximum': 5, 'legal_source_doctests_maximum': 2,
    'direct_fit_calls_upper': 0, 'stats_calls_upper': 0, 'point_admissions_upper': 3,
    'normalization_requests_upper': 0, 'observation_constructor_attempts_upper': 0,
    'monotonic_constructor_attempts_upper': 0, 'utc_coordinate_constructor_attempts_upper': 0,
    'exchange_arithmetic_calls_upper': 0, 'context_constructions_upper': 0,
    'point_getter_requests_upper': 0, 'generated_point_admissions_upper': 0,
    'duration_operations_upper': 0, 'fit_input_constructions_upper': 0,
    'history_constructions_upper': 0, 'history_append_requests_upper': 0,
    'history_product_getter_requests_upper': 0,
    'history_normalization_entries_upper': 0, 'history_statistics_entries_upper': 0,
    'history_statistics_inner_fit_upper': 0,
    'projection_terms_constructor_attempts_upper': 1,
    'projection_terms_getter_requests_upper': 2,
    'projection_term_garde_rule_callbacks_upper': 3,
    'projection_term_predicate_calls_upper': 6,
}
EXTERNAL_CONSUMER_LIMITS = {"legal_executions": 1, "negative_compilations": 7}
GLOBAL_LIMITS = {'charged_fit_work_upper': 4515836, 'point_admissions_upper': 190523037}
SELECTION_PACKAGE_RESERVATION = {"intervals": 492, "minima": 72, "batches": 81,
                                 "selections": 66, "garde_validator_invocations_upper": 1056}
SELECTION_HELPER_RESERVATION = {"read_adapters": 2, "prefix_sorts": 3,
                                "cell_depths": 2, "label_projections": 5,
                                "endpoint_collections": 5, "depth_builds": 5,
                                "component_scans": 5, "witness_builds": 4}
SELECTION_HELPER_CASE_RESERVATIONS = {
    "capacity_neighbor": {"read_adapters": 2, "prefix_sorts": 3, "cell_depths": 2,
                          "label_projections": 5, "endpoint_collections": 2,
                          "depth_builds": 2, "component_scans": 3, "witness_builds": 4},
    "duplicate_recovery": {"read_adapters": 0, "prefix_sorts": 0, "cell_depths": 0,
                           "label_projections": 0, "endpoint_collections": 3,
                           "depth_builds": 3, "component_scans": 2, "witness_builds": 0},
}
# Reviewed checkpoint: 327 variants + two baselines + one coverage = 330
# package recipes, plus one genuine principal legal consumer. These reserve source calls, not actual CPU/RSS
# or arbitrary work inside a defective variant; native limits remain separate.
SELECTION_PACKAGE_INVOCATIONS_UPPER = 330
SELECTION_GLOBAL_LIMITS = {'batches': 58891,
 'garde_validator_invocations_upper': 767727,
 'intervals': 357690,
 'minima': 52347,
 'selections': 47984}
SELECTION_HELPER_GLOBAL_LIMITS = {'cell_depths': 1454,
 'component_scans': 3635,
 'depth_builds': 3635,
 'endpoint_collections': 3635,
 'label_projections': 3635,
 'prefix_sorts': 2181,
 'read_adapters': 1454,
 'witness_builds': 2908}
# Direct raw helpers add nested canonical work outside public evaluator recipes:
# sorts: (3 prefix + 5 endpoint)*496; memberships: 2*16 + 5*1008;
# label sorts/projections: (5 direct + 4 witness)*120/16. Rejected calls
# conservatively keep their full named-operation upper; these are not receipts.
SELECTION_HELPER_CANONICAL_WORK_LIMITS = {
    "direct_read_adapter_attempts_upper": 2,
    "endpoint_collection_writes_upper": 160,
    "prefix_and_endpoint_sort_comparisons_upper": 3968,
    "endpoint_dedup_comparisons_upper": 155,
    "cell_visits_upper": 315,
    "membership_probes_upper": 5072,
    "component_visits_upper": 315,
    "witness_probes_upper": 64,
    "label_sort_comparisons_upper": 1080,
    "label_prefix_assignments_upper": 144,
    "intersection_reductions_upper": 120,
}
SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS = {
    key: value * SELECTION_PACKAGE_INVOCATIONS_UPPER
    for key, value in SELECTION_HELPER_CANONICAL_WORK_LIMITS.items()
}
SOURCE_RAW_PRIVATE_WORK_LIMITS = {"point_candidate_validation_attempts": 28,
                                  "garde_rule_callbacks_upper": 84,
                                  "coefficient_constructions": 16,
                                  "square_root_calls_upper": 33,
                                  "binary_arithmetic_calls_upper": 22}
SOURCE_RAW_PRIVATE_GLOBAL_LIMITS = {'coefficient_constructions': 11632,
 'garde_rule_callbacks_upper': 61068,
 'point_candidate_validation_attempts': 20356,
 'square_root_calls_upper': 0,
 'binary_arithmetic_calls_upper': 0}
SOURCE_RAW_PRIVATE_WORK = {
    "scope": "Source-owned direct raw unit-test calls only; callback upper applies to point-candidate attempts. Not observations, all-core garde totals, coefficient-internal/nested work or CPU/RSS.",
    "per_package": SOURCE_RAW_PRIVATE_WORK_LIMITS,
    "global_limits": SOURCE_RAW_PRIVATE_GLOBAL_LIMITS,
}
SELECTION_TELEMETRY = {
    "status": "admitted-source-owned-observer",
    "counter_families": ["S13_SELECTION_PROFILE", "S13_BATCH_GUARD_PROFILE",
                           "SELECTION_HELPER_PROFILE", "CORE_SELECTION_EXTERNAL_PROFILE"],
    "package_tests": FINAL_PACKAGE_TEST_COUNTS,
    "package_invocations_upper": SELECTION_PACKAGE_INVOCATIONS_UPPER,
    "package_reservation": SELECTION_PACKAGE_RESERVATION,
    "helper_case_records": 2,
    "helper_case_reservations": SELECTION_HELPER_CASE_RESERVATIONS,
    "helper_reservation": SELECTION_HELPER_RESERVATION,
    "helper_canonical_work_scope": "Named canonical operations from direct raw-helper calls, including nested helper work; source-owned uppers only, not observations, all-private work or arbitrary-mutant/CPU/RSS bounds.",
    "helper_canonical_source_work_per_package": SELECTION_HELPER_CANONICAL_WORK_LIMITS,
    "helper_canonical_source_work_global_limits": SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS,
    "global_limits": SELECTION_GLOBAL_LIMITS,
    "helper_global_limits": SELECTION_HELPER_GLOBAL_LIMITS,
}
# Source-authored observer schema; actual controls and admission are root-owned.
NORMALIZATION_TELEMETRY = {
    'status': 'admitted-source-owned-observer',
    'counter_family': 'CORE_NORMALIZATION_PROFILE_COUNTS',
    'observed_fields': [
        'normalizations', 'observations', 'contexts', 'monotonic',
        'coordinates', 'exchanges', 'getters', 'stats', 'points'
    ],
    'source_owned_upper_fields': [
        'duration_operations_upper',
        'generated_point_admissions_upper',
        'fit_input_constructions_upper'
    ],
    'profile_limits': {
        'normalization_requests': 128,
        'observation_constructor_attempts': 8320,
        'monotonic_constructor_attempts': 8320,
        'duration_operations_upper': 8192,
        'generated_point_admissions_upper': 8192,
        'fit_input_constructions_upper': 128,
        'context_constructions': 256,
        'point_getter_requests_per_prepared_result': 66,
        'point_getter_requests_upper': 8448,
        'raw_controls': 32,
        'utc_coordinate_constructor_attempts_upper': 33280,
        'exchange_arithmetic_calls_upper': 8320,
        'additional_downstream_stats_calls_upper': 16
    }
}
PRINCIPAL_FIELDS17 = ('histories', 'appends', 'observations', 'contexts', 'monotonic',
                     'coordinates', 'exchanges', 'getters', 'terms', 'projections', 'enclosures',
                     'batches', 'minima', 'parameters', 'strata', 'seeds', 'advances')
PRINCIPAL_FIELDS = (*PRINCIPAL_FIELDS17, 'state_clones', 'value_equalities', 'state_formats')
PRINCIPAL_FIRST = 'principal_seed_and_genuine_update_hysteresis'
PRINCIPAL_PRIVATE_CASE = 'principal_private_decision_relations'
PRINCIPAL_PRIVATE_WORK = {
    'case': PRINCIPAL_PRIVATE_CASE,
    'explicit_selection_calls_upper': 7,
    'direct_complete_decision_calls_upper': 7,
    'key_comparisons_per_completion_upper': 64,
    'internal_product_getters_per_completion_upper': 64,
}
# Source ceilings, separately from exact observed healthy rows at the frozen
# 95-input fixture. A failed prefix never shrinks the pre-effect reservation.
PRINCIPAL_CASE_CEILINGS = {
    PRINCIPAL_FIRST: (2,10,10,2,64,1,1,320,18,18,18,9,1,1,2,1,9,0,0,0),
    'principal_threshold_exact_and_neighbors': (22,66,66,22,99,44,22,192,22,22,22,11,3,3,22,3,11,0,0,0),
    'principal_unrelated_event_and_pending_wait': (24,72,72,24,104,48,24,192,24,24,24,8,1,1,24,1,8,0,0,0),
    'principal_lifecycle_rejection_is_atomic_and_recovers': (28,84,84,28,126,56,28,204,28,28,28,14,1,1,28,1,20,18,24,6),
    'principal_agreement_loss_preserves_minimum_and_observation_identity': (29,87,87,29,126,58,29,224,29,29,29,10,5,5,29,5,15,0,0,0),
    'principal_cached_absence_and_present_nonmember_keep_score_order': (11,33,33,11,48,22,11,96,11,11,11,4,1,1,11,1,4,2,0,0),
    'principal_raw_scalar_and_stratum_neighbors': (0,0,0,0,0,0,0,24,0,0,0,0,0,20,5,0,0,0,1,0),
    'principal_metadata_and_roster_rejections_preserve_state': (4,12,12,4,18,8,4,96,4,4,4,3,1,1,5,5,11,5,10,5),
    'principal_numeric_rejection_is_atomic_and_recovers': (6,18,18,6,27,12,6,64,6,6,6,3,1,1,6,1,4,2,2,1),
    'principal_cardinality_zero_distance_and_stratum_weight_neighbors': (36,108,108,36,149,72,36,256,36,36,36,5,5,5,36,5,5,0,0,0),
    'principal_error_sources_preserve_embedded_identity': (0,0,0,0,0,0,0,18,0,0,0,0,0,0,0,0,0,0,0,0),
    'principal_equality_distinguishes_context_and_pending_credit': (2,6,6,2,10,4,2,40,2,2,2,2,1,1,2,2,1,1,4,0),
    'principal_inferior_update_floors_score_without_reprojection_credit': (4,12,12,4,18,8,4,64,4,4,4,2,1,1,4,1,3,1,1,0),
    PRINCIPAL_PRIVATE_CASE: (2,6,6,2,3,1,1,96,2,2,2,3,1,1,2,1,4,0,0,0),
}
PRINCIPAL_NOMINAL_GETTERS = dict(zip(PRINCIPAL_CASE_CEILINGS,
    (259,153,128,136,179,68,24,57,28,231,18,28,43,63), strict=True))
PRINCIPAL_CASE_RECIPES = {
    case: tuple((33 if case == PRINCIPAL_FIRST and index == 4 else
                 PRINCIPAL_NOMINAL_GETTERS[case] if index == 7 else value)
                for index, value in enumerate(values))
    for case, values in PRINCIPAL_CASE_CEILINGS.items()}
PRINCIPAL_TELEMETRY = {
    'status': 'pending-final-source-and-cumulative-budget',
    'fields': list(PRINCIPAL_FIELDS), 'case_ceilings': PRINCIPAL_CASE_CEILINGS,
    'case_recipes': PRINCIPAL_CASE_RECIPES, 'global_limits': None,
    'private_case_source_owned_uppers': PRINCIPAL_PRIVATE_WORK,
    'mandatory_garde_native_result_use_evidence': None,
    'scope': 'Full principal integration suite only; source upper units and actual prefixes are distinct.',
}
COMBINATION_TELEMETRY = {
    'status': 'pending-final-source-and-cumulative-budget',
    'counter_family': COMBINATION.MARKER,
    'fields': list(COMBINATION.FIELDS), 'case_recipes': COMBINATION.case_recipes(),
    'source_owned_work_per_uncertainty': COMBINATION.SOURCE_WORK_PER_UNCERTAINTY,
    'source_owned_work_per_combination': COMBINATION.SOURCE_WORK_PER_COMBINATION,
    'global_limits': None, 'literal_doctests_added': 0,
    'scope': 'Twelve disjoint public-consumer cases; producer chains charged once and kernel module uppers separately owned.',
}


def checkpoint_scope_claim(subjects):
    """A complete selected checkpoint never claims the eleven-file manifest."""
    if (type(subjects) is not dict or not subjects
            or any(name not in SUBJECTS or SUBJECTS[name] != path for name, path in subjects.items())):
        raise Incomplete('Unknown checkpoint scope.')
    return {'checkpoint_complete_selection': set(subjects) == set(SUBJECTS),
            'critical_manifest_complete_selection': False,
            'qualification_scope': 'Selected four changed or coupled critical owners; bounded checkpoint evidence only.'}


FIXED = {"schema_version": 11, "source_file_count": SOURCE_FILE_COUNT,
         "graph_status": GRAPH_STATUS, "final_package_test_counts": FINAL_PACKAGE_TEST_COUNTS,
         "projection_telemetry": PROJECTION_TELEMETRY, "coordinator_cohorts": COORDINATOR_COHORTS,
         "enclosure_telemetry": ENCLOSURE_TELEMETRY,
         "projected_batch_telemetry": PROJECTED_BATCH_TELEMETRY,
         "principal_telemetry": PRINCIPAL_TELEMETRY,
         "combination_telemetry": COMBINATION_TELEMETRY,
         "historical_projection_test_counts": HISTORICAL_PROJECTION_TEST_COUNTS,
         "historical_projection_mutation_inventories": HISTORICAL_PROJECTION_MUTATION_INVENTORIES,
         "mutation_inventories": MUTATION_INVENTORIES,
         "history_telemetry": HISTORY_TELEMETRY,
         "normalization_telemetry": NORMALIZATION_TELEMETRY,
         "selection_telemetry": SELECTION_TELEMETRY,
         "source_owned_private_unit_work": SOURCE_RAW_PRIVATE_WORK,
         "literal_doctest_limits": LITERAL_DOCTEST_LIMITS,
         "external_consumer_limits": EXTERNAL_CONSUMER_LIMITS, "package": "time-core", "critical_files": list(SUBJECTS.values()),
         "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
         "maximum_mutants": 512, "coverage_wall_seconds": 300, "coverage_test_threads": 1,
         "mutation_wall_seconds": 1200,
         "mutation_build_seconds": 90, "mutation_test_seconds": 60, "contracts_wall_seconds": 300,
         "jobs": 6, "test_threads": COMMON.RUNTIME.MAX_JOBS,
         "mutation_jobs": 10,
         "mutation_process_limit": 512,
         "mutation_message_format": "json",
         "legacy_profile_limits": LEGACY_PROFILE_LIMITS,
         "profile_limits": PROFILE_LIMITS, "global_limits": GLOBAL_LIMITS}


def exact(value, expected):
    """Compare JSON values including boolean/integer representation differences."""
    return json.dumps(value, sort_keys=True) == json.dumps(expected, sort_keys=True)


def inventory_tool_inputs(root):
    """Name the complete bounded Python tooling closure, without executing it."""
    scripts = root / "scripts"
    if scripts.is_symlink() or not scripts.is_dir():
        raise Incomplete("Inventory tooling directory is missing or linked.")
    names = {"policy/analysis-tools.json"}
    for path in scripts.rglob("*.py"):
        if "__pycache__" in path.parts:
            continue
        if path.resolve(strict=True) != path or not path.is_file():
            raise Incomplete("Inventory tooling contains a linked or nonregular input.")
        names.add(path.relative_to(root).as_posix())
        if len(names) > 128:
            raise Incomplete("Inventory tooling closure exceeds its fixed input allowance.")
    if not INVENTORY_REQUIRED_TOOLS <= names:
        raise Incomplete("Inventory tooling closure omits a required entry point.")
    return names


def inventory_admission_live(policy):
    """Reject future, expired or widened one-hour discovery admissions."""
    created, expires = policy.get("created_unix_seconds"), policy.get("expires_unix_seconds")
    if (type(created) is not int or type(expires) is not int or created <= 0
            or expires - created != 3600 or not created <= time.time() < expires):
        raise Incomplete("Inventory admission is not within its fixed one-hour validity window.")


def _core_json_document(path, *, role):
    """Decode one complete bounded regular JSON document under a closed role."""
    if type(role) is not str or role not in {"inventory", "native-receipt"}:
        raise Incomplete("Unreviewed core JSON document role.")
    maximum = 512 * 1024 if role == "inventory" else 8 * 1024 * 1024
    label = "Inventory admission" if role == "inventory" else "Native mutation receipt"
    if path.resolve(strict=True) != path or not path.is_file():
        kind = "Inventory document" if role == "inventory" else "Native mutation receipt"
        raise Incomplete(kind + " is linked or nonregular.")
    with path.open("rb") as stream:
        data = stream.read(maximum + 1)
    if len(data) > maximum:
        raise Incomplete(label + " exceeds its fixed byte allowance.")

    def pairs(items):
        result = {}
        for name, value in items:
            if name in result:
                raise Incomplete(label + " contains a duplicate JSON field.")
            result[name] = value
        return result

    def constant(_value):
        raise Incomplete(label + " contains a non-finite JSON number.")

    return json.loads(data, object_pairs_hook=pairs, parse_constant=constant)


def inventory_document(path):
    """Keep inventory, coordinator-plan and configuration admission at 512 KiB."""
    return _core_json_document(path, role="inventory")


def native_receipt_document(path):
    """Read one native mutation verification receipt, bounded to eight MiB."""
    return _core_json_document(path, role="native-receipt")


def read_inventory_admission(root, path):
    """Bind one strict discovery document and every product/tool input byte.

    This accepts no campaign, coverage, contract or fuzz admission. The ordinary
    campaign policy and its source-count/critical-manifest checks remain intact.
    """
    root = Path(root).resolve(strict=True)
    path = Path(path)
    path = path if path.is_absolute() else root / path
    expected = root / INVENTORY_ADMISSION
    if path != expected or path.resolve(strict=True) != expected or not path.is_file():
        raise Incomplete("Use the exact canonical in-checkout inventory admission path.")
    policy = inventory_document(path)
    fields = {*INVENTORY_FIXED, "root", "created_unix_seconds", "expires_unix_seconds",
              "product_inputs_sha256", "tool_inputs_sha256"}
    if (not isinstance(policy, dict) or set(policy) != fields
            or any(not exact(policy.get(name), value) for name, value in INVENTORY_FIXED.items())
            or policy["root"] != str(root)):
        raise Incomplete("Unreviewed inventory-only schema, scope, root or allowance.")
    inventory_admission_live(policy)
    maps = ((policy["product_inputs_sha256"], set(INVENTORY_BOUND_INPUTS)),
            (policy["tool_inputs_sha256"], inventory_tool_inputs(root)))
    for identities, names in maps:
        if not isinstance(identities, dict) or set(identities) != names:
            raise Incomplete("Inventory-only input binding set differs from the complete admitted scope.")
        for name, digest in identities.items():
            source = root / name
            if (not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest)
                    or source.resolve(strict=True) != source or PRODUCT.sha256(source) != digest):
                raise Incomplete("Inventory-only source/tool input is unbound, linked or changed.")
    _, product = PRODUCT.admit_product(root)
    if (len(product) != INVENTORY_SOURCE_FILE_COUNT
            or set(product) != set(INVENTORY_BOUND_INPUTS)
            or product != policy["product_inputs_sha256"]):
        raise Incomplete("Inventory requires the exact current complete product and dependency graph.")
    return policy


def historical_admit_core_equivalence_scope(entries):
    """Allow empty or an exact reviewed principal/legacy closed fault set.

    The shared reader additionally checks the full descriptors, candidate,
    source/proof bytes and independent review references. Campaign accounting
    must still observe each exact fault as missed before subtracting it.
    """
    if not isinstance(entries, list) or len(entries) not in (0, 1, 3, 7):
        raise Incomplete('Core equivalence scope is not one reviewed closed set.')
    if not entries:
        return
    if len(entries) == 1:
        entry = entries[0]
        if (not isinstance(entry, dict) or entry.get('id') != 'S13-P-E-001'
                or entry.get('source_sha256') != '876b647b1d8f1df7156b7383c42fcc0a0a3497e0561ebb331079c120888da658'
                or not isinstance(entry.get('mutant'), dict)
                or entry['mutant'].get('name') != 'crates/time-core/src/principal.rs:945:17: replace < with <= in updated_score'
                or entry['mutant'].get('file') != SUBJECTS['principal']
                or entry['mutant'].get('replacement') != '<='
                or not isinstance(entry.get('proof'), dict)
                or entry['proof'].get('path') != 'tasks/s1.3-principal-equivalences.md'):
            raise Incomplete('Principal floor equivalence differs from its exact reviewed boundary.')
        return
    if any(not isinstance(entry, dict) or not isinstance(entry.get('id'), str)
           or not isinstance(entry.get('mutant'), dict)
           or not isinstance(entry['mutant'].get('name'), str)
            or not isinstance(entry['mutant'].get('file'), str)
            or not isinstance(entry['mutant'].get('replacement'), str)
           or not isinstance(entry.get('proof'), dict)
           for entry in entries):
        raise Incomplete('Core equivalence identities are malformed.')
    selection_ids = set(CORE_EQUIVALENCE_IDENTITIES)
    combined_ids = selection_ids | set(ENCLOSURE_EQUIVALENCE_IDENTITIES)
    identifiers = {entry['id'] for entry in entries}
    if len(identifiers) != len(entries) or identifiers not in (selection_ids, combined_ids):
        raise Incomplete('Core equivalence identities are not one complete reviewed closed set.')
    for entry in entries:
        identifier = entry['id']
        selection = identifier in selection_ids
        identities = CORE_EQUIVALENCE_IDENTITIES if selection else ENCLOSURE_EQUIVALENCE_IDENTITIES
        source = CORE_EQUIVALENCE_SOURCE if selection else ENCLOSURE_EQUIVALENCE_SOURCE
        proof = CORE_EQUIVALENCE_PROOF if selection else ENCLOSURE_EQUIVALENCE_PROOF
        filename = HISTORICAL_SUBJECTS['selection'] if selection else HISTORICAL_SUBJECTS['enclosure']
        if (entry['mutant']['name'] != identities[identifier]
                or entry['mutant']['file'] != filename
                or entry['mutant']['replacement'] != CORE_EQUIVALENCE_REPLACEMENTS[identifier]
                or entry.get('source_sha256') != source or entry['proof'].get('path') != proof):
            raise Incomplete('Core equivalence identity, file, operator, own source or maintained proof differs.')


def admit_core_equivalence_scope(entries):
    """Require a fresh proof before excluding any changed-candidate mutant.

    Earlier exact fault sets remain available to historical receipt readers;
    they are not reusable across this candidate's changed arithmetic/projection.
    """
    if type(entries) is not list or entries:
        raise Incomplete('Current combination checkpoint equivalences require fresh exact-source review; old E events are historical.')


def read_policy(root, lane="all"):
    """Admit fixed scope/budgets and current bindings before any code executes."""
    if lane not in {'contracts', 'coverage', 'mutation', 'cohort-union'}:
        raise Incomplete('This checkpoint admits coverage/contracts and four named mutation cohorts only.')
    policy = json.loads((root / POLICY).read_text())
    permitted = {*FIXED, "admission_status", "scope", "inputs_sha256", "mutation_equivalences", "coverage_guided_fuzzing",
                 "historical_projection_inputs_sha256", "historical_projection_scope"}
    if set(policy) != permitted or any(not exact(policy.get(k), v) for k, v in FIXED.items()):
        raise Incomplete("Unreviewed core analysis scope, threshold or budget.")
    if policy["admission_status"] != "admitted-final-source":
        raise Incomplete("Core analysis awaits coordinator final-source/graph binding.")
    if policy["graph_status"] != "admitted-final-source-inventory-and-cohort-binding":
        raise Incomplete("Projection source inventory, recipes and cumulative cohort binding remain pending.")
    if (set(MUTATION_INVENTORIES) != set(SUBJECTS)
            or any(type(row) is not dict or set(row) != {'count', 'sha256'}
                   or type(row['count']) is not int or not 1 <= row['count'] <= 512
                   or type(row['sha256']) is not str or re.fullmatch(r'[0-9a-f]{64}', row['sha256']) is None
                   for row in MUTATION_INVENTORIES.values())):
        raise Incomplete('Four actual current mutation inventories remain pending; no campaign or checkpoint lane is admitted.')
    if (policy["projection_telemetry"]["status"] != "admitted-source-owned-observer"
            or policy["coordinator_cohorts"]["status"] != "admitted-exact-source-and-cohorts"
            or any(type(value) is not int or value <= 0 for value in FINAL_PACKAGE_TEST_COUNTS.values())
            or FINAL_PACKAGE_TEST_COUNTS["baseline"] != FINAL_PACKAGE_TEST_COUNTS["ordinary"] + FINAL_PACKAGE_TEST_COUNTS["doctests"]):
        raise Incomplete("Actual projection test discovery/source recipes/cohort review remain pending.")
    if (policy["enclosure_telemetry"]["status"] != "admitted-source-owned-observer"
            or not exact(policy["enclosure_telemetry"]["case_recipes"], ENCLOSURE.case_recipes())
            or not exact(policy["enclosure_telemetry"]["source_owned_private_work_per_package"], ENCLOSURE.SOURCE_PRIVATE_WORK)
            or policy["enclosure_telemetry"]["literal_doctests_added"] != ENCLOSURE.LITERAL_DOCTESTS_ADDED):
        raise Incomplete("Enclosure source recipes, literal doctests and cumulative binding remain pending.")
    if (policy['projected_batch_telemetry']['status'] != 'admitted-source-owned-observer'
            or not exact(policy['projected_batch_telemetry']['case_recipes'], PROJECTED_BATCH.case_recipes())
            or not exact(policy['projected_batch_telemetry']['guard_recipes'], PROJECTED_BATCH.guard_recipes())
            or not exact(policy['projected_batch_telemetry']['source_owned_work_per_batch'], PROJECTED_BATCH.SOURCE_WORK_PER_BATCH)
            or not exact(policy['projected_batch_telemetry']['source_owned_work_per_getter'], PROJECTED_BATCH.SOURCE_WORK_PER_GETTER)
            or not exact(policy['projected_batch_telemetry']['source_owned_private_work_per_package'], PROJECTED_BATCH.SOURCE_PRIVATE_WORK)
            or policy['projected_batch_telemetry']['literal_doctests_added'] != PROJECTED_BATCH.LITERAL_DOCTESTS_ADDED):
        raise Incomplete('Projected-batch source, passive-observer, literal-doctest and budget binding remain pending.')
    if (policy['principal_telemetry']['status'] != 'admitted-source-owned-profiles'
            or not exact(policy['principal_telemetry']['case_ceilings'], PRINCIPAL_CASE_CEILINGS)
            or not exact(policy['principal_telemetry']['case_recipes'], PRINCIPAL_CASE_RECIPES)):
        raise Incomplete('Principal full-suite profiles and final source/budget binding remain pending.')
    if (policy['combination_telemetry']['status'] != 'admitted-source-owned-profiles'
            or not exact(policy['combination_telemetry']['case_recipes'], COMBINATION.case_recipes())
            or not exact(policy['combination_telemetry']['fields'], list(COMBINATION.FIELDS))
            or policy['combination_telemetry']['literal_doctests_added'] != 0):
        raise Incomplete('Combination full-suite profiles and final source/budget binding remain pending.')
    admit_principal_allowances(policy['principal_telemetry']['global_limits'])
    admit_combination_allowances(policy['combination_telemetry']['global_limits'])
    admit_projected_batch_allowances(policy['projected_batch_telemetry']['global_limits'])
    if (not exact(SELECTION.full_package_reservation(), SELECTION_PACKAGE_RESERVATION)
            or not exact(SELECTION.full_package_helper_reservation(), SELECTION_HELPER_RESERVATION)
            or not exact({case: dict(zip(SELECTION.HELPER_FIELDS, counts, strict=True))
                          for case, counts in SELECTION.HELPER_RECIPES},
                         SELECTION_HELPER_CASE_RESERVATIONS)):
        raise Incomplete("Selection source-call or raw-helper reservation differs from review.")
    if RUNS_TELEMETRY_STATUS != 'admitted-reviewed-runs-telemetry-and-campaign-budget':
        raise Incomplete('Runs telemetry and complete campaign budget remain pending.')
    if policy['history_telemetry']['status'] != 'admitted-source-owned-observer':
        raise Incomplete('History observer, actual caller recipes and finite budget remain pending.')
    admit_core_equivalence_scope(policy["mutation_equivalences"])
    if policy["normalization_telemetry"]["status"] != "admitted-source-owned-observer":
        raise Incomplete("Normalization observer, literal callers and finite campaign remain unreviewed.")
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    if set(policy["inputs_sha256"]) != set(BOUND_INPUTS):
        raise Incomplete("Core analysis source/configuration binding set differs.")
    for name, digest in policy["inputs_sha256"].items():
        if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest) or PRODUCT.sha256(root / name) != digest:
            raise Incomplete("Core source/graph input is unbound or changed.")
    inventory = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    _, actual_product = PRODUCT.admit_product(root)
    if len(actual_product) != SOURCE_FILE_COUNT or set(actual_product) != set(BOUND_INPUTS):
        raise Incomplete(f"Core analysis requires the exact complete{SOURCE_FILE_COUNT}-file product inventory.")
    if actual_product != policy["inputs_sha256"]:
        raise Incomplete("Complete product source differs from the final frozen policy.")
    for filename in {**HISTORICAL_SUBJECTS, **SUBJECTS}.values():
        if (inventory.get("critical_files", []).count(filename) != 1
                or sum(m.get("package") == "time-core" and filename in m.get("files", [])
                       for m in inventory.get("security_modules", [])) != 1):
            raise Incomplete("One core critical/security file is absent or duplicated.")
        if re.search(r"#\s*\[\s*test\s*\]", (root / filename).read_text()):
            raise Incomplete("Inline test bodies would pad a core production denominator.")
    if lane in {"fuzz", "all"}:
        selected = policy["coverage_guided_fuzzing"]
        if (selected.get("status") != "admitted-test-only-source"
                or selected.get("profile") != COMMON.RUNTIME.HISTORY_SANITIZER_PROFILE
                or selected.get("target_contract") != "FixtureHistory::try_append-and-committed-observation-report-public-entrypoints"
                or selected.get("runtime_profile_reuse_review") != "reviewed-history-source-and-resource-envelope"
                or selected.get("fuzzer_flags") != list(COMMON.RUNTIME.HISTORY_SANITIZER_FLAGS)
                or any(selected.get(k) != v for k, v in FUZZ_PATHS.items())
                or set(selected.get("inputs_sha256", {})) != set(FUZZ_INPUTS)):
            raise Incomplete("Core standalone fuzz graph/profile/source admission is incomplete.")
        for name, digest in selected["inputs_sha256"].items():
            if not isinstance(digest, str) or PRODUCT.sha256(root / name) != digest:
                raise Incomplete("Core fuzz source/cohort/notice input changed.")
        limits = {'raw_driver_runs': 4096, 'history_constructions': 4096, 'history_append_requests': 266240, 'history_normalizer_upper': 266240, 'history_stats_upper': 258048, 'internal_fit_upper_bound': 258048, 'charged_fit_work_upper': 258048, 'point_admissions_upper': 8769536, 'maximum_retained_rows': 64, 'maximum_append_attempts': 65, 'maximum_input_bytes': 1561, 'wall_seconds': 15}
        combined = {'invocations': 2, 'raw_driver_runs': 8192, 'history_constructions': 8192, 'history_append_requests': 532480, 'history_normalizer_upper': 532480, 'history_stats_upper': 516096, 'internal_fit_upper_bound': 516096, 'charged_fit_work_upper': 516096, 'point_admissions_upper': 17539072}
        builds = {"metadata_wall_seconds": 120, "build_wall_seconds_each": 600, "builds": 2}
        if (not exact(selected.get("invocation_limits"), limits)
                or not exact(selected.get("combined_limits"), combined)
                or not exact(selected.get("build_metadata_limits"), builds)
                or len(selected.get("locked_registry_packages", [])) != 23):
            raise Incomplete("Core fuzz operation/source-cohort budget differs from review.")
        cohort = json.loads((root / "policy/time-domain-analysis.json").read_text())["coverage_guided_fuzzing"]
        for key in ("locked_registry_packages", "selected_grants", "gnu_reachable_features"):
            if selected[key] != cohort[key]:
                raise Incomplete("Core fuzz changed the existing admitted registry/feature/grant cohort.")
        edges = {name: list(values) for name, values in cohort["gnu_dependency_edges"].items()
                 if name not in {"time-domain", "time-domain-fuzz"}}
        edges.update({"time-domain": ["garde"], "time-core": ["garde", "time-domain"],
                      "time-core-fuzz": ["libfuzzer-sys", "time-core", "time-domain"]})
        if selected["gnu_dependency_edges"] != edges:
            raise Incomplete("Core standalone dependency edges differ from the reviewed cohort adaptation.")
    return policy


def counter_rows(output, marker, fields, *, require_newline=False):
    """Read complete closed decimal records, rejecting malformed/trailing fields.

    The transport has no test identifiers: whole omitted zero rows or replay of
    two legitimate identical scopes require coordinator discovery/source custody.
    Each scalar is bounded to u64 before conversion; no permissive defaults.
    New runs-owner streams require their emitted newline. Older callers retain
    their normalized-string contract, whose final newline may have been stripped.
    """
    terminator = r'(?=\r?\n)' if require_newline else r'(?=\r?(?:\n|$))'
    pattern = marker + ''.join(' ' + field + r'=([0-9]+)' for field in fields) + terminator
    raw = re.findall(pattern, output)
    if output.count(marker) != len(raw):
        raise Incomplete("Counter family is malformed, truncated or contains extra fields.")
    if any(len(value) > 20 or int(value) > (1 << 64) - 1 for row in raw for value in row):
        raise Incomplete("Counter scalar exceeds its exact u64 representation.")
    return [tuple(int(value) for value in row) for row in raw]


def valid_runs_sign_receipt(line):
    """Validate the closed healthy synthetic receipt; error outcomes remain held."""
    number = r'(?:0|[1-9][0-9]{0,19})'
    facts = (r'Success\(SuccessFacts \{ start: 8, count: 4, frame: 17, stop: RunsPassed, '
             r'decision_start: 4, decision_count: 8, decision_runs: 3, fit_runs: 3, '
             r'statistics_frame: 17, statistics_count: 4, degrees: 2, line_frame: 17, '
             r'numeric_bits: \[' + ', '.join([number] * 5) + r'\] \}\)')
    pattern = (r'CORE_RUNS_SIGN_RECEIPT counts=Counts \{ points: 24, inputs: 2, selectors: 2 \} '
               r'observer=SignObserver \{ positive: Some\((?P<positive>' + number + r')\), '
               r'negative: Some\((?P<negative>' + number + r')\), positive_seen: Once, '
               r'negative_seen: Once, calls: (?P<calls>' + number + r') \} '
               r'observed=(?P<facts>' + facts + r') ordinary=(?P=facts) '
               r'preservation=\[true, true, true, true, true, true\] '
               r'identities=\[\(\(17, 12\), \(17, 12\)\), \(\(17, 12\), \(17, 12\)\)\]\r?\n')
    match = re.fullmatch(pattern, line)
    if not match or len(line.encode('utf-8')) > 4096:
        return False
    raw_bits = re.search(r'numeric_bits: \[([^]]+)\]', match.group('facts'))
    if raw_bits is None:
        return False
    values = [int(match.group('positive')), int(match.group('negative'))]
    values += [int(value) for value in raw_bits.group(1).split(', ')]
    if any(value > (1 << 64) - 1 for value in values) or int(match.group('calls')) != 102:
        return False
    floats = [struct.unpack('>d', value.to_bytes(8, 'big'))[0] for value in values]
    positive, negative, phase, rate, sse, scale, variance = floats
    epsilon = 1 / 65536
    expected_sse = 1 / 4096 + epsilon / 64 + 3 * epsilon * epsilon / 10
    return (2**-20 < positive < 2**-19 and -2**-17 < negative < -2**-18
            and abs(phase - (-1/128 + 7*epsilon/10)) <= 3e-9
            and abs(rate - (-3/128 + 3*epsilon/10)) <= 3e-9
            and abs(sse - expected_sse) <= 4e-6
            and abs(scale - expected_sse/2) <= 5e-6
            and abs(variance - expected_sse/2) <= 3e-5)


def runs_counts(output, *, require_complete=False, validate_receipt=True):
    """Charge actual disjoint runs-owner rows and retain bounded partial prefixes.

    Full-scope row counts also require the coordinator's selected-test/source
    discovery: identical unlabelled rows cannot prove distinct test identities.
    Derived work is a conservative upper, never observed inner-fit telemetry.
    """
    if (type(require_complete) is not bool or type(validate_receipt) is not bool
            or (require_complete and not validate_receipt)):
        raise Incomplete('Complete runs observations require healthy receipt validation.')
    external = counter_rows(output, 'CORE_RUNS_PROFILE', ('points', 'inputs', 'selections'), require_newline=True)
    signs = counter_rows(output, 'CORE_RUNS_SIGN_PROFILE', ('points', 'inputs', 'selectors'), require_newline=True)
    guards = counter_rows(output, 'CORE_RUNS_GUARDS',
        ('table', 'runs', 'contexts', 'points', 'inputs', 'prepares', 'direct_fits',
         'selectors', 'returned_window_rows'), require_newline=True)
    receipts = re.findall(r'CORE_RUNS_SIGN_RECEIPT [^\r\n]*\r?\n', output)
    if (output.count('CORE_RUNS_SIGN_RECEIPT') != len(receipts) or len(receipts) > 1
            or any(len(line.encode('utf-8')) > 4096
                   or (validate_receipt and not valid_runs_sign_receipt(line)) for line in receipts)):
        raise Incomplete('Runs sign receipt is duplicated, incomplete or exceeds its byte bound.')
    if len(external) > 9 or len(signs) > 1 or len(guards) > 4:
        raise Incomplete('Duplicate or extra runs-owner counter rows.')
    if len(receipts) > len(signs):
        raise Incomplete('Runs sign receipt has no corresponding source-owned attempt record.')
    if receipts and signs != [(24, 2, 2)]:
        raise Incomplete('Runs sign receipt contradicts its source-owned attempt record.')
    if any(points > 128 or inputs > 8 or selections > 8 or selections > inputs * 8
           for points, inputs, selections in external):
        raise Incomplete('Runs external scope exceeds its owned call ceilings.')
    if any(points > 24 or inputs > 2 or selectors > 2 or selectors > inputs
           for points, inputs, selectors in signs):
        raise Incomplete('Runs sign-observer scope exceeds its owned call ceilings.')
    limits = (64, 13, 21, 64, 1, 5, 0, 0, 196)
    if any(any(value > limit for value, limit in zip(row, limits, strict=True))
           or row[5] > 5 * row[4] or row[8] > 64 * row[5] for row in guards):
        raise Incomplete('Runs private helper scope exceeds its owned call ceilings.')
    if require_complete:
        expected_external = sorted([(75, 3, 3), (21, 3, 3), (12, 1, 1), (12, 1, 1),
                                    (12, 1, 1), (3, 1, 1), (15, 5, 5), (6, 2, 2), (3, 1, 1)])
        expected_guards = sorted([(64, 0, 0, 0, 0, 0, 0, 0, 0), (1, 13, 0, 0, 0, 0, 0, 0, 0),
                                  (0, 0, 21, 0, 0, 0, 0, 0, 0), (0, 0, 0, 64, 1, 5, 0, 0, 196)])
        if (sorted(external) != expected_external or signs != [(24, 2, 2)] or len(receipts) != 1
                or sorted(guards) != expected_guards):
            raise Incomplete('Complete runs scopes have missing, duplicate or partial records.')
    points = sum(row[0] for row in external) + sum(row[0] for row in signs) + sum(row[3] for row in guards)
    inputs = sum(row[1] for row in external) + sum(row[1] for row in signs) + sum(row[4] for row in guards)
    selections = sum(row[2] for row in external) + sum(row[2] for row in signs)
    candidates = sum(row[5] for row in guards)
    return {'runs_points': points, 'runs_inputs': inputs, 'runs_selections': selections,
            'runs_guard_candidates': candidates, 'runs_external_records': len(external),
            'runs_sign_records': len(signs), 'runs_guard_records': len(guards),
            'runs_guard_table_calls': sum(row[0] for row in guards),
            'runs_guard_count_calls': sum(row[1] for row in guards),
            'runs_guard_context_calls': sum(row[2] for row in guards),
            'runs_guard_returned_window_rows': sum(row[8] for row in guards),
            'runs_sign_receipts': len(receipts) if validate_receipt else 0,
            'runs_unqualified_sign_receipts': len(receipts) if not validate_receipt else 0,
            'runs_observer_hooks_upper': sum(3098 for row in signs if row[2] > 0),
            'runs_observer_pair_comparisons_upper': sum(12392 for row in signs if row[2] > 0),
            'runs_fit_upper': selections * 62 + candidates,
            'runs_point_copy_upper': selections * 2077 + candidates * 64,
            'runs_input_slots_upper': selections * 3968 + candidates * 64,
            'runs_residual_slots_upper': selections * 3968 + candidates * 64,
            'runs_decision_residual_upper': selections * 3098 + candidates * 64,
            'runs_checked_float_upper': selections * 53540 + candidates * (21 * 64 + 7 + 3 * 64)}


def selection_external_counts(output):
    """Parse only the bounded standalone legal consumer's native counter line.

    This is a different execution scope from the package's S13 test records.
    An absent record is None, not zero; native exit/sentinel/source checks and
    the complete expected recipe remain contracts() obligations.
    """
    marker = 'CORE_SELECTION_EXTERNAL_PROFILE'
    expected = {'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2}
    if marker not in output:
        return {'selection_external_consumer_records': 0,
                'selection_external_consumer_calls': None}
    decimal = r'(0|[1-9][0-9]{0,19})'
    pattern = re.compile(marker + ''.join(' ' + field + '=' + decimal for field in expected))
    records = []
    lines = output.split('\n')
    for index, raw in enumerate(lines):
        if marker not in raw:
            continue
        if index == len(lines)-1 or len(raw) > 512:
            raise Incomplete('Selection legal consumer counter is truncated or oversized.')
        match = pattern.fullmatch(raw[:-1] if raw.endswith('\r') else raw)
        if match is None:
            raise Incomplete('Selection legal consumer counter is outside its exact native line.')
        values = tuple(int(value) for value in match.groups())
        if (not any(values) or any(value > maximum for value, maximum in zip(values, expected.values(), strict=True))
                or (values[1] and values[0] != 6) or (values[2] and values[1] != 3)
                or (values[3] and values[2] != 4)):
            raise Incomplete('Selection legal consumer counter differs from its finite call order.')
        records.append(dict(zip(expected, values, strict=True)))
    if len(records) != 1:
        raise Incomplete('Selection legal consumer counter is duplicated.')
    return {'selection_external_consumer_records': 1,
            'selection_external_consumer_calls': records[0]}


def selection_package_counts(output, *, require_complete=False):
    """Preserve missing/partial package observations separately from outcomes."""
    try:
        return {**SELECTION.selection_counts(output, require_complete=require_complete),
                **SELECTION.helper_counts(output, require_complete=require_complete)}
    except SELECTION.Incomplete as error:
        raise Incomplete(str(error)) from error


def selection_reservation(*, package=False, external=None):
    """Return a fresh pre-effect source-call reservation with explicit scope."""
    if type(package) is not bool or (package and external is not None):
        raise Incomplete('Selection reservation scope is ambiguous.')
    if package:
        calls = dict(SELECTION_PACKAGE_RESERVATION)
        helpers = dict(SELECTION_HELPER_RESERVATION)
        helper_work = dict(SELECTION_HELPER_CANONICAL_WORK_LIMITS)
    else:
        if (not isinstance(external, dict) or set(external) != {'intervals', 'minima', 'batches', 'selections'}
                or any(type(v) is not int or not 0 <= v <= SELECTION.U64_MAX for v in external.values())
                or external != {'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2}):
            raise Incomplete('Selection external reservation requires exact nonnegative call fields.')
        calls = dict(external)
        calls['garde_validator_invocations_upper'] = 2*calls['intervals'] + calls['minima']
        helpers = {key: 0 for key in SELECTION_HELPER_RESERVATION}
        helper_work = {key: 0 for key in SELECTION_HELPER_CANONICAL_WORK_LIMITS}
    return {**(PROJECTION.full_package_reservation() if package else {}),
            **(ENCLOSURE.full_package_reservation() if package else {}),
            **(projected_batch_reservation() if package else {}),
            **(principal_reservation() if package else {}),
            **(combination_reservation() if package else {}),
            'selection_expected': 'package' if package else 'external-consumer',
            'selection_reserved_calls': calls,
            'selection_reserved_helper_calls': helpers,
            'selection_reserved_canonical_source_work_uppers': SELECTION.source_work_uppers(calls['selections'], calls['batches']),
            'selection_reserved_helper_canonical_source_work_uppers': helper_work,
            'source_raw_private_reserved_work': {key: value if package else 0 for key, value in SOURCE_RAW_PRIVATE_WORK_LIMITS.items()},
            'selection_reservation_scope': 'Full source recipe charged before the effect; no measured internal work or arbitrary-mutant bound.'}


def projected_batch_source_work(calls):
    """Reuse each upstream work owner once for the new disjoint source suite."""
    try:
        own = PROJECTED_BATCH.source_work_uppers(calls)
        producer = ENCLOSURE.source_work_uppers({name: calls[name] for name in ENCLOSURE.FIELDS})
        downstream = SELECTION.source_work_uppers(calls['selections'], own['delegated_batch_attempts_upper'])
    except (PROJECTED_BATCH.Incomplete, ENCLOSURE.Incomplete, SELECTION.Incomplete) as error:
        raise Incomplete(str(error)) from error
    minimum = {'minimum_garde_validate_calls_upper': calls['minima']}
    groups = (own, producer, downstream, minimum)
    if sum(len(group) for group in groups) != len(set().union(*(set(group) for group in groups))):
        raise Incomplete('Projected-batch source projections overlap work ownership.')
    result = {key: value for group in groups for key, value in group.items()}
    if any(type(value) is not int or not 0 <= value <= (1 << 64) - 1 for value in result.values()):
        raise Incomplete('Projected-batch source projection exceeds exact bounded arithmetic.')
    return result


def projected_batch_reservation():
    """Reserve the new suite without borrowing any older observed marker family."""
    try:
        reservation = PROJECTED_BATCH.full_package_reservation()
    except PROJECTED_BATCH.Incomplete as error:
        raise Incomplete(str(error)) from error
    calls = reservation['projected_batch_reserved_calls']
    reservation['projected_batch_reserved_source_work_uppers'] = projected_batch_source_work(calls)
    return reservation


def projected_batch_budget_vector():
    """Flatten each disjoint source charge once; observations add no fields."""
    reservation = projected_batch_reservation()
    return {prefix + key: amount
            for prefix, name in (('projected_batch.call.', 'projected_batch_reserved_calls'),
                                 ('projected_batch.work.', 'projected_batch_reserved_source_work_uppers'),
                                 ('projected_batch.private.', 'projected_batch_reserved_private_source_work'))
            for key, amount in reservation[name].items()}


def admit_projected_batch_allowances(limits):
    """Require every finite cumulative allowance before any admitted effect.

    No candidate may claim final admission with pending limits or a map that
    cannot cover even its nominal complete package. The coordinator separately
    reviews the full union; this guard invents no campaign quota or allowance.
    """
    required = projected_batch_budget_vector()
    if (len(required) != 68 or type(limits) is not dict or set(limits) != set(required)
            or any(type(value) is not int or not 0 <= value <= (1 << 64) - 1
                   for value in limits.values())
            or any(amount > limits[key] for key, amount in required.items())):
        raise Incomplete('Projected-batch cumulative allowances omit finite complete-package source work.')
    return dict(limits)


def principal_source_work(calls, *, private_decision_case=False):
    """Charge genuine producers once and bounded principal work by its own units.

    The named private case adds fixed source-owned selection/completion uppers,
    never synthetic public advances or observations of private execution counts.
    Native mandatory-garde result-use evidence remains a separate claim.
    Equality counts include parameters and state: sixteen slots plus sixteen
    fixed-field checks conservatively bound either representation per call.
    """
    if (type(private_decision_case) is not bool
            or type(calls) is not dict or set(calls) != set(PRINCIPAL_FIELDS)
            or any(type(value) is not int or not 0 <= value <= (1 << 64) - 1
                   for value in calls.values())):
        raise Incomplete('Principal source work requires every exact public-call field.')
    direct_selections = PRINCIPAL_PRIVATE_WORK['explicit_selection_calls_upper'] if private_decision_case else 0
    completions = PRINCIPAL_PRIVATE_WORK['direct_complete_decision_calls_upper'] if private_decision_case else 0
    batch_calls = {name: calls['advances'] + direct_selections if name == 'selections' else calls[name]
                   for name in PROJECTED_BATCH.FIELDS}
    work = projected_batch_source_work(batch_calls)
    # This suite projection includes PB constructor getters and the direct
    # principal receiver's bounded getter chains; PB's own model is unchanged.
    work['internal_product_getter_requests_upper'] += (
        completions * PRINCIPAL_PRIVATE_WORK['internal_product_getters_per_completion_upper'])
    operations = calls['seeds'] + calls['advances']
    own = {
        'principal_checked_scalar_primitives_upper': 82 * calls['advances'],
        'principal_key_comparisons_upper': (4096 * operations + 16 * calls['getters']
                                           + completions * PRINCIPAL_PRIVATE_WORK['key_comparisons_per_completion_upper']),
        'principal_state_clone_cells_upper': 16 * (calls['advances'] + calls['state_clones']),
        'principal_value_equality_slot_visits_upper': 16 * calls['value_equalities'],
        'principal_value_equality_fixed_fields_upper': 16 * calls['value_equalities'],
        'principal_state_format_calls_upper': calls['state_formats'],
        'principal_garde_validation_calls_upper': 64 * operations + calls['parameters'] + calls['strata'],
    }
    if set(work) & set(own):
        raise Incomplete('Principal and reused producer units overlap.')
    work.update(own)
    if any(value > (1 << 64) - 1 for value in work.values()):
        raise Incomplete('Principal work exceeds exact u64 arithmetic.')
    return work


def principal_reservation():
    """Reserve all fourteen cases, including fixed direct private-consumer work."""
    calls = {field: sum(row[index] for row in PRINCIPAL_CASE_CEILINGS.values())
             for index, field in enumerate(PRINCIPAL_FIELDS)}
    return {'principal_reserved_calls': calls,
            'principal_reserved_source_work_uppers': principal_source_work(calls, private_decision_case=True)}


def principal_budget_vector():
    reservation = principal_reservation()
    return {prefix + key: value
            for prefix, name in (('principal.call.', 'principal_reserved_calls'),
                                 ('principal.work.', 'principal_reserved_source_work_uppers'))
            for key, value in reservation[name].items()}


def admit_principal_allowances(limits):
    required = principal_budget_vector()
    if (type(limits) is not dict or set(limits) != set(required)
            or any(type(value) is not int or not 0 <= value <= (1 << 64) - 1 for value in limits.values())
            or any(value > limits[key] for key, value in required.items())):
        raise Incomplete('Principal cumulative allowances await the exact complete checkpoint reservation.')
    return dict(limits)


def combination_source_work(calls):
    """Charge the genuine producer chain once plus disjoint kernel-module units."""
    try:
        producer = principal_source_work(COMBINATION.producer_calls(calls), private_decision_case=False)
        own = COMBINATION.source_work_uppers(calls)
    except COMBINATION.Incomplete as error:
        raise Incomplete(str(error)) from error
    if set(producer) & set(own):
        raise Incomplete('Combination module and genuine producer source units overlap.')
    result = {**producer, **own}
    if any(type(value) is not int or not 0 <= value <= (1 << 64) - 1 for value in result.values()):
        raise Incomplete('Combination source work exceeds exact u64 arithmetic.')
    return result


def combination_reservation():
    """Reserve all twelve cases without borrowing another marker's observations."""
    try:
        calls = COMBINATION.full_package_reservation()['combination_reserved_calls']
    except COMBINATION.Incomplete as error:
        raise Incomplete(str(error)) from error
    return {'combination_reserved_calls': calls,
            'combination_reserved_source_work_uppers': combination_source_work(calls)}


def combination_budget_vector():
    """Flatten each producer and kernel source unit exactly once."""
    reservation = combination_reservation()
    return {prefix + key: value
            for prefix, name in (('combination.call.', 'combination_reserved_calls'),
                                 ('combination.work.', 'combination_reserved_source_work_uppers'))
            for key, value in reservation[name].items()}


def admit_combination_allowances(limits):
    """Require the complete finite cumulative allowance, never a partial schema."""
    required = combination_budget_vector()
    if (type(limits) is not dict or set(limits) != set(required)
            or any(type(value) is not int or not 0 <= value <= (1 << 64) - 1 for value in limits.values())
            or any(value > limits[key] for key, value in required.items())):
        raise Incomplete('Combination cumulative allowances await the exact complete checkpoint reservation.')
    return dict(limits)


def principal_counts(output, *, require_complete=False):
    """Read exact first17/new20 profiles; missing/failed prefixes stay pending."""
    if type(output) is not str or type(require_complete) is not bool:
        raise Incomplete('Principal observations require text and a Boolean mode.')
    recorded, partial = {}, []
    decimal = r'(0|[1-9][0-9]{0,19})'
    for raw in output.splitlines(keepends=True):
        if 'CORE_PRINCIPAL_' not in raw:
            continue
        matched = False
        for case, ceiling in PRINCIPAL_CASE_CEILINGS.items():
            first = case == PRINCIPAL_FIRST
            fields = PRINCIPAL_FIELDS17 if first else PRINCIPAL_FIELDS
            marker = 'CORE_PRINCIPAL_PROFILE' if first else 'CORE_PRINCIPAL_NEXT_PROFILE'
            test = (case if first else 'principal::private_decision_guards::' + case
                    if case == PRINCIPAL_PRIVATE_CASE else 'principal_cases::' + case)
            pattern = (r'(?:test ' + re.escape(test) + r' \.\.\. )?' + marker + ' case='
                       + re.escape(case) + ''.join(' ' + field + '=' + decimal for field in fields) + '\n')
            match = re.fullmatch(pattern, raw)
            if match is None:
                continue
            values = tuple(int(value) for value in match.groups())
            if first:
                # These three operations are absent from the source's first17
                # schema, not zeros synthesized for any missing test family.
                values += (0, 0, 0)
            if (case in recorded or any(value > cap or value > (1 << 64) - 1
                                       for value, cap in zip(values, ceiling, strict=True))):
                raise Incomplete('Principal row is duplicated or exceeds exact source/u64 bounds.')
            recorded[case] = values
            if values != PRINCIPAL_CASE_RECIPES[case]:
                partial.append(case)
            matched = True
            break
        if not matched:
            raise Incomplete('Principal marker has unknown identity, malformed fields or incomplete newline.')
    missing = sorted(set(PRINCIPAL_CASE_CEILINGS) - set(recorded))
    pending = bool(missing or partial)
    if require_complete and pending:
        raise Incomplete('Principal complete package profiles are missing or partial.')
    subtotal = ({field: sum(row[index] for row in recorded.values())
                 for index, field in enumerate(PRINCIPAL_FIELDS)} if recorded else None)
    return {'principal_counter_records': len(recorded), 'principal_missing_cases': missing,
            'principal_private_case_source_upper_present': PRINCIPAL_PRIVATE_CASE in recorded,
            'principal_partial_cases': sorted(partial), 'principal_observation_pending': pending,
            'principal_recorded_call_subtotals': subtotal,
            'principal_observed_calls': subtotal if not pending else None}


def profile_counts(output, *, require_fit=True, require_runs=False, validate_runs_receipt=True,
                   require_selection=False, require_projection=False, require_enclosure=False,
                   require_projected_batch=False, require_principal=False, require_combination=False):
    """Observe disjoint external owners; charge hidden history work only as uppers.

    Raw text has complete exact-u64 rows. History fields denote external public
    calls only; a history append contributes at most one nested normalizer/stat
    and64 generated points by source contract, never observed nested telemetry.
    Missing whole zero rows still require coordinator discovery/source audit.
    """
    if "--- STDERR:" in output or "Summary [" in output:
        raise Incomplete("Nextest summary/replay output is not an admitted core counter stream.")
    if any(marker in output for marker in ('CORE_HISTORY_FUZZ_COUNTS', 'CORE_NORMALIZATION_FUZZ_COUNTS',
                                           'CORE_STATS_FUZZ_COUNTS', 'CORE_FUZZ_COUNTS')):
        raise Incomplete('Fuzz-owner counters are not a package profile stream.')
    direct = counter_rows(output, 'CORE_PROFILE_COUNTS', ('fits', 'points'))
    stats = counter_rows(output, 'CORE_STATS_PROFILE_COUNTS', ('stats', 'points'))
    fields = tuple(NORMALIZATION_TELEMETRY['observed_fields'])
    normalization = counter_rows(output, 'CORE_NORMALIZATION_PROFILE_COUNTS', fields)
    history_fields = tuple(HISTORY_TELEMETRY['observed_fields'])
    history = counter_rows(output, 'CORE_HISTORY_PROFILE_COUNTS', history_fields)
    runs = runs_counts(output, require_complete=require_runs, validate_receipt=validate_runs_receipt)
    selection = selection_package_counts(output, require_complete=require_selection)
    selection_external = selection_external_counts(output)
    try:
        projection = PROJECTION.projection_counts(output, require_complete=require_projection)
    except PROJECTION.Incomplete as error:
        raise Incomplete(str(error)) from error
    projection_calls = projection['projection_recorded_call_subtotals']
    projection_work = PROJECTION.source_work_uppers(projection_calls) if projection_calls is not None else None
    try:
        enclosure = ENCLOSURE.enclosure_counts(output, require_complete=require_enclosure)
        enclosure_calls = enclosure['enclosure_recorded_call_subtotals']
        enclosure_work = ENCLOSURE.source_work_uppers(enclosure_calls) if enclosure_calls is not None else None
    except ENCLOSURE.Incomplete as error:
        raise Incomplete(str(error)) from error
    try:
        projected_batch = PROJECTED_BATCH.projected_batch_counts(output, require_complete=require_projected_batch)
        projected_batch_guards = PROJECTED_BATCH.guard_counts(output, require_complete=require_projected_batch)
        projected_batch_calls = projected_batch['projected_batch_recorded_call_subtotals']
        projected_batch_work = (projected_batch_source_work(projected_batch_calls)
                                if projected_batch_calls is not None else None)
    except PROJECTED_BATCH.Incomplete as error:
        raise Incomplete(str(error)) from error
    principal = principal_counts(output, require_complete=require_principal)
    principal_calls = principal['principal_recorded_call_subtotals']
    principal_work = (principal_source_work(principal_calls, private_decision_case=
                       principal['principal_private_case_source_upper_present'])
                      if principal_calls is not None else None)
    try:
        combination = COMBINATION.combination_counts(output, require_complete=require_combination)
    except COMBINATION.Incomplete as error:
        raise Incomplete(str(error)) from error
    combination_calls = combination['combination_recorded_call_subtotals']
    combination_work = combination_source_work(combination_calls) if combination_calls is not None else None
    selection_records = (selection['selection_external_records'] + selection['selection_guard_records']
                         + selection['selection_helper_records'] + selection_external['selection_external_consumer_records'])
    if selection_external['selection_external_consumer_records'] and (
            require_selection or direct or stats or normalization or history
            or runs['runs_external_records'] or runs['runs_sign_records'] or runs['runs_guard_records']
            or selection['selection_external_records'] or selection['selection_guard_records']
            or selection['selection_helper_records'] or projection['projection_counter_records']
            or enclosure['enclosure_counter_records'] or projected_batch['projected_batch_counter_records']
            or projected_batch_guards['projected_batch_guard_counter_records'] or principal['principal_counter_records']
            or combination['combination_counter_records']):
        raise Incomplete('Standalone selection consumer counters are mixed with a package/other-owner stream.')
    if not direct and not stats and not normalization and not history and not (
            runs['runs_external_records'] or runs['runs_sign_records'] or runs['runs_guard_records'] or selection_records
            or projection['projection_counter_records'] or enclosure['enclosure_counter_records']
            or projected_batch['projected_batch_counter_records'] or principal['principal_counter_records']
            or combination['combination_counter_records']):
        raise Incomplete("Core profile lacks complete public counter records.")
    actual = dict(zip(fields, (sum(row[index] for row in normalization) for index in range(len(fields))), strict=True))
    h = dict(zip(history_fields, (sum(row[index] for row in history) for index in range(len(history_fields))), strict=True))
    limits = {'normalizations': 128, 'observations': 8320, 'contexts': 256,
              'monotonic': 8320, 'coordinates': 33280, 'exchanges': 8320,
              'getters': 8448, 'stats': 16, 'points': 65536}
    if any(actual[key] > maximum for key, maximum in limits.items()):
        raise Incomplete("Normalization public attempts exceed the fixed profile cap.")
    if actual['getters'] > 66 * actual['normalizations']:
        raise Incomplete("Point view attempts exceed the admitted prepared-result upper recipe.")
    if any(h[key] > maximum for key, maximum in HISTORY_TELEMETRY['profile_limits'].items()):
        raise Incomplete("History public attempts exceed the fixed profile cap.")
    if h['monotonic'] != h['observations'] or h['exchanges'] != h['observations'] or h['coordinates'] != 4*h['observations']:
        raise Incomplete("History setup ownership differs from its four-coordinate recipe.")
    fits = sum(row[0] for row in direct) + h['fits']
    calls = sum(row[0] for row in stats) + actual['stats'] + h['stats']
    legacy_points = sum(row[1] for row in direct + stats) + actual['points'] + h['points']
    points = legacy_points + runs['runs_points']
    history_work_upper = h['appends']
    history_generated = 64*h['appends']
    generated = 64*actual['normalizations'] + history_generated
    observed_work = fits + calls
    legacy_work = observed_work + history_work_upper
    legacy_point_upper = legacy_points + generated
    if (legacy_work > LEGACY_PROFILE_LIMITS['charged_fit_work_upper']
            or legacy_point_upper > LEGACY_PROFILE_LIMITS['point_admissions_upper']):
        raise Incomplete('Legacy core profile exceeded its preserved owner budget.')
    original_charged = legacy_work + runs['runs_fit_upper']
    original_points = points + generated + runs['runs_point_copy_upper']
    if (original_charged > PROFILE_LIMITS['charged_fit_work_upper']
            or original_points > PROFILE_LIMITS['point_admissions_upper']):
        raise Incomplete('Existing core owners exceeded their preserved aggregate profile budget.')
    projection_fits = projection_work['charged_fit_work_upper'] if projection_work is not None else 0
    projection_points = projection_work['point_admissions_upper'] if projection_work is not None else 0
    enclosure_fits = enclosure_work['charged_fit_work_upper'] if enclosure_work is not None else 0
    enclosure_points = enclosure_work['point_admissions_upper'] if enclosure_work is not None else 0
    projected_batch_fits = projected_batch_work['charged_fit_work_upper'] if projected_batch_work is not None else 0
    projected_batch_points = projected_batch_work['point_admissions_upper'] if projected_batch_work is not None else 0
    principal_fits = principal_work['charged_fit_work_upper'] if principal_work is not None else 0
    principal_points = principal_work['point_admissions_upper'] if principal_work is not None else 0
    combination_fits = combination_work['charged_fit_work_upper'] if combination_work is not None else 0
    combination_points = combination_work['point_admissions_upper'] if combination_work is not None else 0
    charged = original_charged + projection_fits + enclosure_fits + projected_batch_fits + principal_fits + combination_fits
    point_upper = original_points + projection_points + enclosure_points + projected_batch_points + principal_points + combination_points
    projection_reservation = PROJECTION.full_package_reservation()['projection_reserved_source_work_uppers']
    enclosure_reservation = ENCLOSURE.full_package_reservation()['enclosure_reserved_source_work_uppers']
    projected_batch_upper = projected_batch_reservation()['projected_batch_reserved_source_work_uppers']
    principal_upper = principal_reservation()['principal_reserved_source_work_uppers']
    combination_upper = combination_reservation()['combination_reserved_source_work_uppers']
    if (charged > PROFILE_LIMITS["charged_fit_work_upper"] + projection_reservation['charged_fit_work_upper'] + enclosure_reservation['charged_fit_work_upper'] + projected_batch_upper['charged_fit_work_upper'] + principal_upper['charged_fit_work_upper'] + combination_upper['charged_fit_work_upper']
            or point_upper > PROFILE_LIMITS["point_admissions_upper"] + projection_reservation['point_admissions_upper'] + enclosure_reservation['point_admissions_upper'] + projected_batch_upper['point_admissions_upper'] + principal_upper['point_admissions_upper'] + combination_upper['point_admissions_upper']
            or (require_fit and observed_work == 0 and actual['normalizations'] == 0
                and h['histories'] == 0 and h['appends'] == 0
                and runs['runs_selections'] == 0 and runs['runs_guard_candidates'] == 0
                and not (selection.get('selection_recorded_call_subtotals') or {}).get('selections')
                and not (selection_external['selection_external_consumer_calls'] or {}).get('selections')
                and not (projection_calls or {}).get('terms') and not (projection_calls or {}).get('projections')
                and not (projection_calls or {}).get('histories')
                and not (enclosure_calls or {}).get('enclosures')
                and not (enclosure_calls or {}).get('histories')
                and not any((projected_batch_calls or {}).values()) and not principal_calls
                and not any((combination_calls or {}).values()))):
        raise Incomplete("Core profile exceeded its fixed charged-work/point budget or reached no work.")
    return {**runs, **selection, **selection_external, **projection, **enclosure, **principal, **combination,
            'combination_source_owned_work_uppers': combination_work,
            'principal_source_owned_work_uppers': principal_work,
            **projected_batch, **projected_batch_guards,
            'projected_batch_source_owned_work_uppers': projected_batch_work,
            'projection_source_owned_work_uppers': projection_work,
             'enclosure_source_owned_work_uppers': enclosure_work,
            "fit_calls": fits, "direct_fit_calls": fits, "stats_calls": calls,
            "legacy_point_admissions": legacy_points,
            "legacy_charged_fit_work_upper": legacy_work,
            "legacy_point_admissions_upper": legacy_point_upper,
            "point_admissions": points, "internal_fit_upper_bound": calls + history_work_upper + runs['runs_fit_upper'] + projection_fits + enclosure_fits + projected_batch_fits + principal_fits + combination_fits,
            "observed_charged_fit_work": observed_work, "charged_fit_work_upper": charged,
            "point_admissions_upper": point_upper,
            "normalization_requests": actual['normalizations'],
            "observation_constructor_attempts": actual['observations'],
            "context_constructions": actual['contexts'],
            "monotonic_constructor_attempts": actual['monotonic'],
            "utc_coordinate_constructor_attempts": actual['coordinates'],
            "exchange_arithmetic_calls": actual['exchanges'],
            "point_getter_requests": actual['getters'],
            "normalization_stats_calls": actual['stats'],
            "normalization_direct_point_admissions": actual['points'],
            "duration_operations_upper": generated,
            "generated_point_admissions_upper": generated,
            "fit_input_constructions_upper": actual['normalizations'] + h['appends'],
            "history_constructions": h['histories'], "history_append_requests": h['appends'],
            "history_observation_constructor_attempts": h['observations'],
            "history_context_constructions": h['contexts'],
            "history_monotonic_constructor_attempts": h['monotonic'],
            "history_utc_coordinate_constructor_attempts": h['coordinates'],
            "history_exchange_arithmetic_calls": h['exchanges'],
            "history_product_getter_requests": h['getters'],
            "history_direct_normalization_requests": h['normalizations'],
            "history_direct_stats_calls": h['stats'], "history_direct_fit_calls": h['fits'],
            "history_direct_point_admissions": h['points'],
            "history_normalization_entries_upper": h['appends'],
            "history_statistics_entries_upper": history_work_upper,
            "history_statistics_inner_fit_upper": history_work_upper,
            "history_duration_operations_upper": history_generated,
            "history_generated_point_admissions_upper": history_generated,
            "history_fit_input_constructions_upper": h['appends'],
            "history_staging_Copy_assignments_upper": 127*h['appends'],
            "counter_records": len(direct) + len(stats) + len(normalization) + len(history)
                               + runs['runs_external_records'] + runs['runs_sign_records'] + runs['runs_guard_records']
                               + selection_records + projection['projection_counter_records'] + enclosure['enclosure_counter_records']
                               + projected_batch['projected_batch_counter_records']
                               + projected_batch_guards['projected_batch_guard_counter_records'] + principal['principal_counter_records']
                               + combination['combination_counter_records'],
            "direct_counter_records": len(direct), "stats_counter_records": len(stats),
            "normalization_counter_records": len(normalization), "history_counter_records": len(history)}


def account_literal_doctests(counts):
    """Charge five source-audited literal doctests as separate source uppers.

    One legal snippet constructs3 direct points and calls no fit/statistics,
    normalizer or history. The new legal terms snippet attempts one checked
    constructor and two getters, with three garde callbacks and six predicate calls; two
    projection privacy snippets must fail compilation. Earlier failure may
    prevent execution; no actual counter is fabricated. The separate prospective
    campaign budget's larger doc reservation is not an executed/source call.
    """
    value = dict(counts)
    value["literal_doctest_stats_calls_upper"] = LITERAL_DOCTEST_LIMITS["stats_calls_upper"]
    value["literal_doctest_direct_fit_calls_upper"] = 0
    value["literal_doctest_points_upper"] = LITERAL_DOCTEST_LIMITS["point_admissions_upper"]
    value["charged_fit_work_upper"] += LITERAL_DOCTEST_LIMITS["stats_calls_upper"]
    value["point_admissions_upper"] += LITERAL_DOCTEST_LIMITS["point_admissions_upper"]
    value['legacy_charged_fit_work_upper'] += LITERAL_DOCTEST_LIMITS['stats_calls_upper']
    value['legacy_point_admissions_upper'] += LITERAL_DOCTEST_LIMITS['point_admissions_upper']
    value["internal_fit_upper_bound"] += LITERAL_DOCTEST_LIMITS["stats_calls_upper"]
    for key, maximum in LITERAL_DOCTEST_LIMITS.items():
        if key not in {'total_source_doctests_maximum', 'legal_source_doctests_maximum',
                       'direct_fit_calls_upper', 'stats_calls_upper', 'point_admissions_upper'}:
            value['literal_doctest_' + key] = maximum
    projection_upper = PROJECTION.full_package_reservation()['projection_reserved_source_work_uppers']
    projection_work = value.get('projection_source_owned_work_uppers') or {}
    enclosure_upper = ENCLOSURE.full_package_reservation()['enclosure_reserved_source_work_uppers']
    enclosure_work = value.get('enclosure_source_owned_work_uppers') or {}
    projected_batch_upper = projected_batch_reservation()['projected_batch_reserved_source_work_uppers']
    projected_batch_work = value.get('projected_batch_source_owned_work_uppers') or {}
    principal_upper = principal_reservation()['principal_reserved_source_work_uppers']
    principal_work = value.get('principal_source_owned_work_uppers') or {}
    combination_upper = combination_reservation()['combination_reserved_source_work_uppers']
    combination_work = value.get('combination_source_owned_work_uppers') or {}
    if any(value[key] - projection_work.get(key, 0) - enclosure_work.get(key, 0) - projected_batch_work.get(key, 0) - principal_work.get(key, 0) - combination_work.get(key, 0) > maximum
           for key, maximum in PROFILE_LIMITS.items()):
        raise Incomplete('Literal doctest source upper exceeds the preserved existing-owner aggregate profile.')
    if any(value[key] > maximum + projection_upper[key] + enclosure_upper[key] + projected_batch_upper[key] + principal_upper[key] + combination_upper[key] for key, maximum in PROFILE_LIMITS.items()):
        raise Incomplete("Literal doctest upper allowance exceeds the complete package profile.")
    if any(value['legacy_' + key] > maximum for key, maximum in LEGACY_PROFILE_LIMITS.items()):
        raise Incomplete('Literal doctest upper allowance exceeds the preserved legacy budget.')
    return value


def fuzz_counts(output, *, complete):
    """Observe cumulative raw/public attempts; derive only bounded internal costs."""
    if any(marker in output for marker in ('CORE_HISTORY_FUZZ_COUNTS', 'CORE_STATS_FUZZ_COUNTS', 'CORE_FUZZ_COUNTS',
                                          'CORE_NORMALIZATION_PROFILE_COUNTS', 'CORE_RUNS_PROFILE',
                                          'CORE_RUNS_GUARDS', 'CORE_RUNS_SIGN_PROFILE', 'CORE_RUNS_SIGN_RECEIPT',
                                           'CORE_ENCLOSURE_PROFILE', PROJECTED_BATCH.MARKER, PROJECTED_BATCH.GUARD_MARKER,
                                          'CORE_COMBINATION_',
                                          '--- STDERR:', 'Summary [')):
        raise Incomplete('Wrong-owner or replay counter stream in normalization fuzz output.')
    fields = ('raw', *NORMALIZATION_TELEMETRY['observed_fields'])
    rows = counter_rows(output, 'CORE_NORMALIZATION_FUZZ_COUNTS', fields)
    previous = (0,) * len(fields)
    deltas = (1, 1, 64, 2, 64, 256, 64, 66, 1, 0)
    for row in rows:
        if (row[0] != previous[0]+1 or row[0] > 4096
                or any(not old <= current <= old+maximum for old, current, maximum in zip(previous, row, deltas, strict=True))
                or row[8] > row[1] or row[7] > 66*row[1]):
            raise Incomplete('Normalization fuzz callback sequence or actual attempt bound is invalid.')
        previous = row
    if not rows or previous[1] == 0 or (complete and previous[8] == 0):
        raise Incomplete('Normalization fuzz never observed an actual preparation request.')
    if complete and (previous[0] != 4096 or not re.search(r'Done 4096 runs', output)):
        raise Incomplete('Normalization fuzz did not complete4096 raw callbacks.')
    raw, normalizations, observations, contexts, monotonic, coordinates, exchanges, getters, calls, points = previous
    return {'raw_driver_runs': raw, 'normalization_requests': normalizations,
            'observation_constructor_attempts': observations, 'context_constructions': contexts,
            'monotonic_constructor_attempts': monotonic, 'utc_coordinate_constructor_attempts': coordinates,
            'exchange_arithmetic_calls': exchanges, 'point_getter_requests': getters,
            'point_admissions': points, 'point_admissions_upper': points+64*normalizations,
            'duration_operations_upper': 64*normalizations,
            'generated_point_admissions_upper': 64*normalizations,
            'fit_input_constructions_upper': normalizations,
            'fit_calls': 0, 'direct_fit_calls': 0, 'stats_calls': calls,
            'internal_fit_upper_bound': calls, 'observed_charged_fit_work': calls,
            'charged_fit_work_upper': calls}


def history_fuzz_counts(output, *, complete):
    """Observe actual cumulative history public calls; derive only nested uppers.

    Each accepted stream is one actual process with sequential callbacks1..4096.
    One fresh empty history/callback and65 total appends bound nested stats63,
    generated points2141 and durations2144. Those are charged source bounds,
    never observed calls. Known-report success is an explicit post-oracle marker.
    """
    forbidden = ('CORE_NORMALIZATION_FUZZ_COUNTS', 'CORE_STATS_FUZZ_COUNTS', 'CORE_FUZZ_COUNTS',
                 'CORE_PROFILE_COUNTS', 'CORE_STATS_PROFILE_COUNTS',
                 'CORE_NORMALIZATION_PROFILE_COUNTS', 'CORE_HISTORY_PROFILE_COUNTS',
                 'CORE_RUNS_PROFILE', 'CORE_RUNS_GUARDS', 'CORE_RUNS_SIGN_PROFILE', 'CORE_RUNS_SIGN_RECEIPT',
                 'CORE_ENCLOSURE_PROFILE', PROJECTED_BATCH.MARKER, PROJECTED_BATCH.GUARD_MARKER,
                 'CORE_COMBINATION_',
                 '--- STDERR:', 'Summary [')
    if any(marker in output for marker in forbidden):
        raise Incomplete('Wrong-owner or replay stream in history fuzz output.')
    fields = ('raw', *HISTORY_TELEMETRY['observed_fields'])
    rows = counter_rows(output, 'CORE_HISTORY_FUZZ_COUNTS', fields)
    previous = (0,)*len(fields)
    maxima = (1,1,65,65,2,65,260,65,65536,0,0,0,0)
    statistics_upper = generated_upper = duration_upper = 0
    for row in rows:
        if (row[0] != previous[0]+1 or row[0]>4096
                or any(not old <= current <= old+maximum
                       for old,current,maximum in zip(previous,row,maxima,strict=True))):
            raise Incomplete('History raw sequence or actual callback attempt bound differs.')
        delta = tuple(current-old for old,current in zip(previous,row,strict=True))
        _,histories,appends,observations,contexts,monotonic,coordinates,exchanges,getters,norm,stats,fits,points = delta
        if (appends>observations or histories>contexts or appends and histories!=1
                or not histories and any(delta[2:]) or getters<9*histories+2*observations
                or monotonic!=observations or exchanges!=observations or coordinates!=4*observations
                or norm or stats or fits or points):
            raise Incomplete('History external/setup/direct-call ownership differs.')
        # Canonical integer work envelope only: no time/phase/fit output model.
        statistics_upper += max(0,appends-2)
        duration_upper += appends*(appends+1)//2 if appends<=64 else 2144
        generated_upper += max(0,appends*(appends+1)//2-3) if appends<=64 else 2141
        previous = row
    if not rows or previous[1]==0 or previous[2]==0:
        raise Incomplete('History fuzz reached no actual new-history/append boundary.')
    if complete and (previous[0]!=4096 or not re.search(r'Done 4096 runs',output)
                     or 'HISTORY_FUZZ_KNOWN_REPORT_CHECKED' not in output):
        raise Incomplete('History fuzz did not finish4096 callbacks with a checked actual known report.')
    raw,histories,appends,observations,contexts,monotonic,coordinates,exchanges,getters,norm,stats,fits,points = previous
    return {'raw_driver_runs':raw,'history_constructions':histories,'history_append_requests':appends,
            'history_observation_constructor_attempts':observations,'history_context_constructions':contexts,
            'history_monotonic_constructor_attempts':monotonic,'history_utc_coordinate_constructor_attempts':coordinates,
            'history_exchange_arithmetic_calls':exchanges,'history_product_getter_requests':getters,
            'history_direct_normalization_requests':norm,'history_direct_stats_calls':stats,
            'history_direct_fit_calls':fits,'history_direct_point_admissions':points,
            'history_normalization_entries_upper':appends,
            'history_statistics_entries_upper':statistics_upper,'history_statistics_inner_fit_upper':statistics_upper,
            'history_duration_operations_upper':duration_upper,'history_generated_point_admissions_upper':generated_upper,
            'history_fit_input_constructions_upper':statistics_upper,
            'history_staging_Copy_assignments_upper':127*appends,
            'fit_calls':fits,'direct_fit_calls':fits,'stats_calls':stats,'point_admissions':points,
            'normalization_requests':norm,'observed_charged_fit_work':fits+stats,
            'charged_fit_work_upper':fits+stats+statistics_upper,'internal_fit_upper_bound':statistics_upper,
            'point_admissions_upper':points+generated_upper,
            'duration_operations_upper':duration_upper,'generated_point_admissions_upper':generated_upper,
            'fit_input_constructions_upper':statistics_upper}


def history_fuzz_outcome(result, *, defect):
    """Accept the exact native process cause after Session's post-custody check.

    A defect requires native77, no runtime failure and exactly one actual unnamed
    target.rs panic with the complete intended assertion message. A marker in
    unrelated output, another panic, another status or setup/timeout is rejected.
    Healthy requires native0 and no panic. Counter completion is checked separately.
    """
    if (type(defect) is not bool or not isinstance(result, dict)
            or not {'lane', 'exit_code', 'failure', 'output'} <= result.keys()
            or result['lane'] != ('fuzz-defect' if defect else 'fuzz-healthy')
            or result['failure'] is not None or type(result['exit_code']) is not int
            or result['exit_code'] != (77 if defect else 0)
            or not isinstance(result['output'], str)):
        raise Incomplete('History fuzz native result is not the exact admitted healthy/defect outcome.')
    output = result['output']
    headers = re.findall(r"(?m)^thread [^\r\n]* panicked at [^\r\n]*", output)
    if defect:
        messages = re.findall(
            r"(?m)^thread '<unnamed>'(?: \([0-9]+\))? panicked at (?:[^\r\n]*[/\\])?target\.rs:[0-9]+:[0-9]+:\r?\n([^\r\n]*)",
            output)
        if (len(headers) != 1
                or messages != ['independent history oracle: frozen published report bits']):
            raise Incomplete('History fuzz did not reach its exact intended published-report panic.')
    elif headers or 'independent history oracle:' in output:
        raise Incomplete('Healthy history fuzz contains an oracle failure or panic.')
    return output


def encode_history_case(mode, rows):
    """Encode exact raw literals only; no phase sentinels or output calculation.

    Accept header enum0..4 and<=65 three-scalar tuples with exact int(not bool)
    u64 time/divisor bits and i64 phase. Reject every other representation before
    producing a seed. The header's final-row context mapping is explicit below.
    """
    if type(mode) is not int or not 0<=mode<=4 or not isinstance(rows,(tuple,list)) or len(rows)>65:
        raise Incomplete('Unreviewed history header or row count.')
    result=bytearray([mode])
    for row in rows:
        if (not isinstance(row,(tuple,list)) or len(row)!=3
                or any(type(value) is not int for value in row)):
            raise Incomplete('History wire requires three exact integer scalars.')
        time_value,phase,divisor=row
        if not 0<=time_value<1<<64 or not -(1<<63)<=phase<1<<63 or not 0<=divisor<1<<64:
            raise Incomplete('History wire scalar exceeds its exact representation.')
        result.extend(time_value.to_bytes(8,'big'))
        result.extend(phase.to_bytes(8,'big',signed=True))
        result.extend(divisor.to_bytes(8,'big'))
    return bytes(result)


def history_fuzz_seeds():
    """Frozen fact projections with exact header/context/phase/raw-bit bindings.

    Ordinary cases preserve their raw sequence. Full70 uses64 commits+one named
    boundary (six separate65-append cases), not its first65 raw events. Partial
    context cases use prior successful commits+selected boundary, retaining exact
    frozen expected states/source ordinals. No paired-two-history/allfacts claim.
    """
    base=('17','23','29')
    contexts={0:base,1:(str((1<<64)-1),str((1<<64)-2),str((1<<64)-3)),
              2:('19','23','29'),3:('17','31','29'),4:('17','23','31')}
    seeds={}
    if len(HISTORY_SEED_CASES)!=29:
        raise Incomplete('Frozen history projection count changed.')
    for case in HISTORY_SEED_CASES:
        mode=case['mode']; source=case['rows']
        if len(source)!=len(case['steps']) or len(source)>65:
            raise Incomplete('Frozen projection shape changed.')
        rows=[]
        for index,row in enumerate(source):
            if len(row)!=6 or any(not isinstance(value,str) for value in row):
                raise Incomplete('Frozen history raw tuple changed.')
            expected=contexts[mode] if index+1==len(source) else contexts[1 if mode==1 else 0]
            if tuple(row[:3])!=expected:
                raise Incomplete('History header would silently remap frozen context tags.')
            rows.append((int(row[3]),int(row[4]),int(row[5],16)))
        if any(any(type(index) is not int or not 0<=index<len(rows) for index in step['keep'])
               or len(step['keep'])>64 for step in case['steps']):
            raise Incomplete('Frozen retained-row projection is not bound to actual input literals.')
        seeds['history-'+case['name']]=encode_history_case(mode,rows)
    # Explicit independent constructor/shape controls; no arbitrary numeric oracle.
    seeds['history-phase-outside'] = encode_history_case(0,[(0,(1<<33)+1,0x3ff0000000000000)])
    seeds['history-phase-i64-min-preserved'] = encode_history_case(0,[(0,-(1<<63),0x3ff0000000000000)])
    seeds['history-divisor-nan'] = encode_history_case(0,[(0,0,0x7ff8000000000000)])
    seeds['history-empty-shape'] = b''
    seeds['history-partial-shape'] = b'\0\0'
    seeds['history-unknown-header'] = b'\xff'
    return seeds


def record_counts(report, label, counts):
    """Retain observed operations and conservative charged bounds without conflation."""
    if any(type(counts.get(key)) is not int or counts[key] < 0 for key in GLOBAL_LIMITS):
        raise Incomplete("Core count record omits an exact nonnegative charged bound.")
    selection_kind = counts.get('selection_expected')
    projection_keys = ('projection_reserved_calls', 'projection_reserved_source_work_uppers',
                       'projection_reserved_private_source_work')
    enclosure_keys = ('enclosure_reserved_calls', 'enclosure_reserved_source_work_uppers',
                      'enclosure_reserved_private_source_work')
    projected_batch_keys = ('projected_batch_reserved_calls', 'projected_batch_reserved_module_work_uppers',
                            'projected_batch_reserved_source_work_uppers',
                            'projected_batch_reserved_private_source_work')
    principal_keys = ('principal_reserved_calls', 'principal_reserved_source_work_uppers')
    combination_keys = ('combination_reserved_calls', 'combination_reserved_source_work_uppers')
    if selection_kind == 'package':
        projection = PROJECTION.full_package_reservation()
        if any(not exact(counts.get(key), projection[key]) for key in projection_keys):
            raise Incomplete('Projection full-package reservation is missing or differs from source recipes.')
        enclosure = ENCLOSURE.full_package_reservation()
        if any(not exact(counts.get(key), enclosure[key]) for key in enclosure_keys):
            raise Incomplete('Enclosure full-package reservation is missing or differs from source recipes.')
        projected_batch = projected_batch_reservation()
        if any(not exact(counts.get(key), projected_batch[key]) for key in projected_batch_keys):
            raise Incomplete('Projected-batch full-package reservation is missing or differs from source recipes.')
        principal = principal_reservation()
        if any(not exact(counts.get(key), principal[key]) for key in principal_keys):
            raise Incomplete('Principal full-package reservation is missing or differs from its complete source census.')
        combination = combination_reservation()
        if any(not exact(counts.get(key), combination[key]) for key in combination_keys):
            raise Incomplete('Combination full-package reservation is missing or differs from its complete source census.')
        docs = counts.get('literal_doctests_reserved', False)
        if type(docs) is not bool:
            raise Incomplete('Package doctest reservation flag must be Boolean.')
        if any(counts[key] != amount for key, amount in package_charged_upper(include_doctests=docs).items()):
            raise Incomplete('Complete package reservation omits old or projection charged source work.')
    elif any(key in counts for key in (*projection_keys, *enclosure_keys, *projected_batch_keys,
                                     *principal_keys, *combination_keys)):
        raise Incomplete('Projection/enclosure/projected-batch reservation has no complete package scope.')
    if 'principal_external_reserved_calls' in counts or 'principal_external_reserved_source_work_uppers' in counts:
        legal = principal_legal_calls()
        if (not exact(counts.get('principal_external_reserved_calls'), legal)
                or not exact(counts.get('principal_external_reserved_source_work_uppers'), principal_source_work(legal))
                or 'combination_external_reserved_calls' not in counts):
            raise Incomplete('Principal legal-consumer reservation differs from its fixed real source recipe.')
    if ('combination_external_reserved_calls' in counts
            or 'combination_external_reserved_source_work_uppers' in counts):
        legal = combination_legal_calls()
        if (not exact(counts.get('combination_external_reserved_calls'), legal)
                or not exact(counts.get('combination_external_reserved_source_work_uppers'), combination_source_work(legal))
                or 'principal_external_reserved_calls' not in counts):
            raise Incomplete('Combination legal-consumer delta differs from its fixed already-produced singleton recipe.')
    if selection_kind is not None:
        expected = selection_reservation(package=True) if selection_kind == 'package' else selection_reservation(
            external={'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2}) if selection_kind == 'external-consumer' else None
        if expected is None or any(not exact(counts.get(key), expected[key]) for key in (
                'selection_reserved_calls', 'selection_reserved_helper_calls',
                'selection_reserved_canonical_source_work_uppers',
                'selection_reserved_helper_canonical_source_work_uppers', 'source_raw_private_reserved_work')):
            raise Incomplete('Selection pre-effect reservation is missing or differs from its source-owned scope.')
    elif any(key in counts for key in ('selection_reserved_calls', 'selection_reserved_helper_calls',
                                       'selection_reserved_canonical_source_work_uppers',
                                       'selection_reserved_helper_canonical_source_work_uppers', 'source_raw_private_reserved_work')):
        raise Incomplete('Selection reservation has no declared source scope.')
    report.setdefault("core_invocations", []).append({"invocation": label, **counts})
    refresh_accounting(report)


def refresh_accounting(report):
    """Sum charged bounds and parsed observations separately, exactly once.

    Mutation charges precede their observation parsing. Pending rows contribute
    to charged totals while remaining explicitly unavailable as observations.
    """
    invocations = report.get('core_invocations', [])
    observations, pending = [], []
    for row in invocations:
        if 'observed_counts' in row:
            if not isinstance(row['observed_counts'], dict) or row.get('observation_pending'):
                raise Incomplete('Parsed observation record has contradictory pending state.')
            observations.append(row['observed_counts'])
        elif row.get('observation_pending'):
            pending.append(row['invocation'])
        elif row.get('observation_status') == 'unviable-compile-only':
            continue
        else:
            observations.append(row)
    totals = {key: sum(row[key] for row in invocations) for key in GLOBAL_LIMITS}
    report['core_accounting_schema_version'] = 2
    report['core_charged_subset_totals'] = totals
    report['core_observation_pending_invocations'] = pending
    report['core_recorded_observations_parsed'] = not pending
    report['core_observation_scope'] = 'Parsed direct records and mutation prefixes summed once; pending observations excluded.'
    report["core_direct_and_stats_observations"] = {
        key: sum(row.get(key, 0) for row in observations)
        for key in ("direct_fit_calls", "stats_calls")}
    report["core_direct_and_stats_observations"]["point_admissions"] = sum(
        row.get('legacy_point_admissions', row.get('point_admissions', 0))
        for row in observations)
    report['core_normalization_observations'] = {
        key: sum(row.get(key, 0) for row in observations)
        for key in ('normalization_requests', 'observation_constructor_attempts', 'context_constructions',
                    'monotonic_constructor_attempts', 'utc_coordinate_constructor_attempts',
                    'exchange_arithmetic_calls', 'point_getter_requests')}
    history_fields = ('history_constructions', 'history_append_requests',
                      'history_observation_constructor_attempts', 'history_context_constructions',
                      'history_monotonic_constructor_attempts', 'history_utc_coordinate_constructor_attempts',
                      'history_exchange_arithmetic_calls', 'history_product_getter_requests',
                      'history_direct_normalization_requests', 'history_direct_stats_calls',
                      'history_direct_fit_calls', 'history_direct_point_admissions')
    report['core_history_external_observations'] = {
        key: sum(row.get(key, 0) for row in observations) for key in history_fields}
    upper_fields = ('history_normalization_entries_upper', 'history_statistics_entries_upper',
                    'history_statistics_inner_fit_upper', 'history_duration_operations_upper',
                    'history_generated_point_admissions_upper', 'history_fit_input_constructions_upper',
                    'history_staging_Copy_assignments_upper')
    report['core_history_source_owned_uppers'] = {
        key: sum(row.get(key, 0) for row in observations) for key in upper_fields}
    report['core_runs_external_observations'] = {
        key: sum(row.get(key, 0) for row in observations)
        for key in ('runs_points', 'runs_inputs', 'runs_selections', 'runs_guard_candidates',
                    'runs_external_records', 'runs_sign_records', 'runs_guard_records',
                    'runs_guard_table_calls', 'runs_guard_count_calls', 'runs_guard_context_calls',
                    'runs_guard_returned_window_rows', 'runs_sign_receipts',
                    'runs_unqualified_sign_receipts')}
    report['core_runs_source_owned_uppers'] = {
        key: sum(row.get(key, 0) for row in observations)
        for key in ('runs_fit_upper', 'runs_point_copy_upper', 'runs_decision_residual_upper',
                    'runs_checked_float_upper', 'runs_input_slots_upper', 'runs_residual_slots_upper',
                    'runs_observer_hooks_upper', 'runs_observer_pair_comparisons_upper')}
    selection_reserved = {key: sum(row.get('selection_reserved_calls', {}).get(key, 0) for row in invocations)
                          for key in SELECTION_GLOBAL_LIMITS}
    helper_reserved = {key: sum(row.get('selection_reserved_helper_calls', {}).get(key, 0) for row in invocations)
                       for key in SELECTION_HELPER_GLOBAL_LIMITS}
    report['core_selection_reserved_totals'] = selection_reserved
    report['core_selection_helper_reserved_totals'] = helper_reserved
    helper_work_reserved = {
        key: sum(row.get('selection_reserved_helper_canonical_source_work_uppers', {}).get(key, 0)
                 for row in invocations)
        for key in SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS}
    report['core_selection_helper_canonical_source_work_uppers'] = helper_work_reserved
    report['core_selection_helper_canonical_work_scope'] = SELECTION_TELEMETRY['helper_canonical_work_scope']
    raw_private_reserved = {
        key: sum(row.get('source_raw_private_reserved_work', {}).get(key, 0) for row in invocations)
        for key in SOURCE_RAW_PRIVATE_GLOBAL_LIMITS}
    report['core_source_raw_private_reserved_totals'] = raw_private_reserved
    report['core_source_raw_private_work_scope'] = SOURCE_RAW_PRIVATE_WORK['scope']
    package_calls, helper_calls, external_calls, selection_pending = [], [], [], []
    partial_subtotals = []
    for row in invocations:
        observed = row.get('observed_counts', row)
        if (row.get('observation_status') == 'unviable-compile-only'
                or not isinstance(observed, dict)):
            continue
        if row.get('selection_expected') == 'package':
            if observed.get('selection_observed_calls') is not None:
                package_calls.append(observed['selection_observed_calls'])
            if observed.get('selection_recorded_call_subtotals') is not None:
                partial_subtotals.append(observed['selection_recorded_call_subtotals'])
            if observed.get('selection_helper_observed_calls') is not None:
                helper_calls.append(observed['selection_helper_observed_calls'])
            if (observed.get('selection_observation_pending', True)
                    or observed.get('selection_helper_observation_pending', True)):
                selection_pending.append(row['invocation'])
        elif row.get('selection_expected') == 'external-consumer':
            if observed.get('selection_external_consumer_calls') is not None:
                external_calls.append(observed['selection_external_consumer_calls'])
            else:
                selection_pending.append(row['invocation'])
    def reported_sum(rows, keys):
        return {key: sum(row[key] for row in rows) for key in keys} if rows else None
    report['core_selection_package_observed_calls'] = reported_sum(package_calls, SELECTION.CALL_FIELDS)
    report['core_selection_helper_observed_calls'] = reported_sum(helper_calls, SELECTION_HELPER_RESERVATION)
    report['core_selection_external_observed_calls'] = reported_sum(external_calls, SELECTION.CALL_FIELDS)
    report['core_selection_reported_record_subtotals'] = reported_sum(partial_subtotals, SELECTION.CALL_FIELDS)
    report['core_selection_pending_observation_invocations'] = selection_pending
    report['core_selection_observation_scope'] = (
        'Missing marker families remain None/pending, never zero. Caught-Test prefixes may remain partial; '
        'this is separate from missing native invocations and never reduces the prior full reservation.')
    report['core_selection_source_work_scope'] = (
        'Canonical source projections only; neither arbitrary mutant internals nor CPU/RSS are measured.')
    projection_rows = [row for row in invocations if 'projection_reserved_calls' in row]
    report['core_projection_reserved_calls'] = {
        key: sum(row['projection_reserved_calls'][key] for row in projection_rows)
        for key in PROJECTION.FIELDS}
    projection_observations = [row.get('observed_counts', row) for row in projection_rows]
    projection_recorded = [row['projection_recorded_call_subtotals'] for row in projection_observations
                           if row.get('projection_recorded_call_subtotals') is not None]
    report['core_projection_recorded_call_subtotals'] = (
        {key: sum(row[key] for row in projection_recorded) for key in PROJECTION.FIELDS}
        if projection_recorded else None)
    report['core_projection_pending_observation_invocations'] = [
        row['invocation'] for row in projection_rows
        if row.get('observed_counts', row).get('projection_observation_pending', True)
        and row.get('observation_status') != 'unviable-compile-only']
    report['core_projection_observation_scope'] = (
        'Missing remains None/pending. Public attempts and private canonical source reservations '
        'are distinct; partial native prefixes never shrink the pre-effect reservation.')
    enclosure_rows = [row for row in invocations if 'enclosure_reserved_calls' in row]
    enclosure_reserved = {}
    for prefix, name in (('enclosure.call.', 'enclosure_reserved_calls'),
                         ('enclosure.work.', 'enclosure_reserved_source_work_uppers'),
                         ('enclosure.private.', 'enclosure_reserved_private_source_work')):
        keys = {key for row in enclosure_rows for key in row[name]}
        enclosure_reserved.update({prefix + key: sum(row[name][key] for row in enclosure_rows)
                                   for key in keys})
    report['core_enclosure_reserved_totals'] = enclosure_reserved
    enclosure_observations = [row.get('observed_counts', row) for row in enclosure_rows]
    enclosure_recorded = [row['enclosure_recorded_call_subtotals'] for row in enclosure_observations
                          if row.get('enclosure_recorded_call_subtotals') is not None]
    report['core_enclosure_recorded_call_subtotals'] = (
        {key: sum(row[key] for row in enclosure_recorded) for key in ENCLOSURE.FIELDS}
        if enclosure_recorded else None)
    enclosure_complete = [row['enclosure_observed_calls'] for row in enclosure_observations
                          if row.get('enclosure_observed_calls') is not None]
    report['core_enclosure_complete_observed_call_subtotals'] = (
        {key: sum(row[key] for row in enclosure_complete) for key in ENCLOSURE.FIELDS}
        if enclosure_complete else None)
    report['core_enclosure_pending_observation_invocations'] = [
        row['invocation'] for row in enclosure_rows
        if row.get('observed_counts', row).get('enclosure_observation_pending', True)
        and row.get('observation_status') != 'unviable-compile-only']
    report['core_enclosure_observation_scope'] = (
        'Reported public attempts only. Missing rows and failed prefixes stay pending; '
        'completed source recipes do not establish native outcomes. Private raw guards '
        'and literal doctests retain separate source reservations.')
    enclosure_limits = ENCLOSURE_TELEMETRY['global_limits']
    report['core_enclosure_cumulative_allowances_pending'] = enclosure_limits is None
    if enclosure_limits is not None and (
            type(enclosure_limits) is not dict
            or any(key not in enclosure_limits or type(enclosure_limits[key]) is not int
                   or amount > enclosure_limits[key] for key, amount in enclosure_reserved.items())):
        raise Incomplete('Enclosure selected subset exceeds its complete cumulative source-work allowance.')
    projected_batch_rows = [row for row in invocations if 'projected_batch_reserved_calls' in row]
    projected_batch_reserved = {}
    for prefix, name in (('projected_batch.call.', 'projected_batch_reserved_calls'),
                         ('projected_batch.work.', 'projected_batch_reserved_source_work_uppers'),
                         ('projected_batch.private.', 'projected_batch_reserved_private_source_work')):
        keys = projected_batch_reservation()[name]
        projected_batch_reserved.update({prefix + key: sum(row[name][key] for row in projected_batch_rows)
                                         for key in keys})
    report['core_projected_batch_reserved_totals'] = projected_batch_reserved
    projected_batch_observations = [row.get('observed_counts', row) for row in projected_batch_rows]
    for family, fields in (('projected_batch', PROJECTED_BATCH.FIELDS),
                           ('projected_batch_guard', PROJECTED_BATCH.GUARD_FIELDS)):
        recorded = [row[family + '_recorded_call_subtotals'] for row in projected_batch_observations
                    if row.get(family + '_recorded_call_subtotals') is not None]
        completed = [row[family + '_observed_calls'] for row in projected_batch_observations
                     if row.get(family + '_observed_calls') is not None]
        report['core_' + family + '_recorded_call_subtotals'] = reported_sum(recorded, fields)
        report['core_' + family + '_complete_observed_call_subtotals'] = reported_sum(completed, fields)
        report['core_' + family + '_pending_observation_invocations'] = [
            row['invocation'] for row in projected_batch_rows
            if row.get('observed_counts', row).get(family + '_observation_pending', True)
            and row.get('observation_status') != 'unviable-compile-only']
    report['core_projected_batch_observation_scope'] = (
        'Actual explicit attempt prefixes and passive callback/read observations remain separate. '
        'Callback/read observations overlap source work and are never added as new charges. '
        'Missing rows stay None/pending; no prefix reduces the complete pre-effect reservation.')
    projected_batch_limits = PROJECTED_BATCH_TELEMETRY['global_limits']
    report['core_projected_batch_cumulative_allowances_pending'] = projected_batch_limits is None
    if projected_batch_limits is not None and (
            type(projected_batch_limits) is not dict
            or set(projected_batch_limits) != set(projected_batch_reserved)
            or any(type(projected_batch_limits[key]) is not int
                   or amount > projected_batch_limits[key]
                   for key, amount in projected_batch_reserved.items())):
        raise Incomplete('Projected-batch selected subset exceeds its complete cumulative source-work allowance.')
    principal_rows = [row for row in invocations
                      if 'principal_reserved_calls' in row or 'principal_external_reserved_calls' in row]
    principal_reserved = {}
    for prefix, package_key, external_key in (
            ('principal.call.', 'principal_reserved_calls', 'principal_external_reserved_calls'),
            ('principal.work.', 'principal_reserved_source_work_uppers', 'principal_external_reserved_source_work_uppers')):
        keys = principal_reservation()[package_key]
        principal_reserved.update({prefix + key: sum(row.get(package_key, row.get(external_key, {}))[key]
                                                    for row in principal_rows) for key in keys})
    report['core_principal_reserved_totals'] = principal_reserved
    principal_observations = [row.get('observed_counts', row) for row in principal_rows]
    principal_recorded = [row['principal_recorded_call_subtotals'] for row in principal_observations
                          if row.get('principal_recorded_call_subtotals') is not None]
    report['core_principal_recorded_call_subtotals'] = reported_sum(principal_recorded, PRINCIPAL_FIELDS)
    report['core_principal_pending_observation_invocations'] = [
        row['invocation'] for row in principal_rows
        if row.get('observed_counts', row).get('principal_observation_pending', True)
        and row.get('observation_status') != 'unviable-compile-only']
    principal_limits = PRINCIPAL_TELEMETRY['global_limits']
    report['core_principal_cumulative_allowances_pending'] = principal_limits is None
    if principal_limits is not None and (
            type(principal_limits) is not dict or set(principal_limits) != set(principal_reserved)
            or any(type(principal_limits[key]) is not int or amount > principal_limits[key]
                   for key, amount in principal_reserved.items())):
        raise Incomplete('Principal selected subset exceeds its complete checkpoint source-work allowance.')
    combination_rows = [row for row in invocations
                        if 'combination_reserved_calls' in row or 'combination_external_reserved_calls' in row]
    combination_reserved = {}
    for prefix, package_key, external_key in (
            ('combination.call.', 'combination_reserved_calls', 'combination_external_reserved_calls'),
            ('combination.work.', 'combination_reserved_source_work_uppers', 'combination_external_reserved_source_work_uppers')):
        keys = combination_reservation()[package_key]
        combination_reserved.update({prefix + key: sum(row.get(package_key, row.get(external_key, {}))[key]
                                                      for row in combination_rows) for key in keys})
    report['core_combination_reserved_totals'] = combination_reserved
    combination_observations = [row.get('observed_counts', row) for row in combination_rows]
    combination_recorded = [row['combination_recorded_call_subtotals'] for row in combination_observations
                            if row.get('combination_recorded_call_subtotals') is not None]
    combination_complete = [row['combination_observed_calls'] for row in combination_observations
                            if row.get('combination_observed_calls') is not None]
    report['core_combination_recorded_call_subtotals'] = reported_sum(combination_recorded, COMBINATION.FIELDS)
    report['core_combination_complete_observed_call_subtotals'] = reported_sum(combination_complete, COMBINATION.FIELDS)
    report['core_combination_pending_observation_invocations'] = [
        row['invocation'] for row in combination_rows
        if row.get('observed_counts', row).get('combination_observation_pending', True)
        and row.get('observation_status') != 'unviable-compile-only']
    report['core_combination_observation_scope'] = (
        'Actual precharged public attempt prefixes only. Genuine producer source work is charged '
        'once through its first20 fields; constructor/kernel work is a disjoint source upper. '
        'The separate contract delta reuses the principal producer and adds no fits or points. '
        'Missing/partial profiles remain None/pending and never shrink a full package reservation.')
    combination_limits = COMBINATION_TELEMETRY['global_limits']
    report['core_combination_cumulative_allowances_pending'] = combination_limits is None
    if combination_limits is not None and (
            type(combination_limits) is not dict or set(combination_limits) != set(combination_reserved)
            or any(type(combination_limits[key]) is not int or amount > combination_limits[key]
                   for key, amount in combination_reserved.items())):
        raise Incomplete('Combination selected subset exceeds its complete checkpoint source-work allowance.')
    report["global_campaign_custody"] = "Coordinator must reconcile all invocations, literal doctest upper calls, inherited E events and retries; wrapper records its selected subset only."
    if any(totals[key] > maximum for key, maximum in GLOBAL_LIMITS.items()):
        raise Incomplete("Core selected subset already exceeds the global charged allowance.")
    if (any(selection_reserved[key] > maximum for key, maximum in SELECTION_GLOBAL_LIMITS.items())
            or any(helper_reserved[key] > maximum for key, maximum in SELECTION_HELPER_GLOBAL_LIMITS.items())
            or any(helper_work_reserved[key] > maximum
                   for key, maximum in SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS.items())
            or any(raw_private_reserved[key] > maximum for key, maximum in SOURCE_RAW_PRIVATE_GLOBAL_LIMITS.items())):
        raise Incomplete('Selection selected subset already exceeds the global charged allowance.')


def caught_native_test_failure(outcome, output):
    """Require actual named Rust test failures as well as Cargo phase statuses.

    Doctest compilation and Cargo setup errors do not supply this native-test
    failure block. This establishes reached failed tests, not full mutant
    identity or causality; COMMON's complete outcome gates remain mandatory.
    """
    if outcome.get('summary') != 'CaughtMutant':
        raise Incomplete('Partial counter observations require a caught outcome.')
    COMMON.mutation_phases(SCOPE, outcome)
    blocks = re.findall(
        r'(?m)^failures:\r?\n(?:\r?\n)?((?:    [A-Za-z_][A-Za-z0-9_:]*\r?\n)+)'
        r'\r?\ntest result: FAILED\. ([0-9]{1,3}) passed; ([0-9]{1,3}) failed; 0 ignored;',
        output)
    if not blocks or any(int(failed) != len(names.splitlines())
                         or not 1 <= int(failed) <= final_package_test_count('ordinary')
                         or int(passed) + int(failed) > final_package_test_count('ordinary')
                         for names, passed, failed in blocks):
        raise Incomplete('Caught outcome has no complete actual native failed-test evidence.')


def cargo_phase_segments(outcome, output):
    """Bind raw phase banners to exact successful/failed JSON Cargo phase argv."""
    COMMON.mutation_phases(SCOPE, outcome)
    phases = outcome['phase_results']
    for phase in phases:
        if '--message-format=json' not in phase['argv']:
            raise Incomplete('Core mutation phase lacks the required Cargo JSON format.')
    headers = list(re.finditer(r'(?m)^\*\*\* (cargo [^\r\n]*)\r?\n', output))
    if [header.group(1) for header in headers] != [' '.join(phase['argv']) for phase in phases]:
        raise Incomplete('Core mutation phase banners differ from native phase argv.')
    return {phase['phase']: output[header.end():headers[index + 1].start()
                                   if index + 1 < len(headers) else len(output)]
            for index, (phase, header) in enumerate(zip(phases, headers, strict=True))}


def cargo_json_records(output, *, test_phase=False):
    """Remove only complete known Cargo JSON records; retain other bytes verbatim.

    Test callers may retain complete captured principal assertion blocks and
    the three captured selection assertion tails. They are not JSON records
    or evidence of a compiler error/native test failure.
    Build/default callers still reject every malformed brace-prefixed line.
    """
    if type(test_phase) is not bool:
        raise Incomplete('Cargo JSON Test-context mode must be Boolean.')
    def unique_pairs(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError('duplicate JSON key')
            result[key] = value
        return result
    def invalid_constant(_value):
        raise ValueError('nonfinite JSON number')
    def finite_float(value):
        result = float(value)
        if not math.isfinite(result):
            raise ValueError('nonfinite JSON number')
        return result
    shapes = {
        'compiler-message': {'reason', 'package_id', 'manifest_path', 'target', 'message'},
        'compiler-artifact': {'reason', 'package_id', 'manifest_path', 'target', 'profile',
                              'features', 'filenames', 'executable', 'fresh'},
        'build-finished': {'reason', 'success'},
        'build-script-executed': {'reason', 'package_id', 'linked_libs', 'linked_paths',
                                  'cfgs', 'env', 'out_dir'},
    }
    # These complete source-owned assertions contain Rust braces at line starts.
    # Admit their exact lines together in Test output; a bare brace, near miss or
    # partial block still reaches the strict JSON parser below.
    principal_assertions = (
        ('assertion failed: matches!(rejected,Err(FixturePrincipalError::InvalidParameter',
         '    {field:actual_field,issue:actual_issue}) if actual_field==field &&',
         '    actual_issue==issue)'),
        ('assertion failed: matches!(rejected,',
         '    Err(FixturePrincipalError::Arithmetic',
         '    {',
         '        step: FixturePrincipalStep::StratumPenalty, source:',
         '        FitError::Arithmetic',
         '        {',
         '            stage: ArithmeticStage::Principal, operation:',
         '            ArithmeticOperation::Multiply, issue: ArithmeticIssue::NonFinite',
         '        },',
         '    }))'),
    )
    lines = output.splitlines(keepends=True)
    principal_assertion_lines = set()
    if test_phase:
        for assertion in principal_assertions:
            for newline in ('\n', '\r\n'):
                expected = [line + newline for line in assertion]
                for index, line in enumerate(lines):
                    if line == expected[0] and lines[index:index + len(expected)] == expected:
                        principal_assertion_lines.update(range(index, index + len(expected)))
    records, raw = [], []
    for index, line in enumerate(lines):
        if index in principal_assertion_lines:
            raw.append(line)
            continue
        if not line.lstrip().startswith('{'):
            raw.append(line)
            continue
        if test_phase and re.fullmatch(
                r'    \{ stage: SelectionInvariantStage::(?:Component|Support|Intersection) \}\)\)\r?\n', line):
            raw.append(line)
            continue
        if not line.endswith('\n'):
            raise Incomplete('Cargo JSON record is truncated.')
        if len(line.encode('utf-8')) > 4_194_304 or len(records) >= 65_536:
            raise Incomplete('Cargo JSON transport exceeds its record bounds.')
        try:
            item = json.loads(line, object_pairs_hook=unique_pairs,
                              parse_constant=invalid_constant, parse_float=finite_float)
        except (ValueError, TypeError, RecursionError) as error:
            raise Incomplete('Cargo JSON record is malformed.') from error
        if (not isinstance(item, dict) or not isinstance(item.get('reason'), str)
                or item['reason'] not in shapes or set(item) != shapes[item['reason']]):
            raise Incomplete('Cargo JSON record has an unknown transport shape.')
        if item['reason'] == 'build-finished':
            if type(item['success']) is not bool:
                raise Incomplete('Cargo build-finished status is not boolean.')
        elif item['reason'] == 'build-script-executed':
            packages = {
                'registry+https://github.com/rust-lang/crates.io-index#proc-macro2@1.0.107': 'proc-macro2',
                'registry+https://github.com/rust-lang/crates.io-index#quote@1.0.47': 'quote',
                'registry+https://github.com/rust-lang/crates.io-index#rustversion@1.0.23': 'rustversion',
            }
            package = item['package_id']
            if not isinstance(package, str) or package not in packages:
                raise Incomplete('Cargo build-script package is outside the observed pinned cohort.')
            for key in ('linked_libs', 'linked_paths', 'cfgs'):
                values = item[key]
                if (not isinstance(values, list) or len(values) > 256
                        or any(not isinstance(value, str) or len(value.encode('utf-8')) > 4096
                               for value in values)):
                    raise Incomplete('Cargo build-script list is malformed or exceeds its bounds.')
            env = item['env']
            if (not isinstance(env, list) or len(env) > 256
                    or any(not isinstance(pair, list) or len(pair) != 2
                           or any(not isinstance(value, str) or len(value.encode('utf-8')) > 4096
                                  for value in pair) for pair in env)):
                raise Incomplete('Cargo build-script environment is malformed or exceeds its bounds.')
            out_dir = item['out_dir']
            if (not isinstance(out_dir, str) or len(out_dir.encode('utf-8')) > 4096
                    or not re.fullmatch(
                        r'/work/tmp/cargo-mutants-source-[A-Za-z0-9]{6}\.tmp/target/debug/build/'
                        + re.escape(packages[package]) + r'-[a-f0-9]{16}/out', out_dir)):
                raise Incomplete('Cargo build-script output directory is outside the observed layout.')
        elif (not isinstance(item['package_id'], str) or not isinstance(item['manifest_path'], str)
              or not Path(item['manifest_path']).is_absolute() or not isinstance(item['target'], dict)):
            raise Incomplete('Cargo JSON package or target is malformed.')
        else:
            target = item['target']
            if (set(target) != {'kind', 'crate_types', 'name', 'src_path', 'edition', 'doc', 'doctest', 'test'}
                    or not isinstance(target['kind'], list) or not target['kind']
                    or not all(isinstance(value, str) for value in target['kind'])
                    or not isinstance(target['crate_types'], list) or not target['crate_types']
                    or not all(isinstance(value, str) for value in target['crate_types'])
                    or not isinstance(target['name'], str) or not isinstance(target['src_path'], str)
                    or not Path(target['src_path']).is_absolute()
                    or target['edition'] not in ('2015', '2018', '2021', '2024')
                    or any(type(target[field]) is not bool for field in ('doc', 'doctest', 'test'))):
                raise Incomplete('Cargo JSON target shape is malformed.')
        if item['reason'] == 'compiler-message':
            message = item['message']
            if (not isinstance(message, dict) or set(message) != {
                    '$message_type', 'children', 'code', 'level', 'message', 'rendered', 'spans'}
                    or message['$message_type'] != 'diagnostic'
                    or not isinstance(message['children'], list)
                    or not isinstance(message['spans'], list)
                    or not isinstance(message['level'], str)
                    or not isinstance(message['message'], str)
                    or (message['rendered'] is not None and not isinstance(message['rendered'], str))):
                raise Incomplete('Cargo compiler-message is malformed.')
        elif item['reason'] == 'compiler-artifact' and (
                not isinstance(item['profile'], dict) or not isinstance(item['features'], list)
                or not all(isinstance(value, str) for value in item['features'])
                or not isinstance(item['filenames'], list)
                or not all(isinstance(value, str) for value in item['filenames']) or type(item['fresh']) is not bool
                or (item['executable'] is not None and not isinstance(item['executable'], str))):
            raise Incomplete('Cargo compiler-artifact is malformed.')
        records.append(item)
    return records, ''.join(raw)


INFRASTRUCTURE_FAILURE = (
    r'(?i)resource temporarily unavailable|os error \d+|permission denied|'
    r'could not execute process|failed to spawn|no space left|cannot allocate memory|'
    r'out of memory|failed to run custom build command|linker|linking|'
    r'internal compiler error|unexpectedly panicked|panicked at|fatal runtime error|'
    r'terminated by signal|failed to load manifest'
)


def owned_compiler_span_path(value):
    """Resolve a relative rustc span lexically without leaving the owned crate."""
    prefix = 'crates/time-core/'
    if not isinstance(value, str) or not value.startswith(prefix):
        return None
    parts = []
    for part in value[len(prefix):].split('/'):
        if part == '..':
            if not parts:
                return None
            parts.pop()
        elif re.fullmatch(r'[A-Za-z0-9_-]+(?:\.rs)?', part):
            parts.append(part)
        else:
            return None
    canonical = prefix + '/'.join(parts)
    if len(parts) < 2 or parts[0] not in ('src', 'tests') or canonical not in BOUND_INPUTS:
        return None
    return canonical


def unviable_compile_failure(outcome, output):
    """Qualify one owned Cargo-JSON Build failure, never fragments from other phases."""
    build = cargo_phase_segments(outcome, output)['Build']
    records, raw = cargo_json_records(build)
    finished = [item for item in records if item['reason'] == 'build-finished']
    if len(finished) != 1 or finished[0]['success'] is not False or records[-1] is not finished[0]:
        raise Incomplete('Cargo Build lacks one unsuccessful build-finished record.')
    if (re.search(INFRASTRUCTURE_FAILURE, raw)
            or 'CORE_' in raw):
        raise Incomplete('Cargo Build contains infrastructure or product-execution evidence.')
    for line in raw.splitlines():
        if re.match(r'^\s*error(?:\[[^\]]+\])?:', line) and not re.match(
                r'^\s*error: could not compile \x60time-core\x60(?: \([^)]*\))? due to ', line):
            raise Incomplete('Cargo Build contains an unqualified non-JSON error.')
    errors, failure_notes = [], []
    coded_const_spans, null_const_spans = set(), []
    workspace = None
    for item in records:
        if item['reason'] != 'compiler-message':
            continue
        message = item['message']
        if message.get('level') not in ('error', 'warning', 'note', 'help', 'failure-note'):
            raise Incomplete('Cargo compiler-message has an unsupported level.')
        pending = [(message, 0)]
        inspected = 0
        while pending:
            diagnostic, depth = pending.pop()
            inspected += 1
            if (depth > 16 or inspected > 256 or not isinstance(diagnostic, dict)
                    or not isinstance(diagnostic.get('message'), str)
                    or not isinstance(diagnostic.get('children'), list)
                    or (depth and diagnostic.get('level') == 'error')
                    or re.search(INFRASTRUCTURE_FAILURE, diagnostic['message'])):
                raise Incomplete('Cargo diagnostic contains unqualified nested or infrastructure data.')
            pending.extend((child, depth + 1) for child in diagnostic['children'])
        if message['level'] == 'failure-note' and (
                message['code'] is not None or any(
                    not isinstance(span, dict) or span.get('is_primary') is not False
                    for span in message['spans'])):
            raise Incomplete('Cargo failure-note has unexpected code or primary span.')
        if message.get('level') != 'error':
            if message['level'] == 'failure-note':
                failure_notes.append(item)
            continue
        match = re.fullmatch(r'path\+file://(/work/tmp/cargo-mutants-source-[A-Za-z0-9]{6}\.tmp)'
                             r'/crates/time-core#0\.0\.0', item['package_id'])
        if not match or (workspace is not None and workspace != match.group(1)):
            raise Incomplete('Cargo compiler error belongs to an unqualified package.')
        workspace = match.group(1)
        target = item['target']
        expected_target = {'kind': ['lib'], 'crate_types': ['lib'], 'name': 'time_core',
                           'src_path': workspace + '/crates/time-core/src/lib.rs',
                           'edition': '2024', 'doc': True, 'doctest': True, 'test': True}
        if (item['manifest_path'] != workspace + '/crates/time-core/Cargo.toml'
                or target != expected_target
                or any(type(target[key]) is not bool for key in ('doc', 'doctest', 'test'))):
            raise Incomplete('Cargo compiler error target and manifest disagree.')
        code = message.get('code')
        null_const = code is None and message['message'] == '`TryInto` is not yet stable as a const trait'
        if not null_const and (not isinstance(code, dict) or not isinstance(code.get('code'), str)
                or set(code) != {'code', 'explanation'}
                or (code['explanation'] is not None and not isinstance(code['explanation'], str))
                or (not re.fullmatch(r'E[0-9]{4}', code['code'])
                    and not (code['code'] in ('unused_variables', 'dead_code', 'unused_imports',
                                              'arithmetic_overflow')
                             and code['explanation'] is None)
                    and not (code['code'] == 'unused_comparisons' and code['explanation'] is None
                             and message['message'] == 'comparison is useless due to type limits'))):
            raise Incomplete('Cargo compiler error has an unadmitted diagnostic code.')
        spans = message.get('spans')
        if not isinstance(spans, list) or not spans:
            raise Incomplete('Cargo compiler error lacks an owned primary span.')
        if any(not isinstance(span, dict) or type(span.get('is_primary')) is not bool for span in spans):
            raise Incomplete('Cargo compiler span is malformed.')
        primary = [span for span in spans if span['is_primary']]
        if (not primary or any(owned_compiler_span_path(span.get('file_name')) is None
                               or type(span.get('line_start')) is not int or span['line_start'] < 1
                               or type(span.get('column_start')) is not int or span['column_start'] < 1
                               for span in primary)):
            raise Incomplete('Cargo compiler error lacks an owned primary span.')
        locations = {(workspace, owned_compiler_span_path(span['file_name']), span['line_start'])
                     for span in primary}
        if null_const:
            null_const_spans.extend(locations)
        elif code['code'] == 'E0658':
            coded_const_spans.update(locations)
        errors.append(item)
    if not errors:
        raise Incomplete('Cargo Build lacks an owned typed compiler error.')
    if any(location not in coded_const_spans for location in null_const_spans):
        raise Incomplete('Cargo const-trait diagnostic lacks its same-producer E0658 companion.')
    if any(note['package_id'] != errors[0]['package_id']
           or note['manifest_path'] != errors[0]['manifest_path']
           or note['target'] != errors[0]['target'] for note in failure_notes):
        raise Incomplete('Cargo failure-note belongs to a different producer.')


def observe_mutation_outcome(report, path, label, outcome, reserved):
    """Qualify one already-charged raw outcome without inventing observations."""
    relative = Path(outcome.get("log_path", ""))
    if relative.is_absolute() or ".." in relative.parts or relative == Path("."):
        raise Incomplete("Core per-file mutation log escapes its owned path.")
    log = path.parent / relative
    if log.exists() and not log.resolve(strict=True).is_relative_to(path.parent.resolve(strict=True)):
        raise Incomplete("Core per-file mutation log resolves outside its owned path.")
    baseline = outcome.get("scenario") == "Baseline"
    summary = outcome.get('summary')
    if baseline and summary != 'Success':
        raise Incomplete('Core mutation baseline lacks a successful outcome.')
    if not baseline and summary not in ('CaughtMutant', 'MissedMutant', 'Unviable', 'Timeout'):
        raise Incomplete('Core mutation outcome has an unclassified summary.')
    if not baseline and summary == 'Timeout' and not log.exists():
        reserved['observation_status'] = 'timeout-log-missing'
        return
    text = log.read_bytes().decode('utf-8')
    reserved['raw_log_sha256'] = PRODUCT.sha256(log)
    if not baseline and summary == 'Timeout':
        # An absent prefix is incomplete, not an observed zero or a healthy run.
        test_header = (r'(?m)^\*\*\* cargo test --verbose --package=time-core@0\.0\.0 '
                       r'--message-format=json --frozen\r?\n')
        headers = list(re.finditer(test_header, text))
        if len(headers) > 1:
            raise Incomplete('Timeout contains duplicate mutation Test phase banners.')
        if not headers:
            reserved['observation_status'] = 'timeout-no-counter-prefix'
            return
        _, text = cargo_json_records(text[headers[0].end():], test_phase=True)
        if 'CORE_SELECTION_EXTERNAL_PROFILE' in text:
            raise Incomplete('Mutation Timeout contains a standalone consumer counter.')
        receipt_pattern = r'CORE_RUNS_SIGN_RECEIPT [^\r\n]*\r?\n'
        receipts = re.findall(receipt_pattern, text)
        if (text.count('CORE_RUNS_SIGN_RECEIPT') != len(receipts) or len(receipts) > 1
                or any(len(line.encode('utf-8')) > 4096 for line in receipts)):
            raise Incomplete('Timeout sign receipt is malformed or exceeds its byte bound.')
        # A test can emit its receipt before Drop emits the associated profile.
        # Validate each available counter prefix independently and qualify none.
        prefix = re.sub(receipt_pattern, '', text)
        markers = ('CORE_PROFILE_COUNTS', 'CORE_STATS_PROFILE_COUNTS',
                   'CORE_NORMALIZATION_PROFILE_COUNTS', 'CORE_HISTORY_PROFILE_COUNTS',
                   'CORE_RUNS_PROFILE', 'CORE_RUNS_SIGN_PROFILE', 'CORE_RUNS_GUARDS',
                   'S13_SELECTION_PROFILE', 'S13_BATCH_GUARD_PROFILE', 'SELECTION_HELPER_PROFILE',
                   'CORE_PROJECTION_PROFILE', 'CORE_ENCLOSURE_PROFILE',
                   PROJECTED_BATCH.MARKER, PROJECTED_BATCH.GUARD_MARKER, 'CORE_PRINCIPAL_',
                   'CORE_COMBINATION_')
        if any(marker in prefix for marker in markers):
            reserved['observation_status'] = 'timeout-prefix-invalid'
            profile_counts(prefix, require_fit=False, require_runs=False,
                           validate_runs_receipt=False)
            reserved['observation_status'] = 'timeout-prefix-unqualified'
        elif receipts:
            reserved['observation_status'] = 'timeout-receipt-unqualified'
        else:
            reserved['observation_status'] = 'timeout-no-counter-prefix'
        return
    if not baseline and summary == 'Unviable':
        unviable_compile_failure(outcome, text)
        reserved.update({'charged_fit_work_upper': 0, 'point_admissions_upper': 0,
                         'observation_status': 'unviable-compile-only',
                         'execution': 'descriptive compiler failure; no behavioral kill',
                         'raw_log_sha256': PRODUCT.sha256(log)})
        reserved.pop('observation_pending')
        refresh_accounting(report)
        return
    caught = not baseline and summary == 'CaughtMutant'
    segments = cargo_phase_segments(outcome, text)
    build_records, _ = cargo_json_records(segments['Build'])
    build_finished = [record['success'] for record in build_records if record['reason'] == 'build-finished']
    if build_finished != [True]:
        raise Incomplete('Successful mutation Build lacks its complete Cargo JSON receipt.')
    test_records, test_output = cargo_json_records(segments['Test'], test_phase=True)
    if 'CORE_SELECTION_EXTERNAL_PROFILE' in test_output:
        raise Incomplete('Mutation Test contains a standalone consumer counter.')
    test_finished = [record['success'] for record in test_records if record['reason'] == 'build-finished']
    if test_finished != [True]:
        raise Incomplete('Mutation Test lacks its complete Cargo JSON build receipt.')
    if any(record['reason'] == 'compiler-message' and record['message']['level'] == 'error'
           for record in (*build_records, *test_records)):
        raise Incomplete('Successful Cargo build receipt conflicts with an error diagnostic.')
    if caught:
        caught_native_test_failure(outcome, test_output)
    elif COMMON.test_count(test_output) != final_package_test_count('baseline'):
        raise Incomplete(f'Successful core package invocation did not execute all {final_package_test_count("baseline")} admitted tests/doctests.')
    observed = profile_counts(test_output, require_fit=baseline, require_runs=not caught,
                              validate_runs_receipt=not caught, require_selection=not caught,
                              require_projection=not caught, require_enclosure=not caught,
                              require_projected_batch=not caught, require_principal=not caught,
                              require_combination=not caught)
    reserved['observed_counts'] = account_literal_doctests(observed)
    reserved['observation_pending'] = False
    reserved['raw_log_sha256'] = PRODUCT.sha256(log)
    if caught:
        reserved['counter_observation_scope'] = 'failed-test prefix; raw sign receipts unqualified'
        report.setdefault('mutation_counter_prefix_observations', []).append(label)
    reserved['execution'] = 'pessimistic post-run invocation charge; observed counts retained separately'
    refresh_accounting(report)


def mutation_accounting(report, work):
    """Retain separate complete raw outcomes for each selected critical file.

    Each selected file has its own full-package baseline. Both old and new test
    binaries may run for every variant: never discount work to the mutated file.
    The unchanged COMMON gate remains authoritative for each95/no-unresolved.
    """
    selected = report.get("subjects", SUBJECTS)
    accounting = report.setdefault("mutation_full_accounting", {})
    missing = []
    reservations = []
    limit_failure = None
    report["mutation_missing_selected_files"] = missing
    unprocessed = list(selected)
    report['mutation_unprocessed_selected_files'] = unprocessed
    report['mutation_counter_observation_complete'] = False
    for subject, filename in selected.items():
        path = work / "mutation" / subject / "mutants.out/outcomes.json"
        if not path.exists():
            missing.append(subject)
            unprocessed.remove(subject)
            continue
        value = json.loads(path.read_text())
        if not isinstance(value, dict):
            raise Incomplete("Core per-file raw mutation report must be an object.")
        symbols = {"CaughtMutant": "K", "MissedMutant": "M", "Unviable": "U", "Timeout": "T"}
        counts = {key: 0 for key in ("K", "M", "U", "T", "X", "E")}
        outcomes = value.get("outcomes", [])
        row = {"file": filename, "generated": value.get("total_mutants"),
               "counts": counts, "raw_report": str(path), "equivalences": [],
               "counts_scope": "Raw native outcome labels, not verified kills or qualification.",
               "unclassified_outcomes": 0, "baseline_records": 0}
        accounting[subject] = row
        if (not isinstance(outcomes, list) or len(outcomes) > 513
                or any(not isinstance(item, dict) for item in outcomes)):
            raise Incomplete("Core per-file raw mutation outcomes have an invalid shape.")
        for outcome in outcomes:
            if outcome.get("scenario") == "Baseline":
                row["baseline_records"] += 1
            elif outcome.get("summary") in symbols:
                counts[symbols[outcome["summary"]]] += 1
            else:
                row["unclassified_outcomes"] += 1
        generated = row["generated"]
        if type(generated) is not int or not 0 <= generated <= 512:
            raise Incomplete("Core per-file generated denominator is invalid.")
        row["accounted"] = sum(counts.values())
        row["unobserved"] = max(0, generated - row["accounted"])
        for index, outcome in enumerate(outcomes):
            label = subject + ":mutation-" + ("baseline" if outcome.get("scenario") == "Baseline" else str(index))
            # Reconcile a pessimistic complete-invocation charge before parsing
            # partial child output. This runs after the campaign; the coordinator
            # must independently reserve its entire admitted scope before launch.
            # 640 legacy fits + (9*8+2)*62 selector fits + 5 helper prepares;
            # 65536 legacy points + 9*128 + 24 + 64 external points +
            # (9*8+2)*2077 suffix copies + 5*64 helper suffix copies +
            # the separately declared three-point literal doctest allowance.
            try:
                record_counts(report, label, {
                    **package_charged_upper(include_doctests=True),
                    **selection_reservation(package=True),
                    'literal_doctests_reserved': True,
                    'observation_pending': True,
                    'execution': 'pessimistic post-run invocation charge; observed counts pending'})
            except Incomplete as error:
                if str(error) not in ("Core selected subset already exceeds the global charged allowance.",
                                      "Selection selected subset already exceeds the global charged allowance."):
                    raise
                # record_counts has retained and refreshed the over-limit row.
                # Keep recording every already-present outcome before failing.
                limit_failure = error
            reservations.append((path, label, outcome, report['core_invocations'][-1]))
        unprocessed.remove(subject)
    # Reserve every present cohort before attempting to parse any child log.
    if limit_failure is not None:
        raise limit_failure
    for path, label, outcome, reserved in reservations:
        observe_mutation_outcome(report, path, label, outcome, reserved)
    report["mutation_missing_selected_files"] = missing
    report['mutation_counter_observation_complete'] = (
        not missing and not report.get('core_observation_pending_invocations')
        and not report.get('mutation_counter_prefix_observations')
        and all(row['baseline_records'] == 1 and row['unobserved'] == 0
                and row['unclassified_outcomes'] == 0 for row in accounting.values()))
    report['mutation_counter_observation_scope'] = (
        'Per-invocation observed_counts retain complete healthy or well-formed failed-test prefixes; '
        'pessimistic charges assert no observed operation counts or healthy sign facts.')
    report['mutation_checkpoint_complete_selection'] = set(selected) == set(SUBJECTS)
    report['mutation_full_manifest_selected'] = False
    report["mutation_baselines_required"] = len(selected)
    report["mutation_baselines_observed"] = sum(row["baseline_records"] for row in accounting.values())
    report['mutation_generated_variants_selected_total'] = sum(row['generated'] for row in accounting.values())
    report['mutation_global_variant_ceiling'] = 512
    report['mutation_cross_campaign_reservation'] = (
        'This selected invocation is capped at512 variants; the complete coordinator '
        'plan reserves every disjoint owner cohort and baseline before launch; '
        'historical candidate scopes remain separately recorded, with no implicit retries or prefix reuse.')
    if report['mutation_generated_variants_selected_total'] > 512:
        raise Incomplete('Selected mutation variants exceed the global512 ceiling.')



def legacy_contract_examples():
    """Minimal real external legal callers and diagnostic-specific bypass attempts."""
    cases = {}
    imports = "use time_core::{FitPoint,FitInput,FixtureLine,fit};\nuse std::num::NonZeroU64;\n"
    for name, values, expected in [("legal_constant", "[0.5,0.5,0.5]", "assert_eq!(line.phase_seconds(),0.5); assert_eq!(line.phase_rate_seconds_per_second(),0.0);"),
                                   ("legal_output", "[-1.0,1.0,1.0]", "assert!((line.phase_seconds()-4.0/3.0).abs() <= 2_f64.powi(-36)); assert!((line.phase_rate_seconds_per_second()-1.0).abs() <= 2_f64.powi(-32));")]:
        source = imports + 'fn main(){\n let ys=' + values + ';\n let points=[FitPoint::new(-2.0,ys[0],1.0).unwrap(),FitPoint::new(-1.0,ys[1],1.0).unwrap(),FitPoint::new(0.0,ys[2],1.0).unwrap()];\n let frame=NonZeroU64::new(9).unwrap(); let input=FitInput::new(frame,&points).unwrap(); let line=fit(&input).unwrap();\n assert_eq!(line.frame(),frame); ' + expected + '\n eprintln!("CORE_PROFILE_COUNTS fits=1 points=3"); println!("legal-core-contract");\n}\n'
        cases[name] = {"source": source, "code": None}
    for role in ("FitPoint", "FitInput", "FixtureLine"):
        cases[role.lower() + "_default"] = {
            "source": imports + "fn main(){\n let _: " + role + " = Default::default();\n}\n",
            "code": "E0277", "tokens": [role, "Default"], "line": 4}
    literals = {
        "point_construction": ("FitPoint", "relative_seconds:-1.0,phase_seconds:0.0,variance_divisor:1.0"),
        "line_construction": ("FixtureLine", "frame:NonZeroU64::new(1).unwrap(),phase_seconds:0.0,phase_rate_seconds_per_second:0.0"),
    }
    for name, (role, fields) in literals.items():
        cases[name] = {"source": imports + "fn main(){\n let _=" + role + "{" + fields + "};\n}\n",
                       "code": "E0451", "tokens": [role, "private"], "line": 4}
    for field in ("relative_seconds", "phase_seconds", "variance_divisor"):
        source = imports + 'fn main(){\n let mut point=FitPoint::new(-1.0,0.0,1.0).unwrap();\n point.' + field + '=0.0;\n}\n'
        cases["point_mutation_" + field] = {"source": source, "code": "E0616", "tokens": [field, "private"], "line": 5}
    prelude = ('fn main(){\n let points=[FitPoint::new(-2.0,0.0,1.0).unwrap(),FitPoint::new(-1.0,0.0,1.0).unwrap(),FitPoint::new(0.0,0.0,1.0).unwrap()];\n'
               ' let mut input=FitInput::new(NonZeroU64::new(1).unwrap(),&points).unwrap();\n')
    for field, value in [("frame", "NonZeroU64::new(2).unwrap()"), ("len", "0")]:
        cases["input_mutation_" + field] = {"source": imports + prelude + " input." + field + "=" + value + ";\n}\n",
                                             "code": "E0616", "tokens": [field, "private"], "line": 6}
    for field in ("phase_seconds", "phase_rate_seconds_per_second"):
        cases["line_mutation_" + field] = {"source": imports + prelude + " let mut line=fit(&input).unwrap();\n line." + field + "=0.0;\n}\n",
                                            "code": "E0616", "tokens": [field, "private"], "line": 7}
    cases["private_module"] = {"source": "use time_core::regression;\nfn main(){}\n", "code": "E0603", "tokens": ["regression", "private"], "line": 1}
    cases["raw_points"] = {"source": imports + 'fn main(){\n let _=FitInput::new(NonZeroU64::new(1).unwrap(),&[0.0_f64]);\n}\n',
                           "code": "E0308", "tokens": ["FitPoint", "f64"], "line": 4}
    cases["zeroable_frame"] = {"source": imports + 'fn main(){\n let _=FitInput::new(0_u64,&[]);\n}\n',
                              "code": "E0308", "tokens": ["NonZero", "u64"], "line": 4}
    cases["raw_fit"] = {"source": imports + 'fn main(){\n let _=fit(&[]);\n}\n',
                       "code": "E0308", "tokens": ["FitInput"], "line": 4}
    for name in ("legal_constant", "legal_output"):
        cases[name]["calls"] = {"direct_fit_calls": 1, "stats_calls": 0, "point_admissions": 3}
    cases.update(residual_contract_examples())
    cases.update(normalization_contract_examples())
    cases.update(history_contract_examples())
    cases.update(runs_contract_examples())
    cases.update(SELECTION_CONTRACTS.contract_examples())
    cases.update(projection_enclosure_contract_examples())
    cases.update(projected_batch_contract_examples())
    return cases


def principal_legal_calls():
    """One real three-row producer/seed/advance consumer's pre-effect recipe."""
    return dict(zip(PRINCIPAL_FIELDS, (1,3,3,1,5,1,1,6,1,1,1,1,1,1,1,1,1,0,0,0), strict=True))


def combination_legal_calls():
    """One extra checked singleton call reuses the already charged producer.

    These are only twelve getters, one constructor and one kernel call. No
    history, observation, fit, projection, batch, seed or advance is repeated.
    """
    return {field: 12 if field == 'getters' else 1 if field in {'uncertainties', 'combinations'} else 0
            for field in COMBINATION.FIELDS}


def combination_contract_counts(output):
    """Require one exact independent legal-consumer delta, never a package row."""
    if (type(output) is not str or len(output) > COMBINATION.MAX_OUTPUT_CHARACTERS
            or output.count('CORE_COMBINATION_') != 1):
        raise Incomplete('Combination legal consumer omitted or mixed its one contract marker.')
    expected = combination_legal_calls()
    row = COMBINATION_CONTRACT_MARKER + ''.join(f' {field}={expected[field]}' for field in COMBINATION.FIELDS)
    observed = [line for line in output.splitlines(keepends=True) if 'CORE_COMBINATION_' in line]
    if len(observed) != 1 or observed[0] not in (row + '\n', row + '\r\n'):
        raise Incomplete('Combination legal consumer differs from its exact no-producer delta.')
    return dict(expected)


def combination_contract_examples():
    """Six compile-only private-field probes with typed parameters, no producers."""
    return {
        'combination_uncertainty_private_fields': {
            'source': 'use time_core::{FixtureProjectedInterval,FixtureSupporterUncertainty};\n'
                      'pub fn attempt(entry:&FixtureProjectedInterval){\n'
                      ' let _=FixtureSupporterUncertainty{entry,phase_standard_deviation_seconds:0.0,rate_standard_deviation_seconds_per_second:0.0,tracking_skew_seconds_per_second:0.0};\n}\nfn main(){}\n',
            'code': 'E0451', 'tokens': ['private','phase_standard_deviation_seconds'], 'line': 3},
        'combination_uncertainty_private_mutation': {
            'source': 'use time_core::FixtureSupporterUncertainty;\n'
                      'pub fn attempt(mut value:FixtureSupporterUncertainty<\'_>){\n'
                      ' value.phase_standard_deviation_seconds=0.0;\n}\nfn main(){}\n',
            'code': 'E0616', 'tokens': ['private','phase_standard_deviation_seconds'], 'line': 3},
        'combination_estimate_private_fields': {
            'source': 'use std::num::NonZeroU64;\nuse time_domain::MonotonicNs;\n'
                      'use time_core::{FixtureContext,FixtureCombinedEstimate};\n'
                      'pub fn attempt(context:FixtureContext,at:MonotonicNs,label:NonZeroU64){\n'
                      ' let _=FixtureCombinedEstimate{context,anchor_observed_at:at,latest_observed_at:at,fit_reference:at,projected_at:at,labels:[label;16],supporter_count:1,phase_seconds:0.0,phase_rate_seconds_per_second:0.0,phase_standard_deviation_seconds:0.0,rate_standard_deviation_seconds_per_second:0.0,tracking_skew_seconds_per_second:0.0};\n}\nfn main(){}\n',
            'code': 'E0451', 'tokens': ['private','phase_seconds'], 'line': 5},
        'combination_estimate_private_mutation': {
            'source': 'use time_core::FixtureCombinedEstimate;\n'
                      'pub fn attempt(mut value:FixtureCombinedEstimate){\n'
                      ' value.phase_seconds=0.0;\n}\nfn main(){}\n',
            'code': 'E0616', 'tokens': ['private','phase_seconds'], 'line': 3},
        'combination_outcome_private_fields': {
            'source': 'use time_core::{FixtureCombinationKernelOutcome,FixturePrincipalOutcomeKind};\n'
                      'pub fn attempt(kind:FixturePrincipalOutcomeKind){\n'
                      ' let _=FixtureCombinationKernelOutcome{principal_kind:kind,combined:None};\n}\nfn main(){}\n',
            'code': 'E0451', 'tokens': ['private','combined'], 'line': 3},
        'combination_outcome_private_mutation': {
            'source': 'use time_core::FixtureCombinationKernelOutcome;\n'
                      'pub fn attempt(mut value:FixtureCombinationKernelOutcome){\n'
                      ' value.combined=None;\n}\nfn main(){}\n',
            'code': 'E0616', 'tokens': ['private','combined'], 'line': 3},
    }


def contract_examples():
    """One genuine checked consumer and seven precise private-field bypasses.

    This registry establishes neither every privacy/trait property nor native
    mandatory-garde result use. The latter retains its separate open evidence.
    """
    source = r'''use std::{error::Error,num::NonZeroU64};
use time_core::{FixtureContext,FixtureHistory,FixtureObservation,FixtureProjectionTerms,
 FixtureProjectedBatch,FixtureMinimum,FixturePrincipalParameters,FixturePrincipalStratum,
 FixturePrincipalState,FixtureSupporterUncertainty,FixtureCombinationKernelKind,
 combine_fixture_supporters,project_fixture_anchor,enclose_fixture_projection};
use time_domain::{UtcCoordinateQ32,MonotonicNs,exchange_arithmetic};
struct Calls([u64;17]);
impl Drop for Calls { fn drop(&mut self) { let c=&self.0; eprintln!(
 "CORE_PRINCIPAL_CONTRACT_PROFILE histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} parameters={} strata={} seeds={} advances={}",
 c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8],c[9],c[10],c[11],c[12],c[13],c[14],c[15],c[16]); } }
struct CombinationCalls([u64;22]);
impl Drop for CombinationCalls { fn drop(&mut self) { let c=&self.0; eprintln!(
 "CORE_COMBINATION_CONTRACT_PROFILE histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} parameters={} strata={} seeds={} advances={} state_clones={} value_equalities={} state_formats={} uncertainties={} combinations={}",
 c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8],c[9],c[10],c[11],c[12],c[13],c[14],c[15],c[16],c[17],c[18],c[19],c[20],c[21]); } }
fn main()->Result<(),Box<dyn Error>> {
 let mut calls=Calls([0;17]);
 let mut combination_calls=CombinationCalls([0;22]);
 macro_rules! call { ($slot:literal,$value:expr)=>{{ calls.0[$slot]+=1; $value }}; }
 macro_rules! combination_call { ($slot:literal,$value:expr)=>{{ combination_calls.0[$slot]+=1; $value }}; }
 let label=NonZeroU64::new(41).ok_or("label")?;
 let frame=NonZeroU64::new(43).ok_or("frame")?;
 let generation=NonZeroU64::new(47).ok_or("generation")?;
 let context=call!(3,FixtureContext::new(label,frame,generation));
 let coordinate=call!(5,UtcCoordinateQ32::new(0))?;
 let exchange=call!(6,exchange_arithmetic(coordinate,coordinate,coordinate,coordinate))?;
 let phase=call!(7,exchange.offset());
 let mut history=call!(0,FixtureHistory::new(context));
 for raw in [0,1_000_000_000,2_000_000_000] {
  let at=call!(4,MonotonicNs::new(raw));
  let observation=call!(2,FixtureObservation::new(context,at,phase,1.0))?;
  call!(1,history.try_append(observation))?;
 }
 let anchor=call!(4,MonotonicNs::new(0));
 let at=call!(4,MonotonicNs::new(3_000_000_000));
 let terms=call!(8,FixtureProjectionTerms::new(1.0,0.0,0.0))?;
 let projection=call!(9,project_fixture_anchor(&history,context,0,anchor,at,terms))?;
 let interval=call!(10,enclose_fixture_projection(&projection))?;
 let entries=[interval];
 let batch=call!(11,FixtureProjectedBatch::new(frame,generation,at,&entries))?;
 let minimum=call!(12,FixtureMinimum::new(1))?;
 let parameters=call!(13,FixturePrincipalParameters::new(1.0,0.0))?;
 let strata=[call!(14,FixturePrincipalStratum::new(label,1))?];
 let mut state=call!(15,FixturePrincipalState::seed(&[label],parameters,minimum,&batch,&strata))?;
 let decision=call!(16,state.advance(&batch,&strata))?;
 assert_eq!(call!(7,state.principal()),Some(label));
 assert_eq!(call!(7,decision.principal_label()),Some(label));
 assert!(call!(7,decision.reference_ready()));
 let selected=call!(7,state.source_state(label)).ok_or("source state")?;
 assert_eq!(call!(7,selected.score()).to_bits(),1.0_f64.to_bits());
 let uncertainty=combination_call!(20,FixtureSupporterUncertainty::new(&entries[0],-0.0,0.0,0.0))?;
 assert!(std::ptr::eq(combination_call!(7,uncertainty.entry()),&entries[0]));
 let outcome=combination_call!(21,combine_fixture_supporters(&decision,&[uncertainty]))?;
 assert_eq!(combination_call!(7,outcome.kind()),FixtureCombinationKernelKind::Combined);
 assert_eq!(combination_call!(7,outcome.principal_kind()),combination_call!(7,decision.kind()));
 let estimate=combination_call!(7,outcome.combined()).ok_or("combined estimate")?;
 assert_eq!(combination_call!(7,estimate.principal_label()),label);
 assert_eq!(combination_call!(7,estimate.supporter_count()),1);
 assert_eq!(combination_call!(7,estimate.phase_seconds()).to_bits(),0);
 assert_eq!(combination_call!(7,estimate.phase_rate_seconds_per_second()).to_bits(),0);
 assert_eq!(combination_call!(7,estimate.phase_standard_deviation_seconds()).to_bits(),0x8000_0000_0000_0000);
 assert_eq!(combination_call!(7,estimate.rate_standard_deviation_seconds_per_second()).to_bits(),0);
 assert_eq!(combination_call!(7,estimate.tracking_skew_seconds_per_second()).to_bits(),0);
 println!("legal-core-contract");
 Ok(())
}
'''
    cases = {
        'legal_principal_checked_producer': {'source': source, 'code': None,
                                             # History work belongs to the principal recipe;
                                             # there are no direct legacy fit/stat/point calls.
                                             'calls': {'direct_fit_calls': 0, 'stats_calls': 0,
                                                       'point_admissions': 0},
                                             'principal_calls': principal_legal_calls(),
                                             'combination_calls': combination_legal_calls()},
        'principal_private_parameter_fields': {
            'source': 'use time_core::FixturePrincipalParameters;\nfn main(){\n let _=FixturePrincipalParameters{reselect_distance_seconds:1.0,stratum_weight_seconds:0.0};\n}\n',
            'code': 'E0451', 'tokens': ['reselect_distance_seconds', 'private'], 'line': 3},
    }
    cases.update(combination_contract_examples())
    return cases


def projection_enclosure_contract_examples():
    """Compile-only bypass probes; genuine producer controls live in Rust tests.

    Checked projections and intervals are supplied as typed closure arguments,
    never forged to reach a privacy diagnostic. These thirteen additions consume
    the reviewed 106-negative/10-legal budget without adding executable work.
    """
    cases = {}
    for name, role in (("projection_terms", "FixtureProjectionTerms"),
                       ("anchor_projection", "FixtureAnchorProjection"),
                       ("projected_interval", "FixtureProjectedInterval")):
        imports = "use time_core::" + role + ";\n"
        cases[name + "_default"] = {
            "source": imports + "fn main(){\n let _: " + role + " = Default::default();\n}\n",
            "code": "E0277", "tokens": [role, "Default"], "line": 3}
        cases[name + "_unchecked_from"] = {
            "source": imports + "fn main(){\n let _: " + role + " = <" + role + " as From<()>>::from(());\n}\n",
            "code": "E0277", "tokens": [role, "From<()>"], "line": 3}
    cases["projection_raw_terms_receiver"] = {
        "source": ('use time_core::{FixtureHistory,FixtureContext,project_fixture_anchor};\n'
                   'use time_domain::MonotonicNs;\nfn main(){\n'
                   ' let _probe=|history:&FixtureHistory, context:FixtureContext, at:MonotonicNs| {\n'
                   '  let _=project_fixture_anchor(history,context,0,at,at,[0.0_f64;3]);\n };\n}\n'),
        "code": "E0308", "tokens": ["FixtureProjectionTerms", "f64"], "line": 5}
    cases["projected_interval_private_construction"] = {
        "source": ('use time_core::{FixtureAnchorProjection,FixtureInterval,FixtureEndpointRounding,FixtureProjectedInterval};\n'
                   'fn main(){\n let _probe=|projection:FixtureAnchorProjection, interval:FixtureInterval| {\n'
                   '  let _=FixtureProjectedInterval{projection,interval,lower_rounding:FixtureEndpointRounding::Exact,upper_rounding:FixtureEndpointRounding::Exact};\n };\n}\n'),
        "code": "E0451", "tokens": ["FixtureProjectedInterval", "private"], "line": 4}
    for field, assignment in (("projection", "*item.projection()"),
                              ("interval", "*item.interval()"),
                              ("lower_rounding", "item.lower_rounding()"),
                              ("upper_rounding", "item.upper_rounding()")):
        cases["projected_interval_mutation_" + field] = {
            "source": ('use time_core::FixtureProjectedInterval;\nfn main(){\n'
                       ' let _probe=|mut item:FixtureProjectedInterval| {\n  item.' + field + '=' + assignment + ';\n };\n}\n'),
            "code": "E0616", "tokens": [field, "private"], "line": 4}
    cases["enclosure_raw_projection_receiver"] = {
        "source": ('use time_core::enclose_fixture_projection;\nfn main(){\n'
                   ' let _=enclose_fixture_projection(&[0.0_f64;2]);\n}\n'),
        "code": "E0308", "tokens": ["FixtureAnchorProjection", "f64"], "line": 3}
    return cases


def projected_batch_contract_examples():
    """Compile-only bypasses paired with the genuine external Rust test owner.

    Typed parameters supply already-checked values without public-field, unsafe
    or runtime forgery. The retained-borrow and checked-construction neighbors
    live in tests/projected_batch.rs and execute in the package lanes; this
    registry adds no eleventh legal executable or product-call reservation.
    Diagnostic expectations require Root's actual compiler observations before
    source admission. Pairing source alone proves no execution or acceptance.
    """
    imports = ('use time_core::{FixtureProjectedBatch,FixtureProjectedInterval,FixtureBatch,'
               'FixtureInterval,FixtureMinimum};\n'
               'use time_domain::{MonotonicNs,ActiveAwakeNs};\n'
               'use std::num::NonZeroU64;\n')
    neighbor = {'file': 'crates/time-core/tests/projected_batch.rs',
                'test': 'genuine_compatible_projection_reaches_real_batch_selection'}
    cases = {}

    def negative(name, prefix, expression, code, tokens, suffix='\n}\nfn main(){}\n'):
        """Bind the actual attempted operation and its external legal neighbor."""
        cases['projected_batch_' + name] = {
            'source': imports + prefix + expression + suffix,
            'code': code, 'tokens': tokens,
            'line': len((imports + prefix).splitlines()) + 1,
            'external_legal_neighbor': dict(neighbor),
        }

    negative('default', 'fn main(){\n',
             " let _: FixtureProjectedBatch<'static>=Default::default();",
             'E0277', ['FixtureProjectedBatch', 'Default'], '\n}\n')
    negative('unchecked_from', 'fn main(){\n',
             " let _: FixtureProjectedBatch<'static>=<FixtureProjectedBatch<'static> as From<()>>::from(());",
             'E0277', ['FixtureProjectedBatch', 'From<()>'], '\n}\n')
    negative('private_construction', "fn bypass(batch:FixtureProjectedBatch<'_>){\n",
             ' let _=FixtureProjectedBatch{..batch};', 'E0451', ['FixtureProjectedBatch', 'private'])
    negative('private_expectation', "fn bypass(mut batch:FixtureProjectedBatch<'_>,frame:NonZeroU64){\n",
             ' batch.expected.frame=frame;', 'E0616', ['expected', 'private'])
    negative('private_values', "fn bypass<'a>(mut batch:FixtureProjectedBatch<'a>,values:&'a[FixtureProjectedInterval]){\n",
             ' batch.values=values;', 'E0616', ['values', 'private'])
    negative('private_intervals', "fn bypass(mut batch:FixtureProjectedBatch<'_>,values:FixtureBatch){\n",
             ' batch.intervals=values;', 'E0616', ['intervals', 'private'])
    negative('mutable_entries', "fn bypass(batch:FixtureProjectedBatch<'_>){\n",
             ' let _: &mut [FixtureProjectedInterval]=batch.entries();',
             'E0308', ['mutability', 'expected mutable reference', 'found reference'])
    base = 'fn bypass(frame:NonZeroU64,at:MonotonicNs,values:&[FixtureProjectedInterval]){\n'
    for name, expression, tokens in (
        ('raw_frame', ' let _=FixtureProjectedBatch::new(1_u64,frame,at,values);', ['NonZero', 'u64']),
        ('raw_generation', ' let _=FixtureProjectedBatch::new(frame,1_u64,at,values);', ['NonZero', 'u64']),
        ('raw_evaluation_time', ' let _=FixtureProjectedBatch::new(frame,frame,1_u64,values);', ['MonotonicNs', 'u64']),
    ):
        negative(name, base, expression, 'E0308', tokens)
    negative('wrong_clock', 'fn bypass(frame:NonZeroU64,at:ActiveAwakeNs,values:&[FixtureProjectedInterval]){\n',
             ' let _=FixtureProjectedBatch::new(frame,frame,at,values);',
             'E0308', ['MonotonicNs', 'ActiveAwakeNs'])
    negative('raw_interval_entries', 'fn bypass(frame:NonZeroU64,at:MonotonicNs,values:&[FixtureInterval]){\n',
             ' let _=FixtureProjectedBatch::new(frame,frame,at,values);',
             'E0308', ['FixtureProjectedInterval', 'FixtureInterval'])
    negative('raw_minimum', "fn bypass(batch:FixtureProjectedBatch<'_>){\n",
             ' let _=batch.select(1_usize);', 'E0308', ['FixtureMinimum', 'usize'])
    negative('escaped_borrow',
             'fn bypass(value:FixtureProjectedInterval,frame:NonZeroU64,at:MonotonicNs){\n'
             ' let batch;\n {\n  let values=[value];\n',
             '  batch=FixtureProjectedBatch::new(frame,frame,at,&values).unwrap();',
             'E0597', ['values', 'does not live long enough'], '\n }\n let _=batch.entries();\n}\nfn main(){}\n')
    return cases


def runs_contract_examples():
    """One real legal runs consumer and diagnostic-specific external bypasses.

    Native selector attempts are externally counted; 62-fit/2077-copy bounds
    per attempted selection remain source-owned uppers, not inner telemetry.
    """
    imports = ('use time_core::{FitInput,FitPoint,FitError,ResidualError,FixtureRunsError,'
               'FixtureRunsSelection,FixtureRunsStage,FixtureRunsStop,select_fixture_runs};\n'
               'use std::num::NonZeroU64;\n')
    setup = (imports + 'fn main(){\n'
             ' let points=[FitPoint::new(-2.0,0.25,1.0).unwrap(),'
             'FitPoint::new(-1.0,0.25,1.0).unwrap(),FitPoint::new(0.0,0.25,1.0).unwrap()];\n'
             ' let frame=NonZeroU64::new(9).unwrap(); let input=FitInput::new(frame,&points).unwrap();\n')
    legal_setup = (imports + 'struct Calls{points:u8,inputs:u8,selections:u8}\n'
                   'impl Drop for Calls{fn drop(&mut self){eprintln!("CORE_RUNS_PROFILE points={} inputs={} selections={}",self.points,self.inputs,self.selections);}}\n'
                   'fn main(){\n let mut calls=Calls{points:0,inputs:0,selections:0};\n'
                   ' calls.points+=1; let a=FitPoint::new(-2.0,0.25,1.0).unwrap();\n'
                   ' calls.points+=1; let b=FitPoint::new(-1.0,0.25,1.0).unwrap();\n'
                   ' calls.points+=1; let c=FitPoint::new(0.0,0.25,1.0).unwrap();\n'
                   ' let points=[a,b,c]; let frame=NonZeroU64::new(9).unwrap();\n'
                   ' calls.inputs+=1; let input=FitInput::new(frame,&points).unwrap();\n')
    legal = (legal_setup + ' calls.selections+=1; let selection=select_fixture_runs(&input,3).unwrap();\n'
             ' assert_eq!((selection.start(),selection.sample_count(),selection.frame()),(0,3,frame));\n'
             ' assert_eq!(selection.stop(),FixtureRunsStop::RunsPassed);\n'
             ' assert_eq!((selection.decision_start(),selection.decision_count(),selection.decision_runs(),selection.fit_runs()),(0,3,3,3));\n'
             ' let stats=selection.statistics(); assert_eq!((stats.frame(),stats.sample_count(),stats.degrees_of_freedom()),(frame,3,1));\n'
             ' assert_eq!((stats.line().frame(),stats.line().phase_seconds()),(frame,0.25));\n'
             ' assert_eq!(stats.line().phase_rate_seconds_per_second().to_bits(),0);\n'
             ' assert_eq!(stats.weighted_sse_seconds_squared().to_bits(),0);\n'
             ' assert_eq!(stats.residual_scale_seconds_squared().to_bits(),0);\n'
             ' assert_eq!(stats.normalized_data_variance_seconds_squared().to_bits(),0);\n'
             ' calls.selections+=1; let error=select_fixture_runs(&input,2).unwrap_err();\n'
             ' assert_eq!((error.stage(),error.candidate_start(),error.point_index()),(FixtureRunsStage::Minimum,None,None));\n'
             ' assert!(std::error::Error::source(&error).is_none());\n'
             ' assert_eq!((input.frame(),input.sample_count()),(frame,3));\n'
             ' println!("legal-core-contract");\n}\n')
    cases = {'legal_runs_selection_and_error': {'source': legal, 'code': None,
             'calls': {'direct_fit_calls': 0, 'stats_calls': 0, 'point_admissions': 3,
                       'runs_points': 3, 'runs_inputs': 1, 'runs_selections': 2}}}
    success = setup + ' let mut selection=select_fixture_runs(&input,3).unwrap();\n'
    failure = setup + ' let mut error=select_fixture_runs(&input,2).unwrap_err();\n'

    def negative(name, statement, code, tokens, base=success):
        """Place the intended rejected operation on its own source line."""
        cases[name] = {'source': base + ' ' + statement + '\n}\n', 'code': code,
                       'tokens': tokens, 'line': len(base.splitlines()) + 1}

    negative('runs_selection_default', 'let _: FixtureRunsSelection=Default::default();',
             'E0277', ['FixtureRunsSelection', 'Default'])
    negative('runs_selection_private_update', 'let _=FixtureRunsSelection{..selection};',
             'E0451', ['FixtureRunsSelection', 'private'])
    negative('runs_selection_raw_from', 'let _: FixtureRunsSelection=[0_u8;1].into();',
             'E0277', ['FixtureRunsSelection', 'From'])
    fields = {'start':'0', 'sample_count':'0', 'frame':'frame', 'stop':'FixtureRunsStop::RunsPassed',
              'decision_start':'0', 'decision_count':'0', 'decision_runs':'0', 'fit_runs':'0',
              'statistics':'*selection.statistics()'}
    for field, value in fields.items():
        negative('runs_selection_mutate_' + field, 'selection.' + field + '=' + value + ';',
                 'E0616', [field, 'private'])
    negative('runs_statistics_mutate', '*selection.statistics()=*selection.statistics();',
             'E0594', ['cannot assign', 'reference'])
    negative('runs_error_default', 'let _: FixtureRunsError=Default::default();',
             'E0277', ['FixtureRunsError', 'Default'], base=failure)
    negative('runs_error_private_update', 'let _=FixtureRunsError{..error};',
             'E0451', ['FixtureRunsError', 'private'], base=failure)
    negative('runs_error_raw_from', 'let _: FixtureRunsError=[0_u8;1].into();',
             'E0277', ['FixtureRunsError', 'From'], base=failure)
    error_fields = {'stage':'FixtureRunsStage::Fit', 'candidate_start':'Some(0)',
                    'point_index':'Some(0)', 'fit_source':'Some(FitError::CountOutOfRange)',
                    'residual_source':'Some(ResidualError::CountInvariant)'}
    for field, value in error_fields.items():
        negative('runs_error_mutate_' + field, 'error.' + field + '=' + value + ';',
                 'E0616', [field, 'private'], base=failure)
    negative('runs_raw_input', 'let _=select_fixture_runs(&[0.0_f64;3],3);',
             'E0308', ['FitInput', 'f64'], base=setup)
    negative('runs_line_injection', 'let _=select_fixture_runs(&input,3,selection.statistics().line());',
             'E0061', ['argument', 'supplied'])
    for symbol in ('TerminalCandidate', 'RunsObserver', 'select_fixture_runs_observed'):
        name = 'runs_private_' + symbol.lower()
        cases[name] = {'source': 'use time_core::' + symbol + ';\nfn main(){}\n',
                       'code': 'E0432', 'tokens': [symbol, 'unresolved import'], 'line': 1}
    return cases


def residual_contract_examples():
    """Return two real legal consumers and exact residual bypass compiler causes.

    Legal recipes each construct three points and issue one statistics request,
    with no direct fit call. Negative programs are compiled, never executed.
    Statements carry their actual primary source line and named diagnostic cause;
    unrelated compiler/import/setup failure supplies no privacy qualification.
    """
    imports = ("use time_core::{FitPoint,FitInput,FixtureResidualStatistics,FixtureLine,"
               "FitError,ResidualError,ResidualField,ResidualIssue,ResidualStep,residual_statistics};\n"
               "use std::num::NonZeroU64;\n")
    points = (' let points=[FitPoint::new(-2.0,ys[0],1.0).unwrap(),'
              'FitPoint::new(-1.0,ys[1],1.0).unwrap(),'
              'FitPoint::new(0.0,ys[2],1.0).unwrap()];\n'
              ' let frame=NonZeroU64::new(9).unwrap();\n'
              ' let input=FitInput::new(frame,&points).unwrap();\n'
              ' let report=residual_statistics(&input).unwrap();\n')
    suffix = (' eprintln!("CORE_STATS_PROFILE_COUNTS stats=1 points=3");\n'
              ' println!("legal-core-contract");\n}\n')
    constant = (imports + 'fn main(){\n let ys=[0.5,0.5,0.5];\n' + points
                + ' assert_eq!(report.frame(),frame); assert_eq!(report.line().frame(),frame);\n'
                ' assert_eq!(report.sample_count(),3); assert_eq!(report.degrees_of_freedom(),1);\n'
                ' assert_eq!(report.line().phase_seconds(),0.5);\n'
                ' assert_eq!(report.line().phase_rate_seconds_per_second().to_bits(),0);\n'
                ' assert_eq!(report.weighted_sse_seconds_squared().to_bits(),0);\n'
                ' assert_eq!(report.residual_scale_seconds_squared().to_bits(),0);\n'
                ' assert_eq!(report.normalized_data_variance_seconds_squared().to_bits(),0);\n'
                + suffix)
    noisy = (imports + 'fn main(){\n let ys=[-1.0,1.0,1.0];\n' + points
             + ' for value in [report.weighted_sse_seconds_squared(),'
               'report.residual_scale_seconds_squared(),report.normalized_data_variance_seconds_squared()] {\n'
               '  let expected=2.0/3.0; assert!(value>0.0 && (value-expected).abs()<=2_f64.powi(-38)*expected);\n }\n'
               ' let debug=format!("{report:?}"); assert!(debug.len()<512);\n'
               ' assert!(!debug.contains("0.666") && !debug.contains("1.333"));\n'
               ' let errors=[ResidualError::Fit(FitError::CountOutOfRange),\n'
               '  ResidualError::Arithmetic{step:ResidualStep::Weight,source:FitError::CountOutOfRange},\n'
               '  ResidualError::InvalidStatistic{field:ResidualField::WeightedSse,issue:ResidualIssue::Negative},\n'
               '  ResidualError::ValidationInvariant{field:ResidualField::ResidualScale},ResidualError::CountInvariant];\n'
               ' for (index,error) in errors.iter().enumerate(){\n'
               '  assert!(format!("{error}").len()<512); assert!(format!("{error:?}").len()<512);\n'
               '  assert_eq!(std::error::Error::source(error).is_some(),index<2);\n }\n'
             + suffix)
    calls = {"direct_fit_calls": 0, "stats_calls": 1, "point_admissions": 3}
    cases = {"legal_residual_report": {"source": constant, "code": None, "calls": dict(calls)},
             "legal_residual_error_and_privacy": {"source": noisy, "code": None, "calls": dict(calls)}}
    prelude = imports + 'fn main(){\n let ys=[0.5,0.5,0.5];\n' + points

    def negative(name, statement, code, tokens, *, setup=prelude):
        """Place one causal statement on one independently addressable source line."""
        source = setup + ' ' + statement + '\n}\n'
        cases[name] = {"source": source, "code": code, "tokens": tokens,
                       "line": len(setup.splitlines()) + 1}

    negative("residual_default", "let _: FixtureResidualStatistics=Default::default();",
             "E0277", ["FixtureResidualStatistics", "Default"], setup=imports + 'fn main(){\n')
    negative("residual_raw_update", "let _=FixtureResidualStatistics{..report};",
             "E0451", ["FixtureResidualStatistics", "private"])
    negative("residual_unchecked_from", "let _: FixtureResidualStatistics=[0.0_f64;3].into();",
             "E0277", ["FixtureResidualStatistics", "From"], setup=imports + 'fn main(){\n')
    negative("residual_line_mutation", "*report.line()=FixtureLine::clone(report.line());",
             "E0594", ["cannot assign", "reference"])
    negative("residual_scalar_mutable_reference", "let _: &mut f64=report.weighted_sse_seconds_squared();",
             "E0308", ["f64", "mut"])
    negative("residual_output_type_separation", "let _: FixtureLine=report;",
             "E0308", ["FixtureLine", "FixtureResidualStatistics"])
    negative("residual_raw_input", "let _=residual_statistics(&[0.0_f64;3]);",
             "E0308", ["FitInput", "f64"], setup=imports + 'fn main(){\n')
    negative("residual_independent_line_injection", "let _=residual_statistics(&input,report.line());",
             "E0061", ["argument", "supplied"])
    negative("residual_typed_error_source", "let _=ResidualError::Arithmetic{step:ResidualStep::Weight,source:0.0};",
             "E0308", ["FitError", "floating"], setup=imports + 'fn main(){\n')
    return cases


def normalization_contract_examples():
    """Two genuine bounded normalization consumers and eight causal bypasses.

    Legal attempts increment before each public call; one assembled record is
    emitted even on unwind. Their prepared/insufficient internal costs are frozen
    source-owned upper recipes, not observed internal constructor telemetry.
    """
    imports = ('use time_core::{FixtureContext,FixtureObservation,FixtureNormalization,PreparedFixtureInput,'
               'NormalizationError,NormalizationComponent,NormalizationStep,ObservationError,'
               'FitError,FitPoint,prepare_fixture_input};\n'
               'use time_domain::{MonotonicNs,ActiveAwakeNs,UtcCoordinateQ32,PhaseOffsetQ33,DomainError,exchange_arithmetic};\n'
               'use std::num::NonZeroU64;\n')
    helper = '''
#[derive(Default)] struct Calls([u64;9]);
impl Calls {
 fn context(&mut self)->FixtureContext { self.0[2]+=1; FixtureContext::new(NonZeroU64::new(17).unwrap(),NonZeroU64::new(23).unwrap(),NonZeroU64::new(29).unwrap()) }
 fn observation(&mut self,c:FixtureContext,t:u64)->FixtureObservation {
  self.0[3]+=1; let time=MonotonicNs::new(t);
  self.0[4]+=1; let t1=UtcCoordinateQ32::new(0).unwrap();
  self.0[4]+=1; let t2=UtcCoordinateQ32::new(0).unwrap();
  self.0[4]+=1; let t3=UtcCoordinateQ32::new(0).unwrap();
  self.0[4]+=1; let t4=UtcCoordinateQ32::new(0).unwrap();
  self.0[5]+=1; let phase=exchange_arithmetic(t1,t2,t3,t4).unwrap().offset();
  self.0[1]+=1; FixtureObservation::new(c,time,phase,1.0).unwrap()
 }
}
impl Drop for Calls {
 fn drop(&mut self) { use std::io::Write; let c=self.0;
  let record=format!("CORE_NORMALIZATION_PROFILE_COUNTS normalizations={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} stats={} points={}\\n",c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8]);
  std::io::stderr().lock().write_all(record.as_bytes()).expect("complete normalization consumer counter sink");
 }
}
'''
    prepared = imports + helper + '''fn main(){
 let mut calls=Calls::default(); let context=calls.context();
 let rows=[calls.observation(context,0),calls.observation(context,62499999),calls.observation(context,62500000)];
 calls.0[0]+=1; let FixtureNormalization::Prepared(prepared)=prepare_fixture_input(context,&rows).unwrap() else {panic!("prepared legal neighbor")};
 assert_eq!(prepared.context(),context); assert_eq!(prepared.sample_count(),3); assert_eq!(prepared.input().frame(),context.frame());
 assert_eq!(prepared.reference().nanoseconds(),62500000);
 for (index,expected) in [0xbfb0000000000000_u64,0xbe112e0be826d695,0].into_iter().enumerate(){
  calls.0[6]+=1; let point=prepared.point(index).unwrap(); assert_eq!(point.relative_seconds().to_bits(),expected);
  assert_eq!(point.phase_seconds().to_bits(),0); assert_eq!(point.variance_divisor().to_bits(),0x3ff0000000000000);
 }
 calls.0[6]+=1; assert!(prepared.point(3).is_none()); calls.0[6]+=1; assert!(prepared.point(usize::MAX).is_none());
 println!("legal-core-contract");
}
'''
    insufficient = imports + helper + '''fn main(){
 let mut calls=Calls::default(); let context=calls.context();
 let rows=[calls.observation(context,0),calls.observation(context,1)];
 calls.0[0]+=1; let FixtureNormalization::Insufficient(value)=prepare_fixture_input(context,&rows).unwrap() else {panic!("insufficient legal neighbor")};
 assert_eq!(value.context(),context); assert_eq!(value.sample_count(),2); assert_eq!(value.reference().unwrap().nanoseconds(),1);
 let errors=[NormalizationError::ContextMismatch{index:usize::MAX},NormalizationError::NonIncreasingTime{index:usize::MAX},
  NormalizationError::Domain{index:usize::MAX,source:DomainError::DurationOutOfOrder},
  NormalizationError::Point{index:usize::MAX,source:FitError::NonIncreasingTime{index:usize::MAX}},
  NormalizationError::Input(FitError::CountOutOfRange),
  NormalizationError::Arithmetic{component:NormalizationComponent::Phase,step:NormalizationStep::SecondsDivide,source:FitError::ValidationInvariant}];
 for (index,error) in errors.iter().enumerate(){ assert!(format!("{error}").len()<512); assert_eq!(std::error::Error::source(error).is_some(),index>=2); }
 for error in [ObservationError::PhaseOutOfRange,ObservationError::ValidationInvariant]{assert!(format!("{error}").len()<256);}
 assert!(format!("{value:?}").len()<512); println!("legal-core-contract");
}
'''
    def calls(observations, getters):
        """Literal legal call recipe; no product code is read to derive truth."""
        return {'direct_fit_calls': 0, 'stats_calls': 0, 'point_admissions': 0,
                'normalization_requests': 1, 'observation_constructor_attempts': observations,
                'context_constructions': 1, 'monotonic_constructor_attempts': observations,
                'utc_coordinate_constructor_attempts': 4 * observations,
                'exchange_arithmetic_calls': observations, 'point_getter_requests': getters}
    cases = {
        'legal_normalization_prepared': {'source': prepared, 'code': None, 'calls': calls(3, 5),
            'internal_upper': {'duration_operations_upper': 3, 'generated_point_admissions_upper': 3, 'fit_input_constructions_upper': 1}},
        'legal_normalization_insufficient_and_errors': {'source': insufficient, 'code': None, 'calls': calls(2, 0),
            'internal_upper': {'duration_operations_upper': 2, 'generated_point_admissions_upper': 0, 'fit_input_constructions_upper': 0}}
    }
    setup = imports + '''fn main(){
 let context=FixtureContext::new(NonZeroU64::MIN,NonZeroU64::MIN,NonZeroU64::MIN);
 let zero=UtcCoordinateQ32::new(0).unwrap(); let phase=exchange_arithmetic(zero,zero,zero,zero).unwrap().offset();
 let obs=FixtureObservation::new(context,MonotonicNs::new(0),phase,1.0).unwrap();
'''
    def negative(name, statement, code, tokens, prefix=setup):
        """Place one intended compiler cause on its exact primary source line."""
        cases[name] = {'source': prefix + ' ' + statement + '\n}\n', 'code': code,
                       'tokens': tokens, 'line': len(prefix.splitlines()) + 1}
    negative('norm_context_default', 'let _:FixtureContext=Default::default();', 'E0277', ['FixtureContext','Default'], imports+'fn main(){\n')
    negative('norm_observation_raw_update', 'let _=FixtureObservation{..obs};', 'E0451', ['FixtureObservation','private'])
    negative('norm_utc_substitution', 'let _=FixtureObservation::new(context,zero,phase,1.0);', 'E0308', ['MonotonicNs','UtcCoordinateQ32'])
    negative('norm_awake_substitution', 'let _=FixtureObservation::new(context,ActiveAwakeNs::new(0),phase,1.0);', 'E0308', ['MonotonicNs','ActiveAwakeNs'])
    negative('norm_raw_q33_constructor', 'let _=PhaseOffsetQ33{numerator:0};', 'E0451', ['PhaseOffsetQ33','private'])
    negative('norm_unchecked_factory', 'let _:PreparedFixtureInput=[0.0_f64;3].into();', 'E0277', ['PreparedFixtureInput','From'], imports+'fn main(){\n')
    prepared_setup = setup + ' let make=|t|FixtureObservation::new(context,MonotonicNs::new(t),phase,1.0).unwrap();\n let rows=[make(0),make(1000000000),make(2000000000)];\n let FixtureNormalization::Prepared(prepared)=prepare_fixture_input(context,&rows).unwrap() else{return};\n'
    negative('norm_prepared_raw_update', 'let _=PreparedFixtureInput{..prepared};', 'E0451', ['PreparedFixtureInput','private'], prepared_setup)
    negative('norm_immutable_point', '*prepared.point(0).unwrap()=FitPoint::new(-1.0,0.0,1.0).unwrap();', 'E0594', ['cannot assign','reference'], prepared_setup)
    return cases


def history_contract_examples():
    """Two real history callers and eight intended primary compiler rejections.

    Recipes count before every named public call. The one complete Drop record
    includes external history/domain/getter attempts only. Same-input nested
    normalizer/statistics work is a separate literal-source upper, not telemetry.
    Negative statements reuse a compiling actual-history neighbor; no source
    is executed by this recipe factory, and no invalid domain value is forged.
    """
    imports = ('use time_core::{FixtureContext,FixtureObservation,FixtureHistory,FixtureHistoryReport,'
               'FixtureResidualStatistics,HistoryError,NormalizationError,ResidualError,FitError};\n'
               'use time_domain::{MonotonicNs,UtcCoordinateQ32,exchange_arithmetic};\n'
               'use std::num::NonZeroU64;\n')
    support = '''
#[derive(Default)] struct Calls([u64;12]);
macro_rules! get { ($calls:expr,$value:expr) => {{ $calls.0[7]+=1; $value }} }
impl Calls {
 fn context(&mut self)->FixtureContext {
  self.0[3]+=1;
  FixtureContext::new(NonZeroU64::new(17).unwrap(),NonZeroU64::new(23).unwrap(),NonZeroU64::new(29).unwrap())
 }
 fn history(&mut self,c:FixtureContext)->FixtureHistory { self.0[0]+=1; FixtureHistory::new(c) }
 fn observation(&mut self,c:FixtureContext,t:u64)->FixtureObservation {
  self.0[4]+=1; let time=MonotonicNs::new(t);
  self.0[5]+=1; let t1=UtcCoordinateQ32::new(0).unwrap();
  self.0[5]+=1; let t2=UtcCoordinateQ32::new(0).unwrap();
  self.0[5]+=1; let t3=UtcCoordinateQ32::new(0).unwrap();
  self.0[5]+=1; let t4=UtcCoordinateQ32::new(0).unwrap();
  self.0[6]+=1; let exchange=exchange_arithmetic(t1,t2,t3,t4).unwrap();
  let phase=get!(self,exchange.offset());
  self.0[2]+=1; FixtureObservation::new(c,time,phase,1.0).unwrap()
 }
 fn append(&mut self,h:&mut FixtureHistory,o:FixtureObservation) {
  self.0[1]+=1; h.try_append(o).unwrap();
 }
}
impl Drop for Calls {
 fn drop(&mut self) {
  use std::io::Write;
  let row=format!("CORE_HISTORY_PROFILE_COUNTS histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} normalizations={} stats={} fits={} points={}\n",
   self.0[0],self.0[1],self.0[2],self.0[3],self.0[4],self.0[5],self.0[6],self.0[7],self.0[8],self.0[9],self.0[10],self.0[11]);
  std::io::stderr().lock().write_all(row.as_bytes()).expect("external history counter sink");
 }
}
// Inspect only independently supplied two/three-row prefixes, with no product
// transition or numeric reconstruction. None neighbors accept every usize safely.
fn rows(calls:&mut Calls,h:&FixtureHistory,c:FixtureContext,times:&[u64]) {
 assert!(times.len()<=3);
 for (index,expected) in times.iter().enumerate() {
  let row=get!(calls,h.observation(index)).unwrap();
  assert_eq!(get!(calls,row.context()),c);
  let t=get!(calls,row.observed_at()); assert_eq!(get!(calls,t.nanoseconds()),*expected);
  let p=get!(calls,row.phase()); assert_eq!(get!(calls,p.numerator()),0);
  assert_eq!(get!(calls,row.variance_divisor()).to_bits(),1.0_f64.to_bits());
 }
 assert!(get!(calls,h.observation(times.len())).is_none());
 assert!(get!(calls,h.observation(usize::MAX)).is_none());
}
'''
    def prelude(times):
        """Generate a fixed literal setup; the real calls occur only in root-run Rust."""
        return imports + support + 'fn main(){\n let mut calls=Calls::default(); let c=calls.context(); let mut history=calls.history(c);\n let times=' + times + ';\n for t in times { let o=calls.observation(c,t); calls.append(&mut history,o); }\n'

    prepared_setup = prelude('[0_u64,1_000_000_000,2_000_000_000]')
    prepared = prepared_setup + '''
 assert_eq!(get!(calls,history.context()),c);
 assert_eq!(get!(calls,history.sample_count()),3);
 let reference=get!(calls,history.reference()).unwrap();
 assert_eq!(get!(calls,reference.nanoseconds()),2_000_000_000);
 rows(&mut calls,&history,c,&times);
 let report=get!(calls,history.report()).unwrap();
 assert_eq!(get!(calls,report.context()),c);
 let report_reference=get!(calls,report.reference());
 assert_eq!(get!(calls,report_reference.nanoseconds()),2_000_000_000);
 let summary=get!(calls,report.statistics());
 assert_eq!(get!(calls,summary.frame()),NonZeroU64::new(23).unwrap());
 assert_eq!(get!(calls,summary.sample_count()),3);
 assert_eq!(get!(calls,summary.degrees_of_freedom()),1);
 assert_eq!(get!(calls,summary.weighted_sse_seconds_squared()).to_bits(),0);
 assert_eq!(get!(calls,summary.residual_scale_seconds_squared()).to_bits(),0);
 assert_eq!(get!(calls,summary.normalized_data_variance_seconds_squared()).to_bits(),0);
 let line=get!(calls,summary.line());
 assert_eq!(get!(calls,line.frame()),NonZeroU64::new(23).unwrap());
 assert_eq!(get!(calls,line.phase_seconds()).to_bits(),0);
 assert_eq!(get!(calls,line.phase_rate_seconds_per_second()).to_bits(),0);
 println!("legal-core-contract");
}
'''
    insufficient = prelude('[0_u64,1_000_000_000]') + '''
 assert_eq!(get!(calls,history.context()),c);
 assert_eq!(get!(calls,history.sample_count()),2);
 let reference=get!(calls,history.reference()).unwrap();
 assert_eq!(get!(calls,reference.nanoseconds()),1_000_000_000);
 rows(&mut calls,&history,c,&times);
 assert!(get!(calls,history.report()).is_none());
 let errors=[HistoryError::ContextMismatch,HistoryError::DuplicateTime,HistoryError::OutOfOrderTime,
  HistoryError::Normalization{source:NormalizationError::CountOutOfRange},
  HistoryError::Statistics{source:ResidualError::Fit(FitError::CountOutOfRange)}];
 for (index,error) in errors.iter().enumerate() {
  assert!(format!("{error}").len()<512); assert!(format!("{error:?}").len()<512);
  assert_eq!(std::error::Error::source(error).is_some(),index>=3);
 }
 println!("legal-core-contract");
}
'''
    def calls(count, getters):
        """Freeze literal actual external attempts; no inner owner is observed."""
        return {'direct_fit_calls': 0, 'stats_calls': 0, 'point_admissions': 0,
                'history_constructions': 1, 'history_append_requests': count,
                'history_observation_constructor_attempts': count, 'history_context_constructions': 1,
                'history_monotonic_constructor_attempts': count,
                'history_utc_coordinate_constructor_attempts': 4*count,
                'history_exchange_arithmetic_calls': count, 'history_product_getter_requests': getters,
                'history_direct_normalization_requests': 0, 'history_direct_stats_calls': 0,
                'history_direct_fit_calls': 0, 'history_direct_point_admissions': 0}

    cases = {
        'legal_history_prepared': {'source': prepared, 'code': None, 'calls': calls(3,45),
            'internal_upper': {'history_normalization_entries_upper': 3, 'history_statistics_entries_upper': 1,
                'history_statistics_inner_fit_upper': 1, 'history_duration_operations_upper': 6,
                'history_generated_point_admissions_upper': 3, 'history_fit_input_constructions_upper': 1,
                'history_staging_Copy_assignments_upper': 195}},
        'legal_history_insufficient_and_errors': {'source': insufficient, 'code': None, 'calls': calls(2,23),
            'internal_upper': {'history_normalization_entries_upper': 2, 'history_statistics_entries_upper': 0,
                'history_statistics_inner_fit_upper': 0, 'history_duration_operations_upper': 3,
                'history_generated_point_admissions_upper': 0, 'history_fit_input_constructions_upper': 0,
                'history_staging_Copy_assignments_upper': 129}},
    }

    def negative(name, statement, code, tokens, setup=prepared_setup):
        """Address one causal primary statement on one fixed source line."""
        cases[name] = {'source': setup + ' ' + statement + '\n}\n', 'code': code,
                       'tokens': tokens, 'line': len(setup.splitlines())+1}

    negative('history_private_update', 'let _=FixtureHistory{..history};',
             'E0451', ['FixtureHistory','private'])
    negative('history_report_private_construction',
             'let report=history.report().unwrap(); let _=FixtureHistoryReport{context:c,reference:MonotonicNs::new(0),statistics:report.statistics().clone()};',
             'E0451', ['FixtureHistoryReport','private'])
    negative('history_default', 'let _: FixtureHistory=Default::default();',
             'E0277', ['FixtureHistory','Default'])
    negative('history_immutable_observation', 'let _: &mut FixtureObservation=history.observation(0).unwrap();',
             'E0308', ['FixtureObservation','mut'])
    negative('history_immutable_statistics', 'let _: &mut FixtureResidualStatistics=history.report().unwrap().statistics();',
             'E0308', ['FixtureResidualStatistics','mut'])
    negative('history_context_mutation', 'history.context=c;',
             'E0616', ['context','private'])
    negative('history_append_borrow', 'let held=history.observation(0).unwrap(); history.try_append(calls.observation(c,3_000_000_000)).unwrap(); let _=held.observed_at();',
             'E0502', ['mutable','immutable'])
    negative('history_raw_append', 'let _=history.try_append(0_u64);',
             'E0308', ['FixtureObservation','u64'])
    return cases


def encode_case(points):
    """Preserve raw hostile f64 bit patterns in a test-only packed representation."""
    if len(points) > 64:
        raise Incomplete("Test corpus exceeds 64 point cells.")
    return bytes([len(points)]) + b"".join(struct.pack(">ddd", *point) for point in points)


def encode_bits_case(points):
    """Pack at most64 exact u64 x/y/d triples without changing signed-zero bits."""
    if len(points) > 64 or any(len(row) != 3 or any(type(value) is not int or not 0 <= value < 2 ** 64
                                                  for value in row) for row in points):
        raise Incomplete("Malformed or oversized raw-bit test corpus.")
    return bytes([len(points)]) + b"".join(struct.pack(">QQQ", *point) for point in points)


def rational_input(value):
    """Convert a frozen input rational to one binary64 seed, never an output oracle."""
    parts = value.split("/")
    if len(parts) not in (1, 2):
        raise Incomplete("Frozen input rational shape changed.")
    return int(parts[0]) / (int(parts[1]) if len(parts) == 2 else 1)


REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"


def registry_blocks(text):
    """Retain original reviewed registry blocks with no dependency-edge rewriting."""
    result = {}
    for raw in text.split("[[package]]")[1:]:
        block = "[[package]]\n" + raw.strip() + "\n"
        item = tomllib.loads(block)["package"][0]
        if "source" not in item:
            continue
        name = item["name"]
        if name in result or item.get("source") != REGISTRY:
            raise Incomplete("Ambiguous or unreviewed locked registry identity.")
        result[name] = (item, block)
    return result



def reviewed_fuzz_lock(product_text, control_text):
    """Preserve Garde's reviewed closure and all original libFuzzer registry blocks."""
    product = registry_blocks(product_text)
    original = registry_blocks(control_text)
    local = [item for item in tomllib.loads(product_text)["package"]
             if item["name"] in {"time-core", "time-domain"}]
    expected_local = {"time-core": ["garde", "time-domain"], "time-domain": ["garde"]}
    if (len(local) != 2 or {item["name"] for item in local} != set(expected_local)
            or any(item.get("source") or item.get("version") != "0.0.0"
                   or item.get("dependencies") != expected_local[item["name"]] for item in local)):
        raise Incomplete("The actual checked core/domain local graph is absent or changed.")
    selected, pending = {}, ["garde"]
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        if name not in product:
            raise Incomplete("Garde locked dependency closure is incomplete.")
        selected[name] = product[name]
        pending.extend(value.split(" ")[0] for value in product[name][0].get("dependencies", []))
    for name, entry in original.items():
        if name in selected and selected[name][0] != entry[0]:
            raise Incomplete("Shared product/fuzz registry identity or dependency edges disagree.")
        selected.setdefault(name, entry)
    blocks = {name: entry[1] for name, entry in selected.items()}
    # These are actual new local identities, not a renamed control package.
    blocks["time-core"] = '[[package]]\nname = "time-core"\nversion = "0.0.0"\ndependencies = ["garde", "time-domain"]\n'
    blocks["time-domain"] = '[[package]]\nname = "time-domain"\nversion = "0.0.0"\ndependencies = ["garde"]\n'
    blocks["time-core-fuzz"] = ('[[package]]\nname = "time-core-fuzz"\nversion = "0.0.0"\n'
                                  'dependencies = ["libfuzzer-sys", "time-core", "time-domain"]\n')
    return "version = 4\n\n" + "\n".join(blocks[name] for name in sorted(blocks))



def fuzz_metadata(metadata, selected):
    """Reject wrong identities, feature sets and complete reachable dependency edges."""
    packages = {item["id"]: item for item in metadata["packages"]}
    nodes = {item["id"]: item for item in metadata["resolve"]["nodes"]}
    if (len(packages) != len(metadata["packages"]) or len(nodes) != len(metadata["resolve"]["nodes"])
            or len({item["name"] for item in packages.values()}) != len(packages)):
        raise Incomplete("Duplicate fuzz graph identity.")
    locked = {item["name"]: item for item in selected["locked_registry_packages"]}
    if len(locked) != len(selected["locked_registry_packages"]):
        raise Incomplete("Duplicate locked registry admission.")
    local = {"time-core", "time-domain", "time-core-fuzz"}
    for item in packages.values():
        expected = locked.get(item["name"])
        if expected:
            if item.get("source") != expected["source"] or item["version"] != expected["version"]:
                raise Incomplete("Fuzz metadata package source/version changed.")
        elif item["name"] not in local or item.get("source") is not None or item["version"] != "0.0.0":
            raise Incomplete("Unreviewed fuzz metadata package.")
    root_id = metadata["resolve"]["root"]
    if root_id not in packages or packages[root_id]["name"] != "time-core-fuzz":
        raise Incomplete("Fuzz metadata identifies a different root.")
    reachable, pending = set(), [root_id]
    while pending:
        key = pending.pop()
        if key in reachable:
            continue
        if key not in packages or key not in nodes:
            raise Incomplete("Unresolved reachable fuzz dependency.")
        reachable.add(key)
        pending.extend(item["pkg"] for item in nodes[key]["deps"])
    features = {packages[key]["name"]: sorted(nodes[key]["features"]) for key in reachable}
    edges = {packages[key]["name"]: sorted(packages[edge["pkg"]]["name"] for edge in nodes[key]["deps"])
             for key in reachable}
    expected_features = {**selected["gnu_reachable_features"], "time-core": [], "time-domain": [], "time-core-fuzz": []}
    if len(features) != len(reachable) or features != expected_features or edges != selected["gnu_dependency_edges"]:
        raise Incomplete("Exact GNU fuzz features or dependency edges differ from admission.")
    return {"target": ANALYSIS.PLATFORM, "reachable_features": features, "dependency_edges": edges}



def copy_contract_dependencies(session, artifact, domain_artifact):
    """Copy real owned artifacts into independent read-only hash-sealed inodes.

    Cargo may hardlink its own target root/deps outputs. Do not reject that source
    layout: require canonical owned paths, source hashes before/after, and copies
    with nlink1 and distinct device/inode identities. No writable alias is sealed.
    """
    destination = session.work / "contract-deps"
    destination.mkdir(mode=0o700)
    libraries = {'libtime_core.rlib': artifact, 'libtime_domain.rlib': domain_artifact}
    sources = {}
    for name, emitted in libraries.items():
        source = session.work / emitted.removeprefix('/work/')
        actual = source.resolve(strict=True)
        if (actual != source or not actual.is_file()
                or not source.is_relative_to(session.work / 'target')):
            raise Incomplete("Actual emitted core/domain library has redirected or escaped.")
        sources[name] = source
    dependencies = session.work / "target/debug/deps"
    for item in sorted(dependencies.iterdir()):
        if item.suffix not in {".rlib", ".rmeta", ".so"}:
            continue
        if item.resolve(strict=True) != item:
            raise Incomplete("Contract dependency is redirected.")
        digest = PRODUCT.sha256(item)
        target = destination / item.name
        shutil.copyfile(item, target); target.chmod(0o400)
        if (PRODUCT.sha256(target) != digest or PRODUCT.sha256(item) != digest or target.stat().st_nlink != 1
                or (target.stat().st_dev,target.stat().st_ino)==(item.stat().st_dev,item.stat().st_ino)):
            raise Incomplete("Contract dependency changed while its immutable copy was prepared.")
    for name, source in sources.items():
        library = destination / name
        digest = PRODUCT.sha256(source)
        shutil.copyfile(source, library); library.chmod(0o400)
        if (PRODUCT.sha256(library) != digest or PRODUCT.sha256(source) != digest
                or library.stat().st_nlink != 1
                or (library.stat().st_dev,library.stat().st_ino)==(source.stat().st_dev,source.stat().st_ino)):
            raise Incomplete("The copied core/domain library differs from Cargo's actual artifact.")
    session.protect(destination)
    return destination



def contracts(session, policy, report):
    """Build the real dependency-bearing library, then inspect external compiler causes."""
    deadline = time.monotonic() + policy["contracts_wall_seconds"]
    run = session.run
    output = run("contract-library", [str(session.stable / "bin/cargo"), "build", "--package", "time-core",
                 "--frozen", "--jobs", str(policy["jobs"]), "--lib", "--message-format=json"], COMMON.remaining_timeout(deadline, 120))
    artifact = COMMON.library_artifact(output, "time-core", "/work/source/crates/time-core/Cargo.toml")
    domain_artifact = COMMON.library_artifact(output, "time-domain", "/work/source/crates/time-domain/Cargo.toml")
    dependency_copy = copy_contract_dependencies(session, artifact, domain_artifact)
    consumers = session.work / "contract-inputs"; consumers.mkdir(mode=0o700)
    (session.work / "contracts").mkdir(mode=0o700)
    examples = contract_examples()
    if (sum(item["code"] is None for item in examples.values()) != EXTERNAL_CONSUMER_LIMITS["legal_executions"]
            or sum(item["code"] is not None for item in examples.values()) != EXTERNAL_CONSUMER_LIMITS["negative_compilations"]):
        raise Incomplete("External core consumer count differs from its reviewed complete registry.")
    for name, specimen in examples.items():
        (consumers / (name + ".rs")).write_text(specimen["source"])
    session.protect(consumers)
    for name, specimen in examples.items():
        command = [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", name,
                   "--error-format=json", "/work/contract-inputs/" + name + ".rs",
                   "--extern", "time_core=/work/contract-deps/libtime_core.rlib",
                   "--extern", "time_domain=/work/contract-deps/libtime_domain.rlib",
                   "-L", "dependency=/work/contract-deps", "--out-dir", "/work/contracts"]
        run("external-consumer-" + name, command, COMMON.remaining_timeout(deadline, 60),
            sealed=[consumers, dependency_copy], specimen=specimen if specimen["code"] else None)
        if specimen["code"] is None:
            if 'principal_calls' in specimen:
                expected = specimen['principal_calls']
                work = principal_source_work(expected)
                combination_expected = combination_legal_calls()
                if not exact(specimen.get('combination_calls'), combination_expected):
                    raise Incomplete('The legal consumer omitted its exact combination source delta.')
                combination_work = combination_source_work(combination_expected)
                charged = {key: work[key] + combination_work[key] for key in GLOBAL_LIMITS}
                record_counts(report, name, {
                    **charged,
                    'principal_external_reserved_calls': expected,
                    'principal_external_reserved_source_work_uppers': work,
                    'combination_external_reserved_calls': combination_expected,
                    'combination_external_reserved_source_work_uppers': combination_work,
                    'observation_pending': True,
                    'execution': 'One checked principal producer plus a singleton combination delta reserved before execution.'})
                reserved = report['core_invocations'][-1]
                text = run('external-legal-execution', ['/work/contracts/' + name],
                           COMMON.remaining_timeout(deadline, 30), sealed=[consumers, dependency_copy])
                rows = counter_rows(text, 'CORE_PRINCIPAL_CONTRACT_PROFILE', PRINCIPAL_FIELDS17, require_newline=True)
                if ('legal-core-contract' not in text or len(rows) != 1
                        or tuple(expected[field] for field in PRINCIPAL_FIELDS17) != rows[0]):
                    raise Incomplete('The genuine principal legal consumer omitted its exact attempted-call recipe.')
                combination_observed = combination_contract_counts(text)
                reserved['observed_counts'] = {
                    **charged,
                    'principal_recorded_call_subtotals': dict(expected),
                    'principal_observed_calls': dict(expected),
                    'principal_observation_pending': False,
                    'principal_source_owned_work_uppers': work,
                    'combination_recorded_call_subtotals': combination_observed,
                    'combination_observed_calls': dict(combination_observed),
                    'combination_observation_pending': False,
                    'combination_source_owned_work_uppers': combination_work}
                reserved['observation_pending'] = False
                refresh_accounting(report)
                continue
            selection_reserved = None
            if 'selection_calls' in specimen:
                record_counts(report, name, {
                    'charged_fit_work_upper': 0, 'point_admissions_upper': 0,
                    **selection_reservation(external=specimen['selection_calls']),
                    'observation_pending': True,
                    'execution': 'Legal selection consumer source calls reserved before execution.'})
                selection_reserved = report['core_invocations'][-1]
            text = run("external-legal-execution", ["/work/contracts/" + name], COMMON.remaining_timeout(deadline, 30),
                       sealed=[consumers, dependency_copy])
            if "legal-core-contract" not in text:
                raise Incomplete("Legal external consumer did not reach every selected representation.")
            counts = profile_counts(text)
            expected_calls = specimen.get("calls")
            if (not isinstance(expected_calls, dict)
                    or any(counts.get(key) != value for key, value in expected_calls.items())):
                raise Incomplete("Legal external consumer reached an unexpected literal call recipe.")
            if ('selection_calls' in specimen and (
                    counts['selection_external_consumer_records'] != 1
                    or counts['selection_external_consumer_calls'] != specimen['selection_calls']
                    or counts['selection_external_records'] or counts['selection_guard_records']
                    or counts['selection_helper_records'])):
                raise Incomplete('Legal selection consumer omitted its exact standalone source recipe.')
            if counts['direct_fit_calls'] + counts['stats_calls'] > 1:
                raise Incomplete("Legal consumer exceeds its one charged fit/stat recipe.")
            if counts['normalization_requests']:
                internal = specimen['internal_upper']
                if (counts['normalization_requests'] > 1 or counts['observation_constructor_attempts'] > 3
                        or counts['monotonic_constructor_attempts'] > 3
                        or counts['exchange_arithmetic_calls'] > 3
                        or counts['utc_coordinate_constructor_attempts'] > 12
                        or counts['context_constructions'] > 1 or counts['point_getter_requests'] > 5):
                    raise Incomplete("Legal normalization consumer exceeds its frozen literal recipe.")
                counts.update(internal)
                counts['point_admissions_upper'] = counts['point_admissions'] + internal['generated_point_admissions_upper']
            if counts['history_append_requests']:
                if (counts['history_constructions'] != 1 or counts['history_append_requests'] > 3
                        or counts['history_observation_constructor_attempts'] > 3
                        or counts['history_context_constructions'] > 1
                        or counts['history_monotonic_constructor_attempts'] > 3
                        or counts['history_utc_coordinate_constructor_attempts'] > 12
                        or counts['history_exchange_arithmetic_calls'] > 3
                        or counts['history_product_getter_requests'] > 128):
                    raise Incomplete('Legal history consumer exceeds its frozen external recipe.')
                internal = specimen['internal_upper']
                counts.update(internal)
                calls_upper = internal['history_statistics_entries_upper']
                counts['charged_fit_work_upper'] = counts['observed_charged_fit_work'] + calls_upper
                counts['internal_fit_upper_bound'] = counts['stats_calls'] + calls_upper
                counts['duration_operations_upper'] = internal['history_duration_operations_upper']
                counts['generated_point_admissions_upper'] = internal['history_generated_point_admissions_upper']
                counts['fit_input_constructions_upper'] = internal['history_fit_input_constructions_upper']
                counts['point_admissions_upper'] = counts['point_admissions'] + internal['history_generated_point_admissions_upper']
            if counts['runs_selections']:
                if (name != 'legal_runs_selection_and_error'
                        or counts['runs_selections'] != 2 or counts['runs_inputs'] != 1
                        or counts['runs_points'] != 3 or counts['runs_external_records'] != 1
                        or counts['runs_sign_records'] or counts['runs_guard_records']
                        or counts['direct_fit_calls'] or counts['stats_calls']
                        or counts['charged_fit_work_upper'] > 124
                        or counts['point_admissions_upper'] > 4157):
                    raise Incomplete('Legal runs consumer exceeds its exact public recipe or conservative work.')
            else:
                if counts['charged_fit_work_upper'] > 3:
                    raise Incomplete('Legal consumer exceeds three combined direct/derived charged calls.')
                if counts['point_admissions_upper'] > 3:
                    raise Incomplete("Legal consumer exceeds three combined direct/generated points.")
            if selection_reserved is None:
                record_counts(report, name, counts)
            else:
                selection_reserved['observed_counts'] = counts
                selection_reserved['observation_pending'] = False
                refresh_accounting(report)
    report["contracts"] = {"legal_consumers_compiled_and_executed": EXTERNAL_CONSUMER_LIMITS["legal_executions"], "library_artifact": artifact,
                           "negative_consumers_compiled_and_rejected": EXTERNAL_CONSUMER_LIMITS["negative_compilations"],
                           "literal_principal_legal_calls": principal_legal_calls(),
                           "literal_combination_legal_calls": combination_legal_calls(),
                           "domain_library_artifact": domain_artifact,
                           "immutable_dependencies_sha256": PRODUCT.inventory(dependency_copy),
                           "literal_legal_calls": {
                               key: sum(item["calls"][key] for item in examples.values() if item["code"] is None)
                               for key in ("direct_fit_calls", "stats_calls", "point_admissions")},
                           "literal_runs_legal_calls": {
                               key: sum(item['calls'].get(key, 0) for item in examples.values() if item['code'] is None)
                               for key in ('runs_points', 'runs_inputs', 'runs_selections')},
                           "literal_normalization_legal_calls": {
                               key: sum(item['calls'].get(key,0) for item in examples.values() if item['code'] is None)
                               for key in ('normalization_requests','observation_constructor_attempts','context_constructions',
                                           'monotonic_constructor_attempts','utc_coordinate_constructor_attempts',
                                           'exchange_arithmetic_calls','point_getter_requests')},
                           "literal_legal_internal_upper": {
                               key: sum(item.get('internal_upper',{}).get(key,0) for item in examples.values() if item['code'] is None)
                               for key in ('duration_operations_upper','generated_point_admissions_upper','fit_input_constructions_upper')},
                           "literal_history_legal_calls": {
                               key: sum(item['calls'].get(key,0) for item in examples.values() if item['code'] is None)
                               for key in ('history_constructions','history_append_requests','history_observation_constructor_attempts',
                                           'history_context_constructions','history_monotonic_constructor_attempts',
                                           'history_utc_coordinate_constructor_attempts','history_exchange_arithmetic_calls',
                                           'history_product_getter_requests')},
                           "literal_history_internal_upper": {
                               key: sum(item.get('internal_upper',{}).get(key,0) for item in examples.values() if item['code'] is None)
                               for key in ('history_normalization_entries_upper','history_statistics_entries_upper',
                                           'history_duration_operations_upper','history_generated_point_admissions_upper',
                                           'history_fit_input_constructions_upper')},
                           "required_diagnostics": {name: {key: item[key] for key in ("code", "tokens", "line")}
                                                    for name, item in examples.items() if item["code"]},
                           "external_legal_neighbor_sources": {
                               name: item['external_legal_neighbor'] for name, item in examples.items()
                               if 'external_legal_neighbor' in item},
                           "external_legal_neighbor_scope":
                               'Paired external package-test sources; this contract receipt alone establishes no execution of those neighbors.'}



def admit_fuzz_sources(session, selected):
    """Freshly admit the existing source cohort, then exact new local lock identities.

    The domain helper validates shared registry/native source bytes and grants;
    its historical numerical results/equivalences are not reused or rebound.
    """
    cohort = dict(selected, lockfile="policy/time-domain-fuzz.lock")
    try:
        original = DOMAIN.admit_fuzz_sources(session, cohort)
    except (DOMAIN.Incomplete, DOMAIN.PRODUCT.ENV.Incomplete) as error:
        raise Incomplete(str(error)) from error
    control = session.base / "common/controls-source/fuzz/Cargo.lock"
    actual = reviewed_fuzz_lock((ROOT / "Cargo.lock").read_text(), control.read_text())
    if actual.encode() != (ROOT / selected["lockfile"]).read_bytes():
        raise Incomplete("Core standalone lock does not preserve exact reviewed registry blocks.")
    return original


def fuzz(session, selected, report):
    """Admit before building; reuse one target cache and seal distinct ASan binaries."""
    original = admit_fuzz_sources(session, selected)
    project = session.work / "fuzz"; project.mkdir(mode=0o700)
    for origin, destination in [(selected["manifest"], "Cargo.toml"), (selected["lockfile"], "Cargo.lock"),
                                (selected["target_source"], "target.rs")]:
        shutil.copyfile(ROOT / origin, project / destination)
    vendor = session.work / "fuzz-vendor"; vendor.mkdir(mode=0o700)
    old = {item["name"]: item for item in original["locked_registry_packages"]}
    for item in selected["locked_registry_packages"]:
        name = item["name"] + "-" + item["version"]
        destination = vendor / name
        if item["name"] in old:
            source = next((session.base / "common/cargo/registry/src").glob("*/" + name))
            archive = next((session.base / "common/cargo/registry/cache").glob("*/" + name + ".crate"))
            COMMON.prepare_fuzz_vendor(source, archive, old[item["name"]], destination)
        else:
            source = session.vendor / name
            shutil.copytree(source, destination, copy_function=shutil.copyfile)
            if PRODUCT.inventory(source) != PRODUCT.inventory(destination):
                raise Incomplete("Reviewed product dependency changed while copied for fuzzing.")
            copied = PRODUCT.inventory(destination)
            checksum = json.loads((destination / ".cargo-checksum.json").read_text())
            copied.pop(".cargo-checksum.json")
            admitted = next(entry for entry in session.dependency_policy["packages"] if entry["name"] == item["name"])
            if (checksum.get("package") != item["checksum"] or checksum.get("files") != copied
                    or PREPARE.files_digest(copied) != admitted["files_sha256"]):
                raise Incomplete("Copied product dependency no longer matches its admitted archive map.")
    session.configure(COMMON.FUZZ_CARGO_CONFIG)
    sealed = [project, vendor]
    before = COMMON.fuzz_input_snapshot(session.work, sealed)
    session.protect(project); session.protect(vendor)
    night = session.base / session.admitted["policy"]["common_nightly"]
    tools = session.output / "fuzz-tools"; tools.mkdir(mode=0o700)
    launcher = tools / "cargo"
    launcher.write_text('#!/usr/bin/python3\nimport os,sys\n'
                        'if len(sys.argv)<2 or sys.argv[1] not in ("build","check","rustc","metadata"):\n'
                        ' raise SystemExit("unreviewed fuzz Cargo subcommand")\n'
                        f'os.execv({str(night / "bin/cargo")!r}, [{str(night / "bin/cargo")!r},sys.argv[1],"--frozen",*sys.argv[2:]])\n')
    launcher.chmod(0o500)
    shutil.copyfile(session.base / "common/bin/cargo-fuzz", tools / "cargo-fuzz")
    (tools / "cargo-fuzz").chmod(0o500)
    if PRODUCT.sha256(tools / "cargo-fuzz") != session.admitted["policy"]["tools"]["fuzz"]["sha256"]:
        raise Incomplete("The copied cargo-fuzz executable differs from admitted tool bytes.")
    session.protected_tools[tools] = PRODUCT.inventory(tools)
    metadata = session.run("fuzz-graph", [str(night / "bin/cargo"), "metadata", "--frozen", "--format-version", "1",
                           "--filter-platform", ANALYSIS.PLATFORM, "--manifest-path", "/work/fuzz/Cargo.toml"],
                           120, compiler=night, extra_tools=tools, sealed=sealed, cwd=project)
    report["fuzz_graph"] = fuzz_metadata(json.loads(metadata), selected)
    corpus = session.work / "fuzz-corpus"; corpus.mkdir(mode=0o700)
    seeds = history_fuzz_seeds()
    for name, data in seeds.items():
        (corpus / name).write_bytes(data)
    seed_identity = PRODUCT.inventory(corpus)
    (session.work / "fuzz-artifacts").mkdir(mode=0o700)
    binaries = session.work / "fuzz-binaries"; binaries.mkdir(mode=0o700)
    retained = {}
    for case in ("healthy", "defect"):
        command = [str(tools / "cargo-fuzz"), "fuzz", "build", "fit", "--fuzz-dir", "/work/fuzz",
                   "--sanitizer", "address", "--target", ANALYSIS.PLATFORM, "--target-dir", "/work/target-fuzz"]
        if case == "defect":
            command.extend(["--features", "seeded-defect"])
        session.run("fuzz-build-" + case, command, 600, compiler=night, extra_tools=tools,
                    sealed=[*sealed, *retained], cwd=project)
        built = session.work / "target-fuzz" / ANALYSIS.PLATFORM / "release/fit"
        digest = PRODUCT.sha256(built)
        if digest in retained.values():
            raise Incomplete("Healthy and seeded numeric builds emitted the same executable.")
        binary = binaries / case
        shutil.copyfile(built, binary); binary.chmod(0o500)
        if PRODUCT.sha256(binary) != digest:
            raise Incomplete("Immutable numeric executable differs from its real build.")
        retained[binary] = digest
        session.protect(binaries)
        report.setdefault("fuzz_binaries_sha256", {})[case] = digest
        report.setdefault("fuzz_binary_paths", {})[case] = str(binary)
        text = session.run("fuzz-" + case, ["/work/fuzz-binaries/" + case, "/work/fuzz-corpus",
                           "-artifact_prefix=/work/fuzz-artifacts/", *COMMON.RUNTIME.HISTORY_SANITIZER_FLAGS],
                           selected["invocation_limits"]["wall_seconds"], compiler=night, extra_tools=tools, sealed=[*sealed, *retained], cwd=project,
                           mode="sanitizer", sanitizer_profile=COMMON.RUNTIME.HISTORY_SANITIZER_PROFILE,
                           negative="independent history oracle: frozen published report bits" if case == "defect" else None)
        actual = [row for row in report['lanes'] if row.get('lane') == 'fuzz-' + case]
        if len(actual) != 1:
            raise Incomplete('History fuzz requires one unique actual native receipt.')
        raw_text = history_fuzz_outcome(actual[0], defect=case == 'defect')
        # Session.run returns require_ok(...).strip() for a healthy child and
        # the raw output for its explicit negative child. Keep the receipt raw
        # for cause/counter checks; reconcile only that existing return contract.
        returned_text = raw_text if case == 'defect' else raw_text.strip()
        if returned_text != text:
            raise Incomplete('History fuzz returned text differs from its unique actual native receipt.')
        text = raw_text
        observed = history_fuzz_counts(text, complete=case == "healthy")
        record_counts(report, "fuzz-" + case, observed)
        report.setdefault("fuzz_observed_invocations", {})[case] = observed
        if case == "healthy":
            count = re.search(r"Done (\d+) runs", text)
            if not count or int(count[1]) != 4096:
                raise Incomplete("Healthy history fuzz target did not complete its finite 4096 runs.")
            report["fuzz"] = {"observed_runs": 4096, "subjects": {"history": HISTORICAL_SUBJECTS["history"]}, "seed_files_sha256": seed_identity,
                              "seed_lengths": sorted({len(data) for data in seeds.values()}), "seeded_fault": "pending",
                              "input_format": "headerenum0..4+raw24Btuples;65appends/1561B;no phase sentinel",
                              "frozen_projection_count": 29,
                              "projection_source_facts": "ec6a1b96806c6219bd2b74c37b737fde7cbfddafe6cf2f07b12cb14d1d9c92d1",
                              "projection_source_ordinals": {item["name"]: {"trace": item["source_trace"], "ordinals": item["source_ordinals"]} for item in HISTORY_SEED_CASES},
                              "scope_exclusions": "No simultaneous two-history lifecycle or arbitrary widei128 phase/real-world claim; full70 trace remains publictest coverage.", "sanitizer": "address", "target": ANALYSIS.PLATFORM}
        else:
            report["fuzz"]["seeded_fault"] = "observed intended independent captured published-report bit assertion failure"
        if any(PRODUCT.sha256(path) != expected for path, expected in retained.items()):
            raise Incomplete("Retained ASan executable changed.")
        if COMMON.fuzz_input_snapshot(session.work, sealed) != before:
            raise Incomplete("Fuzz manifest/lock/source/vendor/config inventory changed.")
    admit_fuzz_sources(session, selected)
    report["fuzz"]["fault_artifacts_sha256"] = PRODUCT.inventory(session.work / "fuzz-artifacts")



def execute_inventory(args, report):
    """Collect four listings under one deadline; admit no native product call."""
    if args.subject != "all" or getattr(args, "inventory_admission", None) is None:
        raise Incomplete("Inventory discovery requires all four checkpoint owners and its explicit admission.")
    deadline = time.monotonic() + INVENTORY_FIXED["inventory_wall_seconds"]
    policy = read_inventory_admission(ROOT, args.inventory_admission)
    admission_sha256 = PRODUCT.sha256(ROOT / INVENTORY_ADMISSION)
    scope = COMMON.Scope("time-core", tuple(INVENTORY_SUBJECTS.items()), INVENTORY_ADMISSION)
    # Session uses the mutation lane solely to seal the cargo-mutants executable.
    # Each command below is a listing without mutation_subject, retaining the
    # runtime's ordinary 128-task profile rather than the 512 campaign profile.
    session_args = argparse.Namespace(**{**vars(args), "lane": "mutation"})
    session = COMMON.Session(ROOT, session_args, report, scope, policy,
                             tuple(policy["tool_inputs_sha256"]))
    report.update(subjects=dict(INVENTORY_SUBJECTS),
                   qualification_scope="four-owner inventory discovery only; no campaign or quality score",
                  campaigns_admitted=False, native_test_invocations=0,
                  listing_process_limit=128, inventory_admission_sha256=admission_sha256)

    def run(label, command, timeout, **options):
        inventory_admission_live(policy)
        subject = label.removeprefix("mutation-enumeration:")
        if subject not in INVENTORY_SUBJECTS or label != "mutation-enumeration:" + subject:
            raise Incomplete("Inventory-only execution has an unadmitted listing label.")
        expected = [str(session.tools / "cargo-mutants"), "mutants", "--package", "time-core",
                    "--file", INVENTORY_SUBJECTS[subject], "--test-workspace", "false",
                    "--cap-lints", "false", "--jobs", "10", "--jobserver", "true",
                    "--jobserver-tasks", "10", "--cargo-arg=--message-format=json", "--list", "--json"]
        if options or command != expected:
            raise Incomplete("Inventory-only execution attempted a non-listing command or effect profile.")
        return session.run(label, command, timeout)

    try:
        if (not exact(inventory_document(session.source / INVENTORY_ADMISSION), policy)
                or session.product_identity != policy["product_inputs_sha256"]
                or any(session.identity.get(name) != digest
                       for name, digest in policy["tool_inputs_sha256"].items())):
            raise Incomplete("Inventory session copied different admission, product or tool inputs.")
        inventory_admission_live(policy)
        result = COMMON.enumerate_mutation_inventories(
            scope, INVENTORY_SUBJECTS, policy, session.tools, run, report, deadline=deadline,
            maximum_per_file=policy["listing_maximum_per_file"],
            maximum_total=min(policy["listing_maximum_total"],
                              policy["listing_maximum_per_file"] * len(INVENTORY_SUBJECTS)))
        summaries = {}
        for name, descriptors in result.inventories.items():
            normalized = {identifier: {key: value for key, value in item.items() if key != "diff"}
                          for identifier, item in descriptors.items()}
            summaries[name] = {"count": len(descriptors), "sha256": COMMON.canonical_sha256(normalized)}
        report["inventory_discovery"] = {
            "inventories": summaries, "total": result.total,
            "exceeds_current_campaign_cap": result.campaign_budget_exceeded,
            "campaign_cap": policy["maximum_mutants"], "campaign_admission": "not granted",
            "reservation_next_action": "bind exact inventories and reserve the complete four-owner checkpoint before execution",
        }
    finally:
        primary = sys.exception()
        try:
            session.finish()
        except REPORT_FAILURES as error:
            if primary is None:
                raise
            retain_secondary_error(report, "inventory_session_finish", error)


def final_package_test_count(kind):
    """Use actual frozen discovery; historical 94/96 counts admit no new run."""
    value = FINAL_PACKAGE_TEST_COUNTS.get(kind)
    if type(value) is not int or value <= 0:
        raise Incomplete('Final projection package test discovery remains pending.')
    return value


def package_charged_upper(*, include_doctests=False):
    """Extend preserved package uppers by each disjoint source-owned suite."""
    projection = PROJECTION.full_package_reservation()['projection_reserved_source_work_uppers']
    enclosure = ENCLOSURE.full_package_reservation()['enclosure_reserved_source_work_uppers']
    projected_batch = projected_batch_reservation()['projected_batch_reserved_source_work_uppers']
    principal = principal_reservation()['principal_reserved_source_work_uppers']
    combination = combination_reservation()['combination_reserved_source_work_uppers']
    if type(include_doctests) is not bool:
        raise Incomplete('The package doctest scope must be Boolean.')
    return {'charged_fit_work_upper': 5233 + projection['charged_fit_work_upper'] + enclosure['charged_fit_work_upper'] + projected_batch['charged_fit_work_upper'] + principal['charged_fit_work_upper'] + combination['charged_fit_work_upper'],
            'point_admissions_upper': 220797 + projection['point_admissions_upper'] + enclosure['point_admissions_upper'] + projected_batch['point_admissions_upper'] + principal['point_admissions_upper'] + combination['point_admissions_upper']
                                      + (LITERAL_DOCTEST_LIMITS['point_admissions_upper'] if include_doctests else 0)}


def cohort_budget_vectors(*, require_limits=True):
    """Use the exact reviewed cumulative allowances for the complete owner union.

    One vector covers the entire cohort union, including every baseline and
    remaining coverage/contracts/fuzz source recipe. A new invocation cannot
    reset any allowance. Owner allowances require complete source/union review.
    """
    package = package_charged_upper(include_doctests=True)
    limits = dict(GLOBAL_LIMITS)
    groups = (
        ('selection.call.', SELECTION_PACKAGE_RESERVATION, SELECTION_GLOBAL_LIMITS),
        ('selection.helper.', SELECTION_HELPER_RESERVATION, SELECTION_HELPER_GLOBAL_LIMITS),
        ('selection.helper_work.', SELECTION_HELPER_CANONICAL_WORK_LIMITS, SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS),
        ('source.private.', SOURCE_RAW_PRIVATE_WORK_LIMITS, SOURCE_RAW_PRIVATE_GLOBAL_LIMITS),
    )
    for prefix, per_package, global_limits in groups:
        package.update({prefix + key: value for key, value in per_package.items()})
        limits.update({prefix + key: value for key, value in global_limits.items()})
    reservation = PROJECTION.full_package_reservation()
    for prefix, name in (('projection.call.', 'projection_reserved_calls'),
                         ('projection.work.', 'projection_reserved_source_work_uppers'),
                         ('projection.private.', 'projection_reserved_private_source_work')):
        package.update({prefix + key: value for key, value in reservation[name].items()})
    projection_docs = {key: value for key, value in LITERAL_DOCTEST_LIMITS.items()
                       if key.startswith('projection_')}
    package.update({'projection.doctest.' + key: value for key, value in projection_docs.items()})
    projection_limits = PROJECTION_TELEMETRY['global_limits']
    projection_keys = {key for key in package if key.startswith('projection.')}
    if (not isinstance(projection_limits, dict) or set(projection_limits) != projection_keys
            or any(type(value) is not int or value < 0 for value in projection_limits.values())):
        raise Incomplete('Projection cumulative source-work allowances await final source/cohort review.')
    limits.update(projection_limits)
    enclosure_reservation = ENCLOSURE.full_package_reservation()
    for prefix, name in (('enclosure.call.', 'enclosure_reserved_calls'),
                         ('enclosure.work.', 'enclosure_reserved_source_work_uppers'),
                         ('enclosure.private.', 'enclosure_reserved_private_source_work')):
        package.update({prefix + key: value for key, value in enclosure_reservation[name].items()})
    enclosure_limits = ENCLOSURE_TELEMETRY['global_limits']
    enclosure_keys = {key for key in package if key.startswith('enclosure.')}
    if (not isinstance(enclosure_limits, dict) or set(enclosure_limits) != enclosure_keys
            or any(type(value) is not int or value < 0 for value in enclosure_limits.values())):
        raise Incomplete('Enclosure cumulative source-work allowances await complete source/cohort review.')
    limits.update(enclosure_limits)
    projected_batch = projected_batch_reservation()
    for prefix, name in (('projected_batch.call.', 'projected_batch_reserved_calls'),
                         ('projected_batch.work.', 'projected_batch_reserved_source_work_uppers'),
                         ('projected_batch.private.', 'projected_batch_reserved_private_source_work')):
        package.update({prefix + key: value for key, value in projected_batch[name].items()})
    projected_batch_limits = admit_projected_batch_allowances(PROJECTED_BATCH_TELEMETRY['global_limits'])
    projected_batch_keys = {key for key in package if key.startswith('projected_batch.')}
    if set(projected_batch_limits) != projected_batch_keys:
        raise Incomplete('Projected-batch package and coordinator allowance ownership differ.')
    limits.update(projected_batch_limits)
    principal = principal_reservation()
    for prefix, name in (('principal.call.', 'principal_reserved_calls'),
                         ('principal.work.', 'principal_reserved_source_work_uppers')):
        package.update({prefix + key: value for key, value in principal[name].items()})
    if type(require_limits) is not bool:
        raise Incomplete('Budget derivation requires an explicit Boolean admission mode.')
    limits.update(admit_principal_allowances(PRINCIPAL_TELEMETRY['global_limits']) if require_limits
                  else principal_budget_vector())
    combination = combination_reservation()
    for prefix, name in (('combination.call.', 'combination_reserved_calls'),
                         ('combination.work.', 'combination_reserved_source_work_uppers')):
        package.update({prefix + key: value for key, value in combination[name].items()})
    limits.update(admit_combination_allowances(COMBINATION_TELEMETRY['global_limits']) if require_limits
                  else combination_budget_vector())
    zero = {key: 0 for key in package}
    legal_calls = principal_legal_calls()
    legal_work = principal_source_work(legal_calls)
    combination_calls = combination_legal_calls()
    combination_work = combination_source_work(combination_calls)
    contracts = {**zero, **{key: legal_work[key] + combination_work[key] for key in GLOBAL_LIMITS}}
    contracts.update({'principal.call.' + key: value for key, value in legal_calls.items()})
    contracts.update({'principal.work.' + key: value for key, value in legal_work.items()})
    contracts.update({'combination.call.' + key: value for key, value in combination_calls.items()})
    contracts.update({'combination.work.' + key: value for key, value in combination_work.items()})
    coverage = dict(package)
    coverage['point_admissions_upper'] -= LITERAL_DOCTEST_LIMITS['point_admissions_upper']
    for key in projection_docs:
        coverage['projection.doctest.' + key] = 0
    return package, {'coverage': coverage, 'contracts': contracts}, limits


def checkpoint_source_reservation():
    """Derive a fresh source reservation only after all actual listings exist."""
    if set(MUTATION_INVENTORIES) != set(SUBJECTS):
        raise Incomplete('Four actual current mutation inventories remain pending; no cohort reservation is admitted.')
    package, checks, _ = cohort_budget_vectors(require_limits=False)
    plan = {'inventories': MUTATION_INVENTORIES,
            'remaining_checks': {name: 1 for name in COHORTS.CHECKS}}
    return COHORTS.plan_reservation(plan, package, checks)


def bind_checkpoint_source_allowances():
    """Replace historical initializer values with the exact current source sum.

    This is deterministic finite arithmetic from the literal actual4 inventories
    and independently bounded package/contract recipes, not executed work. Final
    policy/source hashes and the real cohort plan are still admitted separately.
    """
    reservation = checkpoint_source_reservation()
    vector = reservation['source_work']
    groups = (
        ('', GLOBAL_LIMITS), ('selection.call.', SELECTION_GLOBAL_LIMITS),
        ('selection.helper.', SELECTION_HELPER_GLOBAL_LIMITS),
        ('selection.helper_work.', SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS),
        ('source.private.', SOURCE_RAW_PRIVATE_GLOBAL_LIMITS),
        ('', PROJECTION_TELEMETRY['global_limits']),
        ('', ENCLOSURE_TELEMETRY['global_limits']),
        ('', PROJECTED_BATCH_TELEMETRY['global_limits']),
    )
    for prefix, mapping in groups:
        keys = tuple(mapping)
        mapping.update({key: vector[prefix + key] for key in keys})
    PRINCIPAL_TELEMETRY['global_limits'] = {key: value for key, value in vector.items()
                                          if key.startswith('principal.')}
    PRINCIPAL_TELEMETRY['status'] = 'admitted-source-owned-profiles'
    COMBINATION_TELEMETRY['global_limits'] = {key: value for key, value in vector.items()
                                            if key.startswith('combination.')}
    COMBINATION_TELEMETRY['status'] = 'admitted-source-owned-profiles'
    return reservation


def read_cohort_plan(root, path, policy):
    """Read a strict source-bound coordinator plan before any native effect."""
    root = Path(root).resolve(strict=True)
    # Only the declared checkpoint coverage/contracts remain in this plan.
    if not exact(read_policy(root, 'contracts'), policy):
        raise Incomplete('Coordinator plan differs from the complete remaining-lane source admission.')
    path = Path(path)
    path = path if path.is_absolute() else root / path
    if (not path.is_relative_to(root / 'tasks') or path.resolve(strict=True) != path
            or not path.is_file()):
        raise Incomplete('Use an exact regular coordinator cohort plan inside the existing task directory.')
    plan = inventory_document(path)
    proof_inputs = {entry['proof']['path'] for entry in policy['mutation_equivalences']}
    tool_inputs = {name: PRODUCT.sha256(root / name)
                   for name in sorted(inventory_tool_inputs(root) | {POLICY} | proof_inputs)}
    package, checks, limits = cohort_budget_vectors()
    recipe = COMMON.canonical_sha256({'package': package, 'remaining_checks': checks,
                                     'test_counts': FINAL_PACKAGE_TEST_COUNTS,
                                     'projection_recipes': PROJECTION_TELEMETRY['case_recipes'],
                                     'projection_private_work': PROJECTION.SOURCE_PRIVATE_WORK,
                                     'enclosure_recipes': ENCLOSURE.case_recipes(),
                                     'enclosure_private_work': ENCLOSURE.SOURCE_PRIVATE_WORK,
                                     'enclosure_literal_doctests_added': ENCLOSURE.LITERAL_DOCTESTS_ADDED,
                                     'projected_batch_recipes': PROJECTED_BATCH.case_recipes(),
                                     'projected_batch_guard_recipes': PROJECTED_BATCH.guard_recipes(),
                                     'projected_batch_constructor_work': PROJECTED_BATCH.SOURCE_WORK_PER_BATCH,
                                     'projected_batch_getter_work': PROJECTED_BATCH.SOURCE_WORK_PER_GETTER,
                                     'projected_batch_private_work': PROJECTED_BATCH.SOURCE_PRIVATE_WORK,
                                     'projected_batch_literal_doctests_added': PROJECTED_BATCH.LITERAL_DOCTESTS_ADDED,
                                     'principal_case_ceilings': PRINCIPAL_CASE_CEILINGS,
                                     'principal_case_recipes': PRINCIPAL_CASE_RECIPES,
                                     'principal_private_case_source_uppers': PRINCIPAL_PRIVATE_WORK,
                                     'combination_case_recipes': COMBINATION.case_recipes(),
                                     'combination_uncertainty_work': COMBINATION.SOURCE_WORK_PER_UNCERTAINTY,
                                     'combination_kernel_work': COMBINATION.SOURCE_WORK_PER_COMBINATION,
                                     'combination_legal_calls': combination_legal_calls(),
                                     'contract_cases': contract_examples(),
                                     'global_limits': limits})
    try:
        admitted = COHORTS.admit_plan(
            plan, subjects=SUBJECTS, product_inputs=policy['inputs_sha256'], tool_inputs=tool_inputs,
            recipe_sha256=recipe, inventories=policy['mutation_inventories'],
            baseline_tests=final_package_test_count('baseline'), package_budget=package,
            check_budgets=checks, global_limits=limits)
    except COHORTS.Incomplete as error:
        raise Incomplete(str(error)) from error
    return plan, admitted, path


def execute_cohort_union(args, report):
    """Reconcile already completed exact receipts; launch no product or analyzer."""
    policy = read_policy(ROOT, 'contracts')
    plan, admitted, _ = read_cohort_plan(ROOT, args.cohort_plan, policy)
    receipts = [native_receipt_document(path.resolve(strict=True)) for path in args.cohort_receipt]
    try:
        union = COHORTS.reconcile_receipts(plan, receipts, subjects=SUBJECTS)
    except COHORTS.Incomplete as error:
        raise Incomplete(str(error)) from error
    output = args.output_dir
    if (not output.is_absolute() or output.parent != output.parent.resolve(strict=True)
            or output.is_relative_to(ROOT) or output.is_relative_to(Path('/mnt'))):
        raise Incomplete('Use a fresh external canonical private Linux output directory.')
    output.mkdir(mode=0o700)
    report.update(owned_output=str(output), core_cohort_union=union,
                  core_coordinator_plan_reservation=admitted,
                  qualification_scope='Exact mutation receipt union only; remaining checks and native Windows are separate.')


def prospective_scope_reservation(lane, subjects):
    """Admit the full finite selected scope before any product command.

    The pinned current inventories and full source recipes determine this matrix;
    favorable post-run outcomes never reduce it. It is no hard CPU/RSS/filesystem quota or bound
    on arbitrary mutant internals; later observations retain their own scope.
    """
    if (lane not in {'contracts', 'coverage', 'mutation'}
            or not isinstance(subjects, dict) or not subjects
            or any(name not in SUBJECTS or SUBJECTS[name] != filename for name, filename in subjects.items())):
        raise Incomplete('Prospective core scope is outside the exact admitted lanes/owners.')
    if lane == 'mutation' and any(type(MUTATION_INVENTORIES.get(name)) is not dict for name in subjects):
        raise Incomplete('Actual selected mutation inventories remain pending.')
    mutation_invocations = (sum(MUTATION_INVENTORIES[name]['count'] + 1 for name in subjects)
                            if lane == 'mutation' else 0)
    coverage_invocations = int(lane == 'coverage')
    packages = mutation_invocations + coverage_invocations
    legal = int(lane == 'contracts')
    legal_work = principal_source_work(principal_legal_calls())
    combination_legal_work = combination_source_work(combination_legal_calls())
    per_package = package_charged_upper(include_doctests=lane == 'mutation')
    charged = {key: packages * value + legal * (legal_work[key] + combination_legal_work[key])
               for key, value in per_package.items()}
    selection = {key: maximum*packages for key, maximum in SELECTION_PACKAGE_RESERVATION.items()}
    helpers = {key: maximum*packages for key, maximum in SELECTION_HELPER_RESERVATION.items()}
    helper_work = {key: maximum*packages for key, maximum in SELECTION_HELPER_CANONICAL_WORK_LIMITS.items()}
    raw_private = {key: maximum*packages for key, maximum in SOURCE_RAW_PRIVATE_WORK_LIMITS.items()}
    projected_batch = {key: amount*packages for key, amount in projected_batch_budget_vector().items()}
    projected_batch_limits = admit_projected_batch_allowances(PROJECTED_BATCH_TELEMETRY['global_limits'])
    principal = {key: amount * packages for key, amount in principal_budget_vector().items()}
    combination = {key: amount * packages for key, amount in combination_budget_vector().items()}
    for prefix, values in (('principal.call.', principal_legal_calls()), ('principal.work.', legal_work)):
        for key, amount in values.items():
            principal[prefix + key] += legal * amount
    for prefix, values in (('combination.call.', combination_legal_calls()),
                           ('combination.work.', combination_legal_work)):
        for key, amount in values.items():
            combination[prefix + key] += legal * amount
    if PRINCIPAL_TELEMETRY['global_limits'] is not None:
        principal_limits = admit_principal_allowances(PRINCIPAL_TELEMETRY['global_limits'])
        if any(amount > principal_limits[key] for key, amount in principal.items()):
            raise Incomplete('Prospective principal work exceeds its exact checkpoint allowance.')
    combination_limits = admit_combination_allowances(COMBINATION_TELEMETRY['global_limits'])
    if any(amount > combination_limits[key] for key, amount in combination.items()):
        raise Incomplete('Prospective combination work exceeds its exact checkpoint allowance.')
    if (any(charged[key] > maximum for key, maximum in GLOBAL_LIMITS.items())
            or any(selection[key] > maximum for key, maximum in SELECTION_GLOBAL_LIMITS.items())
            or any(helpers[key] > maximum for key, maximum in SELECTION_HELPER_GLOBAL_LIMITS.items())
            or any(helper_work[key] > maximum for key, maximum in SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS.items())
            or any(raw_private[key] > maximum for key, maximum in SOURCE_RAW_PRIVATE_GLOBAL_LIMITS.items())
            or any(projected_batch[key] > maximum for key, maximum in projected_batch_limits.items())):
        raise Incomplete('Complete prospective core scope exceeds its admitted finite reservation.')
    return {'mutation_package_invocations_upper': mutation_invocations,
            'coverage_package_invocations_upper': coverage_invocations,
            'package_invocations_upper': packages,
            'package_test_count_upper_each': final_package_test_count('baseline'),
            'legal_consumer_executions_upper': legal*EXTERNAL_CONSUMER_LIMITS['legal_executions'],
            'negative_consumer_compilations_upper': legal*EXTERNAL_CONSUMER_LIMITS['negative_compilations'],
            'history_fuzz_invocations_upper': 0,
            'charged_upper': charged, 'selection_call_upper': selection,
            'selection_direct_helper_call_upper': helpers,
            'selection_raw_helper_canonical_source_work_upper': helper_work,
            'source_raw_private_direct_work_upper': raw_private,
            'projected_batch_source_work_upper': projected_batch,
            'principal_source_work_upper': principal,
            'combination_source_work_upper': combination,
            'scope': 'Pre-effect source reservation for this fresh scope; historical attempts are retained, never erased or asserted as observations.'}


def execute(args, report):
    """Run only selected current-admitted rows; preserve partial raw observations."""
    if args.lane not in {'inventory', 'contracts', 'coverage', 'mutation', 'cohort-union'}:
        raise Incomplete('Unsupported checkpoint lane; optional fuzz/all execution is not admitted.')
    if args.lane == "inventory":
        return execute_inventory(args, report)
    if args.lane == 'cohort-union':
        return execute_cohort_union(args, report)
    policy = read_policy(ROOT, args.lane)
    subjects = SCOPE.select(args.subject)
    plan = None
    plan_path = None
    if args.lane in {'mutation', 'all'}:
        inventories = policy['mutation_inventories']
        if args.subject == 'all' and sum(row['count'] for row in inventories.values()) > 512:
            raise Incomplete('The complete mutation union exceeds512; use the admitted disjoint focused cohorts.')
        if (args.lane != 'mutation' or args.subject == 'all'
                or getattr(args, 'cohort_plan', None) is None or getattr(args, 'cohort_id', None) is None):
            raise Incomplete('Mutation requires a named focused subject and its explicit same-candidate coordinator plan/cohort.')
        plan, admitted, plan_path = read_cohort_plan(ROOT, args.cohort_plan, policy)
        try:
            report['core_cohort_binding'] = COHORTS.cohort_binding(
                plan, args.cohort_id, subject=args.subject, output_dir=str(args.output_dir))
        except COHORTS.Incomplete as error:
            raise Incomplete(str(error)) from error
        report['core_coordinator_plan_reservation'] = admitted
    else:
        report['core_prospective_scope_reservation'] = prospective_scope_reservation(args.lane, subjects)
    extra = ("scripts/verify-core-analysis.py", "scripts/verify-time-domain-analysis.py",
             "scripts/analysis-controls.py", *SELECTION_INPUTS, *PROJECTION_INPUTS, *ENCLOSURE_INPUTS,
             *PROJECTED_BATCH_INPUTS, *COMBINATION_INPUTS, *BOUND_INPUTS,
             *(tuple(plan['tool_inputs_sha256']) + (plan_path.relative_to(ROOT).as_posix(),) if plan is not None else ()),
             *(FUZZ_INPUTS if args.lane in {"fuzz", "all"} else ()))
    session = COMMON.Session(ROOT, args, report, SCOPE, policy, extra)
    report["subjects"] = subjects
    report.update(checkpoint_scope_claim(report['subjects']))
    report["resource_scope"] = "Logical operation counters and existing per-process rlimits; no hard RAM/disk/cgroup or whole-product claim."
    report["doctest_accounting"] = "Literal source doctests must be audited before execution. Coverage selects lib/tests; mutation cargo-test logs charge the separate frozen doctest upper allowance."
    def run(label, command, timeout, **options):
        if label == "coverage":
            record_counts(report, label, {
                **package_charged_upper(),
                **selection_reservation(package=True), 'observation_pending': True,
                'execution': 'Full package source recipe reserved before coverage execution.'})
            reserved = report['core_invocations'][-1]
            result = session.run(label, command, timeout, **options)
            if COMMON.test_count(result) != final_package_test_count('ordinary'):
                raise Incomplete(f'Coverage did not execute all {final_package_test_count("ordinary")} admitted ordinary core tests.')
            reserved['observed_counts'] = profile_counts(result, require_runs=True, require_selection=True,
                                                        require_projection=True, require_enclosure=True,
                                                        require_projected_batch=True, require_principal=True,
                                                        require_combination=True)
            reserved['observation_pending'] = False
            refresh_accounting(report)
            return result
        result = session.run(label, command, timeout, **options)
        return result
    try:
        if plan is not None:
            if (not exact(session.product_identity, plan['product_inputs_sha256'])
                    or not exact(report.get('tools_sha256'), plan['tool_payloads_sha256'])
                    or any(session.identity.get(name) != digest for name, digest in plan['tool_inputs_sha256'].items())
                    or not exact(inventory_document(session.source / plan_path.relative_to(ROOT)), plan)):
                raise Incomplete('Cohort session copied a different candidate, tool payload/source or coordinator plan.')
        if args.lane in {"contracts", "all"}:
            contracts(session, policy, report)
        try:
            COMMON.run_analysis_lanes(SCOPE, report["subjects"], policy, session.work, session.tools,
                                      run, report, args.lane)
        finally:
            if args.lane in {"mutation", "all"}:
                primary = sys.exception()
                try:
                    mutation_accounting(report, session.work)
                except REPORT_FAILURES as error:
                    if primary is None:
                        raise
                    retain_secondary_error(report, 'mutation_accounting', error)
        if args.lane in {"mutation", "all"}:
            if report.get("mutation_missing_selected_files") or report.get("mutation_baselines_observed") != len(report["subjects"]):
                raise Incomplete("Selected per-file mutation campaigns/baselines are incomplete.")
            if any(row['counts']['T'] for row in report.get('mutation_full_accounting', {}).values()):
                raise Incomplete("Selected mutation campaigns contain unresolved native timeouts.")
        if args.lane in {"fuzz", "all"}:
            fuzz(session, policy["coverage_guided_fuzzing"], report)
    finally:
        primary = sys.exception()
        try:
            session.finish()
        except REPORT_FAILURES as error:
            if primary is None:
                raise
            retain_secondary_error(report, 'session_finish', error)


def retain_secondary_error(report, phase, error):
    """Keep cleanup/accounting failures without replacing the primary exception."""
    report.setdefault('secondary_errors', []).append({
        'phase': phase, 'type': type(error).__name__, 'error': str(error)})


def main(argv=None):
    """Fresh owned outputs, fixed scopes and nonzero incomplete status only."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("inventory", "contracts", "coverage", "mutation", "cohort-union"), default="coverage")
    parser.add_argument("--subject", choices=(*SUBJECTS, "all"), default="all")
    parser.add_argument("--inventory-admission", type=Path)
    parser.add_argument('--cohort-plan', type=Path)
    parser.add_argument('--cohort-id')
    parser.add_argument('--cohort-receipt', type=Path, action='append', default=[])
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    if args.lane == "inventory" and (args.inventory_admission is None or args.subject != "all"):
        parser.error("--lane inventory requires --subject all and --inventory-admission")
    if args.lane != "inventory" and args.inventory_admission is not None:
        parser.error("--inventory-admission is accepted only by --lane inventory")
    if args.lane == 'cohort-union':
        if args.cohort_plan is None or args.cohort_id is not None or len(args.cohort_receipt) != len(SUBJECTS) or args.subject != 'all':
            parser.error('--lane cohort-union requires --subject all, --cohort-plan and exactly four --cohort-receipt paths')
    elif args.cohort_receipt:
        parser.error('--cohort-receipt is accepted only by --lane cohort-union')
    if args.lane == 'mutation':
        if args.cohort_plan is None or args.cohort_id is None or args.subject == 'all':
            # Execution retains an independent aggregate512 receiving guard.
            # The CLI accepts all only with a plan so that guard can reject it.
            if not (args.subject == 'all' and args.cohort_plan is not None and args.cohort_id is not None):
                parser.error('--lane mutation requires an explicit --cohort-plan/--cohort-id and named --subject')
    elif args.lane != 'cohort-union' and (args.cohort_plan is not None or args.cohort_id is not None):
        parser.error('--cohort-plan/--cohort-id are accepted only by mutation or cohort-union')
    report = {"status": "incomplete", "selected_lane": args.lane,
              "scope": "Four changed/coupled critical owners (combination/principal/regression/projection), bounded time-core checkpoint " + args.lane, "native_qualification": "not performed"}
    try:
        if args.vendor_dir is None and args.lane != 'cohort-union':
            args.vendor_dir = Path(json.loads(PRODUCT.POINTER.read_text())["vendor_dir"])
        execute(args, report)
        report["status"] = "passed"
    except REPORT_FAILURES as error:
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


# Frozen input tuples are appended below; no implementation computes an oracle.

NORMALIZATION_SEED_INPUTS = json.loads(r'''
[{"name":"count_zero","rows":[]},{"name":"count_one_at_zero","rows":[["0","0","0x3ff0000000000000"]]},{"name":"count_one_at_u64_max","rows":[["18446744073709551615","0","0x3ff0000000000000"]]},{"name":"count_two_below_minimum_is_insufficient","rows":[["0","0","0x3ff0000000000000"],["1","0","0x3ff0000000000000"]]},{"name":"count_two_at_maximum","rows":[["0","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"count_two_above_maximum","rows":[["0","0","0x3ff0000000000000"],["4096000000001","0","0x3ff0000000000000"]]},{"name":"dyadic_line_small_origin","rows":[["0","0","0x3ff0000000000000"],["1000000000","8388608","0x3ff0000000000000"],["2000000000","16777216","0x3ff0000000000000"]]},{"name":"dyadic_line_u64_max_origin","rows":[["18446744071709551615","0","0x3ff0000000000000"],["18446744072709551615","8388608","0x3ff0000000000000"],["18446744073709551615","16777216","0x3ff0000000000000"]]},{"name":"one_ns_admitted","rows":[["0","0","0x3ff0000000000000"],["62499999","0","0x3ff0000000000000"],["62500000","0","0x3ff0000000000000"]]},{"name":"one_ns_admitted_u64_max","rows":[["18446744073647051615","0","0x3ff0000000000000"],["18446744073709551614","0","0x3ff0000000000000"],["18446744073709551615","0","0x3ff0000000000000"]]},{"name":"minimum_span_below","rows":[["4095937500001","0","0x3ff0000000000000"],["4095968750000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"minimum_span_exact","rows":[["4095937500000","0","0x3ff0000000000000"],["4095968750000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"minimum_span_above","rows":[["4095937499999","0","0x3ff0000000000000"],["4095968750000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"maximum_span_below","rows":[["1","0","0x3ff0000000000000"],["2048000000000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"maximum_span_exact","rows":[["0","0","0x3ff0000000000000"],["2048000000000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"maximum_span_above","rows":[["0","0","0x3ff0000000000000"],["2048000000001","0","0x3ff0000000000000"],["4096000000001","0","0x3ff0000000000000"]]},{"name":"count_64_uniform","rows":[["0","0","0x3ff0000000000000"],["1000000000","0","0x3ff0000000000000"],["2000000000","0","0x3ff0000000000000"],["3000000000","0","0x3ff0000000000000"],["4000000000","0","0x3ff0000000000000"],["5000000000","0","0x3ff0000000000000"],["6000000000","0","0x3ff0000000000000"],["7000000000","0","0x3ff0000000000000"],["8000000000","0","0x3ff0000000000000"],["9000000000","0","0x3ff0000000000000"],["10000000000","0","0x3ff0000000000000"],["11000000000","0","0x3ff0000000000000"],["12000000000","0","0x3ff0000000000000"],["13000000000","0","0x3ff0000000000000"],["14000000000","0","0x3ff0000000000000"],["15000000000","0","0x3ff0000000000000"],["16000000000","0","0x3ff0000000000000"],["17000000000","0","0x3ff0000000000000"],["18000000000","0","0x3ff0000000000000"],["19000000000","0","0x3ff0000000000000"],["20000000000","0","0x3ff0000000000000"],["21000000000","0","0x3ff0000000000000"],["22000000000","0","0x3ff0000000000000"],["23000000000","0","0x3ff0000000000000"],["24000000000","0","0x3ff0000000000000"],["25000000000","0","0x3ff0000000000000"],["26000000000","0","0x3ff0000000000000"],["27000000000","0","0x3ff0000000000000"],["28000000000","0","0x3ff0000000000000"],["29000000000","0","0x3ff0000000000000"],["30000000000","0","0x3ff0000000000000"],["31000000000","0","0x3ff0000000000000"],["32000000000","0","0x3ff0000000000000"],["33000000000","0","0x3ff0000000000000"],["34000000000","0","0x3ff0000000000000"],["35000000000","0","0x3ff0000000000000"],["36000000000","0","0x3ff0000000000000"],["37000000000","0","0x3ff0000000000000"],["38000000000","0","0x3ff0000000000000"],["39000000000","0","0x3ff0000000000000"],["40000000000","0","0x3ff0000000000000"],["41000000000","0","0x3ff0000000000000"],["42000000000","0","0x3ff0000000000000"],["43000000000","0","0x3ff0000000000000"],["44000000000","0","0x3ff0000000000000"],["45000000000","0","0x3ff0000000000000"],["46000000000","0","0x3ff0000000000000"],["47000000000","0","0x3ff0000000000000"],["48000000000","0","0x3ff0000000000000"],["49000000000","0","0x3ff0000000000000"],["50000000000","0","0x3ff0000000000000"],["51000000000","0","0x3ff0000000000000"],["52000000000","0","0x3ff0000000000000"],["53000000000","0","0x3ff0000000000000"],["54000000000","0","0x3ff0000000000000"],["55000000000","0","0x3ff0000000000000"],["56000000000","0","0x3ff0000000000000"],["57000000000","0","0x3ff0000000000000"],["58000000000","0","0x3ff0000000000000"],["59000000000","0","0x3ff0000000000000"],["60000000000","0","0x3ff0000000000000"],["61000000000","0","0x3ff0000000000000"],["62000000000","0","0x3ff0000000000000"],["63000000000","0","0x3ff0000000000000"]]},{"name":"duplicate_first_pair","rows":[["4095937500000","0","0x3ff0000000000000"],["4095937500000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"duplicate_last_pair","rows":[["4095937500000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"out_of_order_last_pair","rows":[["4095937500000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"],["4095999999999","0","0x3ff0000000000000"]]},{"name":"context_mismatch_first","rows":[["4094000000000","-9223372036854775808","0x3ff0000000000000"],["4095000000000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"context_mismatch_middle","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-9223372036854775808","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"context_mismatch_last","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x3ff0000000000000"],["4096000000000","-9223372036854775808","0x3ff0000000000000"]]},{"name":"context_mismatch_last_of_64","rows":[["0","0","0x3ff0000000000000"],["1000000000","0","0x3ff0000000000000"],["2000000000","0","0x3ff0000000000000"],["3000000000","0","0x3ff0000000000000"],["4000000000","0","0x3ff0000000000000"],["5000000000","0","0x3ff0000000000000"],["6000000000","0","0x3ff0000000000000"],["7000000000","0","0x3ff0000000000000"],["8000000000","0","0x3ff0000000000000"],["9000000000","0","0x3ff0000000000000"],["10000000000","0","0x3ff0000000000000"],["11000000000","0","0x3ff0000000000000"],["12000000000","0","0x3ff0000000000000"],["13000000000","0","0x3ff0000000000000"],["14000000000","0","0x3ff0000000000000"],["15000000000","0","0x3ff0000000000000"],["16000000000","0","0x3ff0000000000000"],["17000000000","0","0x3ff0000000000000"],["18000000000","0","0x3ff0000000000000"],["19000000000","0","0x3ff0000000000000"],["20000000000","0","0x3ff0000000000000"],["21000000000","0","0x3ff0000000000000"],["22000000000","0","0x3ff0000000000000"],["23000000000","0","0x3ff0000000000000"],["24000000000","0","0x3ff0000000000000"],["25000000000","0","0x3ff0000000000000"],["26000000000","0","0x3ff0000000000000"],["27000000000","0","0x3ff0000000000000"],["28000000000","0","0x3ff0000000000000"],["29000000000","0","0x3ff0000000000000"],["30000000000","0","0x3ff0000000000000"],["31000000000","0","0x3ff0000000000000"],["32000000000","0","0x3ff0000000000000"],["33000000000","0","0x3ff0000000000000"],["34000000000","0","0x3ff0000000000000"],["35000000000","0","0x3ff0000000000000"],["36000000000","0","0x3ff0000000000000"],["37000000000","0","0x3ff0000000000000"],["38000000000","0","0x3ff0000000000000"],["39000000000","0","0x3ff0000000000000"],["40000000000","0","0x3ff0000000000000"],["41000000000","0","0x3ff0000000000000"],["42000000000","0","0x3ff0000000000000"],["43000000000","0","0x3ff0000000000000"],["44000000000","0","0x3ff0000000000000"],["45000000000","0","0x3ff0000000000000"],["46000000000","0","0x3ff0000000000000"],["47000000000","0","0x3ff0000000000000"],["48000000000","0","0x3ff0000000000000"],["49000000000","0","0x3ff0000000000000"],["50000000000","0","0x3ff0000000000000"],["51000000000","0","0x3ff0000000000000"],["52000000000","0","0x3ff0000000000000"],["53000000000","0","0x3ff0000000000000"],["54000000000","0","0x3ff0000000000000"],["55000000000","0","0x3ff0000000000000"],["56000000000","0","0x3ff0000000000000"],["57000000000","0","0x3ff0000000000000"],["58000000000","0","0x3ff0000000000000"],["59000000000","0","0x3ff0000000000000"],["60000000000","0","0x3ff0000000000000"],["61000000000","0","0x3ff0000000000000"],["62000000000","0","0x3ff0000000000000"],["63000000000","-9223372036854775808","0x3ff0000000000000"]]},{"name":"context_precedes_order","rows":[["4095937500000","0","0x3ff0000000000000"],["4095937500000","0","0x3ff0000000000000"],["4096000000000","-9223372036854775808","0x3ff0000000000000"]]},{"name":"insufficient_context_checked","rows":[["0","-9223372036854775808","0x3ff0000000000000"]]},{"name":"insufficient_order_checked","rows":[["1","0","0x3ff0000000000000"],["0","0","0x3ff0000000000000"]]},{"name":"insufficient_context_precedes_order","rows":[["1","0","0x3ff0000000000000"],["0","-9223372036854775808","0x3ff0000000000000"]]},{"name":"phase_zero","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_positive_one_q33","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","1","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_negative_one_q33","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-1","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_positive_endpoint","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","8589934592","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_negative_endpoint","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-8589934592","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_positive_outside","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","8589934593","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_negative_outside","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-8589934593","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_precedes_nonfinite_divisor","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","8589934593","0x7ff8000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_lower_endpoint","rows":[["4094000000000","0","0x3fb0000000000000"],["4095000000000","0","0x3fb0000000000000"],["4096000000000","0","0x3fb0000000000000"]]},{"name":"divisor_upper_endpoint","rows":[["4094000000000","0","0x4030000000000000"],["4095000000000","0","0x4030000000000000"],["4096000000000","0","0x4030000000000000"]]},{"name":"divisor_below_lower","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x3fafffffffffffff"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_above_upper","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x4030000000000001"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_positive_zero","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x0000000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_negative_zero","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x8000000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_nan","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x7ff8000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_positive_infinity","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x7ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_negative_infinity","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0xfff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_positive_subnormal","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x0000000000000001"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_negative_subnormal","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0x8000000000000001"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_negative_normal","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","0","0xbff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"context_frame_mismatch","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-9223372036854775807","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"context_generation_mismatch","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-9223372036854775806","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"spread_calculated_below","rows":[["4094000000000","0","0x4030000000000000"],["4095000000000","0","0x3fc041041041040f"],["4096000000000","0","0x4030000000000000"]]},{"name":"spread_calculated_exact","rows":[["4094000000000","0","0x4030000000000000"],["4095000000000","0","0x3fc0410410410410"],["4096000000000","0","0x4030000000000000"]]},{"name":"spread_calculated_above","rows":[["4094000000000","0","0x4030000000000000"],["4095000000000","0","0x3fc0410410410411"],["4096000000000","0","0x4030000000000000"]]},{"name":"phase_positive_inside_neighbor","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","8589934591","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"phase_negative_inside_neighbor","rows":[["4094000000000","0","0x3ff0000000000000"],["4095000000000","-8589934591","0x3ff0000000000000"],["4096000000000","0","0x3ff0000000000000"]]},{"name":"divisor_above_lower_neighbor","rows":[["4094000000000","0","0x3fb0000000000001"],["4095000000000","0","0x3fb0000000000001"],["4096000000000","0","0x3fb0000000000001"]]},{"name":"divisor_below_upper_neighbor","rows":[["4094000000000","0","0x402fffffffffffff"],["4095000000000","0","0x402fffffffffffff"],["4096000000000","0","0x402fffffffffffff"]]}]
''')


def encode_normalization_case(rows):
    """Pack actual numeric time/i64 phase/divisor bits in at most1537 bytes.

    The three lowest i64 phase codes are explicit context sentinels, not product
    encodings. Wide-i128 phases and65-row inputs are outside this fuzz format.
    """
    if (not isinstance(rows, (list, tuple)) or len(rows) > 64
            or any(not isinstance(row, (list, tuple)) or len(row) != 3
                   or any(type(value) is not int for value in row)
                   or not 0 <= row[0] < 2**64 or not -(2**63) <= row[1] < 2**63
                   or not 0 <= row[2] < 2**64 for row in rows)):
        raise Incomplete('Malformed or oversized normalization seed cells.')
    sentinels = {row[1] for row in rows if row[1] <= -(2**63)+2}
    if len(sentinels) > 1:
        raise Incomplete('A numeric fuzz callback may construct at most two contexts.')
    return bytes([len(rows)]) + b''.join(struct.pack('>QqQ', *row) for row in rows)


def fuzz_seeds():
    """Copy57 representable frozen ordinary inputs plus bounded malformed shapes.

    Contractd21c51fc/facts2c165c30 data are transcribed without evaluating output
    formulas. No all61/124 claim: wide-i128 phases,65-row and context-family
    expansion are deliberately outside this bounded test-only encoding.
    """
    result = {'malformed-empty': b'', 'malformed-count65': b'\x41',
              'malformed-truncated': b'\x01', 'malformed-trailing': b'\x00\x00'}
    for fact in NORMALIZATION_SEED_INPUTS:
        rows = [(int(time), int(phase), int(bits, 16)) for time, phase, bits in fact['rows']]
        result['normalization-' + fact['name']] = encode_normalization_case(rows)
    if len(NORMALIZATION_SEED_INPUTS) != 57 or max(map(len, result.values())) > 1537:
        raise Incomplete('Frozen normalization seed selection or encoded bound changed.')
    return result




# SOURCE-ONLY frozen literal inputs/states; no production output selects an oracle.
HISTORY_SEED_CASES = json.loads(r'''[{"name":"empty_immutable_endpoint_labels","source_trace":"empty_immutable_endpoint_labels","source_ordinals":[],"mode":1,"rows":[],"steps":[]},{"name":"partial-append_0","source_trace":"partial_guards_and_labels","source_ordinals":[0],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0}]},{"name":"partial-append_1","source_trace":"partial_guards_and_labels","source_ordinals":[0,1],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1}]},{"name":"partial-source_before_duplicate","source_trace":"partial_guards_and_labels","source_ordinals":[0,1,2],"mode":2,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["19","23","29","1000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"ContextMismatch","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":2}]},{"name":"partial-frame_before_older","source_trace":"partial_guards_and_labels","source_ordinals":[0,1,3],"mode":3,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","31","29","0","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"ContextMismatch","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":3}]},{"name":"partial-generation_before_newer","source_trace":"partial_guards_and_labels","source_ordinals":[0,1,4],"mode":4,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","23","31","2000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"ContextMismatch","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":4}]},{"name":"partial-duplicate","source_trace":"partial_guards_and_labels","source_ordinals":[0,1,5],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"DuplicateTime","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":5}]},{"name":"partial-older","source_trace":"partial_guards_and_labels","source_ordinals":[0,1,6],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","23","29","0","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"OutOfOrderTime","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":6}]},{"name":"partial-healthy_third","source_trace":"partial_guards_and_labels","source_ordinals":[0,1,7],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","23","29","2000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"2000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"2000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7}]},{"name":"origin_zero_positive_quarter","source_trace":"origin_zero_positive_quarter","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","2000000000","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"2000000000","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"2000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2}]},{"name":"origin_u64max_positive_quarter","source_trace":"origin_u64max_positive_quarter","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","18446744071709551615","2147483648","0x3ff0000000000000"],["17","23","29","18446744072709551615","2147483648","0x3ff0000000000000"],["17","23","29","18446744073709551615","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"18446744071709551615","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"18446744072709551615","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"18446744073709551615","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"18446744073709551615","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2}]},{"name":"u64max_single_then_rejections","source_trace":"u64max_single_then_rejections","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","18446744073709551615","0","0x3ff0000000000000"],["17","23","29","18446744073709551615","0","0x3ff0000000000000"],["17","23","29","18446744073709551614","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"18446744073709551615","report":null,"source_ordinal":0},{"kind":"DuplicateTime","index":null,"keep":[0],"reference":"18446744073709551615","report":null,"source_ordinal":1},{"kind":"OutOfOrderTime","index":null,"keep":[0],"reference":"18446744073709551615","report":null,"source_ordinal":2}]},{"name":"minimum_span_neighbors","source_trace":"minimum_span_neighbors","source_ordinals":[0,1,2,3,4],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","31250000","0","0x3ff0000000000000"],["17","23","29","62499999","0","0x3ff0000000000000"],["17","23","29","62500000","0","0x3ff0000000000000"],["17","23","29","62500001","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"31250000","report":null,"source_ordinal":1},{"kind":"Normalization.SpanTooShort","index":null,"keep":[0,1],"reference":"31250000","report":null,"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,3],"reference":"62500000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"62500000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,3,4],"reference":"62500001","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"62500001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4}]},{"name":"two_row_maximum_neighbors","source_trace":"two_row_maximum_neighbors","source_ordinals":[0,1,2,3],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","4096000000001","0","0x3ff0000000000000"],["17","23","29","4095999999999","0","0x3ff0000000000000"],["17","23","29","4096000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Normalization.ElapsedRange","index":0,"keep":[0],"reference":"0","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,2],"reference":"4095999999999","report":null,"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,2,3],"reference":"4096000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"4096000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3}]},{"name":"two_row_exact_maximum_no_prune","source_trace":"two_row_exact_maximum_no_prune","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","4096000000000","0","0x3ff0000000000000"],["17","23","29","4096000000001","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"4096000000000","report":null,"source_ordinal":1},{"kind":"Normalization.ElapsedRange","index":0,"keep":[0,1],"reference":"4096000000000","report":null,"source_ordinal":2}]},{"name":"report_window_failure_then_neighbor","source_trace":"report_window_failure_then_neighbor","source_ordinals":[0,1,2,3,4],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","23","29","2000000000","0","0x3ff0000000000000"],["17","23","29","4096000000001","0","0x3ff0000000000000"],["17","23","29","4096000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"2000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"2000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Normalization.ElapsedRange","index":0,"keep":[0,1,2],"reference":"2000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"2000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,4],"reference":"4096000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"4096000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4}]},{"name":"full64-build-neighbor","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63}]},{"name":"full65-context_before_duplicate_full","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64],"mode":2,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"],["19","23","29","1000000062","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63},{"kind":"ContextMismatch","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":64}]},{"name":"full65-duplicate_full","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,65],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63},{"kind":"DuplicateTime","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":65}]},{"name":"full65-older_full","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,66],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63},{"kind":"OutOfOrderTime","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":66}]},{"name":"full65-window_failure_full","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,67],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"],["17","23","29","4098000000000","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63},{"kind":"Normalization.ElapsedRange","index":0,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":67}]},{"name":"full65-span_failure_full","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,68],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"],["17","23","29","1000000063","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63},{"kind":"Normalization.SpanTooShort","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":68}]},{"name":"full65-healthy_65th","source_trace":"full64_replace_and_failures","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,69],"mode":0,"rows":[["17","23","29","0","2147483648","0x3ff0000000000000"],["17","23","29","1000000000","2147483648","0x3ff0000000000000"],["17","23","29","1000000001","2147483648","0x3ff0000000000000"],["17","23","29","1000000002","2147483648","0x3ff0000000000000"],["17","23","29","1000000003","2147483648","0x3ff0000000000000"],["17","23","29","1000000004","2147483648","0x3ff0000000000000"],["17","23","29","1000000005","2147483648","0x3ff0000000000000"],["17","23","29","1000000006","2147483648","0x3ff0000000000000"],["17","23","29","1000000007","2147483648","0x3ff0000000000000"],["17","23","29","1000000008","2147483648","0x3ff0000000000000"],["17","23","29","1000000009","2147483648","0x3ff0000000000000"],["17","23","29","1000000010","2147483648","0x3ff0000000000000"],["17","23","29","1000000011","2147483648","0x3ff0000000000000"],["17","23","29","1000000012","2147483648","0x3ff0000000000000"],["17","23","29","1000000013","2147483648","0x3ff0000000000000"],["17","23","29","1000000014","2147483648","0x3ff0000000000000"],["17","23","29","1000000015","2147483648","0x3ff0000000000000"],["17","23","29","1000000016","2147483648","0x3ff0000000000000"],["17","23","29","1000000017","2147483648","0x3ff0000000000000"],["17","23","29","1000000018","2147483648","0x3ff0000000000000"],["17","23","29","1000000019","2147483648","0x3ff0000000000000"],["17","23","29","1000000020","2147483648","0x3ff0000000000000"],["17","23","29","1000000021","2147483648","0x3ff0000000000000"],["17","23","29","1000000022","2147483648","0x3ff0000000000000"],["17","23","29","1000000023","2147483648","0x3ff0000000000000"],["17","23","29","1000000024","2147483648","0x3ff0000000000000"],["17","23","29","1000000025","2147483648","0x3ff0000000000000"],["17","23","29","1000000026","2147483648","0x3ff0000000000000"],["17","23","29","1000000027","2147483648","0x3ff0000000000000"],["17","23","29","1000000028","2147483648","0x3ff0000000000000"],["17","23","29","1000000029","2147483648","0x3ff0000000000000"],["17","23","29","1000000030","2147483648","0x3ff0000000000000"],["17","23","29","1000000031","2147483648","0x3ff0000000000000"],["17","23","29","1000000032","2147483648","0x3ff0000000000000"],["17","23","29","1000000033","2147483648","0x3ff0000000000000"],["17","23","29","1000000034","2147483648","0x3ff0000000000000"],["17","23","29","1000000035","2147483648","0x3ff0000000000000"],["17","23","29","1000000036","2147483648","0x3ff0000000000000"],["17","23","29","1000000037","2147483648","0x3ff0000000000000"],["17","23","29","1000000038","2147483648","0x3ff0000000000000"],["17","23","29","1000000039","2147483648","0x3ff0000000000000"],["17","23","29","1000000040","2147483648","0x3ff0000000000000"],["17","23","29","1000000041","2147483648","0x3ff0000000000000"],["17","23","29","1000000042","2147483648","0x3ff0000000000000"],["17","23","29","1000000043","2147483648","0x3ff0000000000000"],["17","23","29","1000000044","2147483648","0x3ff0000000000000"],["17","23","29","1000000045","2147483648","0x3ff0000000000000"],["17","23","29","1000000046","2147483648","0x3ff0000000000000"],["17","23","29","1000000047","2147483648","0x3ff0000000000000"],["17","23","29","1000000048","2147483648","0x3ff0000000000000"],["17","23","29","1000000049","2147483648","0x3ff0000000000000"],["17","23","29","1000000050","2147483648","0x3ff0000000000000"],["17","23","29","1000000051","2147483648","0x3ff0000000000000"],["17","23","29","1000000052","2147483648","0x3ff0000000000000"],["17","23","29","1000000053","2147483648","0x3ff0000000000000"],["17","23","29","1000000054","2147483648","0x3ff0000000000000"],["17","23","29","1000000055","2147483648","0x3ff0000000000000"],["17","23","29","1000000056","2147483648","0x3ff0000000000000"],["17","23","29","1000000057","2147483648","0x3ff0000000000000"],["17","23","29","1000000058","2147483648","0x3ff0000000000000"],["17","23","29","1000000059","2147483648","0x3ff0000000000000"],["17","23","29","1000000060","2147483648","0x3ff0000000000000"],["17","23","29","1000000061","2147483648","0x3ff0000000000000"],["17","23","29","1000000062","2147483648","0x3ff0000000000000"],["17","23","29","1062500000","2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"1000000001","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000001","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"1000000002","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000002","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"1000000003","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000003","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"1000000004","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000004","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"1000000005","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000005","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"1000000006","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000006","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"1000000007","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000007","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"1000000008","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000008","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"1000000009","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000009","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"1000000010","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000010","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"1000000011","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000011","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"1000000012","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000012","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"1000000013","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000013","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"1000000014","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000014","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"1000000015","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000015","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reference":"1000000016","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000016","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"reference":"1000000017","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":17,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000017","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":19,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],"reference":"1000000018","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":18,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000018","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":20,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":19},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20],"reference":"1000000019","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":19,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000019","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":21,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":20},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21],"reference":"1000000020","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":20,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000020","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":22,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":21},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],"reference":"1000000021","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":21,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000021","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":23,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":22},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],"reference":"1000000022","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":22,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000022","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":24,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":23},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24],"reference":"1000000023","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":23,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000023","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":25,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":24},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],"reference":"1000000024","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":24,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000024","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":26,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":25},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26],"reference":"1000000025","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":25,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000025","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":27,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":26},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27],"reference":"1000000026","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":26,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000026","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":28,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":27},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28],"reference":"1000000027","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":27,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000027","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":29,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":28},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],"reference":"1000000028","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":28,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000028","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":30,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":29},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],"reference":"1000000029","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":29,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000029","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":31,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":30},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],"reference":"1000000030","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":30,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000030","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":32,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":31},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32],"reference":"1000000031","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":31,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000031","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":33,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":32},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33],"reference":"1000000032","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":32,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000032","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":34,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":33},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34],"reference":"1000000033","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":33,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000033","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":35,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":34},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35],"reference":"1000000034","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":34,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000034","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":36,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":35},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36],"reference":"1000000035","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":35,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000035","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":37,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":36},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37],"reference":"1000000036","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":36,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000036","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":38,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":37},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38],"reference":"1000000037","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":37,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000037","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":39,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":38},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],"reference":"1000000038","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":38,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000038","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":40,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":39},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],"reference":"1000000039","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":39,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000039","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":41,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":40},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41],"reference":"1000000040","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":40,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000040","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":42,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":41},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42],"reference":"1000000041","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":41,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000041","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":43,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":42},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43],"reference":"1000000042","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":42,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000042","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":44,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":43},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44],"reference":"1000000043","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":43,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000043","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":45,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":44},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45],"reference":"1000000044","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":44,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000044","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":46,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":45},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46],"reference":"1000000045","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":45,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000045","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":47,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":46},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47],"reference":"1000000046","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":46,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000046","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":48,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":47},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48],"reference":"1000000047","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":47,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000047","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":49,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":48},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],"reference":"1000000048","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":48,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000048","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":50,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":49},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50],"reference":"1000000049","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":49,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000049","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":51,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":50},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"reference":"1000000050","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":50,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000050","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":52,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":51},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52],"reference":"1000000051","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":51,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000051","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":53,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":52},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53],"reference":"1000000052","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":52,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000052","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":54,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":53},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54],"reference":"1000000053","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":53,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000053","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":55,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":54},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55],"reference":"1000000054","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":54,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000054","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":56,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":55},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56],"reference":"1000000055","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":55,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000055","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":57,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":56},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57],"reference":"1000000056","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":56,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000056","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":58,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":57},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58],"reference":"1000000057","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":57,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000057","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":59,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":58},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59],"reference":"1000000058","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":58,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000058","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":60,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":59},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],"reference":"1000000059","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":59,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000059","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":61,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":60},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61],"reference":"1000000060","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":60,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000060","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":62,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":61},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62],"reference":"1000000061","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":61,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000061","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":63,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":62},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],"reference":"1000000062","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1000000062","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":63},{"kind":"Ok","index":null,"keep":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64],"reference":"1062500000","report":{"analytic_family":"constant_positive_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":62,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x3fd0000000000000","reference_nanoseconds":"1062500000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":64,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":69}]},{"name":"spread_calculated_below","source_trace":"spread_calculated_below","source_ordinals":[0,1,2,3],"mode":0,"rows":[["17","23","29","4094000000000","0","0x4030000000000000"],["17","23","29","4095000000000","0","0x3fc041041041040f"],["17","23","29","4096000000000","0","0x4030000000000000"],["17","23","29","4096000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"4094000000000","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"4095000000000","report":null,"source_ordinal":1},{"kind":"Normalization.Input.InsufficientSpread","index":null,"keep":[0,1],"reference":"4095000000000","report":null,"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,3],"reference":"4096000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"4096000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3}]},{"name":"spread_calculated_exact","source_trace":"spread_calculated_exact","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","4094000000000","0","0x4030000000000000"],["17","23","29","4095000000000","0","0x3fc0410410410410"],["17","23","29","4096000000000","0","0x4030000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"4094000000000","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"4095000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"4096000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"4096000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2}]},{"name":"spread_calculated_above","source_trace":"spread_calculated_above","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","4094000000000","0","0x4030000000000000"],["17","23","29","4095000000000","0","0x3fc0410410410411"],["17","23","29","4096000000000","0","0x4030000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"4094000000000","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"4095000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"4096000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"4096000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2}]},{"name":"report_spread_failure_then_neighbor","source_trace":"report_spread_failure_then_neighbor","source_ordinals":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","1000000000","0","0x3ff0000000000000"],["17","23","29","2000000000","0","0x3ff0000000000000"],["17","23","29","3000000000","0","0x3ff0000000000000"],["17","23","29","4000000000","0","0x3ff0000000000000"],["17","23","29","5000000000","0","0x3ff0000000000000"],["17","23","29","6000000000","0","0x3ff0000000000000"],["17","23","29","7000000000","0","0x3ff0000000000000"],["17","23","29","8000000000","0","0x3ff0000000000000"],["17","23","29","9000000000","0","0x3ff0000000000000"],["17","23","29","10000000000","0","0x3ff0000000000000"],["17","23","29","11000000000","0","0x3ff0000000000000"],["17","23","29","12000000000","0","0x3ff0000000000000"],["17","23","29","13000000000","0","0x3ff0000000000000"],["17","23","29","14000000000","0","0x3ff0000000000000"],["17","23","29","15000000000","0","0x3ff0000000000000"],["17","23","29","16000000000","0","0x3ff0000000000000"],["17","23","29","4096000000000","0","0x4030000000000000"],["17","23","29","17000000000","0","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"2000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"2000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"3000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"3000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":3},{"kind":"Ok","index":null,"keep":[0,1,2,3,4],"reference":"4000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":3,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"4000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":5,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":4},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5],"reference":"5000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":4,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"5000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":6,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":5},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6],"reference":"6000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":5,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"6000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":7,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":6},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7],"reference":"7000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":6,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"7000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":8,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":7},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8],"reference":"8000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":7,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"8000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":9,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":8},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9],"reference":"9000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":8,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"9000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":10,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":9},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10],"reference":"10000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":9,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"10000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":11,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":10},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11],"reference":"11000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":10,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"11000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":12,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":11},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12],"reference":"12000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":11,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"12000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":13,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":12},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13],"reference":"13000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":12,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"13000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":14,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":13},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],"reference":"14000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":13,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"14000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":15,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":14},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"reference":"15000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":14,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"15000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":16,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":15},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"16000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"16000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":16},{"kind":"Normalization.Input.InsufficientSpread","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],"reference":"16000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":15,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"16000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":17,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":17},{"kind":"Ok","index":null,"keep":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,18],"reference":"17000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":16,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"17000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":18,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":18}]},{"name":"changing_data_same_frame_report","source_trace":"changing_data_same_frame_report","source_ordinals":[0,1,2,3],"mode":0,"rows":[["17","23","29","0","0","0x3ff0000000000000"],["17","23","29","2000000000","0","0x3ff0000000000000"],["17","23","29","3000000000","0","0x3ff0000000000000"],["17","23","29","4000000000","5368709120","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"2000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"3000000000","report":{"analytic_family":"constant_zero","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0x0000000000000000","reference_nanoseconds":"3000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2},{"kind":"Ok","index":null,"keep":[0,1,2,3],"reference":"4000000000","report":{"analytic_family":"changed_four_dyadic","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":2,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x3fb4000000000000","phase_rate_seconds_per_second_bits":"0x3fc0000000000000","phase_seconds_bits":"0x3fd8000000000000","reference_nanoseconds":"4000000000","residual_scale_seconds_squared_bits":"0x3fb4000000000000","sample_count":4,"weighted_sse_seconds_squared_bits":"0x3fc4000000000000"},"source_ordinal":3}]},{"name":"same_frame_negative_quarter","source_trace":"same_frame_negative_quarter","source_ordinals":[0,1,2],"mode":0,"rows":[["17","23","29","0","-2147483648","0x3ff0000000000000"],["17","23","29","1000000000","-2147483648","0x3ff0000000000000"],["17","23","29","2000000000","-2147483648","0x3ff0000000000000"]],"steps":[{"kind":"Ok","index":null,"keep":[0],"reference":"0","report":null,"source_ordinal":0},{"kind":"Ok","index":null,"keep":[0,1],"reference":"1000000000","report":null,"source_ordinal":1},{"kind":"Ok","index":null,"keep":[0,1,2],"reference":"2000000000","report":{"analytic_family":"constant_negative_quarter","context":{"frame":"23","generation":"29","source_id":"17"},"degrees_of_freedom":1,"frame":"23","normalized_data_variance_seconds_squared_bits":"0x0000000000000000","phase_rate_seconds_per_second_bits":"0x0000000000000000","phase_seconds_bits":"0xbfd0000000000000","reference_nanoseconds":"2000000000","residual_scale_seconds_squared_bits":"0x0000000000000000","sample_count":3,"weighted_sse_seconds_squared_bits":"0x0000000000000000"},"source_ordinal":2}]}]''')

CHECKPOINT_SOURCE_RESERVATION = None

if __name__ == '__main__':
    raise SystemExit(main())
