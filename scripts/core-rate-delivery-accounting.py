"""Pure source reservations and literal outcomes for active-time rate delivery.

The frozen55-field overlay adds no numerical producers to the existing package.
The coordinator separately funds one genuine retained-history fixture envelope.
No filesystem, import-side execution, native admission or operation observation
occurs here; even failed or partial reached outcomes retain the full reservation.
"""
from __future__ import annotations

import re
from typing import Literal, TypeGuard, TypedDict


class Incomplete(ValueError):
    """A current source identity, finite recipe or outcome is incomplete."""


MAX_UNITS = (1 << 63) - 1
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
SOURCE_CENSUS_SHA256 = "d59ced2684ec22d4f4344c90f4f10a4a3f0a6ffb63582129faad9f6ad0e51b22"
PACKAGE_OVERLAY_SHA256 = "1d3d0f3c40ae37dac50bb333b7810dc0a97ccd7fa35bbef5789576afd8979db6"
_SOURCE_ROWS: tuple[tuple[str, str], ...] = (
    ("crates/time-core/src/lib.rs", "1c6496d9a4a40f77fec75087dbe1811194fff262117e883f88fad6f90f9bb458"),
    ("crates/time-core/src/reference_input.rs", "8f884d25760160c1ea0e975b6c5640a252bdbdb96e74470aa3d8b72369c4c86d"),
    ("crates/time-core/src/reference_input/rate_delivery.rs", "bd2c0149faf2a59f6e6fff874d214a49b3a37baed5215e6fa3ce77e4f19becfb"),
    ("crates/time-core/src/regression/chart.rs", "d4b635c38a27138a64c83329ae3af25a249fce2f76a0cba53fac030c99cae982"),
    ("crates/time-core/src/regression/chart/roundoff.rs", "afb143ca23cb22e7b095a91ef19bbe6bcb8b18a70c928d6a53ab649d8a56086b"),
    ("crates/time-core/tests/private/chart_roundoff.rs", "bde389e2de19652c4bd3b3a022c42e6c6b34bc95d4930d2a89bf3eb976f6c549"),
    ("crates/time-core/tests/rate_delivery.rs", "b26a75db1dd0203e29d0c31f644d3ba431f30e7672bba29314e5bf7df5813345"),
    ("crates/time-core/tests/private/chart_combination_lifecycle_guards.rs", "f6d69edefe79ee7113c704041b0fbac3eb0a7ce8d86b37079be5fa92f1bacb29"),
)
SOURCE_SHA256 = dict(_SOURCE_ROWS)
_OVERLAY_ROWS: tuple[tuple[str, int], ...] = (
    ("rate.active_duration_order_requests_upper", 55),
    ("rate.active_high_water_checks_upper", 60),
    ("rate.affine_candidate_admissions_upper", 97),
    ("rate.affine_candidate_garde_callbacks_upper", 388),
    ("rate.affine_candidate_typed_scalar_checks_upper", 388),
    ("rate.clock_context_constructions", 27),
    ("rate.compose_attempts_upper", 30),
    ("rate.compose_endpoint_frame_constructions_upper", 30),
    ("rate.compose_outward_add_requests_upper", 360),
    ("rate.compose_outward_multiply_requests_upper", 60),
    ("rate.compose_point_binary_requests_upper", 60),
    ("rate.coordinate_changed_feedback_requests_upper", 30),
    ("rate.cursor_constructions", 16),
    ("rate.cursor_state_context_ordinal_mapping_checks_upper", 76),
    ("rate.direct_numeric_effect_attempts", 21),
    ("rate.direct_numeric_rate_support_checks", 13),
    ("rate.endpoint_admission_frame_constructions_upper", 60),
    ("rate.fixture_completion_borrow_requests_upper", 16),
    ("rate.fixture_completion_move_requests_upper", 19),
    ("rate.fixture_direct_charts", 16),
    ("rate.fixture_direct_domains", 34),
    ("rate.fixture_direct_frames", 65),
    ("rate.fixture_observer_readout_requests_upper", 6768),
    ("rate.integration_checked_point_binary_requests_upper", 102),
    ("rate.integration_integer_conversion_requests_upper", 51),
    ("rate.integration_outward_add_requests_upper", 612),
    ("rate.integration_outward_divide_requests_upper", 51),
    ("rate.integration_outward_multiply_requests_upper", 357),
    ("rate.interval_constructor_attempts", 33),
    ("rate.interval_garde_callbacks_upper", 33),
    ("rate.interval_garde_validations_upper", 33),
    ("rate.interval_rate_support_requests_upper", 66),
    ("rate.known_effect_attempts_upper", 30),
    ("rate.known_endpoint_and_target_commits_upper", 30),
    ("rate.known_relationship_passes_upper", 60),
    ("rate.monotonic_high_water_checks_upper", 60),
    ("rate.monotonic_order_requests_upper", 54),
    ("rate.new_compile_fail_doc_compilations_upper", 6),
    ("rate.new_doctest_executed_examples", 0),
    ("rate.ordinal_consumptions_upper", 38),
    ("rate.receiver_attempts", 38),
    ("rate.reference_identity_checks_upper", 76),
    ("rate.reference_owner_constructions", 24),
    ("rate.settlement_garde_callbacks_upper", 38),
    ("rate.settlement_garde_validations_upper", 38),
    ("rate.settlement_relationship_passes_upper", 76),
    ("rate.span_constructor_attempts", 27),
    ("rate.span_garde_callbacks_upper", 27),
    ("rate.span_garde_validations_upper", 27),
    ("rate.span_relationship_passes_upper", 54),
    ("rate.total_new_outward_bound_requests_upper", 1440),
    ("rate.total_new_outward_next_up_requests_upper", 1440),
    ("rate.total_rate_support_requests_upper", 130),
    ("rate.total_settled_effect_attempts_upper", 51),
    ("rate.unknown_feedback_requests_upper", 1),
)
FIELDS = tuple(name for name, _ in _OVERLAY_ROWS)
CASE_FIELDS = (
    "span_constructions", "interval_constructions", "cursor_constructions",
    "reference_owner_constructions", "receiver_attempts",
    "direct_numeric_effect_attempts", "direct_numeric_rate_support_checks",
    "genuine_history_fixture_envelopes",
)
# Literal source entries, not successful-path observations. In particular all38
# possible receivers reserve both admission passes and every later known prefix.
# The final existing lifecycle consumer adds one direct effect and64 observer
# requests. Its exact comparator/two cached anchor projections are funded only
# through the separate zero-producer486-field fixture delta, not a second fit.
_CASE_ROWS: tuple[tuple[str, tuple[int, ...]], ...] = (
    ("dyadic_delivery_and_cancellation_use_active_time_only", (5, 5, 5, 5, 5, 0, 0, 0)),
    ("span_and_rate_admission_preserve_valid_neighbors", (3, 6, 1, 1, 1, 0, 0, 0)),
    ("noeffect_and_unknown_have_distinct_terminal_behavior", (1, 1, 1, 3, 5, 0, 0, 0)),
    ("cursor_carries_identity_and_nonoverlap_across_replacement_owners", (9, 9, 1, 4, 11, 0, 0, 0)),
    ("current_owner_and_chart_relationships_are_required_before_consumption", (1, 4, 2, 1, 6, 0, 0, 0)),
    ("admitted_failures_and_exhaustion_cannot_be_retried", (7, 7, 5, 8, 9, 0, 0, 0)),
    ("settled_translation_preserves_genuine_retained_histories_and_credit", (1, 1, 1, 2, 1, 0, 0, 1)),
    ("regression::chart::roundoff::tests::settled_rate_preserves_general_binary64_integration_error", (0, 0, 0, 0, 0, 2, 0, 0)),
    ("regression::chart::roundoff::tests::settled_rate_keeps_dyadic_and_signed_zero_points", (0, 0, 0, 0, 0, 8, 0, 0)),
    ("regression::chart::roundoff::tests::settled_rate_uses_exact_active_duration_and_integer_neighbors", (0, 0, 0, 0, 0, 6, 0, 0)),
    ("regression::chart::roundoff::tests::settled_rate_rejects_unsupported_rates_and_reachable_results", (0, 0, 0, 0, 0, 4, 13, 0)),
    ("combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_preserve_complete_relationships", (0, 0, 0, 0, 0, 1, 0, 0)),
)
CURRENT_ORDINARY_TESTS = tuple(name for name, _ in _CASE_ROWS)
_DOC_PATH = "crates/time-core/src/reference_input/rate_delivery.rs"
_DOC_ROWS = (
    ("FixtureRateSpan", 190), ("FixtureRateSpan", 197),
    ("FixtureSettledRateInterval", 252), ("FixtureRateSettlementCursor", 383),
    ("FixtureRateSettlementCompletion", 482),
    ("FixtureReferenceInputState<'_>::receive_rate_settlement", 649),
)
DOCTESTS = tuple(
    _DOC_PATH + " - reference_input::rate_delivery::" + item
    + " (line " + str(line) + ") - compile fail" for item, line in _DOC_ROWS
)
_OWNED_PREFIXES = (
    "rate_delivery::", "regression::chart::roundoff::tests::settled_rate_",
    "dyadic_delivery_", "span_and_rate_", "noeffect_and_unknown_",
    "cursor_carries_", "current_owner_and_chart_", "admitted_failures_",
    "settled_translation_",
    "combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_",
)
_OUTCOMES = ("ok", "FAILED", "ignored")
Status = Literal["ok", "FAILED", "ignored", "missing"]


