// SPDX-License-Identifier: GPL-2.0-only
//! Independent S1.1e literal consumers for CHECK-6/7/25.
//!
//! Authored from card 151bb3610b3661e67e198a4fe56c84adb4082bf2d0a07bffeff6aae6be6fb828,
//! exact rational physical trajectories and the public D contract. The sibling
//! driver's implementation supplies no expected result. All storage is bounded;
//! these tests perform no network, native, ambient-clock or privileged operation.
#![forbid(unsafe_code)]

use std::{error::Error, io, net::SocketAddr, num::NonZeroU64};

use ntp_exchange::{
    AssociationContext, AssociationGeneration, Binding, ClockGeneration, ConfigurationGeneration,
    ContextError, ExpireError, Generations, ReceiveError, SocketId, SourceId,
};
use ntp_wire::FrameError;

use super::scenario_driver::{
    self as driver, Capture, CaseId, ClockReading, Completed, EventKind, Fraction, HarnessError,
    Observation, Outcome, RunId, RunReport, StateSnapshot, TimeProjection, ValidationControl,
    ValidationInput,
};

/// Fixture/setup errors propagate; assertion failures identify a violated fact.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact independent era-zero Q32 base, without a trusted-date claim.
const BASE: i128 = 0xe000_0000_0000_0000;
/// A second expressed as an integer number of nanoseconds.
const SECOND: u64 = 1_000_000_000;
/// One Q32 second, used only for independently checking captured coordinates.
const Q32: i128 = 4_294_967_296;
/// The complete concrete inventory, independently enumerated rather than borrowed.
const CASES: [CaseId; 19] = [
    CaseId::R01,
    CaseId::R02,
    CaseId::R03,
    CaseId::R04,
    CaseId::R05,
    CaseId::R06,
    CaseId::R07,
    CaseId::R08,
    CaseId::R09,
    CaseId::R10,
    CaseId::R11,
    CaseId::R12,
    CaseId::R13,
    CaseId::R14,
    CaseId::R15,
    CaseId::R16,
    CaseId::R17,
    CaseId::R18,
    CaseId::R19,
];

/// Complete symmetric reply: literal quality, origin 1, peer stamps 3/16 and 1/4.
const SYMMETRIC_BYTES: [u8; 48] = [
    0x24, 0x02, 0x06, 0xec, 0, 0, 0, 0, 0, 0, 0, 0, 0x47, 0x50, 0x53, 0, 0xdf, 0xff, 0xff, 0xff, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0xe0, 0, 0, 0, 0x30, 0, 0, 0, 0xe0, 0, 0, 0, 0x40, 0, 0, 0,
];
/// Complete asymmetric early-peer reply: identical quality, stamps 1/8 and 3/16.
const EARLY_BYTES: [u8; 48] = [
    0x24, 0x02, 0x06, 0xec, 0, 0, 0, 0, 0, 0, 0, 0, 0x47, 0x50, 0x53, 0, 0xdf, 0xff, 0xff, 0xff, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0xe0, 0, 0, 0, 0x20, 0, 0, 0, 0xe0, 0, 0, 0, 0x30, 0, 0, 0,
];

/// Three fixture generations and the independently named binding variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContextFact {
    /// Configuration, association and clock in that order.
    generations: (u64, u64, u64),
    /// False names source/socket/address 1; true names all three as 2.
    rebound: bool,
}
/// Initial context; no binding or generation change has occurred.
const ORIGINAL: ContextFact = ContextFact {
    generations: (1, 1, 1),
    rebound: false,
};
/// Accepted configuration-only change.
const CONFIGURED: ContextFact = ContextFact {
    generations: (2, 1, 1),
    rebound: false,
};
/// Accepted association/binding change.
const REBOUND: ContextFact = ContextFact {
    generations: (1, 2, 1),
    rebound: true,
};
/// Accepted clock mapping invalidation.
const REMAPPED: ContextFact = ContextFact {
    generations: (1, 1, 2),
    rebound: false,
};

/// Expected product result; every entry comes from a literal card transition.
#[derive(Clone, Copy, Debug)]
enum Wanted {
    /// The explicit fixture arm returns this identity.
    Arm(u64),
    /// One time observation completes this identity.
    Time(u64),
    /// One opaque control completes this identity.
    Control(u64),
    /// A receive rejects without consuming the pending request.
    Rejected(ReceiveError),
    /// Cancellation retires this identity.
    Cancel(u64),
    /// Explicit expiry either preserves or retires the current request.
    Expire(Result<u64, ExpireError>),
    /// A trusted context action has this exact retirement/error result.
    Advance(Result<Option<u64>, ContextError>),
}

/// Literal state after one product action, plus its fixed dispatch position.
#[derive(Clone, Copy, Debug)]
struct ActionFact {
    /// Monotonic nanoseconds, independently specified by the card.
    at: u64,
    /// Total-order tie breaker at the named instant.
    ordinal: u8,
    /// Exact result variant or rejection.
    wanted: Wanted,
    /// Pending ID after this action.
    pending: Option<u64>,
    /// Next ID after this action; none of these finite runs exhaust it.
    next: u64,
    /// Context after this action.
    context: ContextFact,
}

/// Package a literal fact; immutable inputs are retained without computation.
const fn action(
    at: u64,
    ordinal: u8,
    wanted: Wanted,
    pending: Option<u64>,
    next: u64,
    context: ContextFact,
) -> ActionFact {
    ActionFact {
        at,
        ordinal,
        wanted,
        pending,
        next,
        context,
    }
}

