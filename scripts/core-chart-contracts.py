"""Bounded external consumers for charts, batches and S157 retained principal.

GPL-2.0-only, David Osipov with AI assistance, 3 November 2026.
No production admission, dependency build, compiler or native execution is done
by this data module. Existing adaptive/selection legal and negative fixtures are
not changed or replaced. Expected diagnostics and direct call counts are source
expectations until independently compiled and executed on the final candidate.
"""
from __future__ import annotations


class Incomplete(ValueError):
    """The bounded chart consumer's complete record is missing or altered."""


INPUTS = ("scripts/core-chart-contracts.py", "scripts/tests/test_core_chart_contracts.py")
LEGAL_NAME = "legal_chart_tracking_and_enclosure"
MARKER = "CORE_CHART_CONTRACT_PROFILE"
SUCCESS = "legal-chart-contract"
FIELDS = ("labels", "contexts", "monotonic", "coordinates", "exchanges",
          "observations", "histories", "appends", "frames", "domains",
          "charts", "prepares", "terms", "tracked_projections", "enclosures", "getters",
          "batches", "minima", "selections", "batch_getters", "raw_enclosures",
          "raw_batches", "principal_parameters", "principal_strata", "principal_seeds",
          "principal_advances", "chart_principal_advances", "principal_getters")
EXPECTED = (5, 2, 4, 1, 1, 3, 1, 3, 1, 1, 1, 4, 1, 1, 1, 22, 1, 1, 1, 22,
            1, 1, 1, 1, 1, 1, 1, 27)


def legal_expected() -> dict[str, int]:
    """Return literal direct attempts, not the nested selector/resource budget."""
    return dict(zip(FIELDS, EXPECTED, strict=True))


def contract_counts(output: object) -> dict[str, int]:
    """Require the exact completed row and success sentinel in either pipe order."""
    if type(output) is not str or len(output) > 4096:
        raise Incomplete("Chart legal output is not bounded text.")
    lines = output.splitlines(keepends=True)
    if len(lines) != 2 or any(not line.endswith("\n") for line in lines):
        raise Incomplete("Chart legal output is incomplete or has extra records.")
    lines = [line[:-2] if line.endswith("\r\n") else line[:-1] for line in lines]
    record = MARKER + "".join(f" {name}={value}" for name, value in legal_expected().items())
    if sorted(lines) != sorted((record, SUCCESS)):
        raise Incomplete("Chart legal source-work row or success sentinel differs.")
    return legal_expected()


