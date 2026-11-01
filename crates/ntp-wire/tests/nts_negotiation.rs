//! Independent NTPv4-only NTS-KE semantic facts for REQ-8, BND-2 and CHECK-6/25.
//! These inert fixtures establish neither TLS identity nor endpoint authority.
#![forbid(unsafe_code)]

use core::iter::FusedIterator;
use ntp_wire::{
    NtsKeFrameLimits, NtsKeNegotiationError, NtsKeNtpv4Negotiation, NtsKeNtpv4Offer,
    RawNtsKeMessage, frame_nts_ke_message, negotiate_nts_ke_ntpv4, read_nts_ke_ntpv4_offer,
};

/// Literal NTPv4/AEAD-15 request; no production writer supplies these bytes.
const REQUEST: [u8; 16] = [
    0x80, 0x01, 0x00, 0x02, 0x00, 0x00, 0x80, 0x04, 0x00, 0x02, 0x00, 0x0f, 0x80, 0x00, 0x00, 0x00,
];
/// Literal successful response with an opaque, empty cookie and no endpoint hint.
const EMPTY_COOKIE_RESPONSE: [u8; 20] = [
    0x80, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x0f, 0x00, 0x05, 0x00, 0x00,
    0x80, 0x00, 0x00, 0x00,
];
/// A record's independently declared type, critical bit and exact opaque body.
type RecordSpec<'a> = (u16, bool, &'a [u8]);

/// Encode only the fixture's declared tuples, then append a literal final EOM.
///
/// # Errors
/// Reject a fixture type/body that cannot fit its wire representation.
fn message(records: &[RecordSpec<'_>]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    for &(kind, critical, body) in records {
        if kind > 0x7fff {
            return Err("Fixture record type exceeds fifteen bits".into());
        }
        let length = u16::try_from(body.len()).map_err(|error| error.to_string())?;
        let prefix = kind | if critical { 0x8000 } else { 0 };
        bytes.extend_from_slice(&prefix.to_be_bytes());
        bytes.extend_from_slice(&length.to_be_bytes());
        bytes.extend_from_slice(body);
    }
    bytes.extend_from_slice(&[0x80, 0x00, 0x00, 0x00]);
    Ok(bytes)
}

/// Reach the real structural owner using explicit experimental byte/work limits.
///
/// # Errors
/// Preserve an unexpected fixture framing rejection as a readable test failure.
fn frame(bytes: &[u8]) -> Result<RawNtsKeMessage<'_>, String> {
    let limits = NtsKeFrameLimits::new(131_072, 32_768)
        .map_err(|error| format!("fixture limits: {error:?}"))?;
    frame_nts_ke_message(bytes, limits).map_err(|error| format!("fixture framing: {error:?}"))
}

/// Admit the actual immutable request, never a manually asserted offer list.
///
/// # Errors
/// Preserve any real framing or request semantic failure in the healthy fixture.
fn offer(bytes: &[u8]) -> Result<NtsKeNtpv4Offer<'_>, String> {
    read_nts_ke_ntpv4_offer(&frame(bytes)?).map_err(|error| format!("healthy offer: {error:?}"))
}

/// Reach response admission with a checked real request and a framed response.
///
/// # Errors
/// Preserve real framing and response semantic failures without payload output.
fn response<'a>(
    checked_offer: &NtsKeNtpv4Offer<'_>,
    bytes: &'a [u8],
) -> Result<NtsKeNtpv4Negotiation<'a>, String> {
    negotiate_nts_ke_ntpv4(checked_offer, frame(bytes)?)
        .map_err(|error| format!("healthy negotiation: {error:?}"))
}

/// Compile-check the public iterator's stable-exhaustion trait contract.
fn assert_fused<I: FusedIterator>(_iterator: &I) {}

/// Reject at the real request consumer, then admit the independent healthy neighbor.
///
/// # Errors
/// Preserve fixture framing failures or a rejected valid neighboring request.
/// # Panics
/// The exact rejection must match the independently declared semantic condition.
fn rejected_offer(bytes: &[u8], expected: NtsKeNegotiationError) -> Result<(), String> {
    assert_eq!(
        read_nts_ke_ntpv4_offer(&frame(bytes)?).err(),
        Some(expected)
    );
    let healthy = offer(&REQUEST)?;
    assert_eq!(healthy.offered_aead_bytes(), &[0x00, 0x0f]);
    Ok(())
}

/// Reject at the actual response consumer and retain the unchanged checked offer.
///
/// # Errors
/// Preserve fixture framing failures or a rejected valid neighboring response.
/// # Panics
/// The expected rejection and subsequent valid AEAD-15 negotiation must match.
fn rejected_response(bytes: &[u8], expected: NtsKeNegotiationError) -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    assert_eq!(
        negotiate_nts_ke_ntpv4(&checked, frame(bytes)?).err(),
        Some(expected)
    );
    assert_eq!(response(&checked, &EMPTY_COOKIE_RESPONSE)?.aead_id(), 15);
    assert_eq!(checked.request_bytes(), &REQUEST);
    Ok(())
}