/// Immutable card actions for R01, R02, R03, R04, R05 after initial arm.
static ACTIONS_R01_R02_R03_R04_R05: [ActionFact; 2] = [
    action(1_312_500_000, 1, Wanted::Time(1), None, 2, ORIGINAL),
    action(
        1_312_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
];

/// Immutable card actions for R06 after initial arm.
static ACTIONS_R06: [ActionFact; 5] = [
    action(
        1_999_999_999,
        1,
        Wanted::Expire(Err(ExpireError::NotExpired)),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(2_000_000_000, 1, Wanted::Expire(Ok(1)), None, 2, ORIGINAL),
    action(3_000_000_000, 1, Wanted::Arm(2), Some(2), 3, ORIGINAL),
    action(3_312_500_000, 1, Wanted::Time(2), None, 3, ORIGINAL),
    action(
        3_312_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        ORIGINAL,
    ),
];

/// Immutable card actions for R07 after initial arm.
static ACTIONS_R07: [ActionFact; 5] = [
    action(1_500_000_000, 1, Wanted::Cancel(1), None, 2, ORIGINAL),
    action(1_500_000_000, 2, Wanted::Arm(2), Some(2), 3, ORIGINAL),
    action(
        1_562_500_000,
        1,
        Wanted::Rejected(ReceiveError::OriginMismatch),
        Some(2),
        3,
        ORIGINAL,
    ),
    action(1_812_500_000, 1, Wanted::Time(2), None, 3, ORIGINAL),
    action(
        1_812_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        ORIGINAL,
    ),
];

/// Immutable card actions for R08 after initial arm.
static ACTIONS_R08: [ActionFact; 2] = [
    action(1_999_999_999, 1, Wanted::Time(1), None, 2, ORIGINAL),
    action(
        2_000_000_000,
        1,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
];

/// Immutable card actions for R09 after initial arm.
static ACTIONS_R09: [ActionFact; 3] = [
    action(
        2_000_000_000,
        1,
        Wanted::Rejected(ReceiveError::DeadlineReached),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(2_000_000_000, 2, Wanted::Expire(Ok(1)), None, 2, ORIGINAL),
    action(
        2_000_000_000,
        3,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
];

/// Immutable card actions for R10 after initial arm.
static ACTIONS_R10: [ActionFact; 2] = [
    action(
        2_000_000_001,
        1,
        Wanted::Rejected(ReceiveError::DeadlineReached),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(2_000_000_001, 2, Wanted::Expire(Ok(1)), None, 2, ORIGINAL),
];

/// Immutable card actions for R11 after initial arm.
static ACTIONS_R11: [ActionFact; 5] = [
    action(
        1_500_000_000,
        1,
        Wanted::Advance(Ok(Some(1))),
        None,
        2,
        CONFIGURED,
    ),
    action(1_500_000_000, 2, Wanted::Arm(2), Some(2), 3, CONFIGURED),
    action(
        1_562_500_000,
        1,
        Wanted::Rejected(ReceiveError::OriginMismatch),
        Some(2),
        3,
        CONFIGURED,
    ),
    action(1_812_500_000, 1, Wanted::Time(2), None, 3, CONFIGURED),
    action(
        1_812_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        CONFIGURED,
    ),
];

/// Immutable card actions for R12 after initial arm.
static ACTIONS_R12: [ActionFact; 5] = [
    action(
        1_500_000_000,
        1,
        Wanted::Advance(Ok(Some(1))),
        None,
        2,
        REBOUND,
    ),
    action(1_500_000_000, 2, Wanted::Arm(2), Some(2), 3, REBOUND),
    action(
        1_562_500_000,
        1,
        Wanted::Rejected(ReceiveError::SourceMismatch),
        Some(2),
        3,
        REBOUND,
    ),
    action(1_812_500_000, 1, Wanted::Time(2), None, 3, REBOUND),
    action(
        1_812_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        REBOUND,
    ),
];

/// Immutable card actions for R13 after initial arm.
static ACTIONS_R13: [ActionFact; 3] = [
    action(
        1_500_000_000,
        1,
        Wanted::Advance(Err(ContextError::ConfigurationRegression)),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(1_562_500_000, 1, Wanted::Time(1), None, 2, ORIGINAL),
    action(
        1_562_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
];

/// Immutable card actions for R14 after initial arm.
static ACTIONS_R14: [ActionFact; 3] = [
    action(
        1_500_000_000,
        1,
        Wanted::Advance(Err(ContextError::BindingChangedWithoutAssociationAdvance)),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(1_562_500_000, 1, Wanted::Time(1), None, 2, ORIGINAL),
    action(
        1_562_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
];

/// Immutable card actions for R15, R16 after initial arm.
static ACTIONS_R15_R16: [ActionFact; 5] = [
    action(
        1_500_000_000,
        2,
        Wanted::Advance(Ok(Some(1))),
        None,
        2,
        REMAPPED,
    ),
    action(1_500_000_000, 3, Wanted::Arm(2), Some(2), 3, REMAPPED),
    action(
        1_562_500_000,
        1,
        Wanted::Rejected(ReceiveError::MappingGenerationMismatch),
        Some(2),
        3,
        REMAPPED,
    ),
    action(1_812_500_000, 1, Wanted::Time(2), None, 3, REMAPPED),
    action(
        1_812_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        REMAPPED,
    ),
];

/// Immutable card actions for R17 after initial arm.
static ACTIONS_R17: [ActionFact; 5] = [
    action(
        1_562_500_000,
        1,
        Wanted::Advance(Ok(Some(1))),
        None,
        2,
        REMAPPED,
    ),
    action(1_625_000_000, 1, Wanted::Arm(2), Some(2), 3, REMAPPED),
    action(
        1_687_500_000,
        1,
        Wanted::Rejected(ReceiveError::MappingGenerationMismatch),
        Some(2),
        3,
        REMAPPED,
    ),
    action(1_937_500_000, 1, Wanted::Time(2), None, 3, REMAPPED),
    action(
        1_937_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        REMAPPED,
    ),
];

/// Immutable card actions for R18 after initial arm.
static ACTIONS_R18: [ActionFact; 6] = [
    action(1_562_500_000, 1, Wanted::Expire(Ok(1)), None, 2, ORIGINAL),
    action(
        1_562_500_000,
        2,
        Wanted::Advance(Ok(None)),
        None,
        2,
        REMAPPED,
    ),
    action(
        1_562_500_000,
        3,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        REMAPPED,
    ),
    action(1_625_000_000, 1, Wanted::Arm(2), Some(2), 3, REMAPPED),
    action(1_937_500_000, 1, Wanted::Time(2), None, 3, REMAPPED),
    action(
        1_937_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        REMAPPED,
    ),
];

/// Immutable card actions for R19 after initial arm.
static ACTIONS_R19: [ActionFact; 10] = [
    action(
        1_312_500_000,
        1,
        Wanted::Rejected(ReceiveError::Framing(FrameError::HeaderTooShort)),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(
        1_312_500_000,
        2,
        Wanted::Rejected(ReceiveError::OriginMismatch),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(1_312_500_000, 3, Wanted::Time(1), None, 2, ORIGINAL),
    action(
        1_312_500_000,
        4,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
    action(2_000_000_000, 1, Wanted::Arm(2), Some(2), 3, ORIGINAL),
    action(2_312_500_000, 1, Wanted::Control(2), None, 3, ORIGINAL),
    action(
        2_312_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        ORIGINAL,
    ),
    action(3_000_000_000, 1, Wanted::Arm(3), Some(3), 4, ORIGINAL),
    action(3_312_500_000, 1, Wanted::Time(3), None, 4, ORIGINAL),
    action(
        3_312_500_000,
        2,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        4,
        ORIGINAL,
    ),
];

/// Immutable two-exchange reset facts; no opaque-control stage is present.
static RESET_ACTIONS: [ActionFact; 7] = [
    action(
        1_312_500_000,
        2,
        Wanted::Rejected(ReceiveError::Framing(FrameError::HeaderTooShort)),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(
        1_312_500_000,
        3,
        Wanted::Rejected(ReceiveError::OriginMismatch),
        Some(1),
        2,
        ORIGINAL,
    ),
    action(1_312_500_000, 4, Wanted::Time(1), None, 2, ORIGINAL),
    action(
        1_312_500_000,
        5,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        2,
        ORIGINAL,
    ),
    action(3_000_000_000, 1, Wanted::Arm(2), Some(2), 3, ORIGINAL),
    action(3_312_500_000, 2, Wanted::Time(2), None, 3, ORIGINAL),
    action(
        3_312_500_000,
        3,
        Wanted::Rejected(ReceiveError::NoPending),
        None,
        3,
        ORIGINAL,
    ),
];

/// Borrow the named immutable card table after common construction/initial arm.
/// All backing storage is static; no temporary owner or heap allocation exists.
fn actions(case: CaseId) -> &'static [ActionFact] {
    match case {
        CaseId::R01 | CaseId::R02 | CaseId::R03 | CaseId::R04 | CaseId::R05 => {
            &ACTIONS_R01_R02_R03_R04_R05
        }
        CaseId::R06 => &ACTIONS_R06,
        CaseId::R07 => &ACTIONS_R07,
        CaseId::R08 => &ACTIONS_R08,
        CaseId::R09 => &ACTIONS_R09,
        CaseId::R10 => &ACTIONS_R10,
        CaseId::R11 => &ACTIONS_R11,
        CaseId::R12 => &ACTIONS_R12,
        CaseId::R13 => &ACTIONS_R13,
        CaseId::R14 => &ACTIONS_R14,
        CaseId::R15 | CaseId::R16 => &ACTIONS_R15_R16,
        CaseId::R17 => &ACTIONS_R17,
        CaseId::R18 => &ACTIONS_R18,
        CaseId::R19 => &ACTIONS_R19,
    }
}

/// Borrow the immutable thirteen-event recovery workload action facts.
fn reset_actions() -> &'static [ActionFact] {
    &RESET_ACTIONS
}

/// Independent Q32 coordinate deltas and exact completion timing for a success.
#[derive(Clone, Copy, Debug)]
struct TimeFact {
    /// T1/T2/T3/T4 Q32 ticks relative to BASE, with no runtime rounding.
    timestamps: [i128; 4],
    /// Exact Q33 phase numerator, positive when the local clock is behind.
    offset: i128,
    /// Exact signed Q32 raw RTT numerator, without eligibility policy.
    rtt: i128,
    /// Send/arrival/processing/deadline in monotonic nanoseconds.
    timing: [u64; 4],
}

/// Select literal physical facts for the named completion only.
/// # Errors
/// Rejects a request/case pair that has no declared time completion.
fn time_fact(case: CaseId, request: u64) -> Result<TimeFact, &'static str> {
    let symmetric = TimeFact {
        timestamps: [0, 805_306_368, 1_073_741_824, 1_342_177_280],
        offset: 536_870_912,
        rtt: 1_073_741_824,
        timing: [1_000_000_000, 1_312_500_000, 1_312_500_000, 2_000_000_000],
    };
    match (case, request) {
        (CaseId::R01 | CaseId::R19, 1) => Ok(symmetric),
        (CaseId::R02, 1) => Ok(TimeFact {
            timestamps: [0, 536_870_912, 805_306_368, 1_342_177_280],
            offset: 0,
            ..symmetric
        }),
        (CaseId::R03, 1) => Ok(TimeFact {
            timestamps: [0, 1_073_741_824, 1_342_177_280, 1_342_177_280],
            offset: 1_073_741_824,
            ..symmetric
        }),
        (CaseId::R04, 1) => Ok(TimeFact {
            timestamps: [0, 805_306_368, 1_073_741_824, 1_426_063_360],
            offset: 452_984_832,
            rtt: 1_157_627_904,
            ..symmetric
        }),
        (CaseId::R05, 1) => Ok(TimeFact {
            timestamps: [0, 805_306_368, 2_415_919_104, 1_342_177_280],
            offset: 1_879_048_192,
            rtt: -268_435_456,
            ..symmetric
        }),
        (CaseId::R08, 1) => Ok(TimeFact {
            timing: [1_000_000_000, 1_312_500_000, 1_999_999_999, 2_000_000_000],
            ..symmetric
        }),
        (CaseId::R13 | CaseId::R14, 1) => Ok(TimeFact {
            timing: [1_000_000_000, 1_312_500_000, 1_562_500_000, 2_000_000_000],
            ..symmetric
        }),
        (CaseId::R06, 2) | (CaseId::R19, 3) => Ok(TimeFact {
            timestamps: [8_589_934_592, 9_395_240_960, 9_663_676_416, 9_932_111_872],
            timing: [3_000_000_000, 3_312_500_000, 3_312_500_000, 4_000_000_000],
            ..symmetric
        }),
        (CaseId::R07 | CaseId::R11 | CaseId::R12, 2) => Ok(TimeFact {
            timestamps: [2_147_483_648, 2_952_790_016, 3_221_225_472, 3_489_660_928],
            timing: [1_500_000_000, 1_812_500_000, 1_812_500_000, 2_500_000_000],
            ..symmetric
        }),
        (CaseId::R15, 2) => Ok(TimeFact {
            timestamps: [4_294_967_296, 2_952_790_016, 3_221_225_472, 5_637_144_576],
            offset: -3_758_096_384,
            timing: [1_500_000_000, 1_812_500_000, 1_812_500_000, 2_500_000_000],
            ..symmetric
        }),
        (CaseId::R16, 2) => Ok(TimeFact {
            timestamps: [0, 2_952_790_016, 3_221_225_472, 1_342_177_280],
            offset: 4_831_838_208,
            timing: [1_500_000_000, 1_812_500_000, 1_812_500_000, 2_500_000_000],
            ..symmetric
        }),
        (CaseId::R17 | CaseId::R18, 2) => Ok(TimeFact {
            timestamps: [2_684_354_560, 3_489_660_928, 3_758_096_384, 4_026_531_840],
            timing: [1_625_000_000, 1_937_500_000, 1_937_500_000, 2_625_000_000],
            ..symmetric
        }),
        _ => Err("undeclared time completion"),
    }
}

/// Compare one semantic sentinel. No side effects or allocation are performed.
/// # Errors
/// Returns the supplied static field name exactly when the values differ.
fn compare<T: PartialEq>(
    actual: &T,
    expected: &T,
    field: &'static str,
) -> Result<(), &'static str> {
    if actual == expected {
        Ok(())
    } else {
        Err(field)
    }
}

/// Require a bounded invariant without constructing a product value.
/// # Errors
/// Returns the static invariant name on a false predicate.
fn require(condition: bool, field: &'static str) -> Result<(), &'static str> {
    compare(&condition, &true, field)
}

/// Construct one of the two literal expected bindings through public types.
/// # Errors
/// Rejects an impossible zero fixture identifier instead of inventing a default.
fn expected_binding(rebound: bool) -> Result<Binding, &'static str> {
    let id = if rebound {
        NonZeroU64::new(2).ok_or("fixture ID")?
    } else {
        NonZeroU64::MIN
    };
    let last = if rebound { 2 } else { 1 };
    Ok(Binding::new(
        SourceId::new(id),
        SocketId::new(id),
        SocketAddr::from(([192, 0, 2, last], 123)),
    ))
}

/// Construct exact expected generation values; no owner transition is invoked.
fn expected_generations(fact: ContextFact) -> Generations {
    Generations::new(
        ConfigurationGeneration::new(fact.generations.0),
        AssociationGeneration::new(fact.generations.1),
        ClockGeneration::new(fact.generations.2),
    )
}

/// Compare all public owner observations with a literal state, without mutation.
/// # Errors
/// Reports binding/generation, pending or next-sequence mismatch.
fn check_state(
    actual: StateSnapshot,
    pending: Option<u64>,
    next: u64,
    context: ContextFact,
) -> Result<(), &'static str> {
    let expected = AssociationContext::new(
        expected_binding(context.rebound)?,
        expected_generations(context),
    );
    compare(&actual.context, &expected, "owner context")?;
    compare(&actual.pending, &pending, "pending identity")?;
    compare(&actual.next_request, &Some(next), "next identity")
}

/// Compare an exact fraction without invoking any sibling arithmetic helper.
/// # Errors
/// Rejects nonpositive denominators, overflow or unequal rational values.
fn check_fraction(
    actual: Fraction,
    numerator: i128,
    denominator: i128,
) -> Result<(), &'static str> {
    require(
        actual.denominator > 0 && denominator > 0,
        "fraction denominator",
    )?;
    let left = actual
        .numerator
        .checked_mul(denominator)
        .ok_or("fraction overflow")?;
    let right = numerator
        .checked_mul(actual.denominator)
        .ok_or("fraction overflow")?;
    compare(&left, &right, "exact clock fraction")
}

/// Derive only physical U/L/A truth from explicit M; no exchange oracle is computed.
/// # Errors
/// Rejects a before-start clock, arithmetic overflow or a trajectory mismatch.
fn check_clocks(case: CaseId, clock: ClockReading) -> Result<(), &'static str> {
    let elapsed = clock
        .monotonic_ns
        .checked_sub(SECOND)
        .ok_or("clock before initial send")?;
    let elapsed_i = i128::from(elapsed);
    let true_numerator = elapsed_i
        .checked_add(62_500_000)
        .ok_or("true UTC overflow")?;
    check_fraction(clock.true_utc, true_numerator, i128::from(SECOND))?;
    if case == CaseId::R04 {
        check_fraction(
            clock.local_utc,
            elapsed_i.checked_mul(17).ok_or("slope overflow")?,
            16_000_000_000,
        )?;
    } else {
        let shifted = if elapsed >= 500_000_000 {
            match case {
                CaseId::R15 => elapsed_i.checked_add(500_000_000).ok_or("step overflow")?,
                CaseId::R16 => elapsed_i.checked_sub(500_000_000).ok_or("step overflow")?,
                _ => elapsed_i,
            }
        } else {
            elapsed_i
        };
        check_fraction(clock.local_utc, shifted, i128::from(SECOND))?;
    }
    let asleep = if matches!(case, CaseId::R17 | CaseId::R18) {
        if elapsed <= 62_500_000 {
            0
        } else if elapsed >= 562_500_000 {
            500_000_000
        } else {
            elapsed
                .checked_sub(62_500_000)
                .ok_or("sleep interval subtraction")?
        }
    } else {
        0
    };
    let awake = elapsed
        .checked_sub(asleep)
        .and_then(|value| value.checked_add(250_000_000))
        .ok_or("awake overflow")?;
    compare(&clock.active_awake_ns, &awake, "active-awake trajectory")
}

/// Check independent completion identity, coordinate provenance and all quality bits.
/// # Errors
/// Reports any changed exact time, metadata, context or monotonic fact.
fn check_time(
    case: CaseId,
    request: u64,
    actual: TimeProjection,
    context: ContextFact,
) -> Result<(), &'static str> {
    let fact = time_fact(case, request)?;
    let mut timestamps = [0; 4];
    for (target, delta) in timestamps.iter_mut().zip(fact.timestamps) {
        *target = BASE
            .checked_add(delta)
            .ok_or("expected coordinate overflow")?;
    }
    compare(
        &[actual.t1, actual.t2, actual.t3, actual.t4],
        &timestamps,
        "saved/peer/local timestamp",
    )?;
    compare(&actual.offset, &fact.offset, "exact phase numerator")?;
    compare(&actual.rtt, &fact.rtt, "exact signed RTT numerator")?;
    compare(&actual.facts.request, &request, "completed request")?;
    compare(
        &actual.facts.binding,
        &expected_binding(context.rebound)?,
        "completed binding",
    )?;
    compare(
        &actual.facts.generations,
        &expected_generations(context),
        "completed generations",
    )?;
    compare(
        &actual.facts.mapping_generation,
        &context.generations.2,
        "completed mapping",
    )?;
    compare(
        &[
            actual.facts.send_ns,
            actual.facts.arrival_ns,
            actual.facts.processing_ns,
            actual.facts.deadline_ns,
        ],
        &fact.timing,
        "completion timing",
    )?;
    compare(
        &(actual.leap, actual.stratum, actual.poll, actual.precision),
        &(0, 2, 6, -20),
        "peer quality bytes",
    )?;
    compare(
        &(
            actual.root_delay,
            actual.root_dispersion,
            actual.reference_id,
            actual.reference_timestamp,
        ),
        &(0, 0, 0x4750_5300, 0xdfff_ffff_0000_0000),
        "peer quality words",
    )
}

/// Check the actual variant and its independently authored expected action.
/// # Errors
/// Rejects a wrong outcome, dispatch position, metadata or retained state.
fn check_action(
    case: CaseId,
    reset: bool,
    actual: &Observation,
    fact: ActionFact,
) -> Result<(), &'static str> {
    compare(&actual.clocks.monotonic_ns, &fact.at, "action instant")?;
    compare(&actual.ordinal, &fact.ordinal, "action ordinal")?;
    compare(
        &actual.kind,
        &action_kind(case, reset, fact.wanted),
        "product action kind",
    )?;
    match (fact.wanted, actual.outcome) {
        (Wanted::Arm(id), Outcome::Armed(result)) => compare(&result, &Ok(id), "arm result")?,
        (Wanted::Cancel(id), Outcome::Cancelled(result)) => {
            compare(&result, &Ok(id), "cancel result")?;
        }
        (Wanted::Expire(expected), Outcome::Expired(result)) => {
            compare(&result, &expected, "expiry result")?;
        }
        (Wanted::Advance(expected), Outcome::Advanced(result)) => {
            compare(&result, &expected, "context result")?;
        }
        (Wanted::Rejected(expected), Outcome::Received(Err(result))) => {
            compare(&result, &expected, "receive rejection")?;
        }
        (Wanted::Time(id), Outcome::Received(Ok(Completed::Time(result)))) => {
            // The reset's second exchange has the same independently specified
            // send/arrival/timestamps as R06's second exchange, including ID 2.
            let physical_case = if reset && id == 2 { CaseId::R06 } else { case };
            check_time(physical_case, id, result, fact.context)?;
        }
        (Wanted::Control(id), Outcome::Received(Ok(Completed::Control(result)))) => {
            compare(&id, &2, "control fixture identity")?;
            compare(&result.code, &0x5241_5445, "opaque control code")?;
            compare(
                &result.facts.binding,
                &expected_binding(false)?,
                "control binding",
            )?;
            compare(&result.facts.request, &2, "control request")?;
            compare(
                &result.facts.generations,
                &expected_generations(ORIGINAL),
                "control generations",
            )?;
            compare(&result.facts.mapping_generation, &1, "control mapping")?;
            compare(
                &[
                    result.facts.send_ns,
                    result.facts.arrival_ns,
                    result.facts.processing_ns,
                    result.facts.deadline_ns,
                ],
                &[2_000_000_000, 2_312_500_000, 2_312_500_000, 3_000_000_000],
                "control timing",
            )?;
        }
        _ => return Err("product outcome variant"),
    }
    check_state(actual.after, fact.pending, fact.next, fact.context)
}

/// Name the card's dispatch kind, separating captured processing from arrival.
fn action_kind(case: CaseId, reset: bool, wanted: Wanted) -> EventKind {
    match wanted {
        Wanted::Arm(_) => EventKind::Arm,
        Wanted::Cancel(_) => EventKind::Cancel,
        Wanted::Expire(_) => EventKind::Expire,
        Wanted::Advance(_) => EventKind::AdvanceContext,
        Wanted::Time(_) if reset || matches!(case, CaseId::R08 | CaseId::R13 | CaseId::R14) => {
            EventKind::Receive
        }
        Wanted::Time(_) | Wanted::Control(_) => EventKind::CaptureReceive,
        Wanted::Rejected(ReceiveError::Framing(_) | ReceiveError::OriginMismatch)
            if case == CaseId::R19 && !reset =>
        {
            EventKind::CaptureReceive
        }
        Wanted::Rejected(_) => EventKind::Receive,
    }
}

/// Read one independently selected network-order 64-bit field from fixed bytes.
/// # Errors
/// Rejects an out-of-range/overflowed start; no parser or product helper is called.
fn timestamp_bits(packet: &[u8; 48], start: usize) -> Result<u64, &'static str> {
    let end = start.checked_add(8).ok_or("wire field overflow")?;
    let bytes = packet.get(start..end).ok_or("wire field range")?;
    let fixed: [u8; 8] = bytes.try_into().map_err(|_| "wire field width")?;
    Ok(u64::from_be_bytes(fixed))
}

/// Check captured coordinates/bytes using the capture's original physical instant.
/// # Errors
/// Rejects capture rewriting, inexact conversion, invalid bounds or literal mismatch.
fn check_capture(case: CaseId, reset: bool, observation: &Observation) -> Result<(), &'static str> {
    let Some(capture) = observation.capture.as_ref() else {
        return require(
            !matches!(
                observation.kind,
                EventKind::Capture | EventKind::CaptureReceive | EventKind::Receive
            ),
            "missing capture",
        );
    };
    compare(
        &capture.length,
        &expected_capture_length(case, reset, observation),
        "exact captured input length",
    )?;
    check_capture_clock(case, capture, observation.clocks.monotonic_ns)?;
    check_capture_bytes(case, capture)
}

/// Select the independently scripted input length, never a parser/result category.
/// R19's baseline malformed neighbor is ordinal 1; reset's is ordinal 2 after
/// its explicit valid capture. Every other named capture/receive input is 48 bytes.
fn expected_capture_length(case: CaseId, reset: bool, observation: &Observation) -> usize {
    let malformed_ordinal = if reset { 2 } else { 1 };
    if case == CaseId::R19
        && observation.clocks.monotonic_ns == 1_312_500_000
        && observation.ordinal == malformed_ordinal
    {
        47
    } else {
        48
    }
}

/// Check retained arrival/T4/generation against independently modeled physical time.
/// # Errors
/// Rejects rewriting, inexact/overflowed conversion, or mismatched clock domains.
fn check_capture_clock(
    case: CaseId,
    capture: &Capture,
    processing_ns: u64,
) -> Result<(), &'static str> {
    compare(
        &capture.clocks.monotonic_ns,
        &capture.arrival_ns,
        "captured arrival clock",
    )?;
    compare(
        &capture.mapping_generation,
        &capture.clocks.mapping_generation,
        "captured mapping clock",
    )?;
    let remapped = match case {
        CaseId::R15 | CaseId::R16 => capture.arrival_ns >= 1_500_000_000,
        CaseId::R17 | CaseId::R18 => capture.arrival_ns >= 1_562_500_000,
        _ => false,
    };
    compare(
        &capture.mapping_generation,
        &(if remapped { 2 } else { 1 }),
        "physical capture generation",
    )?;
    require(
        capture.arrival_ns <= processing_ns,
        "capture after processing",
    )?;
    check_clocks(case, capture.clocks)?;
    let scaled = capture
        .clocks
        .local_utc
        .numerator
        .checked_mul(Q32)
        .ok_or("captured UTC overflow")?;
    require(
        capture.clocks.local_utc.denominator > 0,
        "captured denominator",
    )?;
    require(
        scaled.checked_rem(capture.clocks.local_utc.denominator) == Some(0),
        "inexact captured UTC",
    )?;
    let relative = scaled
        .checked_div(capture.clocks.local_utc.denominator)
        .ok_or("captured division")?;
    compare(
        &capture.local_t4,
        &BASE.checked_add(relative).ok_or("captured base overflow")?,
        "captured T4 mapping",
    )
}

/// Compare immutable packet literals, including fields rejected before arithmetic.
/// # Errors
/// Reports changed complete bytes, old-mapping peer stamps or opaque-control bits.
fn check_capture_bytes(case: CaseId, capture: &Capture) -> Result<(), &'static str> {
    if case == CaseId::R01 {
        compare(
            &capture.packet,
            &SYMMETRIC_BYTES,
            "complete symmetric literal bytes",
        )?;
    }
    if case == CaseId::R02 {
        compare(
            &capture.packet,
            &EARLY_BYTES,
            "complete asymmetric literal bytes",
        )?;
    }
    if matches!(case, CaseId::R15 | CaseId::R16) && capture.arrival_ns == 1_312_500_000 {
        compare(
            &(capture.mapping_generation, capture.local_t4),
            &(1, BASE + 1_342_177_280),
            "pre-step capture",
        )?;
        compare(
            &capture.packet,
            &SYMMETRIC_BYTES,
            "unchanged pre-step packet",
        )?;
    }
    if matches!(case, CaseId::R17 | CaseId::R18) && capture.arrival_ns == 1_062_500_000 {
        compare(
            &(capture.mapping_generation, capture.local_t4),
            &(1, BASE + 268_435_456),
            "pre-suspend capture",
        )?;
        compare(
            &capture.binding,
            &expected_binding(false)?,
            "pre-suspend binding",
        )?;
        compare(
            &timestamp_bits(&capture.packet, 24)?,
            &1,
            "pre-suspend origin",
        )?;
        compare(
            &timestamp_bits(&capture.packet, 32)?,
            &0xe000_0000_1800_0000,
            "pre-suspend peer receive",
        )?;
        compare(
            &timestamp_bits(&capture.packet, 40)?,
            &0xe000_0000_1800_0000,
            "pre-suspend peer transmit",
        )?;
    }
    if case == CaseId::R19 && capture.arrival_ns == 2_312_500_000 {
        compare(
            &capture.packet.first().copied(),
            &Some(0xe4),
            "control ignored leap bits",
        )?;
        compare(&capture.packet.get(1).copied(), &Some(0), "control stratum")?;
        compare(&timestamp_bits(&capture.packet, 24)?, &2, "control origin")?;
        compare(
            &timestamp_bits(&capture.packet, 32)?,
            &0,
            "control ignored receive",
        )?;
        compare(
            &timestamp_bits(&capture.packet, 40)?,
            &u64::MAX,
            "control ignored transmit",
        )?;
    }
    Ok(())
}

/// Consume every bounded observation against independent literal state/actions.
/// # Errors
/// Returns the first static mismatch; no product calls or effects occur here.
fn check_report(run_id: RunId, report: &RunReport) -> Result<(), &'static str> {
    compare(&report.run_id, &run_id, "report run identity")?;
    let (case, reset) = match run_id {
        RunId::Baseline(case) => (case, false),
        RunId::ResetRecovery => (CaseId::R19, true),
    };
    require(
        report.event_count > 0 && report.event_count <= 32,
        "nonzero bounded event count",
    )?;
    let expected = if reset {
        reset_actions()
    } else {
        actions(case)
    };
    let mut counts = ObservationCounts::empty();
    let mut previous_order = None;
    let mut previous_state = None;
    for slot in &report.observations {
        let Some(observation) = slot.as_ref() else {
            require(counts.events >= report.event_count, "observation hole")?;
            continue;
        };
        require(counts.events < report.event_count, "extra observation")?;
        increment(&mut counts.events)?;
        let charge = if observation.kind == EventKind::CaptureReceive {
            2
        } else {
            1
        };
        counts.charged = counts
            .charged
            .checked_add(charge)
            .ok_or("charged event overflow")?;
        check_observation(case, reset, observation, previous_order, previous_state)?;
        if observation.outcome == Outcome::None {
            check_pure_event(observation, &mut counts)?;
        } else {
            check_product_event(case, reset, observation, counts.product, expected)?;
            increment(&mut counts.product)?;
        }
        previous_order = Some((observation.clocks.monotonic_ns, observation.ordinal));
        previous_state = Some(observation.after);
    }
    check_report_counts(case, reset, report, &counts, expected.len())?;
    check_lifecycle_counts(case, reset, &counts)?;
    compare(
        &previous_state,
        &Some(report.final_state),
        "final owner observation",
    )?;
    compare(
        &report.final_state.pending,
        &None,
        "pre-teardown pending reset",
    )?;
    compare(&report.final_queue_len, &0, "pre-teardown packet reset")
}

