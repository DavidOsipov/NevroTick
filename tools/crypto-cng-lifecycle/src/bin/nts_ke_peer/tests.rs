//! Independent public literals and rejected-neighbor controls; no native effects.

use std::ffi::OsString;
use std::num::NonZeroU16;
use std::time::{Duration, Instant};

use super::{
    Budget, Config, Failed, Failure, Flow, IO_LIMIT, Mode, RESPONSE, admit_offer, completion_line,
    ready_line, resolve,
};

/// Later alert-write refusal stays separate from the original TLS protocol error.
///
/// # Panics
/// Fail if the primary error or distinct cleanup failure is lost.
#[test]
fn primary_tls_error_precedes_a_later_alert_fault() {
    let primary = Failure::Io(std::io::ErrorKind::InvalidData);
    let cleanup = Failure::Io(std::io::ErrorKind::BrokenPipe);
    assert_eq!(
        resolve::<()>(Some(cleanup), Err(primary)),
        Err(Failed {
            primary,
            cleanup: Some(cleanup)
        })
    );
    assert_eq!(resolve::<()>(None, Err(primary)), Err(primary.into()));
    assert_eq!(
        resolve::<()>(Some(primary), Err(primary)),
        Err(primary.into())
    );
}

/// Sticky faults refuse nominal success; ordinary errors and success stay intact.
///
/// # Panics
/// Fail if resolution changes a healthy or sticky-fault outcome.
#[test]
fn sticky_fault_and_healthy_result_neighbors() {
    assert_eq!(resolve(None, Ok(7_u8)), Ok(7));
    assert_eq!(
        resolve(Some(Failure::Deadline), Ok(7_u8)),
        Err(Failure::Deadline.into())
    );
    let refusal = Failure::Io(std::io::ErrorKind::UnexpectedEof);
    assert_eq!(resolve::<()>(None, Err(refusal)), Err(refusal.into()));
    let fault = Failure::Io(std::io::ErrorKind::BrokenPipe);
    assert_eq!(resolve::<()>(Some(fault), Err(refusal)), Err(fault.into()));
}

/// Construct known absolute test paths without opening or modifying any file.
fn arguments() -> Vec<OsString> {
    let base = std::env::temp_dir();
    [
        OsString::from("peer.exe"),
        base.join("leaf.der").into_os_string(),
        base.join("leaf-key.pk8").into_os_string(),
        base.join("peer.ready").into_os_string(),
    ]
    .into()
}

/// All critical-bit, type, length and algorithm positions must match literally.
///
/// # Panics
/// Fail if the exact offer refuses or any altered byte is ignored.
#[test]
fn literal_offer_and_each_changed_byte() {
    let expected = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];
    assert_eq!(admit_offer(&expected), Ok(()));
    for position in 0..expected.len() {
        let mut changed = expected;
        changed[position] ^= 1;
        assert_eq!(admit_offer(&changed), Err(Failure::Offer));
    }
    assert_eq!(admit_offer(&expected[..15]), Err(Failure::Offer));
    assert_eq!(admit_offer(&[0; 17]), Err(Failure::Offer));
}

