"""Fixed external consumers for the coupled S1.4 frequency/phase fixture.

GPL-2.0-only, David Osipov with AI assistance, 8 November 2026.
These are source-bound compiler/executable fixtures, not observed Rust results.
The existing Core executor retains tool/source/dependency custody and the strict
JSON diagnostic reader. No compiler, process or generic runner is added here.
"""
from __future__ import annotations


INPUTS = ("scripts/core-frequency-contracts.py", "scripts/tests/test_core_frequency_contracts.py")
LEGAL_NAME = "frequency_legal"
LEGAL_MARKER = "legal-frequency-contract\n"
NEW_LIMITS = {"legal_executions": 1, "negative_compilations": 11}
LIMITS = {"legal_executions": 2, "negative_compilations": 18}
LEGACY_CODES = {
    "phase_plan_legal": None,
    "phase_plan_producer_lease": "E0502",
    "phase_plan_live_proposal": "E0505",
    "phase_plan_double_transfer": "E0499",
    "phase_plan_retirement_escape": "E0515",
    "phase_plan_baseline_literal": "E0451",
    "phase_plan_profile_mutation": "E0616",
    "phase_plan_cursor_retarget": "E0616",
}
LEGAL_SCOPE = (
    "Both existing phase and new frequency legal binaries execute checked constructor assertions. "
    "The frequency binary also executes actual enabled construction, wrong-association refusal, "
    "NoModel and genuine empty-batch NoReady receiving. Typed proposal, history/projection "
    "and retirement helpers compile but are not called; the typed intake helper executes only "
    "for that empty batch. No fresh-fit learning trajectory is claimed. Actual nonempty producer trajectories "
    "remain separately owned by package tests."
)


def _legal_source():
    """Keep actual constructor execution distinct from compiled typed neighbors."""
    return r'''use std::num::NonZeroU64;
use time_core::{FixtureAffineChart, FixtureChartBatch, FixtureChartDomain, FixtureChartFrames,
    FixtureCombinationParameters, FixtureCombinationState, FixtureFrequencyAdmission,
    FixtureFrequencyClockAssociation, FixtureFrequencyError, FixtureFrequencyHistory,
    FixtureFrequencyProfile, FixtureFrequencyProjection, FixtureMinimum, FixturePhaseBaseline,
    FixturePhaseError, FixturePhaseNow, FixturePhasePlanOwner, FixturePhaseProfile,
    FixturePhaseReferenceAdmission, FixturePhaseResidualPermission, FixturePhaseRetirement,
    FixturePrincipalParameters, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateSettlementCursor};
use time_domain::{ActiveAwakeNs, MonotonicDurationNs, MonotonicNs};
type Checked<T> = Result<T, Box<dyn std::error::Error>>;
fn nz(value:u64)->Checked<NonZeroU64> { Ok(NonZeroU64::new(value).ok_or("zero fixture label")?) }
fn legal_profile(age:MonotonicDurationNs)->Result<FixtureFrequencyProfile,FixtureFrequencyError> {
    FixtureFrequencyProfile::new(1.0/1024.0,0.0,1.0/8.0,age)
}
fn legal_association(frame:NonZeroU64,generation:NonZeroU64,origin:NonZeroU64,
    clock_generation:NonZeroU64)->FixtureFrequencyClockAssociation {
    FixtureFrequencyClockAssociation::new(frame,generation,origin,clock_generation)
}
fn enabled_lease<'producer,'chart>(producer:&'producer mut FixtureCombinationState,
    chart:&'chart FixtureAffineChart,cursor:FixtureRateSettlementCursor,now:FixturePhaseNow,
    phase:FixturePhaseProfile,base:FixturePhaseBaseline,profile:FixtureFrequencyProfile,
    association:FixtureFrequencyClockAssociation)->Checked<FixturePhasePlanOwner<'producer,'chart>> {
    Ok(FixturePhasePlanOwner::new_with_frequency(producer,chart,cursor,now,phase,base,profile,association)?)
}
fn legal_enabled_release(producer:&mut FixtureCombinationState,chart:&FixtureAffineChart,
    cursor:FixtureRateSettlementCursor,now:FixturePhaseNow,phase:FixturePhaseProfile,
    base:FixturePhaseBaseline,profile:FixtureFrequencyProfile,association:FixtureFrequencyClockAssociation)
    ->Checked<()> {
    { let owner=enabled_lease(producer,chart,cursor,now,phase,base,profile,association)?;
      let _=std::hint::black_box(owner.cursor()); }
    let _=std::hint::black_box(producer.emitted_rounds());
    Ok(())
}
fn legal_intake<'chart>(owner:&mut FixturePhasePlanOwner<'_,'chart>,
    batch:&FixtureChartBatch<'_,'chart>,strata:&[FixturePrincipalStratum],now:FixturePhaseNow)
    ->Result<FixturePhaseReferenceAdmission,FixturePhaseError> {
    owner.receive_reference(batch,strata,now)
}
fn legal_history(history:&FixtureFrequencyHistory)->(f64,f64) {
    (history.mean_seconds_per_second(),history.skew_seconds_per_second())
}
fn legal_projection_snapshot(projection:&FixtureFrequencyProjection)->MonotonicNs {
    projection.evaluated_at()
}
fn legal_owner_history(owner:&FixturePhasePlanOwner<'_,'_>)->Option<FixtureFrequencyHistory> {
    owner.frequency_history()
}
fn legal_after_proposal(mut owner:FixturePhasePlanOwner<'_,'_>,now:FixturePhaseNow)->Checked<()> {
    { let proposal=owner.issue(now,FixturePhaseResidualPermission::Allow)?;
      let _=std::hint::black_box(proposal.ordinal()); }
    let _=owner.project_model(now);
    Ok(())
}
fn legal_resume<'producer,'chart>(retired:&'chart mut FixturePhaseRetirement<'producer>,
    now:FixturePhaseNow)->Result<FixturePhasePlanOwner<'producer,'chart>,FixturePhaseError> {
    retired.resume(now,FixturePhaseResidualPermission::Allow)
}
fn main()->Checked<()> {
    let age=MonotonicNs::new(32_000_000_000).duration_since(MonotonicNs::new(0))?;
    let profile=legal_profile(age)?;
    assert_eq!(profile.maximum_measured_skew(),1.0/1024.0);
    assert_eq!(profile.maximum_clock_error_rate(),0.0);
    assert_eq!(profile.growth_cap(),1.0/8.0);
    assert_eq!(profile.maximum_age(),age);
    for args in [(f64::NAN,0.0,1.0,age),(0.0,-1.0,1.0,age),
                 (0.0,0.0,0.0,age),(0.0,0.0,1.0,MonotonicNs::new(0).duration_since(MonotonicNs::new(0))?),
                 (0.0,0.0,1.0,MonotonicNs::new((1_u64<<53)+1).duration_since(MonotonicNs::new(0))?)] {
        assert!(matches!(FixtureFrequencyProfile::new(args.0,args.1,args.2,args.3),
            Err(FixtureFrequencyError::InvalidProfile)));
    }
    let boundary=FixtureFrequencyProfile::new(-0.0,-0.0,f64::MIN_POSITIVE,
        MonotonicNs::new(1_u64<<53).duration_since(MonotonicNs::new(0))?)?;
    assert_eq!(boundary.maximum_measured_skew().to_bits(),(-0.0_f64).to_bits());
    assert_eq!(boundary.maximum_clock_error_rate().to_bits(),(-0.0_f64).to_bits());
    assert_eq!(boundary.maximum_age().nanoseconds(),1_u64<<53);
    let frames=FixtureChartFrames::new(nz(17)?,nz(23)?,nz(19)?,nz(29)?)?;
    let domain=FixtureChartDomain::new(MonotonicNs::new(0),MonotonicNs::new(0),
        MonotonicNs::new(64_000_000_000))?;
    let chart=FixtureAffineChart::new(frames,domain,0.0,0.0)?;
    let empty=FixtureProjectedBatch::new(nz(17)?,nz(23)?,MonotonicNs::new(0),&[])?;
    let mut producer=FixtureCombinationState::seed(&[nz(11)?],FixturePrincipalParameters::new(1.0/1024.0,0.0)?,
        FixtureCombinationParameters::new(2.0,0.0)?,FixtureMinimum::new(1)?,&empty,&[])?;
    let context=FixtureRateClockContext::new(nz(101)?,nz(103)?,nz(107)?);
    let now=FixturePhaseNow::new(context,ActiveAwakeNs::new(0),MonotonicNs::new(0));
    let phase=FixturePhaseProfile::new(ActiveAwakeNs::new(8_000_000_000).duration_since(ActiveAwakeNs::new(0))?,
        MonotonicNs::new(16_000_000_000).duration_since(MonotonicNs::new(0))?,1.0/16.0,1.0/128.0,1.0/64.0)?;
    let base=FixturePhaseBaseline::new(1.0/1024.0)?;
    let cursor=FixtureRateSettlementCursor::new(&chart,context,nz(1)?);
    let foreign=legal_association(nz(999)?,nz(23)?,nz(101)?,nz(107)?);
    let cursor=match FixturePhasePlanOwner::new_with_frequency(&mut producer,&chart,cursor,now,
        phase,base,profile,foreign) {
        Err(rejected)=>{let(cursor,error)=rejected.into_parts();
            assert_eq!(error,FixturePhaseError::Frequency{source:FixtureFrequencyError::ClockAssociationMismatch});cursor},
        Ok(_)=>return Err("foreign association was admitted".into()),
    };
    assert_eq!(cursor.status(),FixtureRateCursorStatus::Pending);
    assert_eq!(cursor.next_ordinal(),Some(nz(1)?));
    let association=legal_association(nz(17)?,nz(23)?,nz(101)?,nz(107)?);
    assert_eq!(format!("{association:?}"),"FixtureFrequencyClockAssociation { .. }");
    let mut owner=enabled_lease(&mut producer,&chart,cursor,now,phase,base,profile,association)?;
    assert_eq!(owner.frequency_admission(),FixtureFrequencyAdmission::NoReady{emitted_round:None});
    assert!(owner.frequency_history().is_none());
    assert_eq!(owner.project_model(now).err(),Some(FixtureFrequencyError::NoModel));
    let batch=FixtureChartBatch::new(&chart,MonotonicNs::new(1),&[])?;
    let at=FixturePhaseNow::new(context,ActiveAwakeNs::new(0),MonotonicNs::new(1));
    assert!(matches!(legal_intake(&mut owner,&batch,&[],at)?,
        FixturePhaseReferenceAdmission::AdmittedNoPlan{emitted_round:None,..}));
    assert_eq!(owner.frequency_admission(),FixtureFrequencyAdmission::NoReady{emitted_round:None});
    assert!(owner.frequency_history().is_none());
    assert_eq!(owner.cursor().next_ordinal(),Some(nz(1)?));
    assert_eq!(owner.project_model(at).err(),Some(FixtureFrequencyError::NoModel));
    drop(owner);
    assert_eq!(producer.emitted_rounds(),0);
    println!("legal-frequency-contract");
    Ok(())
}
'''