/// Fixed local accounting only; counters never construct or mutate a product owner.
struct ObservationCounts {
    /// Actual initialized trace records visited.
    events: usize,
    /// `CaptureReceive` accounts for two explicit operations.
    charged: usize,
    /// Actual reported D constructor/mutator/receive operations.
    product: usize,
    /// Separately queued immutable captures.
    captures: usize,
    /// Explicit retained-packet releases.
    releases: usize,
    /// Explicit dropped replies.
    drops: usize,
    /// Explicit local wall-clock steps.
    steps: usize,
    /// Explicit suspension events.
    suspends: usize,
    /// Explicit resumed markers.
    resumes: usize,
}

impl ObservationCounts {
    /// Create zeroed local counters; no product/default-domain construction occurs.
    const fn empty() -> Self {
        Self {
            events: 0,
            charged: 0,
            product: 0,
            captures: 0,
            releases: 0,
            drops: 0,
            steps: 0,
            suspends: 0,
            resumes: 0,
        }
    }
}

/// Increment one bounded observation counter, preserving it on overflow.
/// # Errors
/// Reports an impossible host-sized overflow instead of silently wrapping.
fn increment(counter: &mut usize) -> Result<(), &'static str> {
    *counter = counter.checked_add(1).ok_or("observation count overflow")?;
    Ok(())
}

