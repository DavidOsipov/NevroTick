//! Actual private receiver controls with public material, never TLS evidence.

use std::cell::Cell;
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::{DirectionalKeys, PreparedRequest, prepare};
use crate::nts_ke::{CancelOutcome, Control, Failure, Phase};

/// Independent complete packet, generated before this receiver existed.
const PACKET: &[u8; 140] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../crypto-aes-siv-witness/tests/fixtures/sealing-zero.bin"
));

/// Construct public component keys without claiming an authenticated producer.
fn keys() -> DirectionalKeys {
    let key = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];
    DirectionalKeys::new(Zeroizing::new(key), Zeroizing::new([0xee; 32]))
}

/// Supply fixed public UID/nonce bytes matching the separate oracle fixture.
///
/// # Errors
/// Refuse any fixture index that cannot fit its declared byte representation.
fn entropy(output: &mut [u8; 48]) -> Result<(), Failure> {
    for (index, byte) in output.iter_mut().enumerate() {
        let value = if index < 32 { index } else { index - 16 };
        *byte = u8::try_from(value).map_err(|_| Failure::Control)?;
    }
    Ok(())
}

/// Construct the public first cookie; no ownership/reuse policy is inferred.
fn cookies() -> Vec<Zeroizing<Vec<u8>>> {
    vec![Zeroizing::new((0xa0_u8..=0xab).collect())]
}

/// The actual component must match the independent answer before final commit.
///
/// # Errors
/// Propagate component preparation/currentness refusal.
/// # Panics
/// A complete packet, unchanged open phase or final counts differ from the oracle.
#[test]
fn real_sealer_prepares_then_commits_exact_packet() -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let request = prepare(&control, keys(), &cookies(), entropy)?;
    assert_eq!(&request.packet[..request.length], PACKET);
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Open
    ));
    let summary = request.finish()?;
    assert_eq!(
        (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
        (1, 12, 140)
    );
    assert_eq!(control.cancel(), Ok(CancelOutcome::AlreadyCommitted));
    Ok(())
}

/// Cancellation after actual sealing must prevent the summary commit.
///
/// # Errors
/// Propagate preparation or cancellation refusal.
/// # Panics
/// The completed packet differs, or cancellation permits a final summary.
#[test]
fn cancellation_after_sealing_refuses_final_completion() -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let request = prepare(&control, keys(), &cookies(), entropy)?;
    assert_eq!(&request.packet[..request.length], PACKET);
    assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    assert!(matches!(request.finish(), Err(Failure::Cancelled)));
    Ok(())
}

/// Abandoning prepared work cannot manufacture historical completion.
///
/// # Errors
/// Propagate preparation or cancellation refusal.
/// # Panics
/// Drop committed, so the first later cancellation cannot close admission.
#[test]
fn abandoned_packet_leaves_no_completion() -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let request = prepare(&control, keys(), &cookies(), entropy)?;
    drop(request);
    assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    Ok(())
}

/// Earlier cancellation prevents entropy acquisition as well as sealing.
///
/// # Errors
/// Propagate setup/cancellation refusal.
/// # Panics
/// A retired invocation requests entropy or returns prepared work.
#[test]
fn retired_invocation_has_no_entropy_effect() -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    control.cancel()?;
    let calls = Cell::new(0);
    let result = prepare(&control, keys(), &cookies(), |output| {
        calls.set(calls.get() + 1);
        entropy(output)
    });
    assert!(matches!(result, Err(Failure::Cancelled)));
    assert_eq!(calls.get(), 0);
    Ok(())
}

/// Entropy refusal and cancellation during its real call cannot commit.
///
/// # Errors
/// Propagate setup or cancellation refusal.
/// # Panics
/// A refusal is replaced, or the post-entropy currentness guard is omitted.
#[test]
fn entropy_failure_and_post_entropy_cancellation_refuse() -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let failed = prepare(&control, keys(), &cookies(), |output| {
        output.fill(0xa5);
        Err(Failure::Entropy)
    });
    assert!(matches!(failed, Err(Failure::Entropy)));
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Open
    ));
    let cancelled = prepare(&control, keys(), &cookies(), |output| {
        entropy(output)?;
        control.cancel()?;
        Ok(())
    });
    assert!(matches!(cancelled, Err(Failure::Cancelled)));
    Ok(())
}

