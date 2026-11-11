// SPDX-License-Identifier: GPL-2.0-only
//! Independent plaintext and publication controls; none simulates TLS success.

use super::{
    CancelOutcome, Config, ConfigCandidate, Control, Failure, IoPhase, Phase, Reply, copy_cookies,
    encode_offer, endpoint, read_response,
};
use garde::Validate;
use ntp_wire::{NtsKeAccumulatorError, NtsKeFrameError, NtsKeNegotiationError};
use rustls_pki_types::ServerName;
use std::ffi::OsString;
use std::io::{self, Read};
use std::time::{Duration, Instant};

/// Literal critical protocol-zero record, independent of the encoder.
const PROTOCOL: [u8; 6] = [0x80, 1, 0, 2, 0, 0];
/// Literal critical algorithm-fifteen record.
const ALGORITHM: [u8; 6] = [0x80, 4, 0, 2, 0, 15];
/// Literal critical, empty end marker.
const END: [u8; 4] = [0x80, 0, 0, 0];

/// Construct the fixed, separately named loopback fixture invocation.
///
/// # Errors
/// Preserve rejection of the fixed numeric peer or separate verification name.
fn config() -> Result<Config, Failure> {
    Config::new(
        OsString::from("127.0.0.1:4460"),
        OsString::from("Time.Example"),
    )
}

/// Encode independent record framing; reject an unrepresentable fixture body.
///
/// # Errors
/// Return `Failure::Control` if the body length cannot fit its wire `u16`.
fn record(kind: u16, body: &[u8]) -> Result<Vec<u8>, Failure> {
    let length = u16::try_from(body.len()).map_err(|_| Failure::Control)?;
    let mut bytes = Vec::from(kind.to_be_bytes());
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

/// Build fixed successful negotiation records around caller-owned literal bodies.
///
/// # Errors
/// Preserve an unrepresentable cookie body reported by the fixture record writer.
fn response(cookies: &[&[u8]], extra: &[u8]) -> Result<Vec<u8>, Failure> {
    let mut bytes = Vec::from(PROTOCOL);
    bytes.extend_from_slice(&ALGORITHM);
    for cookie in cookies {
        bytes.extend(record(5, cookie)?);
    }
    bytes.extend_from_slice(extra);
    bytes.extend_from_slice(&END);
    Ok(bytes)
}

/// Invoke the actual reader using the actual fixed offer; establish no TLS facts.
///
/// # Errors
/// Preserve configuration, offer, I/O, framing, negotiation or reply admission failure.
fn decode(reader: &mut impl Read) -> Result<Reply, Failure> {
    let configuration = config()?;
    let mut request = [0; 16];
    let offer = encode_offer(&mut request)?;
    read_response(reader, &configuration, &offer)
}

/// Supply ordinary EOF after exactly these plaintext bytes.
///
/// # Errors
/// Preserve the actual response helper's refusal of the supplied plaintext.
fn decode_bytes(mut bytes: &[u8]) -> Result<Reply, Failure> {
    decode(&mut bytes)
}

/// Reader with one split and bounded fragments; reading after its EOM must fail.
struct SplitReader<'a> {
    bytes: &'a [u8],
    cut: usize,
    maximum: usize,
    offset: usize,
    calls: usize,
}

impl Read for SplitReader<'_> {
    /// Return a bounded fragment; expose an unwanted post-EOM read as an error.
    ///
    /// # Errors
    /// Return `WouldBlock` once all fixture bytes have already been consumed.
    fn read(&mut self, target: &mut [u8]) -> io::Result<usize> {
        self.calls += 1;
        if self.offset == self.bytes.len() {
            return Err(io::ErrorKind::WouldBlock.into());
        }
        let end = if self.offset < self.cut {
            self.cut
        } else {
            self.bytes.len()
        };
        let count = target.len().min(self.maximum).min(end - self.offset);
        target[..count].copy_from_slice(&self.bytes[self.offset..self.offset + count]);
        self.offset += count;
        Ok(count)
    }
}

/// Return EOF once, then expose repeated reads without allowing a persistent-zero loop.
struct EofOnceReader<'a> {
    /// Supplied bytes not yet consumed by a nonempty read.
    bytes: &'a [u8],
    /// A nonempty read has actually observed the sole zero-byte EOF result.
    observed_eof: bool,
    /// A nonempty read was attempted after that EOF result.
    read_after_eof: bool,
}

