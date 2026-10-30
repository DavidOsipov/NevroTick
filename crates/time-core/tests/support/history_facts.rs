// SPDX-License-Identifier: GPL-2.0-only
//! Literal independent history facts transcribed from the frozen preimplementation artifact.
//! SHA-256 ec6a1b96806c6219bd2b74c37b737fde7cbfddafe6cf2f07b12cb14d1d9c92d1.
//! Contract53ce5e686cbb88175bd7138302f35c5d02939316615134451ba74c3fee847ea0.
//! Catalog deduplication changes storage only; every raw row, outcome and report bit is literal.
//! No normalization, fit, statistics or history implementation supplies expected values.

/// Exact immutable fields observable from one admitted raw observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RowFact {
    /// Complete caller-asserted nonzero source/frame/generation labels.
    pub context: [u64; 3],
    /// Exact monotonic reading, without a native-clock claim.
    pub observed_ns: u64,
    /// Exact positive-behind Q33 numerator from the legitimate exchange path.
    pub phase_q33: i128,
    /// Unchanged admitted binary64 variance divisor.
    pub divisor_bits: u64,
}

/// Five exact numerical fields and their independently associated metadata.
#[derive(Clone, Copy)]
pub(super) struct ReportFact {
    /// Complete same-input context.
    pub context: [u64; 3],
    /// Actual newest committed monotonic reading.
    pub reference: u64,
    /// Frame independently observed on summary and contained line.
    pub frame: u64,
    /// Actual committed count.
    pub sample_count: usize,
    /// Exact count minus two.
    pub degrees_of_freedom: usize,
    /// Intercept/rate/E/scale/normalized-variance bits in that order.
    pub bits: [u64; 5],
}

/// One entire expected public state, including genuine held raw rows.
pub(super) struct SnapshotFact {
    /// Immutable assertion tags.
    pub context: [u64; 3],
    /// Count from the published fact, not from the system under test.
    pub sample_count: usize,
    /// Exact reference, absent only for empty state.
    pub reference: Option<u64>,
    /// Actual chronological prefix to observe at every valid index.
    pub rows: &'static [RowFact],
    /// No report for counts zero through two.
    pub report: Option<ReportFact>,
}

/// The frozen first transition outcome; temporary implementation errors never qualify.
#[derive(Clone, Copy)]
pub(super) enum Expected {
    /// Exactly one candidate commits successfully.
    Commit,
    /// Context mismatch wins before order checks.
    ContextMismatch,
    /// Equal committed newest reading.
    DuplicateTime,
    /// Candidate older than committed newest.
    OutOfOrderTime,
    /// Existing normalizer's first staged elapsed-range index.
    ElapsedRange {
        /// Index in the staged prefix, including after full-buffer replacement.
        index: usize,
    },
    /// A sufficient staged span is below the exact minimum.
    SpanTooShort,
    /// Existing computed-binary64 spread rejection.
    InsufficientSpread,
}

/// One counted append and its complete public postcondition.
pub(super) struct StepFact {
    /// Stable diagnostic identity.
    pub name: &'static str,
    /// One actual typed-constructor recipe, including rejected append candidates.
    pub candidate: RowFact,
    /// Exact first result.
    pub expected: Expected,
    /// Complete retained state after this attempt.
    pub after: &'static SnapshotFact,
}

/// One independently frozen bounded input stream.
pub(super) struct TraceFact {
    /// Stable fixture identity.
    pub name: &'static str,
    /// Construct each immutable context once and observe all labels.
    pub contexts: &'static [[u64; 3]],
    /// Genuine empty-state observation immediately after construction.
    pub initial: &'static SnapshotFact,
    /// No implicit retry, reset, prune or subset selection.
    pub steps: &'static [StepFact],
}

/// Frozen raw observation 0.
const ROW_000: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 0,
    phase_q33: -2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 1.
const ROW_001: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 0,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 2.
const ROW_002: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 0,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 3.
const ROW_003: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_000,
    phase_q33: -2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 4.
const ROW_004: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 5.
const ROW_005: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_000,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 6.
const ROW_006: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 10_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 7.
const ROW_007: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_001,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 8.
const ROW_008: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_002,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 9.
const ROW_009: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_003,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 10.
const ROW_010: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_004,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 11.
const ROW_011: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_005,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 12.
const ROW_012: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_006,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 13.
const ROW_013: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_007,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 14.
const ROW_014: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_008,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 15.
const ROW_015: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_009,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 16.
const ROW_016: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_010,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 17.
const ROW_017: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_011,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 18.
const ROW_018: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_012,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 19.
const ROW_019: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_013,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 20.
const ROW_020: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_014,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 21.
const ROW_021: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_015,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 22.
const ROW_022: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_016,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 23.
const ROW_023: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_017,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 24.
const ROW_024: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_018,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 25.
const ROW_025: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_019,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 26.
const ROW_026: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_020,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 27.
const ROW_027: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_021,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 28.
const ROW_028: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_022,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 29.
const ROW_029: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_023,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 30.
const ROW_030: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_024,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 31.
const ROW_031: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_025,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 32.
const ROW_032: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_026,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 33.
const ROW_033: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_027,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 34.
const ROW_034: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_028,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 35.
const ROW_035: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_029,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 36.
const ROW_036: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_030,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 37.
const ROW_037: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_031,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 38.
const ROW_038: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_032,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 39.
const ROW_039: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_033,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 40.
const ROW_040: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_034,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 41.
const ROW_041: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_035,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 42.
const ROW_042: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_036,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 43.
const ROW_043: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_037,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 44.
const ROW_044: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_038,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 45.
const ROW_045: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_039,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 46.
const ROW_046: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_040,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 47.
const ROW_047: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_041,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 48.
const ROW_048: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_042,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 49.
const ROW_049: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_043,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 50.
const ROW_050: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_044,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 51.
const ROW_051: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_045,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 52.
const ROW_052: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_046,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 53.
const ROW_053: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_047,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 54.
const ROW_054: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_048,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 55.
const ROW_055: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_049,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 56.
const ROW_056: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_050,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 57.
const ROW_057: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_051,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 58.
const ROW_058: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_052,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 59.
const ROW_059: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_053,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 60.
const ROW_060: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_054,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 61.
const ROW_061: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_055,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 62.
const ROW_062: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_056,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 63.
const ROW_063: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_057,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 64.
const ROW_064: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_058,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 65.
const ROW_065: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_059,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 66.
const ROW_066: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_060,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 67.
const ROW_067: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_061,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 68.
const ROW_068: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_062,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 69.
const ROW_069: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_000_000_063,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 70.
const ROW_070: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 1_062_500_000,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 71.
const ROW_071: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 11_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 72.
const ROW_072: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 12_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 73.
const ROW_073: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 13_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 74.
const ROW_074: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 14_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 75.
const ROW_075: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 15_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 76.
const ROW_076: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 16_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 77.
const ROW_077: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 17_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 78.
const ROW_078: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 18_446_744_071_709_551_615,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 79.
const ROW_079: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 18_446_744_072_709_551_615,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 80.
const ROW_080: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 18_446_744_073_709_551_614,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 81.
const ROW_081: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 18_446_744_073_709_551_615,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 82.
const ROW_082: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 18_446_744_073_709_551_615,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 83.
const ROW_083: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 2_000_000_000,
    phase_q33: -2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 84.
