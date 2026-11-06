"""Closed preparation registry for additional bounded mathematical proof suites.

This registry executes no compiler or solver and accepts no historical receipt.
The runner freezes input_names(), loads projections from that frozen tree and
retains its existing admission, namespace, resource and qualification controls.
"""
from __future__ import annotations

from pathlib import Path
import importlib.util


MAX_JOBS = 12
PARTITION_SELECTOR = "fraction-addition-partitions"
PARTITION_SUBJECTS = {"fraction_addition_partition_union", *(
    "fraction_addition_partition_" + str(index) for index in range(6))}


def _partition_projection():
    """Load only this closed source-bound ordinary suite; execute no proof."""
    path = Path(__file__).with_name("fraction_addition_partitions_projection.py")
    spec = importlib.util.spec_from_file_location("formal_partition_registry_projection", path)
    if spec is None or spec.loader is None:
        raise ValueError("Partition projection is unavailable")
    projection = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(projection)
    return projection


def _partition_jobs() -> list[dict]:
    """Reject partial, reordered or extended schedules for the new suite only."""
    result = _partition_projection().schedule()
    expected = [("fraction_addition_partition_union", "healthy", False)]
    expected += [("fraction_addition_partition_" + str(i), "healthy", False) for i in range(6)]
    expected += [("fraction_addition_partition_fault", case, False)
                 for case in ("always_error", "wrong_carry", "wrong_residual")]
    expected += [("fraction_addition_partition_" + str(i), "healthy", True) for i in range(6)]
    if not isinstance(result, list) or len(result) != 16:
        raise ValueError("Partition suite requires exactly sixteen fixed jobs")
    for job, (owner, case, restored) in zip(result, expected, strict=True):
        if (not isinstance(job, dict) or set(job) != _FIELDS
                or (job["subject"], job["case"], job["restored_healthy"]) != (owner, case, restored)
                or type(job["restored_healthy"]) is not bool):
            raise ValueError("Partition suite has a reordered or malformed job")
        if case == "healthy":
            suffix, failure = owner, None
        else:
            suffix, failure = {
                "always_error": ("fraction_addition_control_success", "addition required success"),
                "wrong_carry": ("fraction_addition_control_carry", "addition exact carry"),
                "wrong_residual": ("fraction_addition_control_residual", "addition exact residual"),
            }[case]
        if (job["harness"] != "fraction_addition_partitions::" + suffix
                or job["intended_failure"] != failure):
            raise ValueError("Partition harness or intended failure differs from fixed schedule")
    return result


_STEMS = {
    "coordinate-exchange": "coordinate_exchange",
    "clock": "clock",
    "leap-calendar": "leap_calendar",
    "relative-range": "relative_range",
    "exchange-composition": "exchange_composition",
    "calendar-composition": "calendar_composition",
    "leap-ticks": "leap_ticks",
    "calendar-finite-laws": "calendar_finite_laws",
    "leap-coverage": "leap_coverage",
    "endpoint-finish": "endpoint_finish",
    "fraction-constructor": "fraction_constructor",
    "fraction-comparison": "fraction_comparison",
    "fraction-arithmetic": "fraction_arithmetic",
    "scaled-value-split": "scaled_value_split",
    "scaled-value-representation": "scaled_value_representation",
    "fraction-addition-partitions": "fraction_addition_partitions",
}
_FIELDS = {"subject", "case", "harness", "intended_failure", "restored_healthy"}


def input_names(selector: str) -> list[str]:
    """Return the selected proof-code closure, including this registry itself."""
    if type(selector) is not str or selector not in _STEMS:
        raise ValueError("Unknown additional proof suite")
    stem = _STEMS[selector]
    result = ["tools/formal/additional_suites.py", f"tools/formal/{stem}_harness.rs",
              f"tools/formal/{stem}_projection.py"]
    if selector == "calendar-finite-laws":
        result.extend(("tools/formal/calendar_composition_harness.rs",
                       "tools/formal/calendar_composition_projection.py"))
    if selector in {"leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation"}:
        result.append(f"tools/formal/test_{stem}.py")
    if selector == PARTITION_SELECTOR:
        result.extend(("tools/formal/fraction_arithmetic_harness.rs",
                       "tools/formal/fraction_arithmetic_projection.py",
                       "tools/formal/test_fraction_addition_partitions.py"))
    return result


