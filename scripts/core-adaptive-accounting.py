"""Bounded source accounting for retained/adaptive fixture tests.

Only caller-supplied native counter text is parsed. Observed attempts, conservative
source reservations, test success and native outcomes are distinct. This module
launches no tools, performs no I/O, and makes no arbitrary-mutant work claim.
"""
from __future__ import annotations

import re


class Incomplete(ValueError):
    """A bounded source inventory or a native counter record is incomplete."""


INPUTS = (
    "scripts/core-adaptive-accounting.py",
    "scripts/tests/test_core_adaptive_accounting.py",
    "crates/time-core/tests/retained_runs.rs",
    "crates/time-core/tests/adaptive_history.rs",
    "crates/time-core/tests/private/lifecycle_counter.rs",
    "crates/time-core/tests/private/adaptive_history_guards.rs",
)
FIELDS = (
    "retained_points", "retained_inputs", "retained_selections",
    "adaptive_observations", "adaptive_appends", "adaptive_histories",
    "raw_parameter_validations", "raw_ordinal_validations", "raw_event_validations",
    "external_normalizations",
)
U64_MAX = (1 << 64) - 1
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 1024
_RETAINED = "CORE_RETAINED_RUNS_PROFILE"
_ADAPTIVE = "CORE_ADAPTIVE_HISTORY_PROFILE"
_RAW = "CORE_LIFECYCLE_RAW_VALIDATION_PROFILE"
# Frozen literal owner-call recipes; each native row is separately named.
_CASES = {
    "retained_prefix_preserves_second_round_information": (_RETAINED, (37, 3, 3)),
    "zero_prefix_preserves_legacy_selection": (_RETAINED, (13, 1, 2)),
    "invalid_prefix_and_minimum_boundaries": (_RETAINED, (11, 2, 7)),
    "maximum_combined_window_and_prefix_boundary": (_RETAINED, (64, 1, 2)),
    "adaptive_retains_decision_prefix_across_two_rounds": (_ADAPTIVE, (13, 13, 1)),
    "configured_minimum_warms_up_without_claiming_eligibility": (_ADAPTIVE, (3, 3, 6)),
    "rejected_appends_preserve_complete_state_and_recover": (_ADAPTIVE, (20, 20, 1)),
    "selector_failure_preserves_cause_and_committed_history": (_ADAPTIVE, (13, 13, 1)),
    "adaptive_successful_compaction_preserves_decision_rows": (_ADAPTIVE, (12, 12, 1)),
    "adaptive_minimum_fallback_commits_full_checked_window": (_ADAPTIVE, (9, 9, 1)),
    "full_history_evicts_retained_auxiliary_first": (_ADAPTIVE, (65, 65, 1)),
    "full_fitted_history_evicts_oldest_fitted_row": (_ADAPTIVE, (65, 65, 1)),
    "lifecycle_raw_parameter_validation": (_RAW, (26,)),
    "lifecycle_raw_ordinal_validation": (_RAW, (3,)),
    "lifecycle_raw_event_validation": (_RAW, (8,)),
}
_ROW_FIELDS = {
    _RETAINED: ("points", "inputs", "selections"),
    _ADAPTIVE: ("observations", "appends", "histories"),
    _RAW: ("validations",),
}
_RAW_FIELDS = {
    "lifecycle_raw_parameter_validation": "raw_parameter_validations",
    "lifecycle_raw_ordinal_validation": "raw_ordinal_validations",
    "lifecycle_raw_event_validation": "raw_event_validations",
}
_RAW_PREFIX = "combination::lifecycle::counter_tests::"
_DECIMAL = r"(0|[1-9][0-9]{0,19})"
SOURCE_ONLY_TESTS = tuple("regression::adaptive_history::raw_minimum_guards::" + case for case in (
    "adaptive_raw_minimum_semantic_neighbors",
    "adaptive_combined_prefix_semantic_neighbors",
    "adaptive_warmup_postconditions_reject_wrong_arguments",
    "adaptive_complete_postconditions_bind_configured_minimum",
    "adaptive_complete_postconditions_bind_original_auxiliary",
    "adaptive_unpublished_three_rows_require_report",
    "adaptive_unpublished_report_requires_same_context",
    "adaptive_unpublished_report_requires_same_reference",
))
SOURCE_TESTS = tuple(
    ("retained_runs::" if marker == _RETAINED else
     "adaptive_history::" if marker == _ADAPTIVE else _RAW_PREFIX) + case
    for case, (marker, _) in _CASES.items()
) + SOURCE_ONLY_TESTS
# Binding must be checked against the separately maintained external source.
EXTERNAL_LEGAL_SOURCE_SHA256 = "e33303f800c2358de41beb526958c98978c4e30f89408878ca32406761d32998"


