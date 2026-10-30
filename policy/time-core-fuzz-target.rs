//! SOURCE-ONLY DRAFT: raw fixture history input, actual public append, held-state oracle.
//! GPL-2.0-only, David Osipov with AI assistance, 30 October 2026.
//! Contract53ce5e686cbb88175bd7138302f35c5d02939316615134451ba74c3fee847ea0.
//! Frozen facts ec6a1b96806c6219bd2b74c37b737fde7cbfddafe6cf2f07b12cb14d1d9c92d1.
//! Test-only format: header enum0..4 then n<=65 big-endian(time:u64,phase:i64,divisor:u64).
//! The header is NOT a count:0=base17/23/29,1=max endpoint tags,2/3/4=base
//! with only the final row's source19/frame31/generation31 mismatch respectively.
//! Every phase value/bit is preserved, including i64::MIN: no phase sentinels.
//! Valid length1+24*n<=1561; unsupported header/shape performs no product call.
//! One fresh empty actual history per callback; no hidden prefill/cross-callback state.
//! All public calls are precharged. Nested normalizer/statistics/point/input work is
//! only a source-owned bound:65/63/2141/63 max, never observed direct calls.
//! Owned post-event snapshots inspect the independently expected prefix, not a
//! returned count. Complete counters precede every oracle assertion/observer fault.
//! No second normalization/statistics, direct fit or point constructor is called.
//! Frozen29 projected cases retain literal raw fields and expected rows/report bits.
//! Full70 trace uses initial64 commits plus each specific65th boundary; paired
//! two-live-history lifecycle remains public-test coverage, not claimed here.
//! Actual body/resource/profile/15s admission remains root-owned and pending.
#![no_main]
#![forbid(unsafe_code)]

use libfuzzer_sys::fuzz_target;
use std::io::Write;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicUsize, Ordering};
use time_core::{
    FitError, FixtureContext, FixtureHistory, FixtureObservation, HistoryError, NormalizationError,
    ObservationError, PointIssue, ResidualError,
};
use time_domain::{DomainError, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

static RAW: AtomicUsize = AtomicUsize::new(0);
static TOTALS: [AtomicUsize; 12] = [const { AtomicUsize::new(0) }; 12];

/// Closed precharge owners; direct normalizer/stat/fit/point slots stay zero.
#[derive(Clone, Copy)]
enum Field {
    History,
    Append,
    Observation,
    Context,
    Monotonic,
    Coordinate,
    Exchange,
    Getter,
}
impl Field {
    /// Map every closed owner into the fixed12-field transport, without effects.
    const fn index(self) -> usize {
        match self {
            Self::History => 0,
            Self::Append => 1,
            Self::Observation => 2,
            Self::Context => 3,
            Self::Monotonic => 4,
            Self::Coordinate => 5,
            Self::Exchange => 6,
            Self::Getter => 7,
        }
    }
}
struct Counts {
    raw: usize,
    values: [usize; 12],
    emitted: bool,
}
impl Counts {
    /// Begin one callback; fixed input bounds keep all per-callback scalars finite.
    fn new() -> Self {
        Self {
            raw: RAW.fetch_add(1, Ordering::Relaxed) + 1,
            values: [0; 12],
            emitted: false,
        }
    }
    /// Increment immediately before one actual public call of the closed owner.
    fn bump(&mut self, field: Field) {
        self.values[field.index()] += 1;
    }
    /// Emit one complete cumulative record; Drop emits the same real prefix on panic.
    fn emit(&mut self) {
        if self.emitted {
            return;
        }
        let mut totals = [0; 12];
        for ((destination, global), value) in
            totals.iter_mut().zip(TOTALS.iter()).zip(self.values.iter())
        {
            *destination = global.fetch_add(*value, Ordering::Relaxed) + *value;
        }
        let c = totals;
        let row = format!(
            "CORE_HISTORY_FUZZ_COUNTS raw={} histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} normalizations={} stats={} fits={} points={}\n",
            self.raw, c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10], c[11]
        );
        self.emitted = true;
        std::io::stderr()
            .lock()
            .write_all(row.as_bytes())
            .expect("complete history counter sink");
    }
}
impl Drop for Counts {
    fn drop(&mut self) {
        self.emit();
    }
}
macro_rules! get {
    ($counts:expr,$value:expr) => {{
        $counts.bump(Field::Getter);
        $value
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ContextKey {
    source: u64,
    frame: u64,
    generation: u64,
}
/// Only these nonzero literal tag sets can enter the actual context constructor.
#[derive(Clone, Copy)]
enum ContextKind {
    Base,
    Maximum,
    SourceMismatch,
    FrameMismatch,
    GenerationMismatch,
}
impl ContextKind {
    /// Return exact asserted tags, with no raw/public factory or authority claim.
    const fn key(self) -> ContextKey {
        match self {
            Self::Base => ContextKey {
                source: 17,
                frame: 23,
                generation: 29,
            },
            Self::Maximum => ContextKey {
                source: u64::MAX,
                frame: u64::MAX - 1,
                generation: u64::MAX - 2,
            },
            Self::SourceMismatch => ContextKey {
                source: 19,
                frame: 23,
                generation: 29,
            },
            Self::FrameMismatch => ContextKey {
                source: 17,
                frame: 31,
                generation: 29,
            },
            Self::GenerationMismatch => ContextKey {
                source: 17,
                frame: 23,
                generation: 31,
            },
        }
    }
}
#[derive(Clone, Copy)]
enum Header {
    Base,
    Maximum,
    SourceMismatch,
    FrameMismatch,
    GenerationMismatch,
}
impl Header {
    /// Accept every byte; unsupported enum encodings reject without product work.
    fn decode(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Base),
            1 => Some(Self::Maximum),
            2 => Some(Self::SourceMismatch),
            3 => Some(Self::FrameMismatch),
            4 => Some(Self::GenerationMismatch),
            _ => None,
        }
    }
    const fn code(self) -> u8 {
        match self {
            Self::Base => 0,
            Self::Maximum => 1,
            Self::SourceMismatch => 2,
            Self::FrameMismatch => 3,
            Self::GenerationMismatch => 4,
        }
    }
    const fn base(self) -> ContextKind {
        match self {
            Self::Maximum => ContextKind::Maximum,
            _ => ContextKind::Base,
        }
    }
    const fn last(self) -> ContextKind {
        match self {
            Self::Base => ContextKind::Base,
            Self::Maximum => ContextKind::Maximum,
            Self::SourceMismatch => ContextKind::SourceMismatch,
            Self::FrameMismatch => ContextKind::FrameMismatch,
            Self::GenerationMismatch => ContextKind::GenerationMismatch,
        }
    }
}
/// Frozen test-only scalars; this is not a checked product observation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Raw {
    context: ContextKey,
    time: u64,
    phase: i64,
    divisor: u64,
}