/// Compare position, clocks and capture provenance before consuming an observation.
/// # Errors
/// Reports broken ordering/state continuity, capacities or independent clock facts.
fn check_observation(
    case: CaseId,
    reset: bool,
    observation: &Observation,
    previous_order: Option<(u64, u8)>,
    previous_state: Option<StateSnapshot>,
) -> Result<(), &'static str> {
    let order = (observation.clocks.monotonic_ns, observation.ordinal);
    if let Some(previous) = previous_order {
        require(previous < order, "total event order")?;
    }
    compare(
        &observation.before,
        &previous_state,
        "before/previous owner continuity",
    )?;
    require(observation.queue_len <= 2, "two-slot queue bound")?;
    require(
        observation.clocks.monotonic_ns <= 9_000_000_000,
        "eight-second physical horizon",
    )?;
    check_clocks(case, observation.clocks)?;
    compare(
        &observation.clocks.mapping_generation,
        &observation.after.context.generations().clock().value(),
        "current mapping generation",
    )?;
    check_capture(case, reset, observation)
}

/// Compare a non-product event and count its independently required lifecycle kind.
/// # Errors
/// Rejects hidden effects, wrong release/drop state or an unobserved product call.
fn check_pure_event(
    observation: &Observation,
    counts: &mut ObservationCounts,
) -> Result<(), &'static str> {
    compare(
        &observation.before,
        &Some(observation.after),
        "pure event owner preservation",
    )?;
    match observation.kind {
        EventKind::Capture => {
            increment(&mut counts.captures)?;
        }
        EventKind::Release => {
            increment(&mut counts.releases)?;
            compare(&observation.queue_len, &0, "released packet slots")?;
        }
        EventKind::Drop => {
            increment(&mut counts.drops)?;
            compare(&observation.queue_len, &0, "dropped packet slot")?;
            check_state(observation.after, Some(1), 2, ORIGINAL)?;
        }
        EventKind::Step => {
            increment(&mut counts.steps)?;
        }
        EventKind::Suspend => {
            increment(&mut counts.suspends)?;
        }
        EventKind::Resume => {
            increment(&mut counts.resumes)?;
        }
        _ => return Err("unobserved product action"),
    }
    Ok(())
}

