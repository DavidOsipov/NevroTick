"""Bounded S1.3 counter observations, separate from native outcomes and charges.

This module performs no I/O and launches no tools. The caller owns source/tool
custody, selected test/native-outcome checks and pre-execution reservation. A
complete counter recipe is not behavioral GREEN, a verified mutation kill or a
measurement of private algorithm work. Missing scopes never become zero calls.
Public-call and direct raw-helper records are separate contracts. A complete
package caller checks both; neither parser infers the other's observations.
"""
from __future__ import annotations

import re


class Incomplete(ValueError):
    """The supplied observation does not satisfy the closed transport contract."""


U64_MAX = (1 << 64) - 1
MAX_OUTPUT_CHARACTERS = 16 * 1024 * 1024
MAX_MARKER_CHARACTERS = 1024
EXTERNAL_MARKER = "S13_SELECTION_PROFILE"
GUARD_MARKER = "S13_BATCH_GUARD_PROFILE"
HELPER_MARKER = "SELECTION_HELPER_PROFILE"
CALL_FIELDS = ("intervals", "minima", "batches", "selections")
HELPER_FIELDS = ("read_adapters", "prefix_sorts", "cell_depths", "label_projections")
HELPER_RECIPE = (2, 3, 2, 5)
EXTERNAL_RECIPES = (
    ("interval", (8, 0, 0, 0)),
    ("minimum", (0, 6, 0, 0)),
    ("batch", (19, 1, 8, 1)),
    ("literal_permutations", (446, 65, 65, 65)),
)
GUARD_RECIPES = (
    ("capacity", (1, 1)),
    ("capacity_neighbor", (16, 1)),
    ("duplicate_recovery", (2, 2)),
)
EXTERNAL_TESTS = (
    "interval_construction_rejects_invalid_raw_values_and_accepts_neighbors",
    "minimum_construction_preserves_valid_values",
    "batch_admission_rejects_duplicate_and_capacity_before_recovery",
    "interval_majority_literal_cases_and_permutations",
)
GUARD_TESTS = (
    "selection::selection_guards::over_capacity_rejects_before_adapter_reads",
    "selection::selection_guards::capacity_neighbor_copies_all_values_in_order",
    "selection::selection_guards::duplicate_stops_at_earliest_index_and_recovers",
)
PACKAGE_ORDINARY_TESTS = 94
PACKAGE_DOCTESTS = 2
PACKAGE_BASELINE_TESTS = 96

_DECIMAL = r"(0|[1-9][0-9]{0,19})"


def _progress_prefix(names):
    """Allow only one exact selected libtest progress prefix for this binary."""
    return r"(?:test (?:" + "|".join(re.escape(name) for name in names) + r") \.\.\. )?"


_EXTERNAL_LINE = re.compile(
    _progress_prefix(EXTERNAL_TESTS) + EXTERNAL_MARKER + r" scope=([a-z_]+)"
    + "".join(" " + field + "=" + _DECIMAL for field in CALL_FIELDS)
)
_GUARD_LINE = re.compile(
    _progress_prefix(GUARD_TESTS) + GUARD_MARKER
    + r' \{"case":"([a-z_]+)","intervals":' + _DECIMAL
    + r',"batches":' + _DECIMAL + r"\}"
)
_HELPER_LINE = re.compile(
    _progress_prefix(GUARD_TESTS) + HELPER_MARKER
    + "".join(" " + field + "=" + _DECIMAL for field in HELPER_FIELDS)
)


def _u64(value):
    """Decode an already lexically bounded canonical decimal, with no coercion."""
    result = int(value)
    if result > U64_MAX:
        raise Incomplete("Selection counter exceeds its exact u64 representation.")
    return result


