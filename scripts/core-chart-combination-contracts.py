"""Source-only external contracts for the S158 chart-combination API.

GPL-2.0-only, David Osipov with AI assistance, 4 November 2026.
This module describes genuine public callers and intended diagnostic meanings.
It launches no compiler or product. Python controls qualify only this inventory
and its strict output parser; the coordinator owns actual external execution.
"""
from __future__ import annotations


class Incomplete(ValueError):
    """A bounded external consumer record is incomplete or altered."""


INPUTS = ("scripts/core-chart-combination-contracts.py",
          "scripts/tests/test_core_chart_combination_contracts.py")
LEGAL_NAME = "legal_chart_combination_and_replay"
MARKER = "CORE_CHART_COMBINATION_CONTRACT_PROFILE"
SUCCESS = "legal-chart-combination-contract"
FIELDS = ("labels", "contexts", "monotonic", "coordinates", "exchanges",
          "observations", "histories", "appends", "frames", "domains", "charts",
          "prepares", "terms", "tracked_projections", "chart_enclosures",
          "raw_enclosures", "raw_batches", "principal_parameters",
          "combination_parameters", "minima", "strata", "combination_seeds",
          "chart_batches", "chart_advances", "bootstrap_getters", "outcome_getters",
          "estimate_getters", "iterator_next", "state_equalities")
EXPECTED = (5, 2, 4, 1, 1, 3, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 2, 1, 3, 5, 17, 21, 2, 2)


def legal_expected() -> dict[str, int]:
    """Return fixed direct attempts, separate from nested source-work budgets."""
    return dict(zip(FIELDS, EXPECTED, strict=True))


def contract_counts(output: object) -> dict[str, int]:
    """Require exactly the complete counter row and sentinel, LF or CRLF."""
    if type(output) is not str or len(output) > 4096:
        raise Incomplete("Chart-combination legal output is not bounded text.")
    lines = output.splitlines(keepends=True)
    if len(lines) != 2 or any(not line.endswith("\n") for line in lines):
        raise Incomplete("Chart-combination legal output has missing or extra records.")
    lines = [line[:-2] if line.endswith("\r\n") else line[:-1] for line in lines]
    expected = MARKER + "".join(f" {name}={count}" for name, count in legal_expected().items())
    if sorted(lines) != sorted((expected, SUCCESS)):
        raise Incomplete("Chart-combination legal counters or sentinel differ.")
    return legal_expected()