def legal_contract_source() -> str:
    """Provide one genuine chart/selection/retained-principal legal caller.

    Every listed call is charged before execution; Drop emits reached prefix.
    The three additional prepare calls check actual refusal, unknown, and raw
    context priority. The added batch reuses that interval with explicit fixture
    minimum1; it is no fallback policy. Original16 counters retain their values.
    Batch getters include20 product observations and2 iterator next requests;
    there is no new Debug formatting, history or projection call. The principal
    continuation encloses the already retained genuine raw projection, consumes
    its seed credit through the legacy owner, then receives the actual chart
    batch. Its27 getters observe real entry/batch/descriptor provenance and the
    absence of manufactured observation credit. No second producer is created.
    """
    expected = ",".join(str(value) for value in EXPECTED)
    fields = "".join(f" {name}={{}}" for name in FIELDS)
    arguments = ",".join(f"self.0[{i}]" for i in range(len(FIELDS)))
    return f'''use std::{{error::Error,io,num::NonZeroU64,ptr}};
use time_core::{{FixtureAdaptiveHistory,FixtureAffineChart,FixtureChartDomain,
 FixtureChartError,FixtureChartFrames,FixtureChartOutcome,FixtureContext,
 FixtureObservation,FixtureProjectionTerms,FixtureChartBatch,FixtureMinimum,
 FixtureOutcomeKind,FixturePrincipalParameters,FixturePrincipalStratum,
 FixturePrincipalState,FixturePrincipalOutcomeKind,FixtureProjectedBatch,
 enclose_fixture_chart_projection,enclose_fixture_projection}};
use time_domain::{{MonotonicNs,UtcCoordinateQ32,exchange_arithmetic}};
const EXPECTED:[u64;{len(FIELDS)}]=[{expected}];
struct Calls([u64;{len(FIELDS)}]);
impl Calls{{
 /// Charge a bounded owner call before effect or output.
 fn charge(&mut self,slot:usize)->Result<(),io::Error>{{
  let cap=*EXPECTED.get(slot).ok_or_else(||io::Error::other("slot"))?;
  let value=self.0.get_mut(slot).ok_or_else(||io::Error::other("slot"))?;
  let next=value.checked_add(1).ok_or_else(||io::Error::other("overflow"))?;
  if next>cap {{return Err(io::Error::other("ceiling"));}}
  *value=next;Ok(())
 }}
}}
impl Drop for Calls{{
 /// Report the actual reached prefix, even if the consumer fails.
 fn drop(&mut self){{eprintln!("{MARKER}{fields}",{arguments});}}
}}
macro_rules! call {{($calls:expr,$slot:literal,$value:expr)=>{{{{
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
 let phase=call!(c,15,exchange.offset());
 let mut history=call!(c,6,FixtureAdaptiveHistory::new(raw,64))?;
 for time in [t0,t1,t2]{{
  let row=call!(c,5,FixtureObservation::new(raw,time,phase,1.0))?;
  call!(c,7,history.try_append(row))?;
 }}
 let frames=call!(c,8,FixtureChartFrames::new(b,g,z,j))?;
 let domain=call!(c,9,FixtureChartDomain::new(t0,t2,t4))?;
 let chart=call!(c,10,FixtureAffineChart::new(frames,domain,0.0,0.0))?;
 assert!(call!(c,11,history.prepare_chart(raw,FixtureChartOutcome::Refused))?.is_none());
 let unknown=matches!(call!(c,11,history.prepare_chart(raw,FixtureChartOutcome::Unknown)),
  Err(FixtureChartError::UnknownOutcome));
 assert!(unknown);
 let mismatch=matches!(call!(c,11,history.prepare_chart(target,FixtureChartOutcome::Unknown)),
  Err(FixtureChartError::ContextMismatch));
 assert!(mismatch);
 let view=call!(c,11,history.prepare_chart(raw,FixtureChartOutcome::Known(&chart)))?
  .ok_or_else(||io::Error::other("view"))?;
 assert_eq!(call!(c,15,view.context()),target);
 let terms=call!(c,12,FixtureProjectionTerms::new(0.0,0.0,0.0))?;
 let p=call!(c,13,view.project_tracking(target,2,t2,t4,terms,0.0))?;
 assert_eq!(call!(c,15,p.context()),target);
 assert_eq!(call!(c,15,p.center_seconds()),0.0);
 let center_error=call!(c,15,p.center_roundoff_seconds());
 assert!(center_error.is_finite() && center_error>0.0);
 let tracking=call!(c,15,p.tracking()).ok_or_else(||io::Error::other("tracking"))?;
 assert_eq!(call!(c,15,tracking.phase_seconds()),0.0);
 let phase_error=call!(c,15,tracking.phase_roundoff_seconds());
 assert!(phase_error.is_finite() && phase_error>0.0);
 assert_eq!(call!(c,15,tracking.phase_standard_deviation_seconds()),0.0);
 assert_eq!(call!(c,15,tracking.rate_standard_deviation_seconds_per_second()),0.0);
 let wrapped=call!(c,14,enclose_fixture_chart_projection(&p))?;
 let retained=call!(c,15,wrapped.projection());
 assert_eq!(call!(c,15,retained.context()),target);
 let interval=call!(c,15,wrapped.interval());
 assert!(call!(c,15,interval.lower())<0);
 assert!(call!(c,15,interval.upper())>0);
 assert_eq!(call!(c,15,history.sample_count()),3);
 assert_eq!(call!(c,15,history.auxiliary_count()),0);
 assert!(call!(c,15,history.report()).is_some());
 let raw_projection=call!(c,15,p.source_projection());
 assert_eq!(call!(c,15,raw_projection.context()),raw);
 let actual_chart=call!(c,15,p.chart());
 assert_eq!(call!(c,15,actual_chart.frames()),frames);
 let entries=[wrapped];
 let batch=call!(c,16,FixtureChartBatch::new(&chart,t4,&entries))?;
 let minimum=call!(c,17,FixtureMinimum::new(1))?;
 let selected=call!(c,18,batch.select(minimum))?;
 assert!(ptr::eq(call!(c,19,batch.chart()),ptr::from_ref(&chart)));
 assert_eq!(call!(c,19,batch.projected_at()),t4);
 assert!(ptr::eq(call!(c,19,batch.entries()),entries.as_slice()));
 assert_eq!(call!(c,19,batch.len()),1);
 assert!(!call!(c,19,batch.is_empty()));
 assert!(ptr::eq(call!(c,19,selected.batch()),ptr::from_ref(&batch)));
 let selection=call!(c,19,selected.selection());
 assert_eq!(call!(c,19,selection.population()),1);
 assert_eq!(call!(c,19,selection.minimum()),1);
 assert_eq!(call!(c,19,selection.maximum_depth()),1);
 assert!(call!(c,19,selection.strict_majority()));
 assert!(call!(c,19,selection.minimum_met()));
 assert_eq!(call!(c,19,selection.outcome()),FixtureOutcomeKind::WitnessedFixture);
 let witness=call!(c,19,selection.witnessed()).ok_or_else(||io::Error::other("witness"))?;
 assert_eq!(call!(c,19,witness.component_lower()),-1);
 assert_eq!(call!(c,19,witness.component_upper()),1);
 assert_eq!(call!(c,19,witness.witness()),-1);
 assert_eq!(call!(c,19,witness.intersection_lower()),-1);
 assert_eq!(call!(c,19,witness.intersection_upper()),1);
 let mut supporters=call!(c,19,witness.supporters());
 assert_eq!(call!(c,19,supporters.next()),Some(a));
 assert!(call!(c,19,supporters.next()).is_none());
 let raw_entries=[call!(c,20,enclose_fixture_projection(raw_projection))?];
 let raw_batch=call!(c,21,FixtureProjectedBatch::new(b,g,t4,&raw_entries))?;
 let parameters=call!(c,22,FixturePrincipalParameters::new(0.25,0.0))?;
 let strata=[call!(c,23,FixturePrincipalStratum::new(a,2))?];
 let mut state=call!(c,24,FixturePrincipalState::seed(&[a],parameters,minimum,&raw_batch,&strata))?;
 let ready=call!(c,25,state.advance(&raw_batch,&strata))?;
 assert_eq!(call!(c,27,ready.kind()),FixturePrincipalOutcomeKind::ReferenceReady);
 assert_eq!(call!(c,27,ready.principal_label()),Some(a));
 assert!(call!(c,27,ready.reference_ready()));
 let decision=call!(c,26,state.advance_chart(&chart,&batch,&strata))?;
 assert_eq!(call!(c,27,decision.kind()),FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate);
 assert_eq!(call!(c,27,decision.updated_source()),None);
 assert!(!call!(c,27,decision.reference_ready()));
 assert_eq!(call!(c,27,decision.principal_label()),Some(a));
 let principal_selection=call!(c,27,decision.selection());
 assert_eq!(call!(c,27,principal_selection.minimum()),1);
 let chart_selection=call!(c,27,decision.chart_selection());
 assert!(ptr::eq(call!(c,27,chart_selection.batch()),ptr::from_ref(&batch)));
 assert!(ptr::eq(call!(c,27,decision.batch()),ptr::from_ref(&batch)));
 assert!(ptr::eq(call!(c,27,decision.entries()),entries.as_slice()));
 let principal_entry=call!(c,27,decision.principal_entry()).ok_or_else(||io::Error::other("principal"))?;
 assert!(ptr::eq(principal_entry,ptr::from_ref(&entries[0])));
 let principal_projection=call!(c,27,principal_entry.projection());
 assert!(ptr::eq(call!(c,27,principal_projection.chart()),ptr::from_ref(&chart)));
 let principal_source=call!(c,27,principal_projection.source_projection());
 assert_eq!(call!(c,27,principal_source.context()),raw);
 assert_eq!(call!(c,27,principal_source.latest_observed_at()),t2);
 assert_eq!(call!(c,27,principal_projection.center_seconds()),0.0);
 assert_eq!(call!(c,27,state.frame()),b);
 assert_eq!(call!(c,27,state.mapping_generation()),g);
 assert_eq!(call!(c,27,state.principal()),Some(a));
 assert_eq!(call!(c,27,state.evaluated_at()),t4);
 let source_state=call!(c,27,state.source_state(a)).ok_or_else(||io::Error::other("source"))?;
 assert_eq!(call!(c,27,source_state.latest_observed_at()),Some(t2));
 assert!(!call!(c,27,source_state.pending()));
 assert_eq!(c.0,EXPECTED);
 println!("{SUCCESS}");
 Ok(())
}}
'''


