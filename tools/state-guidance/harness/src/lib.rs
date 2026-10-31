// SPDX-License-Identifier: GPL-2.0-only
//! Bounded raw action traces reaching the real unauthenticated fixture owner.
//!
//! The independent scalar model follows the S1.1d lifecycle contract. It does
//! not call a product reducer, parser or arithmetic helper to obtain expected
//! behavior. Public constructors still admit every product input. All clocks,
//! identities and packets here are inert fixture assertions, never authority.
//! Input version 1 is followed by 1..=32 `(opcode, parameter)` byte pairs.
#![forbid(unsafe_code)]

use std::{
    fmt,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    num::NonZeroU64,
};

use ntp_exchange::{
    ArmError, Association, AssociationContext, AssociationGeneration, Binding, CancelError,
    ClockGeneration, ConfigurationGeneration, ContextError, ExpireError, FixtureEvent, Generations,
    PeerEraContext, ReceiptFacts, ReceiveError, RequestId, RequestTiming, SocketId, SourceId,
    WireToken,
};
use ntp_wire::FIXED_HEADER_LEN;
use time_domain::{EraWindow, MonotonicNs, UtcCoordinateQ32};

/// Actual version byte; unsupported versions are rejected before SUT construction.
pub const TRACE_VERSION: u8 = 1;
/// Maximum real association calls in one independently reset trace.
pub const MAX_STEPS: usize = 32;
/// One version byte plus exactly two bytes per admitted command.
pub const MAX_TRACE_BYTES: usize = 1 + 2 * MAX_STEPS;
/// Exact bitmap size for bounded, deliberately coarsened semantic observations.
pub const SEMANTIC_FEATURE_COUNT: usize = 512;

/// Known raw traces for harness controls; their expected facts are in the tests.
/// A campaign chooses its declared starting corpus separately from these controls.
pub const CONTROL_TRACES: &[&[u8]] = &[
    &[1, 0, 0xf0, 1, 0],
    &[1, 0, 0xf0, 2, 0, 1, 0],
    &[1, 0, 0xf0, 1, 0, 12, 0],
    &[1, 0, 0xf0, 8, 0, 1, 0],
    &[1, 0, 0xf0, 4, 0, 0, 0xf0, 5, 0, 1, 0],
    &[1, 0, 0xf0, 6, 0, 7, 0, 1, 0],
    &[1, 0, 0xf0, 0, 0xf1, 1, 0],
    &[1, 0, 0xf0, 14, 0, 12, 0],
];

/// Closed actual/expected result classes; numeric codes only define feedback slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Outcome {
    /// Request allocated.
    Armed,
    /// Existing request preserved by rejected rearm.
    Busy,
    /// One complete unauthenticated time observation committed.
    Time,
    /// One opaque control committed.
    Control,
    /// A framed response arrived without a pending request.
    ReceiveNoPending,
    /// Nonzero origin differs from the retained wire token.
    OriginMismatch,
    /// Socket binding differs.
    SocketMismatch,
    /// T4 mapping differs from the current clock generation.
    MappingMismatch,
    /// Processing reached the exclusive deadline.
    DeadlineReached,
    /// Retained arrival predates the current send.
    ArrivalBeforeSend,
    /// Full-datagram framing rejected before association guards.
    Framing,
    /// Exactly the requested pending identity was cancelled.
    Cancelled,
    /// Cancellation has no pending request.
    CancelNoPending,
    /// Stale cancellation cannot consume the pending successor.
    CancelMismatch,
    /// Deadline expiry retired the pending request.
    Expired,
    /// Expiry has no pending request.
    ExpireNoPending,
    /// Early expiry preserves the request.
    NotExpired,
    /// A forward context update committed.
    Advanced,
    /// Configuration regression rejected.
    ConfigurationRegression,
    /// Association regression rejected.
    AssociationRegression,
    /// Clock regression rejected.
    ClockRegression,
    /// An unchanged generation tuple rejected.
    NoGenerationAdvance,
    /// A binding change without association advance rejected.
    BindingWithoutAdvance,
    /// Processing predates the asserted arrival.
    ProcessingBeforeArrival,
    /// Zero origin rejected before token comparison.
    ZeroOrigin,
}

/// Input rejection is distinct from reached model or product disagreement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceInputError {
    /// A trace needs one version and at least one complete action.
    Empty,
    /// The complete trace exceeds the fixed bound.
    TooLong,
    /// Version does not name the declared format.
    Version,
    /// A final partial command would lose input identity.
    PartialCommand,
    /// Opcodes outside 0..=15 are not normalized into valid actions.
    Opcode,
}

/// All failures are bounded and contain no packet, token, key or host data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceFailure {
    /// Complete input preflight failed; no association call occurred.
    Malformed(TraceInputError),
    /// A fixed valid fixture failed public construction or bounded bookkeeping.
    Setup,
    /// The actual returned category differs from the independent transition table.
    OracleMismatch {
        /// Zero-based actual action ordinal.
        step: usize,
        /// Independently predicted category.
        expected: Outcome,
        /// Returned product category.
        actual: Outcome,
    },
    /// Pending, sequence or current-context state disagrees after an action.
    StateMismatch {
        /// Zero-based actual action ordinal.
        step: usize,
    },
    /// A committed observation has different identity, times or exact arithmetic.
    CompletionMismatch {
        /// Zero-based actual action ordinal.
        step: usize,
    },
    /// A result outside this trace domain was observed and must not be ignored.
    UnexpectedOutcome {
        /// Zero-based actual action ordinal.
        step: usize,
    },
}

impl TraceFailure {
    /// Distinguish bounded invalid raw input from a reached verification failure.
    #[must_use]
    pub const fn is_malformed(self) -> bool {
        matches!(self, Self::Malformed(_))
    }
}

impl fmt::Display for TraceFailure {
    /// Display only the bounded typed failure and action ordinal.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "state trace: {self:?}")
    }
}

impl std::error::Error for TraceFailure {}

/// Immutable observations derive only from reached actual results and getters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceObservation {
    features: [u8; SEMANTIC_FEATURE_COUNT],
    outcomes: [usize; 32],
    steps: usize,
    completions: usize,
    final_pending: Option<u64>,
}

impl TraceObservation {
    /// Observed outcome/pending/generation-parity bitmap, reset per input.
    /// Generation parity is deliberate coarsening, not an exact-state proof.
    #[must_use]
    pub const fn semantic_features(&self) -> &[u8; SEMANTIC_FEATURE_COUNT] {
        &self.features
    }

    /// Number of actual mutating association calls, never merely parsed commands.
    #[must_use]
    pub const fn step_count(&self) -> usize {
        self.steps
    }