/// Compare common initial ownership and then the immutable literal action table.
/// # Errors
/// Reports an extra product action or incorrect result, identity, position or state.
fn check_product_event(
    case: CaseId,
    reset: bool,
    observation: &Observation,
    index: usize,
    expected: &[ActionFact],
) -> Result<(), &'static str> {
    let order = (observation.clocks.monotonic_ns, observation.ordinal);
    match index {
        0 => {
            compare(
                &observation.kind,
                &EventKind::Construct,
                "first construction",
            )?;
            compare(
                &observation.outcome,
                &Outcome::Constructed,
                "constructed result",
            )?;
            compare(&order, &(SECOND, 1), "construction instant")?;
            check_state(observation.after, None, 1, ORIGINAL)
        }
        1 => {
            compare(&observation.kind, &EventKind::Arm, "initial arm kind")?;
            compare(
                &observation.outcome,
                &Outcome::Armed(Ok(1)),
                "initial arm result",
            )?;
            compare(&order, &(SECOND, 2), "initial arm instant")?;
            check_state(observation.after, Some(1), 2, ORIGINAL)
        }
        _ => {
            let selected = index.checked_sub(2).ok_or("action index")?;
            let fact = *expected.get(selected).ok_or("extra product action")?;
            check_action(case, reset, observation, fact)
        }
    }
}