impl Read for EofOnceReader<'_> {
    /// Consume bytes normally; empty targets leave all EOF observations unchanged.
    ///
    /// # Errors
    /// Return `WouldBlock` and record any nonempty read after the sole EOF result.
    fn read(&mut self, target: &mut [u8]) -> io::Result<usize> {
        if target.is_empty() {
            return Ok(0);
        }
        if self.observed_eof {
            self.read_after_eof = true;
            return Err(io::ErrorKind::WouldBlock.into());
        }
        if self.bytes.is_empty() {
            self.observed_eof = true;
            return Ok(0);
        }
        let count = target.len().min(self.bytes.len());
        let (bytes, remaining) = self.bytes.split_at(count);
        target[..count].copy_from_slice(bytes);
        self.bytes = remaining;
        Ok(count)
    }
}

/// Fail once with interruption and then differently, making a retry observable.
struct InterruptedReader(usize);

impl Read for InterruptedReader {
    /// No input is consumed; a retry changes the observed typed refusal.
    ///
    /// # Errors
    /// Return `Interrupted` on the first call and `PermissionDenied` thereafter.
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        self.0 += 1;
        Err(if self.0 == 1 {
            io::ErrorKind::Interrupted
        } else {
            io::ErrorKind::PermissionDenied
        }
        .into())
    }
}

/// Add exact nanoseconds without relying on unchecked test-time arithmetic.
///
/// # Errors
/// Return `Failure::Deadline` if the requested instant is unrepresentable.
fn at(start: Instant, nanos: u64) -> Result<Instant, Failure> {
    start
        .checked_add(Duration::from_nanos(nanos))
        .ok_or(Failure::Deadline)
}

/// Exact literal offer, opaque bodies and both exporter contexts must survive.
///
/// # Errors
/// Preserve unexpected rejection of the fixed offer or valid opaque-cookie reply.
///
/// # Panics
/// Fail if offer bytes, cookie bodies, endpoint or exporter metadata differ from the literals.
#[test]
fn literal_offer_and_reply_metadata() -> Result<(), Failure> {
    let mut request = [0xa5; 16];
    let offer = encode_offer(&mut request)?;
    assert_eq!(
        offer.request_bytes(),
        [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0]
    );
    assert_eq!(offer.offered_aead_bytes(), [0, 15]);
    let cookies: [&[u8]; 3] = [&[], &[0xaa, 0xbb, 0xcc], &[0x80, 0, 0, 0]];
    let reply = decode_bytes(&response(&cookies, &[])?)?;
    assert_eq!(
        reply
            .cookies
            .iter()
            .map(|body| body.as_slice())
            .collect::<Vec<_>>(),
        cookies
    );
    assert_eq!(reply.udp.to_string(), "127.0.0.1:123");
    assert_eq!(reply.c2s_context.label(), "EXPORTER-network-time-security");
    assert_eq!(reply.s2c_context.label(), "EXPORTER-network-time-security");
    assert_eq!(reply.c2s_context.context_bytes(), &[0, 0, 0, 15, 0]);
    assert_eq!(reply.s2c_context.context_bytes(), &[0, 0, 0, 15, 1]);
    Ok(())
}

/// Every small split, and bytewise progress, must stop at EOM without peer EOF.
///
/// # Errors
/// Preserve a valid fixture's rejection, including an unwanted read after its EOM.
///
/// # Panics
/// Fail if a split changes cookie bytes or causes more reads than the supplied fragments.
#[test]
fn every_small_split_and_bytewise_read_stops_at_eom() -> Result<(), Failure> {
    let bytes = response(&[&[0xaa, 0xbb, 0xcc]], &[])?;
    for cut in 0..=bytes.len() {
        let mut reader = SplitReader {
            bytes: &bytes,
            cut,
            maximum: usize::MAX,
            offset: 0,
            calls: 0,
        };
        let reply = decode(&mut reader)?;
        assert_eq!(reply.cookies[0].as_slice(), [0xaa, 0xbb, 0xcc]);
        assert_eq!(
            reader.calls,
            if cut == 0 || cut == bytes.len() { 1 } else { 2 }
        );
    }
    let mut reader = SplitReader {
        bytes: &bytes,
        cut: 0,
        maximum: 1,
        offset: 0,
        calls: 0,
    };
    let _reply = decode(&mut reader)?;
    assert_eq!(reader.calls, bytes.len());
    Ok(())
}