def legal_contract_source() -> str:
    """Describe one actual singleton, an independent control owner and replay.

    One history supplies three zero-phase observations and one chart projection.
    The nonzero map has D=1/128, F=1/1024, reference2s and query4s. Its independent
    expected transformed fitted phase/rate are -5/512 and -1/1024. Zero original
    distance/SD/skew require genuine-singleton copying. Two independent seeds
    share the same genuine raw batch; three advances and two complete equality
    observations establish the intended initial/control/replay relationship.
    Each direct product call is precharged. Iterator next and state equality
    observations are charged separately; no Clone, refit or observation is added.
    """
    expected = ",".join(str(value) for value in EXPECTED)
    fields = "".join(f" {name}={{}}" for name in FIELDS)
    arguments = ",".join(f"self.0[{index}]" for index in range(len(FIELDS)))
    return f'''use std::{{error::Error,io,num::NonZeroU64,ptr}};
use time_core::{{FixtureAdaptiveHistory,FixtureAffineChart,FixtureChartDomain,
 FixtureChartFrames,FixtureChartOutcome,FixtureContext,FixtureObservation,
 FixtureProjectionTerms,FixtureChartBatch,FixtureMinimum,FixturePrincipalParameters,
 FixturePrincipalStratum,FixturePrincipalOutcomeKind,FixtureProjectedBatch,
 FixtureCombinationParameters,FixtureCombinationState,FixtureCombinationSourceStatus,
 enclose_fixture_chart_projection,enclose_fixture_projection}};
use time_domain::{{MonotonicNs,UtcCoordinateQ32,exchange_arithmetic}};
const EXPECTED:[u64;{len(FIELDS)}]=[{expected}];
struct Calls([u64;{len(FIELDS)}]);
impl Calls{{
 /// Charge one direct operation before its effect or observation.
 fn charge(&mut self,slot:usize)->Result<(),io::Error>{{
  let cap=*EXPECTED.get(slot).ok_or_else(||io::Error::other("slot"))?;
  let value=self.0.get_mut(slot).ok_or_else(||io::Error::other("slot"))?;
  let next=value.checked_add(1).ok_or_else(||io::Error::other("counter overflow"))?;
  if next>cap{{return Err(io::Error::other("counter ceiling"));}}
  *value=next;Ok(())
 }}
}}
impl Drop for Calls{{
 /// Emit the actually reached prefix even when an assertion or operation fails.
 fn drop(&mut self){{eprintln!("{MARKER}{fields}",{arguments});}}
}}
macro_rules! call{{($calls:expr,$slot:literal,$value:expr)=>{{{{
 $calls.charge($slot)?;$value
}}}};}}
fn main()->Result<(),Box<dyn Error>>{{
 let mut c=Calls([0;{len(FIELDS)}]);
 let a=call!(c,0,NonZeroU64::new(11)).ok_or_else(||io::Error::other("label"))?;
 let b=call!(c,0,NonZeroU64::new(17)).ok_or_else(||io::Error::other("label"))?;
 let g=call!(c,0,NonZeroU64::new(23)).ok_or_else(||io::Error::other("label"))?;
 let z=call!(c,0,NonZeroU64::new(19)).ok_or_else(||io::Error::other("label"))?;
 let j=call!(c,0,NonZeroU64::new(29)).ok_or_else(||io::Error::other("label"))?;
 let raw=call!(c,1,FixtureContext::new(a,b,g));
 let target=call!(c,1,FixtureContext::new(a,z,j));
 let t0=call!(c,2,MonotonicNs::new(0));
 let t1=call!(c,2,MonotonicNs::new(1_000_000_000));
 let t2=call!(c,2,MonotonicNs::new(2_000_000_000));
 let t4=call!(c,2,MonotonicNs::new(4_000_000_000));
 let zero=call!(c,3,UtcCoordinateQ32::new(0))?;
 let exchange=call!(c,4,exchange_arithmetic(zero,zero,zero,zero))?;
 let phase=call!(c,24,exchange.offset());
 let mut history=call!(c,6,FixtureAdaptiveHistory::new(raw,3))?;
 for at in [t0,t1,t2]{{
  let row=call!(c,5,FixtureObservation::new(raw,at,phase,1.0))?;
  call!(c,7,history.try_append(row))?;
 }}
 let frames=call!(c,8,FixtureChartFrames::new(b,g,z,j))?;
 let domain=call!(c,9,FixtureChartDomain::new(t0,t2,t4))?;
 let chart=call!(c,10,FixtureAffineChart::new(frames,domain,1.0/128.0,1.0/1024.0))?;
 let view=call!(c,11,history.prepare_chart(raw,FixtureChartOutcome::Known(&chart)))?
  .ok_or_else(||io::Error::other("view"))?;
 let view_context=call!(c,24,view.context());
 let terms=call!(c,12,FixtureProjectionTerms::new(0.0,0.0,0.0))?;
 let projection=call!(c,13,view.project_tracking(view_context,2,t2,t4,terms,0.0))?;
 let entries=[call!(c,14,enclose_fixture_chart_projection(&projection))?];
 let wrapped_projection=call!(c,24,entries[0].projection());
 let original=call!(c,24,wrapped_projection.source_projection());
 assert_eq!(call!(c,24,original.context()),raw);
 let raw_entries=[call!(c,15,enclose_fixture_projection(original))?];
 let raw_batch=call!(c,16,FixtureProjectedBatch::new(b,g,t4,&raw_entries))?;
 let parameters=call!(c,17,FixturePrincipalParameters::new(1.0/1024.0,0.0))?;
 let combination=call!(c,18,FixtureCombinationParameters::new(2.0,0.0))?;
 let minimum=call!(c,19,FixtureMinimum::new(1))?;
 let strata=[call!(c,20,FixturePrincipalStratum::new(a,2))?];
 let mut owner=call!(c,21,FixtureCombinationState::seed(&[a],parameters,combination,minimum,&raw_batch,&strata))?;
 let mut control=call!(c,21,FixtureCombinationState::seed(&[a],parameters,combination,minimum,&raw_batch,&strata))?;
 let batch=call!(c,22,FixtureChartBatch::new(&chart,t4,&entries))?;
 let ready=call!(c,23,owner.advance_chart(&chart,&batch,&strata))?;
 let decision=call!(c,25,ready.principal());
 assert_eq!(call!(c,25,decision.kind()),FixturePrincipalOutcomeKind::ReferenceReady);
 assert_eq!(call!(c,25,ready.witness_count()),Some(1));
 assert_eq!(call!(c,25,ready.contributor_count()),Some(1));
 assert_eq!(call!(c,25,ready.emitted_round()),Some(minimum_label()));
 assert_eq!(call!(c,25,ready.source_status(a)),Some(FixtureCombinationSourceStatus::Principal));
 let estimate=call!(c,25,ready.combined()).ok_or_else(||io::Error::other("estimate"))?;
 assert!(ptr::eq(ptr::from_ref(call!(c,26,estimate.batch())),ptr::from_ref(&batch)));
 assert!(ptr::eq(ptr::from_ref(call!(c,26,estimate.chart())),ptr::from_ref(&chart)));
 assert!(ptr::eq(ptr::from_ref(call!(c,26,estimate.principal_entry())),ptr::from_ref(&entries[0])));
 assert_eq!(call!(c,26,estimate.source_context()),raw);
 assert_eq!(call!(c,26,estimate.context()),target);
 assert_eq!(call!(c,26,estimate.principal_label()),a);
 assert_eq!(call!(c,26,estimate.frame()),z);
 assert_eq!(call!(c,26,estimate.mapping_generation()),j);
 assert_eq!(call!(c,26,estimate.anchor_observed_at()),t2);
 assert_eq!(call!(c,26,estimate.latest_observed_at()),t2);
 assert_eq!(call!(c,26,estimate.fit_reference()),t2);
 assert_eq!(call!(c,26,estimate.projected_at()),t4);
 assert_eq!(call!(c,26,estimate.supporter_count()),1);
 let mut supporters=call!(c,26,estimate.supporters());
 assert_eq!(call!(c,27,supporters.next()),Some(a));
 assert!(call!(c,27,supporters.next()).is_none());
 assert_eq!(call!(c,26,estimate.phase_seconds()),-5.0/512.0);
 assert_eq!(call!(c,26,estimate.phase_rate_seconds_per_second()),-1.0/1024.0);
 assert_eq!(call!(c,26,estimate.phase_standard_deviation_seconds()),0.0);
 assert_eq!(call!(c,26,estimate.rate_standard_deviation_seconds_per_second()),0.0);
 assert_eq!(call!(c,26,estimate.tracking_skew_seconds_per_second()),0.0);
 let phase_error=call!(c,26,estimate.phase_mean_roundoff_seconds());
 let rate_error=call!(c,26,estimate.rate_mean_roundoff_seconds_per_second());
 assert!(phase_error.is_finite() && phase_error>0.0);
 assert!(rate_error.is_finite() && rate_error>0.0);
 let same=call!(c,23,control.advance_chart(&chart,&batch,&strata))?;
 let same_decision=call!(c,25,same.principal());
 assert_eq!(call!(c,25,same_decision.kind()),FixturePrincipalOutcomeKind::ReferenceReady);
 assert_eq!(call!(c,25,same.emitted_round()),Some(minimum_label()));
 assert!(call!(c,28,owner==control));
 let replay=call!(c,23,owner.advance_chart(&chart,&batch,&strata))?;
 let retained=call!(c,25,replay.principal());
 assert_eq!(call!(c,25,retained.kind()),FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate);
 assert!(call!(c,25,replay.combined()).is_none());
 assert_eq!(call!(c,25,replay.emitted_round()),None);
 assert_eq!(call!(c,25,replay.witness_count()),Some(1));
 assert_eq!(call!(c,25,replay.contributor_count()),None);
 assert_eq!(call!(c,25,replay.source_status(a)),Some(FixtureCombinationSourceStatus::NotEvaluated));
 assert!(call!(c,28,owner==control));
 assert_eq!(c.0,EXPECTED);
 println!("{SUCCESS}");Ok(())
}}
/// A compile-time nonzero expected emission, with no product constructor call.
const fn minimum_label()->NonZeroU64{{NonZeroU64::MIN}}
'''