const ROW_084: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 2_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 85.
const ROW_085: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 2_000_000_000,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 86.
const ROW_086: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 3_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 87.
const ROW_087: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 31_250_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 88.
const ROW_088: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 89.
const ROW_089: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_000_000_000,
    phase_q33: 5_368_709_120,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 90.
const ROW_090: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_094_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x4030_0000_0000_0000,
};
/// Frozen raw observation 91.
const ROW_091: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_095_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3fc0_4104_1041_040f,
};
/// Frozen raw observation 92.
const ROW_092: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_095_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3fc0_4104_1041_0410,
};
/// Frozen raw observation 93.
const ROW_093: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_095_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3fc0_4104_1041_0411,
};
/// Frozen raw observation 94.
const ROW_094: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_095_999_999_999,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 95.
const ROW_095: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_096_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 96.
const ROW_096: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_096_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x4030_0000_0000_0000,
};
/// Frozen raw observation 97.
const ROW_097: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_096_000_000_001,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 98.
const ROW_098: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 4_098_000_000_000,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 99.
const ROW_099: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 5_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 100.
const ROW_100: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 6_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 101.
const ROW_101: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 62_499_999,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 102.
const ROW_102: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 62_500_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 103.
const ROW_103: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 62_500_001,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 104.
const ROW_104: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 7_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 105.
const ROW_105: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 8_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 106.
const ROW_106: RowFact = RowFact {
    context: [17, 23, 29],
    observed_ns: 9_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 107.
const ROW_107: RowFact = RowFact {
    context: [17, 23, 31],
    observed_ns: 2_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 108.
const ROW_108: RowFact = RowFact {
    context: [17, 31, 29],
    observed_ns: 0,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 109.
const ROW_109: RowFact = RowFact {
    context: [19, 23, 29],
    observed_ns: 1_000_000_000,
    phase_q33: 0,
    divisor_bits: 0x3ff0_0000_0000_0000,
};
/// Frozen raw observation 110.
const ROW_110: RowFact = RowFact {
    context: [19, 23, 29],
    observed_ns: 1_000_000_062,
    phase_q33: 2_147_483_648,
    divisor_bits: 0x3ff0_0000_0000_0000,
};

/// Frozen complete state 0, including unchanged rejection neighbors.
const SNAPSHOT_000: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 0,
    reference: None,
    rows: &[],
    report: None,
};
/// Frozen complete state 1, including unchanged rejection neighbors.
const SNAPSHOT_001: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 1,
    reference: Some(0),
    rows: &[ROW_000],
    report: None,
};
/// Frozen complete state 2, including unchanged rejection neighbors.
const SNAPSHOT_002: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 1,
    reference: Some(0),
    rows: &[ROW_001],
    report: None,
};
/// Frozen complete state 3, including unchanged rejection neighbors.
const SNAPSHOT_003: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 1,
    reference: Some(0),
    rows: &[ROW_002],
    report: None,
};
/// Frozen complete state 4, including unchanged rejection neighbors.
const SNAPSHOT_004: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 1,
    reference: Some(18_446_744_071_709_551_615),
    rows: &[ROW_078],
    report: None,
};
/// Frozen complete state 5, including unchanged rejection neighbors.
const SNAPSHOT_005: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 1,
    reference: Some(18_446_744_073_709_551_615),
    rows: &[ROW_081],
    report: None,
};
/// Frozen complete state 6, including unchanged rejection neighbors.
const SNAPSHOT_006: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 1,
    reference: Some(4_094_000_000_000),
    rows: &[ROW_090],
    report: None,
};
/// Frozen complete state 7, including unchanged rejection neighbors.
const SNAPSHOT_007: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(1_000_000_000),
    rows: &[ROW_000, ROW_003],
    report: None,
};
/// Frozen complete state 8, including unchanged rejection neighbors.
const SNAPSHOT_008: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(1_000_000_000),
    rows: &[ROW_001, ROW_004],
    report: None,
};
/// Frozen complete state 9, including unchanged rejection neighbors.
const SNAPSHOT_009: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(2_000_000_000),
    rows: &[ROW_001, ROW_084],
    report: None,
};
/// Frozen complete state 10, including unchanged rejection neighbors.
const SNAPSHOT_010: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(31_250_000),
    rows: &[ROW_001, ROW_087],
    report: None,
};
/// Frozen complete state 11, including unchanged rejection neighbors.
const SNAPSHOT_011: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(4_095_999_999_999),
    rows: &[ROW_001, ROW_094],
    report: None,
};
/// Frozen complete state 12, including unchanged rejection neighbors.
const SNAPSHOT_012: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(4_096_000_000_000),
    rows: &[ROW_001, ROW_095],
    report: None,
};
/// Frozen complete state 13, including unchanged rejection neighbors.
const SNAPSHOT_013: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(1_000_000_000),
    rows: &[ROW_002, ROW_005],
    report: None,
};
/// Frozen complete state 14, including unchanged rejection neighbors.
const SNAPSHOT_014: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(18_446_744_072_709_551_615),
    rows: &[ROW_078, ROW_079],
    report: None,
};
/// Frozen complete state 15, including unchanged rejection neighbors.
const SNAPSHOT_015: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(4_095_000_000_000),
    rows: &[ROW_090, ROW_091],
    report: None,
};
/// Frozen complete state 16, including unchanged rejection neighbors.
const SNAPSHOT_016: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(4_095_000_000_000),
    rows: &[ROW_090, ROW_092],
    report: None,
};
/// Frozen complete state 17, including unchanged rejection neighbors.
const SNAPSHOT_017: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 2,
    reference: Some(4_095_000_000_000),
    rows: &[ROW_090, ROW_093],
    report: None,
};
/// Frozen complete state 18, including unchanged rejection neighbors.
const SNAPSHOT_018: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(2_000_000_000),
    rows: &[ROW_000, ROW_003, ROW_083],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 2_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0xbfd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 19, including unchanged rejection neighbors.
const SNAPSHOT_019: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(2_000_000_000),
    rows: &[ROW_001, ROW_004, ROW_084],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 2_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 20, including unchanged rejection neighbors.
const SNAPSHOT_020: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(3_000_000_000),
    rows: &[ROW_001, ROW_084, ROW_086],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 3_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 21, including unchanged rejection neighbors.
const SNAPSHOT_021: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(62_500_000),
    rows: &[ROW_001, ROW_087, ROW_102],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 62_500_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 22, including unchanged rejection neighbors.
const SNAPSHOT_022: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(4_096_000_000_000),
    rows: &[ROW_001, ROW_094, ROW_095],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_096_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 23, including unchanged rejection neighbors.
const SNAPSHOT_023: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(1_000_000_001),
    rows: &[ROW_002, ROW_005, ROW_007],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_001,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 24, including unchanged rejection neighbors.