/// EOF, truncation and same-read trailing bytes cannot complete negotiation.
///
/// # Errors
/// Preserve fixture construction failure or rejection of the complete valid response neighbor.
///
/// # Panics
/// Fail if truncation succeeds, EOF is skipped/retried, EOM needs EOF, or other refusals change.
#[test]
fn eof_truncation_and_trailing_data_refuse() -> Result<(), Failure> {
    let bytes = response(&[&[0xaa]], &[])?;
    for cut in 0..bytes.len() {
        let mut reader = EofOnceReader {
            bytes: &bytes[..cut],
            observed_eof: false,
            read_after_eof: false,
        };
        assert!(decode(&mut reader).is_err());
        assert!(reader.observed_eof);
        assert!(!reader.read_after_eof);
    }
    let mut complete = EofOnceReader {
        bytes: &bytes,
        observed_eof: false,
        read_after_eof: false,
    };
    let _reply = decode(&mut complete)?;
    assert!(!complete.observed_eof);
    assert!(!complete.read_after_eof);
    let mut trailing = bytes;
    trailing.push(0);
    assert!(matches!(
        decode_bytes(&trailing),
        Err(Failure::Frame(NtsKeAccumulatorError::Frame(
            NtsKeFrameError::TrailingData
        )))
    ));
    let mut interrupted = InterruptedReader(0);
    assert!(matches!(
        decode(&mut interrupted),
        Err(Failure::Response(io::ErrorKind::Interrupted))
    ));
    assert_eq!(interrupted.0, 1);
    Ok(())
}

/// Abort codes are explicit, and late malformed records still invalidate them.
///
/// # Errors
/// Preserve an unexpected inability to frame either fixed abort record.
///
/// # Panics
/// Fail if abort codes change or a preceding abort conceals a malformed later record.
#[test]
fn peer_error_warning_and_late_shape_checks() -> Result<(), Failure> {
    for (kind, code, expected) in [
        (0x8002, 1_u16, NtsKeNegotiationError::PeerError { code: 1 }),
        (
            0x8003,
            0_u16,
            NtsKeNegotiationError::UnsupportedWarning { code: 0 },
        ),
    ] {
        let mut bytes = record(kind, &code.to_be_bytes())?;
        bytes.extend_from_slice(&END);
        assert!(
            matches!(decode_bytes(&bytes), Err(Failure::Negotiation(error)) if error == expected)
        );
        bytes.truncate(bytes.len() - 4);
        bytes.extend_from_slice(&[0x80, 4, 0, 1, 0]);
        bytes.extend_from_slice(&END);
        assert!(matches!(
            decode_bytes(&bytes),
            Err(Failure::Negotiation(
                NtsKeNegotiationError::InvalidAeadNegotiation
            ))
        ));
    }
    Ok(())
}

/// Only ignorable records survive; a selected algorithm must occur in the offer.
///
/// # Errors
/// Preserve fixture construction failure or rejection of the ignorable-record control.
///
/// # Panics
/// Fail if an unsupported critical record or an unoffered algorithm is not refused as specified.
#[test]
fn unknown_records_and_unoffered_algorithm() -> Result<(), Failure> {
    let _reply = decode_bytes(&response(&[&[]], &record(42, &[1])?)?)?;
    assert!(matches!(
        decode_bytes(&response(&[&[]], &record(0x802a, &[])?)?),
        Err(Failure::Negotiation(
            NtsKeNegotiationError::UnsupportedCriticalRecord
        ))
    ));
    let mut bytes = Vec::from(PROTOCOL);
    bytes.extend_from_slice(&[0x80, 4, 0, 2, 0, 16]);
    bytes.extend_from_slice(&[0, 5, 0, 0]);
    bytes.extend_from_slice(&END);
    assert!(matches!(
        decode_bytes(&bytes),
        Err(Failure::Negotiation(NtsKeNegotiationError::UnofferedAead))
    ));
    Ok(())
}

/// Response lengths straddling 64 KiB are all admitted independently of cookies.
///
/// # Errors
/// Preserve construction or admission failure for any of the three valid byte neighbors.
///
/// # Panics
/// Fail if a fixture has the wrong total length or its sole empty cookie changes.
#[test]
fn sixty_four_kib_neighbors_are_complete_responses() -> Result<(), Failure> {
    for length in [65_535, 65_536, 65_537] {
        let extra = record(42, &vec![0xa5; length - 24])?;
        let bytes = response(&[&[]], &extra)?;
        assert_eq!(bytes.len(), length);
        let reply = decode_bytes(&bytes)?;
        assert_eq!(reply.cookies.len(), 1);
        assert!(reply.cookies[0].is_empty());
    }
    Ok(())
}