/// Decode any byte slice with a hard65-row/1561-byte bound; no product/parser effect.
fn decode(data: &[u8]) -> Option<(Header, Vec<Raw>)> {
    if data.is_empty() || data.len() > 1561 || (data.len() - 1) % 24 != 0 {
        return None;
    }
    let header = Header::decode(*data.first()?)?;
    let count = (data.len() - 1) / 24;
    let mut rows = Vec::with_capacity(count);
    for (index, chunk) in data.get(1..)?.chunks_exact(24).enumerate() {
        let time = u64::from_be_bytes(chunk.get(0..8)?.try_into().ok()?);
        let phase = i64::from_be_bytes(chunk.get(8..16)?.try_into().ok()?);
        let divisor = u64::from_be_bytes(chunk.get(16..24)?.try_into().ok()?);
        let context = if index + 1 == count {
            header.last()
        } else {
            header.base()
        }
        .key();
        rows.push(Raw {
            context,
            time,
            phase,
            divisor,
        });
    }
    Some((header, rows))
}

/// Construct only a closed nonzero literal context, charging the actual attempt.
/// All ContextKind variants own their nonzero tags; no arbitrary raw key is accepted.
fn context(counts: &mut Counts, kind: ContextKind) -> FixtureContext {
    let key = kind.key();
    let s = NonZeroU64::new(key.source).expect("closed nonzero source");
    let f = NonZeroU64::new(key.frame).expect("closed nonzero frame");
    let g = NonZeroU64::new(key.generation).expect("closed nonzero generation");
    counts.bump(Field::Context);
    FixtureContext::new(s, f, g)
}
/// Read all three actual tags; std NonZero::get is not another product getter.
fn context_key(counts: &mut Counts, value: FixtureContext) -> ContextKey {
    ContextKey {
        source: get!(counts, value.source_id()).get(),
        frame: get!(counts, value.frame()).get(),
        generation: get!(counts, value.generation()).get(),
    }
}
enum BuildError {
    Domain(DomainError),
    Observation(ObservationError),
}
/// Admit any raw i128 through the actual UTC owner; retain its typed cause.
fn coordinate(counts: &mut Counts, ticks: i128) -> Result<UtcCoordinateQ32, BuildError> {
    counts.bump(Field::Coordinate);
    UtcCoordinateQ32::new(ticks).map_err(BuildError::Domain)
}
/// Real checked scalar path: one monotonic, four UTC, one exchange, one observation.
/// i64 phase maps exactly to raw ticks[0,p,0,0]; no remapping or rounding occurs.
fn observation(
    counts: &mut Counts,
    c: FixtureContext,
    row: Raw,
) -> (Result<FixtureObservation, BuildError>, Option<i128>) {
    counts.bump(Field::Monotonic);
    let time = MonotonicNs::new(row.time);
    let coordinates = (|| {
        let a = coordinate(counts, 0)?;
        let b = coordinate(counts, i128::from(row.phase))?;
        let c = coordinate(counts, 0)?;
        let d = coordinate(counts, 0)?;
        Ok((a, b, c, d))
    })();
    let (a, b, c0, d) = match coordinates {
        Ok(value) => value,
        Err(error) => return (Err(error), None),
    };
    counts.bump(Field::Exchange);
    let exchange = match exchange_arithmetic(a, b, c0, d) {
        Ok(value) => value,
        Err(error) => return (Err(BuildError::Domain(error)), None),
    };
    let phase = get!(counts, exchange.offset());
    let numerator = get!(counts, phase.numerator());
    counts.bump(Field::Observation);
    (
        FixtureObservation::new(c, time, phase, f64::from_bits(row.divisor))
            .map_err(BuildError::Observation),
        Some(numerator),
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RowObs {
    context: ContextKey,
    time: u64,
    phase: i128,
    divisor: u64,
}
/// Inspect actual immutable scalar representations; exactly nine product getters.
fn row_observation(counts: &mut Counts, row: &FixtureObservation) -> RowObs {
    let c = get!(counts, row.context());
    let context = context_key(counts, c);
    let t = get!(counts, row.observed_at());
    let time = get!(counts, t.nanoseconds());
    let p = get!(counts, row.phase());
    let phase = get!(counts, p.numerator());
    let divisor = get!(counts, row.variance_divisor()).to_bits();
    RowObs {
        context,
        time,
        phase,
        divisor,
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct ReportObs {
    context: ContextKey,
    reference: u64,
    frame: u64,
    count: usize,
    dof: usize,
    line_frame: u64,
    bits: [u64; 5],
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    context: ContextKey,
    count: usize,
    reference: Option<u64>,
    rows: Vec<Option<RowObs>>,
    at_count_none: bool,
    at_max_none: bool,
    report: Option<ReportObs>,
    bounded_expected: bool,
}
/// Observe only the independently expected prefix (max64), never returned count.
/// Missing/extra rows become owned evidence; no oracle assertion precedes emission.
fn snapshot(counts: &mut Counts, h: &FixtureHistory, expected_length: usize) -> Snapshot {
    let c = get!(counts, h.context());
    let context = context_key(counts, c);
    let count = get!(counts, h.sample_count());
    let reference = get!(counts, h.reference()).map(|r| get!(counts, r.nanoseconds()));
    let mut rows = Vec::with_capacity(expected_length.min(64));
    for index in 0..expected_length.min(64) {
        let value = get!(counts, h.observation(index));
        rows.push(value.map(|row| row_observation(counts, row)));
    }
    let at_count_none = get!(counts, h.observation(expected_length)).is_none();
    let at_max_none = get!(counts, h.observation(usize::MAX)).is_none();
    let report = get!(counts, h.report()).map(|value| {
        let c = get!(counts, value.context());
        let context = context_key(counts, c);
        let reference = get!(counts, value.reference());
        let reference = get!(counts, reference.nanoseconds());
        let summary = get!(counts, value.statistics());
        let frame = get!(counts, summary.frame()).get();
        let count = get!(counts, summary.sample_count());
        let dof = get!(counts, summary.degrees_of_freedom());
        let e = get!(counts, summary.weighted_sse_seconds_squared()).to_bits();
        let s = get!(counts, summary.residual_scale_seconds_squared()).to_bits();
        let v = get!(counts, summary.normalized_data_variance_seconds_squared()).to_bits();
        let line = get!(counts, summary.line());
        let line_frame = get!(counts, line.frame()).get();
        let a = get!(counts, line.phase_seconds()).to_bits();
        let b = get!(counts, line.phase_rate_seconds_per_second()).to_bits();
        ReportObs {
            context,
            reference,
            frame,
            count,
            dof,
            line_frame,
            bits: [a, b, e, s, v],
        }
    });
    Snapshot {
        context,
        count,
        reference,
        rows,
        at_count_none,
        at_max_none,
        report,
        bounded_expected: expected_length <= 64,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Ok,
    Context,
    Duplicate,
    Older,
    Window(usize),
    Span,
    Spread,
    StatisticsArithmetic,
    Unexpected,
}
/// Classify only closed actual typed causes; arbitrary other errors stay unexpected.
/// The catchall still includes Ok, so the temporary RED variant need not be named.
fn append_kind(value: &Result<(), HistoryError>) -> Kind {
    if value.is_ok() {
        return Kind::Ok;
    }
    if matches!(value, Err(HistoryError::ContextMismatch)) {
        return Kind::Context;
    }
    if matches!(value, Err(HistoryError::DuplicateTime)) {
        return Kind::Duplicate;
    }
    if matches!(value, Err(HistoryError::OutOfOrderTime)) {
        return Kind::Older;
    }
    match value {
        Err(HistoryError::Normalization {
            source: NormalizationError::ElapsedRange { index },
        }) => Kind::Window(*index),
        Err(HistoryError::Normalization {
            source: NormalizationError::SpanTooShort,
        }) => Kind::Span,
        Err(HistoryError::Normalization {
            source: NormalizationError::Input(FitError::InsufficientSpread),
        }) => Kind::Spread,
        Err(HistoryError::Statistics {
            source:
                ResidualError::Fit(FitError::Arithmetic { .. })
                | ResidualError::Arithmetic {
                    source: FitError::Arithmetic { .. },
                    ..
                },
        }) => Kind::StatisticsArithmetic,
        _ => Kind::Unexpected,
    }
}
/// Independent raw observation admission predicates; no converted-output oracle.
fn observation_issue(row: Raw) -> Option<ObservationError> {
    if !(-8_589_934_592..=8_589_934_592).contains(&row.phase) {
        return Some(ObservationError::PhaseOutOfRange);
    }
    let d = f64::from_bits(row.divisor);
    let issue = if !d.is_finite() {
        Some(PointIssue::NonFinite)
    } else if d.is_subnormal() {
        Some(PointIssue::Subnormal)
    } else if !(0.0625..=16.0).contains(&d) {
        Some(PointIssue::OutOfRange)
    } else {
        None
    };
    issue.map(|issue| ObservationError::Divisor { issue })
}
/// Canonical raw-list staging accepts any slice but rejects an invalid model bound.
/// It selects the last63 retained rows at capacity, never models normalization/fit.
fn staged(rows: &[Raw], incoming: Raw) -> Option<Vec<Raw>> {
    if rows.len() > 64 {
        return None;
    }
    let mut next = Vec::with_capacity(64);
    next.extend_from_slice(if rows.len() == 64 {
        rows.get(1..)?
    } else {
        rows
    });
    next.push(incoming);
    Some(next)
}
/// Independently predictable raw first errors only; None leaves numeric geometry open.
fn raw_append_error(base: ContextKey, rows: &[Raw], incoming: Raw, next: &[Raw]) -> Option<Kind> {
    if incoming.context != base {
        return Some(Kind::Context);
    }
    if let Some(last) = rows.last() {
        if incoming.time == last.time {
            return Some(Kind::Duplicate);
        }
        if incoming.time < last.time {
            return Some(Kind::Older);
        }
    }
    for (index, row) in next.iter().enumerate() {
        let Some(elapsed) = incoming.time.checked_sub(row.time) else {
            return Some(Kind::Unexpected);
        };
        if elapsed > 4_096_000_000_000 {
            return Some(Kind::Window(index));
        }
    }
    if next.len() >= 3 {
        let Some(first) = next.first() else {
            return Some(Kind::Unexpected);
        };
        if incoming.time - first.time < 62_500_000 {
            return Some(Kind::Span);
        }
    }
    None
}
#[derive(Clone, Copy)]
struct ReportExpected {
    context: ContextKey,
    reference: u64,
    frame: u64,
    count: usize,
    dof: usize,
    bits: [u64; 5],
}
struct KnownStep {
    kind: Kind,
    retained: &'static [usize],
    reference: Option<u64>,
    report: Option<ReportExpected>,
}
struct KnownCase {
    name: &'static str,
    mode: u8,
    rows: &'static [Raw],
    steps: &'static [KnownStep],
}
/// Exact frozen raw tuple matching only; no product output can select expected bits.
fn known_case(header: Header, rows: &[Raw]) -> Option<&'static KnownCase> {
    CASES
        .iter()
        .find(|case| case.mode == header.code() && case.rows == rows)
}
struct Event {
    row: Raw,
    build: Option<BuildError>,
    phase: Option<i128>,
    caller: Option<RowObs>,
    result: Option<Result<(), HistoryError>>,
    expected: Option<Kind>,
    expected_rows: Vec<Raw>,
    known_reference: Option<Option<u64>>,
    known_report: Option<Option<ReportExpected>>,
    after: Snapshot,
    table_valid: bool,
}
/// Compare public raw values with literal/canonical expected rows, after emission.
fn verify_snapshot(
    actual: &Snapshot,
    base: ContextKey,
    rows: &[Raw],
    reference: Option<Option<u64>>,
    report: Option<Option<ReportExpected>>,
) {
    assert!(
        actual.bounded_expected && rows.len() <= 64,
        "independent history oracle: model bound"
    );
    assert!(
        actual.context == base && actual.count == rows.len(),
        "independent history oracle: metadata"
    );
    let expected_reference = reference.unwrap_or_else(|| rows.last().map(|r| r.time));
    assert!(
        actual.reference == expected_reference,
        "independent history oracle: exact committed reference"
    );
    assert!(
        actual.at_count_none && actual.at_max_none && actual.rows.len() == rows.len(),
        "independent history oracle: prefix extent"
    );
    for (value, raw) in actual.rows.iter().zip(rows) {
        let Some(value) = value else {
            panic!("independent history oracle: missing committed row")
        };
        assert!(
            value.context == raw.context
                && value.time == raw.time
                && value.phase == i128::from(raw.phase)
                && value.divisor == raw.divisor,
            "independent history oracle: actual retained raw row"
        );
    }
    assert!(
        actual.report.is_some() == (rows.len() >= 3),
        "independent history oracle: report presence"
    );
    if let Some(value) = &actual.report {
        assert!(
            value.context == base
                && value.reference == expected_reference.unwrap_or(0)
                && value.frame == base.frame
                && value.line_frame == base.frame
                && value.count == rows.len()
                && value.dof == rows.len() - 2,
            "independent history oracle: associated report metadata"
        );
        for (index, bits) in value.bits.iter().enumerate() {
            let scalar = f64::from_bits(*bits);
            assert!(
                scalar.is_finite() && !scalar.is_subnormal(),
                "independent history oracle: supported report value"
            );
            if index >= 2 {
                assert!(
                    scalar >= 0.0,
                    "independent history oracle: nonnegative statistic"
                );
            }
        }
    }
    if let Some(expected) = report {
        assert!(
            actual.report.is_some() == expected.is_some(),
            "independent history oracle: frozen report presence"
        );
        if let (Some(value), Some(expected)) = (&actual.report, expected) {
            assert!(
                value.context == expected.context
                    && value.reference == expected.reference
                    && value.frame == expected.frame
                    && value.line_frame == expected.frame
                    && value.count == expected.count
                    && value.dof == expected.dof
                    && value.bits == expected.bits,
                "independent history oracle: frozen published report bits"
            );
        }
    }
}

/// Run one bounded callback. All assertions remain in the later observation phase.
fn exercise(data: &[u8]) {
    let mut counts = Counts::new();
    let Some((header, raws)) = decode(data) else {
        counts.emit();
        return;
    };
    let base_kind = header.base();
    let base_key = base_kind.key();
    let base = context(&mut counts, base_kind);
    let alternate = if !raws.is_empty() && header.last().key() != base_key {
        Some(context(&mut counts, header.last()))
    } else {
        None
    };
    counts.bump(Field::History);
    let mut history = FixtureHistory::new(base);
    let initial = snapshot(&mut counts, &history, 0);
    let known = known_case(header, &raws);
    let mut committed = Vec::with_capacity(64);
    let mut events = Vec::with_capacity(raws.len());
    for (index, row) in raws.iter().copied().enumerate() {
        let selected = if index + 1 == raws.len() {
            alternate.unwrap_or(base)
        } else {
            base
        };
        let (built, phase) = observation(&mut counts, selected, row);
        let mut build = None;
        let mut result = None;
        let mut caller = None;
        let mut expected = None;
        let mut table_valid = true;
        let mut known_reference = None;
        let mut known_report = None;
        match built {
            Err(error) => build = Some(error),
            Ok(observation) => {
                let next = staged(&committed, row);
                if let Some(next) = next {
                    expected = raw_append_error(base_key, &committed, row, &next);
                    if expected.is_none() && next.len() < 3 {
                        expected = Some(Kind::Ok);
                    }
                    counts.bump(Field::Append);
                    let actual = history.try_append(observation);
                    caller = Some(row_observation(&mut counts, &observation));
                    if known.is_none()
                        && matches!(expected, None | Some(Kind::Ok))
                        && actual.is_ok()
                    {
                        committed = next;
                    }
                    result = Some(actual);
                } else {
                    table_valid = false;
                }
            }
        }
        if let Some(case) = known {
            if let Some(step) = case.steps.get(index) {
                expected = Some(step.kind);
                known_reference = Some(step.reference);
                known_report = Some(step.report);
                committed.clear();
                for selected in step.retained {
                    if let Some(value) = raws.get(*selected) {
                        committed.push(*value);
                    } else {
                        table_valid = false;
                    }
                }
                if committed.len() > 64 {
                    table_valid = false;
                }
            } else {
                table_valid = false;
            }
        }
        let after = snapshot(&mut counts, &history, committed.len());
        events.push(Event {
            row,
            build,
            phase,
            caller,
            result,
            expected,
            expected_rows: committed.clone(),
            known_reference,
            known_report,
            after,
            table_valid,
        });
    }
    counts.emit();

    // A test-only observer corruption discriminates independent numeric assertions.
    // It changes no product value, constructor call, expected table or raw input.
    #[cfg(feature = "seeded-defect")]
    if known.is_some_and(|case| case.name == "changing_data_same_frame_report") {
        if let Some(report) = events
            .last_mut()
            .and_then(|event| event.after.report.as_mut())
        {
            report.bits[0] ^= 1;
        }
    }

    verify_snapshot(&initial, base_key, &[], Some(None), Some(None));
    let mut previous = &initial;
    let mut known_report_checked = false;
    for event in &events {
        assert!(
            event.table_valid,
            "independent history oracle: frozen table binding"
        );
        if let Some(numerator) = event.phase {
            assert!(
                numerator == i128::from(event.row.phase),
                "independent history oracle: exact raw domain phase"
            );
        }
        match (&event.build, observation_issue(event.row)) {
            (Some(BuildError::Observation(actual)), Some(expected)) => assert!(
                *actual == expected,
                "independent history oracle: raw observation rejection"
            ),
            (None, None) => {}
            (Some(BuildError::Domain(cause)), _) => {
                let _ = cause;
                panic!("independent history oracle: unexpected domain setup rejection")
            }
            _ => panic!("independent history oracle: observation admission"),
        }
        if let Some(caller) = &event.caller {
            assert!(
                caller.context == event.row.context
                    && caller.time == event.row.time
                    && caller.phase == i128::from(event.row.phase)
                    && caller.divisor == event.row.divisor,
                "independent history oracle: caller observation preservation"
            );
        }
        if let Some(result) = &event.result {
            let actual = append_kind(result);
            if let Some(expected) = event.expected {
                assert!(
                    actual == expected,
                    "independent history oracle: exact append result"
                );
            } else {
                assert!(
                    matches!(actual, Kind::Ok | Kind::Spread | Kind::StatisticsArithmetic),
                    "independent history oracle: unexpected append failure"
                );
            }
            if result.is_err() {
                assert!(
                    event.after == *previous,
                    "independent history oracle: failed append changed state"
                );
            }
        } else {
            assert!(
                event.build.is_some(),
                "independent history oracle: missing append"
            );
            assert!(
                event.after == *previous,
                "independent history oracle: constructor changed history"
            );
        }
        verify_snapshot(
            &event.after,
            base_key,
            &event.expected_rows,
            event.known_reference,
            event.known_report,
        );
        if event.known_report.is_some_and(|report| report.is_some()) {
            known_report_checked = true;
        }
        previous = &event.after;
    }
    if known_report_checked {
        let marker = format!(
            "HISTORY_FUZZ_KNOWN_REPORT_CHECKED {}\n",
            known.map_or("unknown", |case| case.name)
        );
        std::io::stderr()
            .lock()
            .write_all(marker.as_bytes())
            .expect("history semantic marker sink");
    }
}

// Literal frozen projected cases are appended below. No normalizer/statistics model.
fuzz_target!(|data: &[u8]| {
    exercise(data);
});

// Input/output literals from independently checkedec6 facts; projection ordinals are explicit.
static CASES: &[KnownCase] = &[
    // empty_immutable_endpoint_labels; source ordinals
    KnownCase {
        name: "empty_immutable_endpoint_labels",
        mode: 1,
        rows: &[],
        steps: &[],
    },
    // partial_guards_and_labels; source ordinals 0
    KnownCase {
        name: "partial-append_0",
        mode: 0,
        rows: &[Raw {
            context: ContextKey {
                source: 17,
                frame: 23,
                generation: 29,
            },
            time: 0,
            phase: 0,
            divisor: 0x3ff0000000000000,
        }],
        steps: &[KnownStep {
            kind: Kind::Ok,
            retained: &[0],
            reference: Some(0),
            report: None,
        }],
    },
    // partial_guards_and_labels; source ordinals 0,1
    KnownCase {
        name: "partial-append_1",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
        ],
    },
    // partial_guards_and_labels; source ordinals 0,1,2
    KnownCase {
        name: "partial-source_before_duplicate",
        mode: 2,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 19,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Context,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
        ],
    },
    // partial_guards_and_labels; source ordinals 0,1,3
    KnownCase {
        name: "partial-frame_before_older",
        mode: 3,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 31,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Context,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
        ],
    },
    // partial_guards_and_labels; source ordinals 0,1,4
    KnownCase {
        name: "partial-generation_before_newer",
        mode: 4,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 31,
                },
                time: 2000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Context,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
        ],
    },
    // partial_guards_and_labels; source ordinals 0,1,5
    KnownCase {
        name: "partial-duplicate",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Duplicate,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
        ],
    },
    // partial_guards_and_labels; source ordinals 0,1,6
    KnownCase {
        name: "partial-older",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Older,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
        ],
    },
    // partial_guards_and_labels; source ordinals 0,1,7
    KnownCase {
        name: "partial-healthy_third",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 2000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(2000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 2000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // origin_zero_positive_quarter; source ordinals 0,1,2
    KnownCase {
        name: "origin_zero_positive_quarter",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 2000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(2000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 2000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // origin_u64max_positive_quarter; source ordinals 0,1,2
    KnownCase {
        name: "origin_u64max_positive_quarter",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 18446744071709551615,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 18446744072709551615,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 18446744073709551615,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(18446744071709551615),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(18446744072709551615),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(18446744073709551615),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 18446744073709551615,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // u64max_single_then_rejections; source ordinals 0,1,2
    KnownCase {
        name: "u64max_single_then_rejections",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 18446744073709551615,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 18446744073709551615,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 18446744073709551614,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(18446744073709551615),
                report: None,
            },
            KnownStep {
                kind: Kind::Duplicate,
                retained: &[0],
                reference: Some(18446744073709551615),
                report: None,
            },
            KnownStep {
                kind: Kind::Older,
                retained: &[0],
                reference: Some(18446744073709551615),
                report: None,
            },
        ],
    },
    // minimum_span_neighbors; source ordinals 0,1,2,3,4
    KnownCase {
        name: "minimum_span_neighbors",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 31250000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 62499999,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 62500000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 62500001,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(31250000),
                report: None,
            },
            KnownStep {
                kind: Kind::Span,
                retained: &[0, 1],
                reference: Some(31250000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 3],
                reference: Some(62500000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 62500000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 3, 4],
                reference: Some(62500001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 62500001,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // two_row_maximum_neighbors; source ordinals 0,1,2,3
    KnownCase {
        name: "two_row_maximum_neighbors",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000001,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4095999999999,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Window(0),
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 2],
                reference: Some(4095999999999),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 2, 3],
                reference: Some(4096000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4096000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // two_row_exact_maximum_no_prune; source ordinals 0,1,2
    KnownCase {
        name: "two_row_exact_maximum_no_prune",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000001,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(4096000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Window(0),
                retained: &[0, 1],
                reference: Some(4096000000000),
                report: None,
            },
        ],
    },
    // report_window_failure_then_neighbor; source ordinals 0,1,2,3,4
    KnownCase {
        name: "report_window_failure_then_neighbor",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 2000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000001,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(2000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 2000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Window(0),
                retained: &[0, 1, 2],
                reference: Some(2000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 2000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 4],
                reference: Some(4096000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4096000000000,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63
    KnownCase {
        name: "full64-build-neighbor",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64
    KnownCase {
        name: "full65-context_before_duplicate_full",
        mode: 2,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 19,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Context,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,65
    KnownCase {
        name: "full65-duplicate_full",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Duplicate,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,66
    KnownCase {
        name: "full65-older_full",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Older,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,67
    KnownCase {
        name: "full65-window_failure_full",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4098000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Window(0),
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,68
    KnownCase {
        name: "full65-span_failure_full",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000063,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Span,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // full64_replace_and_failures; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,69
    KnownCase {
        name: "full65-healthy_65th",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000001,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000002,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000003,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000004,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000005,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000006,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000007,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000008,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000009,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000010,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000011,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000012,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000013,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000014,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000015,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000016,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000017,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000018,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000019,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000020,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000021,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000022,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000023,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000024,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000025,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000026,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000027,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000028,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000029,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000030,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000031,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000032,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000033,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000034,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000035,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000036,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000037,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000038,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000039,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000040,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000041,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000042,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000043,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000044,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000045,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000046,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000047,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000048,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000049,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000050,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000051,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000052,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000053,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000054,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000055,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000056,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000057,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000058,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000059,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000060,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000061,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000062,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1062500000,
                phase: 2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(1000000001),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000001,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(1000000002),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000002,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(1000000003),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000003,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(1000000004),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000004,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(1000000005),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000005,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(1000000006),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000006,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(1000000007),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000007,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(1000000008),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000008,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(1000000009),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000009,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(1000000010),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000010,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(1000000011),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000011,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(1000000012),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000012,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(1000000013),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000013,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(1000000014),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000014,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(1000000015),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000015,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                reference: Some(1000000016),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000016,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
                reference: Some(1000000017),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000017,
                    frame: 23,
                    count: 19,
                    dof: 17,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ],
                reference: Some(1000000018),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000018,
                    frame: 23,
                    count: 20,
                    dof: 18,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                reference: Some(1000000019),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000019,
                    frame: 23,
                    count: 21,
                    dof: 19,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
                reference: Some(1000000020),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000020,
                    frame: 23,
                    count: 22,
                    dof: 20,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22,
                ],
                reference: Some(1000000021),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000021,
                    frame: 23,
                    count: 23,
                    dof: 21,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23,
                ],
                reference: Some(1000000022),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000022,
                    frame: 23,
                    count: 24,
                    dof: 22,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24,
                ],
                reference: Some(1000000023),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000023,
                    frame: 23,
                    count: 25,
                    dof: 23,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25,
                ],
                reference: Some(1000000024),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000024,
                    frame: 23,
                    count: 26,
                    dof: 24,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26,
                ],
                reference: Some(1000000025),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000025,
                    frame: 23,
                    count: 27,
                    dof: 25,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27,
                ],
                reference: Some(1000000026),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000026,
                    frame: 23,
                    count: 28,
                    dof: 26,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28,
                ],
                reference: Some(1000000027),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000027,
                    frame: 23,
                    count: 29,
                    dof: 27,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
                ],
                reference: Some(1000000028),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000028,
                    frame: 23,
                    count: 30,
                    dof: 28,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30,
                ],
                reference: Some(1000000029),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000029,
                    frame: 23,
                    count: 31,
                    dof: 29,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
                reference: Some(1000000030),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000030,
                    frame: 23,
                    count: 32,
                    dof: 30,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                reference: Some(1000000031),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000031,
                    frame: 23,
                    count: 33,
                    dof: 31,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                reference: Some(1000000032),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000032,
                    frame: 23,
                    count: 34,
                    dof: 32,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                ],
                reference: Some(1000000033),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000033,
                    frame: 23,
                    count: 35,
                    dof: 33,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                ],
                reference: Some(1000000034),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000034,
                    frame: 23,
                    count: 36,
                    dof: 34,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                ],
                reference: Some(1000000035),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000035,
                    frame: 23,
                    count: 37,
                    dof: 35,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                ],
                reference: Some(1000000036),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000036,
                    frame: 23,
                    count: 38,
                    dof: 36,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                ],
                reference: Some(1000000037),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000037,
                    frame: 23,
                    count: 39,
                    dof: 37,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                reference: Some(1000000038),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000038,
                    frame: 23,
                    count: 40,
                    dof: 38,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                ],
                reference: Some(1000000039),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000039,
                    frame: 23,
                    count: 41,
                    dof: 39,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                ],
                reference: Some(1000000040),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000040,
                    frame: 23,
                    count: 42,
                    dof: 40,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42,
                ],
                reference: Some(1000000041),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000041,
                    frame: 23,
                    count: 43,
                    dof: 41,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43,
                ],
                reference: Some(1000000042),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000042,
                    frame: 23,
                    count: 44,
                    dof: 42,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44,
                ],
                reference: Some(1000000043),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000043,
                    frame: 23,
                    count: 45,
                    dof: 43,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45,
                ],
                reference: Some(1000000044),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000044,
                    frame: 23,
                    count: 46,
                    dof: 44,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46,
                ],
                reference: Some(1000000045),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000045,
                    frame: 23,
                    count: 47,
                    dof: 45,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47,
                ],
                reference: Some(1000000046),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000046,
                    frame: 23,
                    count: 48,
                    dof: 46,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48,
                ],
                reference: Some(1000000047),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000047,
                    frame: 23,
                    count: 49,
                    dof: 47,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49,
                ],
                reference: Some(1000000048),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000048,
                    frame: 23,
                    count: 50,
                    dof: 48,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50,
                ],
                reference: Some(1000000049),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000049,
                    frame: 23,
                    count: 51,
                    dof: 49,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                reference: Some(1000000050),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000050,
                    frame: 23,
                    count: 52,
                    dof: 50,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                ],
                reference: Some(1000000051),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000051,
                    frame: 23,
                    count: 53,
                    dof: 51,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
                reference: Some(1000000052),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000052,
                    frame: 23,
                    count: 54,
                    dof: 52,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
                ],
                reference: Some(1000000053),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000053,
                    frame: 23,
                    count: 55,
                    dof: 53,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                ],
                reference: Some(1000000054),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000054,
                    frame: 23,
                    count: 56,
                    dof: 54,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                ],
                reference: Some(1000000055),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000055,
                    frame: 23,
                    count: 57,
                    dof: 55,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ],
                reference: Some(1000000056),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000056,
                    frame: 23,
                    count: 58,
                    dof: 56,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ],
                reference: Some(1000000057),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000057,
                    frame: 23,
                    count: 59,
                    dof: 57,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                ],
                reference: Some(1000000058),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000058,
                    frame: 23,
                    count: 60,
                    dof: 58,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                reference: Some(1000000059),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000059,
                    frame: 23,
                    count: 61,
                    dof: 59,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                ],
                reference: Some(1000000060),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000060,
                    frame: 23,
                    count: 62,
                    dof: 60,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62,
                ],
                reference: Some(1000000061),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000061,
                    frame: 23,
                    count: 63,
                    dof: 61,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                reference: Some(1000000062),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1000000062,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42,
                    43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
                    63, 64,
                ],
                reference: Some(1062500000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 1062500000,
                    frame: 23,
                    count: 64,
                    dof: 62,
                    bits: [
                        0x3fd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // spread_calculated_below; source ordinals 0,1,2,3
    KnownCase {
        name: "spread_calculated_below",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4094000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4095000000000,
                phase: 0,
                divisor: 0x3fc041041041040f,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(4094000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(4095000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Spread,
                retained: &[0, 1],
                reference: Some(4095000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 3],
                reference: Some(4096000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4096000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // spread_calculated_exact; source ordinals 0,1,2
    KnownCase {
        name: "spread_calculated_exact",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4094000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4095000000000,
                phase: 0,
                divisor: 0x3fc0410410410410,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(4094000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(4095000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(4096000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4096000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // spread_calculated_above; source ordinals 0,1,2
    KnownCase {
        name: "spread_calculated_above",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4094000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4095000000000,
                phase: 0,
                divisor: 0x3fc0410410410411,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(4094000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(4095000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(4096000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4096000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // report_spread_failure_then_neighbor; source ordinals 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18
    KnownCase {
        name: "report_spread_failure_then_neighbor",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 2000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 3000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 5000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 6000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 7000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 8000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 9000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 10000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 11000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 12000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 13000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 14000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 15000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 16000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4096000000000,
                phase: 0,
                divisor: 0x4030000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 17000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(2000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 2000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(3000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 3000000000,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4],
                reference: Some(4000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4000000000,
                    frame: 23,
                    count: 5,
                    dof: 3,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5],
                reference: Some(5000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 5000000000,
                    frame: 23,
                    count: 6,
                    dof: 4,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6],
                reference: Some(6000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 6000000000,
                    frame: 23,
                    count: 7,
                    dof: 5,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7],
                reference: Some(7000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 7000000000,
                    frame: 23,
                    count: 8,
                    dof: 6,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
                reference: Some(8000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 8000000000,
                    frame: 23,
                    count: 9,
                    dof: 7,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                reference: Some(9000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 9000000000,
                    frame: 23,
                    count: 10,
                    dof: 8,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                reference: Some(10000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 10000000000,
                    frame: 23,
                    count: 11,
                    dof: 9,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                reference: Some(11000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 11000000000,
                    frame: 23,
                    count: 12,
                    dof: 10,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                reference: Some(12000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 12000000000,
                    frame: 23,
                    count: 13,
                    dof: 11,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                reference: Some(13000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 13000000000,
                    frame: 23,
                    count: 14,
                    dof: 12,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                reference: Some(14000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 14000000000,
                    frame: 23,
                    count: 15,
                    dof: 13,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                reference: Some(15000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 15000000000,
                    frame: 23,
                    count: 16,
                    dof: 14,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(16000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 16000000000,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Spread,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                reference: Some(16000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 16000000000,
                    frame: 23,
                    count: 17,
                    dof: 15,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18],
                reference: Some(17000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 17000000000,
                    frame: 23,
                    count: 18,
                    dof: 16,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
    // changing_data_same_frame_report; source ordinals 0,1,2,3
    KnownCase {
        name: "changing_data_same_frame_report",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 2000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 3000000000,
                phase: 0,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 4000000000,
                phase: 5368709120,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(2000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(3000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 3000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2, 3],
                reference: Some(4000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 4000000000,
                    frame: 23,
                    count: 4,
                    dof: 2,
                    bits: [
                        0x3fd8000000000000,
                        0x3fc0000000000000,
                        0x3fc4000000000000,
                        0x3fb4000000000000,
                        0x3fb4000000000000,
                    ],
                }),
            },
        ],
    },
    // same_frame_negative_quarter; source ordinals 0,1,2
    KnownCase {
        name: "same_frame_negative_quarter",
        mode: 0,
        rows: &[
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 0,
                phase: -2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 1000000000,
                phase: -2147483648,
                divisor: 0x3ff0000000000000,
            },
            Raw {
                context: ContextKey {
                    source: 17,
                    frame: 23,
                    generation: 29,
                },
                time: 2000000000,
                phase: -2147483648,
                divisor: 0x3ff0000000000000,
            },
        ],
        steps: &[
            KnownStep {
                kind: Kind::Ok,
                retained: &[0],
                reference: Some(0),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1],
                reference: Some(1000000000),
                report: None,
            },
            KnownStep {
                kind: Kind::Ok,
                retained: &[0, 1, 2],
                reference: Some(2000000000),
                report: Some(ReportExpected {
                    context: ContextKey {
                        source: 17,
                        frame: 23,
                        generation: 29,
                    },
                    reference: 2000000000,
                    frame: 23,
                    count: 3,
                    dof: 1,
                    bits: [
                        0xbfd0000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                        0x0000000000000000,
                    ],
                }),
            },
        ],
    },
];
