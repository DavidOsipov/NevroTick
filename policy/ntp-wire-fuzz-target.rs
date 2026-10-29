//! Test-only independent fixed-header and word-state datagram framing oracles.
//! This file is not a product workspace member. Its bounded model consumes
//! four-byte words, independently of the product's whole-field slicing loop.
#![no_main]

use libfuzzer_sys::fuzz_target;
use ntp_wire::{FrameLimits, HeaderError, RawHeader, decode_fixed_header, frame_no_legacy_mac_datagram};

/// Interpret four known available bytes using independent wider arithmetic.
fn word(data: &[u8], start: usize) -> u64 {
    data[start..start + 4]
        .iter()
        .fold(0_u64, |value, byte| value * 256 + u64::from(*byte))
}

/// Independent signed-octet expectation, without the production conversion.
fn signed(byte: u8) -> i16 {
    if byte < 128 {
        i16::from(byte)
    } else {
        i16::from(byte) - 256
    }
}

/// Check exact header fields, independently of datagram/extension admission.
fn check_header(data: &[u8]) {
    let result = decode_fixed_header(data);
    if data.len() != 48 {
        assert_eq!(result, Err(HeaderError::WrongLength));
        return;
    }
    let header = result.expect("an exact header length must decode inertly");
    check_fields(&header, data);
}

/// Check every returned header field against exactly 48 independently read bytes.
fn check_fields(header: &RawHeader, data: &[u8]) {
    assert_eq!(header.leap_indicator(), data[0] / 64);
    assert_eq!(header.version(), (data[0] / 8) % 8);
    assert_eq!(header.mode(), data[0] % 8);
    assert_eq!(header.stratum(), data[1]);
    assert_eq!(i16::from(header.poll()), signed(data[2]));
    assert_eq!(i16::from(header.precision()), signed(data[3]));
    assert_eq!(u64::from(header.root_delay_bits()), word(data, 4));
    assert_eq!(u64::from(header.root_dispersion_bits()), word(data, 8));
    assert_eq!(u64::from(header.reference_id()), word(data, 12));
    for (start, stamp) in [
        (16, header.reference_timestamp()),
        (24, header.origin_timestamp()),
        (32, header.receive_timestamp()),
        (40, header.transmit_timestamp()),
    ] {
        assert_eq!(u64::from(stamp.seconds()), word(data, start));
        assert_eq!(u64::from(stamp.fraction()), word(data, start + 4));
        assert_eq!(stamp.bits(), word(data, start) * (1_u64 << 32) + word(data, start + 4));
    }
}

/// Independent expected byte partitions, with no unbounded model allocation.
struct Partitions {
    starts: [usize; 64],
    ends: [usize; 64],
    count: usize,
}

/// Model the explicit outer grammar one four-byte word at a time.
///
/// Callers supply reviewed limits at or below 4096 bytes/64 fields. At each
/// field boundary, record the declared extent, then count down each body word.
/// Truncated bodies leave a nonzero countdown; the final extent must be >=28.
fn reference(data: &[u8], max_bytes: usize, max_fields: usize) -> Option<Partitions> {
    if data.len() < 48 || data.len() > max_bytes || data.len() % 4 != 0 {
        return None;
    }
    let mut result = Partitions { starts: [0; 64], ends: [0; 64], count: 0 };
    let mut words_left = 0;
    let mut final_length = 0;
    for (index, octets) in data[48..].chunks_exact(4).enumerate() {
        if words_left == 0 {
            if result.count == max_fields {
                return None;
            }
            let length = usize::from(octets[2]) * 256 + usize::from(octets[3]);
            if length < 16 || length % 4 != 0 {
                return None;
            }
            result.starts[result.count] = 48 + index * 4;
            result.ends[result.count] = 48 + index * 4 + length;
            result.count += 1;
            words_left = length / 4;
            final_length = length;
        }
        words_left -= 1;
    }
    if words_left != 0 || (result.count != 0 && final_length < 28) {
        return None;
    }
    Some(result)
}

/// Compare complete acceptance and every returned span with the word-state truth.
fn check_datagram(data: &[u8], max_bytes: usize, max_fields: usize) {
    let limits = FrameLimits::new(max_bytes, max_fields).expect("reviewed model limits");
    let expected = reference(data, max_bytes, max_fields);
    let actual = frame_no_legacy_mac_datagram(data, limits);
    assert_eq!(actual.is_ok(), expected.is_some(), "independent complete datagram grammar");
    if let Ok(frame) = actual {
        let expected = expected.expect("acceptance agreement was checked");
        // The separately built fault control changes only the harness's count
        // observation after a nonempty datagram reaches the real product framer.
        let observed_count = frame.extension_count()
            + usize::from(cfg!(feature = "seeded-defect") && expected.count != 0);
        assert_eq!(observed_count, expected.count, "independent datagram field-count oracle");
        check_fields(frame.header(), &data[..48]);
        for _ in 0..2 {
            let mut fields = frame.extensions();
            for index in 0..expected.count {
                let start = expected.starts[index];
                let end = expected.ends[index];
                let field = fields.next().expect("complete preserved partition");
                assert_eq!(field.bytes(), &data[start..end]);
                assert_eq!(field.bytes().as_ptr(), data[start..end].as_ptr());
                assert_eq!(field.body(), &data[start + 4..end]);
                assert_eq!(usize::from(field.field_type()),
                           usize::from(data[start]) * 256 + usize::from(data[start + 1]));
            }
            assert!(fields.next().is_none());
            assert!(fields.next().is_none());
        }
    }
}

fuzz_target!(|data: &[u8]| {
    check_header(data);
    for (bytes, fields) in [(48, 0), (75, 1), (76, 1), (92, 2), (4096, 64)] {
        check_datagram(data, bytes, fields);
    }
    // A further bounded independent context varies the budgets with raw input.
    let bytes = 48 + usize::from(data.first().copied().unwrap_or(0)) * 15;
    let fields = usize::from(data.get(1).copied().unwrap_or(0)) % 65;
    check_datagram(data, bytes, fields);
});
