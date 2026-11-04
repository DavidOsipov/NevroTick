//! Independent finite segment oracle for the public leap-knowledge consumer.
//!
//! Raw ledgers, expected offset segments and event facts are separately labelled.
//! Coverage is evaluated on a partition, carrying known offsets one adjacent
//! atom at a time only where the continuous rule proves no intervening event.
//! This model uses no production validators or global horizon/bridge formula.

use std::error::Error;

use time_domain::{EraWindow, LeapDateScope, LeapError, LeapEventKnowledge, LeapOffsetKnowledge};

use super::{SECOND, assert_answer, knowledge};

/// Literal monthly instants, independently specified on the NTP coordinate scale.
const START: u64 = 2_272_060_800;
/// Synthetic first event boundary, 1 February 1972.
const FEBRUARY: u64 = 2_274_739_200;
/// Synthetic second event boundary, 1 March 1972.
const MARCH: u64 = 2_277_244_800;
/// Synthetic third event boundary, 1 April 1972.
const APRIL: u64 = 2_279_923_200;
/// All synthetic events precede this 1 May 1972 completeness boundary.
const HISTORY: u64 = 2_282_515_200;
/// Explicit final candidate date; the truth table extends one more second.
const END: u64 = HISTORY + 3;

/// One closed synthetic world, with separate input and literal expected facts.
struct LedgerCase {
    /// Raw product-constructor input, never used to calculate oracle offsets.
    records: &'static [(u64, i32)],
    /// Half-open labelled truth segments covering the complete finite scope.
    segments: &'static [(u64, u64, i32)],
    /// Independently labelled inclusive event-membership facts, excluding baseline.
    events: &'static [u64],
}

/// Five worlds: constant, positive, negative, cancelling and negative/positive/negative.
const CASES: [LedgerCase; 5] = [
    LedgerCase {
        records: &[(START, 10)],
        segments: &[(START, END + 1, 10)],
        events: &[],
    },
    LedgerCase {
        records: &[(START, 10), (FEBRUARY, 11)],
        segments: &[(START, FEBRUARY, 10), (FEBRUARY, END + 1, 11)],
        events: &[FEBRUARY],
    },
    LedgerCase {
        records: &[(START, 10), (FEBRUARY, 9)],
        segments: &[(START, FEBRUARY, 10), (FEBRUARY, END + 1, 9)],
        events: &[FEBRUARY],
    },
    LedgerCase {
        records: &[(START, 10), (FEBRUARY, 11), (MARCH, 10)],
        segments: &[
            (START, FEBRUARY, 10),
            (FEBRUARY, MARCH, 11),
            (MARCH, END + 1, 10),
        ],
        events: &[FEBRUARY, MARCH],
    },
    LedgerCase {
        records: &[(START, 10), (FEBRUARY, 9), (MARCH, 10), (APRIL, 9)],
        segments: &[
            (START, FEBRUARY, 10),
            (FEBRUARY, MARCH, 9),
            (MARCH, APRIL, 10),
            (APRIL, END + 1, 9),
        ],
        events: &[FEBRUARY, MARCH, APRIL],
    },
];

/// Explicit finite knowledge claims selected only from the declared matrix.
#[derive(Clone, Copy)]
struct Claims {
    /// Historical facts cover dates strictly before this endpoint.
    history: u64,
    /// Optional finite coverage is separate, including a present zero.
    expiry: Option<u64>,
    /// No-event facts begin inclusively at this optional rule boundary.
    continuous: Option<u64>,
}

/// One inclusive elementary atom whose truth and coverage do not change inside.
struct Atom {
    /// First Q32 tick represented by this atom.
    first: i128,
    /// Last Q32 tick represented by this atom.
    last: i128,
    /// Offset established by evidence or adjacent continuous propagation.
    offset: Option<i32>,
    /// Event absence/presence is completely covered throughout this atom.
    events_known: bool,
}

/// Separate oracle facts, not a forged checked product interpretation.
struct Expected {
    /// Offset is constant, varies across known segments, or lacks evidence.
    offset: LeapOffsetKnowledge,
    /// Completeness over every intersected atom, not only candidate endpoints.
    events: LeapEventKnowledge,
    /// Known literal event instants belonging to the inclusive candidate interval.
    count: usize,
}