/// Pass the exact public peer reply through the consuming KE and request readers.
///
/// # Errors
/// Preserve an unexpected framing, negotiation or packet preparation refusal.
/// # Panics
/// A four-byte neighbor must refuse without writes; the peer's actual cookie
/// must admit an exact 140-byte never-send request with a 100-byte AAD.
#[test]
fn literal_response_cookie_prepares_the_exact_preview_shape() -> Result<(), String> {
    use ntp_wire::{
        FrameLimits, NtsKeCookies, NtsKeFrameLimits, NtsKeNtpv4Negotiation, NtsKeNtpv4Offer,
        NtsRequestEncodeError, NtsRequestShape, PreparedNtsRequest, RawHeader, RawNtsKeMessage,
        decode_fixed_header, frame_nts_ke_message, negotiate_nts_ke_ntpv4,
        prepare_nts_client_request, read_nts_ke_ntpv4_offer,
    };

    let framing: NtsKeFrameLimits =
        NtsKeFrameLimits::new(32, 4).map_err(|error| format!("KE limits: {error:?}"))?;
    let framed_offer: RawNtsKeMessage<'_> = frame_nts_ke_message(&super::OFFER, framing)
        .map_err(|error| format!("offer frame: {error:?}"))?;
    let offer: NtsKeNtpv4Offer<'_> =
        read_nts_ke_ntpv4_offer(&framed_offer).map_err(|error| format!("offer: {error:?}"))?;
    let framed_reply: RawNtsKeMessage<'_> = frame_nts_ke_message(&RESPONSE, framing)
        .map_err(|error| format!("response frame: {error:?}"))?;
    let reply: NtsKeNtpv4Negotiation<'_> = negotiate_nts_ke_ntpv4(&offer, framed_reply)
        .map_err(|error| format!("reply: {error:?}"))?;
    assert_eq!((reply.server_hint(), reply.port_hint()), (None, None));
    let mut cookies: NtsKeCookies<'_> = reply.cookies();
    let cookie: &[u8] = cookies.next().ok_or("expected one peer cookie")?;
    assert_eq!(cookies.next(), None);

    let mut header_bytes: [u8; 48] = [0; 48];
    header_bytes[0] = 0x23;
    let header: RawHeader =
        decode_fixed_header(&header_bytes).map_err(|error| format!("header: {error:?}"))?;
    let shape: NtsRequestShape =
        NtsRequestShape::new(16, 16, 16).map_err(|error| format!("shape: {error:?}"))?;
    let limits: FrameLimits =
        FrameLimits::new(140, 3).map_err(|error| format!("packet limits: {error:?}"))?;
    let uid: [u8; 32] = [0x55; 32];
    let mut refused: [u8; 140] = [0xa5; 140];
    assert!(matches!(
        prepare_nts_client_request(
            &header,
            &uid,
            &[0xaa, 0xbb, 0xcc, 0xdd],
            0,
            shape,
            &mut refused,
            limits,
        ),
        Err(NtsRequestEncodeError::UnsupportedCookie)
    ));
    assert_eq!(refused, [0xa5; 140]);

    let mut output: [u8; 140] = [0; 140];
    let prepared: PreparedNtsRequest<'_> =
        prepare_nts_client_request(&header, &uid, cookie, 0, shape, &mut output, limits)
            .map_err(|error| format!("actual peer cookie: {error:?}"))?;
    assert_eq!(prepared.associated_data().len(), 100);
    // Inert shape components exercise the formatter without claiming AEAD work.
    let packet: ntp_wire::NtsClientRequest<'_> = prepared
        .finish(&[0; 16], &[0; 16])
        .map_err(|error| format!("finish: {error:?}"))?;
    assert_eq!(packet.packet_bytes().len(), 140);
    assert_eq!(
        RESPONSE.as_slice(),
        [
            0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0, 5, 0, 12, 0xaa, 0xbb, 0xcc, 0xdd, 0x10,
            0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x80, 0, 0, 0,
        ]
    );
    Ok(())
}