/// The selected total byte cap accepts its exact value and rejects one over.
///
/// # Errors
/// Preserve an unexpected failure framing the bounded response records.
///
/// # Panics
/// Fail if a total length is wrong, a permitted neighbor fails or the excess escapes byte refusal.
#[test]
fn response_total_cap_neighbors() -> Result<(), Failure> {
    for (tail, accepted) in [(65_508, true), (65_509, true), (65_510, false)] {
        let mut extra = record(42, &vec![0xa5; 65_535])?;
        extra.extend(record(42, &vec![0x5a; tail])?);
        let bytes = response(&[&[]], &extra)?;
        assert_eq!(bytes.len(), 65_563 + tail);
        match decode_bytes(&bytes) {
            Ok(reply) => {
                assert!(accepted);
                assert_eq!(reply.cookies.len(), 1);
            }
            Err(error) => {
                assert!(!accepted);
                assert_eq!(
                    error,
                    Failure::Frame(NtsKeAccumulatorError::Frame(NtsKeFrameError::ByteLimit))
                );
            }
        }
    }
    Ok(())
}

/// Every empty cookie consumes a slot, and actual response copies own their data.
///
/// # Errors
/// Preserve fixture construction failure or rejection of the legal independent cookie copy.
///
/// # Panics
/// Fail if seven or eight cookies are refused, nine succeed or copied bytes follow source mutation.
#[test]
fn eight_cookies_succeed_and_nine_refuse() -> Result<(), Failure> {
    for count in [7, 8, 9] {
        let cookies = vec![&[][..]; count];
        match decode_bytes(&response(&cookies, &[])?) {
            Ok(reply) => {
                assert!(count <= 8);
                assert_eq!(reply.cookies.len(), count);
            }
            Err(error) => {
                assert_eq!(count, 9);
                assert_eq!(error, Failure::Cookies);
            }
        }
    }
    let mut original = vec![0xaa, 0xbb, 0xcc];
    let copies = copy_cookies([original.as_slice()].into_iter())?;
    original.fill(0);
    assert_eq!(copies[0].as_slice(), [0xaa, 0xbb, 0xcc]);
    Ok(())
}

/// Synthetic direct-helper controls reach an aggregate impossible in the wire cap.
///
/// # Panics
/// Fail if either permitted aggregate fails, its sum changes or one excess byte is accepted.
#[test]
fn defensive_cookie_aggregate_neighbors() {
    let large = vec![0xa5; 65_535];
    for (tail, accepted) in [(1, true), (2, true), (3, false)] {
        let last = vec![0x5a; tail];
        let result =
            copy_cookies([large.as_slice(), large.as_slice(), last.as_slice()].into_iter());
        match result {
            Ok(cookies) => {
                assert!(accepted);
                assert_eq!(
                    cookies.iter().map(|body| body.len()).sum::<usize>(),
                    131_070 + tail
                );
            }
            Err(error) => {
                assert!(!accepted);
                assert_eq!(error, Failure::Cookies);
            }
        }
    }
}

/// Numeric peer and typed verification identity admission remain independent.
///
/// # Errors
/// Preserve rejection of the fixed configuration or valid 253-byte verification name.
///
/// # Panics
/// Fail if typed identity, saved peer/name, length oracle or malformed-input refusal changes.
#[test]
fn configuration_admission_and_typed_identity() -> Result<(), Failure> {
    let configuration = config()?;
    assert!(matches!(&configuration.server_name, ServerName::DnsName(_)));
    assert_eq!(configuration.peer.to_string(), "127.0.0.1:4460");
    assert_eq!(configuration.identity, "Time.Example");
    for peer in [
        "127.0.0.1:0",
        "192.0.2.1:4460",
        "localhost:4460",
        "[::1%0]:4460",
        "[::1%1]:4460",
    ] {
        assert!(matches!(
            Config::new(peer.into(), "Time.Example".into()),
            Err(Failure::Arguments)
        ));
    }
    let name = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(name.len(), 253);
    let _maximum = Config::new("127.0.0.1:4460".into(), name.clone().into())?;
    // Exercise the actual structured-input rule with independent boundary facts.
    for identity in [configuration.identity.as_str(), name.as_str()] {
        let candidate = ConfigCandidate {
            values: (configuration.peer, identity),
        };
        assert!(candidate.validate().is_ok());
    }
    let overlong = format!("{name}d");
    assert_eq!(overlong.len(), 254);
    for identity in ["", overlong.as_str(), "tíme.example"] {
        let candidate = ConfigCandidate {
            values: (configuration.peer, identity),
        };
        assert!(candidate.validate().is_err());
    }
    for identity in [
        String::new(),
        String::from("bad name"),
        String::from("tíme.example"),
        format!("{name}d"),
    ] {
        assert!(matches!(
            Config::new("127.0.0.1:4460".into(), identity.into()),
            Err(Failure::Arguments)
        ));
    }
    Ok(())
}