def _negative(prefix: str, expression: str, code: str, tokens: list[str]) -> dict[str, object]:
    """Bind each intended rejection to its primary line and genuine legal neighbor."""
    return {"source": prefix + expression + "\n}\nfn main(){}\n", "code": code,
            "tokens": tokens, "line": len(prefix.splitlines()) + 1,
            "external_legal_neighbor": {
                "file": "crates/time-core/tests/chart_combination_lifecycle.rs",
                "test": "chart_combination_retains_real_provenance_and_observation_credit"}}


def contract_examples() -> dict[str, dict[str, object]]:
    """Return one genuine legal caller and sixteen diagnostic-bound negatives.

    Intended compiler codes/spans are unexecuted source expectations. No negative
    fabricates checked state; its function inputs are actual public checked types.
    Legal source is the compiling neighbor for every additive API restriction.
    """
    imports = ("use time_core::{FixtureAffineChart,FixtureChartBatch,FixtureChartInterval,"
               "FixtureChartCombinedEstimate,FixtureChartCombinationOutcome,"
               "FixtureCombinationState,FixtureCombinationOutcome,FixtureCombinedEstimate,"
               "FixturePrincipalStratum,FixtureSupporterUncertainty,combine_fixture_tracking_supporters};\n"
               "use time_domain::MonotonicNs;\n")
    cases: dict[str, dict[str, object]] = {
        LEGAL_NAME: {"source": legal_contract_source(), "code": None,
                     "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
                     "chart_combination_calls": legal_expected()}}
    for name, typ in (("estimate", "FixtureChartCombinedEstimate"),
                      ("outcome", "FixtureChartCombinationOutcome")):
        cases[f"chart_combination_{name}_private_update"] = _negative(
            imports + f"fn bypass<'b,'e,'c>(value:{typ}<'b,'e,'c>){{\n",
            f" let _={typ}{{..value}};", "E0451", [typ, "private"])
        cases[f"chart_combination_{name}_default"] = _negative(
            imports + "fn bypass(){\n", f" let _:{typ}<'_,'_,'_>=Default::default();",
            "E0277", [typ, "Default"])
        cases[f"chart_combination_{name}_unchecked_new"] = _negative(
            imports + "fn bypass(){\n", f" let _={typ}::new();",
            "E0599", [typ, "new"])
    cases["chart_combination_bound_private_update"] = _negative(
        imports + "fn bypass(value:&mut FixtureChartCombinedEstimate<'_,'_,'_>){\n",
        " value.quantities.phase_error=0.0;", "E0616", ["quantities", "private"])
    signature = (imports + "fn bypass<'e,'c>(state:&mut FixtureCombinationState,chart:&FixtureAffineChart,"
                 "batch:FixtureChartBatch<'e,'c>,strata:&[FixturePrincipalStratum])"
                 "->FixtureChartCombinationOutcome<'e,'e,'c>{\n")
    cases["chart_combination_batch_lifetime"] = _negative(
        signature, " state.advance_chart(chart,&batch,strata).unwrap()", "E0515",
        ["batch", "cannot return value referencing"])
    cases["chart_combination_entries_lifetime"] = _negative(
        imports + "fn bypass<'c>(state:&mut FixtureCombinationState,chart:&'c FixtureAffineChart,"
        "at:MonotonicNs,value:FixtureChartInterval<'c>,strata:&[FixturePrincipalStratum])"
        "->&'c FixtureChartInterval<'c>{\n"
        " let entries=[value];\n let batch=FixtureChartBatch::new(chart,at,&entries).unwrap();\n",
        " state.advance_chart(chart,&batch,strata).unwrap().combined().unwrap().principal_entry()", "E0515",
        ["entries", "cannot return value referencing"])
    cases["chart_combination_descriptor_lifetime"] = _negative(
        imports + "fn bypass<'e,'c>(state:&mut FixtureCombinationState,original:FixtureAffineChart,"
        "at:MonotonicNs,entries:&'e [FixtureChartInterval<'c>],strata:&[FixturePrincipalStratum])"
        "->&'c FixtureAffineChart{\n"
        " let chart=original;\n let batch=FixtureChartBatch::new(&chart,at,entries).unwrap();\n",
        " state.advance_chart(&chart,&batch,strata).unwrap().combined().unwrap().chart()", "E0515",
        ["chart", "cannot return value referencing"])
    cases["chart_combination_live_entries_borrow"] = _negative(
        imports + "fn bypass<'e,'c>(state:&mut FixtureCombinationState,chart:&'c FixtureAffineChart,"
        "at:MonotonicNs,entries:&'e mut [FixtureChartInterval<'c>],strata:&[FixturePrincipalStratum]){\n"
        " let batch=FixtureChartBatch::new(chart,at,entries).unwrap();\n"
        " let outcome=state.advance_chart(chart,&batch,strata).unwrap();\n",
        " entries.reverse();\n let _=outcome.combined();", "E0502", ["entries", "mutable", "immutable"])
    cases["chart_combination_estimate_to_legacy"] = _negative(
        imports + "fn bypass(value:FixtureChartCombinedEstimate<'_,'_,'_>)->FixtureCombinedEstimate{\n",
        " value", "E0308", ["FixtureCombinedEstimate", "FixtureChartCombinedEstimate"])
    cases["chart_combination_outcome_to_legacy"] = _negative(
        imports + "fn bypass<'b,'e,'c>(value:FixtureChartCombinationOutcome<'b,'e,'c>)"
        "->FixtureCombinationOutcome<'b>{\n", " value", "E0308",
        ["FixtureCombinationOutcome", "FixtureChartCombinationOutcome"])
    cases["chart_combination_estimate_into_legacy"] = _negative(
        imports + "fn bypass(value:FixtureChartCombinedEstimate<'_,'_,'_>)->FixtureCombinedEstimate{\n",
        " value.into()", "E0277", ["FixtureCombinedEstimate", "FixtureChartCombinedEstimate", "From"])
    cases["chart_combination_principal_entry_legacy_uncertainty"] = _negative(
        imports + "fn bypass(value:&FixtureChartCombinedEstimate<'_,'_,'_>){\n",
        " let _=FixtureSupporterUncertainty::new(value.principal_entry(),0.0,0.0,0.0);",
        "E0308", ["FixtureProjectedInterval", "FixtureChartInterval"])
    cases["chart_combination_principal_legacy_consumer"] = _negative(
        imports + "fn bypass(value:&FixtureChartCombinationOutcome<'_,'_,'_>){\n",
        " let _=combine_fixture_tracking_supporters(value.principal());", "E0308",
        ["FixturePrincipalDecision", "FixtureChartPrincipalDecision"])
    return cases