/// Literal bytes establish the checked offer and original immutable projections.
/// # Errors
/// Preserve a real rejection of the independently declared valid request.
/// # Panics
/// Both getters must borrow the exact original request and AEAD body spans.
#[test]
fn literal_request_projects_original_checked_offer() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    assert_eq!(checked.request_bytes(), &REQUEST);
    assert!(core::ptr::eq(
        checked.request_bytes().as_ptr(),
        REQUEST.as_ptr()
    ));
    assert_eq!(checked.offered_aead_bytes(), &[0x00, 0x0f]);
    assert!(core::ptr::eq(
        checked.offered_aead_bytes().as_ptr(),
        REQUEST[10..12].as_ptr()
    ));
    assert!(!core::mem::needs_drop::<NtsKeNtpv4Offer<'_>>());
    Ok(())
}

/// Request protocol shape, cardinality and profile support are separate failures.
/// # Errors
/// Preserve unexpected fixture framing errors and the valid neighboring request.
/// # Panics
/// Every malformed/profile-limited case must reach its named request rejection.
#[test]
fn request_requires_one_critical_nonempty_ntpv4_protocol_record() -> Result<(), String> {
    use NtsKeNegotiationError::{
        DuplicateProtocolNegotiation, InvalidProtocolNegotiation, MissingProtocolNegotiation,
        UnsupportedProtocol,
    };
    let aead = (4, false, &[0x00, 0x0f][..]);
    rejected_offer(&message(&[aead])?, MissingProtocolNegotiation)?;
    for (body, critical, expected) in [
        (&[][..], true, InvalidProtocolNegotiation),
        (&[0x00][..], true, InvalidProtocolNegotiation),
        (&[0x00, 0x00, 0x00][..], true, InvalidProtocolNegotiation),
        (&[0x00, 0x00][..], false, InvalidProtocolNegotiation),
        (&[0x00, 0x01][..], true, UnsupportedProtocol),
        (&[0x00, 0x00, 0xff, 0xff][..], true, UnsupportedProtocol),
    ] {
        rejected_offer(&message(&[(1, critical, body), aead])?, expected)?;
    }
    rejected_offer(
        &message(&[(1, true, &[0, 0]), (1, true, &[0, 0]), aead])?,
        DuplicateProtocolNegotiation,
    )?;
    let repeated = message(&[(1, true, &[0, 0, 0, 0]), aead])?;
    assert_eq!(offer(&repeated)?.offered_aead_bytes(), &[0, 15]);
    Ok(())
}

/// Request AEAD values are opaque offered identifiers, with either critical bit.
/// # Errors
/// Preserve a rejection of either independently valid request encoding.
/// # Panics
/// Original ordering, duplicates and full-width identifiers must be preserved.
#[test]
fn request_aead_offers_preserve_order_duplicates_and_all_bits() -> Result<(), String> {
    let values = [0x00, 0x0f, 0x00, 0x00, 0xff, 0xff, 0x00, 0x0f];
    for critical in [false, true] {
        let bytes = message(&[(1, true, &[0, 0]), (4, critical, &values)])?;
        let checked = offer(&bytes)?;
        assert_eq!(checked.offered_aead_bytes(), &values);
        assert!(core::ptr::eq(
            checked.offered_aead_bytes().as_ptr(),
            bytes[10..18].as_ptr()
        ));
        assert_eq!(checked.request_bytes(), &bytes);
    }
    Ok(())
}

/// Missing, repeated, odd and empty AEAD request bodies cannot create an offer.
/// # Errors
/// Preserve real fixture failures and the independently valid neighboring offer.
/// # Panics
/// Every case must reject at its specific mandatory semantic guard.
#[test]
fn request_aead_shape_and_cardinality_have_healthy_neighbors() -> Result<(), String> {
    use NtsKeNegotiationError::{
        DuplicateAeadNegotiation, InvalidAeadNegotiation, MissingAeadNegotiation,
    };
    let protocol = (1, true, &[0, 0][..]);
    rejected_offer(&message(&[protocol])?, MissingAeadNegotiation)?;
    for body in [&[][..], &[0x0f][..], &[0, 15, 0][..]] {
        rejected_offer(
            &message(&[protocol, (4, false, body)])?,
            InvalidAeadNegotiation,
        )?;
    }
    rejected_offer(
        &message(&[protocol, (4, false, &[0, 15]), (4, true, &[0, 17])])?,
        DuplicateAeadNegotiation,
    )?;
    Ok(())
}

/// Error, Warning and cookie records have no legal client-request direction.
/// # Errors
/// Preserve structural fixture failures or rejection of a subsequent valid offer.
/// # Panics
/// Either critical-bit encoding of these known types must reject the request.
#[test]
fn request_forbids_server_only_records() -> Result<(), String> {
    for kind in [2, 3, 5] {
        for critical in [false, true] {
            let bytes = message(&[
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
                (kind, critical, &[0, 0]),
            ])?;
            rejected_offer(&bytes, NtsKeNegotiationError::ForbiddenRequestRecord)?;
        }
    }
    Ok(())
}