class Observation(TypedDict):
    """Closed assertion statuses, kept distinct from prospective source work."""
    rate_test_statuses: dict[str, Status]
    rate_source_only_pending: bool
    rate_source_test_observation: Literal["complete", "incomplete"]
    rate_test_missing: list[str]
    rate_test_failed: list[str]
    rate_literal_doctests: list[str]
    rate_doctest_statuses: dict[str, Status]
    rate_doctest_pending: bool
    rate_doctest_missing: list[str]
    rate_doctest_failed: list[str]
    rate_recorded_source_work_upper: dict[str, int] | None
    observed_operation_counts: None


def _exact_dict(value: object) -> TypeGuard[dict[str, object]]:
    """Narrow a runtime mapping without introducing an untyped import seam."""
    return type(value) is dict


def _integer(value: object) -> int:
    """Admit exact bounded source units, excluding Python Boolean integers."""
    if type(value) is not int or not 0 <= value <= MAX_UNITS:
        raise Incomplete("Rate source units require exact nonnegative bounded integers.")
    return value


def _boolean(value: object) -> bool:
    """Modes must be actual Booleans, not truthy producer data."""
    if type(value) is not bool:
        raise Incomplete("Rate accounting mode requires an exact Boolean.")
    return value


def _overlay(include_doctests: bool) -> dict[str, int]:
    """Check the literal finite inventory before publishing fresh reservations."""
    docs = _boolean(include_doctests)
    result: dict[str, int] = {}
    for name, value in _OVERLAY_ROWS:
        if (type(name) is not str or not name.startswith("rate.") or name in result
                or len(name) > 128):
            raise Incomplete("Rate source field is unknown or duplicated.")
        result[name] = _integer(value)
    if len(result) != 55 or tuple(result) != FIELDS:
        raise Incomplete("Rate source field inventory is incomplete.")
    if not docs:
        result["rate.new_compile_fail_doc_compilations_upper"] = 0
    return result