def _negative(source, code, rejected_line, tokens, positive_function):
    """Bind one primary operation and semantic tokens, never rendered diagnostics."""
    lines = source.splitlines()
    if lines.count(rejected_line) != 1:
        raise ValueError("Frequency specimen lacks its unique rejected operation.")
    return {"source": source, "code": code, "line": lines.index(rejected_line) + 1,
            "tokens": list(tokens), "external_legal_neighbor": {
                "specimen": LEGAL_NAME, "function": positive_function,
                "scope": "compiled typed neighbor; constructor/NoReady assertions execute in main"}}


def examples():
    """Return the closed new1+11 registry; callers cannot supply its truth via policy."""
    rows = [(LEGAL_NAME, {"source": _legal_source(), "code": None})]

    def add(name, source, code, line, tokens, positive):
        rows.append((name, _negative(source, code, line, tokens, positive)))

    add("frequency_profile_literal", '''use time_core::FixtureFrequencyProfile;
use time_domain::MonotonicNs;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let age=MonotonicNs::new(1).duration_since(MonotonicNs::new(0))?;
    let _ = FixtureFrequencyProfile { max_skew:0.0, clock_error:0.0, growth_cap:1.0, max_age:age };
    Ok(())
}
''', "E0451", "    let _ = FixtureFrequencyProfile { max_skew:0.0, clock_error:0.0, growth_cap:1.0, max_age:age };",
        ("FixtureFrequencyProfile", "private"), "legal_profile")
    add("frequency_profile_mutation", '''use time_core::FixtureFrequencyProfile;
fn mutate(profile:&mut FixtureFrequencyProfile) {
    profile.growth_cap = 0.0;
}
fn main() {}
''', "E0616", "    profile.growth_cap = 0.0;", ("growth_cap", "private"), "legal_profile")
    add("frequency_association_literal", '''use time_core::FixtureFrequencyClockAssociation;
use std::num::NonZeroU64;
fn main() {
    let _ = FixtureFrequencyClockAssociation { source_frame:NonZeroU64::MIN, source_generation:NonZeroU64::MIN, monotonic_origin:NonZeroU64::MIN, clock_generation:NonZeroU64::MIN };
}
''', "E0451", "    let _ = FixtureFrequencyClockAssociation { source_frame:NonZeroU64::MIN, source_generation:NonZeroU64::MIN, monotonic_origin:NonZeroU64::MIN, clock_generation:NonZeroU64::MIN };",
        ("FixtureFrequencyClockAssociation", "private"), "legal_association")
    add("frequency_association_mutation", '''use time_core::FixtureFrequencyClockAssociation;
use std::num::NonZeroU64;
fn mutate(association:&mut FixtureFrequencyClockAssociation) {
    association.monotonic_origin = NonZeroU64::MIN;
}
fn main() {}
''', "E0616", "    association.monotonic_origin = NonZeroU64::MIN;",
        ("monotonic_origin", "private"), "legal_association")
    add("frequency_history_mutation", '''use time_core::FixtureFrequencyHistory;
fn mutate(history:&mut FixtureFrequencyHistory) {
    history.model = history.model;
}
fn main() {}
''', "E0616", "    history.model = history.model;", ("model", "private"), "legal_history")
    add("frequency_projection_mutation", '''use time_core::FixtureFrequencyProjection;
use time_domain::MonotonicNs;
fn mutate(projection:&mut FixtureFrequencyProjection) {
    projection.evaluated = MonotonicNs::new(0);
}
fn main() {}
''', "E0616", "    projection.evaluated = MonotonicNs::new(0);",
        ("evaluated", "private"), "legal_projection_snapshot")
    add("frequency_raw_learn", '''use time_core::FixturePhasePlanOwner;
fn bypass(owner:&mut FixturePhasePlanOwner<'_,'_>) {
    owner.learn(0.0,0.0);
}
fn main() {}
''', "E0599", "    owner.learn(0.0,0.0);", ("learn", "FixturePhasePlanOwner"), "legal_intake")
    add("frequency_projection_as_reference", '''use time_core::{FixtureFrequencyProjection,FixturePhaseNow,FixturePhasePlanOwner,FixturePrincipalStratum};
fn substitute(owner:&mut FixturePhasePlanOwner<'_,'_>,projection:FixtureFrequencyProjection,
    strata:&[FixturePrincipalStratum],now:FixturePhaseNow) {
    let _ = owner.receive_reference(&projection,strata,now);
}
fn main() {}
''', "E0308", "    let _ = owner.receive_reference(&projection,strata,now);",
        ("FixtureChartBatch", "FixtureFrequencyProjection"), "legal_intake")
    add("frequency_enabled_producer_lease", '''use time_core::{FixtureAffineChart,FixtureCombinationState,FixtureFrequencyClockAssociation,
    FixtureFrequencyProfile,FixturePhaseBaseline,FixturePhaseNow,FixturePhasePlanOwner,
    FixturePhaseProfile,FixtureRateSettlementCursor};
fn overlap(producer:&mut FixtureCombinationState,chart:&FixtureAffineChart,
    cursor:FixtureRateSettlementCursor,now:FixturePhaseNow,phase:FixturePhaseProfile,
    base:FixturePhaseBaseline,profile:FixtureFrequencyProfile,association:FixtureFrequencyClockAssociation)
    -> Result<(),Box<dyn std::error::Error>> {
    let owner = FixturePhasePlanOwner::new_with_frequency(producer,chart,cursor,now,phase,base,profile,association)?;
    let _ = std::hint::black_box(producer.emitted_rounds());
    let _ = std::hint::black_box(owner);
    Ok(())
}
fn main() {}
''', "E0502", "    let _ = std::hint::black_box(producer.emitted_rounds());",
        ("producer", "immutable"), "legal_enabled_release")
    add("frequency_live_proposal_projection", '''use time_core::{FixturePhaseNow,FixturePhasePlanOwner,FixturePhaseResidualPermission};
fn overlap(mut owner:FixturePhasePlanOwner<'_,'_>,now:FixturePhaseNow)
    ->Result<(),Box<dyn std::error::Error>> {
    let proposal = owner.issue(now,FixturePhaseResidualPermission::Allow)?;
    let prediction = owner.project_model(now);
    let _ = std::hint::black_box(proposal.ordinal());
    let _ = std::hint::black_box(prediction);
    Ok(())
}
fn main() {}
''', "E0499", "    let prediction = owner.project_model(now);",
        ("owner", "mutable"), "legal_after_proposal")
    add("frequency_owner_model_mutation", '''use time_core::FixturePhasePlanOwner;
fn mutate(owner:&mut FixturePhasePlanOwner<'_,'_>) {
    owner.frequency = owner.frequency;
}
fn main() {}
''', "E0616", "    owner.frequency = owner.frequency;", ("frequency", "private"), "legal_owner_history")
    if len(rows) != 12 or len({name for name, _ in rows}) != len(rows):
        raise ValueError("Frequency contract registry is not its closed source set.")
    return tuple(rows)


def contracts(session, policy, report, execute_registry, phase_examples):
    """Preserve the old1+7 and delegate the complete2+18 once to the existing executor.

    Both callable arguments are the wrapper's pinned code-owned functions, never
    policy-supplied source. Its existing try/finally still owns session.finish().
    """
    legacy = phase_examples()
    if (type(legacy) is not dict or set(legacy) != set(LEGACY_CODES)
            or any(type(row) is not dict or row.get("code") != LEGACY_CODES[name]
                   for name, row in legacy.items())):
        raise ValueError("Frequency contracts require the unchanged affected phase registry.")
    new = dict(examples())
    if set(legacy) & set(new):
        raise ValueError("Frequency and phase contract identities collide.")
    combined = {**legacy, **new}
    actual = {"legal_executions": sum(row["code"] is None for row in combined.values()),
              "negative_compilations": sum(row["code"] is not None for row in combined.values())}
    limits = policy.get("external_consumer_limits")
    if (type(limits) is not dict or set(limits) != set(LIMITS)
            or any(type(value) is not int for value in limits.values())
            or limits != LIMITS or actual != LIMITS):
        raise ValueError("Frequency consumers differ from their fixed actual2+18 limits.")
    markers = {"phase_plan_legal": "legal-phase-plan-contract\n", LEGAL_NAME: LEGAL_MARKER}
    return execute_registry(session, policy, report, combined, markers, LEGAL_SCOPE)