/// Unknown records are ignored only when their independent critical bit is clear.
/// # Errors
/// Preserve rejection of valid noncritical extensions and neighboring requests.
/// # Panics
/// Unknown critical records must prevent creation of any checked offer.
#[test]
fn request_unknown_critical_and_noncritical_records_differ() -> Result<(), String> {
    for kind in [8, 0x1234, 0x7fff] {
        let ignored = message(&[
            (kind, false, &[0xaa, 0xbb, 0xcc]),
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
        ])?;
        assert_eq!(offer(&ignored)?.offered_aead_bytes(), &[0, 15]);
        let critical = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (kind, true, &[0xaa, 0xbb, 0xcc]),
        ])?;
        rejected_offer(&critical, NtsKeNegotiationError::UnsupportedCriticalRecord)?;
    }
    Ok(())
}

/// Wire body capacity, rather than an invented small offer-list ceiling, bounds work.
/// # Errors
/// Preserve rejection of the actual maximum even-length AEAD body.
/// # Panics
/// The final offered full-width identifier must remain selectable after a long scan.
#[test]
fn maximum_even_aead_offer_body_reaches_its_final_identifier() -> Result<(), String> {
    let mut algorithms = vec![0; 65_534];
    algorithms[65_532..].copy_from_slice(&[0xff, 0xff]);
    let bytes = message(&[(1, true, &[0, 0]), (4, false, &algorithms)])?;
    let checked = offer(&bytes)?;
    assert_eq!(checked.offered_aead_bytes().len(), 65_534);
    assert_eq!(&checked.offered_aead_bytes()[65_532..], &[0xff, 0xff]);
    let selected = message(&[
        (1, true, &[0, 0]),
        (4, false, &[0xff, 0xff]),
        (5, false, &[]),
    ])?;
    assert_eq!(response(&checked, &selected)?.aead_id(), 65_535);
    Ok(())
}

/// A literal response supplies pending values without endpoint or TLS authority.
/// # Errors
/// Preserve rejection of the actual valid request and response.
/// # Panics
/// Every public projection and the empty original cookie span must match exactly.
#[test]
fn literal_response_preserves_pending_values_and_original_empty_cookie() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    let result = response(&checked, &EMPTY_COOKIE_RESPONSE)?;
    assert_eq!(result.bytes(), &EMPTY_COOKIE_RESPONSE);
    assert!(core::ptr::eq(
        result.bytes().as_ptr(),
        EMPTY_COOKIE_RESPONSE.as_ptr()
    ));
    assert_eq!(result.aead_id(), 15);
    assert_eq!(result.cookie_count(), 1);
    assert_eq!(result.server_hint(), None);
    assert_eq!(result.port_hint(), None);
    let mut cookies = result.cookies();
    assert_fused(&cookies);
    let empty = cookies.next().ok_or("Missing literal empty cookie")?;
    assert_eq!(empty, []);
    assert!(core::ptr::eq(
        empty.as_ptr(),
        EMPTY_COOKIE_RESPONSE[16..].as_ptr()
    ));
    assert_eq!(cookies.next(), None);
    assert_eq!(cookies.next(), None);
    assert_eq!(result.cookies().count(), 1);
    assert!(!core::mem::needs_drop::<NtsKeNtpv4Negotiation<'_>>());
    Ok(())
}

/// Servers may choose any offered algorithm and either AEAD critical-bit value.
/// # Errors
/// Preserve real request/response rejection for independent valid selections.
/// # Panics
/// A lower-preference offered identifier must not be replaced by the first one.
#[test]
fn response_selects_any_offered_aead_with_either_critical_bit() -> Result<(), String> {
    let bytes = message(&[(1, true, &[0, 0]), (4, true, &[0, 15, 0, 17, 0xff, 0xff])])?;
    let checked = offer(&bytes)?;
    for (body, expected) in [
        (&[0, 15][..], 15),
        (&[0, 17][..], 17),
        (&[0xff, 0xff][..], 65_535),
    ] {
        for critical in [false, true] {
            let selected =
                message(&[(1, true, &[0, 0]), (4, critical, body), (5, false, &[0xa5])])?;
            assert_eq!(response(&checked, &selected)?.aead_id(), expected);
        }
    }
    Ok(())
}