/// Convert any u64 whole second exactly; its 96-bit product always fits i128.
fn ticks(seconds: u64) -> i128 {
    i128::from(seconds) * SECOND
}

/// Enumerate 21 boundary representatives, covering every change and adjacent tick.
fn representatives() -> [i128; 21] {
    [
        ticks(START),
        ticks(START) + 1,
        ticks(FEBRUARY) - 1,
        ticks(FEBRUARY),
        ticks(FEBRUARY) + 1,
        ticks(MARCH) - 1,
        ticks(MARCH),
        ticks(MARCH) + 1,
        ticks(APRIL) - 1,
        ticks(APRIL),
        ticks(APRIL) + 1,
        ticks(HISTORY) - 1,
        ticks(HISTORY),
        ticks(HISTORY) + 1,
        ticks(HISTORY + 1) - 1,
        ticks(HISTORY + 1),
        ticks(HISTORY + 1) + 1,
        ticks(HISTORY + 2) - 1,
        ticks(HISTORY + 2),
        ticks(HISTORY + 2) + 1,
        ticks(END),
    ]
}

/// Partition every truth/coverage change into singleton and intervening atoms.
///
/// Every world event and possible metadata boundary belongs to this partition.
/// All 15 atoms are contiguous, so forward propagation cannot jump over a gap.
///
/// # Errors
/// Rejects missing literal truth labels or inconsistent continuity assumptions.
fn model_atoms(case: &LedgerCase, claims: Claims) -> Result<Vec<Atom>, Box<dyn Error>> {
    let boundaries = [
        START,
        FEBRUARY,
        MARCH,
        APRIL,
        HISTORY,
        HISTORY + 1,
        HISTORY + 2,
        END,
    ];
    let mut ranges = Vec::with_capacity(15);
    for pair in boundaries.windows(2) {
        let [first, next] = pair else {
            return Err("invalid oracle boundary pair".into());
        };
        ranges.push((ticks(*first), ticks(*first)));
        if ticks(*first) + 1 < ticks(*next) {
            ranges.push((ticks(*first) + 1, ticks(*next) - 1));
        }
    }
    ranges.push((ticks(END), ticks(END)));
    let mut atoms = Vec::with_capacity(15);
    let mut carried = None;
    for (first, last) in ranges {
        let truth = case
            .segments
            .iter()
            .find(|&&(from, before, _)| ticks(from) <= first && last < ticks(before))
            .map(|&(_, _, offset)| offset)
            .ok_or("oracle atom lacks a complete truth segment")?;
        let directly_known = last < ticks(claims.history)
            || claims.expiry.is_some_and(|expiry| last < ticks(expiry));
        let continuous = claims.continuous.is_some_and(|start| first >= ticks(start));
        let offset = if directly_known {
            Some(truth)
        } else if continuous {
            carried
        } else {
            None
        };
        if offset.is_some_and(|known| known != truth) {
            return Err("oracle continuity contradicts labelled world truth".into());
        }
        carried = offset;
        atoms.push(Atom {
            first,
            last,
            offset,
            events_known: directly_known || continuous,
        });
    }
    Ok(atoms)
}

/// Find the complete evidence atom containing one exact representative tick.
///
/// # Errors
/// Rejects a query not covered by the declared finite oracle partition.
fn atom_at(atoms: &[Atom], point: i128) -> Result<&Atom, Box<dyn Error>> {
    atoms
        .iter()
        .find(|atom| atom.first <= point && point <= atom.last)
        .ok_or_else(|| "oracle point outside partition".into())
}