# The historical four-owner registry above remains unchanged. This additive
# profile retains its two processes while extending their checked consumers.
BASELINE_LIMITS = {"legal_executions": 2, "negative_compilations": 23}
BASELINE_CODES = {
    "baseline_feedback_raw_map_input": "E0308",
    "baseline_feedback_map_default": "E0277",
    "baseline_feedback_map_mutation": "E0616",
    "baseline_feedback_completion_literal": "E0451",
    "baseline_feedback_live_proposal_settlement": "E0505",
}
BASELINE_LEGAL_SCOPE = (
    LEGAL_SCOPE + " Baseline map constructor assertions execute healthy checked admission, "
    "exact typed nonpositive-ratio refusal and invalid-domain refusal in the same frequency binary. "
    "Baseline completion constructor assertions execute checked healthy chains, intermediate "
    "frame/generation mismatches and endpoint-cycle refusals with exact typed causes. "
    "The issue/settle borrow helper compiles but is not called; it establishes an external legal "
    "typed neighbor, not an executed baseline-feedback trajectory."
)


def _baseline_legal_source():
    """Extend only the new profile's legal consumer; never rewrite historical truth."""
    source = _legal_source()
    hook = '    println!("legal-frequency-contract");'
    if source.count(hook) != 1:
        raise ValueError("Baseline feedback requires its unique legal execution hook.")
    source = source.replace(hook, "    legal_baseline_map(&chart,base,now)?;\n"
                            "    legal_baseline_completion_construction(&chart,context,base)?;\n" + hook, 1)
    return source + r'''
use time_core::{FixtureBaselineCompletion,FixtureBaselineError,FixtureBaselineFeedback,
    FixtureChartError,FixtureControllerClockMap,FixtureControllerClockMapInput,FixtureFrequencyNumericError,
    FixtureFrequencyNumericField,FixtureFrequencyNumericIssue,FixtureRateSpan,FixtureSettledRateInterval};
fn legal_baseline_map(chart:&FixtureAffineChart,base:FixturePhaseBaseline,now:FixturePhaseNow)
    ->Checked<()> {
    let input=FixtureControllerClockMapInput { reference:now,valid_until:MonotonicNs::new(32_000_000_000),
        active_ratio:0.5,active_ratio_error:0.0,actual_phase:1.0/16.0,actual_phase_error:0.0,
        actual_rate:1.0/64.0,actual_rate_error:0.0 };
    let map=FixtureControllerClockMap::new(chart,base,input)?;
    assert_eq!(map.active_ratio(),0.5);
    assert_eq!(map.actual_phase(),1.0/16.0);
    assert_eq!(map.actual_rate(),1.0/64.0);
    assert_eq!(map.reference().monotonic(),now.monotonic());
    let invalid=FixtureControllerClockMapInput { active_ratio:0.0,..input };
    assert!(matches!(FixtureControllerClockMap::new(chart,base,invalid),
        Err(FixtureBaselineError::Numeric { source:FixtureFrequencyNumericError::Input {
            field:FixtureFrequencyNumericField::ActiveRatio,issue:FixtureFrequencyNumericIssue::NonPositive } })));
    let expired=FixtureControllerClockMapInput { valid_until:now.monotonic(),..input };
    assert!(matches!(FixtureControllerClockMap::new(chart,base,expired),Err(FixtureBaselineError::Map)));
    Ok(())
}
fn legal_baseline_completion(delivery:FixtureSettledRateInterval,achieved:FixturePhaseBaseline,
    final_frames:FixtureChartFrames)->Result<FixtureBaselineCompletion,FixtureBaselineError> {
    FixtureBaselineCompletion::new(delivery,achieved,final_frames)
}
fn legal_baseline_completion_construction(chart:&FixtureAffineChart,context:FixtureRateClockContext,
    base:FixturePhaseBaseline)->Checked<()> {
    let span=FixtureRateSpan::new((ActiveAwakeNs::new(0),ActiveAwakeNs::new(1_000_000_000)),
        (MonotonicNs::new(0),MonotonicNs::new(2_000_000_000)))?;
    let frames=FixtureChartFrames::new(chart.frames().target_frame(),chart.frames().target_generation(),nz(31)?,nz(7)?)?;
    let delivery=FixtureSettledRateInterval::new(context,nz(1)?,span,base.rate(),frames,chart.domain())?;
    let valid=FixtureChartFrames::new(nz(31)?,nz(7)?,nz(37)?,nz(9)?)?;
    let _=legal_baseline_completion(delivery,base,valid)?;
    for wrong in [FixtureChartFrames::new(nz(41)?,nz(7)?,nz(37)?,nz(9)?)?,
                  FixtureChartFrames::new(nz(31)?,nz(11)?,nz(37)?,nz(9)?)?] {
        assert!(matches!(legal_baseline_completion(delivery,base,wrong),Err(FixtureBaselineError::Receipt)));
    }
    for cycle in [FixtureChartFrames::new(nz(31)?,nz(7)?,chart.frames().target_frame(),nz(9)?)?,
                  FixtureChartFrames::new(nz(31)?,nz(7)?,nz(37)?,chart.frames().target_generation())?] {
        assert!(matches!(legal_baseline_completion(delivery,base,cycle),
            Err(FixtureBaselineError::Phase { source:FixturePhaseError::Chart {
                source:FixtureChartError::InvalidFrames } })));
    }
    assert_eq!(span.active_elapsed().nanoseconds(),1_000_000_000);
    assert_eq!(frames.source_frame(),chart.frames().target_frame());
    assert_eq!(frames.source_generation(),chart.frames().target_generation());
    let _=legal_baseline_completion(delivery,base,valid)?;
    Ok(())
}
fn legal_baseline_issue_then_settle(mut owner:FixturePhasePlanOwner<'_,'_>,map:FixtureControllerClockMap,
    now:FixturePhaseNow,event:FixtureBaselineFeedback)->Result<(),FixtureBaselineError> {
    { let proposal=owner.issue_baseline(map,now)?;
      let _=std::hint::black_box((proposal.ordinal(),proposal.requested_rate())); }
    let _=std::hint::black_box(owner.settle_baseline(event,now));
    Ok(())
}
'''


def baseline_feedback_examples():
    """Five fixed negative consumers, added only to the five-owner profile."""
    rows = []

    def add(name, source, code, line, tokens, positive):
        row = _negative(source, code, line, tokens, positive)
        row["external_legal_neighbor"]["scope"] = (
            "compiled typed neighbor; checked map/completion constructor assertions execute in main; "
            "issue/settle helper compiles but is not called")
        rows.append((name, row))

    add("baseline_feedback_raw_map_input", '''use time_core::{FixtureControllerClockMapInput,FixturePhaseNow,FixturePhasePlanOwner};
fn bypass(mut owner:FixturePhasePlanOwner<'_,'_>,input:FixtureControllerClockMapInput,now:FixturePhaseNow) {
    let _ = owner.issue_baseline(input,now);
}
fn main() {}
''', "E0308", "    let _ = owner.issue_baseline(input,now);",
        ("FixtureControllerClockMap", "FixtureControllerClockMapInput"), "legal_baseline_issue_then_settle")
    add("baseline_feedback_map_default", '''use time_core::FixtureControllerClockMap;
fn main() {
    let _: FixtureControllerClockMap = Default::default();
}
''', "E0277", "    let _: FixtureControllerClockMap = Default::default();",
        ("FixtureControllerClockMap", "Default"), "legal_baseline_map")
    add("baseline_feedback_map_mutation", '''use time_core::FixtureControllerClockMap;
fn mutate(map:&mut FixtureControllerClockMap) {
    map.baseline = 0;
}
fn main() {}
''', "E0616", "    map.baseline = 0;", ("baseline", "private"), "legal_baseline_map")
    add("baseline_feedback_completion_literal", '''use time_core::{FixtureBaselineCompletion,FixtureChartFrames,FixturePhaseBaseline,FixtureSettledRateInterval};
fn bypass(delivery:FixtureSettledRateInterval,achieved:FixturePhaseBaseline,final_frames:FixtureChartFrames) {
    let _ = FixtureBaselineCompletion { delivery,achieved,final_frames };
}
fn main() {}
''', "E0451", "    let _ = FixtureBaselineCompletion { delivery,achieved,final_frames };",
        ("FixtureBaselineCompletion", "private"), "legal_baseline_completion")
    add("baseline_feedback_live_proposal_settlement", '''use time_core::{FixtureBaselineError,FixtureBaselineFeedback,FixtureControllerClockMap,
    FixturePhaseNow,FixturePhasePlanOwner};
fn overlap(mut owner:FixturePhasePlanOwner<'_,'_>,map:FixtureControllerClockMap,
    now:FixturePhaseNow,event:FixtureBaselineFeedback)->Result<(),FixtureBaselineError> {
    let proposal=owner.issue_baseline(map,now)?;
    let settled=owner.settle_baseline(event,now);
    let _=std::hint::black_box(proposal.ordinal());
    let _=std::hint::black_box(settled);
    Ok(())
}
fn main() {}
''', "E0505", "    let settled=owner.settle_baseline(event,now);",
        ("owner", "borrowed"), "legal_baseline_issue_then_settle")
    return tuple(rows)