def _negative(prefix: str, expression: str, code: str, tokens: list[str], neighbor: str, *,
              neighbor_file: str = "crates/time-core/tests/chart.rs") -> dict[str, object]:
    """Bind expected diagnostic to its rejected primary source line and legal neighbor."""
    return {"source": prefix + expression + "\n}\nfn main(){}\n", "code": code,
            "tokens": tokens, "line": len(prefix.splitlines()) + 1,
            "external_legal_neighbor": {"file": neighbor_file, "test": neighbor}}


def contract_examples() -> dict[str, dict[str, object]]:
    """Return one real legal consumer source plus24 negative source probes.

    The13 preceding chart negatives retain their exact source and meaning;
    seven batch probes and four retained-principal probes are additive. These do not replace
    the existing2 legal and37 negative specimens outside this module. The
    coordinator must add a chart-specific legal accounting branch, actual source
    digest, finite source reservation and intended-diagnostic checks. The actual
    rustc primary span/code/tokens remain unverified until a future admitted run.
    """
    neighbor = "weighted_tracking_uses_one_common_chart_reference_and_genuine_owner"
    imports = ("use time_core::{FixtureAdaptiveHistory,FixtureAdaptiveChartView,"
               "FixtureAffineChart,FixtureChartFrames,FixtureChartDomain,FixtureChartError,"
               "FixtureChartOutcome,FixtureChartProjection,FixtureChartInterval,"
               "FixtureAnchorProjection,FixtureProjectedInterval,FixtureProjectedBatch,"
               "FixtureProjectedBatchError,FixtureObservation};\n"
               "use time_domain::MonotonicNs;\nuse std::num::NonZeroU64;\n")
    cases: dict[str, dict[str, object]] = {LEGAL_NAME: {"source": legal_contract_source(), "code": None,
                          "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
                          "chart_calls": legal_expected()}}
    for name, typ in (("frames", "FixtureChartFrames"), ("domain", "FixtureChartDomain"),
                      ("chart", "FixtureAffineChart"), ("view", "FixtureAdaptiveChartView<'_, '_>"),
                      ("projection", "FixtureChartProjection<'_>"), ("interval", "FixtureChartInterval<'_>")):
        struct_name = typ.split("<", 1)[0]
        cases["chart_" + name + "_private_update"] = _negative(
            imports + f"fn bypass(value:{typ}){{\n", f" let _={struct_name}{{..value}};", "E0451",
            [struct_name, "private"], neighbor)
    for name, typ in (("chart", "FixtureAffineChart"), ("projection", "FixtureChartProjection<'_>")):
        cases["chart_" + name + "_default"] = _negative(
            imports + "fn bypass(){\n", f" let _:{typ}=Default::default();", "E0277",
            [typ.split("<", 1)[0], "Default"], neighbor)
    cases["chart_live_history_borrow"] = _negative(
        imports + "fn bypass(history:&mut FixtureAdaptiveHistory,chart:&FixtureAffineChart,row:FixtureObservation){\n"
        " let view=history.prepare_chart(history.context(),FixtureChartOutcome::Known(chart)).unwrap().unwrap();\n",
        " let _=history.try_append(row);\n let _=&view;", "E0502", ["history", "borrow"], neighbor)
    cases["chart_descriptor_lifetime"] = _negative(
        imports + "fn bypass<'a>(history:&'a FixtureAdaptiveHistory,frames:FixtureChartFrames,domain:FixtureChartDomain)"
        "->Result<FixtureAdaptiveChartView<'a,'a>,FixtureChartError>{\n"
        " let chart=FixtureAffineChart::new(frames,domain,0.0,0.0)?;\n",
        " history.prepare_chart(history.context(),FixtureChartOutcome::Known(&chart))?.ok_or(FixtureChartError::Invariant)",
        "E0515", ["chart", "cannot return value referencing"], neighbor)
    cases["chart_to_legacy_projection"] = _negative(
        imports + "fn bypass(value:FixtureChartProjection<'_>)->FixtureAnchorProjection{\n",
        " value", "E0308", ["FixtureAnchorProjection", "FixtureChartProjection"], neighbor)
    cases["chart_to_legacy_interval"] = _negative(
        imports + "fn bypass(value:FixtureChartInterval<'_>)->FixtureProjectedInterval{\n",
        " value", "E0308", ["FixtureProjectedInterval", "FixtureChartInterval"], neighbor)
    cases["chart_interval_in_old_batch"] = _negative(
        imports + "fn bypass<'a>(frame:NonZeroU64,generation:NonZeroU64,at:MonotonicNs,"
        "values:&'a [FixtureChartInterval<'a>])->Result<FixtureProjectedBatch<'a>,FixtureProjectedBatchError>{\n",
        " FixtureProjectedBatch::new(frame,generation,at,values)", "E0308",
        ["FixtureProjectedInterval", "FixtureChartInterval"], neighbor)
    batch_imports = (imports + "use time_core::{FixtureChartBatch,FixtureChartSelection,FixtureMinimum};\n")
    batch_neighbor = "chart_batch_cases::chart_batch_selects_genuine_majority_at_common_time"
    batch_file = "crates/time-core/tests/private/chart_batch_cases.rs"
    cases["chart_batch_private_update"] = _negative(
        batch_imports + "fn bypass<'entries,'chart>(value:FixtureChartBatch<'entries,'chart>){\n",
        " let _=FixtureChartBatch{..value};", "E0451", ["FixtureChartBatch", "private"],
        batch_neighbor, neighbor_file=batch_file)
    cases["chart_selection_private_update"] = _negative(
        batch_imports + "fn bypass<'batch,'entries,'chart>(value:FixtureChartSelection<'batch,'entries,'chart>){\n",
        " let _=FixtureChartSelection{..value};", "E0451", ["FixtureChartSelection", "private"],
        batch_neighbor, neighbor_file=batch_file)
    cases["chart_selection_batch_lifetime"] = _negative(
        batch_imports + "fn bypass<'entries,'chart>(chart:&'chart FixtureAffineChart,at:MonotonicNs,"
        "values:&'entries [FixtureChartInterval<'chart>],minimum:FixtureMinimum)"
        "->FixtureChartSelection<'entries,'entries,'chart>{\n"
        " let batch=FixtureChartBatch::new(chart,at,values).unwrap();\n",
        " batch.select(minimum).unwrap()", "E0515", ["batch", "cannot return value referencing"],
        batch_neighbor, neighbor_file=batch_file)
    cases["chart_batch_descriptor_lifetime"] = _negative(
        batch_imports + "fn bypass<'entries,'chart>(original:FixtureAffineChart,at:MonotonicNs,"
        "values:&'entries [FixtureChartInterval<'chart>])->FixtureChartBatch<'entries,'chart>{\n"
        " let chart=original;\n",
        " FixtureChartBatch::new(&chart,at,values).unwrap()", "E0515", ["chart", "cannot return value referencing"],
        batch_neighbor, neighbor_file=batch_file)
    cases["chart_batch_entries_lifetime"] = _negative(
        batch_imports + "fn bypass<'chart>(chart:&'chart FixtureAffineChart,at:MonotonicNs,"
        "value:FixtureChartInterval<'chart>)->FixtureChartBatch<'chart,'chart>{\n"
        " let entries=[value];\n",
        " FixtureChartBatch::new(chart,at,&entries).unwrap()", "E0515", ["entries", "cannot return value referencing"],
        batch_neighbor, neighbor_file=batch_file)
    cases["chart_interval_into_legacy"] = _negative(
        batch_imports + "fn bypass(value:FixtureChartInterval<'_>)->FixtureProjectedInterval{\n",
        " value.into()", "E0277", ["FixtureProjectedInterval", "FixtureChartInterval", "From"],
        batch_neighbor, neighbor_file=batch_file)
    cases["chart_batch_live_slice_borrow"] = _negative(
        batch_imports + "fn bypass<'entries,'chart>(chart:&'chart FixtureAffineChart,at:MonotonicNs,"
        "values:&'entries mut [FixtureChartInterval<'chart>],minimum:FixtureMinimum){\n"
        " let batch=FixtureChartBatch::new(chart,at,values).unwrap();\n"
        " let selected=batch.select(minimum).unwrap();\n",
        " values.reverse();\n let _=selected.selection().population();", "E0502", ["values", "mutable", "immutable"],
        batch_neighbor, neighbor_file=batch_file)
    principal_imports = (
        "use time_core::{FixtureAffineChart,FixtureChartBatch,FixtureChartPrincipalDecision,"
        "FixturePrincipalState,FixturePrincipalStratum,FixtureSupporterUncertainty,"
        "combine_fixture_supporters};\n")
    principal_neighbor = "chart_principal_cases::retained_owner_preserves_credit_and_actual_chart_provenance"
    principal_file = "crates/time-core/tests/private/chart_principal_cases.rs"
    cases["chart_principal_private_update"] = _negative(
        principal_imports + "fn bypass<'batch,'entries,'chart>(value:FixtureChartPrincipalDecision<'batch,'entries,'chart>){\n",
        " let _=FixtureChartPrincipalDecision{..value};", "E0451",
        ["FixtureChartPrincipalDecision", "private"], principal_neighbor,
        neighbor_file=principal_file)
    cases["chart_principal_batch_lifetime"] = _negative(
        principal_imports + "fn bypass<'entries,'chart>(state:&mut FixturePrincipalState,chart:&FixtureAffineChart,"
        "batch:FixtureChartBatch<'entries,'chart>,strata:&[FixturePrincipalStratum])"
        "->FixtureChartPrincipalDecision<'entries,'entries,'chart>{\n",
        " state.advance_chart(chart,&batch,strata).unwrap()", "E0515",
        ["batch", "cannot return value referencing"], principal_neighbor,
        neighbor_file=principal_file)
    cases["chart_principal_into_legacy_consumer"] = _negative(
        principal_imports + "fn bypass(value:&FixtureChartPrincipalDecision<'_,'_,'_>){\n",
        " let _=combine_fixture_supporters(value,&[]);", "E0308",
        ["FixturePrincipalDecision", "FixtureChartPrincipalDecision"], principal_neighbor,
        neighbor_file=principal_file)
    cases["chart_principal_legacy_uncertainty"] = _negative(
        principal_imports + "fn bypass(value:&FixtureChartPrincipalDecision<'_,'_,'_>){\n"
        " let entry=value.principal_entry().unwrap();\n",
        " let _=FixtureSupporterUncertainty::new(entry,0.0,0.0,0.0);", "E0308",
        ["FixtureProjectedInterval", "FixtureChartInterval"], principal_neighbor,
        neighbor_file=principal_file)
    return cases