def _validate_external(scope, counts):
    """Reject unknown scopes, over-recipe calls and impossible simple ordering.

    A partial record is a bounded reported prefix, not proof of every internal
    path taken. Native failed-test causes remain the caller's separate owner.
    """
    recipes = dict(EXTERNAL_RECIPES)
    if (scope not in recipes or not any(counts)
            or any(value > maximum for value, maximum in zip(counts, recipes.get(scope, ()), strict=True))):
        raise Incomplete("Selection scope or public-attempt prefix differs from its fixed recipe.")
    intervals, minima, batches, selections = counts
    if scope == "batch":
        allowed = (
            (batches == 0 and minima == selections == 0 and 1 <= intervals <= 3)
            or (intervals == 3 and minima == selections == 0 and 1 <= batches <= 6)
            or (4 <= intervals <= 19 and minima == selections == 0 and batches == 6)
            or (intervals == 19 and minima == selections == 0 and batches in (7, 8))
            or (intervals == 19 and minima == 1 and batches == 8 and selections in (0, 1))
        )
        if not allowed:
            raise Incomplete("Batch-recovery counter ordering differs from its checked-call recipe.")
    elif scope == "literal_permutations":
        if not (batches > 0 and selections <= minima <= batches and batches - selections <= 1):
            raise Incomplete("Literal evaluation counters differ from constructor/batch/minimum ordering.")


def _validate_guard(case, counts):
    """Validate finite constructor/batch prefixes, without inventing read counts."""
    if case not in dict(GUARD_RECIPES):
        raise Incomplete("Unknown selection receiver case.")
    intervals, batches = counts
    if case == "capacity":
        allowed = intervals == 1 and batches in (0, 1)
    elif case == "capacity_neighbor":
        allowed = (1 <= intervals <= 16 and batches == 0) or (intervals == 16 and batches == 1)
    else:
        allowed = (intervals == 1 and batches == 0) or (intervals == 2 and batches in (0, 1, 2))
    if not allowed:
        raise Incomplete("Selection receiver counter prefix differs from its fixed recipe.")


def _recorded_subtotals(external, guards):
    """Sum present records only; None means no record, not zero observed work."""
    if not external and not guards:
        return None
    totals = {field: sum(row[index] for row in external.values()) for index, field in enumerate(CALL_FIELDS)}
    totals["intervals"] += sum(row[0] for row in guards.values())
    totals["batches"] += sum(row[1] for row in guards.values())
    return totals