    /// Count the selected actual result without exposing mutable observer state.
    #[must_use]
    pub fn outcome_count(&self, outcome: Outcome) -> usize {
        self.outcomes[usize::from(outcome as u8)]
    }

    /// Time/control completions accepted by the actual owner.
    #[must_use]
    pub const fn completions(&self) -> usize {
        self.completions
    }

    /// Actual retained request identity after the final checked operation.
    #[must_use]
    pub const fn final_pending(&self) -> Option<u64> {
        self.final_pending
    }
}

/// One returned-category difference whose actual effects still match the model.
/// Values are diagnostic only; predicted categories never enter target feedback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutcomeDifference {
    step: usize,
    expected: Outcome,
    actual: Outcome,
}

impl OutcomeDifference {
    /// Unused fixed-capacity slots are never exposed by the checked prefix.
    const EMPTY: Self = Self {
        step: 0,
        expected: Outcome::Armed,
        actual: Outcome::Armed,
    };

    /// Zero-based ordinal of the actual action with equivalent checked effects.
    #[must_use]
    pub const fn step(self) -> usize {
        self.step
    }

    /// Independently expected category, retained only for diagnosis.
    #[must_use]
    pub const fn expected(self) -> Outcome {
        self.expected
    }

    /// Actual returned category, also observed through the ordinary feedback map.
    #[must_use]
    pub const fn actual(self) -> Outcome {
        self.actual
    }
}

/// Actual trace observations plus at most one effect-equivalent difference/action.
/// The private prefix length cannot exceed the fixed trace-action bound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectTraceObservation {
    observation: TraceObservation,
    differences: [OutcomeDifference; MAX_STEPS],
    difference_count: usize,
}

impl EffectTraceObservation {
    /// Reached actual feedback, categories, calls and accepted completions.
    #[must_use]
    pub const fn observation(&self) -> &TraceObservation {
        &self.observation
    }

    /// Checked-effect-equivalent category differences in actual action order.
    #[must_use]
    pub fn outcome_differences(&self) -> &[OutcomeDifference] {
        &self.differences[..self.difference_count]
    }

    /// Count of recorded differences, bounded by the admitted trace length.
    #[must_use]
    pub const fn outcome_difference_count(&self) -> usize {
        self.difference_count
    }

    /// Start one independent trace with no retained target feedback/diagnostics.
    const fn new() -> Self {
        Self {
            observation: TraceObservation {
                features: [0; SEMANTIC_FEATURE_COUNT],
                outcomes: [0; 32],
                steps: 0,
                completions: 0,
                final_pending: None,
            },
            differences: [OutcomeDifference::EMPTY; MAX_STEPS],
            difference_count: 0,
        }
    }

    /// Retain one verified equivalent difference; overflow is a harness failure.
    fn record_difference(&mut self, difference: OutcomeDifference) -> Result<(), TraceFailure> {
        let slot = self
            .differences
            .get_mut(self.difference_count)
            .ok_or(TraceFailure::Setup)?;
        *slot = difference;
        self.difference_count += 1;
        Ok(())
    }
}

/// Effect-lane failure classification, separate from strict category equality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectFailureKind {
    /// Strict raw trace admission failed before a product operation.
    MalformedInput,
    /// Valid fixed fixture construction or bounded bookkeeping failed.
    HarnessSetup,
    /// An action rejected by the model returned acceptance or a completion.
    ForbiddenAcceptance,
    /// The model required a time/control completion but none was returned.
    MissingRequiredCompletion,
    /// An actual completion has the wrong kind, identity, time or numerical facts.
    CompletionCorruption,
    /// Actual pending/next/context state or a returned lifecycle identity differs.
    LifecycleStateCorruption,
    /// The actual result lies outside this explicitly modelled fixture domain.
    UnexpectedOutcome,
}

/// Fail-fast effect diagnosis with the reached actual prefix and earlier differences.
/// A boxed, fixed-capacity prefix bounds retained evidence and keeps errors small.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectTraceFailure {
    kind: EffectFailureKind,
    cause: TraceFailure,
    attempted_steps: usize,
    observation: Box<EffectTraceObservation>,
}

impl EffectTraceFailure {
    /// Independent effect classification; a tolerated category difference is absent.
    #[must_use]
    pub const fn kind(&self) -> EffectFailureKind {
        self.kind
    }

    /// Bounded underlying mismatch or raw-admission/setup cause.
    #[must_use]
    pub const fn cause(&self) -> TraceFailure {
        self.cause
    }

    /// Failing actual action ordinal, or absence for whole-input admission/setup.
    #[must_use]
    pub const fn step(&self) -> Option<usize> {
        match self.cause {
            TraceFailure::OracleMismatch { step, .. }
            | TraceFailure::StateMismatch { step }
            | TraceFailure::CompletionMismatch { step }
            | TraceFailure::UnexpectedOutcome { step } => Some(step),
            TraceFailure::Malformed(_) | TraceFailure::Setup => None,
        }
    }

    /// Calls attempted through the failing ordinal; admission failures report zero.
    /// An unclassifiable result can be absent from the classified observation prefix.
    #[must_use]
    pub const fn attempted_steps(&self) -> usize {
        self.attempted_steps
    }

    /// Actual classified actions, including a classified failing action, and the
    /// earlier effect-equivalent diagnostics. No unexecuted action is fabricated.
    #[must_use]
    pub fn partial_observation(&self) -> &EffectTraceObservation {
        &self.observation
    }

    /// Attach the owned bounded prefix at the one fail-fast execution boundary.
    fn new(
        kind: EffectFailureKind,
        cause: TraceFailure,
        attempted_steps: usize,
        observation: EffectTraceObservation,
    ) -> Self {
        Self {
            kind,
            cause,
            attempted_steps,
            observation: Box::new(observation),
        }
    }
}

impl fmt::Display for EffectTraceFailure {
    /// Display only bounded effect class and the existing redacted typed cause.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "effect trace {:?}: {}", self.kind, self.cause)
    }
}

impl std::error::Error for EffectTraceFailure {}

/// Privately admitted two-byte instruction; all 16 opcodes have real SUT calls.
#[derive(Clone, Copy)]
struct Command {
    opcode: u8,
    parameter: u8,
}

/// Fully admitted fixed-capacity script; unused slots can never be executed.
struct Script {
    commands: [Command; MAX_STEPS],
    len: usize,
}