/// Response protocol shape and cardinality differ from a valid empty refusal.
/// # Errors
/// Preserve fixture framing failures and independently valid response neighbors.
/// # Panics
/// Named shape/cardinality/profile failures must be distinguishable.
#[test]
fn response_protocol_shape_cardinality_and_profile_rejections() -> Result<(), String> {
    use NtsKeNegotiationError::{
        DuplicateProtocolNegotiation, InvalidProtocolNegotiation, MissingProtocolNegotiation,
        UnsupportedProtocol,
    };
    let aead = (4, false, &[0, 15][..]);
    let cookie = (5, false, &[][..]);
    rejected_response(&message(&[aead, cookie])?, MissingProtocolNegotiation)?;
    for (critical, body, expected) in [
        (false, &[0, 0][..], InvalidProtocolNegotiation),
        (true, &[0][..], InvalidProtocolNegotiation),
        (true, &[0, 0, 0][..], InvalidProtocolNegotiation),
        (true, &[0, 1][..], UnsupportedProtocol),
        (true, &[0, 0, 0, 1][..], UnsupportedProtocol),
    ] {
        rejected_response(&message(&[(1, critical, body), aead, cookie])?, expected)?;
    }
    rejected_response(
        &message(&[(1, true, &[0, 0]), aead, (1, true, &[0, 0]), cookie])?,
        DuplicateProtocolNegotiation,
    )?;
    Ok(())
}

/// Repeated offered `NTPv4` IDs remain legal while a zero-selection response refuses.
/// # Errors
/// Preserve fixture failures and a rejected valid repeated-ID response.
/// # Panics
/// Empty negotiation must not gain success or be misclassified as missing AEAD.
#[test]
fn response_protocol_subset_allows_repeated_zero_and_explicit_empty_refusal() -> Result<(), String>
{
    let checked = offer(&REQUEST)?;
    let repeated = message(&[
        (1, true, &[0, 0, 0, 0, 0, 0]),
        (4, false, &[0, 15]),
        (5, false, &[]),
    ])?;
    assert_eq!(response(&checked, &repeated)?.aead_id(), 15);
    rejected_response(
        &message(&[(1, true, &[])])?,
        NtsKeNegotiationError::NoCommonProtocol,
    )?;
    rejected_response(
        &message(&[(1, true, &[]), (4, false, &[])])?,
        NtsKeNegotiationError::NoCommonProtocol,
    )?;
    Ok(())
}

/// AEAD response shape permits one ID, rather than another list of offers.
/// # Errors
/// Preserve fixture failures and healthy responses after each rejection.
/// # Panics
/// Missing, repeated, odd and multi-ID responses must reach their specific guards.
#[test]
fn response_aead_shape_and_cardinality_reject_before_cookie_use() -> Result<(), String> {
    use NtsKeNegotiationError::{
        DuplicateAeadNegotiation, InvalidAeadNegotiation, MissingAeadNegotiation,
    };
    let protocol = (1, true, &[0, 0][..]);
    let cookie = (5, false, &[][..]);
    rejected_response(&message(&[protocol, cookie])?, MissingAeadNegotiation)?;
    for body in [&[0x0f][..], &[0, 15, 0][..], &[0, 15, 0, 17][..]] {
        rejected_response(
            &message(&[protocol, (4, false, body), cookie])?,
            InvalidAeadNegotiation,
        )?;
    }
    rejected_response(
        &message(&[protocol, (4, false, &[0, 15]), (4, true, &[0, 15]), cookie])?,
        DuplicateAeadNegotiation,
    )?;
    Ok(())
}

/// AEAD-15 interoperability failure differs from a permitted no-common-AEAD refusal.
/// # Errors
/// Preserve fixture framing and healthy-control admission failures.
/// # Panics
/// Neither refusal may become success or be hidden by a premature missing-cookie error.
#[test]
fn response_empty_aead_distinguishes_mandatory_support_and_refusal() -> Result<(), String> {
    let empty = message(&[(1, true, &[0, 0]), (4, false, &[])])?;
    rejected_response(&empty, NtsKeNegotiationError::MandatoryAeadMissing)?;
    let request = message(&[(1, true, &[0, 0]), (4, false, &[0, 17])])?;
    let checked = offer(&request)?;
    assert_eq!(
        negotiate_nts_ke_ntpv4(&checked, frame(&empty)?).err(),
        Some(NtsKeNegotiationError::NoCommonAead)
    );
    let selected = message(&[(1, true, &[0, 0]), (4, false, &[0, 17]), (5, false, &[])])?;
    assert_eq!(response(&checked, &selected)?.aead_id(), 17);
    Ok(())
}

/// An unoffered selection cannot silently choose an algorithm or enable fallback.
/// # Errors
/// Preserve fixture failures and the unchanged valid offer/response neighbor.
/// # Panics
/// Full-width and zero-valued unoffered identifiers must all reject.
#[test]
fn response_unoffered_aead_has_no_accepted_negotiation() -> Result<(), String> {
    for body in [&[0, 0][..], &[0, 14][..], &[0, 16][..], &[0xff, 0xff][..]] {
        rejected_response(
            &message(&[(1, true, &[0, 0]), (4, false, body), (5, false, &[])])?,
            NtsKeNegotiationError::UnofferedAead,
        )?;
    }
    Ok(())
}

