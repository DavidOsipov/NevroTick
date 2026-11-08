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
