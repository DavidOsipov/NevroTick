// SPDX-License-Identifier: GPL-2.0-only
//! Independent bounded oracle for the passive diagnostic v2 decoder.
//!
//! Raw callbacks call the actual decoder exactly once. Only target control
//! features may alter forwarding or a projection sentinel; neither is product
//! behavior or an AddressSanitizer memory-detector control.
#![no_main]

use std::sync::atomic::{AtomicU64, Ordering};

use libfuzzer_sys::fuzz_target;
use windows_platform::diagnostics::{
    DecodeError, DecodedRecord, LifecycleObservationClaim, MetadataClaim, UtcClaim,
};

#[cfg(all(feature = "seeded-defect", feature = "forwarding-defect"))]
compile_error!("codec control features are mutually exclusive");

/// Independent format extent from the accepted contract, not a product constant.
const BYTES: usize = 128;
/// Existing fixed libFuzzer input budget.
const MAX_INPUT: usize = 4096;
/// Bound observations even if a driver exceeds the intended finite run profile.
const MAX_CALLBACKS: u64 = 512;
/// Process-local callback numbering, never producer identity or authority.
static CALLBACKS: AtomicU64 = AtomicU64::new(0);

/// Author one fixed nonsecret control literal without the production encoder.
#[cfg(any(feature = "seeded-defect", feature = "forwarding-defect"))]
const fn literal(event: u8) -> [u8; BYTES] {
    let mut bytes: [u8; BYTES] = [0; BYTES];
    bytes[0] = 2;
    bytes[1] = 1;
    bytes[2] = 1;
    bytes[3] = event;
    bytes[4] = 3;
    bytes[5] = 5;
    bytes[6] = 1;
    bytes[8] = 1;
    bytes
}

/// Exact accepted control stimulus, independent of any producer API.
#[cfg(any(feature = "seeded-defect", feature = "forwarding-defect"))]
const START: [u8; BYTES] = literal(1);
/// Distinct accepted same-length replacement used only by the forwarding fault.
#[cfg(feature = "forwarding-defect")]
const RUNNING: [u8; BYTES] = literal(2);

/// Independently expected error precedence or a complete accepted inert image.
#[derive(Clone, Copy)]
enum Expected {
    Empty,
    Version(u8),
    Length(usize),
    Semantic,
    Accepted([u8; BYTES]),
}

impl Expected {
    /// Closed bounded label; no arbitrary input is formatted.
    const fn class(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Version(_) => "version",
            Self::Length(_) => "length",
            Self::Semantic => "semantic",
            Self::Accepted(_) => "accepted",
        }
    }
}

/// Read a fixed contract-owned in-bounds word; all offsets are literal callers.
fn word<const N: usize>(bytes: &[u8; BYTES], offset: usize) -> [u8; N] {
    let mut value: [u8; N] = [0; N];
    value.copy_from_slice(&bytes[offset..offset + N]);
    value
}

/// Independent little-endian 64-bit wire interpretation.
fn number64(bytes: &[u8; BYTES], offset: usize) -> u64 {
    u64::from_le_bytes(word(bytes, offset))
}

/// Independent little-endian 128-bit wire interpretation.
fn number128(bytes: &[u8; BYTES], offset: usize) -> u128 {
    u128::from_le_bytes(word(bytes, offset))
}

/// Contract-owned event classes and cause intervals, without product catalogue use.
fn classification(id: u8) -> Option<(u8, u8, u8, u8)> {
    match id {
        1 | 2 | 3 | 8 => Some((3, 5, 0, 0)),
        4 => Some((1, 2, 1, 28)),
        5 | 9 => Some((1, 2, 20, 23)),
        6 | 7 => Some((1, 2, 10, 19)),
        _ => None,
    }
}