def jobs(selector: str) -> list[dict]:
    """Return one complete immutable-in-meaning schedule as fresh job records."""
    input_names(selector)
    if selector == PARTITION_SELECTOR:
        return _partition_jobs()
    result = []

    def add(subject, case, harness, intended=None, restored=False):
        result.append({"subject": subject, "case": case, "harness": harness,
                       "intended_failure": intended, "restored_healthy": restored})

    if selector == "coordinate-exchange":
        add("coordinate", "healthy", "coordinate_full_domain")
        add("exchange", "healthy", "exchange_full_domain")
        add("coordinate", "always_error", "coordinate_control_zero", "required coordinate success")
        add("coordinate", "wrong_coordinate", "coordinate_control_retained", "exact retained coordinate")
        add("coordinate", "wrong_error", "coordinate_control_rejection", "coordinate rejection classification")
        add("exchange", "always_error", "exchange_control_zero", "required exchange success")
        add("exchange", "wrong_offset", "exchange_control_phase", "exact phase numerator")
        add("exchange", "wrong_rtt", "exchange_control_round_trip", "exact signed round trip")
        add("coordinate", "healthy", "coordinate_full_domain", restored=True)
        add("exchange", "healthy", "exchange_full_domain", restored=True)
    elif selector == "clock":
        add("clock", "healthy", "monotonic_full_domain")
        add("clock", "healthy", "active_awake_full_domain")
        add("clock", "monotonic_always_error", "monotonic_control_zero", "monotonic duration required success")
        add("clock", "monotonic_wrong_duration", "monotonic_control_duration", "monotonic duration exact result")
        add("clock", "monotonic_wrong_error", "monotonic_control_overflow", "monotonic addition overflow classification")
        add("clock", "active_awake_always_error", "active_awake_control_zero", "active-awake duration required success")
        add("clock", "active_awake_wrong_duration", "active_awake_control_duration", "active-awake duration exact result")
        add("clock", "active_awake_wrong_error", "active_awake_control_overflow", "active-awake addition overflow classification")
        add("clock", "healthy", "monotonic_full_domain", restored=True)
        add("clock", "healthy", "active_awake_full_domain", restored=True)
    elif selector == "leap-calendar":
        add("leap_calendar", "healthy", "leap_calendar_full_domain")
        add("leap_calendar", "healthy", "leap_calendar_oracle_laws")
        add("leap_calendar", "reject_month_start", "leap_calendar_control_success", "required month-start success")
        add("leap_calendar", "omit_alignment", "leap_calendar_control_alignment", "sub-day rejection")
        add("leap_calendar", "omit_century_adjustment", "leap_calendar_control_century", "century month-start boundary")
        add("leap_calendar", "healthy", "leap_calendar_full_domain", restored=True)
        add("leap_calendar", "healthy", "leap_calendar_oracle_laws", restored=True)
    elif selector == "relative-range":
        add("relative_range", "healthy", "relative_full_domain")
        add("relative_range", "reject_all", "relative_control_success", "required relative success")
        add("relative_range", "accept_all", "relative_control_upper", "upper relative rejection")
        add("relative_range", "widen_lower_edge", "relative_control_lower", "lower relative rejection")
        add("relative_range", "healthy", "relative_full_domain", restored=True)
    elif selector == "exchange-composition":
        for harness in ("exchange_phase_full_domain", "exchange_rtt_full_domain", "exchange_scaling_full_domain"):
            add("exchange_composition", "healthy", harness)
        add("exchange_composition", "always_error", "exchange_composition_control_success", "phase required success")
        add("exchange_composition", "wrong_phase", "exchange_composition_control_phase", "exact phase numerator")
        add("exchange_composition", "wrong_rtt", "exchange_composition_control_rtt", "exact signed round trip")
        add("exchange_composition", "wrong_scale", "exchange_composition_control_scaling", "bounded scaling succeeds")
        for harness in ("exchange_phase_full_domain", "exchange_rtt_full_domain", "exchange_scaling_full_domain"):
            add("exchange_composition", "healthy", harness, restored=True)
    elif selector == "calendar-composition":
        harnesses = ("calendar_source_full_domain", "calendar_representation_full_domain",
                     "calendar_suffix_full_domain", "calendar_gregorian_laws")
        for harness in harnesses:
            add("calendar_composition", "healthy", harness)
        add("calendar_composition", "reject_month_start", "calendar_composition_control_success", "required month-start success")
        add("calendar_composition", "omit_alignment", "calendar_composition_control_alignment", "sub-day rejection")
        add("calendar_composition", "omit_century_adjustment", "calendar_composition_control_century", "century month-start boundary")
        for harness in harnesses:
            add("calendar_composition", "healthy", harness, restored=True)
    elif selector == "leap-ticks":
        add("leap_ticks", "healthy", "leap_ticks_full_domain")
        add("leap_ticks", "return_zero", "leap_ticks_control_nonzero", "nonzero second retained")
        add("leap_ticks", "half_scale", "leap_ticks_control_scale", "whole-second fractional alignment")
        add("leap_ticks", "truncate_seconds", "leap_ticks_control_high_bits", "high second bits retained")
        add("leap_ticks", "healthy", "leap_ticks_full_domain", restored=True)
    elif selector == "leap-coverage":
        add("leap_coverage", "healthy", "leap_coverage_full_domain")
        add("leap_coverage", "include_exclusive_end", "leap_coverage_control_exclusive_end",
            "exclusive completeness endpoint remains unknown")
        add("leap_coverage", "exclude_continuous_start", "leap_coverage_control_continuous_start",
            "continuous start is included for events")
        add("leap_coverage", "bridge_one_second_gap", "leap_coverage_control_gap",
            "unbridged gap remains unknown")
        add("leap_coverage", "healthy", "leap_coverage_full_domain", restored=True)
    elif selector == "endpoint-finish":
        add("endpoint_finish", "healthy", "endpoint_finish::endpoint_finish_full_domain")
        add("endpoint_finish", "reject_all", "endpoint_finish::endpoint_finish_control_success",
            "endpoint required success")
        add("endpoint_finish", "omit_negative_floor", "endpoint_finish::endpoint_finish_control_negative_floor",
            "endpoint exact negative floor")
        add("endpoint_finish", "range_as_invariant", "endpoint_finish::endpoint_finish_control_range_error",
            "endpoint range rejection classification")
        add("endpoint_finish", "healthy", "endpoint_finish::endpoint_finish_full_domain", restored=True)
    elif selector == "fraction-constructor":
        add("fraction_constructor", "healthy", "fraction_constructor::fraction_constructor_full_domain")
        add("fraction_constructor", "always_error", "fraction_constructor::fraction_constructor_control_success",
            "fraction required success")
        add("fraction_constructor", "wrong_numerator", "fraction_constructor::fraction_constructor_control_numerator",
            "fraction exact numerator retained")
        add("fraction_constructor", "wrong_error", "fraction_constructor::fraction_constructor_control_error",
            "fraction rejection classification")
        add("fraction_constructor", "healthy", "fraction_constructor::fraction_constructor_full_domain", restored=True)
    elif selector == "fraction-comparison":
        add("fraction_comparison", "healthy", "fraction_comparison::fraction_comparison_full_domain")
        add("fraction_comparison", "always_error", "fraction_comparison::fraction_comparison_control_success",
            "comparison required success")
        add("fraction_comparison", "reverse_unequal", "fraction_comparison::fraction_comparison_control_unequal",
            "comparison strict unequal order")
        add("fraction_comparison", "zero_alignment", "fraction_comparison::fraction_comparison_control_alignment",
            "comparison unreduced equality")
        add("fraction_comparison", "healthy", "fraction_comparison::fraction_comparison_full_domain", restored=True)
    elif selector == "fraction-arithmetic":
        add("fraction_complement", "healthy", "fraction_arithmetic::fraction_complement_full_domain")
        add("fraction_complement", "always_error", "fraction_arithmetic::fraction_complement_control_success",
            "complement required success")
        add("fraction_complement", "wrong_numerator", "fraction_arithmetic::fraction_complement_control_numerator",
            "complement exact numerator relation")
        add("fraction_complement", "wrong_error", "fraction_arithmetic::fraction_complement_control_error",
            "complement rejection classification")
        add("fraction_complement", "healthy", "fraction_arithmetic::fraction_complement_full_domain", restored=True)
        add("fraction_addition", "healthy", "fraction_arithmetic::fraction_addition_full_domain")
        add("fraction_addition", "always_error", "fraction_arithmetic::fraction_addition_control_success",
            "addition required success")
        add("fraction_addition", "wrong_carry", "fraction_arithmetic::fraction_addition_control_carry",
            "addition exact carry")
        add("fraction_addition", "wrong_residual", "fraction_arithmetic::fraction_addition_control_residual",
            "addition exact residual")
        add("fraction_addition", "healthy", "fraction_arithmetic::fraction_addition_full_domain", restored=True)
    elif selector == "scaled-value-split":
        add("scaled_value_split", "healthy", "scaled_value_split::scaled_value_split_full_domain")
        add("scaled_value_split", "reject_all", "scaled_value_split::scaled_value_split_control_success",
            "split required success")
        add("scaled_value_split", "zero_remainder", "scaled_value_split::scaled_value_split_control_remainder",
            "split exact quotient remainder")
        add("scaled_value_split", "wrong_error", "scaled_value_split::scaled_value_split_control_error",
            "split rejection classification")
        add("scaled_value_split", "healthy", "scaled_value_split::scaled_value_split_full_domain", restored=True)
    elif selector == "scaled-value-representation":
        add("signed_whole", "healthy", "scaled_value_representation::signed_whole_full_domain")
        add("signed_whole", "omit_negation", "scaled_value_representation::signed_whole_control_sign",
            "signed whole strict sign")
        add("signed_whole", "narrow_magnitude", "scaled_value_representation::signed_whole_control_magnitude",
            "signed whole exact magnitude")
        add("signed_whole", "use_fraction", "scaled_value_representation::signed_whole_control_fraction",
            "signed whole fraction independence")
        add("signed_whole", "healthy", "scaled_value_representation::signed_whole_full_domain", restored=True)
        add("from_bits", "healthy", "scaled_value_representation::from_bits_full_domain")
        add("from_bits", "wrong_subnormal_scale", "scaled_value_representation::from_bits_control_subnormal",
            "from bits nonzero shift retained")
        add("from_bits", "lose_sign", "scaled_value_representation::from_bits_control_sign",
            "from bits original sign retained")
        add("from_bits", "wrong_error", "scaled_value_representation::from_bits_control_error",
            "from bits rejection classification")
        add("from_bits", "healthy", "scaled_value_representation::from_bits_full_domain", restored=True)
    else:
        add("calendar_finite_laws", "healthy", "calendar_finite_laws_full_domain")
        add("calendar_finite_laws", "cycle_year_coefficient", "calendar_finite_laws_control_year", "Gregorian segment year recurrence")
        add("calendar_finite_laws", "march_prefix_length", "calendar_finite_laws_control_month", "Gregorian March month recurrence")
        add("calendar_finite_laws", "forward_epoch_coefficient", "calendar_finite_laws_control_epoch", "NTP March epoch anchor")
        add("calendar_finite_laws", "healthy", "calendar_finite_laws_full_domain", restored=True)
    if not 1 <= len(result) <= MAX_JOBS:
        raise ValueError("Additional proof schedule exceeds its finite command bound")
    return result


