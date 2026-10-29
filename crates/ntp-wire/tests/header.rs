//! Independent raw-input expectations through the public fixed-header decoder.
#![forbid(unsafe_code)]

use ntp_wire::{FIXED_HEADER_LEN, HeaderError, RawHeader, decode_fixed_header};

#[path = "support/vectors.rs"]
mod vectors;

/// Compare every public projection with literal values independent of the decoder.
///
/// The input is a real decoded header; assertions inspect only immutable values.
/// No state, time or I/O effect is introduced. Assertion failure reports the case.
///
/// # Panics
/// Fails when a field does not equal its independently specified wire value.
fn assert_fields(header: RawHeader, expected: vectors::ExpectedHeader, name: &str) {
    assert_eq!(
        (header.leap_indicator(), header.version(), header.mode()),
        expected.flags,
        "{name}"
    );
    assert_eq!(header.stratum(), expected.stratum, "{name}");
    assert_eq!(header.poll(), expected.poll, "{name}");
    assert_eq!(header.precision(), expected.precision, "{name}");
    assert_eq!(header.root_delay_bits(), expected.root_delay, "{name}");
    assert_eq!(
        header.root_dispersion_bits(),
        expected.root_dispersion,
        "{name}"
    );
    assert_eq!(header.reference_id(), expected.reference_id, "{name}");
    for (actual, bits) in [
        header.reference_timestamp(),
        header.origin_timestamp(),
        header.receive_timestamp(),
        header.transmit_timestamp(),
    ]
    .into_iter()
    .zip(expected.timestamps)
    {
        assert_eq!(actual.bits(), bits, "{name}");
        let octets = bits.to_be_bytes();
        assert_eq!(
            actual.seconds(),
            u32::from_be_bytes([octets[0], octets[1], octets[2], octets[3]]),
            "{name}"
        );
        assert_eq!(
            actual.fraction(),
            u32::from_be_bytes([octets[4], octets[5], octets[6], octets[7]]),
            "{name}"
        );
    }
}

/// Every reviewed literal header reaches the actual decoder and immutable consumer.
///
/// # Errors
/// Propagates an unexpected exact-header rejection.
///
/// # Panics
/// Fails when literal framing or raw-field expectations differ.
#[test]
fn reviewed_literal_header_projections() -> Result<(), HeaderError> {
    for case in vectors::CASES {
        if let Some(expected) = case.expected {
            assert_fields(decode_fixed_header(case.packet)?, expected, case.name);
        } else {
            assert_eq!(
                decode_fixed_header(case.packet),
                Err(HeaderError::WrongLength),
                "{}",
                case.name
            );
        }
    }
    Ok(())
}

/// Every short length and selected long neighbors reject before any field access.
///
/// # Panics
/// Fails when a non-header length is accepted.
#[test]
fn every_non_header_length_is_rejected() {
    assert_eq!(FIXED_HEADER_LEN, 48);
    let oversized = [0_u8; 4096];
    for length in 0..48 {
        assert_eq!(
            decode_fixed_header(&oversized[..length]),
            Err(HeaderError::WrongLength),
            "short length {length}"
        );
    }
    for length in [49, 64, 255, 4096] {
        assert_eq!(
            decode_fixed_header(&oversized[..length]),
            Err(HeaderError::WrongLength),
            "long length {length}"
        );
    }
}

/// Exact-length zero bits are a legal inert header, paired with rejection controls.
///
/// # Errors
/// Propagates an unexpected exact-length healthy-control rejection.
///
/// # Panics
/// Fails when a 48-byte representation changes or acquires drop-owned resources.
#[test]
fn exact_length_healthy_neighbor_is_admitted() -> Result<(), HeaderError> {
    let healthy = decode_fixed_header(&[0; 48])?;
    assert_eq!(healthy.reference_timestamp().bits(), 0);
    assert_eq!(healthy.transmit_timestamp().bits(), 0);
    assert!(!core::mem::needs_drop::<RawHeader>());
    Ok(())
}