def case_recipes() -> dict[str, tuple[int, ...]]:
    """Return a fresh closed case-to-complete-attempt vector, without effects."""
    return {case: values for case, (_, values) in _CASES.items()}


def case_ceilings() -> dict[str, tuple[int, ...]]:
    """Return source-fixed per-case prefix ceilings, independent of observations."""
    return case_recipes()


def _limit_calls(calls: object) -> dict[str, int]:
    """Admit exactly the bounded source-call vector, excluding booleans."""
    if type(calls) is not dict or set(calls) != set(FIELDS):
        raise Incomplete("Adaptive source calls have a missing or extra field.")
    maxima = _complete_calls()
    maxima["external_normalizations"] = 1
    if any(type(calls[key]) is not int or not 0 <= calls[key] <= maxima[key] for key in FIELDS):
        raise Incomplete("Adaptive source calls exceed their exact source ceilings.")
    return dict(calls)


def _complete_calls() -> dict[str, int]:
    """Sum the fixed disjoint named source recipes without native observations."""
    total = dict.fromkeys(FIELDS, 0)
    for case, (marker, values) in _CASES.items():
        if marker == _RETAINED:
            for key, value in zip(FIELDS[:3], values, strict=True):
                total[key] += value
        elif marker == _ADAPTIVE:
            for key, value in zip(FIELDS[3:6], values, strict=True):
                total[key] += value
        else:
            total[_RAW_FIELDS[case]] += values[0]
    return total