/// Raw peer-byte neighbors remain valid socket addresses independently of the cap.
///
/// # Errors
/// Preserve unexpected standard-parser rejection of a pinned valid socket spelling.
///
/// # Panics
/// Fail if raw lengths, parsed peer meaning or the 128-byte constructor boundary changes.
#[test]
fn configuration_peer_byte_budget_neighbors() -> Result<(), Failure> {
    let expected = std::net::SocketAddr::from(([127, 0, 0, 1], 1));
    for (zeros, length) in [(116, 127), (117, 128), (118, 129)] {
        let peer = format!("127.0.0.1:{}1", "0".repeat(zeros));
        assert_eq!(peer.len(), length);
        let parsed = peer
            .parse::<std::net::SocketAddr>()
            .map_err(|_| Failure::Control)?;
        assert_eq!(parsed, expected);
        match Config::new(peer.into(), "Time.Example".into()) {
            Ok(configuration) => {
                assert!(length <= 128);
                assert_eq!(configuration.peer, expected);
            }
            Err(error) => {
                assert_eq!(length, 129);
                assert_eq!(error, Failure::Arguments);
            }
        }
    }
    Ok(())
}

/// Typed verification identities retain the supplied DNS/IP meaning separately from the peer.
///
/// # Errors
/// Preserve rejection of either valid DNS name or literal IP verification identity.
///
/// # Panics
/// Fail if the saved identity changes or the actual typed identity denotes another name/address.
#[test]
fn configuration_keeps_typed_verification_identity() -> Result<(), Failure> {
    let peer = std::net::SocketAddr::from(([127, 0, 0, 1], 4460));
    for expected in ["Time.Example", "other.example"] {
        let configuration = Config::new("127.0.0.1:4460".into(), expected.into())?;
        assert_eq!(configuration.peer, peer);
        assert_eq!(configuration.identity, expected);
        assert!(matches!(
            &configuration.server_name,
            ServerName::DnsName(name) if name.as_ref().eq_ignore_ascii_case(expected)
        ));
    }
    for (identity, expected) in [
        (
            "192.0.2.10",
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 0, 2, 10)),
        ),
        (
            "2001:0db8:0:0:0:0:0:000a",
            std::net::IpAddr::V6(std::net::Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 10)),
        ),
    ] {
        let configuration = Config::new("127.0.0.1:4460".into(), identity.into())?;
        assert_eq!(configuration.peer, peer);
        assert_eq!(configuration.identity, identity);
        assert!(matches!(
            &configuration.server_name,
            ServerName::IpAddress(address) if std::net::IpAddr::from(*address) == expected
        ));
    }
    Ok(())
}

/// Destination hints admit only the saved IP or the explicit named mapping.
///
/// # Errors
/// Preserve rejection of a valid configuration, mapped hint, nonzero port or IPv6 equivalent.
///
/// # Panics
/// Fail if destination authority broadens, port defaults change or saved KE provenance changes.
#[test]
fn endpoint_mapping_ports_and_numeric_equivalence() -> Result<(), Failure> {
    let configuration = config()?;
    for hint in [None, Some("127.0.0.1"), Some("tImE.eXaMpLe")] {
        assert_eq!(
            endpoint(&configuration, hint, None)?.to_string(),
            "127.0.0.1:123"
        );
    }
    for hint in [
        "127.0.0.2",
        "other.example",
        "Time.Example.",
        "[::1]",
        "::1%0",
    ] {
        assert_eq!(
            endpoint(&configuration, Some(hint), None),
            Err(Failure::Endpoint)
        );
    }
    assert_eq!(
        endpoint(&configuration, None, Some(0)),
        Err(Failure::Endpoint)
    );
    for port in [1, 123, 65_535] {
        assert_eq!(endpoint(&configuration, None, Some(port))?.port(), port);
    }
    assert_eq!(configuration.peer.port(), 4460);
    let ipv6 = Config::new("[::1]:4460".into(), "::1".into())?;
    assert!(matches!(&ipv6.server_name, ServerName::IpAddress(_)));
    assert_eq!(
        endpoint(&ipv6, Some("0:0:0:0:0:0:0:1"), None)?.to_string(),
        "[::1]:123"
    );
    Ok(())
}