/// All packed flag patterns are represented without semantic profile admission.
///
/// # Errors
/// Propagates an unexpected fixed-header rejection.
///
/// # Panics
/// Fails on a wrong bit position, dropped bit or semantic rejection.
#[test]
fn every_flag_octet_remains_inert() -> Result<(), HeaderError> {
    for leap in 0..4_u8 {
        for version in 0..8_u8 {
            for mode in 0..8_u8 {
                let mut input = [0_u8; 48];
                input[0] = leap * 64 + version * 8 + mode;
                let header = decode_fixed_header(&input)?;
                assert_eq!(header.leap_indicator(), leap);
                assert_eq!(header.version(), version);
                assert_eq!(header.mode(), mode);
            }
        }
    }
    Ok(())
}

/// All signed exponent values preserve two's-complement meaning independently.
///
/// # Errors
/// Propagates an unexpected fixed-header rejection.
///
/// # Panics
/// Fails on unsigned interpretation, field swapping or endpoint loss.
#[test]
fn signed_exponent_endpoints_and_all_octets() -> Result<(), HeaderError> {
    for expected in i8::MIN..=i8::MAX {
        let mut input = [0_u8; 48];
        input[2] = expected.to_be_bytes()[0];
        input[3] = (!expected).to_be_bytes()[0];
        let header = decode_fixed_header(&input)?;
        assert_eq!(header.poll(), expected);
        assert_eq!(header.precision(), !expected);
    }
    Ok(())
}

/// Distinct byte positions and maximal raw values survive without normalization.
///
/// # Errors
/// Propagates an unexpected fixed-header rejection.
///
/// # Panics
/// Fails on byte-order errors, field aliasing or invalid semantic filtering.
#[test]
fn field_positions_and_full_width_bits() -> Result<(), HeaderError> {
    let input = [
        0xff, 0xfe, 0x80, 0x7f, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x13, 0x57, 0x9b,
        0xdf, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
        0xee, 0xff, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
    ];
    let header = decode_fixed_header(&input)?;
    assert_fields(
        header,
        vectors::ExpectedHeader {
            flags: (3, 7, 7),
            stratum: 254,
            poll: -128,
            precision: 127,
            root_delay: 0x0123_4567,
            root_dispersion: 0x89ab_cdef,
            reference_id: 0x1357_9bdf,
            timestamps: [
                0x0011_2233_4455_6677,
                0x8899_aabb_ccdd_eeff,
                0xfedc_ba98_7654_3210,
                u64::MAX,
            ],
        },
        "independent distinct-position control",
    );
    let maximal = decode_fixed_header(&[0xff; 48])?;
    assert_eq!(maximal.root_delay_bits(), u32::MAX);
    assert_eq!(maximal.root_dispersion_bits(), u32::MAX);
    assert_eq!(maximal.reference_id(), u32::MAX);
    for timestamp in [
        maximal.reference_timestamp(),
        maximal.origin_timestamp(),
        maximal.receive_timestamp(),
        maximal.transmit_timestamp(),
    ] {
        assert_eq!(timestamp.seconds(), u32::MAX);
        assert_eq!(timestamp.fraction(), u32::MAX);
        assert_eq!(timestamp.bits(), u64::MAX);
    }
    Ok(())
}

/// The decoded value owns a fixed copy and outlives changes to the caller's bytes.
///
/// # Errors
/// Propagates an unexpected fixed-header rejection.
///
/// # Panics
/// Fails when a returned value aliases mutable caller input or failure changes it.
#[test]
fn decoded_header_is_an_immutable_owned_snapshot() -> Result<(), HeaderError> {
    let mut input = [0xff; 48];
    let header = decode_fixed_header(&input)?;
    input.fill(0);
    assert_eq!(header.stratum(), 255);
    assert_eq!(header.transmit_timestamp().bits(), u64::MAX);
    assert_eq!(
        decode_fixed_header(&input[..47]),
        Err(HeaderError::WrongLength)
    );
    assert_eq!(header.transmit_timestamp().bits(), u64::MAX);
    assert_eq!(input, [0; 48]);
    Ok(())
}