def source_work(calls: object, *, include_raw_minimum: bool = False) -> dict[str, int]:
    """Project conservative canonical source work from admitted public attempts.

    The optional private tests are source-only, not observed attempts. Their
    genuine 25 appends, 29 observations and seven history constructors enter the
    same producer recurrence once. The genuine raw-event fixture is reserved
    once if its first raw call was reached. Callback/predicate counts overlap
    their respective validation calls.
    """
    if type(include_raw_minimum) is not bool:
        raise Incomplete("Adaptive raw-minimum reservation requires a Boolean scope.")
    c = _limit_calls(calls)
    private_scope = int(include_raw_minimum)
    private_appends = 25 * private_scope
    private_observations = 29 * private_scope
    private_histories = 7 * private_scope
    appends = c["adaptive_appends"] + private_appends
    selections = c["retained_selections"] + appends
    normalizations = appends + c["external_normalizations"]
    observations = c["adaptive_observations"] + private_observations
    histories = c["adaptive_histories"] + private_histories
    event_fixture = int(c["raw_event_validations"] > 0)
    raw_minimum = 6 if include_raw_minimum else 0
    return {
        "charged_fit_work_upper": 62 * selections,
        "point_admissions_upper": c["retained_points"] + 64 * normalizations + 2077 * selections,
        "retained_direct_points_upper": c["retained_points"],
        "retained_direct_inputs_upper": c["retained_inputs"],
        "selector_entries_upper": selections,
        "selector_fit_upper": 62 * selections,
        "selector_point_copy_upper": 2077 * selections,
        "selector_input_slots_upper": 3968 * selections,
        "selector_residual_slots_upper": 3968 * selections,
        "selector_decision_residuals_upper": 3098 * selections,
        "selector_checked_float_upper": 53540 * selections,
        "adaptive_normalizations_upper": normalizations,
        "adaptive_generated_points_upper": 64 * normalizations,
        "adaptive_duration_ops_upper": 64 * normalizations,
        "adaptive_fit_inputs_upper": normalizations,
        "adaptive_normalized_input_slots_upper": 64 * normalizations,
        "retained_direct_input_slots_upper": 64 * c["retained_inputs"],
        "direct_and_generated_point_validations_upper": c["retained_points"] + 64 * normalizations,
        "direct_and_generated_point_rule_callbacks_upper": 3 * (c["retained_points"] + 64 * normalizations),
        "adaptive_stage_init_slots_upper": 64 * appends,
        "adaptive_stage_retained_copies_upper": 63 * appends,
        "adaptive_compaction_copies_upper": 63 * appends,
        # Logical fixed-array ownership transfers at stage/finish/commit boundaries;
        # actual machine moves, elision and physical memory traffic are not measured.
        "adaptive_array_transfer_slots_upper": 4 * 64 * appends,
        "adaptive_complete_context_checks_upper": 64 * appends,
        "adaptive_complete_order_checks_upper": 63 * appends,
        "adaptive_observation_validations_upper": observations,
        "adaptive_observation_rule_callbacks_upper": 2 * observations,
        "adaptive_minimum_validations_upper": histories + raw_minimum,
        "adaptive_minimum_rule_callbacks_upper": histories + raw_minimum,
        "adaptive_minimum_predicates_upper": 2 * histories + raw_minimum,
        # Closed public variants and two actual wrapped errors are source-only
        # test work; their delegated displays/source projections overlap these.
        "adaptive_error_display_requests_upper": 7 if include_raw_minimum else 0,
        "adaptive_error_nested_display_calls_upper": 2 if include_raw_minimum else 0,
        "adaptive_error_source_requests_upper": 9 if include_raw_minimum else 0,
        "adaptive_error_source_downcasts_upper": 3 if include_raw_minimum else 0,
        # These seven direct private tests have no attempt marker. Setup uses only
        # genuine checked owners; unpublished candidates are raw inputs.
        # Their producer work is already included once in appends/observations.
        "adaptive_private_prefix_calls_upper": 11 * private_scope,
        "adaptive_private_final_validator_calls_upper": 24 * private_scope,
        "adaptive_private_context_constructions_upper": 9 * private_scope,
        "adaptive_private_observation_constructions_upper": private_observations,
        "adaptive_private_append_calls_upper": private_appends,
        "adaptive_private_history_constructions_upper": private_histories,
        "adaptive_private_monotonic_constructions_upper": 44 * private_scope,
        "adaptive_private_utc_constructions_upper": 58 * private_scope,
        "adaptive_private_exchange_calls_upper": 29 * private_scope,
        "adaptive_private_getter_calls_upper": 50 * private_scope,
        "adaptive_private_context_checks_upper": 83 * private_scope,
        "adaptive_private_order_checks_upper": 59 * private_scope,
        "adaptive_private_array_init_slots_upper": 128 * private_scope,
        "adaptive_private_array_row_writes_upper": 2 * private_scope,
        "adaptive_private_array_transfer_slots_upper": 256 * private_scope,
        "raw_lifecycle_validations_upper": (c["raw_parameter_validations"]
                                            + c["raw_ordinal_validations"] + c["raw_event_validations"]),
        "raw_lifecycle_rule_callbacks_upper": (2 * c["raw_parameter_validations"]
                                               + c["raw_ordinal_validations"] + c["raw_event_validations"]),
        "raw_event_fixture_entries_upper": event_fixture,
        "raw_event_projected_batches_upper": event_fixture,
        "raw_event_principal_parameters_upper": event_fixture,
        "raw_event_combination_parameters_upper": event_fixture,
        "raw_event_minima_upper": event_fixture,
        "raw_event_seeds_upper": event_fixture,
        "raw_event_update_constructions_upper": 9 * event_fixture,
        "raw_event_record_calls_upper": event_fixture,
        "raw_event_source_state_constructions_upper": 2 * event_fixture,
        "raw_event_owner_admission_predicate_calls_upper": 2 * event_fixture,
        "raw_event_lifecycle_garde_validations_upper": 13 * event_fixture,
        "raw_event_lifecycle_garde_callbacks_upper": 14 * event_fixture,
        "raw_event_state_clones_upper": 2 * event_fixture,
        "raw_event_state_equalities_upper": 8 * event_fixture,
        "raw_event_source_lookups_upper": event_fixture,
    }


def full_package_reservation() -> dict[str, int]:
    """Reserve all named source recipes before a whole package invocation."""
    calls = _complete_calls()
    return {**{"adaptive.call." + key: value for key, value in calls.items()},
            **{"adaptive.work." + key: value for key, value in source_work(calls, include_raw_minimum=True).items()}}


def literal_doctest_reservation() -> dict[str, int]:
    """Reserve the one executable adaptive doctest; compile-fail does not execute."""
    return {"adaptive_history_constructions_upper": 1, "adaptive_minimum_validations_upper": 1,
            "adaptive_minimum_rule_callbacks_upper": 1, "adaptive_minimum_predicates_upper": 2,
            "adaptive_history_getter_requests_upper": 3,
            "charged_fit_work_upper": 0, "point_admissions_upper": 0}