/// Cookie presence is mandatory after a usable protocol/AEAD pair is selected.
/// # Errors
/// Preserve framing failure and acceptance of the independent neighboring cookie.
/// # Panics
/// Missing cookies must not create a pending successful result.
#[test]
fn response_selected_protocol_and_aead_require_a_cookie() -> Result<(), String> {
    rejected_response(
        &message(&[(1, true, &[0, 0]), (4, false, &[0, 15])])?,
        NtsKeNegotiationError::MissingCookie,
    )
}

/// Opaque cookies retain exact body spans, including empty and odd-length bodies.
/// # Errors
/// Preserve framing and successful negotiation failures in these valid variants.
/// # Panics
/// Counts, original pointers, exact contents and repeated iterator exhaustion must match.
#[test]
fn response_cookie_iterator_preserves_spans_and_skips_non_cookie_records() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    for critical in [false, true] {
        let bytes = message(&[
            (5, critical, &[0xa5]),
            (0x1234, false, &[0x99]),
            (4, false, &[0, 15]),
            (5, !critical, &[]),
            (6, false, b"time.example"),
            (1, true, &[0, 0]),
            (5, critical, &[0x00, 0xff, 0x80]),
            (7, true, &[0x12, 0x34]),
        ])?;
        let result = response(&checked, &bytes)?;
        assert_eq!(result.cookie_count(), 3);
        assert_eq!(result.server_hint(), Some("time.example"));
        assert_eq!(result.port_hint(), Some(0x1234));
        let mut cookies = result.cookies();
        assert_fused(&cookies);
        for expected in [&bytes[4..5], &bytes[20..20], &bytes[46..49]] {
            let actual = cookies
                .next()
                .ok_or("Missing independently located cookie")?;
            assert_eq!(actual, expected);
            assert!(core::ptr::eq(actual.as_ptr(), expected.as_ptr()));
        }
        assert_eq!(cookies.next(), None);
        assert_eq!(cookies.next(), None);
        assert_eq!(result.cookies().count(), 3);
    }
    Ok(())
}

/// A cookie is opaque up to its independent sixteen-bit body-length maximum.
/// # Errors
/// Preserve framing and semantic rejection of a valid maximum-size opaque record.
/// # Panics
/// No arbitrary cookie-size constant may truncate or reinterpret the borrowed body.
#[test]
fn response_maximum_cookie_body_remains_opaque_and_borrowed() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    let body = vec![0xa7; 65_535];
    let bytes = message(&[(1, true, &[0, 0]), (4, false, &[0, 15]), (5, false, &body)])?;
    let result = response(&checked, &bytes)?;
    assert_eq!(result.cookie_count(), 1);
    let actual = result.cookies().next().ok_or("Missing maximum cookie")?;
    assert_eq!(actual, body);
    assert_eq!(actual.len(), 65_535);
    assert!(core::ptr::eq(actual.as_ptr(), bytes[16..65_551].as_ptr()));
    Ok(())
}

/// The full experimental byte budget bounds many empty opaque cookies as well.
/// # Errors
/// Preserve real rejection of a complete valid 128-KiB semantic response.
/// # Panics
/// Exact counts and stable exhaustion must survive the maximum byte-sized case.
#[test]
fn response_many_empty_cookies_remain_bounded_and_counted_exactly() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    let mut records: Vec<RecordSpec<'_>> = vec![(1, true, &[0, 0]), (4, false, &[0, 15])];
    records.extend(core::iter::repeat_n((5, false, &[][..]), 32_764));
    let bytes = message(&records)?;
    assert_eq!(bytes.len(), 131_072);
    let result = response(&checked, &bytes)?;
    assert_eq!(result.cookie_count(), 32_764);
    let mut cookies = result.cookies();
    for index in 0..32_764 {
        let actual = cookies
            .next()
            .ok_or("Cookie iteration ended before its exact count")?;
        assert_eq!(actual, []);
        assert!(core::ptr::eq(
            actual.as_ptr(),
            bytes[16 + 4 * index..].as_ptr()
        ));
    }
    assert_eq!(cookies.next(), None);
    assert_eq!(cookies.next(), None);
    Ok(())
}

/// Server error records abort even without next-protocol or AEAD negotiation.
/// # Errors
/// Preserve fixture failures and the valid neighbor after each explicit abort.
/// # Panics
/// Recognized and unknown full-width error codes must remain exact and unusable.
#[test]
fn response_error_and_eom_aborts_without_protocol_negotiation() -> Result<(), String> {
    for code in [0_u16, 1, 2, 0x1234, u16::MAX] {
        rejected_response(
            &message(&[(2, true, &code.to_be_bytes())])?,
            NtsKeNegotiationError::PeerError { code },
        )?;
    }
    Ok(())
}