def expected_contract(subject: str, harness: str) -> tuple[frozenset[str], frozenset[str]]:
    """Require fixed nonempty successful assertions and covers for a full proof.

    Control harnesses have their separate fixed intended_failure job contract;
    they cannot be substituted for a healthy full-domain proof here.
    """
    if type(subject) is not str or type(harness) is not str:
        raise ValueError("Unknown additional healthy proof contract")
    if subject in {"fraction_addition_universal", "fraction_addition_witnesses"}:
        if (subject, harness) not in {
                ("fraction_addition_universal", "fraction_arithmetic::fraction_addition_universal_full_domain"),
                ("fraction_addition_witnesses", "fraction_arithmetic::fraction_addition_witnesses_full_domain")}:
            raise ValueError("Query split requires its exact universal or witness harness")
        path = Path(__file__).with_name("fraction_query_split_projection.py")
        spec = importlib.util.spec_from_file_location("formal_fraction_query_split_registry_projection", path)
        if spec is None or spec.loader is None:
            raise ValueError("Query split projection is unavailable")
        projection = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(projection)
        return projection.expected_contract(subject, harness)
    if subject == "fraction_composition_mechanism":
        if harness != "fraction_arithmetic::composition_mechanism_full_domain":
            raise ValueError("Composition mechanism requires its exact full-domain harness")
        path = Path(__file__).with_name("fraction_composition_projection.py")
        spec = importlib.util.spec_from_file_location("formal_fraction_composition_registry_projection", path)
        if spec is None or spec.loader is None:
            raise ValueError("Composition projection is unavailable")
        projection = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(projection)
        return projection.expected_contract(subject, harness)
    if subject in PARTITION_SUBJECTS:
        if harness != "fraction_addition_partitions::" + subject:
            raise ValueError("Partition harness is not the selected complete leaf")
        projection = _partition_projection()
        assertions = projection.REQUIRED_ASSERTIONS[harness]
        covers = projection.REQUIRED_COVERS[harness]
        if not assertions or not covers:
            raise ValueError("Partition proofs require reachable assertions and covers")
        return frozenset(assertions), frozenset(covers)
    if (subject, harness) == ("coordinate", "coordinate_full_domain"):
        assertions = {"coordinate rejection classification", "exact retained coordinate"}
        covers = {"zero coordinate succeeds", "minimum coordinate succeeds", "maximum coordinate succeeds",
                  "lower coordinate neighbor is rejected", "upper coordinate neighbor is rejected",
                  "raw signed minimum is rejected", "raw signed maximum is rejected"}
    elif (subject, harness) == ("exchange", "exchange_full_domain"):
        assertions = {"exact phase numerator", "exact signed round trip", "phase producer envelope",
                      "round trip producer envelope", "phase nanosecond scaling fits", "round trip nanosecond scaling fits"}
        covers = {"zero exchange", "positive phase endpoint", "negative phase endpoint",
                  "positive round trip endpoint", "negative round trip endpoint", "odd phase and negative round trip"}
    elif subject == "clock" and harness in {"monotonic_full_domain", "active_awake_full_domain"}:
        label = "monotonic" if harness == "monotonic_full_domain" else "active-awake"
        assertions = {f"{label} {name}" for name in (
            "reading exact", "duration exact result", "reverse classification", "elapsed admission exact",
            "addition exact success", "addition overflow classification")}
        covers = {f"{label} {name}" for name in (
            "zero endpoints", "maximum duration", "reversed endpoints", "maximum plus zero",
            "full-span addition", "first addition overflow")}
    elif (subject, harness) == ("leap_calendar", "leap_calendar_full_domain"):
        assertions = {"Gregorian month-start equivalence"}
        covers = {"NTP epoch is a month start", "sub-day value is rejected", "1972 leap day is rejected",
                  "2100 March starts after an ordinary February", "month start above signed-i64 is covered",
                  "raw u64 maximum is covered"}
    elif (subject, harness) == ("leap_calendar", "leap_calendar_oracle_laws"):
        assertions = {"Gregorian year recurrence", "Gregorian month recurrence", "January begins the year",
                      "400-year period length", "arbitrary-cycle forward correspondence",
                      "1900 January NTP epoch anchor", "NTP epoch cycle position"}
        covers = {"ordinary leap year is covered", "non-400 century is covered",
                  "400-year leap century is covered", "arbitrary cycle-count maximum is covered"}
    elif (subject, harness) == ("relative_range", "relative_full_domain"):
        assertions = {"lower relative rejection", "upper relative rejection", "required relative success"}
        covers = {"relative zero is admitted", "lower relative endpoint is admitted",
                  "upper relative endpoint is admitted", "lower relative neighbor is rejected",
                  "upper relative neighbor is rejected", "relative raw minimum is rejected",
                  "relative raw maximum is rejected"}
    elif subject == "exchange_composition" and harness == "exchange_phase_full_domain":
        assertions = {"phase required success", "exact phase numerator", "phase producer envelope"}
        covers = {"phase zero exchange", "phase positive endpoint", "phase negative endpoint", "phase odd half-tick"}
    elif subject == "exchange_composition" and harness == "exchange_rtt_full_domain":
        assertions = {"round trip required success", "exact signed round trip", "round trip producer envelope"}
        covers = {"round trip zero exchange", "round trip positive endpoint", "round trip negative endpoint", "round trip negative tick"}
    elif subject == "exchange_composition" and harness == "exchange_scaling_full_domain":
        assertions = {"bounded scaling succeeds"}
        covers = {"scaling zero succeeds", "scaling positive caller endpoint", "scaling negative caller endpoint", "scaling interior succeeds"}
    elif subject == "calendar_composition" and harness == "calendar_source_full_domain":
        assertions = {"prefix alignment result", "prefix whole-day envelope", "prefix epoch conversion fits", "prefix positive normalization envelope", "prefix exact epoch mapping", "prefix cycle domain"}
        covers = {"source NTP epoch succeeds", "source subday rejects", "source upper word rejects", "source high month succeeds"}
    elif subject == "calendar_composition" and harness == "calendar_representation_full_domain":
        assertions = {"forward search year bracket", "cycle day has forward representation", "forward ordinal domain"}
        covers = {"forward first day represented", "forward final day represented", "forward century day represented", "forward leap February start represented"}
    elif subject == "calendar_composition" and harness == "calendar_suffix_full_domain":
        assertions = {"generated cycle domain", "generated suffix month equivalence"}
        covers = {"suffix cycle origin succeeds", "suffix century March succeeds", "suffix final leap day rejects", "suffix February start succeeds"}
    elif subject == "calendar_composition" and harness == "calendar_gregorian_laws":
        assertions = {"Gregorian segment year recurrence", "Gregorian March month recurrence", "March prefix origin", "March segment total length", "Gregorian cycle period", "arbitrary cycle forward law", "NTP March epoch anchor", "NTP cycle epoch position"}
        covers = {"laws ordinary leap month", "laws non-400 century March", "laws 400-year century March", "laws maximum cycle model"}
    elif subject == "leap_ticks" and harness == "leap_ticks_full_domain":
        assertions = {"whole seconds remain nonnegative", "full u64 Q32 fits signed i128", "exact unsigned Q32 encoding", "zero fractional bits", "full second bits retained", "coordinate admission iff scalar envelope", "exact coordinate ticks retained", "outside coordinate error classification"}
        covers = {"zero whole second is covered", "one whole second retains Q32 units", "leap baseline scalar is covered", "post-u32 second bits are covered", "signed-i64 whole-second endpoint is covered", "first outside whole-second neighbor is covered", "raw u64 maximum remains represented"}
    elif subject == "calendar_finite_laws" and harness == "calendar_finite_laws_full_domain":
        assertions = {"Gregorian segment year recurrence", "Gregorian March month recurrence", "March prefix origin", "March segment total length", "Gregorian cycle period", "NTP March epoch anchor", "NTP cycle epoch position"}
        covers = {"laws ordinary leap month", "laws non-400 century March", "laws 400-year century March", "laws maximum cycle model"}
    elif (subject, harness) == ("leap_coverage", "leap_coverage_full_domain"):
        assertions = {"scope success iff inclusive containment", "outside scope error classification",
                      "offset completeness excludes every unknown point",
                      "event completeness excludes every unknown point",
                      "incomplete offset has a concrete gap", "incomplete events have a concrete gap",
                      "complete offset has no gap witness", "complete events have no gap witness",
                      "complete offset implies complete events"}
        covers = {"signed minimum outside scope is rejected", "signed maximum outside narrow scope is rejected",
                  "singleton at baseline remains covered", "exclusive history cutoff is unknown",
                  "last fractional tick before cutoff is covered", "finite expiry extends history",
                  "continuous start after gap has event-only coverage", "continuous rule exactly bridges history",
                  "continuous zero rule is retained", "finite zero expiry preserves history",
                  "fractional candidate starts inside an unknown gap",
                  "upper coordinate with continuous coverage is admitted", "full scalar metadata endpoints are included"}
    elif (subject, harness) == ("endpoint_finish", "endpoint_finish::endpoint_finish_full_domain"):
        assertions = {"endpoint required success", "endpoint checked overflow classification",
                      "endpoint range rejection classification", "endpoint never interval error",
                      "endpoint oracle success envelope", "endpoint exact floor relation",
                      "endpoint exact ceiling relation", "endpoint residual rounding classification"}
        covers = {"endpoint raw minimum negative floor is invariant",
                  "endpoint raw maximum positive ceiling is invariant",
                  "endpoint exact minimum succeeds", "endpoint exact maximum succeeds",
                  "endpoint negative floor rounds into range", "endpoint positive ceiling rounds into range",
                  "endpoint exact lower neighbor is unrepresentable", "endpoint exact upper neighbor is unrepresentable",
                  "endpoint negative ceiling needs no adjustment", "endpoint positive floor needs no adjustment"}
    elif (subject, harness) == ("fraction_constructor", "fraction_constructor::fraction_constructor_full_domain"):
        assertions = {"fraction required success", "fraction rejection classification",
                      "fraction exact numerator retained", "fraction zero canonicalization",
                      "fraction nonzero shift retained", "fraction output numerator bound",
                      "fraction output shift bound", "fraction output proper dyadic"}
        covers = {"fraction zero at shift zero is canonical", "fraction zero at maximum shift is canonical",
                  "fraction one half succeeds", "fraction maximum numerator at shift 53 succeeds",
                  "fraction unit numerator at maximum shift succeeds", "fraction zero beyond maximum shift is rejected",
                  "fraction zero at raw maximum shift is rejected", "fraction oversized proper numerator is rejected",
                  "fraction raw maximum numerator is rejected", "fraction nonzero numerator at zero shift is rejected",
                  "fraction unit boundary is rejected", "fraction proper neighbor succeeds",
                  "fraction maximum numerator at shift 63 succeeds", "fraction maximum numerator at shift 64 succeeds"}
    elif (subject, harness) == ("fraction_comparison", "fraction_comparison::fraction_comparison_full_domain"):
        assertions = {"comparison left factory completeness", "comparison right factory completeness",
                      "comparison required success", "comparison exact rational ordering",
                      "comparison oracle gap bound", "comparison oracle scaled width",
                      "comparison positive numerator bit bounds", "comparison leading exponent bounds",
                      "comparison equal power alignment relation", "comparison alignment count bound",
                      "comparison alignment retains numerator bits", "comparison unequal powers strict order"}
        covers = {"comparison zero equality", "comparison zero below minimum positive",
                  "comparison minimum positive above zero", "comparison identical one half",
                  "comparison quarter below half", "comparison half above quarter",
                  "comparison same shift less", "comparison same shift greater",
                  "comparison unreduced equality left", "comparison unreduced equality right",
                  "comparison maximum alignment left", "comparison maximum alignment right",
                  "comparison maximum alignment less", "comparison maximum alignment greater",
                  "comparison gap63 greater", "comparison gap63 less", "comparison gap64 greater",
                  "comparison gap64 less", "comparison maximum numerator wide gap", "comparison maximum shift gap"}
    elif (subject, harness) == ("fraction_complement", "fraction_arithmetic::fraction_complement_full_domain"):
        assertions = {"complement factory completeness", "complement required success",
                      "complement rejection classification", "complement exact numerator relation",
                      "complement shift retained", "complement positive output bound", "complement proper output"}
        covers = {"complement zero rejected", "complement quarter rejected", "complement below half neighbor rejected",
                  "complement wide below half rejected", "complement minimum positive rejected",
                  "complement maximum shift numerator rejected", "complement half succeeds",
                  "complement unreduced half succeeds", "complement maximum half representation succeeds",
                  "complement above half neighbor succeeds", "complement near one succeeds",
                  "complement three quarters succeeds", "complement five eighths succeeds"}
    elif (subject, harness) == ("fraction_addition", "fraction_arithmetic::fraction_addition_full_domain"):
        assertions = {"addition left factory completeness", "addition right factory completeness",
                      "addition required success", "addition exact carry", "addition exact residual",
                      "addition oracle alignment exact", "addition oracle proper numerator bounds",
                      "addition oracle sum representable", "addition high denominator positive",
                      "addition global fraction maximum", "addition high numerator bound"}
        covers = {"addition zero sum", "addition quarter plus zero", "addition zero plus quarter",
                  "addition two quarters", "addition exact halves", "addition quarter plus three quarters",
                  "addition three quarters plus quarter", "addition three quarters plus half",
                  "addition half plus three quarters", "addition unreduced halves", "addition just below unit",
                  "addition just above unit", "addition maximum numerator sum", "addition u128 edge left",
                  "addition u128 edge right", "addition high denominator left", "addition high denominator right",
                  "addition minimum positive plus zero", "addition zero plus minimum positive",
                  "addition two minimum positives", "addition raw zeros canonicalize",
                  "addition half plus quarter", "addition quarter plus half"}
    elif (subject, harness) == ("scaled_value_split", "scaled_value_split::scaled_value_split_full_domain"):
        assertions = {"split required success", "split rejection classification",
                      "split exact nonnegative whole", "split nonnegative zero fraction",
                      "split exact quotient remainder", "split strict remainder bound",
                      "split large denominator whole zero", "split large denominator numerator retained",
                      "split zero canonicalization", "split nonzero shift retained",
                      "split output numerator bound", "split output shift bound", "split output proper dyadic"}
        covers = {"split zero at scale zero", "split zero at minimum scale", "split zero at negative sixty four",
                  "split integral unit", "split maximum raw whole", "split exact negative scale",
                  "split fractional negative scale", "split below source cutoff", "split at source cutoff",
                  "split below oracle cutoff", "split at oracle cutoff", "split maximum fraction shift",
                  "split oversized proper significand rejected", "split maximum raw significand rejected",
                  "split zero beyond upper scale rejected", "split nonzero beyond upper scale rejected",
                  "split zero beyond lower scale rejected", "split nonzero beyond lower scale rejected",
                  "split minimum raw scale rejected", "split maximum raw scale rejected"}
    elif (subject, harness) == ("signed_whole", "scaled_value_representation::signed_whole_full_domain"):
        assertions = {"signed whole exact magnitude", "signed whole strict sign", "signed whole zero equivalence",
                      "signed whole raw envelope", "signed whole fraction independence"}
        covers = {"signed whole positive zero", "signed whole negative zero", "signed whole positive one",
                  "signed whole negative one", "signed whole positive two", "signed whole negative two",
                  "signed whole positive i64 maximum", "signed whole negative i64 maximum",
                  "signed whole positive two to sixty three", "signed whole negative two to sixty three",
                  "signed whole positive above two to sixty three", "signed whole negative above two to sixty three",
                  "signed whole positive below u64 maximum", "signed whole negative below u64 maximum",
                  "signed whole positive u64 maximum", "signed whole negative u64 maximum",
                  "signed whole positive fractional zero", "signed whole negative fractional zero",
                  "signed whole ignores raw oversized fraction", "signed whole ignores raw maximum fraction shift"}
    elif (subject, harness) == ("from_bits", "scaled_value_representation::from_bits_full_domain"):
        assertions = {"from bits required success", "from bits rejection classification", "from bits original sign retained",
                      "from bits exact integral magnitude", "from bits integral zero fraction",
                      "from bits exact quotient remainder", "from bits strict remainder bound",
                      "from bits large denominator whole zero", "from bits large denominator numerator retained",
                      "from bits zero canonicalization", "from bits nonzero shift retained", "from bits whole envelope",
                      "from bits output numerator bound", "from bits output shift bound", "from bits output proper dyadic"}
        covers = {"from bits positive zero", "from bits negative zero", "from bits positive minimum subnormal",
                  "from bits negative minimum subnormal", "from bits maximum subnormal", "from bits minimum normal",
                  "from bits half quantum", "from bits below quantum", "from bits integral quantum",
                  "from bits above quantum", "from bits denominator sixty three", "from bits denominator sixty four",
                  "from bits positive magnitude boundary", "from bits negative magnitude boundary",
                  "from bits below magnitude boundary", "from bits above magnitude boundary rejected",
                  "from bits positive infinity rejected", "from bits negative infinity rejected",
                  "from bits quiet NaN rejected", "from bits negative signalling NaN rejected"}
    elif (subject, harness) == ("era_constructor", "proof::era_constructor_full_domain"):
        assertions = {"public window admission iff full raw contract", "exact earliest coordinate retained",
                      "exact latest coordinate retained", "constructed endpoints remain ordered",
                      "every raw constructor rejection is InvalidEraWindow"}
        covers = {"zero singleton is admitted", "minimum singleton is admitted", "maximum singleton is admitted",
                  "complete numeric window is admitted", "inverted valid neighbors rejected",
                  "only lower endpoint fails Garde", "only upper endpoint fails Garde",
                  "both endpoint report entries are reachable", "raw i128 extremes are rejected"}
    else:
        raise ValueError("Unknown additional healthy proof contract")
    if not assertions or not covers:
        raise ValueError("Additional healthy proofs require nonempty mathematical obligations")
    return frozenset(assertions), frozenset(covers)