/// Admit every framing/opcode byte before constructing or mutating the SUT.
fn parse(data: &[u8]) -> Result<Script, TraceFailure> {
    let malformed = TraceFailure::Malformed;
    if data.len() < 3 {
        return Err(malformed(TraceInputError::Empty));
    }
    if data.len() > MAX_TRACE_BYTES {
        return Err(malformed(TraceInputError::TooLong));
    }
    if data.first() != Some(&TRACE_VERSION) {
        return Err(malformed(TraceInputError::Version));
    }
    let payload = data.get(1..).ok_or(malformed(TraceInputError::Empty))?;
    let (pairs, remainder) = payload.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(malformed(TraceInputError::PartialCommand));
    }
    let mut script = Script {
        commands: [Command {
            opcode: 0,
            parameter: 0,
        }; MAX_STEPS],
        len: pairs.len(),
    };
    for (index, pair) in pairs.iter().enumerate() {
        if pair[0] > 15 {
            return Err(malformed(TraceInputError::Opcode));
        }
        script.commands[index] = Command {
            opcode: pair[0],
            parameter: pair[1],
        };
    }
    Ok(script)
}

/// Independent scalar context; no product reducer computes its transitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Context {
    source: u64,
    socket: u64,
    endpoint: SocketAddr,
    configuration: u64,
    association: u64,
    clock: u64,
}

impl Context {
    /// Fixed documentation-address fixture context, with no host identity claim.
    const fn initial() -> Self {
        Self {
            source: 1,
            socket: 1,
            endpoint: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 0, 2, 1), 123)),
            configuration: 1,
            association: 1,
            clock: 1,
        }
    }

    /// Admit scalar fixture inputs through the real public typed constructors.
    fn typed(self) -> Result<AssociationContext, TraceFailure> {
        let socket = NonZeroU64::new(self.socket).ok_or(TraceFailure::Setup)?;
        let source = NonZeroU64::new(self.source).ok_or(TraceFailure::Setup)?;
        let binding = Binding::new(SourceId::new(source), SocketId::new(socket), self.endpoint);
        Ok(AssociationContext::new(
            binding,
            Generations::new(
                ConfigurationGeneration::new(self.configuration),
                AssociationGeneration::new(self.association),
                ClockGeneration::new(self.clock),
            ),
        ))
    }

    /// Read only actual retained context fields for state comparison/feedback.
    fn observed(value: AssociationContext) -> Self {
        Self {
            source: value.binding().source_id().value().get(),
            socket: value.binding().socket_id().value().get(),
            endpoint: value.binding().endpoint(),
            configuration: value.generations().configuration().value(),
            association: value.generations().association().value(),
            clock: value.generations().clock().value(),
        }
    }
}

/// Plain request facts retained by the independent model, not a completed event.
#[derive(Clone, Copy)]
struct Pending {
    id: u64,
    token: u64,
    send: u64,
    deadline: u64,
    context: Context,
}

/// Inert produced packet/receipt facts; raw forwarding is checked by the oracle.
#[derive(Clone, Copy)]
struct Capture {
    token: u64,
    socket: u64,
    mapping: u64,
    arrival: u64,
    processing: u64,
    malformed: bool,
    control: bool,
}

/// Explicit independent lifecycle state; no product function supplies decisions.
struct Model {
    context: Context,
    pending: Option<Pending>,
    next: u64,
    last_capture: Option<Capture>,
}

/// Produce exact canonical era-zero timestamp fixture values independently.
fn coordinate(value: i128) -> Result<UtcCoordinateQ32, TraceFailure> {
    UtcCoordinateQ32::new(value).map_err(|_| TraceFailure::Setup)
}

/// Check the fixed one-era interval through the public domain constructor.
fn eras() -> Result<PeerEraContext, TraceFailure> {
    let window = EraWindow::new(0, i128::from(u64::MAX)).map_err(|_| TraceFailure::Setup)?;
    Ok(PeerEraContext::new(window, window))
}

/// Preserve a bounded positive request identity through its private constructor.
fn request(value: u64) -> Result<RequestId, TraceFailure> {
    NonZeroU64::new(value)
        .map(RequestId::new)
        .ok_or(TraceFailure::Setup)
}

/// Define a raw canonical server header without using the production formatter.
fn packet(capture: Capture) -> [u8; FIXED_HEADER_LEN] {
    let mut bytes = [0; FIXED_HEADER_LEN];
    bytes[0] = 0x24;
    bytes[1] = if capture.control { 0 } else { 2 };
    bytes[2] = 6;
    bytes[3] = 0xec;
    bytes[24..32].copy_from_slice(&capture.token.to_be_bytes());
    bytes[32..40].copy_from_slice(&0x0000_0001_4000_0000_u64.to_be_bytes());
    bytes[40..48].copy_from_slice(&0x0000_0001_8000_0000_u64.to_be_bytes());
    bytes
}

/// Translate inert supplied receipt facts without asserting product acceptance.
fn receipt(capture: Capture) -> Result<ReceiptFacts, TraceFailure> {
    let mut context = Context::initial();
    context.socket = capture.socket;
    Ok(ReceiptFacts::new(
        context.typed()?.binding(),
        ClockGeneration::new(capture.mapping),
        MonotonicNs::new(capture.arrival),
        MonotonicNs::new(capture.processing),
        coordinate(0x0000_0001_c000_0000)?,
    ))
}

/// Generate a legal raw receipt from the action; variants change only named facts.
fn capture(model: &Model, command: Command, now: u64) -> Capture {
    if command.opcode == 12 {
        if let Some(mut old) = model.last_capture {
            old.processing = now + u64::from(command.parameter >> 2);
            return old;
        }
        // Missing retained bytes are an explicit zero-origin attempt, not an
        // invented successful replay or a silently skipped receive operation.
        return Capture {
            token: 0,
            socket: model.context.socket,
            mapping: model.context.clock,
            arrival: now,
            processing: now,
            malformed: false,
            control: false,
        };
    }
    let mut result = Capture {
        token: model.pending.map_or(1, |pending| pending.token),
        socket: model.context.socket,
        mapping: model.context.clock,
        arrival: now + u64::from(command.parameter & 3),
        processing: now + u64::from(command.parameter >> 2),
        malformed: command.opcode == 13,
        control: command.opcode == 14,
    };
    if command.opcode == 2 {
        result.token = if command.parameter & 128 != 0 {
            0
        } else {
            result.token + 1
        };
    }
    if command.opcode == 3 {
        result.socket = if result.socket == 1 { 2 } else { 1 };
    }
    if command.opcode == 15 {
        result.mapping += 1;
    }
    result
}