/// Valid-looking cookies and negotiation cannot override a peer's explicit abort.
/// # Errors
/// Preserve fixture failures and valid neighboring responses.
/// # Panics
/// Error position and multiple error/warning records must not manufacture success.
#[test]
fn response_peer_error_wins_over_pending_success_and_warning() -> Result<(), String> {
    for records in [
        vec![
            (2, true, &[0, 2][..]),
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (5, false, &[]),
        ],
        vec![
            (1, true, &[0, 0][..]),
            (4, false, &[0, 15]),
            (5, false, &[]),
            (2, true, &[0, 2]),
        ],
        vec![
            (3, true, &[0xff, 0xff][..]),
            (2, true, &[0, 2]),
            (2, true, &[0, 1]),
        ],
    ] {
        rejected_response(
            &message(&records)?,
            NtsKeNegotiationError::PeerError { code: 2 },
        )?;
    }
    Ok(())
}

/// Error bodies have an exact two-byte shape and mandatory critical bit.
/// # Errors
/// Preserve structural fixture failures and valid response neighbors.
/// # Panics
/// Malformed errors must not be treated as harmless extensions or usable success.
#[test]
fn response_error_shape_and_critical_bit_are_mandatory() -> Result<(), String> {
    for (critical, body) in [
        (true, &[][..]),
        (true, &[0x01][..]),
        (true, &[0, 1, 0][..]),
        (false, &[0, 1][..]),
    ] {
        rejected_response(
            &message(&[(2, critical, body)])?,
            NtsKeNegotiationError::InvalidErrorRecord,
        )?;
    }
    Ok(())
}

/// This implementation recognizes no warning code, so every well-shaped warning aborts.
/// # Errors
/// Preserve structural fixture failures and valid neighboring negotiations.
/// # Panics
/// A warning cannot be ignored or translated into a successful pending association.
#[test]
fn response_unrecognized_warnings_abort_with_exact_codes() -> Result<(), String> {
    for code in [0_u16, 1, 0x1234, u16::MAX] {
        rejected_response(
            &message(&[
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
                (5, false, &[]),
                (3, true, &code.to_be_bytes()),
            ])?,
            NtsKeNegotiationError::UnsupportedWarning { code },
        )?;
    }
    rejected_response(
        &message(&[(3, true, &[0, 7]), (3, true, &[0, 8])])?,
        NtsKeNegotiationError::UnsupportedWarning { code: 7 },
    )?;
    Ok(())
}

/// Warning syntax is checked independently of unsupported warning-code handling.
/// # Errors
/// Preserve fixture framing and valid response neighbor failures.
/// # Panics
/// The critical bit and exact body width must be enforced at the actual receiver.
#[test]
fn response_warning_shape_and_critical_bit_are_mandatory() -> Result<(), String> {
    for (critical, body) in [
        (true, &[][..]),
        (true, &[0x01][..]),
        (true, &[0, 1, 0][..]),
        (false, &[0, 1][..]),
    ] {
        rejected_response(
            &message(&[(3, critical, body)])?,
            NtsKeNegotiationError::InvalidWarningRecord,
        )?;
    }
    Ok(())
}

/// Structural admission alone does not recognize an unknown critical record.
/// # Errors
/// Preserve fixture errors and real acceptance of valid noncritical extensions.
/// # Panics
/// Extension payloads may influence neither selected values nor cookie counts.
#[test]
fn response_unknown_extensions_preserve_critical_disposition_and_values() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    for kind in [8, 0x1234, 0x7fff] {
        for critical in [false, true] {
            let bytes = message(&[
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
                (5, false, &[0xa5]),
                (kind, critical, &[0xff, 0xff, 0xff]),
            ])?;
            if critical {
                rejected_response(&bytes, NtsKeNegotiationError::UnsupportedCriticalRecord)?;
            } else {
                let result = response(&checked, &bytes)?;
                assert_eq!(result.aead_id(), 15);
                assert_eq!(result.cookie_count(), 1);
                assert_eq!(result.cookies().next(), Some(&[0xa5][..]));
                assert_eq!(result.server_hint(), None);
                assert_eq!(result.port_hint(), None);
            }
        }
    }
    Ok(())
}

/// Server hints are borrowed pending data, including an unrooted wire FQDN.
/// # Errors
/// Preserve rejection of independently valid request and response hint shapes.
/// # Panics
/// Hint bytes must remain exact; no implicit normalization or endpoint grant occurs.
#[test]
fn response_server_hints_preserve_original_valid_ascii_spans() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    for host in [
        "time.example",
        "time.example.",
        "192.0.2.123",
        "2001:db8::1",
    ] {
        for critical in [false, true] {
            let bytes = message(&[
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
                (6, critical, host.as_bytes()),
                (5, false, &[]),
            ])?;
            let result = response(&checked, &bytes)?;
            let hint = result
                .server_hint()
                .ok_or("Missing literal pending server hint")?;
            assert_eq!(hint, host);
            assert!(core::ptr::eq(hint.as_ptr(), bytes[16..].as_ptr()));
            assert_eq!(result.port_hint(), None);
        }
    }
    Ok(())
}