/// Compare executed record/product counts, charged budgets and fixed storage size.
/// # Errors
/// Reports mismatched counts, exceeded bounds or a changed logical trace footprint.
fn check_report_counts(
    case: CaseId,
    reset: bool,
    report: &RunReport,
    counts: &ObservationCounts,
    expected_actions: usize,
) -> Result<(), &'static str> {
    compare(
        &counts.events,
        &report.event_count,
        "recorded/executed count",
    )?;
    compare(
        &counts.charged,
        &report.charged_event_count,
        "capture/receive charged events",
    )?;
    require(
        counts.charged <= if reset { 16 } else { 32 },
        "charged dispatch bound",
    )?;
    compare(
        &counts.product,
        &report.product_action_count,
        "recorded product count",
    )?;
    compare(
        &counts.product,
        &expected_actions
            .checked_add(2)
            .ok_or("expected action count")?,
        "complete action inventory",
    )?;
    compare(
        &report.trace_storage_bytes,
        &size_of_val(&report.observations),
        "reported trace storage bytes",
    )?;
    require(
        report.trace_storage_bytes <= 32_768,
        "logical trace storage bound",
    )?;
    let actual = (report.event_count, counts.charged, counts.product);
    if reset {
        compare(&actual, &(13, 13, 9), "reset concrete event inventory")?;
    }
    if !reset && case == CaseId::R19 {
        compare(&actual, &(15, 20, 12), "R19 concrete event inventory")?;
    }
    Ok(())
}