# Source-reviewed mechanism gap map, not claims of generated-fault sensitivity.
GARDE_CARRIERS = {
    "FramesCandidate": "frames_rule: source/target frame and generation relationship",
    "DomainCandidate": "domain_rule: order and maximum exact-nanosecond span",
    "AffineCandidate": "signed_rule: admitted displacement and rate",
    "HistoryCandidate": "history_rule: whole actual owner and retained-row domain",
    "ProjectionCandidate": "signed_rule/bound_rule: transformed center, rate and both bounds",
    "TrackingPhaseCandidate": "signed_rule/bound_rule: tracked phase and its bound",
}


# A separate current registry leaves every retained historical caller unchanged.
COMPOSITION_LEGAL_NAME = "legal_chart_composition_and_retained_bounds"
COMPOSITION_MARKER = "CORE_CHART_COMPOSITION_CONTRACT_PROFILE"
COMPOSITION_SUCCESS = "legal-chart-composition-contract"
COMPOSITION_FIELDS = ("labels", "monotonic", "frames", "domains", "charts",
                      "compositions", "getters")
COMPOSITION_EXPECTED = (10, 3, 5, 1, 5, 3, 28)
COMPOSITION_SOURCE_SHA256 = {
    "crates/time-core/src/regression/chart.rs":
        "8c1eaae20f287b96f0859b79a4522cf3176ead59f5f46a35c5cfa797aded2221",
    "crates/time-core/src/regression/chart/roundoff.rs":
        "1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc",
    "crates/time-core/src/chart_batch.rs":
        "b25ac73756f94bd2be9692130d8dee201bd8c2b0a9c0adf63c7eba929740c79c",
}
COMPOSITION_GARDE_CARRIERS = {
    "CompositionCandidate": "composition_error: actual chain and complete domain",
    "AffineCandidate": "admit: phase/rate and both coefficient bounds, in direct new and composed output",
}