/// The pending hint's explicit ASCII contract excludes controls and URI/zone syntax.
/// # Errors
/// Preserve fixture failures and successful neighboring negotiation admission.
/// # Panics
/// Invalid server hints cannot be returned as a pending negotiation result.
#[test]
fn response_server_hint_ascii_and_reserved_syntax_rejections() -> Result<(), String> {
    for body in [
        &[][..],
        b" ",
        b"time example",
        b"\t",
        b"a\nb",
        &[0],
        &[0x1f],
        &[0x7f],
        &[0x80],
        &[0xff],
        b"a/b",
        b"a\\b",
        b"a@b",
        b"fe80::1%7",
        b"a[b",
        b"a]b",
        b"[2001:db8::1]",
    ] {
        rejected_response(
            &message(&[
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
                (6, false, body),
                (5, false, &[]),
            ])?,
            NtsKeNegotiationError::InvalidServerHint,
        )?;
    }
    Ok(())
}

/// Server-name cardinality is independently constrained in both request and response.
/// # Errors
/// Preserve fixture failures and successful neighboring offer/response admission.
/// # Panics
/// Identical duplicates and conflicting hints must both reject instead of choosing one.
#[test]
fn request_and_response_duplicate_server_hints_reject() -> Result<(), String> {
    for other in [&b"time.example"[..], &b"other.example"[..]] {
        let request = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (6, false, b"time.example"),
            (6, true, other),
        ])?;
        rejected_offer(&request, NtsKeNegotiationError::DuplicateServer)?;
        let response = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (6, true, b"time.example"),
            (6, false, other),
            (5, false, &[]),
        ])?;
        rejected_response(&response, NtsKeNegotiationError::DuplicateServer)?;
    }
    Ok(())
}

/// Request hints share the pending shape checks without granting endpoint use.
/// # Errors
/// Preserve fixture failures and valid offer admission for both critical encodings.
/// # Panics
/// Request hint rejection must preserve a reusable independent healthy offer.
#[test]
fn request_server_hints_use_the_same_pending_shape_contract() -> Result<(), String> {
    for critical in [false, true] {
        let request = message(&[
            (6, critical, b"time.example"),
            (4, false, &[0, 15]),
            (1, true, &[0, 0]),
        ])?;
        assert_eq!(offer(&request)?.request_bytes(), &request);
    }
    for body in [&[][..], &b"a b"[..], &b"a%7"[..], &[0xff][..]] {
        rejected_offer(
            &message(&[(1, true, &[0, 0]), (4, false, &[0, 15]), (6, false, body)])?,
            NtsKeNegotiationError::InvalidServerHint,
        )?;
    }
    Ok(())
}

/// Port fields retain all sixteen bits; absence remains distinct from a pending value.
/// # Errors
/// Preserve fixture failures and real admission of the independent port neighbors.
/// # Panics
/// Neither zero nor the full-width endpoint may be silently defaulted or normalized.
#[test]
fn response_port_hints_preserve_zero_endpoints_and_byte_order() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    for value in [0_u16, 1, 123, 0x1234, u16::MAX] {
        for critical in [false, true] {
            let bytes = message(&[
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
                (7, critical, &value.to_be_bytes()),
                (5, false, &[]),
            ])?;
            let result = response(&checked, &bytes)?;
            assert_eq!(result.port_hint(), Some(value));
            assert_eq!(result.server_hint(), None);
        }
    }
    assert_eq!(
        response(&checked, &EMPTY_COOKIE_RESPONSE)?.port_hint(),
        None
    );
    Ok(())
}

/// Port body width is exactly two bytes in requests and responses.
/// # Errors
/// Preserve fixture failures and independently valid neighboring admissions.
/// # Panics
/// Every width neighbor must reject at the actual corresponding semantic consumer.
#[test]
fn request_and_response_port_width_neighbors_reject() -> Result<(), String> {
    for body in [&[][..], &[0x7b][..], &[0, 123, 0][..], &[0, 123, 0, 0][..]] {
        let request = message(&[(1, true, &[0, 0]), (4, false, &[0, 15]), (7, false, body)])?;
        rejected_offer(&request, NtsKeNegotiationError::InvalidPort)?;
        let response = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (7, true, body),
            (5, false, &[]),
        ])?;
        rejected_response(&response, NtsKeNegotiationError::InvalidPort)?;
    }
    Ok(())
}

/// Duplicate-port rejection is the declared local ambiguity policy, not an RFC MUST.
/// # Errors
/// Preserve fixture failures and valid neighboring offer/response admissions.
/// # Panics
/// Neither identical nor conflicting repeated port hints may acquire precedence.
#[test]
fn request_and_response_duplicate_ports_follow_explicit_local_policy() -> Result<(), String> {
    for second in [&[0, 123][..], &[0x12, 0x34][..]] {
        let request = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (7, false, &[0, 123]),
            (7, true, second),
        ])?;
        rejected_offer(&request, NtsKeNegotiationError::DuplicatePort)?;
        let response = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (5, false, &[]),
            (7, true, &[0, 123]),
            (7, false, second),
        ])?;
        rejected_response(&response, NtsKeNegotiationError::DuplicatePort)?;
    }
    Ok(())
}