def baseline_feedback_contracts(session, policy, report, execute_registry, phase_examples):
    """Delegate exactly the historical controls plus five new causal consumers."""
    legacy = phase_examples()
    if (type(legacy) is not dict or set(legacy) != set(LEGACY_CODES)
            or any(type(row) is not dict or row.get("code") != LEGACY_CODES[name]
                   for name, row in legacy.items())):
        raise ValueError("Baseline contracts require the unchanged affected phase registry.")
    frequency = dict(examples())
    rows = baseline_feedback_examples()
    added = dict(rows)
    if (len(rows) != len(BASELINE_CODES) or set(added) != set(BASELINE_CODES)
            or any(type(row) is not dict or row.get("code") != BASELINE_CODES[name]
                   for name, row in added.items())
            or set(legacy) & set(frequency) or (set(legacy) | set(frequency)) & set(added)):
        raise ValueError("Baseline feedback consumers differ from their closed source set.")
    combined = {**legacy, **frequency, **added}
    actual = {"legal_executions": sum(row["code"] is None for row in combined.values()),
              "negative_compilations": sum(row["code"] is not None for row in combined.values())}
    limits = policy.get("external_consumer_limits")
    if (type(limits) is not dict or set(limits) != set(BASELINE_LIMITS)
            or any(type(value) is not int for value in limits.values())
            or limits != BASELINE_LIMITS or actual != BASELINE_LIMITS):
        raise ValueError("Baseline consumers differ from their fixed actual2+23 limits.")
    combined[LEGAL_NAME] = {"source": _baseline_legal_source(), "code": None}
    markers = {"phase_plan_legal": "legal-phase-plan-contract\n", LEGAL_NAME: LEGAL_MARKER}
    return execute_registry(session, policy, report, combined, markers, BASELINE_LEGAL_SCOPE)


# Additive current-source registration only. The historical factories above retain
# their own fixtures, counts and meanings; this profile transfers no old score.
PHASE_STEP_INPUTS = (
    "scripts/core-frequency-contracts.py",
    "scripts/tests/test_phase_step_contracts.py",
)
PHASE_STEP_CRITICAL_FILES = (
    "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs",
    "crates/time-core/src/regression/chart/roundoff/phase_plan.rs",
    "crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs",
    "crates/time-core/src/reference_input/rate_delivery.rs",
    "crates/time-core/src/reference_input.rs",
    "crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs",
    "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs",
)
PHASE_STEP_LEGAL_NAME = "phase_step_legal"
PHASE_STEP_LEGAL_MARKER = "legal-phase-step-contract\n"
PHASE_STEP_NEW_LIMITS = {"legal_executions": 1, "negative_compilations": 9}
PHASE_STEP_LIMITS = {"legal_executions": 3, "negative_compilations": 32}
PHASE_STEP_CODES = {
    "phase_step_adaptive_literal": "E0451",
    "phase_step_adaptive_mutation": "E0616",
    "phase_step_limits_literal": "E0451",
    "phase_step_limits_mutation": "E0616",
    "phase_step_completion_literal": "E0451",
    "phase_step_completion_mutation": "E0616",
    "phase_step_live_proposal_settlement": "E0505",
    "phase_step_double_transfer": "E0499",
    "phase_step_retirement_escape": "E0515",
}
PHASE_STEP_VALIDATION_SCOPE = (
    "Privacy and invalid-input controls do not establish mandatory Garde-call execution. "
    "Seven separately registered validation-call boundaries remain: RawAdaptiveProfile and RawAdaptiveArea "
    "in adaptive.rs; RawLimits, RawStepRequest, RawStepCompletion and RawStepTerminal in step.rs; "
    "and numeric RawStep in roundoff/phase_plan.rs. Each invokes validate and repeats its shared "
    "semantic rule for a typed cause. Removing only validate can therefore remain behaviorally "
    "masked. A Garde-call omission control remains a generation/control gap. A future bounded "
    "source-copy control can replace only the raw Garde closure result with a sentinel refusal, "
    "retain its direct semantic rule and require the healthy constructor's ValidationInvariant; "
    "then omission of validate must remove that observed refusal. Such altered-source controls "
    "need separate candidate custody, explicit mechanism admission and actual execution; none are claimed here."
)
PHASE_STEP_LEGAL_SCOPE = (
    BASELINE_LEGAL_SCOPE + " The additional Step binary executes checked adaptive-profile, "
    "per-operation-limit and actual-completion constructors, invalid neighboring controls and "
    "the actual owner's NoReference issue refusal with an unchanged cursor. Step issue/settle "
    "and retirement-transfer helpers compile but are not called. Nonempty Source-to-Step "
    "trajectories remain package-test evidence; no native clock operation is performed. "
    + PHASE_STEP_VALIDATION_SCOPE
)


def _phase_step_legal_source():
    """Exercise public checked admission; compile lifetime neighbors without fake effects."""
    return r'''use std::num::NonZeroU64;
use time_core::{FixtureAdaptiveHorizonProfile,FixtureAffineChart,FixtureChartDomain,
    FixtureChartFrames,FixtureCombinationParameters,FixtureCombinationState,FixtureMinimum,
    FixturePhaseBaseline,FixturePhaseError,FixturePhaseNow,FixturePhasePlanOwner,FixturePhaseProfile,
    FixturePhaseResidualPermission,FixturePrincipalParameters,FixtureProjectedBatch,
    FixtureRateClockContext,FixtureRateSettlementCursor,FixtureRateSpan,FixtureSettledRateInterval,
    FixtureStepCompletion,FixtureStepDirections,FixtureStepError,FixtureStepFeedback,
    FixtureStepLimits,FixtureStepRetirement};
use time_domain::{ActiveAwakeDurationNs,ActiveAwakeNs,MonotonicNs};
type Checked<T> = Result<T,Box<dyn std::error::Error>>;
fn nz(value:u64)->Checked<NonZeroU64> { Ok(NonZeroU64::new(value).ok_or("zero fixture label")?) }
fn legal_adaptive_profile(minimum:ActiveAwakeDurationNs,maximum:ActiveAwakeDurationNs)
    ->Result<FixtureAdaptiveHorizonProfile,FixturePhaseError> {
    FixtureAdaptiveHorizonProfile::new(4.0,minimum,maximum,0.0)
}
fn legal_step_limits()->Result<FixtureStepLimits,FixtureStepError> {
    FixtureStepLimits::new(1.0/8.0,FixtureStepDirections::Both)
}
fn legal_step_completion(delivery:FixtureSettledRateInterval,final_frames:FixtureChartFrames)
    ->Result<FixtureStepCompletion,FixtureStepError> {
    FixtureStepCompletion::new(delivery,1.0/32.0,final_frames)
}
fn legal_step_issue_then_settle(mut owner:FixturePhasePlanOwner<'_,'_>,now:FixturePhaseNow,
    limits:FixtureStepLimits,event:FixtureStepFeedback)->Result<(),FixtureStepError> {
    { let proposal=owner.issue_step(now,FixturePhaseResidualPermission::Allow,limits)?;
      let _=std::hint::black_box((proposal.ordinal(),proposal.requested_phase_seconds(),
          proposal.requested_phase_error_seconds(),proposal.known_baseline(),
          proposal.active_deadline(),proposal.monotonic_deadline(),proposal.provenance())); }
    let _=std::hint::black_box(owner.settle_step(event,now));
    Ok(())
}
fn legal_step_resume<'producer,'chart>(retired:&'chart mut FixtureStepRetirement<'producer>,
    now:FixturePhaseNow)->Result<FixturePhasePlanOwner<'producer,'chart>,FixtureStepError> {
    retired.resume_fresh(now)
}
fn legal_step_release_before_inspection(mut retired:FixtureStepRetirement<'_>,now:FixturePhaseNow)
    ->Result<(),FixtureStepError> {
    { let owner=retired.resume_fresh(now)?;
      let _=std::hint::black_box(owner.cursor()); }
    let _=std::hint::black_box((retired.ordinal(),retired.completion(),retired.provenance()));
    Ok(())
}
fn main()->Checked<()> {
    let minimum=ActiveAwakeNs::new(1_000_000_000).duration_since(ActiveAwakeNs::new(0))?;
    let maximum=ActiveAwakeNs::new(16_000_000_000).duration_since(ActiveAwakeNs::new(0))?;
    let adaptive=legal_adaptive_profile(minimum,maximum)?;
    for ratio in [0.0,-1.0,f64::NAN,f64::INFINITY,f64::from_bits(1)] {
        assert!(matches!(FixtureAdaptiveHorizonProfile::new(ratio,minimum,maximum,0.0),
            Err(FixturePhaseError::InvalidProfile)));
    }
    assert!(matches!(FixtureAdaptiveHorizonProfile::new(4.0,maximum,minimum,0.0),
        Err(FixturePhaseError::InvalidProfile)));
    assert!(matches!(FixtureAdaptiveHorizonProfile::new(4.0,minimum,maximum,-1.0),
        Err(FixturePhaseError::InvalidProfile)));
    let limits=legal_step_limits()?;
    assert_eq!(limits.maximum_abs_seconds(),1.0/8.0);
    assert_eq!(limits.directions(),FixtureStepDirections::Both);
    for cap in [0.0,-0.0,-1.0,f64::NAN,f64::INFINITY,f64::from_bits(1)] {
        assert!(matches!(FixtureStepLimits::new(cap,FixtureStepDirections::Both),
            Err(FixtureStepError::InvalidLimits)));
    }
    let frames=FixtureChartFrames::new(nz(17)?,nz(23)?,nz(19)?,nz(29)?)?;
    let domain=FixtureChartDomain::new(MonotonicNs::new(0),MonotonicNs::new(0),
        MonotonicNs::new(64_000_000_000))?;
    let chart=FixtureAffineChart::new(frames,domain,0.0,0.0)?;
    let context=FixtureRateClockContext::new(nz(101)?,nz(103)?,nz(107)?);
    let now=FixturePhaseNow::new(context,ActiveAwakeNs::new(0),MonotonicNs::new(0));
    let span=FixtureRateSpan::new((ActiveAwakeNs::new(0),ActiveAwakeNs::new(2_000_000_000)),
        (MonotonicNs::new(0),MonotonicNs::new(4_000_000_000)))?;
    let intermediate=FixtureChartFrames::new(nz(19)?,nz(29)?,nz(31)?,nz(37)?)?;
    let delivery=FixtureSettledRateInterval::new(context,nz(1)?,span,1.0/16.0,intermediate,domain)?;
    let final_frames=FixtureChartFrames::new(nz(31)?,nz(37)?,nz(41)?,nz(43)?)?;
    let completion=legal_step_completion(delivery,final_frames)?;
    assert_eq!(completion.achieved_step_seconds(),1.0/32.0);
    let _=std::hint::black_box(completion.delivery());
    assert_eq!(completion.final_frames().target_frame(),nz(41)?);
    for actual in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY,f64::from_bits(1)] {
        assert!(matches!(FixtureStepCompletion::new(delivery,actual,final_frames),
            Err(FixtureStepError::Phase{source:FixturePhaseError::Chart{..}})));
    }
    for actual in [0.0,-0.0] {
        let known=FixtureStepCompletion::new(delivery,actual,final_frames)?;
        assert_eq!(known.achieved_step_seconds().to_bits(),actual.to_bits());
    }
    let foreign=FixtureChartFrames::new(nz(47)?,nz(37)?,nz(41)?,nz(43)?)?;
    assert!(matches!(FixtureStepCompletion::new(delivery,1.0/32.0,foreign),Err(FixtureStepError::Receipt)));
    let empty=FixtureProjectedBatch::new(nz(17)?,nz(23)?,MonotonicNs::new(0),&[])?;
    let mut producer=FixtureCombinationState::seed(&[nz(11)?],FixturePrincipalParameters::new(1.0/1024.0,0.0)?,
        FixtureCombinationParameters::new(2.0,0.0)?,FixtureMinimum::new(1)?,&empty,&[])?;
    let phase=FixturePhaseProfile::new(maximum,
        MonotonicNs::new(32_000_000_000).duration_since(MonotonicNs::new(0))?,
        1.0/4.0,1.0/8.0,1.0/2.0)?.with_adaptive_horizon(adaptive)?;
    let cursor=FixtureRateSettlementCursor::new(&chart,context,nz(1)?);
    let base=FixturePhaseBaseline::new(1.0/16.0)?;
    let mut owner=FixturePhasePlanOwner::new(&mut producer,&chart,cursor,now,phase,base)?;
    assert!(matches!(owner.issue_step(now,FixturePhaseResidualPermission::Allow,limits),
        Err(FixtureStepError::Phase{source:FixturePhaseError::NoReference})));
    assert!(owner.step_proposal().is_none());
    assert_eq!(owner.cursor().next_ordinal(),Some(nz(1)?));
    drop(owner);
    assert_eq!(producer.emitted_rounds(),0);
    println!("legal-phase-step-contract");
    Ok(())
}
'''