def package_overlay(*, include_doctests: bool = False) -> dict[str, int]:
    """Return only the new55-field source overlay, never numerical fit credit."""
    return _overlay(include_doctests)


def validate_overlay(value: object, *, include_doctests: bool = False) -> dict[str, int]:
    """Reject missing, unknown, discounted or changed package reservations."""
    expected = _overlay(include_doctests)
    if not _exact_dict(value) or set(value) != set(expected):
        raise Incomplete("Rate package reservation has unknown or missing fields.")
    checked = {name: _integer(value[name]) for name in expected}
    if checked != expected:
        raise Incomplete("Rate package reservation differs from its complete source recipe.")
    return checked


def case_recipes() -> dict[str, dict[str, int]]:
    """Return twelve source classifications; the last adds no new Rust test."""
    rows: dict[str, dict[str, int]] = {}
    for name, values in _CASE_ROWS:
        if (type(name) is not str or name in rows or len(name) > 192
                or len(values) != len(CASE_FIELDS)):
            raise Incomplete("Rate source case inventory is duplicated or incomplete.")
        rows[name] = {field: _integer(value)
                      for field, value in zip(CASE_FIELDS, values, strict=True)}
    if len(rows) != 12 or tuple(rows) != CURRENT_ORDINARY_TESTS:
        raise Incomplete("Rate source test inventory is incomplete.")
    return rows


