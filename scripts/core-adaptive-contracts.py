"""Bounded external fixtures for retained runs and adaptive-history contracts.

GPL-2.0-only, David Osipov with AI assistance, 2 November 2026.
The coordinator admits source/artifacts and observes compiler diagnostics and
native exit status. This module generates inputs and validates an exact external
call record; neither operation establishes Rust execution or qualification.
"""
from __future__ import annotations


class Incomplete(ValueError):
    """An external call record does not match the complete frozen fixture."""


INPUTS = (
    "scripts/core-adaptive-contracts.py",
    "scripts/tests/test_core_adaptive_contracts.py",
)
FIELDS = (
    "retained_points", "retained_inputs", "retained_selections",
    "adaptive_observations", "adaptive_appends", "adaptive_histories",
    "raw_parameter_validations", "raw_ordinal_validations", "raw_event_validations",
    "external_normalizations",
)
AUXILIARY_FIELDS = ("contexts", "monotonic", "coordinates", "exchanges", "getters")
MARKER = "CORE_ADAPTIVE_CONTRACT_PROFILE"
SUCCESS = "legal-core-contract"
LEGAL_NAME = "legal_adaptive_history_and_retained_selection"


def legal_expected() -> dict[str, int]:
    """Return literal direct-owner attempts; nested work needs source uppers."""
    return dict(zip(FIELDS, (0, 0, 2, 3, 3, 1, 0, 0, 0, 1), strict=True))


def legal_auxiliary_expected() -> dict[str, int]:
    """Return the separate exact setup/accessor recipe, not inner telemetry."""
    return dict(zip(AUXILIARY_FIELDS, (1, 3, 1, 1, 29), strict=True))


def _record() -> str:
    """Assemble the one exact complete call row in its fixed field order."""
    values = {**legal_expected(), **legal_auxiliary_expected()}
    return MARKER + "".join(f" {name}={value}" for name, value in values.items())


def contract_counts(output: object) -> dict[str, int]:
    """Require one complete call row and success sentinel, in either pipe order.

    Only two newline-terminated literal records are admitted. CRLF is a permitted
    line ending; altered fields, partial prefixes and unrelated owner rows reject.
    This checks transport/content, not native exit status or observed inner work.
    """
    if not isinstance(output, str) or len(output) > 4096:
        raise Incomplete("Adaptive legal output is not a bounded text record.")
    lines = output.splitlines(keepends=True)
    if len(lines) != 2 or any(not line.endswith("\n") for line in lines):
        raise Incomplete("Adaptive legal output is incomplete or has extra records.")
    records = [line[:-2] if line.endswith("\r\n") else line[:-1] for line in lines]
    if sorted(records) != sorted((_record(), SUCCESS)):
        raise Incomplete("Adaptive legal call recipe or success sentinel differs.")
    return legal_expected()


