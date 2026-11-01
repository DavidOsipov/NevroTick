//! Test-only independent raw framing and pending NTPv4 negotiation oracles.
//! This file is not a product workspace member. The models use word/byte states
//! and record facts independently of production framing/negotiation helpers.
#![no_main]

use libfuzzer_sys::fuzz_target;
use ntp_wire::{
    FrameLimits, HeaderError, NtsKeFrameLimits, RawHeader, decode_fixed_header,
    frame_no_legacy_mac_datagram, frame_nts_ke_message, negotiate_nts_ke_ntpv4,
    read_nts_ke_ntpv4_offer,
};

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
        assert_eq!(
            stamp.bits(),
            word(data, start) * (1_u64 << 32) + word(data, start + 4)
        );
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
    let mut result = Partitions {
        starts: [0; 64],
        ends: [0; 64],
        count: 0,
    };
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
    assert_eq!(
        actual.is_ok(),
        expected.is_some(),
        "independent complete datagram grammar"
    );
    if let Ok(frame) = actual {
        let expected = expected.expect("acceptance agreement was checked");
        // The separately built fault control changes only the harness's count
        // observation after a nonempty datagram reaches the real product framer.
        let observed_count = frame.extension_count()
            + usize::from(cfg!(feature = "seeded-defect") && expected.count != 0);
        assert_eq!(
            observed_count, expected.count,
            "independent datagram field-count oracle"
        );
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
                assert_eq!(
                    usize::from(field.field_type()),
                    usize::from(data[start]) * 256 + usize::from(data[start + 1])
                );
            }
            assert!(fields.next().is_none());
            assert!(fields.next().is_none());
        }
    }
}

/// Independent byte-at-a-time KE envelope automaton; no production splitting.
/// Header bytes accumulate separately from a body-byte countdown. No model heap.
fn ke_reference(data: &[u8], maximum: usize, records: usize) -> Option<usize> {
    if data.len() > maximum {
        return None;
    }
    let mut header = [0_u8; 4];
    let mut position = 0;
    let mut body_left = 0_usize;
    let mut count = 0;
    let mut ended = false;
    for (index, byte) in data.iter().copied().enumerate() {
        if body_left != 0 {
            body_left -= 1;
            continue;
        }
        if position == 0 && count == records {
            return None;
        }
        header[position] = byte;
        position += 1;
        if position != 4 {
            continue;
        }
        position = 0;
        count += 1;
        let kind = usize::from(header[0] % 128) * 256 + usize::from(header[1]);
        body_left = usize::from(header[2]) * 256 + usize::from(header[3]);
        if kind == 0 {
            if header[0] < 128 || body_left != 0 || index + 1 != data.len() {
                return None;
            }
            ended = true;
        }
    }
    if ended && position == 0 && body_left == 0 {
        Some(count)
    } else {
        None
    }
}

/// Compare actual acceptance, borrowed original spans and exact header facts.
fn check_ke(data: &[u8], maximum: usize, records: usize) {
    let expected = ke_reference(data, maximum, records);
    let limits = NtsKeFrameLimits::new(maximum, records).expect("explicit KE experiment budgets");
    let actual = frame_nts_ke_message(data, limits);
    assert_eq!(
        actual.is_ok(),
        expected.is_some(),
        "independent NTS-KE envelope/EOM oracle"
    );
    if let Ok(message) = actual {
        let count = expected.expect("independent acceptance was checked");
        // The reached fault changes only the observation after real admission.
        let observed = message.record_count() + usize::from(cfg!(feature = "seeded-ke-defect"));
        assert_eq!(observed, count, "independent NTS-KE record-count oracle");
        assert_eq!(message.bytes().as_ptr(), data.as_ptr());
        assert_eq!(message.bytes().len(), data.len());
        let mut cursor = 0;
        let mut items = message.records();
        for _ in 0..count {
            let body = usize::from(data[cursor + 2]) * 256 + usize::from(data[cursor + 3]);
            let end = cursor + 4 + body;
            let record = items.next().expect("complete independent KE partition");
            assert_eq!(record.bytes().as_ptr(), data[cursor..end].as_ptr());
            assert_eq!(record.bytes(), &data[cursor..end]);
            assert_eq!(record.body().as_ptr(), data[cursor + 4..end].as_ptr());
            assert_eq!(record.body(), &data[cursor + 4..end]);
            assert_eq!(
                usize::from(record.record_type()),
                usize::from(data[cursor] % 128) * 256 + usize::from(data[cursor + 1])
            );
            assert_eq!(record.critical(), data[cursor] >= 128);
            cursor = end;
        }
        assert_eq!(cursor, data.len());
        assert!(items.next().is_none());
        assert!(items.next().is_none());
    }
}