def composition_legal_expected() -> dict[str, int]:
    """Return direct external attempts; no fit or nested bound work is measured."""
    return dict(zip(COMPOSITION_FIELDS, COMPOSITION_EXPECTED, strict=True))


def composition_contract_counts(output: object) -> dict[str, int]:
    """Require the complete focused composition row and sentinel, LF or CRLF."""
    if type(output) is not str or len(output) > 4096:
        raise Incomplete("Composition legal output is not bounded text.")
    lines = output.splitlines(keepends=True)
    if len(lines) != 2 or any(not line.endswith("\n") for line in lines):
        raise Incomplete("Composition legal output has missing or extra records.")
    lines = [line[:-2] if line.endswith("\r\n") else line[:-1] for line in lines]
    record = COMPOSITION_MARKER + "".join(
        f" {name}={value}" for name, value in composition_legal_expected().items())
    if sorted(lines) != sorted((record, COMPOSITION_SUCCESS)):
        raise Incomplete("Composition legal source-work row or sentinel differs.")
    return composition_legal_expected()


def composition_legal_source() -> str:
    """Use both composed operands and retain independently known lost units.

    The exact stored-input phase/rate sum is 2^53+1-2^53+0=1. Each separately
    rounded point is zero; both inherited coefficient bounds must still enclose
    the lost unit. A direct zero descriptor has zero coefficient error. Whole
    operand scalar-bit snapshots remain unchanged. There are no histories,
    projections, fits, appends, lifecycle advances or observation-credit effects.
    Counts describe the 55 direct calls, not nested Garde/outward work; the
    accounting owner must separately fund all five affine admissions and three
    complete composition paths. Failed prefixes never reduce this reservation.
    """
    expected = ",".join(str(value) for value in COMPOSITION_EXPECTED)
    fields = "".join(f" {name}={{}}" for name in COMPOSITION_FIELDS)
    arguments = ",".join(f"self.0[{i}]" for i in range(len(COMPOSITION_FIELDS)))
    return f'''use std::{{error::Error,io,num::NonZeroU64}};
use time_core::{{FixtureAffineChart,FixtureChartDomain,FixtureChartFrames}};
use time_domain::MonotonicNs;
const EXPECTED:[u64;7]=[{expected}];
struct Calls([u64;7]);
impl Calls{{
 /// Precharge each direct call and reject a source census overrun.
 fn charge(&mut self,slot:usize)->Result<(),io::Error>{{
  let cap=*EXPECTED.get(slot).ok_or_else(||io::Error::other("slot"))?;
  let value=self.0.get_mut(slot).ok_or_else(||io::Error::other("slot"))?;
  let next=value.checked_add(1).ok_or_else(||io::Error::other("overflow"))?;
  if next>cap {{return Err(io::Error::other("ceiling"));}}
  *value=next;Ok(())
 }}
}}
impl Drop for Calls{{
 /// Retain reached prefixes without converting them into complete evidence.
 fn drop(&mut self){{eprintln!("{COMPOSITION_MARKER}{fields}",{arguments});}}
}}
macro_rules! call {{($calls:expr,$slot:literal,$value:expr)=>{{{{
 $calls.charge($slot)?;$value
}}}};}}
fn main()->Result<(),Box<dyn Error>>{{
 let mut c=Calls([0;7]);
 let a=call!(c,0,NonZeroU64::new(11)).ok_or_else(||io::Error::other("label"))?;
 let ga=call!(c,0,NonZeroU64::new(17)).ok_or_else(||io::Error::other("label"))?;
 let b=call!(c,0,NonZeroU64::new(19)).ok_or_else(||io::Error::other("label"))?;
 let gb=call!(c,0,NonZeroU64::new(23)).ok_or_else(||io::Error::other("label"))?;
 let cc=call!(c,0,NonZeroU64::new(29)).ok_or_else(||io::Error::other("label"))?;
 let gc=call!(c,0,NonZeroU64::new(31)).ok_or_else(||io::Error::other("label"))?;
 let d=call!(c,0,NonZeroU64::new(37)).ok_or_else(||io::Error::other("label"))?;
 let gd=call!(c,0,NonZeroU64::new(41)).ok_or_else(||io::Error::other("label"))?;
 let e=call!(c,0,NonZeroU64::new(43)).ok_or_else(||io::Error::other("label"))?;
 let ge=call!(c,0,NonZeroU64::new(47)).ok_or_else(||io::Error::other("label"))?;
 let start=call!(c,1,MonotonicNs::new(0));
 let reference=call!(c,1,MonotonicNs::new(1_000_000_000));
 let end=call!(c,1,MonotonicNs::new(2_000_000_000));
 let domain=call!(c,3,FixtureChartDomain::new(start,reference,end))?;
 let ab=call!(c,2,FixtureChartFrames::new(a,ga,b,gb))?;
 let bc=call!(c,2,FixtureChartFrames::new(b,gb,cc,gc))?;
 let cd=call!(c,2,FixtureChartFrames::new(cc,gc,d,gd))?;
 let de=call!(c,2,FixtureChartFrames::new(d,gd,e,ge))?;
 let ae=call!(c,2,FixtureChartFrames::new(a,ga,e,ge))?;
 let big=9_007_199_254_740_992.0;
 let first=call!(c,4,FixtureAffineChart::new(ab,domain,big,big))?;
 let unit=call!(c,4,FixtureAffineChart::new(bc,domain,1.0,1.0))?;
 let negative=call!(c,4,FixtureAffineChart::new(cd,domain,-big,-big))?;
 let zero=call!(c,4,FixtureAffineChart::new(de,domain,0.0,0.0))?;
 let direct=call!(c,4,FixtureAffineChart::new(ae,domain,0.0,0.0))?;
 assert_eq!(call!(c,6,first.phase_coefficient_roundoff_seconds()).to_bits(),0);
 assert_eq!(call!(c,6,first.rate_coefficient_roundoff_seconds_per_second()).to_bits(),0);
 assert_eq!(call!(c,6,direct.phase_coefficient_roundoff_seconds()).to_bits(),0);
 assert_eq!(call!(c,6,direct.rate_coefficient_roundoff_seconds_per_second()).to_bits(),0);
 let left=call!(c,5,first.compose(&unit))?;
 let right=call!(c,5,negative.compose(&zero))?;
 let left_before=[call!(c,6,left.phase_at_reference_seconds()).to_bits(),
  call!(c,6,left.rate_seconds_per_second()).to_bits(),
  call!(c,6,left.phase_coefficient_roundoff_seconds()).to_bits(),
  call!(c,6,left.rate_coefficient_roundoff_seconds_per_second()).to_bits()];
 let right_before=[call!(c,6,right.phase_at_reference_seconds()).to_bits(),
  call!(c,6,right.rate_seconds_per_second()).to_bits(),
  call!(c,6,right.phase_coefficient_roundoff_seconds()).to_bits(),
  call!(c,6,right.rate_coefficient_roundoff_seconds_per_second()).to_bits()];
 let combined=call!(c,5,left.compose(&right))?;
 assert_eq!(call!(c,6,combined.phase_at_reference_seconds()).to_bits(),0);
 assert_eq!(call!(c,6,combined.rate_seconds_per_second()).to_bits(),0);
 let phase_error=call!(c,6,combined.phase_coefficient_roundoff_seconds());
 let rate_error=call!(c,6,combined.rate_coefficient_roundoff_seconds_per_second());
 // The new sum's exact u*(abs(2^53)+abs(-2^53)) term is2. Both
 // inherited stored bounds must be added too; mere lost-unit containment
 // would accept dropping either operand under this conservative fresh term.
 let phase_inherited=f64::from_bits(left_before[2])+f64::from_bits(right_before[2]);
 let rate_inherited=f64::from_bits(left_before[3])+f64::from_bits(right_before[3]);
 assert!(phase_error>=phase_inherited+2.0);
 assert!(rate_error>=rate_inherited+2.0);
 assert!((1.0..8.0).contains(&phase_error));
 assert!((1.0..8.0).contains(&rate_error));
 assert_eq!(call!(c,6,combined.frames()),ae);
 assert_eq!(call!(c,6,combined.domain()),domain);
 assert_eq!([call!(c,6,left.phase_at_reference_seconds()).to_bits(),
  call!(c,6,left.rate_seconds_per_second()).to_bits(),
  call!(c,6,left.phase_coefficient_roundoff_seconds()).to_bits(),
  call!(c,6,left.rate_coefficient_roundoff_seconds_per_second()).to_bits()],left_before);
 assert_eq!([call!(c,6,right.phase_at_reference_seconds()).to_bits(),
  call!(c,6,right.rate_seconds_per_second()).to_bits(),
  call!(c,6,right.phase_coefficient_roundoff_seconds()).to_bits(),
  call!(c,6,right.rate_coefficient_roundoff_seconds_per_second()).to_bits()],right_before);
 assert_eq!(call!(c,6,first.phase_at_reference_seconds()),big);
 assert_eq!(call!(c,6,first.rate_seconds_per_second()),big);
 assert_eq!(c.0,EXPECTED);
 println!("{COMPOSITION_SUCCESS}");
 Ok(())
}}
'''