/// Classify original bytes by independent contract literals before observing output.
fn expected(input: &[u8]) -> Expected {
    let Some(version): Option<&u8> = input.first() else {
        return Expected::Empty;
    };
    if *version != 2 {
        return Expected::Version(*version);
    }
    let Ok(bytes): Result<[u8; BYTES], std::array::TryFromSliceError> = input.try_into() else {
        return Expected::Length(input.len());
    };
    let Some((level, importance, cause_low, cause_high)): Option<(u8, u8, u8, u8)> =
        classification(bytes[3])
    else {
        return Expected::Semantic;
    };
    if bytes[1] != 1
        || !matches!(bytes[2], 1 | 2)
        || bytes[4] != level
        || bytes[5] != importance
        || bytes[6] != 1
        || !(cause_low..=cause_high).contains(&bytes[7])
        || number64(&bytes, 8) == 0
        || bytes[16] > 2
        || (bytes[16] == 0 && number64(&bytes, 17) != 0)
        || bytes[108..128].iter().any(|byte: &u8| *byte != 0)
    {
        return Expected::Semantic;
    }
    for (tag, offset) in [(25usize, 26usize), (78, 79)] {
        let value: u128 = number128(&bytes, offset);
        if !((bytes[tag] == 0 && value == 0) || (bytes[tag] == 1 && value != 0)) {
            return Expected::Semantic;
        }
    }
    for (tag, offset) in [(42usize, 43usize), (51, 52), (60, 61), (69, 70)] {
        let value: u64 = number64(&bytes, offset);
        if !((bytes[tag] == 0 && value == 0) || (bytes[tag] == 1 && value != 0)) {
            return Expected::Semantic;
        }
    }
    let nanos: u32 = u32::from_le_bytes(word(&bytes, 104));
    if !((bytes[95] == 0 && number64(&bytes, 96) == 0 && nanos == 0)
        || (bytes[95] == 1 && nanos < 1_000_000_000))
    {
        return Expected::Semantic;
    }
    Expected::Accepted(bytes)
}

/// Classify the real returned outcome without formatting input/error payloads.
fn actual_class(actual: &Result<DecodedRecord, DecodeError>) -> &'static str {
    match actual {
        Ok(_) => "accepted",
        Err(DecodeError::Empty) => "empty",
        Err(DecodeError::UnsupportedVersion(_)) => "version",
        Err(DecodeError::InvalidLength { .. }) => "length",
        Err(DecodeError::InvalidV2) => "semantic",
        Err(DecodeError::InvalidImportanceMask) => "mask",
    }
}

/// Expected passive metadata is structural only; it confers no issuer authority.
fn claim64(bytes: &[u8; BYTES], tag: usize, offset: usize) -> MetadataClaim<u64> {
    if bytes[tag] == 0 {
        MetadataClaim::Unavailable
    } else {
        MetadataClaim::ObservedClaim(number64(bytes, offset))
    }
}

/// Expected passive wide metadata is never a producer token.
fn claim128(bytes: &[u8; BYTES], tag: usize, offset: usize) -> MetadataClaim<u128> {
    if bytes[tag] == 0 {
        MetadataClaim::Unavailable
    } else {
        MetadataClaim::ObservedClaim(number128(bytes, offset))
    }
}