/// Model receipt admission using only declared scalar fixture facts and rules.
fn expected_receive(model: &mut Model, input: Capture) -> Outcome {
    if input.malformed {
        return Outcome::Framing;
    }
    let Some(pending) = model.pending else {
        return Outcome::ReceiveNoPending;
    };
    if input.socket != pending.context.socket {
        return Outcome::SocketMismatch;
    }
    if input.mapping != model.context.clock {
        return Outcome::MappingMismatch;
    }
    if input.token == 0 {
        return Outcome::ZeroOrigin;
    }
    if input.token != pending.token {
        return Outcome::OriginMismatch;
    }
    if input.arrival < pending.send {
        return Outcome::ArrivalBeforeSend;
    }
    if input.processing < input.arrival {
        return Outcome::ProcessingBeforeArrival;
    }
    if input.processing >= pending.deadline {
        return Outcome::DeadlineReached;
    }
    model.pending = None;
    if input.control {
        Outcome::Control
    } else {
        Outcome::Time
    }
}

/// Classify only reached actual responses; uncovered categories are never skipped.
fn actual_receive(
    result: Result<FixtureEvent, ReceiveError>,
    pending: Option<Pending>,
    input: Capture,
    step: usize,
) -> Result<Outcome, TraceFailure> {
    match result {
        Ok(event) => {
            let pending = pending.ok_or(TraceFailure::CompletionMismatch { step })?;
            let facts = match &event {
                FixtureEvent::Time(time) => time.facts(),
                FixtureEvent::Control(control) => control.facts(),
            };
            if facts.request_id().value().get() != pending.id
                || Context::observed(AssociationContext::new(
                    facts.binding(),
                    facts.generations(),
                )) != pending.context
                || facts.send().nanoseconds() != pending.send
                || facts.deadline().nanoseconds() != pending.deadline
                || facts.arrival().nanoseconds() != input.arrival
                || facts.processing_now().nanoseconds() != input.processing
                || facts.mapping_generation().value() != input.mapping
            {
                return Err(TraceFailure::CompletionMismatch { step });
            }
            match event {
                FixtureEvent::Time(time) => {
                    if time.saved_real_t1().ticks() != 0x0000_0001_0000_0000
                        || time.peer_t2().ticks() != 0x0000_0001_4000_0000
                        || time.peer_t3().ticks() != 0x0000_0001_8000_0000
                        || time.local_t4().ticks() != 0x0000_0001_c000_0000
                        || time.stratum() != 2
                        || time.leap_indicator() != 0
                        || time.offset().numerator() != 0
                        || time.signed_round_trip().numerator() != 0x8000_0000
                    {
                        return Err(TraceFailure::CompletionMismatch { step });
                    }
                    Ok(Outcome::Time)
                }
                FixtureEvent::Control(_) => Ok(Outcome::Control),
            }
        }
        Err(error) => Ok(match error {
            ReceiveError::Framing(_) => Outcome::Framing,
            ReceiveError::NoPending => Outcome::ReceiveNoPending,
            ReceiveError::OriginMismatch => Outcome::OriginMismatch,
            ReceiveError::ZeroOrigin => Outcome::ZeroOrigin,
            ReceiveError::SocketMismatch => Outcome::SocketMismatch,
            ReceiveError::MappingGenerationMismatch => Outcome::MappingMismatch,
            ReceiveError::DeadlineReached => Outcome::DeadlineReached,
            ReceiveError::ArrivalBeforeSend => Outcome::ArrivalBeforeSend,
            ReceiveError::ProcessingBeforeArrival => Outcome::ProcessingBeforeArrival,
            _ => return Err(TraceFailure::UnexpectedOutcome { step }),
        }),
    }
}

/// Compute one explicit context candidate from the trace, before either owner acts.
fn next_context(current: Context, command: Command) -> Context {
    let mut next = current;
    let advance = 1 + u64::from(command.parameter & 3);
    match command.opcode {
        8 => next.configuration += advance,
        9 => {
            next.association += advance;
            next.socket = if next.socket == 1 { 2 } else { 1 };
        }
        10 => next.clock += advance,
        11 => match command.parameter % 5 {
            0 => next.configuration -= 1,
            1 => next.association -= 1,
            2 => next.clock -= 1,
            3 => {}
            _ => {
                next.configuration += 1;
                next.socket = if next.socket == 1 { 2 } else { 1 };
            }
        },
        _ => {}
    }
    next
}

/// Independently decide admission of the supplied context, preserving rejection.
fn expected_context(model: &mut Model, next: Context) -> Outcome {
    let previous = model.context;
    let outcome = if next.configuration < previous.configuration {
        Outcome::ConfigurationRegression
    } else if next.association < previous.association {
        Outcome::AssociationRegression
    } else if next.clock < previous.clock {
        Outcome::ClockRegression
    } else if (next.configuration, next.association, next.clock)
        == (previous.configuration, previous.association, previous.clock)
    {
        Outcome::NoGenerationAdvance
    } else if next.socket != previous.socket && next.association == previous.association {
        Outcome::BindingWithoutAdvance
    } else {
        Outcome::Advanced
    };
    if outcome == Outcome::Advanced {
        model.context = next;
        model.pending = None;
    }
    outcome
}

/// Actual returned category and its separately modelled transition. Detail failures
/// remain attached until the strict or effect adapter applies its own ordering.
#[derive(Clone, Copy)]
struct StepObservation {
    expected: Outcome,
    actual: Outcome,
    detail_failure: Option<TraceFailure>,
}