const SNAPSHOT_024: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(2_000_000_000),
    rows: &[ROW_002, ROW_005, ROW_085],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 2_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 25, including unchanged rejection neighbors.
const SNAPSHOT_025: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(18_446_744_073_709_551_615),
    rows: &[ROW_078, ROW_079, ROW_082],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 18_446_744_073_709_551_615,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 26, including unchanged rejection neighbors.
const SNAPSHOT_026: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(4_096_000_000_000),
    rows: &[ROW_090, ROW_091, ROW_095],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_096_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 27, including unchanged rejection neighbors.
const SNAPSHOT_027: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(4_096_000_000_000),
    rows: &[ROW_090, ROW_092, ROW_096],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_096_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 28, including unchanged rejection neighbors.
const SNAPSHOT_028: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 3,
    reference: Some(4_096_000_000_000),
    rows: &[ROW_090, ROW_093, ROW_096],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_096_000_000_000,
        frame: 23,
        sample_count: 3,
        degrees_of_freedom: 1,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 29, including unchanged rejection neighbors.
const SNAPSHOT_029: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 4,
    reference: Some(3_000_000_000),
    rows: &[ROW_001, ROW_004, ROW_084, ROW_086],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 3_000_000_000,
        frame: 23,
        sample_count: 4,
        degrees_of_freedom: 2,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 30, including unchanged rejection neighbors.
const SNAPSHOT_030: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 4,
    reference: Some(4_096_000_000_000),
    rows: &[ROW_001, ROW_004, ROW_084, ROW_095],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_096_000_000_000,
        frame: 23,
        sample_count: 4,
        degrees_of_freedom: 2,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 31, including unchanged rejection neighbors.