/// Check finite lifecycle counts independently of the driver dispatch implementation.
/// # Errors
/// Reports any omitted/extra capture, drop, release, step or suspension transition.
fn check_lifecycle_counts(
    case: CaseId,
    reset: bool,
    counts: &ObservationCounts,
) -> Result<(), &'static str> {
    let releases = if reset {
        2
    } else if case == CaseId::R19 {
        3
    } else {
        1
    };
    compare(&counts.releases, &releases, "explicit release count")?;
    compare(
        &counts.drops,
        &usize::from(case == CaseId::R06),
        "explicit loss transition",
    )?;
    compare(
        &counts.steps,
        &usize::from(matches!(case, CaseId::R15 | CaseId::R16)),
        "explicit step transition",
    )?;
    let sleep = usize::from(matches!(case, CaseId::R17 | CaseId::R18));
    compare(&counts.suspends, &sleep, "explicit suspension")?;
    compare(&counts.resumes, &sleep, "explicit resume")?;
    let queued = if reset {
        2
    } else {
        usize::from(!matches!(
            case,
            CaseId::R01 | CaseId::R02 | CaseId::R03 | CaseId::R04 | CaseId::R05 | CaseId::R19
        ))
    };
    compare(&counts.captures, &queued, "explicit queued capture")
}

/// Alter a returned projection solely to challenge the real report comparator.
/// # Errors
/// Rejects missing time observations or overflow; no product owner is changed.
fn corrupt_observed_phase(report: &mut RunReport) -> Result<(), &'static str> {
    for observation in report.observations.iter_mut().flatten() {
        if let Outcome::Received(Ok(Completed::Time(time))) = &mut observation.outcome {
            time.offset = time.offset.checked_add(1).ok_or("sentinel overflow")?;
            return Ok(());
        }
    }
    Err("missing real time observation for comparator control")
}

/// Change only one already returned input projection at a fixed card position.
/// # Errors
/// Rejects a missing capture; the product owner and all other observations remain untouched.
fn replace_observed_length(
    report: &mut RunReport,
    ordinal: u8,
    length: usize,
) -> Result<usize, &'static str> {
    for observation in report.observations.iter_mut().flatten() {
        if observation.clocks.monotonic_ns == 1_312_500_000 && observation.ordinal == ordinal {
            let capture = observation
                .capture
                .as_mut()
                .ok_or("length sentinel capture missing")?;
            let previous = capture.length;
            capture.length = length;
            return Ok(previous);
        }
    }
    Err("length sentinel position missing")
}

/// Challenge exact 47-byte and healthy 48-byte facts using an existing report.
/// # Errors
/// Rejects acceptance of length 0/46, failed restoration or an unsupported run.
/// No D call, additional run, allocation or real captured input is introduced.
fn check_length_discrimination(run_id: RunId, report: &mut RunReport) -> Result<(), &'static str> {
    let ordinals = match run_id {
        RunId::Baseline(CaseId::R19) => [1, 3],
        RunId::ResetRecovery => [2, 4],
        RunId::Baseline(_) => return Err("run has no declared malformed-length sentinel"),
    };
    for ordinal in ordinals {
        for length in [0, 46] {
            let original = replace_observed_length(report, ordinal, length)?;
            let accepted = check_report(run_id, report).is_ok();
            replace_observed_length(report, ordinal, original)?;
            if accepted {
                return Err("changed 47/48-byte input length escaped comparison");
            }
            check_report(run_id, report)?;
        }
    }
    Ok(())
}

/// Reach the real driver and retain its bounded report for independent comparison.
/// # Errors
/// Propagates a setup/driver failure, never classifying it as behavioral success.
fn execute(case: CaseId) -> TestResult<RunReport> {
    driver::run_case(case)
        .map_err(|error| io::Error::other(format!("{case:?} harness: {error:?}")).into())
}

/// Define separately discoverable literal cases with the same independent checker.
/// Each generated test panics only if actual output violates the named oracle.
macro_rules! literal_case {
    ($name:ident, $case:ident) => {
        /// Exercise the named literal card run through the actual public consumer.
        /// # Errors
        /// Propagates fixture construction or driver execution failure.
        /// # Panics
        /// Asserts exact outcomes, timestamps, clocks, state and pre-teardown reset.
        #[test]
        fn $name() -> TestResult {
            let mut report = execute(CaseId::$case)?;
            let run_id = RunId::Baseline(CaseId::$case);
            assert_eq!(
                check_report(run_id, &report),
                Ok(()),
                "{}",
                stringify!($case)
            );
            if CaseId::$case == CaseId::R01 {
                assert_eq!(corrupt_observed_phase(&mut report), Ok(()));
                assert!(
                    check_report(run_id, &report).is_err(),
                    "changed real projection escaped report comparison"
                );
            }
            if CaseId::$case == CaseId::R19 {
                assert_eq!(check_length_discrimination(run_id, &mut report), Ok(()));
            }
            Ok(())
        }
    };
}