/// A client port hint may appear without a server hint while remaining inert data.
/// # Errors
/// Preserve framing or semantic rejection of these legitimate pending hint shapes.
/// # Panics
/// Original request bytes and the admitted offer list must remain unchanged.
#[test]
fn request_port_only_hints_do_not_need_endpoint_authority() -> Result<(), String> {
    for critical in [false, true] {
        for port in [0_u16, 123, u16::MAX] {
            let request = message(&[
                (7, critical, &port.to_be_bytes()),
                (1, true, &[0, 0]),
                (4, false, &[0, 15]),
            ])?;
            let checked = offer(&request)?;
            assert_eq!(checked.request_bytes(), &request);
            assert_eq!(checked.offered_aead_bytes(), &[0, 15]);
        }
    }
    Ok(())
}

/// An Error record does not short-circuit validation of the complete response.
/// # Errors
/// Preserve structural fixture failures and independently valid response neighbors.
/// # Panics
/// Later unknown-critical or malformed records must retain their actual failures.
#[test]
fn response_error_still_requires_complete_record_semantics() -> Result<(), String> {
    rejected_response(
        &message(&[(2, true, &[0, 2]), (0x1234, true, &[])])?,
        NtsKeNegotiationError::UnsupportedCriticalRecord,
    )?;
    rejected_response(
        &message(&[(2, true, &[0, 2]), (4, false, &[0])])?,
        NtsKeNegotiationError::InvalidAeadNegotiation,
    )?;
    rejected_response(
        &message(&[(2, true, &[0, 2]), (3, false, &[0, 1])])?,
        NtsKeNegotiationError::InvalidWarningRecord,
    )?;
    rejected_response(
        &message(&[(2, true, &[0, 2]), (4, false, &[0, 15, 0, 17])])?,
        NtsKeNegotiationError::InvalidAeadNegotiation,
    )?;
    rejected_response(
        &message(&[(1, true, &[]), (4, true, &[0, 15, 0, 17])])?,
        NtsKeNegotiationError::InvalidAeadNegotiation,
    )?;
    Ok(())
}

/// Borrowed projections and cookie traversal may outlive the temporary result view.
/// # Errors
/// Preserve healthy request and response admission failures.
/// # Panics
/// The input buffer alone owns projected spans, with no hidden copied cookie store.
#[test]
fn response_cookie_iterator_and_hint_outlive_temporary_negotiation_view() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    let bytes = message(&[
        (1, true, &[0, 0]),
        (4, false, &[0, 15]),
        (5, false, &[0xaa]),
        (6, false, b"time.example"),
    ])?;
    let (mut cookies, hint) = {
        let result = response(&checked, &bytes)?;
        (result.cookies(), result.server_hint())
    };
    assert_eq!(cookies.next(), Some(&[0xaa][..]));
    assert_eq!(cookies.next(), None);
    assert_eq!(cookies.next(), None);
    assert_eq!(hint, Some("time.example"));
    let actual_hint = hint.ok_or("Missing borrowed server hint")?;
    assert!(core::ptr::eq(actual_hint.as_ptr(), bytes[21..].as_ptr()));
    Ok(())
}

/// Zero is an exact offered identifier here, rather than proof of provider support.
/// # Errors
/// Preserve fixture failures and admission of an explicitly matching identifier.
/// # Panics
/// Raw identifier comparison must preserve zero and both octets without fallback.
#[test]
fn response_matching_zero_aead_id_stays_pending_without_provider_claim() -> Result<(), String> {
    let request = message(&[(1, true, &[0, 0]), (4, false, &[0, 15, 0, 0])])?;
    let checked = offer(&request)?;
    let bytes = message(&[(1, true, &[0, 0]), (4, false, &[0, 0]), (5, false, &[])])?;
    assert_eq!(response(&checked, &bytes)?.aead_id(), 0);
    Ok(())
}

/// Printable-ASCII endpoints delimit the explicitly limited pending-hint contract.
/// # Errors
/// Preserve framing and admission failures for the literal pending byte values.
/// # Panics
/// Exact graphic endpoints must remain pending bytes without a DNS-validity claim.
#[test]
fn response_pending_server_hint_graphic_ascii_endpoints_are_exact() -> Result<(), String> {
    let checked = offer(&REQUEST)?;
    for hint in ["!", "~"] {
        let bytes = message(&[
            (1, true, &[0, 0]),
            (4, false, &[0, 15]),
            (6, false, hint.as_bytes()),
            (5, false, &[]),
        ])?;
        assert_eq!(response(&checked, &bytes)?.server_hint(), Some(hint));
    }
    Ok(())
}