REFERENCE_LEGAL_NAME = "legal_reference_input_and_feedback"
REFERENCE_MARKER = "CORE_REFERENCE_INPUT_CONTRACT_PROFILE"
REFERENCE_SUCCESS = "legal-reference-input-contract"
REFERENCE_FIELDS = (*FIELDS, "reference_states", "reference_readiness",
                    "reference_admissions", "reference_feedback", "reference_combined",
                    "reference_rounds", "reference_estimate_getters")
REFERENCE_EXPECTED = (*EXPECTED, 1, 2, 4, 4, 1, 2, 2)


def reference_legal_expected() -> dict[str, int]:
    """Retain the genuine caller recipe and bound each added reference operation."""
    return dict(zip(REFERENCE_FIELDS, REFERENCE_EXPECTED, strict=True))


def reference_contract_counts(output: object) -> dict[str, int]:
    """Receive exactly one complete reference row and sentinel, preserving LF/CRLF."""
    if type(output) is not str or len(output) > 4096:
        raise Incomplete("Reference-input legal output is not bounded text.")
    lines = output.splitlines(keepends=True)
    if len(lines) != 2 or any(not line.endswith("\n") for line in lines):
        raise Incomplete("Reference-input legal output has missing or extra records.")
    lines = [line[:-2] if line.endswith("\r\n") else line[:-1] for line in lines]
    expected = REFERENCE_MARKER + "".join(
        f" {name}={count}" for name, count in reference_legal_expected().items())
    if sorted(lines) != sorted((expected, REFERENCE_SUCCESS)):
        raise Incomplete("Reference-input legal counters or sentinel differ.")
    return reference_legal_expected()