/// The actual negotiation reader receives one port record before EOM.
///
/// # Errors
/// Preserve an offer, framing or negotiation refusal.
/// # Panics
/// Fail if the literal records, accepted port or cookie count differ.
#[test]
fn protected_response_advertises_exact_port_and_original_cookie() -> Result<(), String> {
    use ntp_wire::{
        NtsKeFrameLimits, NtsKeNtpv4Negotiation, NtsKeNtpv4Offer, RawNtsKeMessage,
        frame_nts_ke_message, negotiate_nts_ke_ntpv4, read_nts_ke_ntpv4_offer,
    };

    let frame_limits: NtsKeFrameLimits =
        NtsKeFrameLimits::new(38, 5).map_err(|error| format!("KE limits: {error:?}"))?;
    let framed_offer: RawNtsKeMessage<'_> = frame_nts_ke_message(&super::OFFER, frame_limits)
        .map_err(|error| format!("offer frame: {error:?}"))?;
    let offer: NtsKeNtpv4Offer<'_> =
        read_nts_ke_ntpv4_offer(&framed_offer).map_err(|error| format!("offer: {error:?}"))?;
    let port: NonZeroU16 = NonZeroU16::new(49_666).ok_or("expected nonzero UDP port")?;
    let response: [u8; 38] = super::protected_response(port);
    assert_eq!(
        response,
        [
            0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0, 5, 0, 12, 0xaa, 0xbb, 0xcc, 0xdd, 0x10,
            0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0, 7, 0, 2, 0xc2, 0x02, 0x80, 0, 0, 0,
        ],
    );
    let frame: RawNtsKeMessage<'_> = frame_nts_ke_message(&response, frame_limits)
        .map_err(|error| format!("response frame: {error:?}"))?;
    let negotiated: NtsKeNtpv4Negotiation<'_> = negotiate_nts_ke_ntpv4(&offer, frame)
        .map_err(|error| format!("response negotiation: {error:?}"))?;
    assert_eq!(negotiated.port_hint(), Some(49_666));
    assert_eq!(negotiated.server_hint(), None);
    assert_eq!(negotiated.cookie_count(), 1);
    Ok(())
}

/// Port zero refuses; both admitted numeric extremes have the exact ASCII form.
///
/// # Errors
/// Preserve an admitted neighbor's serialization failure.
/// # Panics
/// Fail if zero is admitted or either exact ASCII representation differs.
#[test]
fn readiness_port_is_bounded_and_has_no_optional_fields() -> Result<(), Failure> {
    assert_eq!(ready_line(0, None), Err(Failure::Arguments));
    assert_eq!(ready_line(1, None)?, "port=1\n");
    assert_eq!(ready_line(u16::MAX, None)?, "port=65535\n");
    Ok(())
}

/// The successor mode's readiness names both independently bound endpoints.
///
/// # Errors
/// Preserve a refusal to serialize the admitted control.
/// # Panics
/// Fail if either recorded value or the exact readiness grammar changes.
#[test]
fn successor_readiness_names_both_owned_endpoints() -> Result<(), Failure> {
    assert_eq!(
        ready_line(1, Some(NonZeroU16::MAX))?,
        "port=1 udp_port=65535\n"
    );
    assert_eq!(
        ready_line(0, Some(NonZeroU16::MAX)),
        Err(Failure::Arguments)
    );
    Ok(())
}

/// Exact argument shape succeeds; missing, extra, relative and aliased paths fail.
///
/// # Errors
/// Preserve failure to admit the healthy arguments.
/// # Panics
/// Fail if an invalid argument set is admitted or a checked path is relative.
#[test]
fn cli_requires_paths_and_at_most_one_exact_final_mode() -> Result<(), Failure> {
    let config: Config = Config::parse(arguments().into_iter())?;
    assert!(config.certificate.is_absolute());
    assert!(config.key.is_absolute());
    assert!(config.ready.is_absolute());
    assert_eq!(config.mode, Mode::Ke);
    let mut missing = arguments();
    let _removed = missing.pop();
    assert!(matches!(
        Config::parse(missing.into_iter()),
        Err(Failure::Arguments)
    ));
    let mut extra = arguments();
    extra.push(OsString::from("healthy"));
    assert!(matches!(
        Config::parse(extra.into_iter()),
        Err(Failure::Arguments)
    ));
    let mut protected: Vec<OsString> = arguments();
    protected.push(OsString::from("--protected-datagram"));
    assert_eq!(
        Config::parse(protected.clone().into_iter())?.mode,
        Mode::Protected
    );
    protected.push(OsString::from("--protected-datagram"));
    assert!(matches!(
        Config::parse(protected.into_iter()),
        Err(Failure::Arguments)
    ));
    let mut successor: Vec<OsString> = arguments();
    successor.push(OsString::from("--protected-successor-deny"));
    assert_eq!(
        Config::parse(successor.clone().into_iter())?.mode,
        Mode::ProtectedSuccessorDeny
    );
    successor.push(OsString::from("--protected-datagram"));
    assert!(matches!(
        Config::parse(successor.into_iter()),
        Err(Failure::Arguments)
    ));
    for value in ["relative.der", "", "é", "nul\0path"] {
        let mut invalid = arguments();
        invalid[1] = OsString::from(value);
        assert!(matches!(
            Config::parse(invalid.into_iter()),
            Err(Failure::Arguments)
        ));
    }
    let mut same = arguments();
    same[2] = same[1].clone();
    assert!(matches!(
        Config::parse(same.into_iter()),
        Err(Failure::Arguments)
    ));
    let mut long = arguments();
    long[1] = OsString::from("x".repeat(1025));
    assert!(matches!(
        Config::parse(long.into_iter()),
        Err(Failure::Arguments)
    ));
    Ok(())
}