/// The first exact body cannot be silently replaced by a later usable cookie.
///
/// # Errors
/// Propagate control setup or cancellation refusal.
/// # Panics
/// An unusable first body is rewritten/skipped or failure commits the invocation.
#[test]
fn first_unusable_cookie_refuses_despite_valid_second() -> Result<(), Failure> {
    for first in [vec![], vec![0x55; 11], vec![0x55; 3972]] {
        let control = Control::new(Instant::now())?;
        let mut population = vec![Zeroizing::new(first)];
        population.extend(cookies());
        assert!(matches!(
            prepare(&control, keys(), &population, entropy),
            Err(Failure::Packet(_))
        ));
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    }
    Ok(())
}

/// Complete population bounds precede entropy, including unused trailing bodies.
///
/// # Errors
/// Propagate setup refusal.
/// # Panics
/// Invalid count/aggregate reaches entropy or produces prepared output.
#[test]
fn whole_cookie_population_is_bounded_before_entropy() -> Result<(), Failure> {
    for population in [
        vec![],
        (0..9).map(|_| Zeroizing::new(vec![0; 12])).collect(),
        vec![
            Zeroizing::new(vec![0; 12]),
            Zeroizing::new(vec![0; 131_072]),
        ],
    ] {
        let control = Control::new(Instant::now())?;
        let calls = Cell::new(0);
        let result = prepare(&control, keys(), &population, |output| {
            calls.set(calls.get() + 1);
            entropy(output)
        });
        assert!(matches!(result, Err(Failure::Cookies)));
        assert_eq!(calls.get(), 0);
    }
    Ok(())
}

/// Exact count and aggregate limits remain usable without selecting later bodies.
///
/// # Errors
/// Propagate setup/preparation/final-commit refusal.
/// # Panics
/// A limit is exclusive, unused bodies alter the packet, or total counts differ.
#[test]
fn exact_cookie_population_limits_have_healthy_neighbors() -> Result<(), Failure> {
    let mut eight = cookies();
    eight.extend((0..7).map(|_| Zeroizing::new(vec![])));
    let mut maximum = cookies();
    maximum.extend((0..2).map(|_| Zeroizing::new(vec![0; 65_530])));
    for (population, count, total) in [(eight, 8, 12), (maximum, 3, 131_072)] {
        let control = Control::new(Instant::now())?;
        let request = prepare(&control, keys(), &population, entropy)?;
        assert_eq!(&request.packet[..request.length], PACKET);
        let summary = request.finish()?;
        assert_eq!(
            (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
            (count, total, 140)
        );
    }
    let control = Control::new(Instant::now())?;
    let mut over = cookies();
    over.extend([
        Zeroizing::new(vec![0; 65_530]),
        Zeroizing::new(vec![0; 65_531]),
    ]);
    let calls = Cell::new(0);
    assert!(matches!(
        prepare(&control, keys(), &over, |output| {
            calls.set(calls.get() + 1);
            entropy(output)
        }),
        Err(Failure::Cookies)
    ));
    assert_eq!(calls.get(), 0);
    Ok(())
}

/// The original bound control is checked again after sealing, at finish.
///
/// # Errors
/// Propagate setup/preparation refusal or checked fixture deadline arithmetic.
/// # Panics
/// An expired prepared packet commits or its original phase changes on refusal.
#[test]
fn prepared_packet_cannot_outlive_bound_expiry() -> Result<(), Failure> {
    let mut control = Control::new(Instant::now())?;
    // Reduce only this private component fixture's wait; production remains60s.
    control.expiry = Instant::now()
        .checked_add(Duration::from_secs(1))
        .ok_or(Failure::Deadline)?;
    let request: PreparedRequest<'_> = prepare(&control, keys(), &cookies(), entropy)?;
    std::thread::sleep(
        control.expiry.saturating_duration_since(Instant::now()) + Duration::from_millis(1),
    );
    assert!(matches!(request.finish(), Err(Failure::Deadline)));
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Open
    ));
    Ok(())
}