/// Exact nanosecond neighbors share one origin; candidate expiry never resets.
///
/// # Errors
/// Preserve failed time construction, control construction or a legal remaining-time observation.
///
/// # Panics
/// Fail if expiry shifts, equality is admitted, remaining time is wrong or refusal changes phase.
#[test]
fn strict_deadline_neighbors_and_fixed_expiry() -> Result<(), Failure> {
    let start = Instant::now();
    let control = Control::new(start)?;
    assert_eq!(control.expiry, at(start, 60_000_000_000)?);
    for (phase, limit) in [
        (IoPhase::Handshake, 20_000_000_000),
        (IoPhase::Exchange, 60_000_000_000),
    ] {
        assert_eq!(
            control.remaining_at(phase, at(start, limit - 1)?)?,
            Duration::from_nanos(1)
        );
        assert_eq!(
            control.remaining_at(phase, at(start, limit)?),
            Err(Failure::Deadline)
        );
        assert_eq!(
            control.remaining_at(phase, at(start, limit + 1)?),
            Err(Failure::Deadline)
        );
    }
    assert_eq!(
        control.remaining_at(IoPhase::Handshake, at(start, 19_000_000_000)?)?,
        Duration::from_secs(1)
    );
    assert_eq!(
        control.remaining_at(IoPhase::Handshake, at(start, 21_000_000_000)?),
        Err(Failure::Deadline)
    );
    for offset in [59_999_999_999, 60_000_000_000, 60_000_000_001] {
        let candidate = Control::new(start)?;
        candidate.publish_ke_at(start)?;
        assert_eq!(
            candidate.commit_at(at(start, offset)?),
            if offset < 60_000_000_000 {
                Ok(())
            } else {
                Err(Failure::Deadline)
            }
        );
        assert_eq!(
            candidate.cancel()?,
            if offset < 60_000_000_000 {
                CancelOutcome::AlreadyCommitted
            } else {
                CancelOutcome::Cancelled
            }
        );
    }
    Ok(())
}

/// A valid time observation cannot replace the required KE publication phase.
/// # Errors
/// Preserve control construction, phase observation and legal transition refusal.
/// # Panics
/// Fail if an open control commits, loses its original cap, or blocks its neighbor.
#[test]
fn open_control_cannot_skip_ke_publication_before_commit() -> Result<(), Failure> {
    let start = Instant::now();
    let control = Control::new(start)?;
    let original_expiry = control.expiry;
    assert_eq!(control.commit_at(start), Err(Failure::Control));
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Open
    ));
    assert_eq!(control.expiry, original_expiry);
    control.publish_ke_at(start)?;
    control.commit_at(start)?;
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Committed
    ));
    assert_eq!(control.expiry, original_expiry);
    Ok(())
}

/// Pre-origin observations refuse without closing the legal original-start neighbor.
///
/// # Errors
/// Preserve failed time/control construction, phase observation or legal start-time admission.
///
/// # Panics
/// Fail if earlier time gains authority, refusal changes the open phase or the start cannot commit.
#[test]
fn pre_origin_refusal_preserves_open_phase() -> Result<(), Failure> {
    let before = Instant::now();
    let start = at(before, 1)?;
    let control = Control::new(start)?;
    for phase in [IoPhase::Handshake, IoPhase::Exchange] {
        assert_eq!(control.remaining_at(phase, before), Err(Failure::Deadline));
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Open
        ));
    }
    assert_eq!(control.commit_at(before), Err(Failure::Deadline));
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Open
    ));
    assert_eq!(
        control.remaining_at(IoPhase::Handshake, start)?,
        Duration::from_secs(20)
    );
    assert_eq!(
        control.remaining_at(IoPhase::Exchange, start)?,
        Duration::from_secs(60)
    );
    control.publish_ke_at(start)?;
    control.commit_at(start)?;
    assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
    Ok(())
}