def phase_step_examples():
    """Return the closed new1+9 registry with attributable primary operations."""
    rows = [(PHASE_STEP_LEGAL_NAME, {"source": _phase_step_legal_source(), "code": None})]

    def add(name, source, code, line, tokens, positive):
        row = _negative(source, code, line, tokens, positive)
        row["external_legal_neighbor"] = {
            "specimen": PHASE_STEP_LEGAL_NAME, "function": positive,
            "scope": "compiled typed neighbor; checked constructors and NoReference refusal execute in main",
        }
        rows.append((name, row))

    add("phase_step_adaptive_literal", '''use time_core::FixtureAdaptiveHorizonProfile;
use time_domain::ActiveAwakeDurationNs;
fn bypass(minimum:ActiveAwakeDurationNs,maximum:ActiveAwakeDurationNs) {
    let _ = FixtureAdaptiveHorizonProfile { correction_time_ratio:4.0,minimum,maximum,small_debt_threshold:0.0 };
}
fn main() {}
''', "E0451", "    let _ = FixtureAdaptiveHorizonProfile { correction_time_ratio:4.0,minimum,maximum,small_debt_threshold:0.0 };",
        ("FixtureAdaptiveHorizonProfile", "private"), "legal_adaptive_profile")
    add("phase_step_adaptive_mutation", '''use time_core::FixtureAdaptiveHorizonProfile;
fn bypass(profile:&mut FixtureAdaptiveHorizonProfile) {
    profile.correction_time_ratio = 0.0;
}
fn main() {}
''', "E0616", "    profile.correction_time_ratio = 0.0;",
        ("correction_time_ratio", "private"), "legal_adaptive_profile")
    add("phase_step_limits_literal", '''use time_core::{FixtureStepDirections,FixtureStepLimits};
fn main() {
    let _ = FixtureStepLimits { maximum:1.0/8.0,directions:FixtureStepDirections::Both };
}
''', "E0451", "    let _ = FixtureStepLimits { maximum:1.0/8.0,directions:FixtureStepDirections::Both };",
        ("FixtureStepLimits", "private"), "legal_step_limits")
    add("phase_step_limits_mutation", '''use time_core::FixtureStepLimits;
fn bypass(limits:&mut FixtureStepLimits) {
    limits.maximum = f64::INFINITY;
}
fn main() {}
''', "E0616", "    limits.maximum = f64::INFINITY;", ("maximum", "private"), "legal_step_limits")
    add("phase_step_completion_literal", '''use time_core::{FixtureChartFrames,FixtureSettledRateInterval,FixtureStepCompletion};
fn bypass(delivery:FixtureSettledRateInterval,final_frames:FixtureChartFrames) {
    let _ = FixtureStepCompletion { delivery,achieved:0.0,final_frames };
}
fn main() {}
''', "E0451", "    let _ = FixtureStepCompletion { delivery,achieved:0.0,final_frames };",
        ("FixtureStepCompletion", "private"), "legal_step_completion")
    add("phase_step_completion_mutation", '''use time_core::FixtureStepCompletion;
fn bypass(completion:&mut FixtureStepCompletion) {
    completion.achieved = f64::NAN;
}
fn main() {}
''', "E0616", "    completion.achieved = f64::NAN;", ("achieved", "private"), "legal_step_completion")
    add("phase_step_live_proposal_settlement", '''use time_core::{FixturePhaseNow,FixturePhasePlanOwner,FixturePhaseResidualPermission,
    FixtureStepError,FixtureStepFeedback,FixtureStepLimits};
fn overlap(mut owner:FixturePhasePlanOwner<'_,'_>,now:FixturePhaseNow,
    limits:FixtureStepLimits,event:FixtureStepFeedback)->Result<(),FixtureStepError> {
    let proposal=owner.issue_step(now,FixturePhaseResidualPermission::Allow,limits)?;
    let settled=owner.settle_step(event,now);
    let _=std::hint::black_box(proposal.ordinal());
    let _=std::hint::black_box(settled);
    Ok(())
}
fn main() {}
''', "E0505", "    let settled=owner.settle_step(event,now);",
        ("owner", "borrowed"), "legal_step_issue_then_settle")
    add("phase_step_double_transfer", '''use time_core::{FixturePhaseNow,FixtureStepError,FixtureStepRetirement};
fn overlap(mut retired:FixtureStepRetirement<'_>,now:FixturePhaseNow)->Result<(),FixtureStepError> {
    let first=retired.resume_fresh(now)?;
    let second=retired.resume_fresh(now)?;
    let _=std::hint::black_box(first.cursor());
    let _=std::hint::black_box(second);
    Ok(())
}
fn main() {}
''', "E0499", "    let second=retired.resume_fresh(now)?;",
        ("retired", "mutable"), "legal_step_release_before_inspection")
    add("phase_step_retirement_escape", '''use time_core::{FixturePhaseNow,FixturePhasePlanOwner,FixtureStepError,FixtureStepRetirement};
fn escape<'producer:'chart,'chart>(mut retired:FixtureStepRetirement<'producer>,now:FixturePhaseNow)
    ->Result<FixturePhasePlanOwner<'producer,'chart>,FixtureStepError> {
    retired.resume_fresh(now)
}
fn main() {}
''', "E0515", "    retired.resume_fresh(now)", ("retired", "owned by the current function"), "legal_step_resume")
    added = dict(rows)
    if (len(rows) != 10 or len(added) != 10
            or set(added) != {PHASE_STEP_LEGAL_NAME, *PHASE_STEP_CODES}
            or any(added[name]["code"] != code for name, code in PHASE_STEP_CODES.items())):
        raise ValueError("Phase-step consumers differ from their fixed new1+9 registry.")
    return tuple(rows)