def legal_contract_source() -> str:
    """Generate one actual three-row caller and two direct selector attempts.

    Every product attempt is charged before its call, with a fixed local ceiling.
    The history owns its real terminal report; the extra normalizer/selector
    consumes the same immutable checked rows. Wrapping the actual selector error
    checks error projection only; it claims no history failure or recovery.
    """
    values = {**legal_expected(), **legal_auxiliary_expected()}
    expected = ",".join(str(value) for value in values.values())
    format_fields = "".join(f" {name}={{}}" for name in values)
    arguments = ",".join(f"self.0[{index}]" for index in range(len(values)))
    support = f'''use std::{{error::Error,io,num::NonZeroU64}};
use time_core::{{AdaptiveHistoryError,FixtureAdaptiveHistory,FixtureContext,
 FixtureNormalization,FixtureObservation,FixtureRunsError,FixtureRunsStage,
 prepare_fixture_input,select_fixture_runs_with_retained}};
use time_domain::{{MonotonicNs,PhaseOffsetQ33,UtcCoordinateQ32,exchange_arithmetic}};
const EXPECTED:[u64;15]=[{expected}];
struct Calls([u64;15]);
impl Calls {{
 /// Charge one fixed slot before its actual product attempt.
 fn charge(&mut self,slot:usize)->Result<(),io::Error>{{
  let cap=*EXPECTED.get(slot).ok_or_else(||io::Error::other("counter slot"))?;
  let value=self.0.get_mut(slot).ok_or_else(||io::Error::other("counter slot"))?;
  let next=value.checked_add(1).ok_or_else(||io::Error::other("counter overflow"))?;
  if next>cap {{return Err(io::Error::other("counter ceiling"));}}
  *value=next;Ok(())
 }}
}}
impl Drop for Calls {{
 /// Emit the actual reached prefix, including after a failed assertion.
 fn drop(&mut self){{eprintln!("{MARKER}{format_fields}",{arguments});}}
}}
macro_rules! call {{($calls:expr,$slot:literal,$value:expr)=>{{{{
 $calls.charge($slot)?;$value
}}}};}}
/// Construct one checked row at an explicit monotonic instant; no ambient reads.
fn observation(calls:&mut Calls,context:FixtureContext,phase:PhaseOffsetQ33,nanos:u64)
 ->Result<FixtureObservation,Box<dyn Error>>{{
 let at=call!(calls,11,MonotonicNs::new(nanos));
 Ok(call!(calls,3,FixtureObservation::new(context,at,phase,1.0))?)
}}
'''
    return support + r'''fn main()->Result<(),Box<dyn Error>>{
 let mut calls=Calls([0;15]);
 let context=call!(calls,10,FixtureContext::new(
  NonZeroU64::new(11).ok_or_else(||io::Error::other("source"))?,
  NonZeroU64::new(17).ok_or_else(||io::Error::other("frame"))?,
  NonZeroU64::MIN));
 let zero=call!(calls,12,UtcCoordinateQ32::new(0))?;
 let exchange=call!(calls,13,exchange_arithmetic(zero,zero,zero,zero))?;
 let phase=call!(calls,14,exchange.offset());
 let rows=[observation(&mut calls,context,phase,0)?,
  observation(&mut calls,context,phase,1_000_000_000)?,
  observation(&mut calls,context,phase,2_000_000_000)?];
 let mut history=call!(calls,5,FixtureAdaptiveHistory::new(context,64))?;
 for row in rows {call!(calls,4,history.try_append(row))?;}
 assert_eq!(call!(calls,14,history.sample_count()),3);
 assert_eq!(call!(calls,14,history.auxiliary_count()),0);
 assert_eq!(call!(calls,14,history.configured_minimum()),64);
 let fitted=call!(calls,14,history.observation(0)).ok_or_else(||io::Error::other("fitted"))?;
 let decision=call!(calls,14,history.decision_observation(0)).ok_or_else(||io::Error::other("decision"))?;
 assert!(std::ptr::eq(fitted,decision));
 let report=call!(calls,14,history.report()).ok_or_else(||io::Error::other("report"))?;
 assert_eq!(call!(calls,14,report.context()),context);
 assert_eq!(call!(calls,14,report.configured_minimum()),64);
 assert_eq!(call!(calls,14,report.effective_minimum()),3);
 let history_selection=call!(calls,14,report.selection());
 assert_eq!(call!(calls,14,history_selection.sample_count()),3);
 let prepared=match call!(calls,9,prepare_fixture_input(context,&rows))?{
  FixtureNormalization::Prepared(value)=>value,
  FixtureNormalization::Insufficient(_)=>return Err(io::Error::other("insufficient").into()),
 };
 let input=call!(calls,14,prepared.input());
 let selected=call!(calls,2,select_fixture_runs_with_retained(input,0,3))?;
 assert_eq!(call!(calls,14,selected.start()),0);
 assert_eq!(call!(calls,14,selected.sample_count()),3);
 let stats=call!(calls,14,selected.statistics());
 let line=call!(calls,14,stats.line());
 assert_eq!(call!(calls,14,line.phase_seconds()).to_bits(),0);
 assert_eq!(call!(calls,14,line.phase_rate_seconds_per_second()).to_bits(),0);
 let error=call!(calls,2,select_fixture_runs_with_retained(input,usize::MAX,3))
  .expect_err("invalid retained prefix accepted");
 assert_eq!(call!(calls,14,error.stage()),FixtureRunsStage::Suffix);
 assert_eq!(call!(calls,14,error.candidate_start()),None);
 assert_eq!(call!(calls,14,error.point_index()),None);
 assert!(call!(calls,14,Error::source(&error)).is_none());
 // Projection of a genuine returned error; not an actual history failure.
 let wrapped=AdaptiveHistoryError::Runs{source:error};
 let cause=call!(calls,14,Error::source(&wrapped)).ok_or_else(||io::Error::other("cause"))?;
 let actual=cause.downcast_ref::<FixtureRunsError>().ok_or_else(||io::Error::other("cause type"))?;
 assert_eq!(call!(calls,14,actual.stage()),FixtureRunsStage::Suffix);
 assert_eq!(call!(calls,14,history.sample_count()),3);
 assert_eq!(call!(calls,14,history.auxiliary_count()),0);
 assert_eq!(call!(calls,14,report.reference()),call!(calls,14,rows[2].observed_at()));
 assert_eq!(calls.0,EXPECTED);
 println!("legal-core-contract");
 Ok(())
}
'''