const SNAPSHOT_031: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 4,
    reference: Some(4_000_000_000),
    rows: &[ROW_001, ROW_084, ROW_086, ROW_089],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_000_000_000,
        frame: 23,
        sample_count: 4,
        degrees_of_freedom: 2,
        bits: [
            0x3fd8_0000_0000_0000,
            0x3fc0_0000_0000_0000,
            0x3fc4_0000_0000_0000,
            0x3fb4_0000_0000_0000,
            0x3fb4_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 32, including unchanged rejection neighbors.
const SNAPSHOT_032: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 4,
    reference: Some(62_500_001),
    rows: &[ROW_001, ROW_087, ROW_102, ROW_103],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 62_500_001,
        frame: 23,
        sample_count: 4,
        degrees_of_freedom: 2,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 33, including unchanged rejection neighbors.
const SNAPSHOT_033: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 4,
    reference: Some(1_000_000_002),
    rows: &[ROW_002, ROW_005, ROW_007, ROW_008],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_002,
        frame: 23,
        sample_count: 4,
        degrees_of_freedom: 2,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 34, including unchanged rejection neighbors.
const SNAPSHOT_034: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 5,
    reference: Some(4_000_000_000),
    rows: &[ROW_001, ROW_004, ROW_084, ROW_086, ROW_088],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 4_000_000_000,
        frame: 23,
        sample_count: 5,
        degrees_of_freedom: 3,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 35, including unchanged rejection neighbors.
const SNAPSHOT_035: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 5,
    reference: Some(1_000_000_003),
    rows: &[ROW_002, ROW_005, ROW_007, ROW_008, ROW_009],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_003,
        frame: 23,
        sample_count: 5,
        degrees_of_freedom: 3,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 36, including unchanged rejection neighbors.
const SNAPSHOT_036: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 6,
    reference: Some(5_000_000_000),
    rows: &[ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 5_000_000_000,
        frame: 23,
        sample_count: 6,
        degrees_of_freedom: 4,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 37, including unchanged rejection neighbors.
const SNAPSHOT_037: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 6,
    reference: Some(1_000_000_004),
    rows: &[ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_004,
        frame: 23,
        sample_count: 6,
        degrees_of_freedom: 4,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 38, including unchanged rejection neighbors.
const SNAPSHOT_038: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 7,
    reference: Some(6_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 6_000_000_000,
        frame: 23,
        sample_count: 7,
        degrees_of_freedom: 5,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 39, including unchanged rejection neighbors.
const SNAPSHOT_039: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 7,
    reference: Some(1_000_000_005),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_005,
        frame: 23,
        sample_count: 7,
        degrees_of_freedom: 5,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 40, including unchanged rejection neighbors.
const SNAPSHOT_040: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 8,
    reference: Some(7_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 7_000_000_000,
        frame: 23,
        sample_count: 8,
        degrees_of_freedom: 6,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 41, including unchanged rejection neighbors.
const SNAPSHOT_041: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 8,
    reference: Some(1_000_000_006),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_006,
        frame: 23,
        sample_count: 8,
        degrees_of_freedom: 6,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 42, including unchanged rejection neighbors.
const SNAPSHOT_042: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 9,
    reference: Some(8_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 8_000_000_000,
        frame: 23,
        sample_count: 9,
        degrees_of_freedom: 7,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 43, including unchanged rejection neighbors.
const SNAPSHOT_043: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 9,
    reference: Some(1_000_000_007),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_007,
        frame: 23,
        sample_count: 9,
        degrees_of_freedom: 7,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 44, including unchanged rejection neighbors.
const SNAPSHOT_044: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 10,
    reference: Some(9_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 9_000_000_000,
        frame: 23,
        sample_count: 10,
        degrees_of_freedom: 8,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 45, including unchanged rejection neighbors.
const SNAPSHOT_045: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 10,
    reference: Some(1_000_000_008),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_008,
        frame: 23,
        sample_count: 10,
        degrees_of_freedom: 8,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 46, including unchanged rejection neighbors.
const SNAPSHOT_046: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 11,
    reference: Some(10_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 10_000_000_000,
        frame: 23,
        sample_count: 11,
        degrees_of_freedom: 9,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 47, including unchanged rejection neighbors.
const SNAPSHOT_047: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 11,
    reference: Some(1_000_000_009),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_009,
        frame: 23,
        sample_count: 11,
        degrees_of_freedom: 9,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 48, including unchanged rejection neighbors.
const SNAPSHOT_048: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 12,
    reference: Some(11_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 11_000_000_000,
        frame: 23,
        sample_count: 12,
        degrees_of_freedom: 10,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 49, including unchanged rejection neighbors.
const SNAPSHOT_049: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 12,
    reference: Some(1_000_000_010),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_010,
        frame: 23,
        sample_count: 12,
        degrees_of_freedom: 10,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 50, including unchanged rejection neighbors.
const SNAPSHOT_050: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 13,
    reference: Some(12_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071, ROW_072,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 12_000_000_000,
        frame: 23,
        sample_count: 13,
        degrees_of_freedom: 11,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 51, including unchanged rejection neighbors.
const SNAPSHOT_051: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 13,
    reference: Some(1_000_000_011),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_011,
        frame: 23,
        sample_count: 13,
        degrees_of_freedom: 11,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 52, including unchanged rejection neighbors.
const SNAPSHOT_052: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 14,
    reference: Some(13_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071, ROW_072, ROW_073,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 13_000_000_000,
        frame: 23,
        sample_count: 14,
        degrees_of_freedom: 12,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 53, including unchanged rejection neighbors.
const SNAPSHOT_053: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 14,
    reference: Some(1_000_000_012),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_012,
        frame: 23,
        sample_count: 14,
        degrees_of_freedom: 12,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 54, including unchanged rejection neighbors.
const SNAPSHOT_054: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 15,
    reference: Some(14_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071, ROW_072, ROW_073, ROW_074,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 14_000_000_000,
        frame: 23,
        sample_count: 15,
        degrees_of_freedom: 13,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 55, including unchanged rejection neighbors.
const SNAPSHOT_055: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 15,
    reference: Some(1_000_000_013),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_013,
        frame: 23,
        sample_count: 15,
        degrees_of_freedom: 13,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 56, including unchanged rejection neighbors.
const SNAPSHOT_056: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 16,
    reference: Some(15_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071, ROW_072, ROW_073, ROW_074, ROW_075,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 15_000_000_000,
        frame: 23,
        sample_count: 16,
        degrees_of_freedom: 14,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 57, including unchanged rejection neighbors.
const SNAPSHOT_057: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 16,
    reference: Some(1_000_000_014),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_014,
        frame: 23,
        sample_count: 16,
        degrees_of_freedom: 14,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 58, including unchanged rejection neighbors.
const SNAPSHOT_058: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 17,
    reference: Some(16_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071, ROW_072, ROW_073, ROW_074, ROW_075, ROW_076,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 16_000_000_000,
        frame: 23,
        sample_count: 17,
        degrees_of_freedom: 15,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 59, including unchanged rejection neighbors.
const SNAPSHOT_059: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 17,
    reference: Some(1_000_000_015),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_015,
        frame: 23,
        sample_count: 17,
        degrees_of_freedom: 15,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 60, including unchanged rejection neighbors.
const SNAPSHOT_060: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 18,
    reference: Some(17_000_000_000),
    rows: &[
        ROW_001, ROW_004, ROW_084, ROW_086, ROW_088, ROW_099, ROW_100, ROW_104, ROW_105, ROW_106,
        ROW_006, ROW_071, ROW_072, ROW_073, ROW_074, ROW_075, ROW_076, ROW_077,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 17_000_000_000,
        frame: 23,
        sample_count: 18,
        degrees_of_freedom: 16,
        bits: [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 61, including unchanged rejection neighbors.
const SNAPSHOT_061: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 18,
    reference: Some(1_000_000_016),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_016,
        frame: 23,
        sample_count: 18,
        degrees_of_freedom: 16,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 62, including unchanged rejection neighbors.
const SNAPSHOT_062: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 19,
    reference: Some(1_000_000_017),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_017,
        frame: 23,
        sample_count: 19,
        degrees_of_freedom: 17,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 63, including unchanged rejection neighbors.
const SNAPSHOT_063: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 20,
    reference: Some(1_000_000_018),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_018,
        frame: 23,
        sample_count: 20,
        degrees_of_freedom: 18,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 64, including unchanged rejection neighbors.
const SNAPSHOT_064: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 21,
    reference: Some(1_000_000_019),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_019,
        frame: 23,
        sample_count: 21,
        degrees_of_freedom: 19,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 65, including unchanged rejection neighbors.
const SNAPSHOT_065: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 22,
    reference: Some(1_000_000_020),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_020,
        frame: 23,
        sample_count: 22,
        degrees_of_freedom: 20,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 66, including unchanged rejection neighbors.
const SNAPSHOT_066: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 23,
    reference: Some(1_000_000_021),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_021,
        frame: 23,
        sample_count: 23,
        degrees_of_freedom: 21,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 67, including unchanged rejection neighbors.
const SNAPSHOT_067: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 24,
    reference: Some(1_000_000_022),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_022,
        frame: 23,
        sample_count: 24,
        degrees_of_freedom: 22,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 68, including unchanged rejection neighbors.
const SNAPSHOT_068: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 25,
    reference: Some(1_000_000_023),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_023,
        frame: 23,
        sample_count: 25,
        degrees_of_freedom: 23,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 69, including unchanged rejection neighbors.
const SNAPSHOT_069: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 26,
    reference: Some(1_000_000_024),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_024,
        frame: 23,
        sample_count: 26,
        degrees_of_freedom: 24,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 70, including unchanged rejection neighbors.
const SNAPSHOT_070: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 27,
    reference: Some(1_000_000_025),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_025,
        frame: 23,
        sample_count: 27,
        degrees_of_freedom: 25,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 71, including unchanged rejection neighbors.
const SNAPSHOT_071: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 28,
    reference: Some(1_000_000_026),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_026,
        frame: 23,
        sample_count: 28,
        degrees_of_freedom: 26,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 72, including unchanged rejection neighbors.
const SNAPSHOT_072: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 29,
    reference: Some(1_000_000_027),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_027,
        frame: 23,
        sample_count: 29,
        degrees_of_freedom: 27,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 73, including unchanged rejection neighbors.
const SNAPSHOT_073: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 30,
    reference: Some(1_000_000_028),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_028,
        frame: 23,
        sample_count: 30,
        degrees_of_freedom: 28,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 74, including unchanged rejection neighbors.
const SNAPSHOT_074: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 31,
    reference: Some(1_000_000_029),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_029,
        frame: 23,
        sample_count: 31,
        degrees_of_freedom: 29,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 75, including unchanged rejection neighbors.
const SNAPSHOT_075: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 32,
    reference: Some(1_000_000_030),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_030,
        frame: 23,
        sample_count: 32,
        degrees_of_freedom: 30,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 76, including unchanged rejection neighbors.
const SNAPSHOT_076: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 33,
    reference: Some(1_000_000_031),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_031,
        frame: 23,
        sample_count: 33,
        degrees_of_freedom: 31,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 77, including unchanged rejection neighbors.
const SNAPSHOT_077: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 34,
    reference: Some(1_000_000_032),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_032,
        frame: 23,
        sample_count: 34,
        degrees_of_freedom: 32,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 78, including unchanged rejection neighbors.
const SNAPSHOT_078: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 35,
    reference: Some(1_000_000_033),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_033,
        frame: 23,
        sample_count: 35,
        degrees_of_freedom: 33,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 79, including unchanged rejection neighbors.
const SNAPSHOT_079: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 36,
    reference: Some(1_000_000_034),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_034,
        frame: 23,
        sample_count: 36,
        degrees_of_freedom: 34,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 80, including unchanged rejection neighbors.
const SNAPSHOT_080: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 37,
    reference: Some(1_000_000_035),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_035,
        frame: 23,
        sample_count: 37,
        degrees_of_freedom: 35,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 81, including unchanged rejection neighbors.
const SNAPSHOT_081: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 38,
    reference: Some(1_000_000_036),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_036,
        frame: 23,
        sample_count: 38,
        degrees_of_freedom: 36,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 82, including unchanged rejection neighbors.
const SNAPSHOT_082: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 39,
    reference: Some(1_000_000_037),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_037,
        frame: 23,
        sample_count: 39,
        degrees_of_freedom: 37,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 83, including unchanged rejection neighbors.
const SNAPSHOT_083: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 40,
    reference: Some(1_000_000_038),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_038,
        frame: 23,
        sample_count: 40,
        degrees_of_freedom: 38,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 84, including unchanged rejection neighbors.
const SNAPSHOT_084: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 41,
    reference: Some(1_000_000_039),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_039,
        frame: 23,
        sample_count: 41,
        degrees_of_freedom: 39,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 85, including unchanged rejection neighbors.
const SNAPSHOT_085: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 42,
    reference: Some(1_000_000_040),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_040,
        frame: 23,
        sample_count: 42,
        degrees_of_freedom: 40,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 86, including unchanged rejection neighbors.
const SNAPSHOT_086: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 43,
    reference: Some(1_000_000_041),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_041,
        frame: 23,
        sample_count: 43,
        degrees_of_freedom: 41,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 87, including unchanged rejection neighbors.
const SNAPSHOT_087: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 44,
    reference: Some(1_000_000_042),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_042,
        frame: 23,
        sample_count: 44,
        degrees_of_freedom: 42,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 88, including unchanged rejection neighbors.
const SNAPSHOT_088: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 45,
    reference: Some(1_000_000_043),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_043,
        frame: 23,
        sample_count: 45,
        degrees_of_freedom: 43,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 89, including unchanged rejection neighbors.
const SNAPSHOT_089: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 46,
    reference: Some(1_000_000_044),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_044,
        frame: 23,
        sample_count: 46,
        degrees_of_freedom: 44,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 90, including unchanged rejection neighbors.
const SNAPSHOT_090: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 47,
    reference: Some(1_000_000_045),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_045,
        frame: 23,
        sample_count: 47,
        degrees_of_freedom: 45,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 91, including unchanged rejection neighbors.
const SNAPSHOT_091: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 48,
    reference: Some(1_000_000_046),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_046,
        frame: 23,
        sample_count: 48,
        degrees_of_freedom: 46,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 92, including unchanged rejection neighbors.
const SNAPSHOT_092: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 49,
    reference: Some(1_000_000_047),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_047,
        frame: 23,
        sample_count: 49,
        degrees_of_freedom: 47,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 93, including unchanged rejection neighbors.
const SNAPSHOT_093: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 50,
    reference: Some(1_000_000_048),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_048,
        frame: 23,
        sample_count: 50,
        degrees_of_freedom: 48,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 94, including unchanged rejection neighbors.
const SNAPSHOT_094: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 51,
    reference: Some(1_000_000_049),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_049,
        frame: 23,
        sample_count: 51,
        degrees_of_freedom: 49,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 95, including unchanged rejection neighbors.
const SNAPSHOT_095: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 52,
    reference: Some(1_000_000_050),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_050,
        frame: 23,
        sample_count: 52,
        degrees_of_freedom: 50,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 96, including unchanged rejection neighbors.
const SNAPSHOT_096: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 53,
    reference: Some(1_000_000_051),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_051,
        frame: 23,
        sample_count: 53,
        degrees_of_freedom: 51,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 97, including unchanged rejection neighbors.
const SNAPSHOT_097: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 54,
    reference: Some(1_000_000_052),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_052,
        frame: 23,
        sample_count: 54,
        degrees_of_freedom: 52,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 98, including unchanged rejection neighbors.
const SNAPSHOT_098: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 55,
    reference: Some(1_000_000_053),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_053,
        frame: 23,
        sample_count: 55,
        degrees_of_freedom: 53,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 99, including unchanged rejection neighbors.
const SNAPSHOT_099: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 56,
    reference: Some(1_000_000_054),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_054,
        frame: 23,
        sample_count: 56,
        degrees_of_freedom: 54,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 100, including unchanged rejection neighbors.
const SNAPSHOT_100: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 57,
    reference: Some(1_000_000_055),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_055,
        frame: 23,
        sample_count: 57,
        degrees_of_freedom: 55,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 101, including unchanged rejection neighbors.
const SNAPSHOT_101: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 58,
    reference: Some(1_000_000_056),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_056,
        frame: 23,
        sample_count: 58,
        degrees_of_freedom: 56,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 102, including unchanged rejection neighbors.
const SNAPSHOT_102: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 59,
    reference: Some(1_000_000_057),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_057,
        frame: 23,
        sample_count: 59,
        degrees_of_freedom: 57,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 103, including unchanged rejection neighbors.
const SNAPSHOT_103: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 60,
    reference: Some(1_000_000_058),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063, ROW_064,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_058,
        frame: 23,
        sample_count: 60,
        degrees_of_freedom: 58,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 104, including unchanged rejection neighbors.
const SNAPSHOT_104: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 61,
    reference: Some(1_000_000_059),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063, ROW_064,
        ROW_065,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_059,
        frame: 23,
        sample_count: 61,
        degrees_of_freedom: 59,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 105, including unchanged rejection neighbors.
const SNAPSHOT_105: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 62,
    reference: Some(1_000_000_060),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063, ROW_064,
        ROW_065, ROW_066,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_060,
        frame: 23,
        sample_count: 62,
        degrees_of_freedom: 60,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 106, including unchanged rejection neighbors.
const SNAPSHOT_106: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 63,
    reference: Some(1_000_000_061),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063, ROW_064,
        ROW_065, ROW_066, ROW_067,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_061,
        frame: 23,
        sample_count: 63,
        degrees_of_freedom: 61,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 107, including unchanged rejection neighbors.
const SNAPSHOT_107: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 64,
    reference: Some(1_000_000_062),
    rows: &[
        ROW_002, ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014,
        ROW_015, ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024,
        ROW_025, ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034,
        ROW_035, ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044,
        ROW_045, ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054,
        ROW_055, ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063, ROW_064,
        ROW_065, ROW_066, ROW_067, ROW_068,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_000_000_062,
        frame: 23,
        sample_count: 64,
        degrees_of_freedom: 62,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 108, including unchanged rejection neighbors.
const SNAPSHOT_108: SnapshotFact = SnapshotFact {
    context: [17, 23, 29],
    sample_count: 64,
    reference: Some(1_062_500_000),
    rows: &[
        ROW_005, ROW_007, ROW_008, ROW_009, ROW_010, ROW_011, ROW_012, ROW_013, ROW_014, ROW_015,
        ROW_016, ROW_017, ROW_018, ROW_019, ROW_020, ROW_021, ROW_022, ROW_023, ROW_024, ROW_025,
        ROW_026, ROW_027, ROW_028, ROW_029, ROW_030, ROW_031, ROW_032, ROW_033, ROW_034, ROW_035,
        ROW_036, ROW_037, ROW_038, ROW_039, ROW_040, ROW_041, ROW_042, ROW_043, ROW_044, ROW_045,
        ROW_046, ROW_047, ROW_048, ROW_049, ROW_050, ROW_051, ROW_052, ROW_053, ROW_054, ROW_055,
        ROW_056, ROW_057, ROW_058, ROW_059, ROW_060, ROW_061, ROW_062, ROW_063, ROW_064, ROW_065,
        ROW_066, ROW_067, ROW_068, ROW_070,
    ],
    report: Some(ReportFact {
        context: [17, 23, 29],
        reference: 1_062_500_000,
        frame: 23,
        sample_count: 64,
        degrees_of_freedom: 62,
        bits: [
            0x3fd0_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ],
    }),
};
/// Frozen complete state 109, including unchanged rejection neighbors.
const SNAPSHOT_109: SnapshotFact = SnapshotFact {
    context: [
        18_446_744_073_709_551_615,
        18_446_744_073_709_551_614,
        18_446_744_073_709_551_613,
    ],
    sample_count: 0,
    reference: None,
    rows: &[],
    report: None,
};

/// All16 frozen streams; the coexisting pair is executed together, never replayed.
pub(super) const TRACES: &[TraceFact] = &[
    TraceFact {
        name: "empty_immutable_endpoint_labels",
        contexts: &[[
            18_446_744_073_709_551_615,
            18_446_744_073_709_551_614,
            18_446_744_073_709_551_613,
        ]],
        initial: &SNAPSHOT_109,
        steps: &[],
    },
    TraceFact {
        name: "partial_guards_and_labels",
        contexts: &[[17, 23, 29], [19, 23, 29], [17, 31, 29], [17, 23, 31]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_004,
                expected: Expected::Commit,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "source_before_duplicate",
                candidate: ROW_109,
                expected: Expected::ContextMismatch,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "frame_before_older",
                candidate: ROW_108,
                expected: Expected::ContextMismatch,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "generation_before_newer",
                candidate: ROW_107,
                expected: Expected::ContextMismatch,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "duplicate",
                candidate: ROW_004,
                expected: Expected::DuplicateTime,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "older",
                candidate: ROW_001,
                expected: Expected::OutOfOrderTime,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "healthy_third",
                candidate: ROW_084,
                expected: Expected::Commit,
                after: &SNAPSHOT_019,
            },
        ],
    },
    TraceFact {
        name: "origin_zero_positive_quarter",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_002,
                expected: Expected::Commit,
                after: &SNAPSHOT_003,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_005,
                expected: Expected::Commit,
                after: &SNAPSHOT_013,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_085,
                expected: Expected::Commit,
                after: &SNAPSHOT_024,
            },
        ],
    },
    TraceFact {
        name: "origin_u64max_positive_quarter",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_078,
                expected: Expected::Commit,
                after: &SNAPSHOT_004,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_079,
                expected: Expected::Commit,
                after: &SNAPSHOT_014,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_082,
                expected: Expected::Commit,
                after: &SNAPSHOT_025,
            },
        ],
    },
    TraceFact {
        name: "u64max_single_then_rejections",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "first_at_max",
                candidate: ROW_081,
                expected: Expected::Commit,
                after: &SNAPSHOT_005,
            },
            StepFact {
                name: "duplicate_max",
                candidate: ROW_081,
                expected: Expected::DuplicateTime,
                after: &SNAPSHOT_005,
            },
            StepFact {
                name: "older_max",
                candidate: ROW_080,
                expected: Expected::OutOfOrderTime,
                after: &SNAPSHOT_005,
            },
        ],
    },
    TraceFact {
        name: "minimum_span_neighbors",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_087,
                expected: Expected::Commit,
                after: &SNAPSHOT_010,
            },
            StepFact {
                name: "below",
                candidate: ROW_101,
                expected: Expected::SpanTooShort,
                after: &SNAPSHOT_010,
            },
            StepFact {
                name: "exact",
                candidate: ROW_102,
                expected: Expected::Commit,
                after: &SNAPSHOT_021,
            },
            StepFact {
                name: "above",
                candidate: ROW_103,
                expected: Expected::Commit,
                after: &SNAPSHOT_032,
            },
        ],
    },
    TraceFact {
        name: "two_row_maximum_neighbors",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "first",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "above",
                candidate: ROW_097,
                expected: Expected::ElapsedRange { index: 0 },
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "below",
                candidate: ROW_094,
                expected: Expected::Commit,
                after: &SNAPSHOT_011,
            },
            StepFact {
                name: "exact_third",
                candidate: ROW_095,
                expected: Expected::Commit,
                after: &SNAPSHOT_022,
            },
        ],
    },
    TraceFact {
        name: "two_row_exact_maximum_no_prune",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "first",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "exact_second",
                candidate: ROW_095,
                expected: Expected::Commit,
                after: &SNAPSHOT_012,
            },
            StepFact {
                name: "above_third",
                candidate: ROW_097,
                expected: Expected::ElapsedRange { index: 0 },
                after: &SNAPSHOT_012,
            },
        ],
    },
    TraceFact {
        name: "report_window_failure_then_neighbor",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_004,
                expected: Expected::Commit,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_084,
                expected: Expected::Commit,
                after: &SNAPSHOT_019,
            },
            StepFact {
                name: "window_failure",
                candidate: ROW_097,
                expected: Expected::ElapsedRange { index: 0 },
                after: &SNAPSHOT_019,
            },
            StepFact {
                name: "healthy_earlier_than_rejected_attempt",
                candidate: ROW_095,
                expected: Expected::Commit,
                after: &SNAPSHOT_030,
            },
        ],
    },
    TraceFact {
        name: "full64_replace_and_failures",
        contexts: &[[17, 23, 29], [19, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_002,
                expected: Expected::Commit,
                after: &SNAPSHOT_003,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_005,
                expected: Expected::Commit,
                after: &SNAPSHOT_013,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_007,
                expected: Expected::Commit,
                after: &SNAPSHOT_023,
            },
            StepFact {
                name: "append_3",
                candidate: ROW_008,
                expected: Expected::Commit,
                after: &SNAPSHOT_033,
            },
            StepFact {
                name: "append_4",
                candidate: ROW_009,
                expected: Expected::Commit,
                after: &SNAPSHOT_035,
            },
            StepFact {
                name: "append_5",
                candidate: ROW_010,
                expected: Expected::Commit,
                after: &SNAPSHOT_037,
            },
            StepFact {
                name: "append_6",
                candidate: ROW_011,
                expected: Expected::Commit,
                after: &SNAPSHOT_039,
            },
            StepFact {
                name: "append_7",
                candidate: ROW_012,
                expected: Expected::Commit,
                after: &SNAPSHOT_041,
            },
            StepFact {
                name: "append_8",
                candidate: ROW_013,
                expected: Expected::Commit,
                after: &SNAPSHOT_043,
            },
            StepFact {
                name: "append_9",
                candidate: ROW_014,
                expected: Expected::Commit,
                after: &SNAPSHOT_045,
            },
            StepFact {
                name: "append_10",
                candidate: ROW_015,
                expected: Expected::Commit,
                after: &SNAPSHOT_047,
            },
            StepFact {
                name: "append_11",
                candidate: ROW_016,
                expected: Expected::Commit,
                after: &SNAPSHOT_049,
            },
            StepFact {
                name: "append_12",
                candidate: ROW_017,
                expected: Expected::Commit,
                after: &SNAPSHOT_051,
            },
            StepFact {
                name: "append_13",
                candidate: ROW_018,
                expected: Expected::Commit,
                after: &SNAPSHOT_053,
            },
            StepFact {
                name: "append_14",
                candidate: ROW_019,
                expected: Expected::Commit,
                after: &SNAPSHOT_055,
            },
            StepFact {
                name: "append_15",
                candidate: ROW_020,
                expected: Expected::Commit,
                after: &SNAPSHOT_057,
            },
            StepFact {
                name: "append_16",
                candidate: ROW_021,
                expected: Expected::Commit,
                after: &SNAPSHOT_059,
            },
            StepFact {
                name: "append_17",
                candidate: ROW_022,
                expected: Expected::Commit,
                after: &SNAPSHOT_061,
            },
            StepFact {
                name: "append_18",
                candidate: ROW_023,
                expected: Expected::Commit,
                after: &SNAPSHOT_062,
            },
            StepFact {
                name: "append_19",
                candidate: ROW_024,
                expected: Expected::Commit,
                after: &SNAPSHOT_063,
            },
            StepFact {
                name: "append_20",
                candidate: ROW_025,
                expected: Expected::Commit,
                after: &SNAPSHOT_064,
            },
            StepFact {
                name: "append_21",
                candidate: ROW_026,
                expected: Expected::Commit,
                after: &SNAPSHOT_065,
            },
            StepFact {
                name: "append_22",
                candidate: ROW_027,
                expected: Expected::Commit,
                after: &SNAPSHOT_066,
            },
            StepFact {
                name: "append_23",
                candidate: ROW_028,
                expected: Expected::Commit,
                after: &SNAPSHOT_067,
            },
            StepFact {
                name: "append_24",
                candidate: ROW_029,
                expected: Expected::Commit,
                after: &SNAPSHOT_068,
            },
            StepFact {
                name: "append_25",
                candidate: ROW_030,
                expected: Expected::Commit,
                after: &SNAPSHOT_069,
            },
            StepFact {
                name: "append_26",
                candidate: ROW_031,
                expected: Expected::Commit,
                after: &SNAPSHOT_070,
            },
            StepFact {
                name: "append_27",
                candidate: ROW_032,
                expected: Expected::Commit,
                after: &SNAPSHOT_071,
            },
            StepFact {
                name: "append_28",
                candidate: ROW_033,
                expected: Expected::Commit,
                after: &SNAPSHOT_072,
            },
            StepFact {
                name: "append_29",
                candidate: ROW_034,
                expected: Expected::Commit,
                after: &SNAPSHOT_073,
            },
            StepFact {
                name: "append_30",
                candidate: ROW_035,
                expected: Expected::Commit,
                after: &SNAPSHOT_074,
            },
            StepFact {
                name: "append_31",
                candidate: ROW_036,
                expected: Expected::Commit,
                after: &SNAPSHOT_075,
            },
            StepFact {
                name: "append_32",
                candidate: ROW_037,
                expected: Expected::Commit,
                after: &SNAPSHOT_076,
            },
            StepFact {
                name: "append_33",
                candidate: ROW_038,
                expected: Expected::Commit,
                after: &SNAPSHOT_077,
            },
            StepFact {
                name: "append_34",
                candidate: ROW_039,
                expected: Expected::Commit,
                after: &SNAPSHOT_078,
            },
            StepFact {
                name: "append_35",
                candidate: ROW_040,
                expected: Expected::Commit,
                after: &SNAPSHOT_079,
            },
            StepFact {
                name: "append_36",
                candidate: ROW_041,
                expected: Expected::Commit,
                after: &SNAPSHOT_080,
            },
            StepFact {
                name: "append_37",
                candidate: ROW_042,
                expected: Expected::Commit,
                after: &SNAPSHOT_081,
            },
            StepFact {
                name: "append_38",
                candidate: ROW_043,
                expected: Expected::Commit,
                after: &SNAPSHOT_082,
            },
            StepFact {
                name: "append_39",
                candidate: ROW_044,
                expected: Expected::Commit,
                after: &SNAPSHOT_083,
            },
            StepFact {
                name: "append_40",
                candidate: ROW_045,
                expected: Expected::Commit,
                after: &SNAPSHOT_084,
            },
            StepFact {
                name: "append_41",
                candidate: ROW_046,
                expected: Expected::Commit,
                after: &SNAPSHOT_085,
            },
            StepFact {
                name: "append_42",
                candidate: ROW_047,
                expected: Expected::Commit,
                after: &SNAPSHOT_086,
            },
            StepFact {
                name: "append_43",
                candidate: ROW_048,
                expected: Expected::Commit,
                after: &SNAPSHOT_087,
            },
            StepFact {
                name: "append_44",
                candidate: ROW_049,
                expected: Expected::Commit,
                after: &SNAPSHOT_088,
            },
            StepFact {
                name: "append_45",
                candidate: ROW_050,
                expected: Expected::Commit,
                after: &SNAPSHOT_089,
            },
            StepFact {
                name: "append_46",
                candidate: ROW_051,
                expected: Expected::Commit,
                after: &SNAPSHOT_090,
            },
            StepFact {
                name: "append_47",
                candidate: ROW_052,
                expected: Expected::Commit,
                after: &SNAPSHOT_091,
            },
            StepFact {
                name: "append_48",
                candidate: ROW_053,
                expected: Expected::Commit,
                after: &SNAPSHOT_092,
            },
            StepFact {
                name: "append_49",
                candidate: ROW_054,
                expected: Expected::Commit,
                after: &SNAPSHOT_093,
            },
            StepFact {
                name: "append_50",
                candidate: ROW_055,
                expected: Expected::Commit,
                after: &SNAPSHOT_094,
            },
            StepFact {
                name: "append_51",
                candidate: ROW_056,
                expected: Expected::Commit,
                after: &SNAPSHOT_095,
            },
            StepFact {
                name: "append_52",
                candidate: ROW_057,
                expected: Expected::Commit,
                after: &SNAPSHOT_096,
            },
            StepFact {
                name: "append_53",
                candidate: ROW_058,
                expected: Expected::Commit,
                after: &SNAPSHOT_097,
            },
            StepFact {
                name: "append_54",
                candidate: ROW_059,
                expected: Expected::Commit,
                after: &SNAPSHOT_098,
            },
            StepFact {
                name: "append_55",
                candidate: ROW_060,
                expected: Expected::Commit,
                after: &SNAPSHOT_099,
            },
            StepFact {
                name: "append_56",
                candidate: ROW_061,
                expected: Expected::Commit,
                after: &SNAPSHOT_100,
            },
            StepFact {
                name: "append_57",
                candidate: ROW_062,
                expected: Expected::Commit,
                after: &SNAPSHOT_101,
            },
            StepFact {
                name: "append_58",
                candidate: ROW_063,
                expected: Expected::Commit,
                after: &SNAPSHOT_102,
            },
            StepFact {
                name: "append_59",
                candidate: ROW_064,
                expected: Expected::Commit,
                after: &SNAPSHOT_103,
            },
            StepFact {
                name: "append_60",
                candidate: ROW_065,
                expected: Expected::Commit,
                after: &SNAPSHOT_104,
            },
            StepFact {
                name: "append_61",
                candidate: ROW_066,
                expected: Expected::Commit,
                after: &SNAPSHOT_105,
            },
            StepFact {
                name: "append_62",
                candidate: ROW_067,
                expected: Expected::Commit,
                after: &SNAPSHOT_106,
            },
            StepFact {
                name: "append_63",
                candidate: ROW_068,
                expected: Expected::Commit,
                after: &SNAPSHOT_107,
            },
            StepFact {
                name: "context_before_duplicate_full",
                candidate: ROW_110,
                expected: Expected::ContextMismatch,
                after: &SNAPSHOT_107,
            },
            StepFact {
                name: "duplicate_full",
                candidate: ROW_068,
                expected: Expected::DuplicateTime,
                after: &SNAPSHOT_107,
            },
            StepFact {
                name: "older_full",
                candidate: ROW_067,
                expected: Expected::OutOfOrderTime,
                after: &SNAPSHOT_107,
            },
            StepFact {
                name: "window_failure_full",
                candidate: ROW_098,
                expected: Expected::ElapsedRange { index: 0 },
                after: &SNAPSHOT_107,
            },
            StepFact {
                name: "span_failure_full",
                candidate: ROW_069,
                expected: Expected::SpanTooShort,
                after: &SNAPSHOT_107,
            },
            StepFact {
                name: "healthy_65th",
                candidate: ROW_070,
                expected: Expected::Commit,
                after: &SNAPSHOT_108,
            },
        ],
    },
    TraceFact {
        name: "spread_calculated_below",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "oldest",
                candidate: ROW_090,
                expected: Expected::Commit,
                after: &SNAPSHOT_006,
            },
            StepFact {
                name: "middle",
                candidate: ROW_091,
                expected: Expected::Commit,
                after: &SNAPSHOT_015,
            },
            StepFact {
                name: "newest",
                candidate: ROW_096,
                expected: Expected::InsufficientSpread,
                after: &SNAPSHOT_015,
            },
            StepFact {
                name: "healthy_same_state_neighbor",
                candidate: ROW_095,
                expected: Expected::Commit,
                after: &SNAPSHOT_026,
            },
        ],
    },
    TraceFact {
        name: "spread_calculated_exact",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "oldest",
                candidate: ROW_090,
                expected: Expected::Commit,
                after: &SNAPSHOT_006,
            },
            StepFact {
                name: "middle",
                candidate: ROW_092,
                expected: Expected::Commit,
                after: &SNAPSHOT_016,
            },
            StepFact {
                name: "newest",
                candidate: ROW_096,
                expected: Expected::Commit,
                after: &SNAPSHOT_027,
            },
        ],
    },
    TraceFact {
        name: "spread_calculated_above",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "oldest",
                candidate: ROW_090,
                expected: Expected::Commit,
                after: &SNAPSHOT_006,
            },
            StepFact {
                name: "middle",
                candidate: ROW_093,
                expected: Expected::Commit,
                after: &SNAPSHOT_017,
            },
            StepFact {
                name: "newest",
                candidate: ROW_096,
                expected: Expected::Commit,
                after: &SNAPSHOT_028,
            },
        ],
    },
    TraceFact {
        name: "report_spread_failure_then_neighbor",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_004,
                expected: Expected::Commit,
                after: &SNAPSHOT_008,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_084,
                expected: Expected::Commit,
                after: &SNAPSHOT_019,
            },
            StepFact {
                name: "append_3",
                candidate: ROW_086,
                expected: Expected::Commit,
                after: &SNAPSHOT_029,
            },
            StepFact {
                name: "append_4",
                candidate: ROW_088,
                expected: Expected::Commit,
                after: &SNAPSHOT_034,
            },
            StepFact {
                name: "append_5",
                candidate: ROW_099,
                expected: Expected::Commit,
                after: &SNAPSHOT_036,
            },
            StepFact {
                name: "append_6",
                candidate: ROW_100,
                expected: Expected::Commit,
                after: &SNAPSHOT_038,
            },
            StepFact {
                name: "append_7",
                candidate: ROW_104,
                expected: Expected::Commit,
                after: &SNAPSHOT_040,
            },
            StepFact {
                name: "append_8",
                candidate: ROW_105,
                expected: Expected::Commit,
                after: &SNAPSHOT_042,
            },
            StepFact {
                name: "append_9",
                candidate: ROW_106,
                expected: Expected::Commit,
                after: &SNAPSHOT_044,
            },
            StepFact {
                name: "append_10",
                candidate: ROW_006,
                expected: Expected::Commit,
                after: &SNAPSHOT_046,
            },
            StepFact {
                name: "append_11",
                candidate: ROW_071,
                expected: Expected::Commit,
                after: &SNAPSHOT_048,
            },
            StepFact {
                name: "append_12",
                candidate: ROW_072,
                expected: Expected::Commit,
                after: &SNAPSHOT_050,
            },
            StepFact {
                name: "append_13",
                candidate: ROW_073,
                expected: Expected::Commit,
                after: &SNAPSHOT_052,
            },
            StepFact {
                name: "append_14",
                candidate: ROW_074,
                expected: Expected::Commit,
                after: &SNAPSHOT_054,
            },
            StepFact {
                name: "append_15",
                candidate: ROW_075,
                expected: Expected::Commit,
                after: &SNAPSHOT_056,
            },
            StepFact {
                name: "append_16",
                candidate: ROW_076,
                expected: Expected::Commit,
                after: &SNAPSHOT_058,
            },
            StepFact {
                name: "spread_failure_with_report",
                candidate: ROW_096,
                expected: Expected::InsufficientSpread,
                after: &SNAPSHOT_058,
            },
            StepFact {
                name: "healthy_after_failed_attempt",
                candidate: ROW_077,
                expected: Expected::Commit,
                after: &SNAPSHOT_060,
            },
        ],
    },
    TraceFact {
        name: "changing_data_same_frame_report",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_001,
                expected: Expected::Commit,
                after: &SNAPSHOT_002,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_084,
                expected: Expected::Commit,
                after: &SNAPSHOT_009,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_086,
                expected: Expected::Commit,
                after: &SNAPSHOT_020,
            },
            StepFact {
                name: "nonzero_fresh_summary",
                candidate: ROW_089,
                expected: Expected::Commit,
                after: &SNAPSHOT_031,
            },
        ],
    },
    TraceFact {
        name: "same_frame_negative_quarter",
        contexts: &[[17, 23, 29]],
        initial: &SNAPSHOT_000,
        steps: &[
            StepFact {
                name: "append_0",
                candidate: ROW_000,
                expected: Expected::Commit,
                after: &SNAPSHOT_001,
            },
            StepFact {
                name: "append_1",
                candidate: ROW_003,
                expected: Expected::Commit,
                after: &SNAPSHOT_007,
            },
            StepFact {
                name: "append_2",
                candidate: ROW_083,
                expected: Expected::Commit,
                after: &SNAPSHOT_018,
            },
        ],
    },
];

/// Four additional peer reads use the existing positive history, not a reconstruction.
pub(super) const PAIRED_POSITIVE: &SnapshotFact = &SNAPSHOT_024;