/// An actually poisoned mutex refuses every control operation without repairing authority.
///
/// # Errors
/// Preserve control construction, unexpected loss of poison or rejection of the fresh legal neighbor.
///
/// # Panics
/// Catch one intentional unwind under the guard; fail if poison clears, phase changes or admission succeeds.
#[test]
fn poisoned_mutex_refuses_and_preserves_open_phase() -> Result<(), Failure> {
    let start = Instant::now();
    let poisoned = Control::new(start)?;
    let caught = std::panic::catch_unwind(|| -> Result<(), Failure> {
        let _held = poisoned.phase.lock().map_err(|_| Failure::Control)?;
        std::panic::resume_unwind(Box::new(()))
    });
    assert!(caught.is_err());
    assert!(poisoned.phase.is_poisoned());
    for phase in [IoPhase::Handshake, IoPhase::Exchange] {
        assert_eq!(poisoned.remaining_at(phase, start), Err(Failure::Control));
    }
    assert_eq!(poisoned.commit_at(start), Err(Failure::Control));
    assert_eq!(poisoned.cancel(), Err(Failure::Control));
    assert!(poisoned.phase.is_poisoned());
    let phase = match poisoned.phase.lock() {
        Ok(_held) => return Err(Failure::Control),
        Err(error) => error.into_inner(),
    };
    assert!(matches!(*phase, Phase::Open));
    drop(phase);
    assert!(poisoned.phase.is_poisoned());
    let healthy = Control::new(start)?;
    assert_eq!(
        healthy.remaining_at(IoPhase::Handshake, start)?,
        Duration::from_secs(20)
    );
    assert_eq!(
        healthy.remaining_at(IoPhase::Exchange, start)?,
        Duration::from_secs(60)
    );
    healthy.publish_ke_at(start)?;
    healthy.commit_at(start)?;
    assert_eq!(healthy.cancel()?, CancelOutcome::AlreadyCommitted);
    Ok(())
}

/// Cancellation closes publication while historical committed completion survives.
///
/// # Errors
/// Preserve failure constructing controls or taking a legal cancellation/commit transition.
///
/// # Panics
/// Fail if cancellation permits publication or committed history can be cancelled or committed twice.
#[test]
fn cancellation_and_commit_are_consume_once() -> Result<(), Failure> {
    let start = Instant::now();
    let cancelled = Control::new(start)?;
    assert_eq!(cancelled.cancel()?, CancelOutcome::Cancelled);
    assert_eq!(cancelled.cancel()?, CancelOutcome::AlreadyCancelled);
    assert_eq!(cancelled.commit_at(start), Err(Failure::Cancelled));
    assert_eq!(
        cancelled.remaining_at(IoPhase::Exchange, start),
        Err(Failure::Cancelled)
    );
    let committed = Control::new(start)?;
    committed.publish_ke_at(start)?;
    committed.commit_at(start)?;
    assert_eq!(committed.cancel()?, CancelOutcome::AlreadyCommitted);
    assert_eq!(committed.commit_at(start), Err(Failure::Control));
    assert_eq!(committed.cancel()?, CancelOutcome::AlreadyCommitted);
    Ok(())
}

/// Competing cancellation/commit can have exactly one winning terminal effect.
///
/// # Errors
/// Preserve control/cancellation failure or a panic observed while joining the competing commit.
///
/// # Panics
/// Fail unless exactly one terminal winner is paired with the other operation's required refusal.
#[test]
fn concurrent_cancel_and_commit_have_one_winner() -> Result<(), Failure> {
    let start = Instant::now();
    let control = Control::new(start)?;
    control.publish_ke_at(start)?;
    let barrier = std::sync::Barrier::new(2);
    let (commit, cancel) = std::thread::scope(|scope| {
        let job = scope.spawn(|| {
            barrier.wait();
            control.commit_at(start)
        });
        barrier.wait();
        let cancellation = control.cancel()?;
        let completion = job.join().map_err(|_| Failure::Control)?;
        Ok::<_, Failure>((completion, cancellation))
    })?;
    assert!(matches!(
        (commit, cancel),
        (Ok(()), CancelOutcome::AlreadyCommitted)
            | (Err(Failure::Cancelled), CancelOutcome::Cancelled)
    ));
    Ok(())
}

/// Raw non-Unicode invocations refuse before provider or network admission.
///
/// # Panics
/// Fail if either non-Unicode peer bytes or non-Unicode identity bytes evade argument refusal.
#[cfg(unix)]
#[test]
fn non_unicode_arguments_refuse() {
    use std::os::unix::ffi::OsStringExt;
    assert!(matches!(
        Config::new(OsString::from_vec(vec![0xff]), "Time.Example".into()),
        Err(Failure::Arguments)
    ));
    assert!(matches!(
        Config::new("127.0.0.1:4460".into(), OsString::from_vec(vec![0xff])),
        Err(Failure::Arguments)
    ));
}