def selection_counts(output, *, require_complete=False):
    """Parse complete native marker lines from one source-bound package stream.

    Accept LF or CRLF and the exact selected binary's libtest progress prefix.
    Do not search substrings, unescape compiler JSON or strip arbitrary text.
    Duplicate, malformed, unknown, over-recipe and truncated records reject even
    when a valid neighbor is present. Unrelated lines without markers are left
    to the caller's native phase/cause checks.

    Complete means all seven records match their fixed healthy call recipes;
    native success, all 96 baseline tests and actual assertions are not inferred.
    Missing records set pending state and observed_calls=None. Present partial
    records retain their reported subtotal with explicit partial/missing labels.
    They never reduce the caller's prior full-package reservation.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Selection observations require bounded text and a Boolean completeness mode.")
    external, guards = {}, {}
    lines = output.split("\n")
    for index, raw in enumerate(lines):
        if EXTERNAL_MARKER not in raw and GUARD_MARKER not in raw:
            continue
        if index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Selection marker lacks its complete bounded native line.")
        line = raw[:-1] if raw.endswith("\r") else raw
        match = _EXTERNAL_LINE.fullmatch(line)
        if match:
            scope, *values = match.groups()
            counts = tuple(_u64(value) for value in values)
            _validate_external(scope, counts)
            if scope in external:
                raise Incomplete("Duplicate selection scope observation.")
            external[scope] = counts
            continue
        match = _GUARD_LINE.fullmatch(line)
        if match:
            case, *values = match.groups()
            counts = tuple(_u64(value) for value in values)
            _validate_guard(case, counts)
            if case in guards:
                raise Incomplete("Duplicate selection receiver observation.")
            guards[case] = counts
            continue
        raise Incomplete("Selection counter marker is malformed or outside its exact native prefix.")
    missing_external = [name for name, _ in EXTERNAL_RECIPES if name not in external]
    missing_guards = [name for name, _ in GUARD_RECIPES if name not in guards]
    partial_external = [name for name, recipe in EXTERNAL_RECIPES if name in external and external[name] != recipe]
    partial_guards = [name for name, recipe in GUARD_RECIPES if name in guards and guards[name] != recipe]
    pending = bool(missing_external or missing_guards)
    complete = not pending and not partial_external and not partial_guards
    recorded = _recorded_subtotals(external, guards)
    result = {
        "selection_external_records": len(external),
        "selection_guard_records": len(guards),
        "selection_external_profiles": {name: dict(zip(CALL_FIELDS, counts, strict=True)) for name, counts in external.items()},
        "selection_guard_profiles": {name: {"intervals": counts[0], "batches": counts[1]} for name, counts in guards.items()},
        "selection_missing_external_scopes": missing_external,
        "selection_missing_guard_cases": missing_guards,
        "selection_partial_external_scopes": partial_external,
        "selection_partial_guard_cases": partial_guards,
        "selection_observation_pending": pending,
        "selection_counter_observation_complete": complete,
        "selection_observation_status": "complete-source-recipe" if complete else "partial-source-recipe" if recorded is not None else "missing",
        "selection_recorded_call_subtotals": recorded,
        "selection_observed_calls": recorded.copy() if not pending else None,
        "selection_observation_scope": "Reported public attempts only; counter completeness proves no native outcome or private operation count.",
        "selection_batch_read_observation": "Adapter-read attempts are not transported; reached native assertions and source review own that evidence.",
    }
    if require_complete and not complete:
        raise Incomplete("Selection package observations omit scopes or differ from the complete healthy recipe.")
    return result


def full_package_reservation():
    """Return one fresh canonical source reservation before a package invocation.

    These are source-derived maxima for the unchanged test recipe, not already
    observed work, a global quota or a bound on arbitrary mutant internals.
    Native wall/process/resource controls remain separate caller obligations.
    """
    return {"intervals": 492, "minima": 72, "batches": 77, "selections": 66,
            "garde_validator_invocations_upper": 1056}


def _validate_helper(counts):
    """Admit only finite prefixes of the direct raw-helper call order.

    The helper scope is created immediately before its first precharged read.
    It then attempts two reads, three sorts, two depths and five projections.
    These are direct test calls, not counts of calls made inside each helper.
    """
    if not any(counts) or any(value > limit for value, limit in zip(counts, HELPER_RECIPE, strict=True)):
        raise Incomplete("Raw-helper attempts exceed or precede their fixed source recipe.")
    reads, sorts, depths, labels = counts
    allowed = (
        (1 <= reads <= 2 and sorts == depths == labels == 0)
        or (reads == 2 and 1 <= sorts <= 3 and depths == labels == 0)
        or (reads == 2 and sorts == 3 and 1 <= depths <= 2 and labels == 0)
        or (reads == 2 and sorts == 3 and depths == 2 and 1 <= labels <= 5)
    )
    if not allowed:
        raise Incomplete("Raw-helper attempt ordering differs from its fixed source recipe.")


def helper_counts(output, *, require_complete=False):
    """Parse the one direct raw-helper record independently of public calls.

    The caller supplies source-bound native test bytes after trusted Cargo JSON
    phase separation. Exact LF/CRLF lines and the three owned library progress
    prefixes are admitted. Missing output remains None and pending; a reached
    failed-test prefix is reported as partial, without reducing prior charges.
    The source recipe covers twelve direct helper attempts, not recursively
    invoked helpers, total private operations, native success or CPU/RSS.
    Complete-package callers require this and selection_counts independently.
    """
    if type(output) is not str or len(output) > MAX_OUTPUT_CHARACTERS or type(require_complete) is not bool:
        raise Incomplete("Raw-helper observations require bounded text and a Boolean completeness mode.")
    counts = None
    lines = output.split("\n")
    for index, raw in enumerate(lines):
        if HELPER_MARKER not in raw:
            continue
        if index == len(lines) - 1 or len(raw) > MAX_MARKER_CHARACTERS:
            raise Incomplete("Raw-helper marker lacks its complete bounded native line.")
        line = raw[:-1] if raw.endswith("\r") else raw
        match = _HELPER_LINE.fullmatch(line)
        if match is None:
            raise Incomplete("Raw-helper marker is malformed or outside its exact native prefix.")
        if counts is not None:
            raise Incomplete("Duplicate raw-helper scope observation.")
        counts = tuple(_u64(value) for value in match.groups())
        _validate_helper(counts)
    pending = counts is None
    complete = counts == HELPER_RECIPE
    observed = None if pending else dict(zip(HELPER_FIELDS, counts, strict=True))
    result = {
        "selection_helper_records": 0 if pending else 1,
        "selection_helper_observed_calls": observed,
        "selection_helper_observation_pending": pending,
        "selection_helper_counter_observation_complete": complete,
        "selection_helper_observation_status": "complete-source-recipe" if complete else "missing" if pending else "partial-source-recipe",
        "selection_helper_observation_scope": "Reported precharged direct raw-helper attempts only; no public construction, native outcome or recursive/private work count is inferred.",
    }
    if require_complete and not complete:
        raise Incomplete("Raw-helper package observation is missing or differs from the complete healthy recipe.")
    return result


def full_package_helper_reservation():
    """Return fresh source-derived direct-helper charges before a package run.

    No public constructor/evaluator call is added. The successful label helper
    calls sorting internally; that is deliberately outside these direct-attempt
    counters. This reservation is no global quota or arbitrary-mutant work bound.
    """
    return dict(zip(HELPER_FIELDS, HELPER_RECIPE, strict=True))


def source_work_uppers(evaluator_attempts, batch_attempts):
    """Project canonical implementation bounds; never label them observations.

    Callers may use the full reservation before execution or an explicitly
    reported subset afterward. Missing observations must not be supplied as zero.
    Counts and every product remain exact nonnegative u64 values. Endpoint
    collection writes exclude sorting and deduplication scratch writes. The
    checked algorithm's private bounds do not measure CPU/RSS, guards or mutant
    work. Direct raw-helper test attempts have their separate source reservation.
    """
    if any(type(value) is not int or not 0 <= value <= U64_MAX for value in (evaluator_attempts, batch_attempts)):
        raise Incomplete("Selection source-bound projection requires exact nonnegative u64 counts.")
    factors = {
        "selection_endpoint_collection_writes_upper": (32, evaluator_attempts),
        "selection_endpoint_sort_comparisons_upper": (496, evaluator_attempts),
        "selection_endpoint_dedup_comparisons_upper": (31, evaluator_attempts),
        "selection_cell_visits_upper": (63, evaluator_attempts),
        "selection_membership_probes_upper": (1008, evaluator_attempts),
        "selection_component_visits_upper": (63, evaluator_attempts),
        "selection_witness_probes_upper": (16, evaluator_attempts),
        "selection_label_sort_comparisons_upper": (120, evaluator_attempts),
        "selection_intersection_reductions_upper": (30, evaluator_attempts),
        "selection_batch_copy_adapter_attempts_upper": (16, batch_attempts),
        "selection_batch_label_comparisons_upper": (120, batch_attempts),
    }
    result = {name: factor * count for name, (factor, count) in factors.items()}
    if any(value > U64_MAX for value in result.values()):
        raise Incomplete("Selection source-bound projection exceeds exact u64 arithmetic.")
    return result