/// Dispatch one admitted action once and evaluate the same independent model.
/// Retain actual completion categories even when completion facts are corrupt;
/// unknown categories and setup failures still stop without invented feedback.
fn observe_step(
    owner: &mut Association,
    model: &mut Model,
    command: Command,
    index: usize,
) -> Result<StepObservation, TraceFailure> {
    let now = 1_000 + u64::try_from(index).map_err(|_| TraceFailure::Setup)? * 10;
    let mut detail_failure = None;
    let (expected, actual) = match command.opcode {
        0 => {
            let token = 1 + u64::from(command.parameter & 15);
            let deadline = now + 20 + u64::from(command.parameter >> 4) * 10;
            let expected = if model.pending.is_some() {
                Outcome::Busy
            } else {
                Outcome::Armed
            };
            let result = owner.arm(
                coordinate(0x0000_0001_0000_0000)?,
                WireToken::new(token).map_err(|_| TraceFailure::Setup)?,
                RequestTiming::new(MonotonicNs::new(now), MonotonicNs::new(deadline))
                    .map_err(|_| TraceFailure::Setup)?,
                eras()?,
            );
            let actual = match result {
                Ok(id) => {
                    if id.value().get() != model.next {
                        detail_failure = Some(TraceFailure::StateMismatch { step: index });
                    }
                    Outcome::Armed
                }
                Err(ArmError::Busy) => Outcome::Busy,
                Err(_) => return Err(TraceFailure::UnexpectedOutcome { step: index }),
            };
            if expected == Outcome::Armed {
                model.pending = Some(Pending {
                    id: model.next,
                    token,
                    send: now,
                    deadline,
                    context: model.context,
                });
                model.next += 1;
            }
            (expected, actual)
        }
        4 | 5 => {
            let old = model.pending;
            let id = old.map_or(1, |p| {
                if command.opcode == 4 {
                    p.id
                } else if p.id == 1 {
                    2
                } else {
                    p.id - 1
                }
            });
            let expected = if old.is_none() {
                Outcome::CancelNoPending
            } else if command.opcode == 5 {
                Outcome::CancelMismatch
            } else {
                Outcome::Cancelled
            };
            let actual = match owner.cancel(request(id)?) {
                Ok(retired) => {
                    if Some(retired.value().get()) != old.map(|p| p.id) {
                        detail_failure = Some(TraceFailure::StateMismatch { step: index });
                    }
                    Outcome::Cancelled
                }
                Err(CancelError::NoPending) => Outcome::CancelNoPending,
                Err(CancelError::RequestMismatch) => Outcome::CancelMismatch,
            };
            if expected == Outcome::Cancelled {
                model.pending = None;
            }
            (expected, actual)
        }
        6 | 7 => {
            let old = model.pending;
            let at = old.map_or(now, |p| {
                if command.opcode == 6 {
                    p.deadline - 1
                } else {
                    p.deadline
                }
            });
            let expected = if old.is_none() {
                Outcome::ExpireNoPending
            } else if command.opcode == 6 {
                Outcome::NotExpired
            } else {
                Outcome::Expired
            };
            let actual = match owner.expire(MonotonicNs::new(at)) {
                Ok(retired) => {
                    if Some(retired.value().get()) != old.map(|p| p.id) {
                        detail_failure = Some(TraceFailure::StateMismatch { step: index });
                    }
                    Outcome::Expired
                }
                Err(ExpireError::NoPending) => Outcome::ExpireNoPending,
                Err(ExpireError::NotExpired) => Outcome::NotExpired,
            };
            if expected == Outcome::Expired {
                model.pending = None;
            }
            (expected, actual)
        }
        8..=11 => {
            let next = next_context(model.context, command);
            let pending = model.pending;
            let expected = expected_context(model, next);
            let actual = match owner.advance_context(next.typed()?) {
                Ok(retired) => {
                    if retired.map(|id| id.value().get()) != pending.map(|p| p.id) {
                        detail_failure = Some(TraceFailure::StateMismatch { step: index });
                    }
                    Outcome::Advanced
                }
                Err(error) => match error {
                    ContextError::ConfigurationRegression => Outcome::ConfigurationRegression,
                    ContextError::AssociationRegression => Outcome::AssociationRegression,
                    ContextError::ClockRegression => Outcome::ClockRegression,
                    ContextError::NoGenerationAdvance => Outcome::NoGenerationAdvance,
                    ContextError::BindingChangedWithoutAssociationAdvance => {
                        Outcome::BindingWithoutAdvance
                    }
                },
            };
            (expected, actual)
        }
        _ => {
            let input = capture(model, command, now);
            if command.opcode != 12 {
                model.last_capture = Some(input);
            }
            let old = model.pending;
            let expected = expected_receive(model, input);
            let bytes = packet(input);
            let length = if input.malformed {
                47
            } else {
                FIXED_HEADER_LEN
            };
            let result = owner.receive(&bytes[..length], receipt(input)?);
            let completion = match &result {
                Ok(FixtureEvent::Time(_)) => Some(Outcome::Time),
                Ok(FixtureEvent::Control(_)) => Some(Outcome::Control),
                Err(_) => None,
            };
            let actual = match actual_receive(result, old, input, index) {
                Ok(outcome) => outcome,
                Err(failure @ TraceFailure::CompletionMismatch { .. }) => {
                    detail_failure = Some(failure);
                    completion.ok_or(failure)?
                }
                Err(failure) => return Err(failure),
            };
            (expected, actual)
        }
    };
    Ok(StepObservation {
        expected,
        actual,
        detail_failure,
    })
}

/// Preserve the strict lane's existing detail-failure precedence and outcome pair.
/// Its caller continues to reject every actual/expected category difference.
fn step(
    owner: &mut Association,
    model: &mut Model,
    command: Command,
    index: usize,
) -> Result<(Outcome, Outcome), TraceFailure> {
    let observed = observe_step(owner, model, command, index)?;
    if let Some(failure) = observed.detail_failure {
        return Err(failure);
    }
    Ok((observed.expected, observed.actual))
}

/// Run one completely admitted trace from fresh product/model/feedback state.
/// Every successful step compares actual return, pending/next/context and accepted
/// completion facts; malformed inputs perform no SUT operations. No input performs
/// I/O, host-clock reads/writes, network work or real authentication.
/// # Errors
/// Distinguishes malformed bytes, fixture setup, and reached oracle disagreement.
pub fn execute_trace(data: &[u8]) -> Result<TraceObservation, TraceFailure> {
    let script = parse(data)?;
    let context = Context::initial();
    let mut owner = Association::new_plain_header_fixture(context.typed()?, request(1)?);
    let mut model = Model {
        context,
        pending: None,
        next: 1,
        last_capture: None,
    };
    let mut observation = TraceObservation {
        features: [0; SEMANTIC_FEATURE_COUNT],
        outcomes: [0; 32],
        steps: 0,
        completions: 0,
        final_pending: None,
    };
    for (index, command) in script.commands.iter().take(script.len).copied().enumerate() {
        let (expected, actual) = step(&mut owner, &mut model, command, index)?;
        if expected != actual {
            return Err(TraceFailure::OracleMismatch {
                step: index,
                expected,
                actual,
            });
        }
        let pending = owner.pending_request_id().map(|id| id.value().get());
        let observed_context = Context::observed(owner.current_context());
        if pending != model.pending.map(|p| p.id)
            || owner.next_request_id().map(|id| id.value().get()) != Some(model.next)
            || observed_context != model.context
        {
            return Err(TraceFailure::StateMismatch { step: index });
        }
        let state = usize::from(observed_context.configuration % 2 != 0) * 4
            + usize::from(observed_context.association % 2 != 0) * 2
            + usize::from(observed_context.clock % 2 != 0);
        let feature = usize::from(actual as u8) * 16 + usize::from(pending.is_some()) * 8 + state;
        observation.features[feature] = 1;
        observation.outcomes[usize::from(actual as u8)] += 1;
        observation.steps += 1;
        if matches!(actual, Outcome::Time | Outcome::Control) {
            observation.completions += 1;
        }
        observation.final_pending = pending;
    }
    Ok(observation)
}