/// The actual shared publication path samples time while its phase is locked.
/// These explicit observations are component boundaries, not a native-clock pass.
/// # Errors
/// Preserve control construction or fresh neighboring publication refusal.
/// # Panics
/// Fail if observation precedes locking or equality publishes/renews authority.
#[test]
fn publication_observation_is_inside_the_actual_phase_guard() -> Result<(), Failure> {
    let start = Instant::now();
    let expired = Control::new(start)?;
    assert_eq!(
        expired.publish_ke_observing(|| {
            assert!(matches!(
                expired.phase.try_lock(),
                Err(std::sync::TryLockError::WouldBlock)
            ));
            expired.expiry
        }),
        Err(Failure::Deadline)
    );
    assert!(matches!(
        *expired.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Open
    ));
    let healthy = Control::new(start)?;
    healthy.publish_ke_observing(|| {
        assert!(matches!(
            healthy.phase.try_lock(),
            Err(std::sync::TryLockError::WouldBlock)
        ));
        start
    })?;
    assert!(matches!(
        *healthy.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Published
    ));
    assert_eq!(healthy.expiry, expired.expiry);
    Ok(())
}

/// CLI compatibility is never-send; protected UDP needs one exact explicit flag.
/// # Errors
/// Preserve valid configuration construction failures.
/// # Panics
/// Fail if missing/extra/unknown arguments or non-loopback peers are admitted.
#[test]
fn protected_mode_requires_exact_explicit_cli_admission() -> Result<(), Failure> {
    let preview = Config::from_arguments(
        ["127.0.0.1:4460", "Time.Example"]
            .map(OsString::from)
            .into_iter(),
    )?;
    assert_eq!(preview.mode, super::ExchangeMode::Preview);
    let protected = Config::from_arguments(
        ["127.0.0.1:4460", "Time.Example", "--protected-datagram"]
            .map(OsString::from)
            .into_iter(),
    )?;
    assert_eq!(protected.mode, super::ExchangeMode::ProtectedDatagram);
    for arguments in [
        vec![],
        vec!["127.0.0.1:4460"],
        vec!["127.0.0.1:4460", "Time.Example", "--protected-datagrams"],
        vec![
            "127.0.0.1:4460",
            "Time.Example",
            "--protected-datagram",
            "extra",
        ],
        vec!["192.0.2.1:4460", "Time.Example", "--protected-datagram"],
    ] {
        assert!(matches!(
            Config::from_arguments(arguments.into_iter().map(OsString::from)),
            Err(Failure::Arguments)
        ));
    }
    Ok(())
}

/// The dedicated syscall guard is exclusive and available only while published.
/// # Errors
/// Preserve actual Control construction and publication failures.
/// # Panics
/// Fail if an unready/closed/expired context gains a syscall guard.
#[test]
fn syscall_guard_preserves_original_phase_and_expiry() -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    assert!(matches!(control.published_guard(), Err(Failure::Control)));
    control.publish_ke()?;
    {
        let guard = control.published_guard()?;
        assert!(matches!(*guard, Phase::Published));
        assert!(matches!(
            control.phase.try_lock(),
            Err(std::sync::TryLockError::WouldBlock)
        ));
    }
    assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    assert!(matches!(control.published_guard(), Err(Failure::Cancelled)));
    let mut expired = Control::new(Instant::now())?;
    expired.publish_ke()?;
    expired.expiry = expired.origin;
    assert!(matches!(expired.published_guard(), Err(Failure::Deadline)));
    let committed = Control::new(Instant::now())?;
    committed.publish_ke()?;
    committed.commit()?;
    assert!(matches!(committed.published_guard(), Err(Failure::Control)));
    Ok(())
}

/// A registered waiter sees cancellation even when it occurs before its await.
/// This exercises no socket, TLS or native operation.
/// # Errors
/// Preserve actual Control construction/cancellation or timer-runtime failures.
/// # Panics
/// Fail if registered cancellation is lost.
#[test]
fn cancellation_wakes_an_already_registered_waiter() -> Result<(), Failure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .map_err(|error| {
            Failure::Datagram(super::datagram::DatagramError::Runtime(error.kind()))
        })?;
    runtime.block_on(async {
        let control = Control::new(Instant::now())?;
        let notified = control.cancelled.notified();
        tokio::pin!(notified);
        assert!(!notified.as_mut().enable());
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        assert!(
            tokio::time::timeout(Duration::from_secs(1), notified)
                .await
                .is_ok()
        );
        Ok(())
    })
}