/// Check every public projection against original independent wire facts.
fn projections(bytes: &[u8; BYTES], decoded: &DecodedRecord) {
    assert!(decoded.bytes() == bytes, "LOGGER_CODEC_ORIGINAL_BYTES");
    assert!(
        decoded.component().code() == bytes[2],
        "LOGGER_CODEC_COMPONENT"
    );
    assert!(decoded.event().id() == bytes[3], "LOGGER_CODEC_EVENT_ID");
    assert!(
        decoded.event().cause_code() == bytes[7],
        "LOGGER_CODEC_EVENT_CAUSE"
    );
    let Some((level, importance, low, high)): Option<(u8, u8, u8, u8)> = classification(bytes[3])
    else {
        panic!("LOGGER_CODEC_ORACLE_ACCEPTED_ID");
    };
    let descriptor: &windows_platform::diagnostics::EventDescriptor = decoded.event().descriptor();
    assert!(descriptor.id() == bytes[3], "LOGGER_CODEC_DESCRIPTOR_ID");
    assert!(descriptor.version() == 2, "LOGGER_CODEC_DESCRIPTOR_VERSION");
    assert!(descriptor.level().code() == level, "LOGGER_CODEC_LEVEL");
    assert!(
        descriptor.importance().code() == importance,
        "LOGGER_CODEC_IMPORTANCE"
    );
    assert!(descriptor.required(), "LOGGER_CODEC_REQUIRED");
    assert!(
        descriptor.optional_duplicate_eligible(),
        "LOGGER_CODEC_OPTIONAL_DUPLICATE"
    );
    for cause in 0u8..=255 {
        assert!(
            descriptor.permits_cause(cause) == (low..=high).contains(&cause),
            "LOGGER_CODEC_DESCRIPTOR_CAUSE_DOMAIN"
        );
    }
    let sequence: u64 = decoded.sequence();
    #[cfg(feature = "seeded-defect")]
    let sequence: u64 = if bytes == &START {
        sequence + 1
    } else {
        sequence
    };
    assert!(
        sequence == number64(bytes, 8),
        "LOGGER_CODEC_PROJECTION_SEQUENCE"
    );
    let elapsed: u64 = number64(bytes, 17);
    let observation: LifecycleObservationClaim = match bytes[16] {
        0 => LifecycleObservationClaim::Unavailable,
        1 => LifecycleObservationClaim::LastAccepted(elapsed),
        2 => LifecycleObservationClaim::HistoricalAfterClockFault(elapsed),
        _ => panic!("LOGGER_CODEC_ORACLE_ACCEPTED_OBSERVATION"),
    };
    assert!(
        decoded.observation() == observation,
        "LOGGER_CODEC_OBSERVATION"
    );
    assert!(
        decoded.process_instance() == claim128(bytes, 25, 26),
        "LOGGER_CODEC_PROCESS"
    );
    assert!(
        decoded.policy_generation() == claim64(bytes, 42, 43),
        "LOGGER_CODEC_POLICY"
    );
    assert!(
        decoded.configuration_generation() == claim64(bytes, 51, 52),
        "LOGGER_CODEC_CONFIGURATION"
    );
    assert!(
        decoded.association_generation() == claim64(bytes, 60, 61),
        "LOGGER_CODEC_ASSOCIATION"
    );
    assert!(
        decoded.clock_generation() == claim64(bytes, 69, 70),
        "LOGGER_CODEC_CLOCK"
    );
    assert!(
        decoded.correlation() == claim128(bytes, 78, 79),
        "LOGGER_CODEC_CORRELATION"
    );
    let utc: UtcClaim = match bytes[95] {
        0 => UtcClaim::Unavailable,
        1 => UtcClaim::SystemWallUnqualified {
            unix_seconds: i64::from_le_bytes(word(bytes, 96)),
            nanoseconds: u32::from_le_bytes(word(bytes, 104)),
        },
        _ => panic!("LOGGER_CODEC_ORACLE_ACCEPTED_UTC"),
    };
    assert!(decoded.utc() == utc, "LOGGER_CODEC_UTC");
}

/// Compare complete acceptance and exact typed errors; no rejection payload is lost.
fn verify(expectation: Expected, actual: Result<DecodedRecord, DecodeError>) {
    match (expectation, actual) {
        (Expected::Empty, Err(DecodeError::Empty))
        | (Expected::Semantic, Err(DecodeError::InvalidV2)) => {}
        (Expected::Version(expected), Err(DecodeError::UnsupportedVersion(actual))) => {
            assert!(actual == expected, "LOGGER_CODEC_VERSION_VALUE");
        }
        (Expected::Length(expected), Err(DecodeError::InvalidLength { actual })) => {
            assert!(actual == expected, "LOGGER_CODEC_LENGTH_VALUE");
        }
        (Expected::Accepted(bytes), Ok(decoded)) => projections(&bytes, &decoded),
        _ => panic!("LOGGER_CODEC_CLASSIFICATION"),
    }
}

fuzz_target!(|input: &[u8]| {
    assert!(input.len() <= MAX_INPUT, "LOGGER_CODEC_INPUT_BUDGET");
    let ordinal: u64 = CALLBACKS.fetch_add(1, Ordering::Relaxed) + 1;
    assert!(ordinal <= MAX_CALLBACKS, "LOGGER_CODEC_CALLBACK_BUDGET");
    let expectation: Expected = expected(input);
    let forwarded: &[u8] = input;
    #[cfg(feature = "forwarding-defect")]
    let forwarded: &[u8] = if input == START.as_slice() {
        &RUNNING
    } else {
        forwarded
    };
    let actual: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(forwarded);
    eprintln!(
        "LOGGER_CODEC_CALLBACK ordinal={} length={} expected={} actual={} calls=1",
        ordinal,
        input.len(),
        expectation.class(),
        actual_class(&actual)
    );
    verify(expectation, actual);
});