/// Project the completed outcomes with independent exact line contracts.
///
/// # Panics
/// Fail on a changed default line or missing protected observation.
#[test]
fn completion_keeps_default_exact_and_protected_distinct() {
    assert_eq!(
        completion_line(Mode::Ke),
        "scope=laboratory-ke-peer completed=1 offer_bytes=16 response_bytes=32 exporters=2 peer_close=1"
    );
    assert_eq!(
        completion_line(Mode::Protected),
        "scope=laboratory-ke-peer-protected completed=1 offer_bytes=16 response_bytes=38 exporters=2 peer_close=1 udp_received=1 udp_sent=1"
    );
    assert_eq!(
        completion_line(Mode::ProtectedSuccessorDeny),
        "scope=laboratory-ke-peer-protected-successor-deny completed=1 offer_bytes=16 response_bytes=38 exporters=2 peer_close=1 udp_received=2 udp_sent=2"
    );
}

/// Polls never renew the origin; equality and later times both refuse.
///
/// # Errors
/// Preserve refusal to construct the original budget.
/// # Panics
/// Fail if the remaining wait or exact deadline boundary differs.
#[test]
fn original_deadline_has_exact_neighbors() -> Result<(), Failure> {
    let origin = Instant::now();
    let budget = Budget::new(origin)?;
    assert_eq!(budget.poll(origin), Ok(Duration::from_millis(200)));
    let end = origin
        .checked_add(Duration::from_secs(60))
        .ok_or(Failure::Deadline)?;
    let before = end
        .checked_sub(Duration::from_nanos(1))
        .ok_or(Failure::Deadline)?;
    assert_eq!(budget.poll(before), Ok(Duration::from_nanos(1)));
    assert_eq!(budget.poll(end), Err(Failure::Deadline));
    let after = end
        .checked_add(Duration::from_nanos(1))
        .ok_or(Failure::Deadline)?;
    assert_eq!(budget.poll(after), Err(Failure::Deadline));
    Ok(())
}

/// Each direction independently accepts its cap; overflow preserves old counts.
///
/// # Errors
/// Preserve an unexpected refusal of a valid byte charge.
/// # Panics
/// Fail if a counter exceeds its cap or changes the other allowance.
#[test]
fn actual_byte_caps_have_independent_healthy_neighbors() -> Result<(), Failure> {
    let mut budget = Budget::new(Instant::now())?;
    for flow in [Flow::Read, Flow::Write] {
        assert_eq!(budget.allowance(flow), Ok(IO_LIMIT));
        budget.charge(flow, IO_LIMIT - 1)?;
        assert_eq!(budget.allowance(flow), Ok(1));
        assert_eq!(budget.charge(flow, 2), Err(Failure::Budget));
        assert_eq!(budget.allowance(flow), Ok(1));
        budget.charge(flow, 1)?;
        assert_eq!(budget.allowance(flow), Err(Failure::Budget));
        assert_eq!(budget.charge(flow, usize::MAX), Err(Failure::Budget));
    }
    assert_eq!((budget.read, budget.written), (IO_LIMIT, IO_LIMIT));
    Ok(())
}