/// Classify an actual/expected category without treating it as target feedback.
const fn is_completion(outcome: Outcome) -> bool {
    matches!(outcome, Outcome::Time | Outcome::Control)
}

/// Success categories acknowledge acceptance; all other modelled categories reject.
const fn is_acceptance(outcome: Outcome) -> bool {
    matches!(
        outcome,
        Outcome::Armed
            | Outcome::Time
            | Outcome::Control
            | Outcome::Cancelled
            | Outcome::Expired
            | Outcome::Advanced
    )
}

/// Record only a reached actual result and actual public state getters. At most
/// MAX_STEPS admitted actions and closed outcome codes bound every index/count.
fn record_actual_step(
    observation: &mut TraceObservation,
    actual: Outcome,
    pending: Option<u64>,
    context: Context,
) {
    let state = usize::from(context.configuration % 2 != 0) * 4
        + usize::from(context.association % 2 != 0) * 2
        + usize::from(context.clock % 2 != 0);
    let feature = usize::from(actual as u8) * 16 + usize::from(pending.is_some()) * 8 + state;
    observation.features[feature] = 1;
    observation.outcomes[usize::from(actual as u8)] += 1;
    observation.steps += 1;
    observation.completions += usize::from(is_completion(actual));
    observation.final_pending = pending;
}

/// Check effects before permitting a returned-category diagnostic to continue.
/// Actual state must match the same independent model after this exact action.
/// Public getters expose pending identity, next identity and context; pending
/// token/timing metadata is checked at completion and through later real actions.
fn check_effect_step(
    owner: &Association,
    model: &Model,
    observed: StepObservation,
    index: usize,
    observation: &mut EffectTraceObservation,
) -> Result<(), (EffectFailureKind, TraceFailure)> {
    let StepObservation {
        expected,
        actual,
        detail_failure,
    } = observed;
    let pending = owner.pending_request_id().map(|id| id.value().get());
    let context = Context::observed(owner.current_context());
    record_actual_step(&mut observation.observation, actual, pending, context);
    let outcome_failure = TraceFailure::OracleMismatch {
        step: index,
        expected,
        actual,
    };
    if is_completion(actual) && !is_completion(expected) {
        return Err((EffectFailureKind::ForbiddenAcceptance, outcome_failure));
    }
    if is_completion(expected) && !is_completion(actual) {
        return Err((
            EffectFailureKind::MissingRequiredCompletion,
            outcome_failure,
        ));
    }
    if is_completion(actual) && actual != expected {
        return Err((EffectFailureKind::CompletionCorruption, outcome_failure));
    }
    if let Some(failure) = detail_failure {
        let kind = match failure {
            TraceFailure::CompletionMismatch { .. } => EffectFailureKind::CompletionCorruption,
            TraceFailure::StateMismatch { .. } => EffectFailureKind::LifecycleStateCorruption,
            _ => EffectFailureKind::HarnessSetup,
        };
        return Err((kind, failure));
    }
    if is_acceptance(actual) && !is_acceptance(expected) {
        return Err((EffectFailureKind::ForbiddenAcceptance, outcome_failure));
    }
    if pending != model.pending.map(|p| p.id)
        || owner.next_request_id().map(|id| id.value().get()) != Some(model.next)
        || context != model.context
    {
        return Err((
            EffectFailureKind::LifecycleStateCorruption,
            TraceFailure::StateMismatch { step: index },
        ));
    }
    if expected != actual {
        observation
            .record_difference(OutcomeDifference {
                step: index,
                expected,
                actual,
            })
            .map_err(|failure| (EffectFailureKind::HarnessSetup, failure))?;
    }
    Ok(())
}