/// Visit complete body ranges using the independent byte-envelope automaton.
///
/// A rejected envelope produces no usable model result. The prior byte-state
/// check bounds every emitted range and the record count; callbacks inspect
/// input facts only and cannot construct expected values from production views.
fn visit_model_records(data: &[u8], mut visit: impl FnMut(usize, bool, usize, usize)) -> bool {
    if ke_reference(data, 131_072, 32_768).is_none() {
        return false;
    }
    let mut prefix = [0_u8; 4];
    let mut prefix_used = 0;
    let mut left = 0;
    let mut kind = 0;
    let mut critical = false;
    let mut body_start = 0;
    for (index, byte) in data.iter().copied().enumerate() {
        if left != 0 {
            left -= 1;
            if left == 0 {
                visit(kind, critical, body_start, index + 1);
            }
            continue;
        }
        prefix[prefix_used] = byte;
        prefix_used += 1;
        if prefix_used == 4 {
            prefix_used = 0;
            kind = usize::from(prefix[0] % 128) * 256 + usize::from(prefix[1]);
            critical = prefix[0] >= 128;
            left = usize::from(prefix[2]) * 256 + usize::from(prefix[3]);
            body_start = index + 1;
            if left == 0 {
                visit(kind, critical, body_start, body_start);
            }
        }
    }
    true
}

/// Independent record facts retained in constant space, never a live association.
struct NegotiationReference {
    aead_start: usize,
    aead_end: usize,
    cookies: usize,
    server: Option<(usize, usize)>,
    port: Option<u16>,
}

/// Derive success from record facts and the documented NTPv4-only candidate rules.
///
/// RFC 8915 supplies criticality, direction, parity, selection and cookie rules.
/// The candidate additionally rejects ambiguous duplicate ports and recognizes
/// only protocol zero and a limited pending ASCII endpoint shape. Response
/// expectations bind to the independent literal request offering IDs 15 and 16.
/// Counts and body facts are evaluated after a byte-state traversal; no production
/// scanner, iterator, constructor, formatter or error text supplies expected truth.
fn negotiation_reference(data: &[u8], request: bool) -> Option<NegotiationReference> {
    let mut counts = [0_usize; 8];
    let mut good_shape = true;
    let mut protocol_length = 0;
    let mut protocol_nonzero = false;
    let mut abort = false;
    let mut result = NegotiationReference {
        aead_start: 0,
        aead_end: 0,
        cookies: 0,
        server: None,
        port: None,
    };
    let complete = visit_model_records(data, |kind, critical, start, end| {
        let body = &data[start..end];
        if kind < counts.len() {
            counts[kind] += 1;
        } else if critical {
            good_shape = false;
        }
        if request && [2, 3, 5].contains(&kind) {
            good_shape = false;
        }
        match kind {
            1 => {
                good_shape &= critical && body.len() % 2 == 0;
                protocol_length = body.len();
                protocol_nonzero |= body.iter().any(|byte| *byte != 0);
            }
            2 | 3 => {
                good_shape &= critical && body.len() == 2;
                abort = true;
            }
            4 => {
                good_shape &= body.len() % 2 == 0 && (request || body.len() <= 2);
                result.aead_start = start;
                result.aead_end = end;
            }
            5 => result.cookies += 1,
            6 => {
                good_shape &= !body.is_empty()
                    && body.iter().all(|byte| {
                        *byte > 32
                            && *byte < 127
                            && ![b'/', b'\\', b'@', b'%', b'[', b']'].contains(byte)
                    });
                result.server = Some((start, end));
            }
            7 => {
                good_shape &= body.len() == 2;
                if body.len() == 2 {
                    result.port = Some(u16::from(body[0]) * 256 + u16::from(body[1]));
                }
            }
            _ => {}
        }
    });
    if !complete
        || !good_shape
        || counts[1] != 1
        || counts[4] != 1
        || counts[6] > 1
        || counts[7] > 1
        || protocol_length == 0
        || protocol_nonzero
    {
        return None;
    }
    let aeads = &data[result.aead_start..result.aead_end];
    if request {
        if aeads.is_empty() {
            return None;
        }
    } else if abort
        || aeads.len() != 2
        || result.cookies == 0
        || aeads[0] != 0
        || ![15, 16].contains(&aeads[1])
    {
        return None;
    }
    Some(result)
}