def phase_step_contracts(session, policy, report, execute_registry, phase_examples):
    """Register current seven-file contracts only after explicit outer-role admission.

    The existing executor owns captured-source hashes, protected inputs, compiler
    admission and exact diagnostic attribution. These checks additionally prevent
    reusing this new registry under a historical or incompletely bound role.
    """
    inputs = policy.get("inputs_sha256")
    if (policy.get("critical_files") != list(PHASE_STEP_CRITICAL_FILES)
            or policy.get("tool_admission_profile") != "stable-core"
            or policy.get("contracts_admitted") is not True
            or type(inputs) is not dict
            or not set((*PHASE_STEP_INPUTS, *PHASE_STEP_CRITICAL_FILES)).issubset(inputs)):
        raise ValueError("Phase-step contracts require their admitted seven-file role and protected inputs.")
    # Reuse the fixed baseline registry through its existing validation path,
    # capturing its rows rather than executing any historical consumer twice.
    baseline = None

    def collect(_session, _policy, _report, rows, markers, scope):
        nonlocal baseline
        baseline = rows, markers, scope

    baseline_policy = {**policy, "external_consumer_limits": dict(BASELINE_LIMITS)}
    baseline_feedback_contracts(session, baseline_policy, report, collect, phase_examples)
    if baseline is None:
        raise ValueError("Phase-step contracts lack their validated predecessor registry.")
    legacy, markers, _scope = baseline
    rows = phase_step_examples()
    added = dict(rows)
    if (len(rows) != 10 or len(added) != 10
            or set(added) != {PHASE_STEP_LEGAL_NAME, *PHASE_STEP_CODES}
            or any(type(added[name]) is not dict or added[name].get("code") != code
                   for name, code in PHASE_STEP_CODES.items())
            or added[PHASE_STEP_LEGAL_NAME].get("code") is not None
            or set(legacy) & set(added)):
        raise ValueError("Phase-step contracts differ from their closed current registry.")
    combined = {**legacy, **added}
    actual = {"legal_executions": sum(row["code"] is None for row in combined.values()),
              "negative_compilations": sum(row["code"] is not None for row in combined.values())}
    limits = policy.get("external_consumer_limits")
    if (type(limits) is not dict or set(limits) != set(PHASE_STEP_LIMITS)
            or any(type(value) is not int for value in limits.values())
            or limits != PHASE_STEP_LIMITS or actual != PHASE_STEP_LIMITS):
        raise ValueError("Phase-step consumers differ from their fixed actual3+32 limits.")
    markers = {**markers, PHASE_STEP_LEGAL_NAME: PHASE_STEP_LEGAL_MARKER}
    return execute_registry(session, policy, report, combined, markers, PHASE_STEP_LEGAL_SCOPE)


# Separate actual-source validation-call controls.  The public compiler registry
# above remains exactly three legal executions and 32 negative compilations.
PHASE_STEP_VALIDATION_LIMITS = {
    "mechanisms": 7, "specimens_per_mechanism": 3, "test_executions": 21,
}
PHASE_STEP_VALIDATION_MODES = ("original", "sentinel", "omission")
PHASE_STEP_VALIDATION_NAMES = (
    "adaptive_profile", "adaptive_area", "step_limits", "step_request",
    "step_completion", "step_terminal", "numeric_step",
)
PHASE_STEP_VALIDATION_FIXTURE = "crates/time-core/tests/private/phase_plan_lifecycle.rs"
PHASE_STEP_VALIDATION_TEST = (
    "reference_input::rate_delivery::phase_plan::tests::"
    "__phase_step_validation_mechanism::observed"
)

_VALIDATION_OWNER = r'''
    fn with_owner(
        action: impl for<'producer, 'chart> FnOnce(FixturePhasePlanOwner<'producer, 'chart>) -> TestResult,
    ) -> TestResult {
        let chart = chart()?;
        let mut fixture = producer(&chart)?;
        let at = controller_clock_point(0, 4)?;
        let batch = FixtureChartBatch::new(&chart, at.monotonic(), &fixture.entries)?;
        let mut owner = FixturePhasePlanOwner::new(
            &mut fixture.state, &chart,
            FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?),
            at, profile()?, FixturePhaseBaseline::new(1.0 / 1024.0)?,
        )?;
        assert!(matches!(owner.receive_reference(&batch, &fixture.strata, at)?,
            FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?));
        assert_eq!(owner.remaining_phase_seconds(), Some(1.0 / 32.0));
        action(owner)
    }
'''
_VALIDATION_PARTS = r'''
    fn completion_parts(owner: &FixturePhasePlanOwner<'_, '_>)
        -> TestResult<(FixtureSettledRateInterval, FixtureChartFrames)> {
        let span = FixtureRateSpan::new(
            (controller_clock_point(0, 4)?.active(), controller_clock_point(2, 8)?.active()),
            (controller_clock_point(0, 4)?.monotonic(), controller_clock_point(2, 8)?.monotonic()),
        )?;
        let before = owner.chart().frames();
        let delivery = FixtureSettledRateInterval::new(
            clocks()?, label(1)?, span, 1.0 / 1024.0,
            FixtureChartFrames::new(before.target_frame(), before.target_generation(), label(31)?, label(7)?)?,
            owner.chart().domain(),
        )?;
        let final_frames = FixtureChartFrames::new(label(31)?, label(7)?, label(37)?, label(9)?)?;
        Ok((delivery, final_frames))
    }
'''