def reference_legal_contract_source() -> str:
    """Extend the unchanged genuine caller without adding an upstream producer.

    Two scoped tokens retain the same actual estimate and emission. No-effect
    preserves admission; changed then unknown reject and no-effect cannot restore.
    All added direct calls are charged before execution, separate from source uppers.
    """
    source = legal_contract_source()
    source = source.replace(f"[u64;{len(FIELDS)}]", f"[u64;{len(REFERENCE_FIELDS)}]")
    source = source.replace(
        "=[" + ",".join(str(value) for value in EXPECTED) + "];",
        "=[" + ",".join(str(value) for value in REFERENCE_EXPECTED) + "];")
    source = source.replace(f"Calls([0;{len(FIELDS)}])", f"Calls([0;{len(REFERENCE_FIELDS)}])")
    old_fields = "".join(f" {name}={{}}" for name in FIELDS)
    old_args = ",".join(f"self.0[{index}]" for index in range(len(FIELDS)))
    new_fields = "".join(f" {name}={{}}" for name in REFERENCE_FIELDS)
    new_args = ",".join(f"self.0[{index}]" for index in range(len(REFERENCE_FIELDS)))
    source = source.replace(f'eprintln!("{MARKER}{old_fields}",{old_args});',
                            f'eprintln!("{REFERENCE_MARKER}{new_fields}",{new_args});')
    source = source.replace(f'println!("{SUCCESS}")', f'println!("{REFERENCE_SUCCESS}")')
    source = source.replace(" enclose_fixture_chart_projection,enclose_fixture_projection};",
                            " FixtureReferenceInputState,FixtureReferenceReadiness,FixtureCorrectionFeedback,\n"
                            " FixtureReferenceInputError,enclose_fixture_chart_projection,enclose_fixture_projection};")
    extension = ''' let mut reference=call!(c,29,FixtureReferenceInputState::new(&chart));
 assert_eq!(call!(c,30,reference.readiness()),FixtureReferenceReadiness::Ready);
 {
  let input=call!(c,31,reference.admit_reference_input(&ready))?;
  let actual=call!(c,33,input.combined());
  assert!(ptr::eq(ptr::from_ref(actual),ptr::from_ref(estimate)));
  assert_eq!(call!(c,34,input.emitted_round()),minimum_label());
  assert_eq!(call!(c,35,actual.latest_observed_at()),t2);
  assert_eq!(call!(c,35,actual.phase_seconds()),-5.0/512.0);
 }
 assert_eq!(call!(c,32,reference.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect)),FixtureReferenceReadiness::Ready);
 {
  let input=call!(c,31,reference.admit_reference_input(&ready))?;
  assert_eq!(call!(c,34,input.emitted_round()),minimum_label());
 }
 assert_eq!(call!(c,32,reference.receive_feedback(FixtureCorrectionFeedback::CoordinateChanged)),FixtureReferenceReadiness::CoordinateChanged);
 assert_eq!(call!(c,31,reference.admit_reference_input(&ready)).err(),Some(FixtureReferenceInputError::Unavailable{readiness:FixtureReferenceReadiness::CoordinateChanged}));
 assert_eq!(call!(c,32,reference.receive_feedback(FixtureCorrectionFeedback::Unknown)),FixtureReferenceReadiness::Unknown);
 assert_eq!(call!(c,32,reference.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect)),FixtureReferenceReadiness::Unknown);
 assert_eq!(call!(c,30,reference.readiness()),FixtureReferenceReadiness::Unknown);
 assert_eq!(call!(c,31,reference.admit_reference_input(&ready)).err(),Some(FixtureReferenceInputError::Unavailable{readiness:FixtureReferenceReadiness::Unknown}));
'''
    anchor = " let same=call!(c,23,control.advance_chart(&chart,&batch,&strata))?;\n"
    if source.count(anchor) != 1:
        raise Incomplete("Reference-input legal adaptation lost its genuine source anchor.")
    return source.replace(anchor, extension + anchor)