/// Compare both real consumers against independent accepted/rejected record facts.
///
/// Each input is used unchanged as an actual request and as an actual response.
/// Response context comes from the real request constructor over a fixed literal
/// offer, never an unchecked test factory. All results remain pending/inert.
fn check_negotiation(data: &[u8]) {
    let expected_request = negotiation_reference(data, true);
    let expected_response = negotiation_reference(data, false);
    let limits = NtsKeFrameLimits::new(131_072, 32_768).expect("explicit semantic fuzz limits");
    let Ok(message) = frame_nts_ke_message(data, limits) else {
        assert!(expected_request.is_none());
        assert!(expected_response.is_none());
        return;
    };
    let actual_request = read_nts_ke_ntpv4_offer(&message);
    assert_eq!(
        actual_request.is_ok(),
        expected_request.is_some(),
        "independent NTS-KE request-negotiation acceptance oracle"
    );
    if let Ok(offer) = actual_request {
        let expected = expected_request.expect("request acceptance agreed");
        assert_eq!(offer.request_bytes(), data);
        assert_eq!(offer.request_bytes().as_ptr(), data.as_ptr());
        let original = &data[expected.aead_start..expected.aead_end];
        assert_eq!(offer.offered_aead_bytes(), original);
        assert_eq!(offer.offered_aead_bytes().as_ptr(), original.as_ptr());
    }

    let fixed_request = [0x80, 1, 0, 2, 0, 0, 0, 4, 0, 4, 0, 15, 0, 16, 0x80, 0, 0, 0];
    let fixed_raw = frame_nts_ke_message(&fixed_request, limits).expect("literal complete request");
    let fixed_offer = read_nts_ke_ntpv4_offer(&fixed_raw).expect("literal NTPv4 offer 15 and 16");
    let actual_response = negotiate_nts_ke_ntpv4(&fixed_offer, message);
    assert_eq!(
        actual_response.is_ok(),
        expected_response.is_some(),
        "independent NTS-KE response-negotiation acceptance oracle"
    );
    if let Ok(pending) = actual_response {
        let expected = expected_response.expect("response acceptance agreed");
        assert_eq!(pending.bytes(), data);
        assert_eq!(pending.bytes().as_ptr(), data.as_ptr());
        assert_eq!(
            usize::from(pending.aead_id()),
            usize::from(data[expected.aead_start]) * 256
                + usize::from(data[expected.aead_start + 1])
        );
        assert_eq!(pending.port_hint(), expected.port);
        match (pending.server_hint(), expected.server) {
            (Some(actual), Some((start, end))) => {
                assert_eq!(actual.as_bytes(), &data[start..end]);
                assert_eq!(actual.as_ptr(), data[start..end].as_ptr());
            }
            (None, None) => {}
            _ => panic!("independent NTS-KE pending-server presence oracle"),
        }
        for _ in 0..2 {
            let mut cookies = pending.cookies();
            assert!(visit_model_records(data, |kind, _, start, end| {
                if kind == 5 {
                    let cookie = cookies.next().expect("independently counted cookie");
                    assert_eq!(cookie, &data[start..end]);
                    assert_eq!(cookie.as_ptr(), data[start..end].as_ptr());
                }
            }));
            assert!(cookies.next().is_none());
            assert!(cookies.next().is_none());
        }
        assert_eq!(
            pending.cookie_count(),
            expected.cookies,
            "independent NTS-KE negotiation count agreement"
        );
        // This feature changes only a test observation, after genuine accepted
        // production input and every unmodified independent projection agree.
        let observed =
            pending.cookie_count() + usize::from(cfg!(feature = "seeded-negotiation-defect"));
        assert_eq!(
            observed, expected.cookies,
            "independent NTS-KE negotiation cookie-count oracle"
        );
    }
}

fuzz_target!(|data: &[u8]| {
    check_header(data);
    for (bytes, records) in [(4, 1), (65_536, 2), (131_072, 32_768)] {
        check_ke(data, bytes, records);
    }
    for (bytes, fields) in [(48, 0), (75, 1), (76, 1), (92, 2), (4096, 64)] {
        check_datagram(data, bytes, fields);
    }
    // A further bounded independent context varies the budgets with raw input.
    let bytes = 48 + usize::from(data.first().copied().unwrap_or(0)) * 15;
    let fields = usize::from(data.get(1).copied().unwrap_or(0)) % 65;
    check_datagram(data, bytes, fields);
    check_negotiation(data);
});