def phase_step_validation_cases():
    """Return seven closed actual-consumer specimens; returning them executes no Rust."""
    adaptive = "crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs"
    step = "crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs"
    numeric = "crates/time-core/src/regression/chart/roundoff/phase_plan.rs"
    rows = []

    def add(name, raw, path, sentinel_from, sentinel_to, scope_start, scope_end, body,
            *, owner=False, parts=False, validate_indent=8):
        rows.append({"name": name, "raw_type": raw, "owner_path": path,
                     "append_path": PHASE_STEP_VALIDATION_FIXTURE,
                     "sentinel_from": sentinel_from, "sentinel_to": sentinel_to,
                     "scope_start": scope_start, "scope_end": scope_end,
                     "validate_from": " " * validate_indent + "let validated = raw.validate();",
                     "validate_to": " " * validate_indent + "let validated: Result<(), garde::Report> = Ok(());",
                     "test_identity": PHASE_STEP_VALIDATION_TEST,
                     "helpers": (_VALIDATION_OWNER if owner else "") + (_VALIDATION_PARTS if parts else ""),
                     "body": body})

    anchor = "if adaptive_profile_valid(value) { Ok(()) }"
    add("adaptive_profile", "RawAdaptiveProfile", adaptive, anchor,
        "if adaptive_profile_valid(value) && value.correction_time_ratio != 4.0 { Ok(()) }",
        "impl FixtureAdaptiveHorizonProfile {", "impl fmt::Debug for FixtureAdaptiveHorizonProfile",
        r'''
        let minimum = ActiveAwakeNs::new(SECOND).duration_since(ActiveAwakeNs::new(0))?;
        let maximum = ActiveAwakeNs::new(16 * SECOND).duration_since(ActiveAwakeNs::new(0))?;
        let result = crate::FixtureAdaptiveHorizonProfile::new(4.0, minimum, maximum, 0.0);
        if REFUSAL {
            assert!(matches!(result, Err(FixturePhaseError::ValidationInvariant)));
        } else { assert!(result.is_ok()); }
        assert!(crate::FixtureAdaptiveHorizonProfile::new(2.0, minimum, maximum, 0.0).is_ok());
        assert!(matches!(crate::FixtureAdaptiveHorizonProfile::new(0.0, minimum, maximum, 0.0),
            Err(FixturePhaseError::InvalidProfile)));
''')
    anchor = "if adaptive_area_valid(*point) { Ok(()) }"
    add("adaptive_area", "RawAdaptiveArea", adaptive, anchor,
        "if adaptive_area_valid(*point) && *point != 0.0 { Ok(()) }",
        "    fn new(point: f64)", "    /// Form the accepted reference",
        r'''
        let minimum = ActiveAwakeNs::new(SECOND).duration_since(ActiveAwakeNs::new(0))?;
        let maximum = ActiveAwakeNs::new(16 * SECOND).duration_since(ActiveAwakeNs::new(0))?;
        let adaptive = crate::FixtureAdaptiveHorizonProfile::new(4.0, minimum, maximum, 0.0)?;
        assert!(super::super::AdaptiveArea::from_reference(adaptive, 1.0, minimum).is_ok());
        assert!(matches!(super::super::AdaptiveArea::from_reference(adaptive, -1.0, minimum),
            Err(FixturePhaseError::AdaptiveHorizon { source: crate::FixtureAdaptiveHorizonError::UnsupportedStatistic })));
        let chart = chart()?;
        let mut fixture = producer(&chart)?;
        let at = controller_clock_point(0, 4)?;
        let batch = FixtureChartBatch::new(&chart, at.monotonic(), &fixture.entries)?;
        let mut owner = FixturePhasePlanOwner::new(
            &mut fixture.state, &chart,
            FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?),
            at, profile()?.with_adaptive_horizon(adaptive)?, FixturePhaseBaseline::new(1.0 / 1024.0)?,
        )?;
        let result = owner.receive_reference(&batch, &fixture.strata, at)?;
        if REFUSAL {
            assert!(matches!(result, FixturePhaseReferenceAdmission::AdmittedNoPlan {
                emitted_round: Some(round), reason: FixturePhaseError::AdaptiveHorizon {
                    source: crate::FixtureAdaptiveHorizonError::ValidationInvariant
                } } if round == label(1)?));
            assert_eq!(owner.last_accepted_reference_active, None);
            assert!(owner.obligation.is_none());
        } else {
            assert!(matches!(result, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?));
            assert_eq!(owner.last_accepted_reference_active, Some(at.active()));
        }
        assert_eq!(owner.producer.emitted_rounds(), 1);
        assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
''')
    anchor = 'limits_rule(value).map_err(|_| garde::Error::new("invalid explicit step limits"))'
    add("step_limits", "RawLimits", step, anchor,
        'if value.maximum == 1.0 / 16.0 { Err(garde::Error::new("validation mechanism sentinel")) } else { ' + anchor + ' }',
        "impl FixtureStepLimits {", "    /// Copy the explicit cap",
        r'''
        let result = crate::FixtureStepLimits::new(1.0 / 16.0, crate::FixtureStepDirections::Both);
        if REFUSAL {
            assert!(matches!(result, Err(crate::FixtureStepError::Phase { source: FixturePhaseError::ValidationInvariant })));
        } else { assert!(result.is_ok()); }
        assert!(crate::FixtureStepLimits::new(1.0 / 8.0, crate::FixtureStepDirections::Both).is_ok());
        assert!(matches!(crate::FixtureStepLimits::new(0.0, crate::FixtureStepDirections::Both), Err(crate::FixtureStepError::InvalidLimits)));
''')
    anchor = 'request_rule(value).map_err(|_| garde::Error::new("invalid finite step request"))'
    add("step_request", "RawStepRequest", step, anchor,
        'if value.limits.maximum == 1.0 / 16.0 { Err(garde::Error::new("validation mechanism sentinel")) } else { ' + anchor + ' }',
        "    pub fn issue_step(", "\n    }\n",
        r'''
        for cap in [1.0 / 64.0, 1.0 / 16.0, 1.0 / 8.0] {
            with_owner(|mut owner| {
                let limits = crate::FixtureStepLimits::new(cap, crate::FixtureStepDirections::Both)?;
                let result = owner.issue_step(controller_clock_point(0, 4)?, FixturePhaseResidualPermission::Allow, limits);
                if cap == 1.0 / 64.0 {
                    assert!(matches!(result, Err(crate::FixtureStepError::Magnitude)));
                } else if REFUSAL && cap == 1.0 / 16.0 {
                    assert!(matches!(result, Err(crate::FixtureStepError::Phase { source: FixturePhaseError::ValidationInvariant })));
                } else { assert_eq!(result?.ordinal(), label(1)?); }
                assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
                assert_eq!(owner.producer.emitted_rounds(), 1);
                assert_eq!(owner.step_proposal().is_some(), !(cap == 1.0 / 64.0 || (REFUSAL && cap == 1.0 / 16.0)));
                Ok(())
            })?;
        }
''', owner=True)
    anchor = 'completion_rule(value).map_err(|_| garde::Error::new("invalid known step descriptor"))'
    add("step_completion", "RawStepCompletion", step, anchor,
        'if value.achieved == 1.0 / 64.0 { Err(garde::Error::new("validation mechanism sentinel")) } else { ' + anchor + ' }',
        "impl FixtureStepCompletion {", "    /// Copy the exact original aggregate interval",
        r'''
        with_owner(|mut owner| {
            let limits = crate::FixtureStepLimits::new(1.0 / 8.0, crate::FixtureStepDirections::Both)?;
            owner.issue_step(controller_clock_point(0, 4)?, FixturePhaseResidualPermission::Allow, limits)?;
            let (delivery, final_frames) = completion_parts(&owner)?;
            let result = crate::FixtureStepCompletion::new(delivery, 1.0 / 64.0, final_frames);
            if REFUSAL {
                assert!(matches!(result, Err(crate::FixtureStepError::Phase { source: FixturePhaseError::ValidationInvariant })));
            } else { assert_eq!(result?.achieved_step_seconds(), 1.0 / 64.0); }
            let healthy = crate::FixtureStepCompletion::new(delivery, 1.0 / 128.0, final_frames)?;
            let broken = FixtureChartFrames::new(label(47)?, label(7)?, label(37)?, label(9)?)?;
            assert!(matches!(crate::FixtureStepCompletion::new(delivery, 1.0 / 128.0, broken), Err(crate::FixtureStepError::Receipt)));
            assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
            assert_eq!(healthy.achieved_step_seconds(), 1.0 / 128.0);
            assert!(owner.step_proposal().is_some());
            Ok(())
        })?;
''', owner=True, parts=True)
    anchor = 'terminal_rule(value).map_err(|_| garde::Error::new("invalid matched step terminal"))'
    add("step_terminal", "RawStepTerminal", step, anchor,
        'if matches!(value.event, FixtureStepFeedback::Known(_)) { Err(garde::Error::new("validation mechanism sentinel")) } else { ' + anchor + ' }',
        "fn check_terminal(value: StepTerminal)", "\n}\n",
        r'''
        with_owner(|mut owner| {
            let limits = crate::FixtureStepLimits::new(1.0 / 8.0, crate::FixtureStepDirections::Both)?;
            owner.issue_step(controller_clock_point(0, 4)?, FixturePhaseResidualPermission::Allow, limits)?;
            let wrong = crate::FixtureStepFeedback::ProvenNoEffect { context: clocks()?, ordinal: label(2)? };
            let crate::FixtureStepSettlement::OuterRejected { owner: returned, error } = owner.settle_step(wrong, controller_clock_point(0, 5)?) else { return Err(io::Error::other("wrong ordinal consumed").into()); };
            assert_eq!(error, crate::FixtureStepError::Receipt);
            let owner = returned;
            let (delivery, final_frames) = completion_parts(&owner)?;
            let receipt = crate::FixtureStepCompletion::new(delivery, 1.0 / 64.0, final_frames)?;
            let result = owner.settle_step(crate::FixtureStepFeedback::Known(receipt), controller_clock_point(2, 8)?);
            if REFUSAL {
                let crate::FixtureStepSettlement::OuterRejected { owner, error } = result else { return Err(io::Error::other("validation did not retain pending owner").into()); };
                assert_eq!(error, crate::FixtureStepError::Phase { source: FixturePhaseError::ValidationInvariant });
                assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
                assert!(owner.step_proposal().is_some());
                assert_eq!(owner.producer.emitted_rounds(), 1);
            } else {
                let crate::FixtureStepSettlement::Known(retired) = result else { return Err(io::Error::other("matching Known did not retire").into()); };
                assert_eq!(retired.remaining_phase_seconds(), Some(1.0 / 64.0));
                assert_eq!(retired.replacement()?.phase_at_reference_seconds(), 9.0 / 512.0);
                assert_eq!(retired.cursor().ok_or_else(|| io::Error::other("lost cursor"))?.next_ordinal(), Some(label(2)?));
            }
            Ok(())
        })?;
        with_owner(|mut owner| {
            let limits = crate::FixtureStepLimits::new(1.0 / 8.0, crate::FixtureStepDirections::Both)?;
            owner.issue_step(controller_clock_point(0, 4)?, FixturePhaseResidualPermission::Allow, limits)?;
            let result = owner.settle_step(crate::FixtureStepFeedback::ProvenNoEffect { context: clocks()?, ordinal: label(1)? }, controller_clock_point(0, 5)?);
            let crate::FixtureStepSettlement::ProvenNoEffect(owner) = result else { return Err(io::Error::other("healthy NoEffect refused").into()); };
            assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
            assert_eq!(owner.cursor().last_active_end(), None);
            Ok(())
        })?;
''', owner=True, parts=True, validate_indent=4)
    anchor = 'supported(*value, FixturePhaseNumericField::AchievedStep)\n            .map_err(|_| garde::Error::new("invalid achieved phase step"))'
    add("numeric_step", "RawStep", numeric, anchor,
        'if *value == 1.0 / 64.0 { Err(garde::Error::new("validation mechanism sentinel")) } else { ' + anchor + ' }',
        "    pub(crate) fn stepped(self, achieved: f64)", "\n    }\n",
        r'''
        let debt = super::super::PhaseDebt::new(1.0 / 32.0, 0.0)?;
        assert!(matches!(debt.stepped(f64::NAN), Err(crate::FixturePhaseNumericError::Input {
            field: crate::FixturePhaseNumericField::AchievedStep, issue: crate::FixturePhaseNumericIssue::NonFinite })));
        for achieved in [1.0 / 64.0, 1.0 / 128.0] {
            with_owner(|mut owner| {
                let limits = crate::FixtureStepLimits::new(1.0 / 8.0, crate::FixtureStepDirections::Both)?;
                owner.issue_step(controller_clock_point(0, 4)?, FixturePhaseResidualPermission::Allow, limits)?;
                let (delivery, final_frames) = completion_parts(&owner)?;
                let receipt = crate::FixtureStepCompletion::new(delivery, achieved, final_frames)?;
                let result = owner.settle_step(crate::FixtureStepFeedback::Known(receipt), controller_clock_point(2, 8)?);
                let crate::FixtureStepSettlement::Known(retired) = result else { return Err(io::Error::other("numeric refusal erased Known").into()); };
                assert_eq!(retired.achieved_step_seconds(), achieved);
                assert_eq!(retired.requested_phase_seconds(), 1.0 / 32.0);
                assert_eq!(retired.pre_step_phase_seconds(), Some(1.0 / 32.0));
                assert_eq!(retired.cursor().ok_or_else(|| io::Error::other("lost consumed cursor"))?.next_ordinal(), Some(label(2)?));
                assert_eq!(retired.provenance().ok_or_else(|| io::Error::other("lost source"))?.emitted_round(), label(1)?);
                if REFUSAL && achieved == 1.0 / 64.0 {
                    assert!(matches!(retired.debt_error(), Some(FixturePhaseError::Numeric { source: crate::FixturePhaseNumericError::ValidationInvariant })));
                    assert_eq!(retired.remaining_phase_seconds(), None);
                    assert_eq!(retired.replacement()?.phase_at_reference_seconds(), 9.0 / 512.0);
                } else {
                    let (remaining, displacement) = if achieved == 1.0 / 64.0 { (1.0 / 64.0, 9.0 / 512.0) } else { (3.0 / 128.0, 5.0 / 512.0) };
                    assert_eq!(retired.remaining_phase_seconds(), Some(remaining));
                    assert_eq!(retired.replacement()?.phase_at_reference_seconds(), displacement);
                    assert!(retired.debt_error().is_none());
                }
                Ok(())
            })?;
        }
''', owner=True, parts=True)
    return tuple(rows)