def validate_case_recipes(value: object) -> dict[str, dict[str, int]]:
    """Receive the complete exact source census before any policy projection."""
    expected = case_recipes()
    if not _exact_dict(value) or set(value) != set(expected):
        raise Incomplete("Rate source recipes have unknown or missing tests.")
    checked: dict[str, dict[str, int]] = {}
    for name, recipe in expected.items():
        row = value[name]
        if not _exact_dict(row) or set(row) != set(CASE_FIELDS):
            raise Incomplete("A rate source recipe has unknown or missing fields.")
        checked[name] = {field: _integer(row[field]) for field in CASE_FIELDS}
        if checked[name] != recipe:
            raise Incomplete("A rate source recipe differs from its frozen census.")
    return checked


def admit_sources(actual: dict[str, str]) -> dict[str, str]:
    """Compare all eight current hashes; source hashing/custody stays with caller."""
    expected = dict(_SOURCE_ROWS)
    if type(actual) is not dict or actual != expected:
        raise Incomplete("A rate source dependency changed or is missing.")
    return expected


def _status(value: str) -> Status:
    """Close literal native statuses; neither partial data nor aliases pass."""
    if value == "ok":
        return "ok"
    if value == "FAILED":
        return "FAILED"
    if value == "ignored":
        return "ignored"
    raise Incomplete("A rate source outcome has an unknown status.")


def profile_observation(
    output: str, *, require_complete: bool = False, require_doctests: bool = False,
) -> Observation:
    """Receive exact complete libtest lines; retain failed/ignored/missing axes.

    Other package rows are outside this owner. Unknown rate-owned names and
    literal doctest signatures fail closed. Nextest summaries, JSON and replay
    prose are not libtest outcome observations. No inner operation is measured.
    """
    _boolean(require_complete)
    _boolean(require_doctests)
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS:
        raise Incomplete("Rate outcome input requires bounded exact text.")
    ordinary: dict[str, Status] = {}
    docs: dict[str, Status] = {}
    for raw in output.splitlines(keepends=True):
        line = raw[:-2] if raw.endswith("\r\n") else raw[:-1] if raw.endswith("\n") else raw
        if not line.startswith("test "):
            continue
        body = line[5:]
        is_doc = _DOC_PATH in body or "reference_input::rate_delivery::" in body
        name = body.split(" ", 1)[0]
        is_ordinary = name in CURRENT_ORDINARY_TESTS or name.startswith(_OWNED_PREFIXES)
        if not is_doc and not is_ordinary:
            continue
        if not raw.endswith("\n"):
            raise Incomplete("A rate source outcome lacks its complete line delimiter.")
        match = re.fullmatch(r"(.+) \.\.\. (ok|FAILED|ignored)", body)
        if match is None:
            raise Incomplete("A rate source outcome is malformed.")
        identity, status = match.groups()
        target = docs if is_doc else ordinary
        expected = DOCTESTS if is_doc else CURRENT_ORDINARY_TESTS
        if identity not in expected or identity in target:
            raise Incomplete("A rate source outcome is duplicated or unknown.")
        target[identity] = _status(status)
    missing = [name for name in CURRENT_ORDINARY_TESTS if name not in ordinary]
    failed = [name for name in CURRENT_ORDINARY_TESTS
              if name in ordinary and ordinary[name] != "ok"]
    doc_missing = [name for name in DOCTESTS if name not in docs]
    doc_failed = [name for name in DOCTESTS if name in docs and docs[name] != "ok"]
    ordinary_complete = not missing and not failed
    docs_complete = not doc_missing and not doc_failed
    if require_complete and not ordinary_complete:
        raise Incomplete("The twelve actual rate source outcomes remain incomplete.")
    if require_doctests and not docs_complete:
        raise Incomplete("The six actual rate compile-fail outcomes remain incomplete.")
    complete = ordinary_complete and (not require_doctests or docs_complete)
    return {
        "rate_test_statuses": {name: ordinary.get(name, "missing") for name in CURRENT_ORDINARY_TESTS},
        "rate_source_only_pending": not complete,
        "rate_source_test_observation": "complete" if complete else "incomplete",
        "rate_test_missing": missing, "rate_test_failed": failed,
        "rate_literal_doctests": [name for name in DOCTESTS if name in docs],
        "rate_doctest_statuses": {name: docs.get(name, "missing") for name in DOCTESTS},
        "rate_doctest_pending": not docs_complete,
        "rate_doctest_missing": doc_missing, "rate_doctest_failed": doc_failed,
        "rate_recorded_source_work_upper": _overlay(bool(docs)) if ordinary or docs else None,
        "observed_operation_counts": None,
    }