/// Derive interval truth from all intersected labelled atoms, including the middle.
///
/// This compares established segment labels, not endpoint offsets or production
/// transition predicates. Unknown facts take precedence over varying known ones.
///
/// # Errors
/// Rejects a candidate absent from the declared finite truth partition.
fn expected(
    case: &LedgerCase,
    atoms: &[Atom],
    first: i128,
    last: i128,
) -> Result<Expected, Box<dyn Error>> {
    let mut first_offset = None;
    let mut missing_offset = false;
    let mut varies = false;
    let mut events_complete = true;
    let mut intersected = false;
    for atom in atoms {
        if atom.last < first || atom.first > last {
            continue;
        }
        intersected = true;
        events_complete &= atom.events_known;
        if let Some(value) = atom.offset {
            varies |= first_offset.is_some_and(|initial| initial != value);
            first_offset = Some(value);
        } else {
            missing_offset = true;
        }
    }
    if !intersected {
        return Err("oracle interval outside partition".into());
    }
    let offset = if missing_offset {
        LeapOffsetKnowledge::Unknown
    } else if varies {
        LeapOffsetKnowledge::Varying
    } else {
        LeapOffsetKnowledge::Known(first_offset.ok_or("oracle offset evidence missing")?)
    };
    let count = case
        .events
        .iter()
        .filter(|&&event| first <= ticks(event) && ticks(event) <= last)
        .count();
    let events = if !events_complete {
        LeapEventKnowledge::Unknown
    } else if count == 0 {
        LeapEventKnowledge::NoEvents
    } else {
        LeapEventKnowledge::KnownEvents
    };
    Ok(Expected {
        offset,
        events,
        count,
    })
}

/// Execute the exact declared finite property inventory through real public APIs.
///
/// Every one of 120 contexts must construct successfully; no failed case is
/// filtered out. Exactly 27,720 scoped queries and 240 out-of-scope queries run.
///
/// # Errors
/// Propagates oracle-inventory and real constructor/interpretation failures.
/// # Panics
/// Fails on a mismatch, incorrect inventory, or an unexercised truth category.
pub(super) fn check_closed_inventory() -> Result<(), Box<dyn Error>> {
    let points = representatives();
    let scope = LeapDateScope::new(EraWindow::new(ticks(START), ticks(END))?)?;
    let mut contexts = 0;
    let mut accepted = 0;
    let mut rejected = 0;
    let mut known = 0;
    let mut varying = 0;
    let mut unknown = 0;
    let mut covered_endpoints_with_gap = 0;
    for case in &CASES {
        for history in [HISTORY, HISTORY + 1] {
            for expiry in [None, Some(0), Some(HISTORY + 2)] {
                for continuous in [None, Some(HISTORY), Some(HISTORY + 1), Some(HISTORY + 2)] {
                    let claims = Claims {
                        history,
                        expiry,
                        continuous,
                    };
                    let atoms = model_atoms(case, claims)?;
                    let actual = knowledge(case.records, history, expiry, continuous)?;
                    contexts += 1;
                    for (index, &first) in points.iter().enumerate() {
                        for &last in points.iter().skip(index) {
                            let expected = expected(case, &atoms, first, last)?;
                            let dates = EraWindow::new(first, last)?;
                            let answer = actual.interpret(dates, scope)?;
                            assert_answer(answer, expected.offset, expected.events, expected.count);
                            assert_eq!(answer.dates(), dates);
                            accepted += 1;
                            match expected.offset {
                                LeapOffsetKnowledge::Known(_) => known += 1,
                                LeapOffsetKnowledge::Varying => varying += 1,
                                LeapOffsetKnowledge::Unknown => unknown += 1,
                            }
                            if expected.events == LeapEventKnowledge::Unknown
                                && atom_at(&atoms, first)?.events_known
                                && atom_at(&atoms, last)?.events_known
                            {
                                covered_endpoints_with_gap += 1;
                            }
                        }
                    }
                    for point in [ticks(START) - 1, ticks(END) + 1] {
                        assert_eq!(
                            actual.interpret(EraWindow::new(point, point)?, scope),
                            Err(LeapError::UnsupportedDates)
                        );
                        rejected += 1;
                    }
                }
            }
        }
    }
    assert_eq!(contexts, 120);
    assert_eq!(accepted, 27_720);
    assert_eq!(rejected, 240);
    assert!(known > 0 && varying > 0 && unknown > 0);
    assert!(covered_endpoints_with_gap > 0);
    Ok(())
}