def phase_step_validation_variant(case, source, mode):
    """Alter only one named Garde closure and its actual consumer's validate call."""
    if mode not in PHASE_STEP_VALIDATION_MODES or case["name"] not in PHASE_STEP_VALIDATION_NAMES:
        raise ValueError("Unknown phase-step validation specimen.")
    if source.count(case["sentinel_from"]) != 1 or source.count(case["scope_start"]) != 1:
        raise ValueError("Actual Garde closure or consumer source anchor changed.")
    if mode == "original":
        return source
    sentinel = source.replace(case["sentinel_from"], case["sentinel_to"])
    start = sentinel.index(case["scope_start"])
    end = sentinel.index(case["scope_end"], start) + len(case["scope_end"])
    selected = sentinel[start:end]
    if selected.count(case["validate_from"]) != 1:
        raise ValueError("Actual consumer validate statement is missing or ambiguous.")
    if mode == "sentinel":
        return sentinel
    return sentinel[:start] + selected.replace(case["validate_from"], case["validate_to"]) + sentinel[end:]


def phase_step_validation_test(case, mode):
    """Append an actual-owner test with code-owned literal outcome expectations."""
    if mode not in PHASE_STEP_VALIDATION_MODES:
        raise ValueError("Unknown phase-step validation mode.")
    if case["body"].count("REFUSAL") < 1:
        raise ValueError("Validation specimen lacks its independent expected outcome.")
    body = case["body"].replace("REFUSAL", "true" if mode == "sentinel" else "false")
    marker = "PHASE_STEP_VALIDATION " + case["name"] + " " + mode
    return ('\n#[cfg(test)]\nmod __phase_step_validation_mechanism {\n    use super::*;\n'
            + case["helpers"] + '\n    #[test]\n    fn observed() -> TestResult {\n'
            + body + '\n        println!("' + marker + '");\n        Ok(())\n    }\n}\n')


def phase_step_validation_mechanisms(session, policy, report, *, deadline):
    """Execute the seven source-bound controls only under separate finite admission."""
    import hashlib
    import math
    from pathlib import Path
    import re
    import shutil
    import time

    limits = policy.get("validation_mechanism_limits")
    if (policy.get("validation_mechanisms_admitted") is not True
            or policy.get("contracts_admitted") is not True
            or type(limits) is not dict or limits != PHASE_STEP_VALIDATION_LIMITS
            or any(type(value) is not int for value in limits.values())
            or policy.get("critical_files") != list(PHASE_STEP_CRITICAL_FILES)
            or policy.get("tool_admission_profile") != "stable-core"
            or type(policy.get("inputs_sha256")) is not dict
            or not set((*PHASE_STEP_INPUTS, PHASE_STEP_VALIDATION_FIXTURE,
                        *PHASE_STEP_CRITICAL_FILES)).issubset(policy["inputs_sha256"])):
        raise ValueError("Phase-step validation mechanisms require their separate exact admission.")
    if (type(deadline) not in (int, float) or not math.isfinite(deadline)
            or not 0 < deadline - time.monotonic() <= policy["contracts_wall_seconds"]):
        raise ValueError("Phase-step validation mechanisms lack the shared finite contract deadline.")
    cases = phase_step_validation_cases()
    if (tuple(row["name"] for row in cases) != PHASE_STEP_VALIDATION_NAMES
            or len(cases) != limits["mechanisms"]
            or any(row["owner_path"] not in PHASE_STEP_CRITICAL_FILES
                   or row["append_path"] != PHASE_STEP_VALIDATION_FIXTURE
                   or row["test_identity"] != PHASE_STEP_VALIDATION_TEST for row in cases)):
        raise ValueError("Phase-step validation mechanism registry differs from its seven owners.")
    prepared = {}
    for case in cases:
        paths = (case["owner_path"], case["append_path"])
        source_text = {name: (session.source / name).read_text() for name in paths}
        if any(hashlib.sha256(text.encode()).hexdigest() != policy["inputs_sha256"][name]
               for name, text in source_text.items()):
            raise ValueError("Validation constructor or actual fixture binding differs.")
        for mode in PHASE_STEP_VALIDATION_MODES:
            prepared[case["name"], mode] = (
                source_text, phase_step_validation_variant(case, source_text[paths[0]], mode),
                phase_step_validation_test(case, mode))
    probe = session.work / "phase-step-validation-source"
    shutil.copytree(session.source, probe)
    originals = dict(session.identity)

    def identity():
        found = {}
        for path in probe.rglob("*"):
            if path.is_symlink() or not (path.is_file() or path.is_dir()):
                raise ValueError("Validation source copy contains an unsupported entry.")
            if path.is_file():
                found[path.relative_to(probe).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
        return found

    if identity() != originals:
        raise ValueError("Validation source copy differs from the captured candidate.")
    evidence = {"status": "incomplete", "limits": dict(limits), "specimens": [],
                "generated_mutation_credit": 0,
                "scope": "Actual seven Garde call/result-use boundaries only; no mutation, native or full-domain claim."}
    report["validation_mechanisms"] = evidence
    for case in cases:
        paths = (case["owner_path"], case["append_path"])
        for mode in PHASE_STEP_VALIDATION_MODES:
            source_text, owner, test = prepared[case["name"], mode]
            try:
                (probe / paths[0]).write_text(owner)
                (probe / paths[1]).write_text(source_text[paths[1]] + test)
                expected = {**originals, paths[0]: hashlib.sha256(owner.encode()).hexdigest(),
                            paths[1]: hashlib.sha256((source_text[paths[1]] + test).encode()).hexdigest()}
                if identity() != expected:
                    raise ValueError("Validation specimen changed unrelated source inputs.")
                label = "validation-" + case["name"] + "-" + mode
                captured = {}
                for name in paths:
                    capture = session.output / (label + "--" + Path(name).name)
                    shutil.copyfile(probe / name, capture)
                    captured[name] = hashlib.sha256(capture.read_bytes()).hexdigest()
                remaining = math.floor(deadline - time.monotonic())
                if remaining < 1:
                    raise ValueError("Shared phase-step contract budget is exhausted.")
                command = [str(session.stable / "bin/cargo"), "test", "--package", "time-core",
                           "--lib", "--frozen", "--jobs", "6", "--manifest-path",
                           "/work/phase-step-validation-source/Cargo.toml", PHASE_STEP_VALIDATION_TEST,
                           "--", "--exact", "--test-threads=1", "--nocapture"]
                output = session.run(label, command, min(90, remaining), sealed=[probe])
                marker = "PHASE_STEP_VALIDATION " + case["name"] + " " + mode
                names = re.findall(r"^test (\S+) \.\.\.", output, re.MULTILINE)
                summaries = re.findall(r"^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", output, re.MULTILINE)
                if (names != [PHASE_STEP_VALIDATION_TEST] or summaries != [("1", "0", "0")]
                        or output.count(marker) != 1 or output.count("running 1 test") != 1
                        or identity() != expected):
                    raise ValueError("Actual validation consumer or intended neighboring assertions did not execute exactly once.")
                evidence["specimens"].append({"name": case["name"], "mode": mode,
                    "raw_type": case["raw_type"], "test_identity": PHASE_STEP_VALIDATION_TEST,
                    "captured_sources_sha256": captured, "status": "passed", "generated_mutation_credit": 0})
            finally:
                for name, text in source_text.items():
                    (probe / name).write_text(text)
                if identity() != originals:
                    raise ValueError("Validation source copy failed exact restoration.")
    if len(evidence["specimens"]) != limits["test_executions"]:
        raise ValueError("Phase-step validation controls are incomplete.")
    if time.monotonic() > deadline:
        raise ValueError("Shared phase-step contract budget is exhausted.")
    evidence["status"] = "passed"
