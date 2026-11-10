//! Independent public literals and rejected-neighbor controls; no native effects.

use std::ffi::OsString;
use std::time::{Duration, Instant};

use super::{Budget, Config, Failed, Failure, Flow, IO_LIMIT, RESPONSE, admit_offer, ready_line, resolve};

/// Later alert-write refusal stays separate from the original TLS protocol error.
#[test]
fn primary_tls_error_precedes_a_later_alert_fault() {
    let primary = Failure::Io(std::io::ErrorKind::InvalidData);
    let cleanup = Failure::Io(std::io::ErrorKind::BrokenPipe);
    assert_eq!(
        resolve::<()>(Some(cleanup), Err(primary)),
        Err(Failed { primary, cleanup: Some(cleanup) })
    );
    assert_eq!(resolve::<()>(None, Err(primary)), Err(primary.into()));
    assert_eq!(resolve::<()>(Some(primary), Err(primary)), Err(primary.into()));
}

/// Sticky faults refuse nominal success; ordinary errors and success stay intact.
#[test]
fn sticky_fault_and_healthy_result_neighbors() {
    assert_eq!(resolve(None, Ok(7_u8)), Ok(7));
    assert_eq!(resolve(Some(Failure::Deadline), Ok(7_u8)), Err(Failure::Deadline.into()));
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

/// The sole four-byte cookie implies a132-byte untransmitted client request.
#[test]
fn literal_response_is_twenty_four_bytes_with_four_byte_cookie() {
    assert_eq!(
        RESPONSE,
        [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0, 5, 0, 4,
         0xaa, 0xbb, 0xcc, 0xdd, 0x80, 0, 0, 0]
    );
    let cookie_bytes = usize::from(u16::from_be_bytes([RESPONSE[14], RESPONSE[15]]));
    assert_eq!(48 + (4 + 32) + (4 + cookie_bytes) + (4 + 4 + 16 + 16), 132);
}

/// Port zero refuses; both admitted numeric extremes have the exact ASCII form.
#[test]
fn readiness_port_is_bounded_and_has_no_optional_fields() -> Result<(), Failure> {
    assert_eq!(ready_line(0), Err(Failure::Arguments));
    assert_eq!(ready_line(1)?, "port=1\n");
    assert_eq!(ready_line(u16::MAX)?, "port=65535\n");
    Ok(())
}

/// Exact argument shape succeeds; missing, extra, relative and aliased paths fail.
#[test]
fn cli_has_no_optional_effect_or_case_switch() -> Result<(), Failure> {
    let config = Config::parse(arguments().into_iter())?;
    assert!(config.certificate.is_absolute());
    assert!(config.key.is_absolute());
    assert!(config.ready.is_absolute());
    let mut missing = arguments();
    let _removed = missing.pop();
    assert!(matches!(Config::parse(missing.into_iter()), Err(Failure::Arguments)));
    let mut extra = arguments();
    extra.push(OsString::from("healthy"));
    assert!(matches!(Config::parse(extra.into_iter()), Err(Failure::Arguments)));
    for value in ["relative.der", "", "é", "nul\0path"] {
        let mut invalid = arguments();
        invalid[1] = OsString::from(value);
        assert!(matches!(Config::parse(invalid.into_iter()), Err(Failure::Arguments)));
    }
    let mut same = arguments();
    same[2] = same[1].clone();
    assert!(matches!(Config::parse(same.into_iter()), Err(Failure::Arguments)));
    let mut long = arguments();
    long[1] = OsString::from("x".repeat(1025));
    assert!(matches!(Config::parse(long.into_iter()), Err(Failure::Arguments)));
    Ok(())
}

/// Polls never renew the origin; equality and later times both refuse.
#[test]
fn original_deadline_has_exact_neighbors() -> Result<(), Failure> {
    let origin = Instant::now();
    let budget = Budget::new(origin)?;
    assert_eq!(budget.poll(origin), Ok(Duration::from_millis(200)));
    let end = origin.checked_add(Duration::from_secs(60)).ok_or(Failure::Deadline)?;
    let before = end.checked_sub(Duration::from_nanos(1)).ok_or(Failure::Deadline)?;
    assert_eq!(budget.poll(before), Ok(Duration::from_nanos(1)));
    assert_eq!(budget.poll(end), Err(Failure::Deadline));
    let after = end.checked_add(Duration::from_nanos(1)).ok_or(Failure::Deadline)?;
    assert_eq!(budget.poll(after), Err(Failure::Deadline));
    Ok(())
}

/// Each direction independently accepts its cap; overflow preserves old counts.
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