def composition_contract_examples() -> dict[str, dict[str, object]]:
    """Return only one focused legal caller and six coefficient-bound probes.

    This registry is selected explicitly for the current three affected owners;
    it neither replaces nor silently reruns the old chart/selection/principal
    registry. Negative sources invoke no numerical producer. Intended rustc
    diagnostics remain unverified until the coordinator's admitted execution.
    """
    cases: dict[str, dict[str, object]] = {
        COMPOSITION_LEGAL_NAME: {
            "source": composition_legal_source(), "code": None,
            "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
            "chart_composition_calls": composition_legal_expected(),
        },
    }
    neighbor = "chart_composition::composition_retains_both_operand_errors_through_signed_elapsed"
    neighbor_file = "crates/time-core/tests/private/chart_composition.rs"
    imports = "use time_core::FixtureAffineChart;\n"
    for name, field, getter in (
        ("phase", "phase_error", "phase_coefficient_roundoff_seconds"),
        ("rate", "rate_error", "rate_coefficient_roundoff_seconds_per_second"),
    ):
        cases[f"chart_composition_{name}_bound_private_update"] = _negative(
            imports + "fn bypass(value:FixtureAffineChart){\n",
            f" let _=FixtureAffineChart{{{field}:0.0,..value}};", "E0451",
            ["FixtureAffineChart", "private"], neighbor, neighbor_file=neighbor_file)
        cases[f"chart_composition_{name}_bound_private_write"] = _negative(
            imports + "fn bypass(value:&mut FixtureAffineChart){\n",
            f" value.{field}=0.0;", "E0616", [field, "private"],
            neighbor, neighbor_file=neighbor_file)
        cases[f"chart_composition_{name}_bound_owned_getter"] = _negative(
            imports + "fn bypass(value:&FixtureAffineChart){\n",
            f" let _: &mut f64=value.{getter}();", "E0308", ["&mut f64", "f64"],
            neighbor, neighbor_file=neighbor_file)
    return cases