/// Run the separately named effect-focused experiment from fresh target/model state.
/// Strict raw admission, real public operations, the independent model and full
/// completion-fact checks are shared with `execute_trace`, which remains strict.
/// Only returned-category differences with equal checked effects may continue;
/// every such difference is retained outside actual-target semantic feedback.
///
/// Pending identity, next identity and full context are checked after every action.
/// Hidden pending token/timing fields have no public getter: completion facts and
/// later trace actions challenge them, without claiming immediate full-state access.
/// No operation performs network, authentication, host-clock or native OS effects.
/// # Errors
/// Fails at the first malformed input, setup failure, unmodelled result or effect
/// divergence, preserving its independent class and bounded reached actual prefix.
pub fn execute_effect_trace(data: &[u8]) -> Result<EffectTraceObservation, EffectTraceFailure> {
    let mut observation = EffectTraceObservation::new();
    let script = parse(data).map_err(|failure| {
        let kind = if failure.is_malformed() {
            EffectFailureKind::MalformedInput
        } else {
            EffectFailureKind::HarnessSetup
        };
        EffectTraceFailure::new(kind, failure, 0, observation.clone())
    })?;
    let context = Context::initial();
    let (typed_context, initial_request) = match (context.typed(), request(1)) {
        (Ok(context), Ok(request)) => (context, request),
        (Err(failure), _) | (_, Err(failure)) => {
            return Err(EffectTraceFailure::new(
                EffectFailureKind::HarnessSetup,
                failure,
                0,
                observation,
            ));
        }
    };
    let mut owner = Association::new_plain_header_fixture(typed_context, initial_request);
    let mut model = Model {
        context,
        pending: None,
        next: 1,
        last_capture: None,
    };
    for (index, command) in script.commands.iter().take(script.len).copied().enumerate() {
        let observed = match observe_step(&mut owner, &mut model, command, index) {
            Ok(observed) => observed,
            Err(failure) => {
                let (kind, attempted_steps) = if failure == TraceFailure::Setup {
                    (EffectFailureKind::HarnessSetup, index)
                } else {
                    (EffectFailureKind::UnexpectedOutcome, index + 1)
                };
                return Err(EffectTraceFailure::new(
                    kind,
                    failure,
                    attempted_steps,
                    observation,
                ));
            }
        };
        if let Err((kind, cause)) =
            check_effect_step(&owner, &model, observed, index, &mut observation)
        {
            return Err(EffectTraceFailure::new(kind, cause, index + 1, observation));
        }
    }
    Ok(observation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_time_has_exact_independent_arithmetic() -> Result<(), TraceFailure> {
        let observed = execute_trace(&[1, 0, 0xf0, 1, 0])?;
        assert_eq!(observed.step_count(), 2);
        assert_eq!(observed.completions(), 1);
        assert_eq!(observed.final_pending(), None);
        assert_eq!(observed.outcome_count(Outcome::Time), 1);
        Ok(())
    }

    #[test]
    fn wrong_origin_preserves_pending_for_a_valid_neighbor() -> Result<(), TraceFailure> {
        let observed = execute_trace(&[1, 0, 0xf0, 2, 0, 1, 0])?;
        assert_eq!(observed.outcome_count(Outcome::OriginMismatch), 1);
        assert_eq!(observed.outcome_count(Outcome::Time), 1);
        Ok(())
    }

    #[test]
    fn completed_reply_is_not_accepted_twice() -> Result<(), TraceFailure> {
        let observed = execute_trace(&[1, 0, 0xf0, 1, 0, 12, 0])?;
        assert_eq!(observed.completions(), 1);
        assert_eq!(observed.outcome_count(Outcome::ReceiveNoPending), 1);
        Ok(())
    }

    #[test]
    fn context_advance_invalidates_pending_work() -> Result<(), TraceFailure> {
        let observed = execute_trace(&[1, 0, 0xf0, 8, 0, 1, 0])?;
        assert_eq!(observed.completions(), 0);
        assert_eq!(observed.outcome_count(Outcome::Advanced), 1);
        assert_eq!(observed.outcome_count(Outcome::ReceiveNoPending), 1);
        Ok(())
    }

    #[test]
    fn stale_cancel_cannot_retire_a_successor() -> Result<(), TraceFailure> {
        let observed = execute_trace(&[1, 0, 0xf0, 4, 0, 0, 0xf0, 5, 0, 1, 0])?;
        assert_eq!(observed.outcome_count(Outcome::CancelMismatch), 1);
        assert_eq!(observed.completions(), 1);
        Ok(())
    }

    #[test]
    fn early_and_exact_expiry_have_distinct_results() -> Result<(), TraceFailure> {
        let observed = execute_trace(&[1, 0, 0xf0, 6, 0, 7, 0, 1, 0])?;
        assert_eq!(observed.outcome_count(Outcome::NotExpired), 1);
        assert_eq!(observed.outcome_count(Outcome::Expired), 1);
        assert_eq!(observed.completions(), 0);
        Ok(())
    }

    #[test]
    fn actual_semantic_states_have_distinct_feedback() -> Result<(), TraceFailure> {
        let pending = execute_trace(&[1, 0, 0xf0])?;
        let completed = execute_trace(&[1, 0, 0xf0, 1, 0])?;
        assert_ne!(pending.semantic_features(), completed.semantic_features());
        assert_eq!(pending.final_pending(), Some(1));
        Ok(())
    }

    #[test]
    fn equal_path_forward_context_values_remain_semantically_distinguishable()
    -> Result<(), TraceFailure> {
        let even = execute_trace(&[1, 8, 0])?;
        let odd = execute_trace(&[1, 8, 1])?;
        assert_eq!(even.outcome_count(Outcome::Advanced), 1);
        assert_eq!(odd.outcome_count(Outcome::Advanced), 1);
        assert_eq!(even.final_pending(), None);
        assert_eq!(odd.final_pending(), None);
        assert_ne!(even.semantic_features(), odd.semantic_features());
        Ok(())
    }

    #[test]
    fn independent_callbacks_reset_product_model_and_feedback() -> Result<(), TraceFailure> {
        let first = execute_trace(&[1, 0, 0xf0, 2, 0, 8, 0, 1, 0])?;
        let _ = execute_trace(&[1, 0, 0xf0, 1, 0])?;
        let again = execute_trace(&[1, 0, 0xf0, 2, 0, 8, 0, 1, 0])?;
        assert_eq!(first, again);
        Ok(())
    }

    #[test]
    fn complete_framing_admission_does_not_ignore_partial_or_unknown_commands() {
        for bytes in [
            &[][..],
            &[1][..],
            &[1, 0][..],
            &[2, 0, 0][..],
            &[1, 16, 0][..],
        ] {
            assert!(matches!(
                execute_trace(bytes),
                Err(TraceFailure::Malformed(_))
            ));
        }
        let too_long = [1; MAX_TRACE_BYTES + 2];
        assert_eq!(
            execute_trace(&too_long),
            Err(TraceFailure::Malformed(TraceInputError::TooLong))
        );
    }

    #[test]
    fn canonical_control_traces_all_reach_actual_actions() -> Result<(), TraceFailure> {
        for bytes in CONTROL_TRACES {
            let observed = execute_trace(bytes)?;
            assert_eq!(observed.step_count(), (bytes.len() - 1) / 2);
        }
        Ok(())
    }

    /// A real fresh owner/model pair for adapter-seam classification controls.
    /// Injected result categories below qualify the adapter, not a product fault.
    fn effect_fixture() -> Result<(Association, Model, EffectTraceObservation), TraceFailure> {
        let context = Context::initial();
        let owner = Association::new_plain_header_fixture(context.typed()?, request(1)?);
        let model = Model {
            context,
            pending: None,
            next: 1,
            last_capture: None,
        };
        Ok((owner, model, EffectTraceObservation::new()))
    }

    #[test]
    fn effect_lane_matches_strict_actual_observations_on_healthy_controls()
    -> Result<(), Box<dyn std::error::Error>> {
        for bytes in CONTROL_TRACES {
            let strict = execute_trace(bytes)?;
            let effects = execute_effect_trace(bytes)?;
            assert_eq!(effects.observation(), &strict);
            assert_eq!(effects.outcome_difference_count(), 0);
            assert!(effects.outcome_differences().is_empty());
        }
        Ok(())
    }

    #[test]
    fn effect_equivalent_error_difference_preserves_later_real_completion()
    -> Result<(), TraceFailure> {
        let (mut owner, mut model, mut observation) = effect_fixture()?;
        let armed = observe_step(
            &mut owner,
            &mut model,
            Command {
                opcode: 0,
                parameter: 0xf0,
            },
            0,
        )?;
        assert_eq!(
            check_effect_step(&owner, &model, armed, 0, &mut observation),
            Ok(())
        );
        let mut rejected = observe_step(
            &mut owner,
            &mut model,
            Command {
                opcode: 2,
                parameter: 0,
            },
            1,
        )?;
        assert_eq!(rejected.actual, Outcome::OriginMismatch);
        // Inject only the observed error category at the adapter seam. The real
        // product/model state remains untouched and is checked before continuing.
        rejected.actual = Outcome::SocketMismatch;
        assert_eq!(
            check_effect_step(&owner, &model, rejected, 1, &mut observation),
            Ok(())
        );
        let complete = observe_step(
            &mut owner,
            &mut model,
            Command {
                opcode: 1,
                parameter: 0,
            },
            2,
        )?;
        assert_eq!(
            check_effect_step(&owner, &model, complete, 2, &mut observation),
            Ok(())
        );
        assert_eq!(observation.observation().step_count(), 3);
        assert_eq!(observation.observation().completions(), 1);
        assert_eq!(observation.observation().final_pending(), None);
        assert_eq!(
            observation
                .observation()
                .outcome_count(Outcome::SocketMismatch),
            1
        );
        assert_eq!(
            observation
                .observation()
                .outcome_count(Outcome::OriginMismatch),
            0
        );
        let [difference] = observation.outcome_differences() else {
            return Err(TraceFailure::Setup);
        };
        assert_eq!(difference.step(), 1);
        assert_eq!(difference.expected(), Outcome::OriginMismatch);
        assert_eq!(difference.actual(), Outcome::SocketMismatch);
        Ok(())
    }

    #[test]
    fn effect_classifier_distinguishes_acceptance_completion_and_lifecycle_faults()
    -> Result<(), TraceFailure> {
        // Pure diagnostic-seam controls: these deliberately supplied categories
        // do not establish real target fault sensitivity or reached feedback.
        let cases = [
            (
                Outcome::OriginMismatch,
                Outcome::Time,
                None,
                EffectFailureKind::ForbiddenAcceptance,
            ),
            (
                Outcome::Time,
                Outcome::OriginMismatch,
                None,
                EffectFailureKind::MissingRequiredCompletion,
            ),
            (
                Outcome::Time,
                Outcome::Control,
                None,
                EffectFailureKind::CompletionCorruption,
            ),
            (
                Outcome::Time,
                Outcome::Time,
                Some(TraceFailure::CompletionMismatch { step: 0 }),
                EffectFailureKind::CompletionCorruption,
            ),
            (
                Outcome::Busy,
                Outcome::Armed,
                None,
                EffectFailureKind::ForbiddenAcceptance,
            ),
            (
                Outcome::OriginMismatch,
                Outcome::OriginMismatch,
                Some(TraceFailure::StateMismatch { step: 0 }),
                EffectFailureKind::LifecycleStateCorruption,
            ),
        ];
        for (expected, actual, detail_failure, kind) in cases {
            let (owner, model, mut observation) = effect_fixture()?;
            let result = check_effect_step(
                &owner,
                &model,
                StepObservation {
                    expected,
                    actual,
                    detail_failure,
                },
                0,
                &mut observation,
            );
            let Err((actual_kind, _)) = result else {
                return Err(TraceFailure::Setup);
            };
            assert_eq!(actual_kind, kind);
            assert!(observation.outcome_differences().is_empty());
        }
        Ok(())
    }

    #[test]
    fn equal_outcomes_cannot_hide_pending_sequence_or_context_divergence()
    -> Result<(), TraceFailure> {
        for field in 0..3 {
            let (mut owner, mut model, mut observation) = effect_fixture()?;
            let armed = observe_step(
                &mut owner,
                &mut model,
                Command {
                    opcode: 0,
                    parameter: 0xf0,
                },
                0,
            )?;
            assert_eq!(armed.expected, armed.actual);
            // Change independent expected state only to challenge each getter
            // comparison; the real target is never repaired by its observer.
            match field {
                0 => model.pending = None,
                1 => model.next += 1,
                _ => model.context.clock += 1,
            }
            assert_eq!(
                check_effect_step(&owner, &model, armed, 0, &mut observation),
                Err((
                    EffectFailureKind::LifecycleStateCorruption,
                    TraceFailure::StateMismatch { step: 0 }
                )),
            );
            assert!(observation.outcome_differences().is_empty());
        }
        Ok(())
    }

    #[test]
    fn effect_diagnostics_have_an_enforced_fixed_capacity() -> Result<(), TraceFailure> {
        let mut observation = EffectTraceObservation::new();
        for step in 0..MAX_STEPS {
            observation.record_difference(OutcomeDifference {
                step,
                expected: Outcome::OriginMismatch,
                actual: Outcome::SocketMismatch,
            })?;
        }
        assert_eq!(observation.outcome_differences().len(), MAX_STEPS);
        assert_eq!(
            observation.record_difference(OutcomeDifference {
                step: MAX_STEPS,
                expected: Outcome::OriginMismatch,
                actual: Outcome::SocketMismatch,
            }),
            Err(TraceFailure::Setup),
        );
        assert_eq!(observation.outcome_difference_count(), MAX_STEPS);
        Ok(())
    }

    #[test]
    fn effect_lane_rejects_whole_invalid_inputs_before_actual_calls() {
        for bytes in [
            &[][..],
            &[1][..],
            &[1, 0][..],
            &[2, 0, 0][..],
            &[1, 16, 0][..],
            &[1, 0, 0xf0, 1][..],
        ] {
            let Err(failure) = execute_effect_trace(bytes) else {
                panic!("Malformed trace must not execute any product action");
            };
            assert_eq!(failure.kind(), EffectFailureKind::MalformedInput);
            assert!(failure.cause().is_malformed());
            assert_eq!(failure.step(), None);
            assert_eq!(failure.attempted_steps(), 0);
            assert_eq!(failure.partial_observation().observation().step_count(), 0);
            assert!(
                failure
                    .partial_observation()
                    .outcome_differences()
                    .is_empty()
            );
        }
        assert!(matches!(
            execute_effect_trace(&[1; MAX_TRACE_BYTES + 2]),
            Err(failure) if failure.cause() == TraceFailure::Malformed(TraceInputError::TooLong)
        ));
    }

    #[test]
    fn effect_lane_resets_between_callbacks_and_reaches_the_action_limit()
    -> Result<(), Box<dyn std::error::Error>> {
        let first = execute_effect_trace(&[1, 0, 0xf0, 2, 0, 8, 0, 1, 0])?;
        let _ = execute_effect_trace(&[1, 0, 0xf0, 1, 0])?;
        let again = execute_effect_trace(&[1, 0, 0xf0, 2, 0, 8, 0, 1, 0])?;
        assert_eq!(first, again);
        let mut maximum = [0; MAX_TRACE_BYTES];
        maximum[0] = TRACE_VERSION;
        for command in maximum[1..].chunks_exact_mut(2) {
            command[0] = 2;
        }
        let observed = execute_effect_trace(&maximum)?;
        assert_eq!(observed.observation().step_count(), MAX_STEPS);
        assert_eq!(observed.observation().completions(), 0);
        assert_eq!(observed.outcome_difference_count(), 0);
        Ok(())
    }
}