def external_legal_reservation() -> dict[str, int]:
    """Reserve the fixed legal consumer; caller must bind its exact source digest.

    The separately guarded auxiliary profile owns context, clock, exchange and
    getter calls. This vector excludes package-only raw guards and event fixture.
    """
    calls = dict.fromkeys(FIELDS, 0)
    calls.update(retained_selections=2, adaptive_observations=3,
                 adaptive_appends=3, adaptive_histories=1, external_normalizations=1)
    return {**{"adaptive.call." + key: value for key, value in calls.items()},
            **{"adaptive.work." + key: value for key, value in source_work(calls).items()}}


def package_counts(output: object, *, require_complete: bool = False) -> dict[str, object]:
    """Parse named rows with source caps and distinguish partial from complete.

    Missing and partial observations never establish test outcomes. Lines with a
    marker require exact fields, a bounded complete newline and unique case ID.
    Optional libtest prefixes must name the same case. An all-zero row is invalid.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Adaptive observations require bounded text and Boolean completeness.")
    recorded: dict[str, tuple[int, ...]] = {}
    for line in output.splitlines(keepends=True):
        markers = [marker for marker in _ROW_FIELDS if marker in line]
        if not markers:
            continue
        if len(markers) != 1 or not line.endswith("\n") or len(line) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Adaptive marker is mixed, truncated or oversized.")
        marker = markers[0]
        text = line.removesuffix("\n").removesuffix("\r")
        pattern = (r"(?:(?:test (?P<test>[a-zA-Z0-9_:]+) \.\.\. ))?" + marker
                   + r" case=(?P<case>[a-z_]+)"
                   + "".join(" " + field + "=" + _DECIMAL for field in _ROW_FIELDS[marker]))
        match = re.fullmatch(pattern, text)
        if match is None:
            raise Incomplete("Adaptive marker has malformed fields or prefix.")
        case = match.group("case")
        row = _CASES.get(case)
        if row is None or row[0] != marker or case in recorded:
            raise Incomplete("Adaptive case is unknown, mismatched or duplicated.")
        test = match.group("test")
        qualified = ((_RAW_PREFIX if marker == _RAW else
                      "retained_runs::" if marker == _RETAINED else "adaptive_history::") + case)
        if test is not None and test not in (case, qualified):
            raise Incomplete("Adaptive libtest prefix does not name its source case.")
        values = tuple(int(value) for value in match.groups()[2:])
        if (len(values) != len(row[1]) or not any(values)
                or any(value > U64_MAX or value > maximum
                       for value, maximum in zip(values, row[1], strict=True))):
            raise Incomplete("Adaptive case has zero or exceeds its source ceiling.")
        if marker == _ADAPTIVE and (values[1] > values[0] or (values[0] and not values[2])):
            raise Incomplete("Adaptive attempts violate observation/history call order.")
        if marker == _RETAINED and values[2] > 8 * values[1]:
            raise Incomplete("Retained selector attempts exceed checked input attempts.")
        recorded[case] = values
    missing = [case for case in _CASES if case not in recorded]
    partial = [case for case, values in recorded.items() if values != _CASES[case][1]]
    complete = not missing and not partial
    if require_complete and not complete:
        raise Incomplete("Adaptive complete package is missing or has partial cases.")
    calls = dict.fromkeys(FIELDS, 0)
    for case, values in recorded.items():
        marker = _CASES[case][0]
        keys = FIELDS[:3] if marker == _RETAINED else FIELDS[3:6] if marker == _ADAPTIVE else (_RAW_FIELDS[case],)
        for key, value in zip(keys, values, strict=True):
            calls[key] += value
    profiles = {case: dict(zip(_ROW_FIELDS[_CASES[case][0]], values, strict=True))
                for case, values in recorded.items()}
    return {"adaptive_profiles": profiles, "adaptive_missing_cases": missing,
            "adaptive_partial_cases": partial, "adaptive_observation_complete": complete,
            "adaptive_recorded_calls": calls if recorded else None,
            "adaptive_observed_calls": dict(calls) if complete else None,
            "adaptive_observation_scope": ("Named public-attempt records; external_normalizations=0 is the "
                                           "source-bound package scope, not observed telemetry."),
            "adaptive_recorded_source_work_upper": source_work(calls, include_raw_minimum=True) if recorded else None}