def prepare(loader, source: Path, inputs: Path, job: dict, index: int) -> dict:
    """Prepare only an exact fixed job using a projection in the frozen tree.

    loader(name, frozen_path) is the runner's existing local module loader. Its
    source identity and the returned proof inputs remain the runner's custody
    responsibility. This function neither admits tools nor starts a process.
    """
    if (type(job) is not dict or set(job) != _FIELDS
            or any(type(job[field]) is not str for field in ("subject", "case", "harness"))
            or type(job["restored_healthy"]) is not bool
            or job["intended_failure"] is not None and type(job["intended_failure"]) is not str):
        raise ValueError("Additional proof preparation requires a complete fixed job")
    subject = job["subject"]
    if subject in {"coordinate", "exchange"}:
        selector = "coordinate-exchange"
    elif subject == "clock":
        selector = "clock"
    elif subject == "leap_calendar":
        selector = "leap-calendar"
    elif subject == "relative_range":
        selector = "relative-range"
    elif subject == "exchange_composition":
        selector = "exchange-composition"
    elif subject == "calendar_composition":
        selector = "calendar-composition"
    elif subject == "leap_ticks":
        selector = "leap-ticks"
    elif subject == "calendar_finite_laws":
        selector = "calendar-finite-laws"
    elif subject == "leap_coverage":
        selector = "leap-coverage"
    elif subject == "endpoint_finish":
        selector = "endpoint-finish"
    elif subject == "fraction_constructor":
        selector = "fraction-constructor"
    elif subject == "fraction_comparison":
        selector = "fraction-comparison"
    elif subject in {"fraction_complement", "fraction_addition"}:
        selector = "fraction-arithmetic"
    elif subject in PARTITION_SUBJECTS or subject == "fraction_addition_partition_fault":
        selector = PARTITION_SELECTOR
    elif subject == "scaled_value_split":
        selector = "scaled-value-split"
    elif subject in {"signed_whole", "from_bits"}:
        selector = "scaled-value-representation"
    else:
        raise ValueError("Unknown additional proof subject")
    schedule = jobs(selector)
    if type(index) is not int or not 1 <= index <= len(schedule) or job != schedule[index - 1]:
        raise ValueError("Additional proof job differs from its fixed schedule position")
    stem = _STEMS[selector]
    projection = loader(f"formal_additional_{stem}_{index}", source / f"tools/formal/{stem}_projection.py")
    harness = source / f"tools/formal/{stem}_harness.rs"
    if selector == "coordinate-exchange":
        prepared = projection.prepare_coordinate_exchange(source, inputs, subject, job["case"], harness)
    elif selector == "clock":
        prepared = projection.prepare_clock(source, inputs, job["case"], harness)
    elif selector == "leap-calendar":
        prepared = projection.prepare_leap_calendar(source, inputs, job["case"], harness)
    elif selector == "relative-range":
        prepared = projection.prepare_relative_range(source, inputs, job["case"], harness)
    elif selector == "exchange-composition":
        prepared = projection.prepare_exchange_composition(source, inputs, job["case"], harness)
    elif selector == "calendar-composition":
        prepared = projection.prepare_calendar_composition(source, inputs, job["case"], harness)
    elif selector == "leap-ticks":
        prepared = projection.prepare_leap_ticks(source, inputs, job["case"], harness)
    elif selector == "leap-coverage":
        prepared = projection.prepare_leap_coverage(source, inputs, job["case"], harness)
    elif selector == "endpoint-finish":
        prepared = projection.prepare_endpoint_finish(source, inputs, job["case"], harness)
    elif selector == "fraction-constructor":
        prepared = projection.prepare_fraction_constructor(source, inputs, job["case"], harness)
    elif selector == "fraction-comparison":
        prepared = projection.prepare_fraction_comparison(source, inputs, job["case"], harness)
    elif selector == "fraction-arithmetic":
        prepared = projection.prepare_fraction_arithmetic(source, inputs, subject, job["case"], harness)
    elif selector == PARTITION_SELECTOR:
        prepared = projection.prepare_fraction_addition_partitions(source, inputs, subject, job["case"], harness)
    elif selector == "scaled-value-split":
        prepared = projection.prepare_scaled_value_split(source, inputs, job["case"], harness)
    elif selector == "scaled-value-representation":
        prepared = projection.prepare_scaled_value_representation(source, inputs, subject, job["case"], harness)
    else:
        prepared = projection.prepare_calendar_finite_laws(source, inputs, job["case"], harness)
    if not isinstance(prepared, dict) or prepared.get("entrypoint") != inputs / "proof.rs":
        raise ValueError("Additional projection did not return inputs/proof.rs metadata")
    return prepared