def reference_contract_examples() -> dict[str, dict[str, object]]:
    """Return only the focused genuine caller and six intended reference bypasses.

    Each negative uses actual checked typed arguments. Codes and primary spans
    remain source expectations until the coordinator receives real diagnostics.
    """
    imports = ("use time_core::{FixtureReferenceInput,FixtureReferenceInputState,"
               "FixtureChartCombinationOutcome,FixtureChartCombinedEstimate,FixtureCorrectionFeedback};\n")
    neighbor = {"file": "crates/time-core/tests/private/reference_input_readiness.rs",
                "test": "genuine_ready_input_is_readonly_and_replay_has_no_new_credit"}
    cases: dict[str, dict[str, object]] = {
        REFERENCE_LEGAL_NAME: {"source": reference_legal_contract_source(), "code": None,
                               "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
                               "reference_input_calls": reference_legal_expected()}}
    owned = imports + "fn bypass(value:FixtureReferenceInput<'_,'_,'_,'_>){\n"
    cases["reference_input_token_private_update"] = _negative(
        owned, " let _=FixtureReferenceInput{..value};", "E0451", ["FixtureReferenceInput", "private"])
    cases["reference_input_token_clone"] = _negative(
        owned, " let _=value.clone();", "E0599", ["FixtureReferenceInput", "clone"])
    prefix = (imports + "fn bypass<'b,'e,'c>(owner:&mut FixtureReferenceInputState<'c>,"
              "outcome:&FixtureChartCombinationOutcome<'b,'e,'c>){\n"
              " let input=owner.admit_reference_input(outcome).unwrap();\n")
    cases["reference_input_estimate_move"] = _negative(
        prefix, " let _owned:FixtureChartCombinedEstimate<'_,'_,'_>=*input.combined();",
        "E0507", ["shared reference", "FixtureChartCombinedEstimate"])
    for name, prepare, use in (
            ("chart", "let view=input.combined().chart();", "view.domain()"),
            ("entry", "let view=*input.combined().principal_entry();", "view.projection().chart().domain()"),
            ("projection", "let view=*input.combined().principal_entry().projection();", "view.chart().domain()")):
        cases[f"reference_input_{name}_feedback_borrow"] = _negative(
            prefix + " " + prepare + "\n",
            " owner.receive_feedback(FixtureCorrectionFeedback::CoordinateChanged);\n let _=" + use + ";",
            "E0502", ["owner", "mutable", "immutable"])
    for name, specimen in cases.items():
        if name != REFERENCE_LEGAL_NAME:
            specimen["external_legal_neighbor"] = dict(neighbor)
    return cases