literal_case!(scenario_r01_symmetric_literal, R01);
literal_case!(scenario_r02_asymmetric_early_literal, R02);
literal_case!(scenario_r03_asymmetric_late_literal, R03);
literal_case!(scenario_r04_oscillator_literal, R04);
literal_case!(scenario_r05_negative_rtt_literal, R05);
literal_case!(scenario_r06_loss_expiry_recovery, R06);
literal_case!(scenario_r07_reorder_correlation, R07);
literal_case!(scenario_r08_captured_t4_before_deadline, R08);
literal_case!(scenario_r09_deadline_then_expire_then_receive, R09);
literal_case!(scenario_r10_late_processing, R10);
literal_case!(scenario_r11_configuration_retirement, R11);
literal_case!(scenario_r12_binding_retirement, R12);
literal_case!(scenario_r13_regression_preserves_request, R13);
literal_case!(scenario_r14_binding_guard_preserves_request, R14);
literal_case!(scenario_r15_positive_step_capture, R15);
literal_case!(scenario_r16_negative_step_capture, R16);
literal_case!(scenario_r17_suspend_generation_recovery, R17);
literal_case!(scenario_r18_monotonic_expiry_before_remap, R18);
literal_case!(scenario_r19_hostile_control_recovery, R19);

/// Inventory comparison reaches no product action.
/// # Panics
/// Asserts all nineteen distinct card IDs, including grouped fresh subruns.
#[test]
fn scenario_inventory_is_exactly_nineteen() {
    assert_eq!(driver::ALL_CASES, CASES);
}

/// Reject a changed semantic sentinel through the actual comparison owner.
/// # Panics
/// A suppressed comparison must fail this control while its legal neighbor passes.
#[test]
fn scenario_comparator_detects_wrong_numerator_and_suppressed_comparison() {
    assert_eq!(
        compare(&536_870_912_i128, &536_870_912_i128, "phase sentinel"),
        Ok(())
    );
    assert_eq!(
        compare(&536_870_912_i128, &536_870_913_i128, "phase sentinel"),
        Err("phase sentinel")
    );
}

/// Challenge the real preflight with independently supplied capacity coordinates.
/// # Panics
/// Asserts exact-cap acceptance, one-over rejection and zero product actions.
#[test]
fn scenario_validation_caps_and_conversion_have_zero_product_actions() {
    assert!(size_of::<[Option<Observation>; 32]>() <= 32_768);
    let exact = ValidationInput {
        run_count: 20,
        event_count: 32,
        queued_count: 2,
        horizon_ns: 8_000_000_000,
        trace_count: 32,
        utc_numerator: 1,
        utc_denominator: 16,
    };
    let inputs = [
        (exact, None),
        (
            ValidationInput {
                run_count: 21,
                ..exact
            },
            Some(HarnessError::RunCapacity),
        ),
        (
            ValidationInput {
                event_count: 33,
                ..exact
            },
            Some(HarnessError::EventCapacity),
        ),
        (
            ValidationInput {
                queued_count: 3,
                ..exact
            },
            Some(HarnessError::QueueCapacity),
        ),
        (
            ValidationInput {
                horizon_ns: 8_000_000_001,
                ..exact
            },
            Some(HarnessError::Horizon),
        ),
        (
            ValidationInput {
                trace_count: 33,
                ..exact
            },
            Some(HarnessError::TraceCapacity),
        ),
        (
            ValidationInput {
                utc_numerator: 1,
                utc_denominator: 1_000_000_000,
                ..exact
            },
            Some(HarnessError::InexactCoordinate),
        ),
    ];
    for (input, expected) in inputs {
        let report = driver::validate_only(input);
        assert_eq!(report.error, expected);
        assert_eq!(report.product_action_count, 0);
    }
}

/// Exercise structural preflight paths which the scalar limits alone cannot reach.
/// # Panics
/// Asserts invalid ordering/segments/coordinate/wire bounds before D construction.
#[test]
fn scenario_invalid_timeline_and_wire_have_zero_product_actions() {
    let controls = [
        (ValidationControl::ExactRunCap, None),
        (
            ValidationControl::RunOverflow,
            Some(HarnessError::RunCapacity),
        ),
        (ValidationControl::ExactEventCap, None),
        (
            ValidationControl::EventOverflow,
            Some(HarnessError::EventCapacity),
        ),
        (ValidationControl::ExactQueueCap, None),
        (
            ValidationControl::QueueOverflow,
            Some(HarnessError::QueueCapacity),
        ),
        (ValidationControl::ExactHorizon, None),
        (
            ValidationControl::HorizonOverflow,
            Some(HarnessError::Horizon),
        ),
        (ValidationControl::ExactTraceCap, None),
        (
            ValidationControl::TraceOverflow,
            Some(HarnessError::TraceCapacity),
        ),
        (ValidationControl::ExactCapture, None),
        (
            ValidationControl::InexactCapture,
            Some(HarnessError::InexactCoordinate),
        ),
        (ValidationControl::InvalidOrder, Some(HarnessError::Order)),
        (
            ValidationControl::InvalidSegment,
            Some(HarnessError::ClockSegment),
        ),
        (
            ValidationControl::CoordinateOverflow,
            Some(HarnessError::Coordinate),
        ),
        (ValidationControl::WireOverflow, Some(HarnessError::Wire)),
    ];
    for (control, expected) in controls {
        let report = driver::validate_control(control);
        assert_eq!(report.error, expected, "{control:?}");
        assert_eq!(report.product_action_count, 0, "{control:?}");
    }
}

/// Replay the fixed thirteen-event recovery run from fresh owners 256 times.
/// # Errors
/// Propagates driver/setup failure or checked event-count overflow.
/// # Panics
/// Asserts each initial identity/context, full behavior and released pre-teardown state.
#[test]
fn scenario_256_fresh_recovery_runs_release_state() -> TestResult {
    let mut events = 0_usize;
    for iteration in 0..256 {
        let mut report = driver::run_reset_recovery()
            .map_err(|error| io::Error::other(format!("reset harness: {error:?}")))?;
        assert_eq!(check_report(RunId::ResetRecovery, &report), Ok(()));
        if iteration == 0 {
            assert_eq!(
                check_length_discrimination(RunId::ResetRecovery, &mut report),
                Ok(())
            );
        }
        assert!(report.charged_event_count <= 16);
        assert_eq!(report.final_state.pending, None);
        assert_eq!(report.final_queue_len, 0);
        events = events
            .checked_add(report.charged_event_count)
            .ok_or_else(|| io::Error::other("reset event count overflow"))?;
    }
    assert!(events > 0 && events <= 4_096);
    Ok(())
}