def _negative(prefix: str, expression: str, code: str,
              tokens: list[str], neighbor: str) -> dict[str, object]:
    """Place the rejected operation on its own line, with a real legal neighbor."""
    imports = ("use time_core::{FixtureAdaptiveHistory,FixtureAdaptiveHistoryReport,"
               "FixtureObservation};\n")
    head = imports + prefix
    return {"source": head + expression + "\n}\nfn main(){}\n",
            "code": code, "tokens": tokens, "line": len(head.splitlines()) + 1,
            "external_legal_neighbor": {
                "file": "crates/time-core/tests/adaptive_history.rs", "test": neighbor}}


def contract_examples() -> dict[str, dict[str, object]]:
    """Return one legal runtime fixture and eight compile-only bypass probes.

    Typed parameters avoid constructing invalid domain state. The coordinator
    must observe the intended code/token/line; listed diagnostics are expectations.
    Existing package tests provide independent live-row/report legal neighbors.
    """
    cases = {LEGAL_NAME: {
        "source": legal_contract_source(), "code": None,
        "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
        "adaptive_calls": legal_expected(),
        "adaptive_auxiliary_calls": legal_auxiliary_expected(),
    }}
    warmup = "configured_minimum_warms_up_without_claiming_eligibility"
    borrowed = "adaptive_retains_decision_prefix_across_two_rounds"
    cases["adaptive_history_private_update"] = _negative(
        "fn bypass(history:FixtureAdaptiveHistory){\n",
        " let _=FixtureAdaptiveHistory{..history};", "E0451",
        ["FixtureAdaptiveHistory", "private"], warmup)
    cases["adaptive_history_minimum_mutation"] = _negative(
        "fn bypass(mut history:FixtureAdaptiveHistory){\n",
        " history.configured_minimum=0;", "E0616", ["configured_minimum", "private"], warmup)
    cases["adaptive_report_private_update"] = _negative(
        "fn bypass(report:FixtureAdaptiveHistoryReport){\n",
        " let _=FixtureAdaptiveHistoryReport{..report};", "E0451",
        ["FixtureAdaptiveHistoryReport", "private"], borrowed)
    cases["adaptive_observation_shared_mutation"] = _negative(
        "fn bypass(history:&FixtureAdaptiveHistory,replacement:FixtureObservation){\n",
        " *history.observation(0).unwrap()=replacement;", "E0594",
        ["cannot assign", "reference"], borrowed)
    cases["adaptive_report_shared_mutation"] = _negative(
        "fn bypass(history:&FixtureAdaptiveHistory,replacement:FixtureAdaptiveHistoryReport){\n",
        " *history.report().unwrap()=replacement;", "E0594",
        ["cannot assign", "reference"], borrowed)
    for suffix, method, role in (
        ("fitted_row", "observation(0)", "FixtureObservation"),
        ("decision_row", "decision_observation(0)", "FixtureObservation"),
        ("report", "report()", "FixtureAdaptiveHistoryReport"),
    ):
        cases["adaptive_" + suffix + "_escape"] = _negative(
            f"fn bypass(history:FixtureAdaptiveHistory)->&'static {role}{{\n",
            f" history.{method}.unwrap()", "E0515",
            ["history", "cannot return value referencing"], borrowed)
    return cases
